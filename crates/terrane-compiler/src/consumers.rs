use std::io::{Read as _, Write as _};
use std::process::{Command, Stdio};

use crate::{Package, SourceFile, SourceRole, source::Span};

const MAX_OUTPUT: usize = 16 * 1024 * 1024;
const MAX_INPUT: usize = 16 * 1024 * 1024;

#[derive(serde::Serialize)]
struct ConsumerInput<'a> {
    format: u32,
    package: &'a str,
    declarations: Vec<&'a crate::semantics::DeclarationMetadata>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumerOutput {
    format: u32,
    generated_sources: Vec<GeneratedSource>,
    diagnostics: Vec<ConsumerDiagnostic>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedSource {
    identity: String,
    source: String,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumerDiagnostic {
    declaration: crate::semantics::MetadataSpan,
    message: String,
}

pub(crate) fn run(
    package: &Package,
    semantic: &crate::SemanticPackage,
) -> Result<Vec<(String, String)>, crate::CompilationFailure> {
    let fallback = fallback_span(package);
    let mut generated = Vec::new();
    let mut generated_identities = std::collections::BTreeSet::new();
    for config in &package.consumer_configs {
        let mut available = std::collections::BTreeMap::new();
        let mut external_sources =
            std::collections::BTreeMap::<u32, (std::path::PathBuf, usize)>::new();
        let mut next_external_file = u32::MAX;
        for declaration in semantic.declarations() {
            if available
                .insert(declaration.identity.clone(), declaration.clone())
                .is_some()
            {
                return Err(failure(
                    package,
                    fallback,
                    format!(
                        "consumer `{}` sees duplicate canonical declaration identity `{}` in semantic metadata",
                        config.name, declaration.identity
                    ),
                ));
            }
        }
        for interface_path in &config.interfaces {
            let path = package.root.join(interface_path);
            let bytes = read_interface_bytes(&path).map_err(|error| {
                failure(
                    package,
                    fallback,
                    format!(
                        "consumer `{}` cannot read declaration interface `{}`: {error}",
                        config.name,
                        path.display()
                    ),
                )
            })?;
            if bytes.len() > MAX_OUTPUT {
                return Err(failure(
                    package,
                    fallback,
                    format!(
                        "consumer `{}` declaration interface `{}` exceeds the 16 MiB limit",
                        config.name,
                        path.display()
                    ),
                ));
            }
            let interface = load_declaration_interface(&bytes).map_err(|message| {
                failure(
                    package,
                    fallback,
                    format!(
                        "consumer `{}` has invalid declaration interface `{}`: {message}",
                        config.name,
                        path.display()
                    ),
                )
            })?;
            let mut remap = std::collections::BTreeMap::new();
            for (file, source_path) in &interface.sources {
                if source_path.trim().is_empty() {
                    return Err(failure(
                        package,
                        fallback,
                        format!(
                            "consumer `{}` interface has an empty source path for file {file}",
                            config.name
                        ),
                    ));
                }
                let id = next_external_file;
                next_external_file = next_external_file.checked_sub(1).ok_or_else(|| {
                    failure(
                        package,
                        fallback,
                        "too many declaration-interface source files".to_owned(),
                    )
                })?;
                remap.insert(*file, id);
                external_sources.insert(id, (std::path::PathBuf::from(source_path), 0));
            }
            for declaration in interface.declarations {
                let declaration = rebase_declaration(declaration, &remap, &mut external_sources)
                    .map_err(|message| failure(package, fallback, format!("consumer `{}` has invalid source references in declaration interface `{}`: {message}", config.name, path.display())))?;
                if available
                    .insert(declaration.identity.clone(), declaration.clone())
                    .is_some()
                {
                    return Err(failure(
                        package,
                        fallback,
                        format!(
                            "consumer `{}` sees duplicate canonical declaration identity `{}` across package/interface metadata",
                            config.name, declaration.identity
                        ),
                    ));
                }
            }
        }
        let mut selected = std::collections::BTreeSet::new();
        let declarations = config
            .declarations
            .iter()
            .map(|identity| {
                if !selected.insert(identity) {
                    return Err(failure(package, fallback, format!("consumer `{}` repeats declaration selector `{identity}`", config.name)));
                }
                available
                    .get(identity)
                    .ok_or_else(|| failure(package, fallback, format!("consumer `{}` selects unknown canonical declaration identity `{identity}`", config.name)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let input = ConsumerInput {
            format: 1,
            package: &package.identity,
            declarations,
        };
        let input = serde_json::to_vec(&input).expect("consumer input is serializable");
        if input.len() > MAX_INPUT {
            return Err(failure(
                package,
                fallback,
                format!("consumer `{}` input exceeds the 16 MiB limit", config.name),
            ));
        }
        let output = Command::new(&config.executable)
            .args(&config.arguments)
            .current_dir(&package.root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| {
                failure(
                    package,
                    fallback,
                    format!(
                        "cannot start configured consumer `{}`: {error}",
                        config.name
                    ),
                )
            })?
            .wait_with_input(&input, config, package, fallback)?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(failure(
                package,
                fallback,
                format!(
                    "consumer `{}` exited unsuccessfully: {}",
                    config.name,
                    stderr.trim()
                ),
            ));
        }
        let response: ConsumerOutput = serde_json::from_slice(&output.stdout).map_err(|error| {
            failure(
                package,
                fallback,
                format!(
                    "consumer `{}` returned malformed protocol JSON: {error}",
                    config.name
                ),
            )
        })?;
        if response.format != 1 {
            return Err(failure(
                package,
                fallback,
                format!(
                    "consumer `{}` returned unsupported protocol format {}",
                    config.name, response.format
                ),
            ));
        }
        for diagnostic in response.diagnostics {
            return Err(failure_at_metadata_span(package, diagnostic.declaration, &external_sources, diagnostic.message)
                .unwrap_or_else(|| failure(package, fallback, format!("consumer `{}` returned a diagnostic with an unknown source or invalid span", config.name))));
        }
        for source in response.generated_sources {
            if !valid_generated_identity(&source.identity) {
                return Err(failure(
                    package,
                    fallback,
                    format!(
                        "consumer `{}` returned invalid generated-source identity `{}`",
                        config.name, source.identity
                    ),
                ));
            }
            if !generated_identities.insert((config.name.clone(), source.identity.clone())) {
                return Err(failure(
                    package,
                    fallback,
                    format!(
                        "consumer `{}` returned duplicate generated-source identity `{}`",
                        config.name, source.identity
                    ),
                ));
            }
            generated.push((
                format!("{}::{}", config.name, source.identity),
                source.source,
            ));
        }
    }
    Ok(generated)
}

trait WaitWithInput {
    fn wait_with_input(
        self,
        input: &[u8],
        config: &crate::package::ConsumerConfig,
        package: &Package,
        fallback: Span,
    ) -> Result<std::process::Output, crate::CompilationFailure>;
}

impl WaitWithInput for std::process::Child {
    fn wait_with_input(
        mut self,
        input: &[u8],
        config: &crate::package::ConsumerConfig,
        package: &Package,
        fallback: Span,
    ) -> Result<std::process::Output, crate::CompilationFailure> {
        let stdin = self.stdin.take().expect("piped consumer stdin");
        let stdout = self.stdout.take().expect("piped consumer stdout");
        let stderr = self.stderr.take().expect("piped consumer stderr");
        let input = input.to_vec();
        let writer = std::thread::spawn(move || {
            let mut stdin = stdin;
            stdin.write_all(&input)
        });
        let stdout_reader = std::thread::spawn(move || read_bounded(stdout));
        let stderr_reader = std::thread::spawn(move || read_bounded(stderr));
        let status = self.wait().map_err(|error| {
            failure(
                package,
                fallback,
                format!(
                    "cannot collect output from consumer `{}`: {error}",
                    config.name
                ),
            )
        })?;
        let stdout = stdout_reader
            .join()
            .ok()
            .and_then(Result::ok)
            .ok_or_else(|| {
                failure(
                    package,
                    fallback,
                    format!("cannot read consumer `{}` stdout", config.name),
                )
            })?;
        let stderr = stderr_reader
            .join()
            .ok()
            .and_then(Result::ok)
            .ok_or_else(|| {
                failure(
                    package,
                    fallback,
                    format!("cannot read consumer `{}` stderr", config.name),
                )
            })?;
        let write_result = writer.join();
        if !matches!(write_result, Ok(Ok(()))) && status.success() {
            return Err(failure(
                package,
                fallback,
                format!("cannot send protocol input to consumer `{}`", config.name),
            ));
        }
        if stdout.len() > MAX_OUTPUT || stderr.len() > MAX_OUTPUT {
            return Err(failure(
                package,
                fallback,
                format!(
                    "consumer `{}` exceeded the 16 MiB output limit",
                    config.name
                ),
            ));
        }
        Ok(std::process::Output {
            status,
            stdout,
            stderr,
        })
    }
}

fn read_bounded(mut reader: impl std::io::Read) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(MAX_OUTPUT + 1);
    let mut buffer = [0u8; 8192];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        let retained = (MAX_OUTPUT + 1 - bytes.len()).min(count);
        bytes.extend_from_slice(&buffer[..retained]);
    }
    Ok(bytes)
}

fn load_declaration_interface(bytes: &[u8]) -> Result<crate::DeclarationInterface, String> {
    let json = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
    crate::DeclarationInterface::from_json(json)
}

fn read_interface_bytes(path: &std::path::Path) -> std::io::Result<Vec<u8>> {
    let file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take((MAX_OUTPUT + 1) as u64).read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn rebase_declaration(
    declaration: crate::semantics::DeclarationMetadata,
    file_ids: &std::collections::BTreeMap<u32, u32>,
    external_sources: &mut std::collections::BTreeMap<u32, (std::path::PathBuf, usize)>,
) -> Result<crate::semantics::DeclarationMetadata, String> {
    let mut value = serde_json::to_value(declaration).map_err(|error| error.to_string())?;
    rebase_span_values(&mut value, file_ids, external_sources)?;
    serde_json::from_value(value).map_err(|error| error.to_string())
}

fn rebase_span_values(
    value: &mut serde_json::Value,
    file_ids: &std::collections::BTreeMap<u32, u32>,
    external_sources: &mut std::collections::BTreeMap<u32, (std::path::PathBuf, usize)>,
) -> Result<(), String> {
    if let Some(object) = value.as_object_mut() {
        if object.contains_key("start")
            && object.contains_key("end")
            && let Some(file) = object.get("file").and_then(serde_json::Value::as_u64)
        {
            let old_file =
                u32::try_from(file).map_err(|_| "source file id exceeds u32".to_owned())?;
            let new_file = *file_ids.get(&old_file).ok_or_else(|| {
                format!("source file {old_file} is absent from interface source map")
            })?;
            if let Some((path, _)) = external_sources.get(&new_file) {
                object.insert(
                    "path".to_owned(),
                    serde_json::Value::String(path.to_string_lossy().into_owned()),
                );
            }
            let start = object
                .get("start")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| "source span start is invalid".to_owned())?;
            let end = object
                .get("end")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| "source span end is invalid".to_owned())?;
            let start = usize::try_from(start)
                .map_err(|_| "source span start exceeds address space".to_owned())?;
            let end = usize::try_from(end)
                .map_err(|_| "source span end exceeds address space".to_owned())?;
            if start > end || end > MAX_OUTPUT {
                return Err("source span is invalid or exceeds 16 MiB".to_owned());
            }
            object.insert("file".to_owned(), serde_json::Value::from(new_file));
            if let Some((_, length)) = external_sources.get_mut(&new_file) {
                *length = (*length).max(end);
            }
        }
        for child in object.values_mut() {
            rebase_span_values(child, file_ids, external_sources)?;
        }
    } else if let Some(array) = value.as_array_mut() {
        for child in array {
            rebase_span_values(child, file_ids, external_sources)?;
        }
    }
    Ok(())
}

fn failure_at_metadata_span(
    package: &Package,
    span: crate::semantics::MetadataSpan,
    external_sources: &std::collections::BTreeMap<u32, (std::path::PathBuf, usize)>,
    message: String,
) -> Option<crate::CompilationFailure> {
    if package
        .units
        .iter()
        .any(|unit| unit.source.id() == span.file)
        && let Some(span) = checked_source_span(package, &span)
    {
        return Some(failure(package, span, message));
    }
    let (path, length) = external_sources.get(&span.file)?;
    if (!span.path.is_empty() && span.path != path.to_string_lossy())
        || span.start > span.end
        || span.end > *length
    {
        return None;
    }
    let source = SourceFile::new(span.file, path.clone(), String::new());
    let mut diagnostic = crate::Diagnostic::error("S2060", message, Span::new(span.file, 0, 0));
    diagnostic.primary = None;
    diagnostic.help = Some(format!(
        "origin: {} bytes {}..{} (source unavailable; exported declaration metadata)",
        path.display(),
        span.start,
        span.end
    ));
    Some(crate::CompilationFailure {
        source,
        diagnostics: vec![diagnostic],
    })
}

fn checked_source_span(package: &Package, span: &crate::semantics::MetadataSpan) -> Option<Span> {
    let unit = package
        .units
        .iter()
        .find(|unit| unit.source.id() == span.file)?;
    let source = unit.source.text();
    let path = unit.source.path().to_string_lossy();
    if !span.path.is_empty() && span.path != unit.relative_path_text() && span.path != path {
        return None;
    }
    (span.start <= span.end
        && span.end <= source.len()
        && source.is_char_boundary(span.start)
        && source.is_char_boundary(span.end))
    .then_some(Span::new(span.file, span.start, span.end))
}

fn fallback_span(package: &Package) -> Span {
    package
        .units
        .first()
        .map_or(Span::new(0, 0, 0), |unit| Span::new(unit.source.id(), 0, 0))
}

pub(crate) fn add_generated_sources(package: &mut Package, sources: Vec<(String, String)>) {
    for (index, (_identity, text)) in sources.into_iter().enumerate() {
        let id = package.next_source_id();
        let relative_path =
            std::path::PathBuf::from(".trn/generated").join(format!("consumer-output-{index}.trn"));
        package.units.push(crate::SourceUnit {
            relative_path: relative_path.clone(),
            source: SourceFile::new(id, package.root.join(relative_path), text),
            expected_namespace: None,
            prelude: false,
            role: SourceRole::Production,
        });
    }
}

fn failure(package: &Package, span: Span, message: String) -> crate::CompilationFailure {
    let source = package
        .units
        .iter()
        .find(|unit| unit.source.id() == span.file)
        .or_else(|| package.units.first())
        .map_or_else(
            || SourceFile::new(0, package.root.clone(), String::new()),
            |unit| unit.source.clone(),
        );
    let span = if span.file == source.id() {
        span
    } else {
        Span::new(source.id(), 0, 0)
    };
    crate::CompilationFailure {
        source,
        diagnostics: vec![crate::Diagnostic::error("S2060", message, span)],
    }
}
fn valid_generated_identity(identity: &str) -> bool {
    !identity.is_empty()
        && identity
            .split('/')
            .all(|part| !part.is_empty() && !matches!(part, "." | ".."))
        && identity
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.' | '/'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Digest as _;

    #[test]
    fn response_protocol_rejects_unknown_fields_and_missing_payloads() {
        assert!(
            serde_json::from_str::<ConsumerOutput>(
                r#"{"format":1,"generated_sources":[],"diagnostics":[],"other":true}"#
            )
            .is_err()
        );
        assert!(serde_json::from_str::<ConsumerOutput>(r#"{"format":1}"#).is_err());
        assert!(serde_json::from_str::<ConsumerOutput>(
            r#"{"format":1,"generated_sources":[{"identity":"../escape","source":""}],"diagnostics":[]}"#
        )
        .is_ok());
    }

    #[test]
    fn generated_source_identities_reject_traversal_and_empty_components() {
        for invalid in [
            "",
            ".",
            "..",
            "../outside",
            "nested/../outside",
            "nested//file",
            "a b",
        ] {
            assert!(
                !valid_generated_identity(invalid),
                "accepted invalid identity {invalid:?}"
            );
        }
        assert!(valid_generated_identity("commands/help"));
    }

    #[test]
    fn consumer_diagnostic_spans_require_known_utf8_source_boundaries() {
        let package = Package::implicit("main.trn", "aéz".to_owned());
        let valid = crate::semantics::MetadataSpan {
            file: 0,
            path: "main.trn".to_owned(),
            start: 1,
            end: 3,
        };
        let split_character = crate::semantics::MetadataSpan {
            file: 0,
            path: "main.trn".to_owned(),
            start: 2,
            end: 3,
        };
        let unknown_source = crate::semantics::MetadataSpan {
            file: 42,
            path: "unknown.trn".to_owned(),
            start: 0,
            end: 0,
        };
        assert!(checked_source_span(&package, &valid).is_some());
        assert!(checked_source_span(&package, &split_character).is_none());
        assert!(checked_source_span(&package, &unknown_source).is_none());
    }

    #[test]
    fn declaration_interfaces_validate_format_and_metadata_fingerprint() {
        let mut interface = crate::DeclarationInterface {
            format: 1,
            package: "example/library".to_owned(),
            fingerprint: String::new(),
            sources: std::collections::BTreeMap::new(),
            declarations: Vec::new(),
        };
        interface.fingerprint = format!(
            "{:x}",
            sha2::Sha256::digest(serde_json::to_vec(&interface).unwrap())
        );
        let serialized = serde_json::to_vec(&interface).unwrap();
        assert!(load_declaration_interface(&serialized).is_ok());
        interface.package.push_str("/tampered");
        let tampered = serde_json::to_vec(&interface).unwrap();
        assert!(load_declaration_interface(&tampered).is_err());
    }

    #[test]
    fn process_output_reader_retains_only_limit_plus_one_while_draining() {
        let input = vec![b'x'; MAX_OUTPUT + 4096];
        let output = read_bounded(std::io::Cursor::new(input)).unwrap();
        assert_eq!(output.len(), MAX_OUTPUT + 1);
    }
}
