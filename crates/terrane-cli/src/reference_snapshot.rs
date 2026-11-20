use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
    path::Path,
    process::ExitCode,
};

use serde_json::{Value, json};
use terrane_compiler::rust_interop::projection::{ProjectedKind, ProjectedType};
use terrane_compiler::semantics::{FunctionContract, SemanticUnit, SymbolKind};
use terrane_compiler::{
    DeclarationKind, DeclarationMetadata, Package, SemanticPackage, ValueType, Visibility,
};

use crate::CliFailure;

const USAGE: &str = "usage: terrane reference-snapshot <package-or-source> --scope <namespace|all> --output <snapshot.json>";

fn manual_id(identity: &str) -> String {
    format!(
        "api.{}",
        identity
            .trim_start_matches('/')
            .replace("::", ".")
            .replace('/', ".")
            .replace('_', "-")
            .to_ascii_lowercase()
    )
}

fn valid_id(id: &str) -> bool {
    id.split('.').all(|segment| {
        segment
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_lowercase)
            && segment
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    })
}

fn type_ref(ty: Option<&ValueType>) -> Value {
    let display = ty.map_or_else(|| "none".to_owned(), ToString::to_string);
    json!({"display":display,"entity":null,"arguments":[],"nullable": matches!(ty, Some(ValueType::Optional(_)))})
}

fn source_default(metadata: Option<&Value>) -> Option<String> {
    let default = metadata?.get("default")?;
    let value = default.get("value");
    match default["kind"].as_str()? {
        "none" => Some("none".to_owned()),
        "boolean" => value?.as_bool().map(|value| value.to_string()),
        "integer" | "float" => value?.as_str().map(str::to_owned),
        "string" => value?
            .as_str()
            .map(|value| format!("{:?}", value.replace('{', "{{").replace('}', "}}"))),
        _ => None,
    }
}

fn signature(function: &FunctionContract, record: &DeclarationMetadata) -> Option<Value> {
    if !function.generic_parameters.is_empty() {
        return None;
    }
    let stored = record.signature.as_ref()?.get("parameters")?.as_array()?;
    let construct = function.name == "construct" && function.owner.is_some();
    let mut display = String::new();
    for (enabled, prefix) in [
        (function.is_unsafe, "unsafe "),
        (function.is_async, "async "),
        (function.is_static, "static "),
    ] {
        if enabled {
            display.push_str(prefix);
        }
    }
    let mode = match function.written_invocation_mode {
        terrane_compiler::InvocationMode::Shared => "none",
        terrane_compiler::InvocationMode::Mutable => "mutable",
        terrane_compiler::InvocationMode::Consuming => "consuming",
    };
    if mode != "none" {
        display.push_str(mode);
        display.push(' ');
    }
    display.push_str("function ");
    display.push_str(&function.name);
    if let Some(result) = &function.return_type {
        display.push(' ');
        display.push_str(&result.to_string());
    }
    if function.throws {
        display.push_str(" throws throwable");
    }
    display.push(';');
    let mut parameters = Vec::new();
    for (index, parameter) in function.parameters.iter().enumerate() {
        let ty = parameter.value_type.as_ref()?;
        // Never emit a plausible signature for a shape that this renderer does not support.
        if parameter.variadic || parameter.mutable {
            return None;
        }
        let metadata = stored.get(index)?;
        let default = source_default(Some(metadata));
        if !metadata["default"].is_null() && default.is_none() {
            return None;
        }
        display.push_str(if index == 0 { " " } else { ", " });
        display.push_str(&parameter.name);
        display.push(' ');
        display.push_str(&ty.to_string());
        if let Some(default) = &default {
            display.push_str(" = ");
            display.push_str(default);
        }
        let passing = match ty {
            ValueType::Reference(_) => "ref",
            ValueType::MutableReference(_) => "mutable ref",
            ValueType::SharedReference(_) => "shared ref",
            _ => "value",
        };
        parameters.push(json!({"id":parameter.name,"name":parameter.name,"type":type_ref(Some(ty)),"mode":if parameter.optional {"optional"} else {"required"},"default":default,"passing":passing}));
    }
    if function.parameters.is_empty() {
        display.pop();
    }
    let throws = if function.throws && function.thrown_types.is_empty() {
        vec![
            json!({"id":"throwable", "type":{"display":"throwable","entity":null,"arguments":[],"nullable":false}}),
        ]
    } else {
        function.thrown_types.iter().enumerate().map(|(index, ty)| json!({"id":format!("throwable-{}",index+1),"type":type_ref(Some(ty))})).collect()
    };
    Some(
        json!({"id":if construct {"construct"} else {"default"},"form":if construct {"construct"} else {"declaration"},"display":display,"receiver":if function.is_static || construct {None} else {function.owner.as_ref()},"parameters":parameters,"returns":{"type":type_ref(function.return_type.as_ref())},"throws":throws,"contracts":{"asynchronous":function.is_async,"receiver-mutation":mode}}),
    )
}

fn surface(
    symbol: &str,
    namespace: Option<&str>,
    owner: Option<&str>,
    category: &str,
    package: &str,
) -> Value {
    json!({"symbol":symbol,"aliases":[],"namespace":namespace,"owner":owner,"visibility":"public","availability":{"targets":[std::env::consts::OS],"feature":null},"origin":{"category":category,"package":package}})
}

fn insert(
    entities: &mut BTreeMap<String, Value>,
    id: String,
    kind: &str,
    surface: Value,
) -> Result<(), CliFailure> {
    if !valid_id(&id) || entities.contains_key(&id) {
        return Err(CliFailure::package(format!(
            "unrepresentable or colliding reference identity: {id}"
        )));
    }
    entities.insert(id.clone(), json!({"id":id,"kind":kind,"surface":surface}));
    Ok(())
}

fn declaration<'a>(
    unit: &'a SemanticUnit,
    metadata: &DeclarationMetadata,
) -> Option<&'a FunctionContract> {
    unit.functions.iter().find(|contract| {
        contract.span.file == metadata.span.file
            && contract.span.start == metadata.span.start
            && contract.span.end == metadata.span.end
    })
}

fn append_namespace(
    semantic: &SemanticPackage,
    namespace: &str,
    package: &str,
    category: &str,
    entities: &mut BTreeMap<String, Value>,
    omissions: &mut BTreeSet<String>,
    authored_sources: Option<&BTreeSet<u32>>,
) -> Result<(), CliFailure> {
    let namespace_id = manual_id(namespace);
    let Some(semantic_namespace) = semantic.namespaces.get(namespace) else {
        return Ok(());
    };
    let public: BTreeMap<_, _> = semantic_namespace
        .symbols
        .values()
        .filter(|symbol| symbol.visibility == Visibility::Public)
        .map(|symbol| (symbol.identity.as_str(), symbol.kind))
        .collect();
    let projected_functions: BTreeMap<_, _> = semantic
        .projection
        .dependencies
        .iter()
        .flat_map(|dependency| dependency.items.iter())
        .filter(|item| item.namespace == namespace)
        .filter_map(|item| match &item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                Some((item.name.as_str(), (item, function)))
            }
            _ => None,
        })
        .collect();
    let mut members = BTreeMap::new();
    let mut exported = BTreeSet::new();
    for unit in semantic.units.iter().filter(|unit| {
        unit.namespace == namespace
            && authored_sources.is_none_or(|ids| ids.contains(&unit.source.id()))
    }) {
        for metadata in unit
            .declaration_metadata
            .iter()
            .filter(|metadata| metadata.visibility == "public")
        {
            let identity = metadata.identity.as_str();
            let Some((parent, name)) = identity.rsplit_once("::") else {
                continue;
            };
            let owner = (parent != namespace).then(|| manual_id(parent));
            if owner.is_some() && public.get(parent) != Some(&SymbolKind::Class) {
                continue;
            }
            if owner.is_none() && !public.contains_key(identity) {
                continue;
            }
            let id = manual_id(identity);
            let mut data = surface(
                identity,
                owner.is_none().then_some(&namespace_id),
                owner.as_deref(),
                category,
                package,
            );
            let kind = match metadata.kind {
                DeclarationKind::Callable => {
                    let Some(mut sig) = declaration(unit, metadata)
                        .and_then(|function| signature(function, metadata))
                    else {
                        omissions.insert(identity.to_owned());
                        continue;
                    };
                    if owner.is_none()
                        && let Some((item, projected)) = projected_functions.get(name)
                    {
                        data["origin"]["upstream"] = json!(item.rust_path);
                        if projected.destination_result.is_some() {
                            let display = match &projected.result {
                                ProjectedType::Generic(name) => name.clone(),
                                value => value.terrane_name(),
                            };
                            sig["returns"]["type"]["display"] = json!(display);
                            sig["returns"]["type"]["nullable"] =
                                json!(matches!(projected.result, ProjectedType::Optional(_)));
                            sig["contracts"]["destination-selected-result"] = json!(true);
                        }
                    }
                    if name == "construct" && owner.is_some() {
                        members.insert(parent, sig);
                        continue;
                    }
                    data["signatures"] = json!([sig]);
                    if owner.is_some() {
                        "method"
                    } else {
                        "function"
                    }
                }
                DeclarationKind::Class => {
                    let Some(descriptor) = unit.descriptors.iter().find(|descriptor| {
                        descriptor.identity.namespace == namespace
                            && descriptor.identity.name == name
                    }) else {
                        omissions.insert(identity.to_owned());
                        continue;
                    };
                    if !descriptor.generic_parameters.is_empty()
                        || descriptor.base.is_some()
                        || !descriptor.interfaces.is_empty()
                        || !descriptor.traits.is_empty()
                    {
                        omissions.insert(identity.to_owned());
                    }
                    data["ownership"] = json!(if descriptor.resource_owning {
                        "resource"
                    } else {
                        "value"
                    });
                    "class"
                }
                DeclarationKind::Field => {
                    let Some(field) = unit
                        .descriptors
                        .iter()
                        .filter(|descriptor| {
                            descriptor.identity.namespace == namespace
                                && identity
                                    .strip_prefix(namespace)
                                    .and_then(|value| value.strip_prefix("::"))
                                    .and_then(|value| value.split_once("::"))
                                    .is_some_and(|(class, _)| class == descriptor.identity.name)
                        })
                        .find_map(|descriptor| {
                            descriptor.fields.iter().find(|field| field.name == name)
                        })
                    else {
                        omissions.insert(identity.to_owned());
                        continue;
                    };
                    data["value-type"] = type_ref(Some(&field.value_type));
                    data["static"] = json!(field.is_static);
                    data["readable"] = json!(true);
                    "field"
                }
                DeclarationKind::Parameter => continue,
            };
            exported.insert(identity.to_owned());
            insert(entities, id, kind, data)?;
        }
    }
    for (owner, sig) in members {
        if let Some(class) = entities.get_mut(&manual_id(owner)) {
            class["surface"]["construct-signature"] = json!("construct");
            class["surface"]["signatures"] = json!([sig]);
        } else {
            omissions.insert(owner.to_owned());
        }
    }
    omissions.extend(
        public
            .keys()
            .filter(|symbol| !exported.contains(**symbol))
            .map(|symbol| (*symbol).to_owned()),
    );
    if !exported.is_empty() {
        insert(
            entities,
            namespace_id,
            "namespace",
            surface(namespace, None, None, category, package),
        )?;
    }
    Ok(())
}

pub(crate) fn run(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    if arguments.len() != 6 || arguments[2] != "--scope" || arguments[4] != "--output" {
        return Err(CliFailure::usage_with(USAGE));
    }
    let input = Path::new(&arguments[1]);
    let scope = arguments[3]
        .to_str()
        .filter(|scope| *scope == "all" || scope.starts_with('/'))
        .ok_or_else(|| CliFailure::usage_with(USAGE))?;
    let is_manifest = input.is_dir()
        || input
            .extension()
            .is_some_and(|extension| extension == "toml");
    let package = if is_manifest {
        Package::load(input).map_err(|errors| {
            CliFailure::package(
                errors
                    .into_iter()
                    .map(|error| error.diagnostic.render(&error.source))
                    .collect::<String>(),
            )
        })?
    } else {
        Package::implicit(
            input,
            fs::read_to_string(input)
                .map_err(|error| CliFailure::package(format!("{}: {error}", input.display())))?,
        )
    };
    let semantic = terrane_compiler::with_compilation_cache(|| terrane_compiler::analyze(&package))
        .map_err(|failure| {
            CliFailure::package(
                failure
                    .diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.render(&failure.source))
                    .collect::<String>(),
            )
        })?;
    let mut entities = BTreeMap::new();
    let mut omissions = BTreeSet::new();
    let authored_sources: BTreeSet<_> = package
        .units
        .iter()
        .filter(|unit| {
            unit.role == terrane_compiler::SourceRole::Production
                && !package.library_source_ids.contains(&unit.source.id())
        })
        .map(|unit| unit.source.id())
        .collect();
    let namespaces: BTreeSet<_> = semantic
        .units
        .iter()
        .filter(|unit| {
            if scope == "all" {
                authored_sources.contains(&unit.source.id())
            } else {
                unit.namespace == scope
            }
        })
        .map(|unit| unit.namespace.as_str())
        .collect();
    if scope != "all" && namespaces.is_empty() {
        return Err(CliFailure::package(format!(
            "scope {scope} was not loaded by input; import it in the input package"
        )));
    }
    for namespace in namespaces {
        let (origin, owner) = if namespace.starts_with("/core/") {
            ("compiler", "core")
        } else if namespace.starts_with("/deps/") {
            (
                "dependency",
                namespace.split('/').nth(2).unwrap_or(namespace),
            )
        } else {
            ("package", semantic.identity.as_str())
        };
        append_namespace(
            &semantic,
            namespace,
            owner,
            origin,
            &mut entities,
            &mut omissions,
            (scope == "all").then_some(&authored_sources),
        )?;
    }
    let declaration_view = semantic.projection.documented_inventory(&BTreeMap::new());
    // `all` describes the selected manifest package's authored namespaces. It is
    // never a completeness claim for bundled core or demand-admitted projections.
    let complete = scope == "all"
        && is_manifest
        && omissions.is_empty()
        && package.consumer_configs.is_empty();
    let profile = json!({"coverage":"authored-package-v1","package":semantic.identity,"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"profile":semantic.profile,"prelude":semantic.prelude}).to_string();
    let snapshot = json!({"format":1,"scope":scope,"profile":profile,"complete":complete,"declaration_view":declaration_view,"entities":entities.into_values().collect::<Vec<_>>(),"omissions":omissions});
    let output = Path::new(&arguments[5]);
    fs::write(
        output,
        serde_json::to_vec_pretty(&snapshot)
            .map_err(|error| CliFailure::package(error.to_string()))?,
    )
    .map_err(|error| CliFailure::package(format!("{}: {error}", output.display())))?;
    Ok(ExitCode::SUCCESS)
}
