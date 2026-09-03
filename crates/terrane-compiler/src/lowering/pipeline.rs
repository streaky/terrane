use super::prelude::*;

pub(super) fn module_destination(unit: &SemanticUnit) -> ModuleDestination {
    if unit.bundled || unit.namespace == "/deps" || unit.namespace.starts_with("/deps/") {
        ModuleDestination::Support
    } else {
        ModuleDestination::Application
    }
}
#[expect(
    clippy::too_many_lines,
    reason = "package lowering assembles one deterministic generated-crate prelude and unit set"
)]
pub(crate) fn lower(package: &SemanticPackage) -> Program {
    let mut runtime = Vec::new();
    let mut globals = String::new();
    let registry = LoweringRegistry::default();
    let uses_errors = package_uses_structured_errors(package) || package_uses_task_scope(package);
    let has_dependency = package
        .units
        .iter()
        .any(|unit| unit.namespace.starts_with("/deps/") && !unit.functions.is_empty());
    let has_custom_throwable = has_dependency
        || package.units.iter().any(|unit| {
            unit.objects.iter().any(|object| {
                object.interfaces.iter().any(|interface| {
                    interface.namespace == "/core/errors" && interface.name == "throwable"
                })
            })
        });
    if has_dependency {
        registry.register_descriptor("/core/errors::dependency-error", "dependency-error");
        registry.register_descriptor("/core/errors::dependency-panic", "dependency-panic");
    }
    if package
        .units
        .iter()
        .any(|unit| unit.functions.iter().any(|function| function.is_async))
    {
        let mut support = include_str!("../runtime/async.rs").to_owned();
        if package_uses_task_scope(package) {
            support.push_str(include_str!("../runtime/async_cancellable.rs"));
        }
        runtime.push(GeneratedModule {
            name: "async",
            items: vec![Item::generated(&support)],
        });
    }
    if package_uses_task_scope(package) {
        let support = match package.executor {
            crate::package::ExecutorProfile::Cooperative => {
                include_str!("../runtime/tasks_cooperative.rs")
            }
            crate::package::ExecutorProfile::Threaded => {
                include_str!("../runtime/tasks_threaded.rs")
            }
        };
        runtime.push(GeneratedModule {
            name: "tasks",
            items: vec![Item::generated(support)],
        });
    }
    let mut uses_streams = false;
    let mut uses_filesystem = false;
    let mut uses_process = false;
    let mut uses_documents = false;
    let mut uses_json = false;
    let mut uses_yaml = false;
    let mut uses_urls = false;
    let mut uses_random = false;
    let mut uses_codecs = false;
    let mut uses_compression = false;
    let mut uses_uuid = false;
    let mut uses_networking = false;
    let mut uses_tls = false;
    let mut uses_concurrency = false;
    for unit in &package.units {
        match unit.namespace.as_str() {
            "/core/streams" => uses_streams = true,
            "/core/filesystem" => uses_filesystem = true,
            "/core/process" => uses_process = true,
            "/core/documents" => uses_documents = true,
            "/core/documents/json" => uses_json = true,
            "/core/documents/yaml" => uses_yaml = true,
            "/core/urls" => uses_urls = true,
            "/core/random" => uses_random = true,
            "/core/codecs" => uses_codecs = true,
            "/core/compression" => uses_compression = true,
            "/core/random/uuid" => uses_uuid = true,
            "/core/networking" => uses_networking = true,
            "/core/networking/tls" => uses_tls = true,
            "/core/concurrency" => uses_concurrency = true,
            _ => {}
        }
    }
    let uses_platform_capabilities = uses_random
        || uses_codecs
        || uses_compression
        || uses_uuid
        || uses_networking
        || uses_tls
        || uses_concurrency;
    let requires_platform_support = uses_streams
        || uses_filesystem
        || uses_process
        || uses_documents
        || uses_json
        || uses_yaml
        || uses_urls
        || uses_platform_capabilities;
    if uses_streams || uses_filesystem {
        let mut items = vec![Item::generated(include_str!(
            "../runtime/platform_streams.rs"
        ))];
        if uses_streams {
            items.push(Item::generated(include_str!(
                "../runtime/platform_standard_streams.rs"
            )));
        }
        if uses_filesystem {
            items.push(Item::generated(include_str!(
                "../runtime/platform_files.rs"
            )));
        }
        runtime.push(GeneratedModule {
            name: "platform_streams",
            items,
        });
    }
    if uses_filesystem || uses_process {
        let mut items = Vec::new();
        if uses_filesystem {
            items.push(Item::generated(include_str!(
                "../runtime/platform_system.rs"
            )));
        }
        if uses_process {
            if !uses_platform_capabilities {
                items.push(Item::generated(include_str!(
                    "../runtime/platform_result_type.rs"
                )));
            }
            items.push(Item::generated(include_str!(
                "../runtime/platform_process.rs"
            )));
        }
        runtime.push(GeneratedModule {
            name: "platform_system",
            items,
        });
    }
    if uses_documents || uses_json || uses_yaml || uses_urls {
        let mut items = Vec::new();
        if uses_documents || uses_json || uses_yaml {
            items.push(Item::generated(include_str!(
                "../runtime/platform_data_base.rs"
            )));
        }
        if uses_documents {
            items.push(Item::generated(include_str!(
                "../runtime/platform_documents.rs"
            )));
        }
        if uses_json {
            items.push(Item::generated(include_str!("../runtime/platform_json.rs")));
        }
        if uses_yaml {
            items.push(Item::generated(include_str!("../runtime/platform_yaml.rs")));
        }
        if uses_urls {
            items.push(Item::generated(include_str!("../runtime/platform_urls.rs")));
        }
        runtime.push(GeneratedModule {
            name: "platform_data",
            items,
        });
    }
    if uses_platform_capabilities {
        let mut items = vec![
            Item::generated(include_str!("../runtime/platform_capability_types.rs")),
            Item::generated(include_str!("../runtime/platform_result_type.rs")),
        ];
        if uses_random
            || uses_compression
            || uses_uuid
            || uses_networking
            || uses_tls
            || uses_concurrency
        {
            items.push(Item::generated(include_str!(
                "../runtime/platform_int_conversion.rs"
            )));
        }
        items.push(Item::generated(include_str!(
            "../runtime/platform_capability_base.rs"
        )));
        if uses_random {
            items.push(Item::generated(include_str!(
                "../runtime/platform_random.rs"
            )));
        }
        if uses_codecs {
            items.push(Item::generated(include_str!(
                "../runtime/platform_codecs.rs"
            )));
        }
        if uses_compression {
            items.push(Item::generated(include_str!(
                "../runtime/platform_compression.rs"
            )));
        }
        if uses_uuid {
            items.push(Item::generated(include_str!("../runtime/platform_uuid.rs")));
        }
        if uses_networking {
            items.push(Item::generated(include_str!(
                "../runtime/platform_networking.rs"
            )));
        }
        if uses_tls {
            items.push(Item::generated(include_str!("../runtime/platform_tls.rs")));
        }
        if uses_concurrency {
            items.push(Item::generated(include_str!(
                "../runtime/platform_concurrency.rs"
            )));
        }
        runtime.push(GeneratedModule {
            name: "platform_capabilities",
            items,
        });
    }
    if package.units.iter().any(|unit| {
        unit.typed_bindings.iter().any(|binding| {
            matches!(binding.value_type, ValueType::Descriptor(_))
                && descriptor_binding_is_materialized(package, unit, binding.span)
        })
    }) {
        runtime.push(GeneratedModule {
            name: "reflection",
            items: vec![Item::generated(
                "#[allow(dead_code)]\n\
                 #[derive(Clone, Copy)]\n\
                 struct TerraneDescriptor { identity: &'static str, name: &'static str, kind: &'static str }\n",
            )],
        });
    }
    emit_global_storage(package, &registry, &mut globals);
    let modules = package
        .units
        .iter()
        .map(|unit| {
            if unit.bundled && unit.namespace.starts_with("/deps/") {
                let rust = emit_dependency_unit(package, unit);
                return Module {
                    source_path: unit.source_path.clone(),
                    namespace: unit.namespace.clone(),
                    destination: module_destination(unit),
                    items: vec![Item::generated(&rust)],
                };
            }
            let mut emitter = Emitter {
                registry: &registry,
                package,
                unit,
                source: &unit.source,
                output: String::new(),
                indent: 0,
                continue_label: None,
                loop_counter: 0,
                return_type: None,
                parameter_types: Vec::new(),
                namespace_initializer: None,
                propagate_errors: false,
                discarded_call: None,
                function_errors: false,
                try_counter: 0,
                current_error: None,
                current_function: None,
                current_object: None,
                try_completion: false,
                in_loop: false,
                bounded_integer_ranges: Vec::new(),
                closure_depth: 0,
                assignment_target: false,
            };
            emitter.emit_union_types();
            let mut items = Vec::new();
            if !emitter.output.is_empty() {
                items.push(Item::generated(&emitter.output));
                emitter.output.clear();
            }
            for node in &unit.tree.root.children {
                match node.kind {
                    SyntaxKind::Binding | SyntaxKind::Assignment => {
                        emitter.namespace_binding(node);
                    }
                    SyntaxKind::FunctionDeclaration => emitter.function(node),
                    SyntaxKind::ClassDeclaration
                    | SyntaxKind::InterfaceDeclaration
                    | SyntaxKind::TraitDeclaration => emitter.object(node),
                    _ => {}
                }
                if !emitter.output.is_empty() {
                    items.push(Item::sourced(node.span, &emitter.output));
                    emitter.output.clear();
                }
            }
            Module {
                source_path: unit.source_path.clone(),
                namespace: unit.namespace.clone(),
                destination: module_destination(unit),
                items,
            }
        })
        .collect();
    if uses_errors || !registry.sites.borrow().is_empty() {
        let mut support = String::new();
        emit_error_support(
            &mut support,
            has_custom_throwable,
            has_dependency,
            &registry,
        );
        runtime.insert(
            0,
            GeneratedModule {
                name: "errors",
                items: vec![Item::generated(&support)],
            },
        );
    }
    Program {
        version: crate::VERSION,
        requires_platform_support,
        runtime,
        globals: (!globals.is_empty())
            .then(|| Item::generated(&globals))
            .into_iter()
            .collect(),
        modules,
    }
}
