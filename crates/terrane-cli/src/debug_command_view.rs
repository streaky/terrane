use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufWriter, Write as _};

use super::source::{
    all_source_associations, find_source, generated_association, generated_path, lexical_normalize,
};
use super::{
    BufRead, BufReader, CliFailure, Command, ExitCode, OsString, Path, ProvenanceManifest, Stdio,
    Value, debugger_failure, fs, json, load_provenance, validate_provenance,
};
use terrane_compiler::debugging::GeneratedFileIdentity;

// Each row stores its emission ordinal, address, and instruction. Addresses can restart
// at zero in each section or archive member, so only the ordinal is globally ordered.
type Disassembly = BTreeMap<String, BTreeMap<usize, Vec<(usize, u64, String)>>>;

enum Selection<'a> {
    Generated { path: &'a Path, line: usize },
    Source { include_assembly: bool },
}

pub(crate) fn run(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let [executable, sidecar, source, line, generated @ ..] = arguments else {
        return Err(CliFailure::usage());
    };
    let (generated, relocation) = match generated {
        [rest @ .., flag, payload] if flag == "--relocation" => {
            let relocation = payload
                .to_str()
                .and_then(|text| serde_json::from_str::<Value>(text).ok())
                .filter(Value::is_object)
                .ok_or_else(|| CliFailure::usage_with("expected a JSON relocation object"))?;
            (rest, Some(relocation))
        }
        _ => (generated, None),
    };
    let selection = match generated {
        [] => Selection::Source {
            include_assembly: true,
        },
        [flag] if flag == "--rust-only" => Selection::Source {
            include_assembly: false,
        },
        [path, line] => Selection::Generated {
            path: Path::new(path),
            line: parse_line(line)?,
        },
        _ => return Err(CliFailure::usage()),
    };
    let line = parse_line(line)?;
    let requested_executable = Path::new(executable);
    let executable = requested_executable.canonicalize().map_err(|err| {
        debugger_failure(format!(
            "cannot locate debug executable {}: {err}",
            requested_executable.display()
        ))
    })?;
    let provenance = load_provenance(Path::new(sidecar))?;
    let recorded_build_root = provenance.relocation.build_root.clone();
    let provenance = validate_provenance(provenance, &executable, relocation.as_ref())?;
    let requested_source = Path::new(source);
    let (source, source_path) = find_source(&provenance, requested_source).ok_or_else(|| {
        debugger_failure(format!(
            "{} is not in this debug build",
            requested_source.display()
        ))
    })?;
    if !fs::read(&source_path)
        .is_ok_and(|bytes| terrane_compiler::debugging::hash_bytes(&bytes) == source.content_hash)
    {
        return Err(debugger_failure(format!(
            "source has changed or is unavailable in this debug build: {}",
            source_path.display()
        )));
    }

    let (rust, assembly) = match selection {
        Selection::Generated {
            path,
            line: rust_line,
        } => (
            vec![select_generated(
                &provenance,
                source.id,
                &source_path,
                line,
                path,
                rust_line,
            )?],
            Vec::new(),
        ),
        Selection::Source { include_assembly } => query_source(
            &provenance,
            &executable,
            Path::new(&recorded_build_root),
            source.id,
            line,
            include_assembly,
        )?,
    };
    println!(
        "{}",
        json!({
            "source": { "path": source_path, "line": line },
            "rust": rust,
            "assembly": assembly,
        })
    );
    Ok(ExitCode::SUCCESS)
}

fn select_generated(
    provenance: &ProvenanceManifest,
    source_id: u32,
    source_path: &Path,
    line: usize,
    path: &Path,
    rust_line: usize,
) -> Result<Value, CliFailure> {
    let build_root = Path::new(&provenance.relocation.build_root);
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
                .is_some_and(|cause| cause.source_id == source_id && cause.line == line)
        })
        .ok_or_else(|| {
            debugger_failure(format!(
                "{}:{rust_line} does not map to {}:{line} in this debug build",
                path.display(),
                source_path.display()
            ))
        })?;
    validate_displayed_file(file, &rooted)?;
    Ok(json!({ "path": rooted, "line": rust_line }))
}

fn query_source(
    provenance: &ProvenanceManifest,
    executable: &Path,
    recorded_build_root: &Path,
    source_id: u32,
    line: usize,
    include_assembly: bool,
) -> Result<(Vec<Value>, Vec<Value>), CliFailure> {
    let mut points = BTreeMap::<&str, BTreeSet<usize>>::new();
    let mut other = BTreeMap::<&str, BTreeSet<usize>>::new();
    for (file, association) in all_source_associations(provenance) {
        if association
            .causes
            .iter()
            .any(|cause| cause.source_id == source_id && cause.line == line)
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
    let mut index = if !include_assembly || locations.is_empty() {
        BTreeMap::new()
    } else {
        load_disassembly(
            provenance,
            executable,
            Path::new(&provenance.relocation.build_root),
            recorded_build_root,
        )?
    };
    let mut rust = Vec::new();
    let mut assembly = Vec::new();
    for (relative, lines) in locations {
        let path = generated_path(provenance, relative);
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
                assembly.extend(rows.into_iter().map(|(ordinal, address, instruction)| {
                    (
                        ordinal,
                        json!({
                            "address": format!("0x{address:x}"),
                            "instruction": instruction,
                            "path": path,
                            "line": line,
                        }),
                    )
                }));
            }
        }
    }
    assembly.sort_unstable_by_key(|(ordinal, _)| *ordinal);
    Ok((rust, assembly.into_iter().map(|(_, row)| row).collect()))
}

fn parse_line(value: &OsString) -> Result<usize, CliFailure> {
    value
        .to_str()
        .and_then(|text| text.parse().ok())
        .filter(|line| *line > 0)
        .ok_or_else(|| CliFailure::usage_with("expected a positive, one-based line number"))
}

fn validate_displayed_file(file: &GeneratedFileIdentity, path: &Path) -> Result<(), CliFailure> {
    // Provenance validation already ensures external files match when no embedded copy exists.
    if file.embedded_source.is_some()
        && !fs::read(path)
            .is_ok_and(|bytes| terrane_compiler::debugging::hash_bytes(&bytes) == file.content_hash)
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
    recorded_build_root: &Path,
) -> Result<Disassembly, CliFailure> {
    let cache = build_root.join("terrane-debug-disassembly-v2.json");
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
    // DWARF line tables still name the build's recorded paths after relocation.
    let index = disassemble(executable, recorded_build_root, &generated_files)?;
    // Do not publish instructions if the underlying artifact changed during disassembly.
    provenance
        .validate_executable(executable)
        .map_err(debugger_failure)?;
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
        .args([
            "--disassemble",
            "--line-numbers",
            "--wide",
            "--no-show-raw-insn",
        ])
        .env("LC_ALL", "C")
        .env_remove("DEBUGINFOD_URLS")
        .arg(executable)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| {
            debugger_failure(format!("cannot run GNU objdump for assembly view: {err}"))
        })?;
    let mut stdout = BufReader::new(child.stdout.take().expect("objdump stdout is piped"));
    let result = parse_disassembly(&mut stdout, build_root, generated_files);
    drop(stdout);
    let status = child
        .wait()
        .map_err(|err| debugger_failure(format!("cannot wait for objdump: {err}")))?;
    let index =
        result.map_err(|err| debugger_failure(format!("cannot read objdump output: {err}")))?;
    if !status.success() {
        return Err(debugger_failure(format!("objdump failed: {status}")));
    }
    Ok(index)
}

fn parse_disassembly(
    mut stdout: impl BufRead,
    build_root: &Path,
    generated_files: &BTreeSet<&Path>,
) -> std::io::Result<Disassembly> {
    let mut text = Vec::new();
    let mut selected = None;
    let mut ordinal = 0;
    let mut result = BTreeMap::<&str, BTreeMap<usize, Vec<(usize, u64, String)>>>::new();
    loop {
        text.clear();
        if stdout.read_until(b'\n', &mut text)? == 0 {
            break;
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
                    .push((ordinal, address, instruction.to_owned()));
                ordinal += 1;
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
    }
    Ok(result
        .into_iter()
        .map(|(path, rows)| (path.to_owned(), rows))
        .collect())
}

fn line_marker(text: &str) -> Option<(&str, usize)> {
    let text = text
        .split_once(" (discriminator ")
        .map_or(text, |(prefix, _)| prefix);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disassembly_ordinals_span_archive_sections() {
        let text = b"first.o: file format elf64-x86-64\n\
            Disassembly of section .text.first:\n\
            0000000000000000 <first>:\n\
            /build/src/lib.rs:12\n\
            0:\tpush %rbp\n\
            /build/src/lib.rs:8\n\
            1:\tnop\n\
            Disassembly of section .text.second:\n\
            0000000000000000 <second>:\n\
            0:\tint3\n\
            /build/src/lib.rs:12\n\
            1:\tret\n\
            second.o: file format elf64-x86-64\n\
            Disassembly of section .text.third:\n\
            0000000000000000 <third>:\n\
            /build/src/lib.rs:8\n\
            0:\tret\n";
        let index = parse_disassembly(
            text.as_slice(),
            Path::new("/build"),
            &BTreeSet::from([Path::new("src/lib.rs")]),
        )
        .unwrap();
        let rows = &index["src/lib.rs"];
        assert_eq!(
            rows[&12],
            [
                (0, 0, String::from("push %rbp")),
                (2, 1, String::from("ret"))
            ],
        );
        assert_eq!(
            rows[&8],
            [(1, 1, String::from("nop")), (3, 0, String::from("ret"))],
        );
    }
}
