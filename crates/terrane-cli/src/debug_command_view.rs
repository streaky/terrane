use std::io::{BufWriter, Write as _};
use std::collections::{BTreeMap, BTreeSet};

use super::source::{
    all_source_associations, generated_association, generated_path, lexical_normalize,
    normalized_source_path, requested_source_paths,
};
use super::{
    BufRead, BufReader, CliFailure, Command, ExitCode, OsString, Path, ProvenanceManifest, Stdio,
    Value, debugger_failure, fs, json, load_and_validate,
};
use terrane_compiler::debugging::GeneratedFileIdentity;

// The on-disk index contains only instructions attributed to this build's generated files.
type Disassembly = BTreeMap<String, BTreeMap<usize, Vec<(u64, String)>>>;

pub(crate) fn run(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let [executable, sidecar, source, line, generated @ ..] = arguments else {
        return Err(CliFailure::usage());
    };
    let rust_only = matches!(generated, [flag] if flag == "--rust-only");
    let selected = match generated {
        [] => None,
        [_] if rust_only => None,
        [path, line] => Some((Path::new(path), parse_line(line)?)),
        _ => return Err(CliFailure::usage()),
    };
    let line = parse_line(line)?;
    let executable = Path::new(executable).canonicalize().map_err(|err| {
        debugger_failure(format!("cannot locate debug executable {executable:?}: {err}"))
    })?;
    let provenance = load_and_validate(Path::new(sidecar), &executable, None)?;
    let requested = requested_source_paths(&provenance, Path::new(source));
    let (source, source_path) = provenance
        .debug
        .sources
        .iter()
        .find_map(|identity| {
            let path = normalized_source_path(&provenance, Path::new(&identity.uri));
            requested.contains(&path).then_some((identity, path))
        })
        .ok_or_else(|| debugger_failure(format!("{source:?} is not in this debug build")))?;
    if !fs::read(&source_path).is_ok_and(|bytes| {
        terrane_compiler::debugging::hash_bytes(&bytes) == source.content_hash
    }) {
        return Err(debugger_failure(format!(
            "source has changed or is unavailable in this debug build: {}",
            source_path.display()
        )));
    }

    let build_root = Path::new(&provenance.relocation.build_root);
    if let Some((path, rust_line)) = selected {
        let rooted = if path.is_absolute() {
            lexical_normalize(path)
        } else {
            lexical_normalize(&build_root.join(path))
        };
        let file = provenance
            .debug
            .generated_files
            .iter()
            .find(|file| rooted.strip_prefix(build_root).ok() == Some(Path::new(&file.path)))
            .filter(|file| {
                generated_association(file, rust_line)
                    .and_then(|association| association.causes.first())
                    .is_some_and(|cause| cause.source_id == source.id && cause.line == line)
            })
            .ok_or_else(|| {
                debugger_failure(format!(
                    "{}:{rust_line} does not map to {}:{line} in this debug build",
                    path.display(),
                    source_path.display()
                ))
            })?;
        validate_displayed_file(file, &rooted)?;
        println!("{}", json!({
            "source": { "path": source_path, "line": line },
            "rust": [{ "path": rooted, "line": rust_line }],
            "assembly": [],
        }));
        return Ok(ExitCode::SUCCESS);
    }

    let mut points = BTreeMap::<&str, BTreeSet<usize>>::new();
    let mut other = BTreeMap::<&str, BTreeSet<usize>>::new();
    for (file, association) in all_source_associations(&provenance) {
        if association
            .causes
            .iter()
            .any(|cause| cause.source_id == source.id && cause.line == line)
        {
            let locations = if association.sequence_point {
                &mut points
            } else {
                &mut other
            };
            locations
                .entry(file)
                .or_default()
                .insert(association.generated.line);
        }
    }
    let locations = if points.is_empty() { other } else { points };
    let mut index = if rust_only || locations.is_empty() {
        BTreeMap::new()
    } else {
        load_disassembly(&provenance, &executable, build_root)?
    };
    let mut rust = Vec::new();
    let mut assembly = Vec::new();
    for (relative, lines) in locations {
        let path = generated_path(&provenance, relative);
        let file = provenance
            .debug
            .generated_files
            .iter()
            .find(|file| file.path == relative)
            .expect("mapped location belongs to a generated file");
        validate_displayed_file(file, &path)?;
        let mut instructions = index.remove(relative).unwrap_or_default();
        for line in lines {
            rust.push(json!({ "path": path, "line": line }));
            if let Some(rows) = instructions.remove(&line) {
                assembly.extend(rows.into_iter().map(|(address, instruction)| {
                    (address, json!({
                        "address": format!("0x{address:x}"),
                        "instruction": instruction,
                        "path": path,
                        "line": line,
                    }))
                }));
            }
        }
    }
    assembly.sort_unstable_by_key(|(address, _)| *address);
    let assembly: Vec<Value> = assembly.into_iter().map(|(_, row)| row).collect();
    println!("{}", json!({
        "source": { "path": source_path, "line": line },
        "rust": rust,
        "assembly": assembly,
    }));
    Ok(ExitCode::SUCCESS)
}

fn parse_line(value: &OsString) -> Result<usize, CliFailure> {
    value
        .to_str()
        .and_then(|text| text.parse().ok())
        .filter(|line| *line > 0)
        .ok_or_else(|| CliFailure::usage_with("expected a positive, one-based line number"))
}

fn validate_displayed_file(file: &GeneratedFileIdentity, path: &Path) -> Result<(), CliFailure> {
    // load_and_validate already ensures external files match when no embedded copy exists.
    if file.embedded_source.is_some()
        && !fs::read(path).is_ok_and(|bytes| {
            terrane_compiler::debugging::hash_bytes(&bytes) == file.content_hash
        })
    {
        return Err(debugger_failure(format!(
            "on-disk generated source identity mismatch for {}",
            path.display()
        )));
    }
    Ok(())
}

fn load_disassembly(
    provenance: &ProvenanceManifest,
    executable: &Path,
    build_root: &Path,
) -> Result<Disassembly, CliFailure> {
    let cache = build_root.join("terrane-debug-disassembly-v1.json");
    let identity = &provenance.build.native_module.content_hash;
    if let Ok(bytes) = fs::read(&cache)
        && let Ok((key, index)) = serde_json::from_slice::<(String, Disassembly)>(&bytes)
        && key == *identity
    {
        return Ok(index);
    }
    let generated_files = provenance
        .debug
        .generated_files
        .iter()
        .map(|file| Path::new(&file.path))
        .collect::<BTreeSet<_>>();
    let index = disassemble(executable, build_root, &generated_files)?;
    // Do not publish instructions if the underlying artifact changed during disassembly.
    provenance.validate_executable(executable).map_err(debugger_failure)?;
    let temporary = cache.with_extension(format!("{}.tmp", std::process::id()));
    if let Ok(file) = fs::File::create_new(&temporary) {
        let mut writer = BufWriter::new(file);
        if serde_json::to_writer(&mut writer, &(identity, &index)).is_err()
            || writer.flush().is_err()
            || fs::rename(&temporary, &cache).is_err()
        {
            let _ = fs::remove_file(temporary);
        }
    }
    Ok(index)
}

fn disassemble(
    executable: &Path,
    build_root: &Path,
    generated_files: &BTreeSet<&Path>,
) -> Result<Disassembly, CliFailure> {
    let mut child = Command::new("objdump")
        .args(["--disassemble", "--line-numbers", "--wide", "--no-show-raw-insn"])
        .env("LC_ALL", "C")
        .env_remove("DEBUGINFOD_URLS")
        .arg(executable)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| debugger_failure(format!("cannot run GNU objdump for assembly view: {err}")))?;
    let mut stdout = BufReader::new(child.stdout.take().expect("objdump stdout is piped"));
    let mut text = Vec::new();
    let mut selected = None;
    let mut result = BTreeMap::<&str, BTreeMap<usize, Vec<(u64, String)>>>::new();
    let read_result = loop {
        text.clear();
        match stdout.read_until(b'\n', &mut text) {
            Ok(0) => break Ok(()),
            Ok(_) => {}
            Err(err) => break Err(err),
        }
        let Ok(text) = str::from_utf8(&text) else {
            selected = None;
            continue;
        };
        let text = text.trim_end();
        if let Some((address, instruction)) = instruction_row(text) {
            if let Some((path, line)) = selected {
                result
                    .entry(path)
                    .or_default()
                    .entry(line)
                    .or_default()
                    .push((address, instruction.to_owned()));
            }
        } else if !text.is_empty() {
            // An unmapped source line, function header or section boundary ends the previous line.
            selected = line_marker(text).and_then(|(path, line)| {
                let path = Path::new(path);
                let relative = if path.is_absolute() {
                    path.strip_prefix(build_root).ok()?
                } else {
                    path
                };
                let path = generated_files.get(relative)?.to_str()?;
                Some((path, line))
            });
        }
    };
    drop(stdout);
    let status = child.wait().map_err(|err| debugger_failure(format!("cannot wait for objdump: {err}")))?;
    read_result.map_err(|err| debugger_failure(format!("cannot read objdump output: {err}")))?;
    if !status.success() {
        return Err(debugger_failure(format!("objdump failed: {status}")));
    }
    Ok(result.into_iter().map(|(path, rows)| (path.to_owned(), rows)).collect())
}

fn line_marker(text: &str) -> Option<(&str, usize)> {
    let text = text.split_once(" (discriminator ").map_or(text, |(prefix, _)| prefix);
    let (path, line) = text.rsplit_once(':')?;
    Some((path, line.parse().ok()?))
}

fn instruction_row(text: &str) -> Option<(u64, &str)> {
    // GNU objdump separates the address from the mnemonic with a tab.
    let (address, instruction) = text.trim_start().split_once(":\t")?;
    let instruction = instruction.trim();
    if instruction.is_empty() {
        return None;
    }
    Some((u64::from_str_radix(address, 16).ok()?, instruction))
}
