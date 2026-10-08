use base64::Engine as _;

use super::{BTreeMap, Backend, BigUint, CliFailure, ProvenanceManifest, StopContext, Value, json};

#[expect(
    clippy::too_many_lines,
    reason = "value translation keeps privacy, scope, layout, and response bounds in one reviewable boundary"
)]
pub(super) fn translate_variables(
    body: &mut Value,
    provenance: &ProvenanceManifest,
    parent_reference: i64,
    variable_objects: &mut BTreeMap<i64, String>,
    value_summaries: &BTreeMap<String, String>,
    stop_context: Option<&StopContext>,
) {
    const MAX_VARIABLES: usize = 100;
    const MAX_VALUE_BYTES: usize = 4_096;
    let mut bindings = BTreeMap::new();
    for binding in &provenance.debug.bindings {
        let visible = stop_context.is_none_or(|context| {
            binding.source.source_id == context.source_id
                && binding.visible_from <= context.position
                && binding.visible_until >= context.position
                && binding.function_id == context.function_id
                && binding
                    .scope_id
                    .as_ref()
                    .is_none_or(|scope| context.scope_ids.contains(scope))
        });
        if !visible {
            continue;
        }
        let rank = stop_context
            .and_then(|context| {
                binding
                    .scope_id
                    .as_ref()
                    .and_then(|scope| context.scope_ids.iter().position(|id| id == scope))
            })
            .unwrap_or(0);
        let replace = bindings.get(binding.rust_name.as_str()).is_none_or(
            |(current_rank, current): &(usize, &terrane_compiler::debugging::DebugBinding)| {
                (rank, binding.visible_from) > (*current_rank, current.visible_from)
            },
        );
        if replace {
            bindings.insert(binding.rust_name.as_str(), (rank, binding));
        }
    }
    let parent_object = variable_objects.get(&parent_reference).and_then(|id| {
        provenance
            .debug
            .objects
            .iter()
            .find(|object| object.id == *id)
    });
    let Some(variables) = body["variables"].as_array_mut() else {
        return;
    };
    let mut last_logical_variable = BTreeMap::new();
    for (index, variable) in variables.iter().enumerate() {
        let backend_name = variable["name"].as_str().unwrap_or_default();
        let logical_name = backend_name
            .split_once(" @ ")
            .map_or(backend_name, |(name, _)| name);
        if bindings.contains_key(logical_name) {
            last_logical_variable.insert(logical_name.to_owned(), index);
        }
    }
    let mut index = 0;
    variables.retain(|variable| {
        let backend_name = variable["name"].as_str().unwrap_or_default();
        let logical_name = backend_name
            .split_once(" @ ")
            .map_or(backend_name, |(name, _)| name);
        let retain = (!logical_name.contains("_terrane_f") || bindings.contains_key(logical_name))
            && last_logical_variable
                .get(logical_name)
                .is_none_or(|last| *last == index);
        index += 1;
        retain
    });
    let truncated_variables = variables.len().saturating_sub(MAX_VARIABLES);
    variables.truncate(MAX_VARIABLES);
    for variable in variables.iter_mut() {
        let backend_name = variable["name"].as_str().unwrap_or_default();
        let rust_name = backend_name
            .split_once(" @ ")
            .map_or(backend_name, |(name, _)| name);
        let union_storage = bindings
            .get(rust_name)
            .is_some_and(|(_, binding)| binding.physical_type_name.starts_with("Union("));
        let presentation = parent_object
            .and_then(|object| {
                object
                    .fields
                    .iter()
                    .find(|field| field.rust_name == rust_name)
                    .map(|field| {
                        (
                            field.name.clone(),
                            field.object_id.clone(),
                            field.secret,
                            field.type_name.as_str(),
                        )
                    })
            })
            .or_else(|| {
                bindings.get(rust_name).map(|(_, binding)| {
                    (
                        binding.name.clone(),
                        variable["terraneObjectId"]
                            .as_str()
                            .map(str::to_owned)
                            .or_else(|| binding.object_id.clone()),
                        false,
                        binding.type_name.as_str(),
                    )
                })
            });
        if let Some((name, object_id, secret, type_name)) = presentation {
            let is_object = object_id.is_some();
            let object_name = debug_object_name(object_id.as_deref(), type_name);
            variable["name"] = name.into();
            if secret {
                variable["value"] = "<secret>".into();
                variable["variablesReference"] = 0.into();
                variable["memoryReference"] = Value::Null;
                variable["evaluateName"] = Value::Null;
            } else {
                let reference = variable["variablesReference"].as_i64().unwrap_or(0);
                if reference != 0
                    && let Some(object_id) = object_id
                {
                    variable_objects.insert(reference, object_id);
                }
                let raw = variable["value"].as_str().unwrap_or_default();
                if is_object {
                    variable["value"] = object_name.into();
                } else if type_name == "Scalar(Int)" {
                    if let Some(summary) = variable["memoryReference"]
                        .as_str()
                        .and_then(|reference| value_summaries.get(reference))
                    {
                        variable["value"] = summary.clone().into();
                    }
                    variable["variablesReference"] = 0.into();
                } else if (matches!(type_name, "Scalar(String)" | "Bytes")
                    || type_name.starts_with("Union(")
                    || union_storage)
                    && let Some(summary) = variable["memoryReference"]
                        .as_str()
                        .and_then(|reference| value_summaries.get(reference))
                {
                    variable["value"] = summary.clone().into();
                    variable["variablesReference"] = 0.into();
                } else if raw.contains("optimized out") {
                    variable["value"] = "<optimized out>".into();
                } else if raw.contains("unavailable") {
                    variable["value"] = "<unavailable debug information>".into();
                }
            }
        }
        if let Some(value) = variable["value"].as_str()
            && value.len() > MAX_VALUE_BYTES
        {
            let mut end = MAX_VALUE_BYTES;
            while !value.is_char_boundary(end) {
                end -= 1;
            }
            variable["value"] = format!("{}… <truncated>", &value[..end]).into();
        }
    }
    if truncated_variables != 0 {
        variables.push(json!({
            "name": "…",
            "value": format!("<truncated: {truncated_variables} more values; request another DAP page>"),
            "variablesReference": 0
        }));
    }
}
pub(super) fn debug_object_name(object_id: Option<&str>, fallback: &str) -> String {
    object_id
        .and_then(|id| id.rsplit_once("::").map(|(_, name)| name))
        .unwrap_or(fallback)
        .to_owned()
}

pub(super) fn request_terrane_variables(
    backend: &mut Backend,
    arguments: Value,
    provenance: &ProvenanceManifest,
) -> Result<Value, CliFailure> {
    let _ = backend.request(
        "evaluate",
        json!({"expression": "`type category disable Rust", "context": "repl"}),
    );
    let result = backend
        .request("variables", arguments)
        .and_then(|mut response| {
            if let Some(variables) = response["body"]["variables"].as_array_mut() {
                for variable in variables {
                    let name = variable["name"]
                        .as_str()
                        .unwrap_or_default()
                        .split(" @ ")
                        .next()
                        .unwrap_or_default();
                    if let Some(binding) = provenance
                        .debug
                        .bindings
                        .iter()
                        .find(|binding| binding.rust_name == name)
                        && (binding.storage_may_be_unassigned
                            || binding.physical_type_name.starts_with("Union("))
                    {
                        unwrap_storage_variable(backend, variable, binding)?;
                    }
                }
            }
            Ok(response)
        });
    let _ = backend.request(
        "evaluate",
        json!({"expression": "`type category enable Rust", "context": "repl"}),
    );
    result
}

pub(super) fn unwrap_storage_variable(
    backend: &mut Backend,
    variable: &mut Value,
    binding: &terrane_compiler::debugging::DebugBinding,
) -> Result<(), CliFailure> {
    let original_name = variable["name"].clone();
    let evaluate_name = variable["evaluateName"].clone();
    let mut object_id = variable["terraneObjectId"].clone();
    let mut current = variable.clone();
    let mut payloads = usize::from(binding.storage_may_be_unassigned)
        + usize::from(binding.physical_type_name.starts_with("Union("));
    let mut visited = std::collections::BTreeSet::new();
    while payloads != 0 {
        let native_type = current["type"].as_str().unwrap_or_default();
        if native_type
            .rsplit("::")
            .next()
            .is_some_and(|name| name.split([':', '<']).next() == Some("None"))
        {
            variable["value"] = "<unassigned>".into();
            variable["variablesReference"] = 0.into();
            variable["memoryReference"] = Value::Null;
            return Ok(());
        }
        if let Some(arm) = native_type
            .rsplit_once("::Arm")
            .and_then(|(_, index)| index.split(':').next()?.parse::<usize>().ok())
        {
            object_id = binding
                .union_object_ids
                .get(arm)
                .and_then(Option::as_ref)
                .map_or(Value::Null, |id| id.clone().into());
        }
        let reference = current["variablesReference"].as_i64().unwrap_or(0);
        if reference == 0 || !visited.insert(reference) {
            break;
        }
        let response = backend.request("variables", json!({"variablesReference": reference}))?;
        let Some(children) = response["body"]["variables"].as_array() else {
            break;
        };
        let discriminant = children
            .iter()
            .find(|child| child["name"] == "$discr$")
            .and_then(native_discriminant);
        let mut selected = children
            .iter()
            .find(|child| child["name"] == "$variants$")
            .cloned();
        if selected.is_none() {
            for variant in children.iter().filter(|child| {
                child["name"]
                    .as_str()
                    .is_some_and(|name| name.starts_with("$variant$") && name != "$variant$")
            }) {
                let tag = variant["name"]
                    .as_str()
                    .and_then(|name| name.strip_prefix("$variant$"))
                    .and_then(|tag| tag.parse::<u32>().ok());
                let actual = if discriminant.is_some() {
                    discriminant
                } else {
                    let reference = variant["variablesReference"].as_i64().unwrap_or(0);
                    let nested =
                        backend.request("variables", json!({"variablesReference": reference}))?;
                    nested["body"]["variables"]
                        .as_array()
                        .and_then(|children| {
                            children.iter().find(|child| child["name"] == "$discr$")
                        })
                        .and_then(native_discriminant)
                };
                if tag.is_some() && tag == actual {
                    selected = Some(variant.clone());
                    break;
                }
            }
        }
        let selected = selected.or_else(|| {
            ["$variant$", "value", "__0"]
                .into_iter()
                .find_map(|name| children.iter().find(|child| child["name"] == name).cloned())
        });
        let Some(selected) = selected else {
            break;
        };
        if selected["name"] == "__0" {
            payloads -= 1;
        }
        current = selected;
    }
    if payloads == 0 {
        *variable = current;
        variable["name"] = original_name;
        variable["evaluateName"] = evaluate_name;
        if !object_id.is_null() {
            variable["terraneObjectId"] = object_id;
        }
    } else if binding.storage_may_be_unassigned {
        variable["value"] = "<unavailable debug information>".into();
        variable["variablesReference"] = 0.into();
        variable["memoryReference"] = Value::Null;
    }
    Ok(())
}

// LLDB's raw DWARF variant names contain a 32-bit discriminant, even for
// Rust niche tags wider than that. The value is read from the native storage,
// not inferred from source types or declaration order.
pub(super) fn native_discriminant(variable: &Value) -> Option<u32> {
    variable["value"]
        .as_str()?
        .parse::<i128>()
        .ok()
        .and_then(|value| u32::try_from(value & i128::from(u32::MAX)).ok())
}

pub(super) fn read_value_summaries(
    backend: &mut Backend,
    body: &Value,
    selected_layout: bool,
) -> BTreeMap<String, String> {
    let mut summaries = BTreeMap::new();
    for variable in body["variables"].as_array().into_iter().flatten() {
        let Some(reference) = variable["memoryReference"].as_str() else {
            continue;
        };
        let type_name = variable["type"].as_str().unwrap_or_default();
        let has_recipe = type_name == "terrane_int_support::Int"
            || type_name.contains("string::String")
            || type_name.contains("Vec<u8");
        if !has_recipe {
            continue;
        }
        if !selected_layout {
            continue;
        }
        let summary = if type_name == "terrane_int_support::Int" {
            decode_adaptive_int(backend, reference)
        } else if type_name.contains("string::String") {
            decode_string(backend, reference)
        } else {
            decode_bytes(backend, reference)
        };
        summaries.insert(
            reference.to_owned(),
            summary.unwrap_or_else(|failure| failure),
        );
    }
    summaries
}

pub(super) fn decode_string(backend: &mut Backend, reference: &str) -> Result<String, String> {
    let (bytes, truncated) = decode_vec_bytes(backend, reference)?;
    let text = String::from_utf8(bytes)
        .map_err(|_| "<unsupported layout: invalid string bytes>".to_owned())?;
    Ok(if truncated {
        format!("{text:?}… <truncated>")
    } else {
        format!("{text:?}")
    })
}

pub(super) fn decode_bytes(backend: &mut Backend, reference: &str) -> Result<String, String> {
    use std::fmt::Write as _;

    let (bytes, truncated) = decode_vec_bytes(backend, reference)?;
    let mut rendered = String::from("b'");
    for byte in bytes {
        write!(rendered, "\\\\x{byte:02x}").expect("writing to a string cannot fail");
    }
    rendered.push('\'');
    if truncated {
        rendered.push_str("… <truncated>");
    }
    Ok(rendered)
}

pub(super) fn decode_vec_bytes(
    backend: &mut Backend,
    reference: &str,
) -> Result<(Vec<u8>, bool), String> {
    const MAX_BYTES: usize = 4_096;
    let response = backend
        .request(
            "readMemory",
            json!({"memoryReference": reference, "offset": 0, "count": 24}),
        )
        .map_err(|failure| format!("<unavailable debug information: {}>", failure.message))?;
    let encoded = response["body"]["data"]
        .as_str()
        .ok_or_else(|| "<unavailable debug information>".to_owned())?;
    let header = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "<unsupported layout: invalid vector memory>".to_owned())?;
    if header.len() < 24 {
        return Err("<unavailable debug information>".to_owned());
    }
    let word = |index: usize| {
        let mut bytes = [0_u8; 8];
        bytes.copy_from_slice(&header[index * 8..index * 8 + 8]);
        u64::from_ne_bytes(bytes)
    };
    let pointer = word(1);
    let length = usize::try_from(word(2))
        .map_err(|_| "<unsupported layout: invalid vector length>".to_owned())?;
    let selected = length.min(MAX_BYTES);
    if selected == 0 {
        return Ok((Vec::new(), false));
    }
    let response = backend
        .request(
            "readMemory",
            json!({
                "memoryReference": format!("0x{pointer:x}"),
                "offset": 0,
                "count": selected
            }),
        )
        .map_err(|failure| format!("<unavailable debug information: {}>", failure.message))?;
    let encoded = response["body"]["data"]
        .as_str()
        .ok_or_else(|| "<unavailable debug information>".to_owned())?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "<unsupported layout: invalid vector contents>".to_owned())?;
    Ok((bytes, selected < length))
}

pub(super) fn decode_adaptive_int(
    backend: &mut Backend,
    reference: &str,
) -> Result<String, String> {
    const NICHE: u64 = 1_u64 << 63;
    let response = backend
        .request(
            "readMemory",
            json!({"memoryReference": reference, "offset": 0, "count": 32}),
        )
        .map_err(|failure| format!("<unavailable debug information: {}>", failure.message))?;
    let encoded = response["body"]["data"]
        .as_str()
        .ok_or_else(|| "<unavailable debug information>".to_owned())?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "<unsupported layout: invalid adaptive-int memory>".to_owned())?;
    if bytes.len() < 32 {
        return Err("<unavailable debug information>".to_owned());
    }
    let word = |index: usize| {
        u64::from_le_bytes(
            bytes[index * 8..index * 8 + 8]
                .try_into()
                .expect("bounded word slice"),
        )
    };
    match word(0) {
        NICHE => Ok(word(1).cast_signed().to_string()),
        tag if tag == NICHE + 1 => {
            let value = i128::from_le_bytes(bytes[16..32].try_into().expect("wide payload"));
            Ok(value.to_string())
        }
        _ => {
            let length = usize::try_from(word(2))
                .map_err(|_| "<unsupported layout: invalid bigint length>".to_owned())?;
            if length > 1_024 {
                return Err(format!("<truncated: adaptive int has {length} limbs>"));
            }
            let pointer = word(1);
            let magnitude = backend
                .request(
                    "readMemory",
                    json!({
                        "memoryReference": format!("0x{pointer:x}"),
                        "offset": 0,
                        "count": length.saturating_mul(8)
                    }),
                )
                .map_err(|failure| {
                    format!("<unavailable debug information: {}>", failure.message)
                })?;
            let encoded = magnitude["body"]["data"]
                .as_str()
                .ok_or_else(|| "<unavailable debug information>".to_owned())?;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|_| "<unsupported layout: invalid bigint memory>".to_owned())?;
            let magnitude = BigUint::from_bytes_le(&bytes).to_string();
            match word(3) & 0xff {
                0 => Ok(format!("-{magnitude}")),
                1 => Ok("0".to_owned()),
                2 => Ok(magnitude),
                _ => Err("<unsupported layout: unknown bigint sign>".to_owned()),
            }
        }
    }
}
