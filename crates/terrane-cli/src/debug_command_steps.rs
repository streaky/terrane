use super::{
    BTreeSet, Backend, BreakpointManager, CliFailure, MAX_RAW_STEPS, MAX_TEMPORARY_SEQUENCE_POINTS,
    NEXT_COMMAND_FILE, Ordering, Path, PathBuf, ProvenanceManifest, Value,
    backend::backend_console_output,
    debugger_failure, fs, generated_path, json,
    source::{association_for_frame, lexical_normalize},
};

#[derive(Clone, Eq, PartialEq)]
pub(super) struct StopLocation {
    frame_depth: usize,
    generated_path: String,
    generated_line: usize,
    function_id: Option<String>,
}

pub(super) fn mapped_stop_location(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    frame_index: usize,
) -> Result<Option<StopLocation>, CliFailure> {
    let response = backend.request(
        "stackTrace",
        json!({"threadId": thread_id, "startFrame": 0}),
    )?;
    let frames = response["body"]["stackFrames"].as_array();
    let frame_depth = response["body"]["totalFrames"]
        .as_u64()
        .and_then(|depth| usize::try_from(depth).ok())
        .unwrap_or_else(|| frames.map_or(0, Vec::len));
    Ok(frames
        .and_then(|frames| frames.get(frame_index))
        .and_then(|frame| {
            let association = association_for_frame(provenance, frame)?;
            Some(StopLocation {
                frame_depth: frame_depth.saturating_sub(frame_index),
                generated_path: frame["source"]["path"].as_str()?.to_owned(),
                generated_line: association.generated.line,
                function_id: association.function_id.clone(),
            })
        }))
}

#[expect(
    clippy::too_many_lines,
    reason = "temporary breakpoint ownership and cleanup remain visibly paired"
)]
pub(super) fn temporary_sequence_step(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    breakpoints: &BreakpointManager,
    thread_id: i64,
) -> Result<Option<Value>, CliFailure> {
    let response = backend.request(
        "stackTrace",
        json!({"threadId": thread_id, "startFrame": 0}),
    )?;
    let Some(frames) = response["body"]["stackFrames"].as_array() else {
        return Ok(None);
    };
    let Some(frame) = frames.first() else {
        return Ok(None);
    };
    let Some(current) = association_for_frame(provenance, frame) else {
        return Ok(None);
    };
    let origin_depth = response["body"]["totalFrames"]
        .as_u64()
        .and_then(|depth| usize::try_from(depth).ok())
        .unwrap_or(frames.len());
    let caller = frames
        .get(1)
        .and_then(|frame| association_for_frame(provenance, frame));
    let mut targets = BTreeSet::new();
    for file in &provenance.debug.generated_files {
        for association in &file.associations {
            if !association.sequence_point {
                continue;
            }
            let same_current_point = association.causes.iter().any(|candidate| {
                current.causes.iter().any(|cause| {
                    candidate.source_id == cause.source_id && candidate.line == cause.line
                })
            }) || (file.path
                == frame["source"]["path"].as_str().unwrap_or_default()
                && association.generated.line == current.generated.line);
            let same_function = association.function_id == current.function_id;
            let caller_function = caller.is_some_and(|caller| {
                association.function_id == caller.function_id
                    && association.generated.line != caller.generated.line
            });
            if !same_current_point && (same_function || caller_function) {
                targets.insert((file.path.clone(), association.generated.line));
            }
        }
    }
    if targets.is_empty() {
        return Ok(None);
    }
    if targets.len() > MAX_TEMPORARY_SEQUENCE_POINTS {
        return Ok(None);
    }
    let target_locations = targets
        .iter()
        .map(|(path, line)| (lexical_normalize(&generated_path(provenance, path)), *line))
        .collect::<BTreeSet<_>>();
    let current_file = frame["source"]["path"].as_str().unwrap_or_default();
    let suspended_ids = breakpoints.backend_ids_at(current_file, current.generated.line);

    let existing_ids = backend_breakpoint_ids(backend)?;
    let command_file = TemporaryBreakpointCommands::write(provenance, &targets)?;
    let response = backend.request(
        "evaluate",
        json!({
            "expression": format!(
                "`command source -s 0 {}",
                lldb_quote(&command_file.path.to_string_lossy())
            ),
            "context": "repl"
        }),
    )?;
    let _ = backend_console_output(backend, &response);
    let temporary_ids = backend_breakpoint_ids(backend)?
        .difference(&existing_ids)
        .copied()
        .collect::<Vec<_>>();
    if temporary_ids.len() != targets.len() {
        let _ = delete_backend_breakpoints(backend, &temporary_ids);
        let _ = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
        return Ok(None);
    }
    if let Err(failure) = set_backend_breakpoints_enabled(backend, &suspended_ids, false) {
        let _ = delete_backend_breakpoints(backend, &temporary_ids);
        let _ = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
        return Err(failure);
    }
    let result = (|| {
        backend.request(
            "continue",
            json!({"threadId": thread_id, "singleThread": false}),
        )?;
        loop {
            let mut event = backend.wait_for_event(&["stopped", "terminated"])?;
            if event["event"] != "stopped" {
                return Ok(event);
            }
            let reason = event["body"]["reason"].as_str().unwrap_or_default();
            if reason != "breakpoint" {
                return Ok(event);
            }
            let hit_ids = event["body"]["hitBreakpointIds"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_i64)
                .collect::<BTreeSet<_>>();
            if hit_ids
                .iter()
                .any(|id| breakpoints.contains_backend_id(*id))
            {
                return Ok(event);
            }
            let location = mapped_stop_location(backend, provenance, thread_id, 0)?;
            let at_temporary_location = location.as_ref().is_some_and(|location| {
                target_locations.contains(&(
                    lexical_normalize(Path::new(&location.generated_path)),
                    location.generated_line,
                ))
            });
            let hit_temporary = hit_ids.iter().any(|id| temporary_ids.contains(id))
                || (hit_ids.is_empty() && at_temporary_location);
            if !hit_temporary {
                return Ok(event);
            }
            if location.is_some_and(|location| location.frame_depth <= origin_depth) {
                event["body"]["reason"] = "step".into();
                if let Some(body) = event["body"].as_object_mut() {
                    body.remove("description");
                    body.remove("hitBreakpointIds");
                }
                return Ok(event);
            }
            backend.request(
                "continue",
                json!({"threadId": thread_id, "singleThread": false}),
            )?;
        }
    })();
    if !temporary_ids.is_empty() {
        let _ = delete_backend_breakpoints(backend, &temporary_ids);
    }
    let _ = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
    result.map(Some)
}

pub(super) struct TemporaryBreakpointCommands {
    path: PathBuf,
}

impl TemporaryBreakpointCommands {
    pub(super) fn write(
        provenance: &ProvenanceManifest,
        targets: &BTreeSet<(String, usize)>,
    ) -> Result<Self, CliFailure> {
        let sequence = NEXT_COMMAND_FILE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "terrane-debug-breakpoints-{}-{sequence}.lldb",
            std::process::id()
        ));
        let commands = targets
            .iter()
            .map(|(relative, line)| {
                let path = generated_path(provenance, relative);
                format!(
                    "breakpoint set --file {} --line {line}",
                    lldb_quote(&path.to_string_lossy())
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&path, format!("{commands}\n")).map_err(|error| {
            debugger_failure(format!(
                "cannot write temporary debugger commands {}: {error}",
                path.display()
            ))
        })?;
        Ok(Self { path })
    }
}

impl Drop for TemporaryBreakpointCommands {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub(super) fn backend_breakpoint_ids(backend: &mut Backend) -> Result<BTreeSet<i64>, CliFailure> {
    let response = backend.request(
        "evaluate",
        json!({"expression": "`breakpoint list -b", "context": "repl"}),
    )?;
    Ok(
        parse_lldb_breakpoint_ids(&backend_console_output(backend, &response))
            .into_iter()
            .collect(),
    )
}

pub(super) fn delete_backend_breakpoints(
    backend: &mut Backend,
    ids: &[i64],
) -> Result<(), CliFailure> {
    if ids.is_empty() {
        return Ok(());
    }
    let ids = ids.iter().map(i64::to_string).collect::<Vec<_>>().join(" ");
    backend.request(
        "evaluate",
        json!({
            "expression": format!("`breakpoint delete {ids}"),
            "context": "repl"
        }),
    )?;
    Ok(())
}

pub(super) fn set_backend_breakpoints_enabled(
    backend: &mut Backend,
    ids: &[i64],
    enabled: bool,
) -> Result<(), CliFailure> {
    if ids.is_empty() {
        return Ok(());
    }
    let ids = ids.iter().map(i64::to_string).collect::<Vec<_>>().join(" ");
    backend.request(
        "evaluate",
        json!({
            "expression": format!(
                "`breakpoint {} {ids}",
                if enabled { "enable" } else { "disable" }
            ),
            "context": "repl"
        }),
    )?;
    Ok(())
}

pub(super) fn parse_lldb_breakpoint_ids(output: &str) -> Vec<i64> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let identifier = line
                .strip_prefix("Breakpoint ")
                .unwrap_or(line)
                .split_once(':')?
                .0;
            identifier.parse().ok()
        })
        .collect()
}

pub(super) fn step_to_source(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    breakpoints: &BreakpointManager,
    command: &str,
    thread_id: i64,
) -> Result<Value, CliFailure> {
    if command == "next"
        && let Some(event) = temporary_sequence_step(backend, provenance, breakpoints, thread_id)?
    {
        return Ok(event);
    }
    let origin = mapped_stop_location(backend, provenance, thread_id, 0)?;
    let caller = if command == "stepOut" {
        mapped_stop_location(backend, provenance, thread_id, 1)?
    } else {
        None
    };
    let suspended_ids = origin.as_ref().map_or_else(Vec::new, |origin| {
        breakpoints.backend_ids_at(&origin.generated_path, origin.generated_line)
    });
    set_backend_breakpoints_enabled(backend, &suspended_ids, false)?;
    let result = (|| {
        let mut native_command = command;
        for _ in 0..MAX_RAW_STEPS {
            backend.request(
                native_command,
                json!({"threadId": thread_id, "singleThread": true, "granularity": "statement"}),
            )?;
            let event = backend.wait_for_event(&["stopped", "terminated"])?;
            if event["event"] != "stopped" {
                return Ok(event);
            }
            let reason = event["body"]["reason"].as_str().unwrap_or_default();
            if !matches!(reason, "step" | "entry" | "") {
                return Ok(event);
            }
            if command == "stepOut" {
                native_command = "next";
            }
            let Some(current) = mapped_stop_location(backend, provenance, thread_id, 0)? else {
                continue;
            };
            let Some(origin) = &origin else {
                return Ok(event);
            };
            let changed_point = current.generated_path != origin.generated_path
                || current.generated_line != origin.generated_line
                || current.function_id != origin.function_id;
            let reached_source_target = match command {
                "next" => {
                    current.frame_depth < origin.frame_depth
                        || (current.frame_depth == origin.frame_depth && changed_point)
                }
                "stepOut" => {
                    current.frame_depth < origin.frame_depth
                        && caller.as_ref().is_none_or(|caller| current != *caller)
                }
                _ => current.frame_depth > origin.frame_depth || changed_point,
            };
            if reached_source_target {
                return Ok(event);
            }
        }
        eprintln!(
            "source progress unavailable after {MAX_RAW_STEPS} bounded native steps; exposing the native stop"
        );
        Ok(json!({
            "seq": 0,
            "type": "event",
            "event": "stopped",
            "body": {"reason": "step", "threadId": thread_id}
        }))
    })();
    let restored = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
    match result {
        Ok(event) => {
            restored?;
            Ok(event)
        }
        Err(failure) => Err(failure),
    }
}

pub(super) fn lldb_quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
