use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use indoc::indoc;
use num_bigint::BigInt;

use crate::{
    ScalarType, SourceFile, TypeCategory,
    rust_ir::{GeneratedModule, Item, Module, Program},
    semantics::{
        ArithmeticFamily, CoercionPolicy, ContextualConstant, ElementType, FunctionContract,
        MemberFamily, ObjectContract, ObjectField, ObjectIdentity, ObjectKind, SemanticPackage,
        SemanticUnit, StringFamily, SymbolKind, TypedBinding, ValueType, binding_span_is_mutated,
        binding_store_value_is_read, bound_method, contextual_constant, narrowed_optional_type,
        narrowed_value_type, promoted_integer_type, string_call_selection,
    },
    syntax::{SyntaxKind, SyntaxNode},
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct LoweringSite {
    function: u32,
    file: u32,
    line: u32,
    column: u32,
    end_line: u32,
    end_column: u32,
}

#[derive(Default)]
struct LoweringRegistry {
    files: RefCell<Vec<String>>,
    functions: RefCell<Vec<String>>,
    sites: RefCell<Vec<LoweringSite>>,
    descriptors: RefCell<Vec<(String, String)>>,
    descriptor_ids: RefCell<BTreeMap<String, u32>>,
}

impl LoweringRegistry {
    fn intern(values: &RefCell<Vec<String>>, value: &str) -> u32 {
        let mut values = values.borrow_mut();
        if let Some(index) = values.iter().position(|candidate| candidate == value) {
            return u32::try_from(index).expect("lowering registry index must fit u32");
        }
        let index = u32::try_from(values.len()).expect("lowering registry index must fit u32");
        values.push(value.to_owned());
        index
    }

    fn register_site(
        &self,
        source_path: &str,
        function: &str,
        source: &SourceFile,
        span: crate::Span,
    ) -> u32 {
        let file = Self::intern(&self.files, source_path);
        let function = Self::intern(&self.functions, function);
        let (line, column) = source.line_column(span.start);
        let (end_line, end_column) = source.line_column(span.end);
        let candidate = LoweringSite {
            function,
            file,
            line: u32::try_from(line).expect("source line must fit u32"),
            column: u32::try_from(column).expect("source column must fit u32"),
            end_line: u32::try_from(end_line).expect("source line must fit u32"),
            end_column: u32::try_from(end_column).expect("source column must fit u32"),
        };
        let mut sites = self.sites.borrow_mut();
        if let Some(site) = sites.iter().position(|existing| existing == &candidate) {
            return u32::try_from(site).expect("lowering site index must fit u32");
        }
        let site = u32::try_from(sites.len()).expect("lowering site index must fit u32");
        assert_ne!(
            site,
            u32::MAX,
            "u32::MAX is reserved for an unattributed site"
        );
        sites.push(candidate);
        site
    }

    fn register_descriptor(&self, identity: &str, source_name: &str) -> u32 {
        if let Some(id) = self.descriptor_ids.borrow().get(identity).copied() {
            return id;
        }
        let mut descriptors = self.descriptors.borrow_mut();
        let id = u32::try_from(descriptors.len()).expect("descriptor index must fit u32");
        descriptors.push((identity.to_owned(), source_name.to_owned()));
        self.descriptor_ids
            .borrow_mut()
            .insert(identity.to_owned(), id);
        id
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
        let mut support = include_str!("runtime/async.rs").to_owned();
        if package_uses_task_scope(package) {
            support.push_str(include_str!("runtime/async_cancellable.rs"));
        }
        runtime.push(GeneratedModule {
            name: "async",
            items: vec![Item::generated(&support)],
        });
    }
    if package_uses_task_scope(package) {
        let support = match package.executor {
            crate::package::ExecutorProfile::Cooperative => {
                include_str!("runtime/tasks_cooperative.rs")
            }
            crate::package::ExecutorProfile::Threaded => {
                include_str!("runtime/tasks_threaded.rs")
            }
        };
        runtime.push(GeneratedModule {
            name: "tasks",
            items: vec![Item::generated(support)],
        });
    }
    let uses_standard_streams = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/streams");
    let uses_filesystem = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/filesystem");
    let uses_process = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/process");
    let uses_documents = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/documents");
    let uses_json = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/json");
    let uses_yaml = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/yaml");
    let uses_urls = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/urls");
    let uses_random = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/random");
    let uses_codecs = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/codecs");
    let uses_compression = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/compression");
    let uses_uuid = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/uuid");
    let uses_networking = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/networking");
    let uses_tls = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/tls");
    let uses_concurrency = package
        .units
        .iter()
        .any(|unit| unit.namespace == "/standard/concurrency");
    let uses_platform_capabilities = uses_random
        || uses_codecs
        || uses_compression
        || uses_uuid
        || uses_networking
        || uses_tls
        || uses_concurrency;
    let requires_platform_support = uses_standard_streams
        || uses_filesystem
        || uses_process
        || uses_documents
        || uses_json
        || uses_yaml
        || uses_urls
        || uses_platform_capabilities;
    if uses_standard_streams || uses_filesystem {
        let mut items = vec![Item::generated(include_str!("runtime/platform_streams.rs"))];
        if uses_standard_streams {
            items.push(Item::generated(include_str!(
                "runtime/platform_standard_streams.rs"
            )));
        }
        if uses_filesystem {
            items.push(Item::generated(include_str!("runtime/platform_files.rs")));
        }
        runtime.push(GeneratedModule {
            name: "platform_streams",
            items,
        });
    }
    if uses_filesystem || uses_process {
        let mut items = Vec::new();
        if uses_filesystem {
            items.push(Item::generated(include_str!("runtime/platform_system.rs")));
        }
        if uses_process {
            if !uses_platform_capabilities {
                items.push(Item::generated(include_str!(
                    "runtime/platform_result_type.rs"
                )));
            }
            items.push(Item::generated(include_str!("runtime/platform_process.rs")));
        }
        runtime.push(GeneratedModule {
            name: "platform_system",
            items,
        });
    }
    if uses_documents || uses_urls {
        let mut items = Vec::new();
        if uses_documents {
            items.push(Item::generated(include_str!(
                "runtime/platform_documents.rs"
            )));
        }
        if uses_json {
            items.push(Item::generated(include_str!("runtime/platform_json.rs")));
        }
        if uses_yaml {
            items.push(Item::generated(include_str!("runtime/platform_yaml.rs")));
        }
        if uses_urls {
            items.push(Item::generated(include_str!("runtime/platform_urls.rs")));
        }
        runtime.push(GeneratedModule {
            name: "platform_data",
            items,
        });
    }
    if uses_platform_capabilities {
        let mut items = vec![
            Item::generated(include_str!("runtime/platform_capability_types.rs")),
            Item::generated(include_str!("runtime/platform_result_type.rs")),
        ];
        if uses_random
            || uses_compression
            || uses_uuid
            || uses_networking
            || uses_tls
            || uses_concurrency
        {
            items.push(Item::generated(include_str!(
                "runtime/platform_int_conversion.rs"
            )));
        }
        items.push(Item::generated(include_str!(
            "runtime/platform_capability_base.rs"
        )));
        if uses_random {
            items.push(Item::generated(include_str!("runtime/platform_random.rs")));
        }
        if uses_codecs {
            items.push(Item::generated(include_str!("runtime/platform_codecs.rs")));
        }
        if uses_compression {
            items.push(Item::generated(include_str!(
                "runtime/platform_compression.rs"
            )));
        }
        if uses_uuid {
            items.push(Item::generated(include_str!("runtime/platform_uuid.rs")));
        }
        if uses_networking {
            items.push(Item::generated(include_str!(
                "runtime/platform_networking.rs"
            )));
        }
        if uses_tls {
            items.push(Item::generated(include_str!("runtime/platform_tls.rs")));
        }
        if uses_concurrency {
            items.push(Item::generated(include_str!(
                "runtime/platform_concurrency.rs"
            )));
        }
        runtime.push(GeneratedModule {
            name: "platform_capabilities",
            items,
        });
    }
    if package.units.iter().any(|unit| {
        unit.typed_bindings
            .iter()
            .any(|binding| matches!(binding.value_type, ValueType::Descriptor(_)))
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

fn emit_dependency_imports(package: &SemanticPackage, unit: &SemanticUnit, output: &mut String) {
    let mut imported = BTreeSet::new();
    for object in &unit.objects {
        if object.identity.namespace != unit.namespace {
            continue;
        }
        if let Some(path) = package
            .projection
            .foreign_rust_path(&object.identity.namespace, &object.identity.name)
        {
            let rust_name = rust_object_type_name(package, &object.identity);
            if !imported.insert(rust_name.clone()) {
                continue;
            }
            write_foreign_import(output, path, &rust_name);
        }
    }
    for (name, path) in package.projection.foreign_imports(&unit.namespace) {
        let rust_name = rust_object_name(&name);
        if imported.insert(rust_name.clone()) {
            write_foreign_import(output, &path, &rust_name);
        }
    }
}

fn write_foreign_import(output: &mut String, path: &str, rust_name: &str) {
    if path.rsplit("::").next() == Some(rust_name) {
        writeln!(output, "pub use {path};").expect("writing to a string cannot fail");
    } else {
        writeln!(output, "pub use {path} as {rust_name};")
            .expect("writing to a string cannot fail");
    }
}

fn projected_argument_expression(name: &str, ty: &crate::projection::ProjectedType) -> String {
    match ty {
        crate::projection::ProjectedType::RustInt(rust_type) => format!(
            "terrane_int_support::coerce::<{rust_type}>(&{name}).map_err(|error| crate::TerraneForeignError(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)))?"
        ),
        crate::projection::ProjectedType::Char => format!(
            "{name}.parse::<char>().map_err(|_| crate::TerraneForeignError(crate::TerraneError::raised_with_message(crate::TerraneErrorKind::CoercionError, \"projected `char` requires exactly one Unicode scalar\", crate::TERRANE_NO_SITE)))?"
        ),
        crate::projection::ProjectedType::Optional(inner) => match inner.as_ref() {
            crate::projection::ProjectedType::RustInt(rust_type) => format!(
                "{name}.map(|value| terrane_int_support::coerce::<{rust_type}>(&value)).transpose().map_err(|error| crate::TerraneForeignError(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)))?"
            ),
            crate::projection::ProjectedType::Char => format!(
                "{name}.map(|value| value.parse::<char>().map_err(|_| crate::TerraneForeignError(crate::TerraneError::raised_with_message(crate::TerraneErrorKind::CoercionError, \"projected `char` requires exactly one Unicode scalar\", crate::TERRANE_NO_SITE)))).transpose()?"
            ),
            _ => name.to_owned(),
        },
        _ => name.to_owned(),
    }
}

fn projected_result_expression(value: &str, ty: &crate::projection::ProjectedType) -> String {
    match ty {
        crate::projection::ProjectedType::RustInt(rust_type) if rust_type.starts_with('u') => {
            format!("terrane_int_support::Int::from_u128({value} as u128)")
        }
        crate::projection::ProjectedType::RustInt(_) => {
            format!("terrane_int_support::Int::from({value} as i128)")
        }
        crate::projection::ProjectedType::Char => format!("{value}.to_string()"),
        crate::projection::ProjectedType::Optional(inner) => {
            let converted = projected_result_expression("value", inner);
            if converted == "value" {
                value.to_owned()
            } else {
                format!("{value}.map(|value| {converted})")
            }
        }
        _ => value.to_owned(),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "dependency shim emission keeps each generated branch beside the shared call contract"
)]
fn emit_dependency_unit(package: &SemanticPackage, unit: &SemanticUnit) -> String {
    let mut output = String::new();
    emit_dependency_imports(package, unit, &mut output);
    for contract in unit
        .functions
        .iter()
        .filter(|contract| contract.owner.is_none())
    {
        let Some(item) = package.projection.item(&unit.namespace, &contract.name) else {
            continue;
        };
        let crate::projection::ProjectedKind::Function(projected) = &item.kind else {
            continue;
        };
        let dependency_name = package
            .projection
            .dependency_name(&unit.namespace, &contract.name)
            .unwrap_or("dependency");
        let parameters = contract
            .parameters
            .iter()
            .zip(&projected.parameters)
            .map(|(parameter, projected)| {
                let value_type = parameter.value_type.clone().map_or_else(
                    || "()".to_owned(),
                    |value_type| rust_value_type(package, value_type),
                );
                let preserves_identity = projected.borrowed
                    && matches!(
                        projected.ty,
                        crate::projection::ProjectedType::Foreign { .. }
                    );
                format!(
                    "{}: {}{value_type}",
                    rust_name(&parameter.name),
                    if preserves_identity {
                        if projected.mutable_borrow {
                            "&mut "
                        } else {
                            "&"
                        }
                    } else {
                        ""
                    },
                )
            })
            .collect::<Vec<_>>();
        let mut argument_conversions = Vec::new();
        let mut arguments = Vec::new();
        for (parameter, projected) in contract.parameters.iter().zip(&projected.parameters) {
            let name = rust_name(&parameter.name);
            if projected.borrowed
                && matches!(
                    projected.ty,
                    crate::projection::ProjectedType::Foreign { .. }
                )
            {
                arguments.push(name);
                continue;
            }
            let value = projected_argument_expression(&name, &projected.ty);
            argument_conversions.push(format!(
                "    let {}{name} = {value};",
                if projected.mutable_borrow { "mut " } else { "" }
            ));
            arguments.push(if projected.mutable_borrow {
                format!("&mut {name}")
            } else if projected.borrowed {
                format!("&{name}")
            } else {
                name
            });
        }
        let arguments = arguments.join(", ");
        let value = contract.return_type.clone().map_or_else(
            || "()".to_owned(),
            |value_type| rust_value_type(package, value_type),
        );
        let result = format!("Result<{value}, crate::TerraneForeignError>");
        let converted_value = projected_result_expression("value", &projected.result);
        let unit_variant = package.projection.is_unit_variant(item);
        if unit_variant {
            writeln!(
                output,
                "/// Projected enum variant constructor for `{}`.",
                item.rust_path
            )
            .expect("writing to a string cannot fail");
        }
        writeln!(
            output,
            "pub fn {}({}) -> {result} {{",
            rust_name(&contract.name),
            parameters.join(", ")
        )
        .expect("writing to a string cannot fail");
        for conversion in argument_conversions {
            writeln!(output, "{conversion}").expect("writing to a string cannot fail");
        }
        let call = if unit_variant {
            item.rust_path.clone()
        } else {
            format!("{}({arguments})", item.rust_path)
        };
        let caught = if projected
            .parameters
            .iter()
            .any(|parameter| parameter.mutable_borrow)
        {
            format!("std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {call}))")
        } else {
            format!("std::panic::catch_unwind(|| {call})")
        };
        if package.profile.panic == crate::package::PanicProfile::Abort {
            if projected.error.is_some() {
                writeln!(
                    output,
                    "    match {call} {{\n        Ok(value) => Ok({converted_value}),\n        Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{dependency_name}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n    }}",
                    item.rust_path,
                )
                .expect("writing to a string cannot fail");
            } else {
                writeln!(output, "    let value = {call};\n    Ok({converted_value})",)
                    .expect("writing to a string cannot fail");
            }
        } else if projected.error.is_some() {
            writeln!(
                output,
                "    match {caught} {{\n        Ok(Ok(value)) => Ok({converted_value}),\n        Ok(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n        Err(payload) => Err(crate::__terrane_dependency_panic(payload, {:?}, {:?})),\n    }}",
                dependency_name,
                item.rust_path,
                dependency_name,
                item.rust_path,
            )
            .expect("writing to a string cannot fail");
        } else {
            writeln!(
                output,
                "    match {caught} {{\n        Ok(value) => Ok({converted_value}),\n        Err(payload) => Err(crate::__terrane_dependency_panic(payload, {:?}, {:?})),\n    }}",
                dependency_name,
                item.rust_path,
            )
            .expect("writing to a string cannot fail");
        }
        output.push_str("}\n");
    }
    output
}

#[derive(Clone, Debug)]
struct BoundedIntegerRange {
    binding: crate::Span,
    lower: BigInt,
    upper: BigInt,
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "these independent lexical control contexts are saved and restored separately"
)]
struct Emitter<'a> {
    registry: &'a LoweringRegistry,
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    source: &'a SourceFile,
    output: String,
    indent: usize,
    continue_label: Option<String>,
    loop_counter: usize,
    return_type: Option<ValueType>,
    parameter_types: Vec<(String, ValueType)>,
    namespace_initializer: Option<(String, String)>,
    propagate_errors: bool,
    discarded_call: Option<crate::Span>,
    function_errors: bool,
    try_counter: usize,
    current_error: Option<String>,
    current_function: Option<String>,
    current_object: Option<ObjectIdentity>,
    try_completion: bool,
    in_loop: bool,
    closure_depth: usize,
    bounded_integer_ranges: Vec<BoundedIntegerRange>,
}

fn package_uses_task_scope(package: &SemanticPackage) -> bool {
    fn contains(package: &SemanticPackage, unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
            && package
                .resolve_name_at(
                    unit,
                    callee.span.start,
                    &unit.source.text()[callee.span.start..callee.span.end],
                )
                .is_some_and(|symbol| symbol.identity == "/core/async::task-scope")
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| contains(package, unit, child))
    }

    package
        .units
        .iter()
        .any(|unit| contains(package, unit, &unit.tree.root))
}

fn package_uses_structured_errors(package: &SemanticPackage) -> bool {
    fn contains(package: &SemanticPackage, unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        matches!(
            node.kind,
            SyntaxKind::ThrowStatement | SyntaxKind::TryStatement | SyntaxKind::IndexExpression
        ) || string_call_selection(&unit.source, node)
            .is_some_and(|selection| selection.family == StringFamily::Decode)
            || node
                .children
                .iter()
                .any(|child| contains(package, unit, child))
            || (node.kind == SyntaxKind::CallExpression
                && node.children.first().is_some_and(|callee| {
                    let range = if callee.kind == SyntaxKind::MemberExpression {
                        callee.children.first()
                    } else {
                        Some(callee)
                    };
                    range.is_some_and(|range| {
                        package
                            .resolve_name_at(
                                unit,
                                range.span.start,
                                &unit.source.text()[range.span.start..range.span.end],
                            )
                            .is_some_and(|symbol| symbol.identity == "/core/collections::range")
                    })
                }))
            || (node.kind == SyntaxKind::CallExpression
                && node.children.first().is_some_and(|callee| {
                    callee.kind == SyntaxKind::MemberExpression
                        && callee.children.get(1).is_some_and(|member| {
                            &unit.source.text()[member.span.start..member.span.end] == "set"
                        })
                }))
    }
    package.units.iter().any(|unit| {
        unit.functions.iter().any(|contract| contract.throws)
            || contains(package, unit, &unit.tree.root)
    }) || package.projection.dependencies.iter().any(|dependency| {
        dependency.items.iter().any(|item| match &item.kind {
            crate::projection::ProjectedKind::Function(_) => true,
            crate::projection::ProjectedKind::ForeignType { methods } => !methods.is_empty(),
            crate::projection::ProjectedKind::Enum { .. } => false,
        })
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "the generated error runtime remains directly reviewable as one canonical Rust template"
)]
fn emit_error_support(
    output: &mut String,
    has_custom_throwable: bool,
    has_dependency: bool,
    registry: &LoweringRegistry,
) {
    output.push_str(indoc! {r#"
        type TerraneSite = u32;
        const TERRANE_NO_SITE: TerraneSite = u32::MAX;
        #[allow(
            dead_code,
            reason = "custom descriptors are absent from some lowered programs"
        )]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct DescriptorId(u16);
        #[allow(
            dead_code,
            reason = "one canonical runtime enum covers every compiler-owned throwable kind"
        )]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(u16)]
        enum TerraneErrorKind {
            ArithmeticOverflow,
            DivisionByZero,
            IntegerConversionOverflow,
            NegativeShiftCount,
            CoercionError,
            DecodeError,
            IndexError,
            MissingKey,
            ResourceError,
            SourceError,
    "#});
    if has_custom_throwable {
        output.push_str("    Custom(DescriptorId),\n");
    }
    output.push_str(indoc! {r#"
        }
        impl TerraneErrorKind {
            fn display_name(self) -> &'static str {
                match self {
                    Self::ArithmeticOverflow => "arithmetic-overflow",
                    Self::DivisionByZero => "division-by-zero",
                    Self::IntegerConversionOverflow => "integer-conversion-overflow",
                    Self::NegativeShiftCount => "negative-shift-count",
                    Self::CoercionError => "coercion-error",
                    Self::DecodeError => "decode-error",
                    Self::IndexError => "index-error",
                    Self::MissingKey => "missing-key",
                    Self::ResourceError => "resource-error",
                    Self::SourceError => "error",
    "#});
    if has_custom_throwable {
        output.push_str(
            "                Self::Custom(descriptor) => __terrane_error_registry::DESCRIPTORS[usize::from(descriptor.0)],\n",
        );
    }
    output.push_str(indoc! {r#"
                }
            }
            fn default_message(self) -> &'static str {
                match self {
                    Self::ArithmeticOverflow => "fixed-width integer arithmetic overflow",
                    Self::DivisionByZero => "integer division by zero",
                    Self::IntegerConversionOverflow => "integer conversion overflow",
                    Self::NegativeShiftCount => "negative integer shift count",
                    Self::CoercionError => "coercion has no compatible result",
                    Self::DecodeError => "invalid byte sequence for selected encoding",
                    Self::IndexError => "collection index is out of range",
                    Self::MissingKey => "collection key is absent",
                    Self::ResourceError => {
                        "integer shift count cannot be represented on this target"
                    }
                    Self::SourceError => "source error",
    "#});
    if has_custom_throwable {
        output.push_str("            Self::Custom(_) => \"source error\",\n");
    }
    output.push_str(indoc! {r#"
                }
            }
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct TerraneErrorDetail {
            message: Option<String>,
            cause: Option<Box<TerraneError>>,
            frames: Vec<TerraneSite>,
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct TerraneError {
            kind: TerraneErrorKind,
            origin: TerraneSite,
            detail: Option<Box<TerraneErrorDetail>>,
        }
        #[cfg(target_pointer_width = "64")]
        const _: () = assert!(std::mem::size_of::<TerraneError>() == 16);
        #[cfg(target_pointer_width = "64")]
        const _: () = assert!(std::mem::size_of::<Result<i64, TerraneError>>() == 16);
        #[allow(
            dead_code,
            reason = "one canonical runtime implementation serves every lowered error shape"
        )]
        impl TerraneError {
            #[cold]
            #[inline(never)]
            fn raised(kind: TerraneErrorKind, origin: TerraneSite) -> Self {
                Self {
                    kind,
                    origin,
                    detail: None,
                }
            }
            #[cold]
            #[inline(never)]
            fn raised_with_message(
                kind: TerraneErrorKind,
                message: impl Into<String>,
                origin: TerraneSite,
            ) -> Self {
                Self {
                    kind,
                    origin,
                    detail: Some(Box::new(TerraneErrorDetail {
                        message: Some(message.into()),
                        cause: None,
                        frames: Vec::new(),
                    })),
                }
            }
    "#});
    if has_custom_throwable {
        output.push_str(indoc! {r"
            #[cold]
            #[inline(never)]
            fn custom_raised(
                descriptor: DescriptorId,
                message: impl Into<String>,
                origin: TerraneSite,
            ) -> Self {
                Self::raised_with_message(TerraneErrorKind::Custom(descriptor), message, origin)
            }
        "});
    }
    output.push_str(indoc! {r#"
            #[cold]
            #[inline(never)]
            fn with_cause(mut self, cause: TerraneError) -> Self {
                self.detail
                    .get_or_insert_with(|| {
                        Box::new(TerraneErrorDetail {
                            message: None,
                            cause: None,
                            frames: Vec::new(),
                        })
                    })
                    .cause = Some(Box::new(cause));
                self
            }
            #[cold]
            #[inline(never)]
            fn attributed(mut self, origin: TerraneSite) -> Self {
                debug_assert_eq!(self.origin, TERRANE_NO_SITE);
                self.origin = origin;
                self
            }
            #[cold]
            #[inline(never)]
            fn at(mut self, frame: TerraneSite) -> Self {
                self.detail
                    .get_or_insert_with(|| {
                        Box::new(TerraneErrorDetail {
                            message: None,
                            cause: None,
                            frames: Vec::new(),
                        })
                    })
                    .frames
                    .push(frame);
                self
            }
            fn message(&self) -> &str {
                self.detail
                    .as_ref()
                    .and_then(|detail| detail.message.as_deref())
                    .unwrap_or_else(|| self.kind.default_message())
            }
            #[cold]
            #[inline(never)]
            fn render(&self) -> String {
                let mut rendered = format!("{}: {}", self.kind.display_name(), self.message());
                if let Some(cause) = self
                    .detail
                    .as_ref()
                    .and_then(|detail| detail.cause.as_ref())
                {
                    rendered.push_str("\ncaused by: ");
                    rendered.push_str(&cause.render());
                }
                if self.origin != TERRANE_NO_SITE {
                    rendered.push_str("\nat ");
                    rendered.push_str(&__terrane_trace::render(self.origin));
                }
                if let Some(detail) = &self.detail {
                    for frame in &detail.frames {
                        rendered.push_str("\nat ");
                        rendered.push_str(&__terrane_trace::render(*frame));
                    }
                }
                rendered
            }
        }
        impl std::fmt::Display for TerraneError {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&self.render())
            }
        }
        #[allow(
            dead_code,
            reason = "fresh support failures are absent from some lowered programs"
        )]
        trait TerraneRaised {
            fn raised(self, origin: TerraneSite) -> TerraneError;
        }
        pub struct TerraneForeignError(TerraneError);
        impl TerraneForeignError {
            pub fn render(&self) -> String {
                self.0.render()
            }
        }
        impl TerraneRaised for TerraneForeignError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                self.0.attributed(origin)
            }
        }
        impl TerraneRaised for terrane_int_support::ArithmeticError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                use terrane_int_support::ArithmeticError;
                match self {
                    ArithmeticError::DivisionByZero => {
                        TerraneError::raised(TerraneErrorKind::DivisionByZero, origin)
                    }
                    ArithmeticError::ArithmeticOverflow => {
                        TerraneError::raised(TerraneErrorKind::ArithmeticOverflow, origin)
                    }
                    ArithmeticError::NegativeShiftCount => {
                        TerraneError::raised(TerraneErrorKind::NegativeShiftCount, origin)
                    }
                    ArithmeticError::ShiftCountTooLarge => {
                        TerraneError::raised(TerraneErrorKind::ResourceError, origin)
                    }
                    error @ (ArithmeticError::IntegerConversionOverflow
                    | ArithmeticError::IntegerConversionOverflowDetail { .. }) => {
                        TerraneError::raised_with_message(
                            TerraneErrorKind::IntegerConversionOverflow,
                            error.to_string(),
                            origin,
                        )
                    }
                    error @ (ArithmeticError::InvalidRadix | ArithmeticError::InvalidRadixText) => {
                        TerraneError::raised_with_message(
                            TerraneErrorKind::CoercionError,
                            error.to_string(),
                            origin,
                        )
                    }
                }
            }
        }
        impl TerraneRaised for terrane_string_support::DecodeError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::DecodeError,
                    self.to_string(),
                    origin,
                )
            }
        }
        impl TerraneRaised for terrane_collection_support::IndexError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::IndexError,
                    self.to_string(),
                    origin,
                )
            }
        }
        impl TerraneRaised for terrane_collection_support::MissingKey {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::MissingKey,
                    self.to_string(),
                    origin,
                )
            }
        }
        impl TerraneRaised for terrane_collection_support::RangeStepError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::SourceError,
                    self.to_string(),
                    origin,
                )
            }
        }
        #[allow(
            dead_code,
            reason = "terminating fresh failures are absent from some lowered programs"
        )]
        #[cold]
        #[inline(never)]
        fn __terrane_raise<E: TerraneRaised>(error: E, origin: TerraneSite) -> ! {
            __terrane_uncaught(error.raised(origin))
        }
        #[allow(
            dead_code,
            reason = "propagating failures are absent from some lowered programs"
        )]
        #[cold]
        #[inline(never)]
        fn __terrane_trace_error(error: TerraneError, frame: TerraneSite) -> TerraneError {
            error.at(frame)
        }
        #[allow(
            dead_code,
            reason = "terminating fresh failures are absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_raised<T, E: TerraneRaised>(
            result: Result<T, E>,
            origin: TerraneSite,
        ) -> T {
            result.unwrap_or_else(|error| __terrane_raise(error, origin))
        }
        #[allow(
            dead_code,
            reason = "fresh failure propagation is absent from some lowered programs"
        )]
        #[cold]
        #[inline(never)]
        fn __terrane_fresh_error<E: TerraneRaised>(
            error: E,
            origin: TerraneSite,
        ) -> TerraneError {
            error.raised(origin)
        }
        #[allow(
            dead_code,
            reason = "returning fresh failures are absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_raised_err<T, E: TerraneRaised>(
            result: Result<T, E>,
            origin: TerraneSite,
        ) -> Result<T, TerraneError> {
            result.map_err(|error| __terrane_fresh_error(error, origin))
        }
        macro_rules! __terrane_raised_completion {
            ($result:expr, $origin:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(error) => {
                        return TerraneCompletion::Error(__terrane_fresh_error(error, $origin));
                    }
                }
            };
        }
        #[allow(
            dead_code,
            reason = "terminating propagation is absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_traced<T>(
            result: Result<T, TerraneError>,
            frame: TerraneSite,
        ) -> T {
            result.unwrap_or_else(|error| __terrane_uncaught(__terrane_trace_error(error, frame)))
        }
        #[allow(
            dead_code,
            reason = "returning propagation is absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_traced_err<T>(
            result: Result<T, TerraneError>,
            frame: TerraneSite,
        ) -> Result<T, TerraneError> {
            result.map_err(|error| __terrane_trace_error(error, frame))
        }
        macro_rules! __terrane_traced_completion {
            ($result:expr, $frame:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(error) => {
                        return TerraneCompletion::Error(__terrane_trace_error(error, $frame));
                    }
                }
            };
        }
        fn __terrane_uncaught(error: TerraneError) -> ! {
            eprintln!("{}", error.render());
            std::process::exit(1);
        }
        fn __terrane_generated_defect(message: &str) -> ! {
            eprintln!(
                "internal compiler defect: generated program reached an impossible completion: {message}"
            );
            std::process::exit(5);
        }
        #[allow(dead_code)]
        enum TerraneCompletion<T> {
            Normal,
            Return(T),
            Error(TerraneError),
            Break,
            Continue,
        }
    "#});
    if has_dependency {
        let error_descriptor =
            registry.register_descriptor("/core/errors::dependency-error", "dependency-error");
        let panic_descriptor =
            registry.register_descriptor("/core/errors::dependency-panic", "dependency-panic");
        writeln!(
            output,
            "#[allow(dead_code, reason = \"a projected dependency may expose no Result members\")]\nconst TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId({error_descriptor});\n#[allow(dead_code, reason = \"panic catching may be disabled or not crossed\")]\nconst TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId({panic_descriptor});"
        )
        .expect("writing to a String cannot fail");
        output.push_str(indoc! {r#"
            #[allow(
                dead_code,
                reason = "projected type methods may be imported without being crossed"
            )]
            fn __terrane_dependency_panic(
                payload: Box<dyn std::any::Any + Send>,
                crate_name: &'static str,
                member: &'static str,
            ) -> TerraneForeignError {
                let detail = payload
                    .downcast_ref::<&str>()
                    .copied()
                    .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
                    .unwrap_or("non-string panic payload");
                TerraneForeignError(TerraneError::custom_raised(
                    TERRANE_DEPENDENCY_PANIC,
                    format!("Rust dependency `{crate_name}` member `{member}` panicked: {detail}"),
                    TERRANE_NO_SITE,
                ))
            }
        "#});
    }
    emit_site_tables(output, registry);
}

fn emit_site_tables(output: &mut String, registry: &LoweringRegistry) {
    let files = registry.files.borrow();
    let functions = registry.functions.borrow();
    let sites = registry.sites.borrow();
    let descriptors = registry.descriptors.borrow();
    output.push_str(indoc! {r#"
        mod __terrane_error_registry {
            #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    "#});
    writeln!(
        output,
        "    pub static DESCRIPTORS: [&str; {}] = [",
        descriptors.len()
    )
    .expect("writing to a String cannot fail");
    for (_, name) in descriptors.iter() {
        writeln!(output, "        {name:?},").expect("writing to a String cannot fail");
    }
    output.push_str("    ];\n}\n");
    output.push_str(indoc! {r"
        mod __terrane_trace {
            pub struct Site {
                pub function: u32,
                pub file: u32,
                pub line: u32,
                pub column: u32,
                pub end_line: u32,
                pub end_column: u32,
            }
    "});
    writeln!(output, "    pub static FILES: [&str; {}] = [", files.len())
        .expect("writing to a String cannot fail");
    for file in files.iter() {
        writeln!(output, "        {file:?},").expect("writing to a String cannot fail");
    }
    output.push_str("    ];\n");
    writeln!(
        output,
        "    pub static FUNCTIONS: [&str; {}] = [",
        functions.len()
    )
    .expect("writing to a String cannot fail");
    for function in functions.iter() {
        writeln!(output, "        {function:?},").expect("writing to a String cannot fail");
    }
    output.push_str("    ];\n");
    writeln!(output, "    pub static SITES: [Site; {}] = [", sites.len())
        .expect("writing to a String cannot fail");
    for (id, site) in sites.iter().enumerate() {
        let function = &functions[usize::try_from(site.function).expect("u32 must fit usize")];
        let file = &files[usize::try_from(site.file).expect("u32 must fit usize")];
        writeln!(
            output,
            "        {{\n            __terrane_site_comment!({:?});\n            Site {{ function: {}, file: {}, line: {}, column: {}, end_line: {}, end_column: {} }}\n        }},",
            format!("site {id}: {function} ({file}:{}:{}-{}:{})", site.line, site.column, site.end_line, site.end_column),
            site.function,
            site.file,
            site.line,
            site.column,
            site.end_line,
            site.end_column,
        )
        .expect("writing to a String cannot fail");
    }
    output.push_str(indoc! {r#"
            ];
            #[cold]
            #[inline(never)]
            pub fn render(site: u32) -> String {
                let site = &SITES[usize::try_from(site).expect("site id must fit usize")];
                format!(
                    "{} ({}:{}:{}-{}:{})",
                    FUNCTIONS[usize::try_from(site.function).expect("function id must fit usize")],
                    FILES[usize::try_from(site.file).expect("file id must fit usize")],
                    site.line,
                    site.column,
                    site.end_line,
                    site.end_column,
                )
            }
        }
    "#});
}

#[expect(
    clippy::too_many_lines,
    reason = "program-global declarations and their initialization policy remain auditable together"
)]
fn emit_global_storage(
    package: &SemanticPackage,
    registry: &LoweringRegistry,
    output: &mut String,
) {
    for (name, symbol) in &package.globals {
        if symbol.kind != SymbolKind::Binding {
            continue;
        }
        let Some(span) = symbol.declaration_span else {
            continue;
        };
        let Some(unit) = package
            .units
            .iter()
            .find(|unit| unit.source.id() == span.file)
        else {
            continue;
        };
        let Some(node) = find_node_by_span(&unit.tree.root, span) else {
            continue;
        };
        let Some(name_node) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
        else {
            continue;
        };
        let emitter = Emitter {
            registry,
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
        };
        let value_type = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span)
            .map(|binding| binding.value_type.clone())
            .or_else(|| emitter.value_type(name_node));
        let Some(ValueType::Scalar(scalar)) = value_type else {
            continue;
        };
        let initial = package.units.iter().rev().find_map(|candidate_unit| {
            candidate_unit
                .tree
                .root
                .children
                .iter()
                .rev()
                .find_map(|candidate| {
                    let global = candidate.children.iter().any(|child| {
                        child.kind == SyntaxKind::DeclarationQualifier
                            && candidate_unit.source.text()[child.span.start..child.span.end].trim()
                                == "global"
                    });
                    let candidate_name = candidate
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)?;
                    (global
                        && &candidate_unit.source.text()
                            [candidate_name.span.start..candidate_name.span.end]
                            == name.as_str())
                    .then_some((candidate_unit, candidate, candidate_name))
                })
        });
        let initial = initial.and_then(|(initial_unit, initial_node, initial_name)| {
            let name_index = initial_node
                .children
                .iter()
                .position(|child| child.span == initial_name.span)?;
            let initializer = binding_initializer(initial_node, name_index)?;
            let mut initial_emitter = Emitter {
                registry,
                package,
                unit: initial_unit,
                source: &initial_unit.source,
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
            };
            Some(initial_emitter.expression_as(initializer, ValueType::Scalar(scalar)))
        });
        let initial = initial.map_or_else(|| "None".to_owned(), |value| format!("Some({value})"));
        writeln!(
            output,
            "static {}: std::sync::LazyLock<std::sync::Mutex<Option<{}>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new({initial}));",
            global_binding_name(name),
            rust_type(scalar)
        )
        .unwrap();
    }
    if package
        .globals
        .values()
        .any(|symbol| symbol.kind == SymbolKind::Binding)
    {
        output.push_str(
            "fn __terrane_uninitialized_global(name: &str, path: &str, line: usize, column: usize) -> ! {\n    eprintln!(\"{path}:{line}:{column}: error[T0007]: `{name}` may be read before it is assigned\");\n    std::process::exit(1);\n}\n",
        );
    }
}

impl Emitter<'_> {
    fn global_storage(&self, node: &SyntaxNode) -> Option<String> {
        (node.kind == SyntaxKind::Name)
            .then(|| {
                self.package
                    .resolve_name_at(self.unit, node.span.start, self.text(node))
            })
            .flatten()
            .filter(|symbol| symbol.global && symbol.kind == SymbolKind::Binding)
            .map(|symbol| global_binding_name(&symbol.name))
    }

    fn global_assignment(&mut self, node: &SyntaxNode) -> bool {
        let Some(name) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
        else {
            return false;
        };
        let declared_global = node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier && self.text(child) == "global"
        });
        let storage = if declared_global {
            Some(global_binding_name(self.text(name)))
        } else {
            self.global_storage(name)
        };
        let Some(storage) = storage else {
            return false;
        };
        let Some((name_index, _)) = node
            .children
            .iter()
            .enumerate()
            .find(|(_, child)| child.kind == SyntaxKind::Name)
        else {
            return false;
        };
        let Some(initializer) = binding_initializer(node, name_index) else {
            return false;
        };
        let value = if let Some(ty) = self.value_type(name) {
            self.expression_as(initializer, ty)
        } else {
            self.expression(initializer)
        };
        let value = Self::unwrapped_expression(value);
        self.line("{");
        self.indent += 1;
        self.line(&format!("let value = {value};"));
        self.line(&format!(
            "*{storage}.lock().expect(\"program-global lock poisoned\") = Some(value);"
        ));
        self.indent -= 1;
        self.line("}");
        true
    }
    #[expect(
        clippy::too_many_lines,
        reason = "namespace initialization sequencing remains auditable as one lowering operation"
    )]
    fn namespace_binding(&mut self, node: &SyntaxNode) {
        if node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier && self.text(child) == "global"
        }) {
            return;
        }
        let Some(name_node) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
        else {
            return;
        };
        let source_name = self.text(name_node);
        let Some(symbol) =
            self.package
                .resolve_name_at(self.unit, name_node.span.start, source_name)
        else {
            return;
        };
        let Some(declaration_span) = symbol.declaration_span else {
            return;
        };
        if symbol.global || !self.is_namespace_binding_span(declaration_span) {
            return;
        }
        let Some(binding) = self
            .unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == declaration_span)
        else {
            return;
        };
        let ValueType::Scalar(scalar) = binding.value_type.clone() else {
            return;
        };
        let initializers = self
            .unit
            .tree
            .root
            .children
            .iter()
            .filter(|candidate| {
                matches!(candidate.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
                    && !candidate.children.iter().any(|child| {
                        child.kind == SyntaxKind::DeclarationQualifier
                            && self.text(child) == "global"
                    })
            })
            .filter_map(|candidate| {
                let (name_index, candidate_name) = candidate
                    .children
                    .iter()
                    .enumerate()
                    .find(|(_, child)| child.kind == SyntaxKind::Name)?;
                (self.text(candidate_name) == source_name)
                    .then_some(binding_initializer(candidate, name_index))
                    .flatten()
                    .cloned()
            })
            .collect::<Vec<_>>();
        let Some(first) = initializers.first() else {
            assert!(
                !self.text(node).contains('='),
                "analyzed initialized value binding must have a selected initializer"
            );
            return;
        };
        if !node.children.iter().any(|child| child.span == first.span) {
            return;
        }

        let ty = rust_type(scalar);
        let storage = namespace_binding_name(declaration_span.file, source_name);
        let local = format!("__terrane_{}_value", rust_name(source_name));
        self.namespace_initializer = Some((source_name.to_owned(), local.clone()));
        let values = initializers
            .iter()
            .map(|initializer| self.expression_as(initializer, binding.value_type.clone()))
            .collect::<Vec<_>>();
        self.namespace_initializer = None;
        if values.len() == 1 {
            self.line(&format!(
                "static {storage}: std::sync::LazyLock<{ty}> = std::sync::LazyLock::new(|| {});",
                values[0]
            ));
            return;
        }
        self.line(&format!(
            "static {storage}: std::sync::LazyLock<{ty}> = std::sync::LazyLock::new(|| {{"
        ));
        self.indent += 1;
        self.line(&format!("let mut {local} = {};", values[0]));
        for value in &values[1..] {
            self.line(&format!(
                "{local} = {};",
                Self::unwrapped_expression(value.clone())
            ));
        }
        self.line(&local);
        self.indent -= 1;
        self.line("});");
    }
    #[expect(
        clippy::too_many_lines,
        reason = "object lowering emits one complete, ordered Rust object contract"
    )]
    fn object(&mut self, node: &SyntaxNode) {
        let object = self
            .unit
            .objects
            .iter()
            .find(|object| object.span == node.span)
            .expect("analyzed object declaration must have a semantic contract");
        match object.kind {
            ObjectKind::Interface => {
                let name = rust_object_type_name(self.package, &object.identity);
                let protocol = format!("{name}Protocol");
                let methods = effective_object_methods(self.unit, object);
                self.line(&format!("pub trait {protocol} {{"));
                self.indent += 1;
                self.line(&format!("fn clone_box(&self) -> Box<dyn {protocol}>;"));
                self.line(&format!("fn separate_box(&self) -> Box<dyn {protocol}>;"));
                for method in &methods {
                    self.line_start();
                    let receiver = if method.consumes_receiver {
                        "self: Box<Self>"
                    } else if method.mutates_receiver {
                        "&mut self"
                    } else {
                        "&self"
                    };
                    write!(self.output, "fn {}({receiver}", rust_name(&method.name)).unwrap();
                    for parameter in &method.parameters {
                        let ty = parameter.value_type.clone().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        write!(self.output, ", {}: {ty}", rust_name(&parameter.name)).unwrap();
                    }
                    self.output.push(')');
                    if let Some(result) = method
                        .return_type
                        .clone()
                        .filter(|result| *result != ValueType::Scalar(ScalarType::None))
                    {
                        write!(self.output, " -> {}", rust_value_type(self.package, result))
                            .unwrap();
                    }
                    self.output.push_str(";\n");
                }
                self.indent -= 1;
                self.line("}");
                self.line(&format!(
                    "impl Clone for Box<dyn {protocol}> {{ fn clone(&self) -> Self {{ self.clone_box() }} }}"
                ));
                self.line("#[derive(Clone)]");
                self.line(&format!("pub struct {name}(Box<dyn {protocol}>);"));
                self.line(&format!("impl {name} {{"));
                self.indent += 1;
                for method in &methods {
                    self.line_start();
                    let receiver = if method.consumes_receiver {
                        "self"
                    } else if method.mutates_receiver {
                        "&mut self"
                    } else {
                        "&self"
                    };
                    write!(self.output, "pub fn {}({receiver}", rust_name(&method.name)).unwrap();
                    for parameter in &method.parameters {
                        let ty = parameter.value_type.clone().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        write!(self.output, ", {}: {ty}", rust_name(&parameter.name)).unwrap();
                    }
                    self.output.push(')');
                    if let Some(result) = method
                        .return_type
                        .clone()
                        .filter(|result| *result != ValueType::Scalar(ScalarType::None))
                    {
                        write!(self.output, " -> {}", rust_value_type(self.package, result))
                            .unwrap();
                    }
                    self.output.push_str(" {\n");
                    self.indent += 1;
                    let arguments = method
                        .parameters
                        .iter()
                        .map(|parameter| rust_name(&parameter.name))
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.line(&format!("self.0.{}({arguments})", rust_name(&method.name)));
                    self.indent -= 1;
                    self.line("}");
                }
                if self.object_requires_separation(&object.identity) {
                    self.line("fn terrane_separate(&self) -> Self { Self(self.0.separate_box()) }");
                }
                self.indent -= 1;
                self.line("}");
            }
            ObjectKind::Trait => {}
            ObjectKind::Class => {
                let fields = effective_object_fields(self.unit, object);
                let class_type = rust_object_type_name(self.package, &object.identity);
                let descendants = object_descendants(self.unit, object);
                let storage_type = if descendants.is_empty() {
                    class_type.clone()
                } else {
                    format!("{class_type}Storage")
                };
                let methods = effective_object_methods(self.unit, object);
                let has_destructor = methods.iter().any(|method| method.name == "destruct");
                if !object.resource_owning {
                    self.line("#[derive(Clone)]");
                }
                self.line(&format!("pub struct {storage_type} {{"));
                self.indent += 1;
                if has_destructor && !object.resource_owning {
                    self.line("__terrane_lifetime: std::sync::Arc<()>,");
                }
                for field in &fields {
                    self.line(&format!(
                        "pub {}: {},",
                        rust_name(&field.name),
                        rust_value_type(self.package, field.value_type.clone())
                    ));
                }
                self.indent -= 1;
                self.line("}");
                self.line(&format!("impl {storage_type} {{"));
                self.indent += 1;
                if let Some(construct) = methods.iter().find(|method| method.name == "construct") {
                    self.line_start();
                    write!(self.output, "pub fn terrane_construct(").unwrap();
                    for (index, parameter) in construct.parameters.iter().enumerate() {
                        if index != 0 {
                            self.output.push_str(", ");
                        }
                        let ty = parameter.value_type.clone().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        write!(self.output, "{}: {ty}", rust_name(&parameter.name)).unwrap();
                    }
                    self.output.push_str(") -> Self {\n");
                    self.indent += 1;
                    self.line("let mut value = Self {");
                    self.indent += 1;
                    for field in &fields {
                        let initializer = find_node_by_span(&self.unit.tree.root, field.span)
                            .and_then(|binding| {
                                binding
                                    .children
                                    .iter()
                                    .position(|child| child.kind == SyntaxKind::Name)
                                    .and_then(|index| binding_initializer(binding, index))
                            });
                        let value = initializer.map_or_else(
                            || match field.value_type {
                                ValueType::PlatformStreamHandle
                                | ValueType::PlatformResourceHandle
                                | ValueType::FilesystemAuthority => "Default::default()".to_owned(),
                                _ => "panic!(\"object field was not initialized\")".to_owned(),
                            },
                            |initializer| self.expression_as(initializer, field.value_type.clone()),
                        );
                        self.line(&format!("{}: {value},", rust_name(&field.name)));
                    }
                    if has_destructor && !object.resource_owning {
                        self.line("__terrane_lifetime: std::sync::Arc::new(()),");
                    }
                    self.indent -= 1;
                    self.line("};");
                    let arguments = construct
                        .parameters
                        .iter()
                        .map(|parameter| rust_name(&parameter.name))
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.line(&format!("value.construct({arguments});"));
                    self.line("value");
                    self.indent -= 1;
                    self.line("}");
                } else {
                    self.line("pub fn terrane_construct() -> Self {");
                    self.indent += 1;
                    self.line("Self {");
                    self.indent += 1;
                    for field in &fields {
                        let initializer = find_node_by_span(&self.unit.tree.root, field.span)
                            .and_then(|binding| {
                                binding
                                    .children
                                    .iter()
                                    .position(|child| child.kind == SyntaxKind::Name)
                                    .and_then(|index| binding_initializer(binding, index))
                            });
                        let value = initializer.map_or_else(
                            || match field.value_type {
                                ValueType::PlatformStreamHandle
                                | ValueType::PlatformResourceHandle
                                | ValueType::FilesystemAuthority => "Default::default()".to_owned(),
                                _ => "panic!(\"object field was not initialized\")".to_owned(),
                            },
                            |initializer| self.expression_as(initializer, field.value_type.clone()),
                        );
                        self.line(&format!("{}: {value},", rust_name(&field.name)));
                    }
                    if has_destructor && !object.resource_owning {
                        self.line("__terrane_lifetime: std::sync::Arc::new(()),");
                    }
                    self.indent -= 1;
                    self.line("}");
                    self.indent -= 1;
                    self.line("}");
                }
                if has_destructor && !object.resource_owning {
                    self.line("pub fn terrane_separate(&self) -> Self {");
                    self.indent += 1;
                    self.line("let mut value = self.clone();");
                    self.line("value.__terrane_lifetime = std::sync::Arc::new(());");
                    self.line("value");
                    self.indent -= 1;
                    self.line("}");
                }
                let previous_object = self.current_object.replace(object.identity.clone());
                for method in &methods {
                    let method_node = find_node(
                        &self.unit.tree.root,
                        SyntaxKind::FunctionDeclaration,
                        method.span,
                    )
                    .expect("object method contract must retain its syntax");
                    self.object_method(method_node);
                }
                let destructors = object_destructor_chain(self.unit, object);
                for (index, destructor) in destructors
                    .iter()
                    .take(destructors.len().saturating_sub(1))
                    .enumerate()
                {
                    let method_node = find_node(
                        &self.unit.tree.root,
                        SyntaxKind::FunctionDeclaration,
                        destructor.span,
                    )
                    .expect("destructor contract must retain its syntax");
                    self.object_method_as(method_node, &format!("terrane_destruct_{index}"));
                }
                self.current_object = previous_object;
                self.indent -= 1;
                self.line("}");
                if !descendants.is_empty() {
                    if !object.resource_owning {
                        self.line("#[derive(Clone)]");
                    }
                    self.line(&format!("pub enum {class_type} {{"));
                    self.indent += 1;
                    self.line(&format!("Own({storage_type}),"));
                    for descendant in &descendants {
                        let descendant_type =
                            rust_object_type_name(self.package, &descendant.identity);
                        self.line(&format!("{descendant_type}({descendant_type}),"));
                    }
                    self.indent -= 1;
                    self.line("}");
                    self.line(&format!("impl {class_type} {{"));
                    self.indent += 1;
                    if let Some(construct) =
                        methods.iter().find(|method| method.name == "construct")
                    {
                        self.line_start();
                        self.output.push_str("pub fn terrane_construct(");
                        for (index, parameter) in construct.parameters.iter().enumerate() {
                            if index != 0 {
                                self.output.push_str(", ");
                            }
                            let ty = parameter.value_type.clone().map_or_else(
                                || "i128".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            write!(self.output, "{}: {ty}", rust_name(&parameter.name)).unwrap();
                        }
                        self.output.push_str(") -> Self {\n");
                        self.indent += 1;
                        let arguments = construct
                            .parameters
                            .iter()
                            .map(|parameter| rust_name(&parameter.name))
                            .collect::<Vec<_>>()
                            .join(", ");
                        self.line(&format!(
                            "Self::Own({storage_type}::terrane_construct({arguments}))"
                        ));
                        self.indent -= 1;
                        self.line("}");
                    } else {
                        self.line(&format!(
                            "pub fn terrane_construct() -> Self {{ Self::Own({storage_type}::terrane_construct()) }}"
                        ));
                    }
                    let hierarchy_has_destructor = has_destructor
                        || descendants.iter().any(|descendant| {
                            effective_object_methods(self.unit, descendant)
                                .iter()
                                .any(|method| method.name == "destruct")
                        });
                    if hierarchy_has_destructor {
                        self.line("pub fn terrane_separate(&self) -> Self {");
                        self.indent += 1;
                        self.line("match self {");
                        self.indent += 1;
                        let own_copy = if has_destructor {
                            "value.terrane_separate()"
                        } else {
                            "value.clone()"
                        };
                        self.line(&format!("Self::Own(value) => Self::Own({own_copy}),"));
                        for descendant in &descendants {
                            let descendant_type =
                                rust_object_type_name(self.package, &descendant.identity);
                            let descendant_has_destructor =
                                effective_object_methods(self.unit, descendant)
                                    .iter()
                                    .any(|method| method.name == "destruct");
                            let copy = if descendant_has_destructor {
                                "value.terrane_separate()"
                            } else {
                                "value.clone()"
                            };
                            self.line(&format!(
                                "Self::{descendant_type}(value) => Self::{descendant_type}({copy}),"
                            ));
                        }
                        self.indent -= 1;
                        self.line("}");
                        self.indent -= 1;
                        self.line("}");
                    }
                    for method in methods
                        .iter()
                        .filter(|method| !matches!(method.name.as_str(), "construct" | "destruct"))
                    {
                        self.line_start();
                        let receiver = if method.consumes_receiver {
                            "self"
                        } else if method.mutates_receiver {
                            "&mut self"
                        } else {
                            "&self"
                        };
                        write!(self.output, "pub fn {}({receiver}", rust_name(&method.name))
                            .unwrap();
                        for parameter in &method.parameters {
                            let ty = parameter.value_type.clone().map_or_else(
                                || "i128".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            write!(self.output, ", {}: {ty}", rust_name(&parameter.name)).unwrap();
                        }
                        self.output.push(')');
                        if let Some(result) = method
                            .return_type
                            .clone()
                            .filter(|result| *result != ValueType::Scalar(ScalarType::None))
                        {
                            write!(self.output, " -> {}", rust_value_type(self.package, result))
                                .unwrap();
                        }
                        self.output.push_str(" {\n");
                        self.indent += 1;
                        self.line("match self {");
                        self.indent += 1;
                        let arguments = method
                            .parameters
                            .iter()
                            .map(|parameter| rust_name(&parameter.name))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let mut receiver_binding = "value".to_owned();
                        while method
                            .parameters
                            .iter()
                            .any(|parameter| rust_name(&parameter.name) == receiver_binding)
                        {
                            receiver_binding.push('_');
                        }
                        self.line(&format!(
                            "Self::Own({receiver_binding}) => {receiver_binding}.{}({arguments}),",
                            rust_name(&method.name)
                        ));
                        for descendant in &descendants {
                            let descendant_type =
                                rust_object_type_name(self.package, &descendant.identity);
                            self.line(&format!(
                                "Self::{descendant_type}({receiver_binding}) => {receiver_binding}.{}({arguments}),",
                                rust_name(&method.name)
                            ));
                        }
                        self.indent -= 1;
                        self.line("}");
                        self.indent -= 1;
                        self.line("}");
                    }
                    for field in &fields {
                        let field_name = rust_name(&field.name);
                        let field_type = rust_value_type(self.package, field.value_type.clone());
                        self.line(&format!(
                            "pub fn terrane_field_{field_name}(&self) -> &{field_type} {{"
                        ));
                        self.indent += 1;
                        self.line("match self {");
                        self.indent += 1;
                        self.line(&format!("Self::Own(value) => &value.{field_name},"));
                        for descendant in &descendants {
                            let descendant_type =
                                rust_object_type_name(self.package, &descendant.identity);
                            if self.unit.objects.iter().any(|candidate| {
                                candidate.base.as_ref() == Some(&descendant.identity)
                            }) {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => value.terrane_field_{field_name}(),"
                                ));
                            } else {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => &value.{field_name},"
                                ));
                            }
                        }
                        self.indent -= 1;
                        self.line("}");
                        self.indent -= 1;
                        self.line("}");
                        self.line(&format!(
                            "pub fn terrane_field_{field_name}_mut(&mut self) -> &mut {field_type} {{"
                        ));
                        self.indent += 1;
                        self.line("match self {");
                        self.indent += 1;
                        self.line(&format!("Self::Own(value) => &mut value.{field_name},"));
                        for descendant in &descendants {
                            let descendant_type =
                                rust_object_type_name(self.package, &descendant.identity);
                            if self.unit.objects.iter().any(|candidate| {
                                candidate.base.as_ref() == Some(&descendant.identity)
                            }) {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => value.terrane_field_{field_name}_mut(),"
                                ));
                            } else {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => &mut value.{field_name},"
                                ));
                            }
                        }
                        self.indent -= 1;
                        self.line("}");
                        self.indent -= 1;
                        self.line("}");
                    }
                    self.indent -= 1;
                    self.line("}");
                }
                for interface_identity in effective_object_interfaces(self.unit, object) {
                    if interface_identity.namespace == "/core/errors"
                        && interface_identity.name == "throwable"
                    {
                        continue;
                    }
                    let interface_unit = self
                        .package
                        .units
                        .iter()
                        .find(|candidate| candidate.namespace == interface_identity.namespace)
                        .expect("resolved interface namespace");
                    let interface = interface_unit
                        .objects
                        .iter()
                        .find(|candidate| candidate.identity == *interface_identity)
                        .expect("validated interface contract");
                    let interface_type = rust_object_type_name(self.package, &interface.identity);
                    let protocol = format!("{interface_type}Protocol");
                    let class_type = rust_object_type_name(self.package, &object.identity);
                    self.line(&format!("impl {protocol} for {class_type} {{"));
                    self.indent += 1;
                    self.line(&format!(
                        "fn clone_box(&self) -> Box<dyn {protocol}> {{ Box::new(self.clone()) }}"
                    ));
                    if self.object_requires_separation(&object.identity) {
                        self.line(&format!(
                            "fn separate_box(&self) -> Box<dyn {protocol}> {{ Box::new(self.terrane_separate()) }}"
                        ));
                    } else {
                        self.line(&format!(
                            "fn separate_box(&self) -> Box<dyn {protocol}> {{ Box::new(self.clone()) }}"
                        ));
                    }
                    for method in effective_object_methods(interface_unit, interface) {
                        self.line_start();
                        let receiver = if method.consumes_receiver {
                            "self: Box<Self>"
                        } else if method.mutates_receiver {
                            "&mut self"
                        } else {
                            "&self"
                        };
                        write!(self.output, "fn {}({receiver}", rust_name(&method.name)).unwrap();
                        for parameter in &method.parameters {
                            let ty = parameter.value_type.clone().map_or_else(
                                || "i128".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            write!(self.output, ", {}: {ty}", rust_name(&parameter.name)).unwrap();
                        }
                        self.output.push(')');
                        if let Some(result) = method
                            .return_type
                            .clone()
                            .filter(|result| *result != ValueType::Scalar(ScalarType::None))
                        {
                            write!(self.output, " -> {}", rust_value_type(self.package, result))
                                .unwrap();
                        }
                        self.output.push_str(" {\n");
                        self.indent += 1;
                        let arguments = method
                            .parameters
                            .iter()
                            .map(|parameter| rust_name(&parameter.name))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let receiver = if method.consumes_receiver {
                            "*self"
                        } else {
                            "self"
                        };
                        self.line(&format!(
                            "{class_type}::{}({receiver}, {arguments})",
                            rust_name(&method.name)
                        ));
                        self.indent -= 1;
                        self.line("}");
                    }
                    self.indent -= 1;
                    self.line("}");
                    self.line(&format!(
                        "impl From<{class_type}> for {interface_type} {{ fn from(value: {class_type}) -> Self {{ Self(Box::new(value)) }} }}"
                    ));
                }
                if has_destructor {
                    self.line(&format!("impl Drop for {storage_type} {{"));
                    self.indent += 1;
                    self.line("fn drop(&mut self) {");
                    self.indent += 1;
                    if object.resource_owning {
                        self.line("self.destruct();");
                        for index in (0..object_destructor_chain(self.unit, object)
                            .len()
                            .saturating_sub(1))
                            .rev()
                        {
                            self.line(&format!("self.terrane_destruct_{index}();"));
                        }
                    } else {
                        self.line(
                            "if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {",
                        );
                        self.indent += 1;
                        self.line("self.destruct();");
                        for index in (0..object_destructor_chain(self.unit, object)
                            .len()
                            .saturating_sub(1))
                            .rev()
                        {
                            self.line(&format!("self.terrane_destruct_{index}();"));
                        }
                        self.indent -= 1;
                        self.line("}");
                    }
                    self.indent -= 1;
                    self.line("}");
                    self.indent -= 1;
                    self.line("}");
                }
            }
        }
    }

    fn function(&mut self, node: &SyntaxNode) {
        self.emit_function(node, None);
    }

    fn object_method(&mut self, node: &SyntaxNode) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("object method must have an analyzed contract");
        let receiver = if contract.consumes_receiver {
            "self"
        } else if contract.mutates_receiver {
            "&mut self"
        } else {
            "&self"
        };
        self.emit_function_as(node, Some(receiver), None);
    }
    fn object_method_as(&mut self, node: &SyntaxNode, name: &str) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("object method must have an analyzed contract");
        let receiver = if contract.consumes_receiver {
            "self"
        } else if contract.mutates_receiver {
            "&mut self"
        } else {
            "&self"
        };
        self.emit_function_as(node, Some(receiver), Some(name));
    }

    fn emit_function(&mut self, node: &SyntaxNode, receiver: Option<&str>) {
        self.emit_function_as(node, receiver, None);
    }

    #[expect(
        clippy::too_many_lines,
        reason = "function lowering preserves one ordered signature and body pipeline"
    )]
    fn emit_function_as(
        &mut self,
        node: &SyntaxNode,
        receiver: Option<&str>,
        name_override: Option<&str>,
    ) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|item| item.span == node.span)
            .expect("analyzed function declaration must have a semantic contract");
        self.line_start();
        let name = name_override.map_or_else(|| function_name(contract), str::to_owned);
        let async_main = contract.is_async && contract.name == "main" && receiver.is_none();
        write!(
            self.output,
            "{}{}fn {name}(",
            if (receiver.is_some() && name_override.is_none())
                || (receiver.is_none() && self.unit.bundled)
            {
                "pub "
            } else {
                ""
            },
            if contract.is_async && !async_main {
                "async "
            } else {
                ""
            }
        )
        .unwrap();
        if let Some(receiver) = receiver {
            self.output.push_str(receiver);
        }
        for (index, parameter) in contract.parameters.iter().enumerate() {
            if receiver.is_some() || index != 0 {
                self.output.push_str(", ");
            }
            let ty = parameter.value_type.clone().map_or_else(
                || "i128".to_owned(),
                |value_type| rust_value_type(self.package, value_type),
            );
            let mutable = if parameter.mutable { "mut " } else { "" };
            write!(self.output, "{mutable}{}: {ty}", rust_name(&parameter.name)).unwrap();
        }
        self.output.push(')');
        let function_errors = contract.throws && contract.name != "main";
        if function_errors {
            let result = contract.return_type.clone().map_or_else(
                || "()".to_owned(),
                |value_type| rust_value_type(self.package, value_type),
            );
            write!(self.output, " -> Result<{result}, TerraneError>").unwrap();
        } else if let Some(return_type) = contract.return_type.clone()
            && return_type != ValueType::Scalar(ScalarType::None)
        {
            write!(
                self.output,
                " -> {}",
                rust_value_type(self.package, return_type)
            )
            .unwrap();
        }
        let block = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block);
        if block.is_none_or(|block| block.children.is_empty())
            && contract.parameters.is_empty()
            && !async_main
            && !function_errors
        {
            self.output.push_str(" {}\n");
            return;
        }
        if async_main
            && block.is_none_or(|block| block.children.is_empty())
            && contract.parameters.is_empty()
        {
            self.output.push_str(" {\n");
            self.indent += 1;
            self.line("__terrane_block_on(async move {});");
            self.indent -= 1;
            self.line("}");
            return;
        }
        self.output.push_str(" {\n");
        if async_main {
            self.indent += 1;
            self.line("__terrane_block_on(async move {");
        }
        let outer_return_type =
            std::mem::replace(&mut self.return_type, contract.return_type.clone());
        let outer_function_errors = std::mem::replace(&mut self.function_errors, function_errors);
        let outer_propagation = std::mem::replace(&mut self.propagate_errors, function_errors);
        let outer_function = self.current_function.replace(format!(
            "{}::{}",
            self.unit.namespace.trim_end_matches('/'),
            contract.name
        ));
        let outer_parameter_types = std::mem::replace(
            &mut self.parameter_types,
            contract
                .parameters
                .iter()
                .filter_map(|parameter| {
                    parameter
                        .value_type
                        .clone()
                        .map(|value_type| (parameter.name.clone(), value_type))
                })
                .collect(),
        );
        self.indent += 1;
        let unused_parameters = contract
            .parameters
            .iter()
            .filter(|parameter| {
                !binding_store_value_is_read(self.package, parameter.span, parameter.span)
            })
            .map(|parameter| format!("&{}", rust_name(&parameter.name)))
            .collect::<Vec<_>>();
        match unused_parameters.as_slice() {
            [] => {}
            [parameter] => self.line(&format!("let _ = {parameter};")),
            parameters => self.line(&format!("let _ = ({});", parameters.join(", "))),
        }
        if let Some(block) = block {
            self.block(block);
        }
        if function_errors
            && contract
                .return_type
                .clone()
                .is_none_or(|ty| ty == ValueType::Scalar(ScalarType::None))
            && node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
                .is_some_and(block_may_fall_through)
        {
            self.line("Ok(())");
        }
        self.return_type = outer_return_type;
        self.function_errors = outer_function_errors;
        self.propagate_errors = outer_propagation;
        self.parameter_types = outer_parameter_types;
        self.current_function = outer_function;
        self.indent -= 1;
        if async_main {
            self.line("});");
            self.indent -= 1;
        }
        self.line("}");
    }

    fn anonymous_function(&mut self, node: &SyntaxNode) -> String {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed closure must have a semantic contract");
        let parameters = contract
            .parameters
            .iter()
            .map(|parameter| {
                let ty = parameter.value_type.clone().map_or_else(
                    || "i128".to_owned(),
                    |value_type| rust_value_type(self.package, value_type),
                );
                format!("{}: {ty}", rust_name(&parameter.name))
            })
            .collect::<Vec<_>>()
            .join(", ");
        let result = contract
            .return_type
            .clone()
            .unwrap_or(ValueType::Scalar(ScalarType::None));
        let result_type = rust_value_type(self.package, result.clone());
        let outer_output = std::mem::take(&mut self.output);
        let outer_indent = self.indent;
        let outer_return_type = self.return_type.replace(result);
        let outer_function_errors = std::mem::replace(&mut self.function_errors, false);
        let outer_propagation = std::mem::replace(&mut self.propagate_errors, false);
        let outer_parameter_types = std::mem::replace(
            &mut self.parameter_types,
            contract
                .parameters
                .iter()
                .filter_map(|parameter| {
                    parameter
                        .value_type
                        .clone()
                        .map(|ty| (parameter.name.clone(), ty))
                })
                .collect(),
        );
        self.closure_depth += 1;
        self.indent = outer_indent + 1;
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            self.block(block);
        }
        self.closure_depth -= 1;
        let body = std::mem::replace(&mut self.output, outer_output);
        self.indent = outer_indent;
        self.return_type = outer_return_type;
        self.function_errors = outer_function_errors;
        self.propagate_errors = outer_propagation;
        self.parameter_types = outer_parameter_types;
        let mut captures = String::new();
        for capture in &contract.captures {
            let name = rust_name(capture);
            let source = if capture == "this" { "self" } else { &name };
            let transfer = self
                .unit
                .typed_bindings
                .iter()
                .rev()
                .find(|binding| {
                    binding.name == *capture
                        && binding.is_visible_at(self.source.id(), node.span.start)
                })
                .is_some_and(|binding| self.value_type_owns_resource(&binding.value_type));
            if transfer {
                write!(captures, "let {name} = {source}; ")
                    .expect("writing to a String cannot fail");
            } else {
                write!(captures, "let {name} = {source}.clone(); ")
                    .expect("writing to a String cannot fail");
            }
        }
        format!(
            "{{ {captures}std::sync::Arc::new(move |{parameters}| -> {result_type} {{\n{body}{}}}) }}",
            "    ".repeat(outer_indent)
        )
    }

    fn block(&mut self, block: &SyntaxNode) {
        for statement in &block.children {
            self.statement(statement);
        }
    }

    fn union_binding(&self, node: &SyntaxNode) -> Option<TypedBinding> {
        (node.kind == SyntaxKind::Name)
            .then(|| {
                self.unit
                    .typed_bindings
                    .iter()
                    .rev()
                    .find(|binding| {
                        binding.name == self.text(node)
                            && binding.is_visible_at(self.source.id(), node.span.start)
                            && !binding.destination_arms.is_empty()
                    })
                    .cloned()
            })
            .flatten()
    }

    fn union_value(&mut self, binding: &TypedBinding, value: &SyntaxNode) -> String {
        let actual = self
            .value_type(value)
            .and_then(|value_type| match value_type {
                ValueType::Scalar(scalar) => Some(scalar),
                _ => None,
            });
        let constant = binding
            .destination_arms
            .iter()
            .any(|arm| contextual_constant(self.source, value, *arm).is_some());
        let selected = (!constant)
            .then_some(actual)
            .flatten()
            .filter(|actual| binding.destination_arms.contains(actual))
            .or_else(|| {
                binding.destination_arms.iter().copied().find(|arm| {
                    contextual_constant(self.source, value, *arm)
                        .is_some_and(|result| result.is_ok())
                })
            })
            .or_else(|| {
                actual.and_then(|actual| {
                    is_numeric(actual).then(|| {
                        binding
                            .destination_arms
                            .iter()
                            .copied()
                            .find(|arm| is_numeric(*arm))
                            .expect("validated numeric union destination")
                    })
                })
            })
            .expect("validated union destination");
        let index = binding
            .destination_arms
            .iter()
            .position(|arm| *arm == selected)
            .expect("selected union arm belongs to destination");
        format!(
            "{}::Arm{index}({})",
            union_type_name(binding),
            self.expression_as(value, ValueType::Scalar(selected))
        )
    }

    fn emit_union_types(&mut self) {
        for binding in self
            .unit
            .typed_bindings
            .iter()
            .filter(|binding| !binding.destination_arms.is_empty())
        {
            let name = union_type_name(binding);
            self.line("#[allow(dead_code)]");
            self.line("#[derive(Clone)]");
            self.line(&format!("enum {name} {{"));
            self.indent += 1;
            for (index, arm) in binding.destination_arms.iter().enumerate() {
                self.line(&format!("Arm{index}({}),", rust_type(*arm)));
            }
            self.indent -= 1;
            self.line("}");
            self.line(&format!(
                "impl terrane_scalar_support::ScalarDisplay for {name} {{"
            ));
            self.indent += 1;
            self.line("fn write_scalar(&self, output: &mut String) {");
            self.indent += 1;
            self.line("match self {");
            self.indent += 1;
            for (index, _) in binding.destination_arms.iter().enumerate() {
                self.line(&format!(
                    "Self::Arm{index}(value) => terrane_scalar_support::ScalarDisplay::write_scalar(value, output),"
                ));
            }
            self.indent -= 1;
            self.line("}");
            self.indent -= 1;
            self.line("}");
            self.indent -= 1;
            self.line("}");
        }
    }

    fn statement(&mut self, node: &SyntaxNode) {
        match node.kind {
            SyntaxKind::Binding => {
                if !self.global_assignment(node) {
                    self.binding(node);
                }
            }
            SyntaxKind::Assignment => self.assignment(node),
            SyntaxKind::CallExpression => {
                let expression = if let Some(expression) = self.collection_mutation_statement(node)
                {
                    expression
                } else {
                    self.discarded_call = Some(node.span);
                    let expression = self.expression(node);
                    self.discarded_call = None;
                    expression
                };
                self.line(&format!("{expression};"));
            }
            SyntaxKind::PostfixExpression => self.postfix(node),
            SyntaxKind::IfStatement => self.if_statement(node),
            SyntaxKind::WhileStatement => self.while_statement(node),
            SyntaxKind::ForStatement => self.for_statement(node),
            SyntaxKind::ReturnStatement => {
                let value = node.children.first().map_or_else(
                    || "()".to_owned(),
                    |value| {
                        let value = if let Some(return_type) = self.return_type.clone() {
                            self.expression_as(value, return_type)
                        } else {
                            self.expression(value)
                        };
                        Self::unwrapped_expression(value)
                    },
                );
                if self.try_completion {
                    self.line(&format!("return TerraneCompletion::Return({value});"));
                } else if self.function_errors {
                    self.line(&format!("return Ok({value});"));
                } else if self.propagate_errors {
                    self.line(&format!("return Ok(Some({value}));"));
                } else {
                    self.line(&format!("return {value};"));
                }
            }
            SyntaxKind::ThrowStatement => self.throw_statement(node),
            SyntaxKind::TryStatement => self.try_statement(node),
            SyntaxKind::BreakStatement if self.try_completion => {
                self.line("return TerraneCompletion::Break;");
            }
            SyntaxKind::BreakStatement => self.line("break;"),
            SyntaxKind::ContinueStatement if self.try_completion => {
                self.line("return TerraneCompletion::Continue;");
            }
            SyntaxKind::ContinueStatement => {
                if let Some(label) = &self.continue_label {
                    self.line(&format!("break '{label};"));
                } else {
                    self.line("continue;");
                }
            }
            _ => {}
        }
    }

    fn collection_mutation_statement(&mut self, node: &SyntaxNode) -> Option<String> {
        let [callee, arguments] = node.children.as_slice() else {
            return None;
        };
        let [receiver, member] = callee.children.as_slice() else {
            return None;
        };
        if callee.kind != SyntaxKind::MemberExpression {
            return None;
        }
        let receiver_type = self.receiver_value_type(receiver)?;
        let receiver_value = self.receiver_guard_expression(receiver);
        let values = arguments
            .children
            .iter()
            .map(|argument| argument.children.last().unwrap_or(argument))
            .collect::<Vec<_>>();
        let mutation = match (receiver_type, self.text(member)) {
            (ValueType::List(item), "append") => Some(format!(
                "({receiver_value}).append({})",
                self.expression_as(values[0], item.value_type())
            )),
            (ValueType::List(item), "set") => {
                let index = self.expression_as(values[0], ValueType::Scalar(ScalarType::Int));
                let index = self.fallible(
                    format!("terrane_collection_support::index_from_int(&({index}))"),
                    node,
                );
                let value = self.expression_as(values[1], item.value_type());
                Some(self.fallible(format!("({receiver_value}).set({index}, {value})"), node))
            }
            (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "set") => {
                Some(format!(
                    "({receiver_value}).set({}, {})",
                    self.expression_as(values[0], key.value_type()),
                    self.expression_as(values[1], value.value_type())
                ))
            }
            (ValueType::Set(item) | ValueType::UnorderedSet(item), "add") => Some(format!(
                "({receiver_value}).add({})",
                self.expression_as(values[0], item.value_type())
            )),
            _ => None,
        };
        mutation.map(|mutation| self.wrap_receiver_guard(receiver, mutation))
    }

    fn assignment(&mut self, node: &SyntaxNode) {
        if self.global_assignment(node) {
            return;
        }
        if let [target, value] = node.children.as_slice()
            && target.kind == SyntaxKind::IndexExpression
            && let [receiver, index] = target.children.as_slice()
            && let Some(receiver_type) = self.receiver_value_type(receiver)
        {
            let receiver_value = self.receiver_guard_expression(receiver);
            match receiver_type {
                ValueType::List(item) => {
                    let index = self.expression_as(index, ValueType::Scalar(ScalarType::Int));
                    let index = self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index}))"),
                        node,
                    );
                    let value = self.expression_as(value, item.value_type());
                    let mutation =
                        self.fallible(format!("({receiver_value}).set({index}, {value})"), node);
                    let mutation = self.wrap_receiver_guard(receiver, mutation);
                    self.line(&format!("let _ = {mutation};"));
                }
                ValueType::Map(key, value_type) | ValueType::UnorderedMap(key, value_type) => {
                    let key = self.expression_as(index, key.value_type());
                    let value = self.expression_as(value, value_type.value_type());
                    let mutation = format!("({receiver_value}).set({key}, {value})");
                    let mutation = self.wrap_receiver_guard(receiver, mutation);
                    self.line(&format!("let _ = {mutation};"));
                }
                _ => {}
            }
            return;
        }
        if self
            .unit
            .typed_bindings
            .iter()
            .any(|binding| binding.span == node.span)
        {
            self.binding(node);
            return;
        }
        let [left, right] = node.children.as_slice() else {
            return;
        };
        let assigned_binding = (left.kind == SyntaxKind::Name)
            .then(|| {
                self.package
                    .resolve_name_at(self.unit, left.span.start, self.text(left))
            })
            .flatten()
            .and_then(|symbol| symbol.declaration_span)
            .and_then(|span| {
                self.unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == span)
            });
        let union_binding = self.union_binding(left);
        let value_type = assigned_binding
            .map(|binding| binding.value_type.clone())
            .or_else(|| self.value_type(left));
        let value = if let Some(binding) = union_binding {
            self.union_value(&binding, right)
        } else if let Some(value_type) = value_type {
            self.expression_as(right, value_type)
        } else {
            self.expression(right)
        };
        let value = Self::unwrapped_expression(value);
        let reference_backed =
            assigned_binding.is_some_and(|binding| self.reference_backed(binding));
        let target = if reference_backed {
            format!(
                "*{}.lock().expect(\"reference lock poisoned\")",
                rust_name(self.text(left))
            )
        } else if left.kind == SyntaxKind::Name {
            rust_name(self.text(left))
        } else if let [receiver, member] = left.children.as_slice()
            && left.kind == SyntaxKind::MemberExpression
            && self.wrapped_object_field(receiver, self.text(member))
        {
            format!(
                "*({}).terrane_field_{}_mut()",
                self.receiver_expression(receiver),
                rust_name(self.text(member))
            )
        } else {
            self.expression(left)
        };
        self.line(&format!("{target} = {value};"));
        if let Some(binding) = assigned_binding
            && !reference_backed
            && !binding_store_value_is_read(self.package, binding.span, node.span)
        {
            self.line(&format!("let _ = &mut {target};"));
        }
    }

    fn error_kind(&self, node: &SyntaxNode) -> String {
        let descriptor = if node.kind == SyntaxKind::CallExpression {
            node.children.first().unwrap_or(node)
        } else {
            node
        };
        self.package
            .resolve_name_at(self.unit, descriptor.span.start, self.text(descriptor))
            .map_or_else(
                || {
                    self.text(descriptor)
                        .trim()
                        .trim_start_matches('.')
                        .to_owned()
                },
                |symbol| symbol.name.clone(),
            )
    }

    fn rust_error_kind(&self, node: &SyntaxNode) -> String {
        let descriptor = if node.kind == SyntaxKind::CallExpression {
            node.children.first().unwrap_or(node)
        } else {
            node
        };
        if let Some(symbol) =
            self.package
                .resolve_name_at(self.unit, descriptor.span.start, self.text(descriptor))
        {
            if let Some(kind) = rust_builtin_error_kind(&symbol.name) {
                return kind.to_owned();
            }
            let descriptor = self
                .registry
                .register_descriptor(&symbol.identity, &symbol.name);
            return format!("Custom(DescriptorId({descriptor}))");
        }
        let name = self.error_kind(node);
        if let Some(kind) = rust_builtin_error_kind(&name) {
            kind.to_owned()
        } else {
            let identity = format!("{}::{name}", self.unit.namespace.trim_end_matches('/'));
            let descriptor = self.registry.register_descriptor(&identity, &name);
            format!("Custom(DescriptorId({descriptor}))")
        }
    }

    fn throw_statement(&mut self, node: &SyntaxNode) {
        if let Some(current_error) = &self.current_error
            && node.children.is_empty()
        {
            let frame = self.error_site(node);
            let error = format!("{current_error}.clone().at({frame})");
            if self.try_completion {
                self.line(&format!("return TerraneCompletion::Error({error});"));
            } else if self.propagate_errors {
                self.line(&format!("return Err({error});"));
            } else {
                self.line(&format!("__terrane_uncaught({error});"));
            }
            return;
        }
        let Some(error_node) = node.children.first() else {
            return;
        };
        let kind = self.rust_error_kind(error_node);
        let origin = self.error_site(node);
        let mut error = if kind.starts_with("Custom(") {
            let value = self.expression(error_node);
            format!(
                "{{ let value = {value}; TerraneError::raised_with_message(TerraneErrorKind::{kind}, value.render(), {origin}) }}"
            )
        } else {
            format!("TerraneError::raised(TerraneErrorKind::{kind}, {origin})")
        };
        if let Some(current_error) = &self.current_error {
            error = format!("{error}.with_cause({current_error}.clone())");
        }
        if self.try_completion {
            self.line(&format!("return TerraneCompletion::Error({error});"));
        } else if self.propagate_errors {
            self.line(&format!("return Err({error});"));
        } else {
            self.line(&format!("__terrane_uncaught({error});"));
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "the emitted try/catch/finally control flow is clearer as one auditable state machine"
    )]
    fn try_statement(&mut self, node: &SyntaxNode) {
        let Some(block) = node.children.first() else {
            return;
        };
        let index = self.try_counter;
        self.try_counter += 1;
        let result = self.return_type.clone().map_or_else(
            || "()".to_owned(),
            |value_type| rust_value_type(self.package, value_type),
        );
        let mutable = if node
            .children
            .iter()
            .any(|child| child.kind == SyntaxKind::FinallyClause)
        {
            "mut "
        } else {
            ""
        };
        self.line(&format!(
            "let {mutable}__terrane_completion_{index}: TerraneCompletion<{result}> = (|| {{"
        ));
        self.indent += 1;
        self.line(&format!(
            "let __terrane_try_{index}: TerraneCompletion<{result}> = (|| {{"
        ));
        self.indent += 1;
        let outer_completion = std::mem::replace(&mut self.try_completion, true);
        let outer_propagation = std::mem::replace(&mut self.propagate_errors, true);
        let outer_function_errors = std::mem::replace(&mut self.function_errors, false);
        self.block(block);
        if block_may_fall_through(block) {
            self.line("TerraneCompletion::Normal");
        }
        self.function_errors = outer_function_errors;
        self.propagate_errors = outer_propagation;
        self.indent -= 1;
        self.line("})();");
        self.line(&format!("match __terrane_try_{index} {{"));
        self.indent += 1;
        self.line("TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),");
        self.line("TerraneCompletion::Break => return TerraneCompletion::Break,");
        self.line("TerraneCompletion::Continue => return TerraneCompletion::Continue,");
        self.line("TerraneCompletion::Normal => {}");
        self.line(&format!(
            "TerraneCompletion::Error(__terrane_error_{index}) => {{"
        ));
        self.indent += 1;
        self.line(&format!("let mut __terrane_handled_{index} = false;"));
        for clause in node
            .children
            .iter()
            .filter(|child| child.kind == SyntaxKind::CatchClause)
        {
            let descriptor = clause
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name);
            let condition = descriptor.map_or_else(
                || format!("!__terrane_handled_{index}"),
                |descriptor| {
                    let name = self.error_kind(descriptor);
                    if name == "error" {
                        format!("!__terrane_handled_{index}")
                    } else {
                        format!(
                            "!__terrane_handled_{index} && __terrane_error_{index}.kind == TerraneErrorKind::{}",
                            self.rust_error_kind(descriptor)
                        )
                    }
                },
            );
            self.line(&format!("if {condition} {{"));
            self.indent += 1;
            self.line(&format!("__terrane_handled_{index} = true;"));
            let outer_error = self
                .current_error
                .replace(format!("__terrane_error_{index}"));
            if let Some(catch_block) = clause.children.last() {
                self.block(catch_block);
            }
            self.current_error = outer_error;
            self.indent -= 1;
            self.line("}");
        }
        self.try_completion = outer_completion;
        self.line(&format!("if !__terrane_handled_{index} {{"));
        self.indent += 1;
        self.line(&format!(
            "return TerraneCompletion::Error(__terrane_error_{index});"
        ));
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
        self.line("TerraneCompletion::Normal");
        self.indent -= 1;
        self.line("})();");
        if let Some(finally) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::FinallyClause)
            .and_then(|clause| clause.children.first())
        {
            self.line(&format!(
                "let __terrane_finally_{index}: TerraneCompletion<{result}> = (|| {{"
            ));
            self.indent += 1;
            let outer_completion = std::mem::replace(&mut self.try_completion, true);
            let outer_propagation = std::mem::replace(&mut self.propagate_errors, true);
            let outer_function_errors = std::mem::replace(&mut self.function_errors, false);
            self.block(finally);
            if block_may_fall_through(finally) {
                self.line("TerraneCompletion::Normal");
            }
            self.function_errors = outer_function_errors;
            self.propagate_errors = outer_propagation;
            self.try_completion = outer_completion;
            self.indent -= 1;
            self.line("})();");
            self.line(&format!("match __terrane_finally_{index} {{"));
            self.indent += 1;
            self.line("TerraneCompletion::Normal => {}");
            self.line(&format!(
                "replacement => __terrane_completion_{index} = replacement,"
            ));
            self.indent -= 1;
            self.line("}");
        }
        self.line(&format!("match __terrane_completion_{index} {{"));
        self.indent += 1;
        if statement_may_fall_through(node) {
            self.line("TerraneCompletion::Normal => {}");
        } else {
            self.line("TerraneCompletion::Normal => __terrane_generated_defect(\"non-fallthrough try completed normally\"),");
        }
        if self.try_completion {
            self.line(
                "TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),",
            );
            self.line("TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),");
        } else if self.function_errors {
            self.line("TerraneCompletion::Return(value) => return Ok(value),");
            self.line("TerraneCompletion::Error(error) => return Err(error),");
        } else if self.propagate_errors {
            self.line("TerraneCompletion::Return(value) => return Ok(Some(value)),");
            self.line("TerraneCompletion::Error(error) => return Err(error),");
        } else {
            self.line("TerraneCompletion::Return(value) => return value,");
            self.line("TerraneCompletion::Error(error) => __terrane_uncaught(error),");
        }
        if self.in_loop {
            self.line("TerraneCompletion::Break => break,");
            if let Some(label) = &self.continue_label {
                self.line(&format!("TerraneCompletion::Continue => break '{label},"));
            } else {
                self.line("TerraneCompletion::Continue => continue,");
            }
        } else {
            self.line("TerraneCompletion::Break | TerraneCompletion::Continue => __terrane_generated_defect(\"loop control escaped a non-loop try\"),");
        }
        self.indent -= 1;
        self.line("}");
    }

    fn binding(&mut self, node: &SyntaxNode) {
        let Some((name_index, name_node)) = node
            .children
            .iter()
            .enumerate()
            .find(|(_, child)| child.kind == SyntaxKind::Name)
        else {
            return;
        };
        let name = rust_name(self.text(name_node));
        let binding = self
            .unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == node.span);
        let reference_backed = binding.is_some_and(|binding| self.reference_backed(binding));
        let storage_type = binding
            .and_then(|binding| binding.storage_type)
            .filter(|_| !reference_backed)
            .filter(|_| !binding_span_is_mutated(self.package, self.unit, node.span, true));
        let ty = binding.map(|binding| {
            let value_type = if !binding.destination_arms.is_empty() {
                union_type_name(binding)
            } else if let Some(storage_type) = storage_type {
                rust_type(storage_type).to_owned()
            } else {
                rust_value_type(self.package, binding.value_type.clone())
            };
            if reference_backed {
                format!("std::sync::Arc<std::sync::Mutex<{value_type}>>")
            } else {
                value_type
            }
        });
        let initializer = binding_initializer(node, name_index);
        assert!(
            initializer.is_some() || !self.text(node).contains('='),
            "analyzed initialized value binding must have a selected initializer"
        );
        let mutable = !reference_backed
            && binding.is_some_and(|binding| {
                binding.mutable
                    && !matches!(
                        binding.value_type,
                        ValueType::Reference(_) | ValueType::SharedReference(_)
                    )
            });
        if self
            .package
            .is_lexical_replacement(self.unit, node.span, self.text(name_node))
        {
            self.line(&format!("let _ = &{name};"));
        }
        self.line_start();
        self.output.push_str("let ");
        if mutable {
            self.output.push_str("mut ");
        }
        self.output.push_str(&name);
        if let Some(ty) = ty {
            write!(self.output, ": {ty}").unwrap();
        }
        if let Some(initializer) = initializer {
            let value = if let Some(binding) = binding
                && !binding.destination_arms.is_empty()
            {
                self.union_value(binding, initializer)
            } else if let Some(storage_type) = storage_type {
                self.expression_as(initializer, ValueType::Scalar(storage_type))
            } else if let Some(binding) = binding {
                self.expression_as(initializer, binding.value_type.clone())
            } else {
                self.expression(initializer)
            };
            let value = Self::unwrapped_expression(value);
            let value = if reference_backed {
                format!("std::sync::Arc::new(std::sync::Mutex::new({value}))")
            } else {
                value
            };
            write!(self.output, " = {value}").unwrap();
        }
        self.output.push_str(";\n");
        if initializer.is_some() && !binding_store_value_is_read(self.package, node.span, node.span)
        {
            let borrow = if mutable { "&mut " } else { "&" };
            self.line(&format!("let _ = {borrow}{name};"));
        }
    }

    fn postfix(&mut self, node: &SyntaxNode) {
        let Some(value) = node.children.first() else {
            return;
        };
        let operator = &self.source.text()[value.span.end..node.span.end];
        let addition = operator.trim() == "++";
        let value_type = self.value_type(value);
        if let Some(storage) = self.global_storage(value) {
            self.line("{");
            self.indent += 1;
            self.line(&format!(
                "let mut value = {storage}.lock().expect(\"program-global lock poisoned\");"
            ));
            let failure = self.uninitialized_global_failure(value);
            let current = format!("value.clone().unwrap_or_else(|| {failure})");
            let updated = self.postfix_updated_value(&current, value_type, addition, node);
            self.line(&format!("*value = Some({updated});"));
            self.indent -= 1;
            self.line("}");
            return;
        }
        let target = self.expression(value);
        let updated = self.postfix_updated_value(&target, value_type, addition, node);
        self.line(&format!("{target} = {updated};"));
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "the optional recursive value type is matched as a complete lowering decision"
    )]
    fn postfix_updated_value(
        &self,
        value: &str,
        value_type: Option<ValueType>,
        addition: bool,
        node: &SyntaxNode,
    ) -> String {
        if value_type == Some(ValueType::Scalar(ScalarType::Int)) {
            let operator = if addition { "+" } else { "-" };
            return format!("{value}.clone() {operator} terrane_int_support::Int::from(1_i128)");
        }
        if matches!(value_type, Some(ValueType::Scalar(ty)) if ty.is_integer()) {
            if addition
                && node
                    .children
                    .first()
                    .is_some_and(|target| self.binding_has_bounded_integer_range(target))
            {
                return format!("{value} + 1");
            }
            let helper = if addition {
                "fixed_addition"
            } else {
                "fixed_subtraction"
            };
            return self.fallible(format!("terrane_int_support::{helper}({value}, 1)"), node);
        }
        unreachable!("semantic analysis validated postfix integer target");
    }

    fn if_statement(&mut self, node: &SyntaxNode) {
        let Some(condition) = node.children.first() else {
            return;
        };
        let Some(block) = node.children.get(1) else {
            return;
        };
        let condition = self.control_condition(condition);
        self.line(&format!("if {condition} {{"));
        self.indent += 1;
        self.block(block);
        self.indent -= 1;
        for clause in node.children.iter().skip(2) {
            self.line_start();
            if clause.children.len() == 1 {
                self.output.push_str("} else {\n");
                self.indent += 1;
                self.block(&clause.children[0]);
                self.indent -= 1;
            } else if let [condition, block] = clause.children.as_slice() {
                let condition = self.control_condition(condition);
                writeln!(self.output, "}} else if {condition} {{").unwrap();
                self.indent += 1;
                self.block(block);
                self.indent -= 1;
            }
        }
        self.line("}");
    }
    fn while_statement(&mut self, node: &SyntaxNode) {
        let [condition, block] = node.children.as_slice() else {
            return;
        };
        let bounded_range = self.bounded_integer_range(condition, block);
        let has_bounded_range = bounded_range.is_some();
        let condition = self.control_condition(condition);
        self.line(&format!("while {condition} {{"));
        self.indent += 1;
        let outer_continue = self.continue_label.take();
        let outer_loop = std::mem::replace(&mut self.in_loop, true);
        if let Some(range) = bounded_range {
            self.bounded_integer_ranges.push(range);
        }
        self.block(block);
        if has_bounded_range {
            self.bounded_integer_ranges.pop();
        }
        self.in_loop = outer_loop;
        self.continue_label = outer_continue;
        self.indent -= 1;
        self.line("}");
    }

    fn for_statement(&mut self, node: &SyntaxNode) {
        match node.children.as_slice() {
            [target, collection, block] if target.kind == SyntaxKind::ForTarget => {
                let collection_type = self.value_type(collection);
                let collection = self.expression(collection);
                let loop_index = self.loop_counter;
                let iterator = format!("__terrane_iterator_{loop_index}");
                self.loop_counter += 1;
                let constructor = match collection_type {
                    Some(ValueType::Scalar(ScalarType::Bytes)) => {
                        format!("terrane_collection_support::bytes_iterator(&({collection}))")
                    }
                    Some(ValueType::Iterator(_)) => format!("&mut ({collection})"),
                    Some(
                        ValueType::List(_)
                        | ValueType::Map(_, _)
                        | ValueType::Set(_)
                        | ValueType::Tuple(_, _)
                        | ValueType::Range
                        | ValueType::UnorderedMap(_, _)
                        | ValueType::UnorderedSet(_),
                    ) => format!(
                        "terrane_collection_support::Iterable::terrane_iterator(&({collection}))"
                    ),
                    _ => format!("terrane_collection_support::string_iterator(&({collection}))"),
                };
                self.line(&format!("let mut {iterator} = {constructor};"));
                self.line("loop {");
                self.indent += 1;
                self.iteration_target_bindings(target, &iterator, loop_index);
                let outer_continue = self.continue_label.take();
                let outer_loop = std::mem::replace(&mut self.in_loop, true);
                self.block(block);
                self.in_loop = outer_loop;
                self.continue_label = outer_continue;
                self.indent -= 1;
                self.line("}");
            }
            [initial, condition, update, block] => {
                self.statement(initial);
                let condition = self.control_condition(condition);
                self.line(&format!("while {condition} {{"));
                self.indent += 1;
                let label = format!("__terrane_continue_{}", self.loop_counter);
                self.loop_counter += 1;
                self.line(&format!("'{label}: {{"));
                self.indent += 1;
                let outer_continue = self.continue_label.replace(label);
                let outer_loop = std::mem::replace(&mut self.in_loop, true);
                self.block(block);
                self.in_loop = outer_loop;
                self.continue_label = outer_continue;
                self.indent -= 1;
                self.line("}");
                self.statement(update);
                self.indent -= 1;
                self.line("}");
            }
            _ => {}
        }
    }

    fn iteration_target_bindings(
        &mut self,
        target: &SyntaxNode,
        iterator: &str,
        loop_index: usize,
    ) {
        match target.children.as_slice() {
            [name] => {
                let name_span = name.span;
                let mutable = if binding_span_is_mutated(self.package, self.unit, name.span, true) {
                    "mut "
                } else {
                    ""
                };
                let name = rust_name(self.text(name));
                self.line(&format!("let {mutable}{name} = match {iterator}.next() {{"));
                self.indent += 1;
                self.line("terrane_collection_support::IterationStep::Item(item) => item,");
                self.line("terrane_collection_support::IterationStep::End => break,");
                self.indent -= 1;
                self.line("};");
                if !binding_store_value_is_read(self.package, name_span, name_span) {
                    self.line(&format!("let _ = &{name};"));
                }
            }
            [key, value] => {
                let item = format!("__terrane_item_{loop_index}");
                self.line(&format!("let {item} = match {iterator}.next() {{"));
                self.indent += 1;
                self.line("terrane_collection_support::IterationStep::Item(item) => item,");
                self.line("terrane_collection_support::IterationStep::End => break,");
                self.indent -= 1;
                self.line("};");
                for (target, field) in [(key, "key"), (value, "value")] {
                    let target_span = target.span;
                    let mutable =
                        if binding_span_is_mutated(self.package, self.unit, target_span, true) {
                            "mut "
                        } else {
                            ""
                        };
                    let name = rust_name(self.text(target));
                    self.line(&format!("let {mutable}{name} = {item}.{field};"));
                    if !binding_store_value_is_read(self.package, target_span, target_span) {
                        self.line(&format!("let _ = &{name};"));
                    }
                }
            }
            _ => unreachable!("semantic analysis admitted invalid iteration target arity"),
        }
    }

    fn expression(&mut self, node: &SyntaxNode) -> String {
        match node.kind {
            SyntaxKind::Literal => literal(self.text(node)),
            SyntaxKind::AnonymousFunction => self.anonymous_function(node),
            SyntaxKind::Name => {
                let binding = self.unit.typed_bindings.iter().rev().find(|binding| {
                    binding.name == self.text(node)
                        && binding.is_visible_at(self.unit.source.id(), node.span.start)
                });
                match self.value_type(node) {
                    Some(ValueType::Descriptor(identity))
                        if binding.is_none_or(|binding| binding.scope.is_none()) =>
                    {
                        format!(
                            "TerraneDescriptor {{ identity: {identity:?}, name: {identity:?}, kind: \"type\" }}"
                        )
                    }
                    _ => self.name(node),
                }
            }
            SyntaxKind::GroupExpression => node
                .children
                .first()
                .map_or_else(String::new, |child| self.expression(child)),
            SyntaxKind::UnaryExpression => {
                let Some(operand) = node.children.last() else {
                    return String::new();
                };
                let source_operator = self.unary_operator(node).unwrap_or_default();
                if source_operator == "ref" {
                    return match self.value_type(operand) {
                        Some(ValueType::Reference(_)) => {
                            format!("({}).clone()", self.expression(operand))
                        }
                        _ => format!(
                            "std::sync::Arc::downgrade(&{})",
                            self.reference_storage_expression(operand)
                        ),
                    };
                }
                if source_operator == "shared ref" {
                    return match self.value_type(operand) {
                        Some(ValueType::SharedReference(_)) => {
                            format!("({}).clone()", self.expression(operand))
                        }
                        Some(ValueType::Reference(_)) => format!(
                            "{}.upgrade().expect(\"reference expired\")",
                            self.expression(operand)
                        ),
                        _ => self.reference_storage_expression(operand),
                    };
                }
                if source_operator == "await" {
                    return format!("__terrane_await({}).await", self.expression(operand));
                }
                if source_operator == "move" {
                    return match operand.kind {
                        SyntaxKind::Name => self.name(operand),
                        _ => self.expression(operand),
                    };
                }
                if self.is_adaptive_expression(operand) {
                    return self.adaptive_expression(node);
                }
                let operator = match source_operator.as_str() {
                    "not" => "!",
                    other => other,
                };
                let operand = if let Some(value_type) = self.receiver_value_type(operand) {
                    self.expression_as(operand, value_type)
                } else {
                    self.receiver_expression(operand)
                };
                format!("{operator}{operand}")
            }
            SyntaxKind::BinaryExpression => self.binary(node),
            SyntaxKind::TypeMembershipExpression => self.type_membership(node),
            SyntaxKind::MemberExpression => self.member(node),
            SyntaxKind::IndexExpression => self.index(node),
            SyntaxKind::CallExpression => self.call(node),
            SyntaxKind::PostfixExpression => node
                .children
                .first()
                .map_or_else(String::new, |child| self.expression(child)),
            _ => self.text(node).trim().to_owned(),
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "destination-directed lowering keeps every recursive value form in one auditable dispatch"
    )]
    fn expression_as(&mut self, node: &SyntaxNode, value_type: ValueType) -> String {
        if matches!(
            &value_type,
            ValueType::List(_)
                | ValueType::Map(_, _)
                | ValueType::Set(_)
                | ValueType::Tuple(_, _)
                | ValueType::Entry(_, _)
                | ValueType::UnorderedMap(_, _)
                | ValueType::UnorderedSet(_)
        ) && node.kind == SyntaxKind::GroupExpression
            && let [grouped] = node.children.as_slice()
        {
            return self.expression_as(grouped, value_type);
        }
        if let Some(actual) = self.value_type(node)
            && let ValueType::Reference(item) | ValueType::SharedReference(item) = actual
            && item.value_type() == value_type
        {
            return self.receiver_expression(node);
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
        {
            let constructor = match value_type.clone() {
                ValueType::List(item) if self.is_builtin(callee, "/core/collections::list") => {
                    Some(("List", item))
                }
                ValueType::Tuple(item, _)
                    if self.is_builtin(callee, "/core/collections::tuple") =>
                {
                    Some(("Tuple", item))
                }
                ValueType::Set(item) if self.is_builtin(callee, "/core/collections::set") => {
                    Some(("Set", item))
                }
                ValueType::UnorderedSet(item)
                    if self.is_builtin(callee, "/core/collections::unordered-set") =>
                {
                    Some(("UnorderedSet", item))
                }
                _ => None,
            };
            if let Some((kind, item)) = constructor {
                let values = arguments
                    .children
                    .iter()
                    .map(|argument| argument.children.last().unwrap_or(argument))
                    .map(|value| self.expression_as(value, item.value_type()))
                    .collect::<Vec<_>>();
                return format!(
                    "terrane_collection_support::{kind}::<{}>::new(vec![{}])",
                    rust_element_type(self.package, item),
                    values.join(", ")
                );
            }
            if let ValueType::Entry(key, value) = value_type.clone()
                && self.is_builtin(callee, "/core/collections::entry")
            {
                let [key_argument, value_argument] = arguments.children.as_slice() else {
                    unreachable!("semantic analysis validates entry constructor arity");
                };
                let key_node = key_argument.children.last().unwrap_or(key_argument);
                let value_node = value_argument.children.last().unwrap_or(value_argument);
                let key_expression = self.expression_as(key_node, key.value_type());
                let value_expression = self.expression_as(value_node, value.value_type());
                return format!(
                    "terrane_collection_support::Entry::<{}, {}>::new({key_expression}, {value_expression})",
                    rust_element_type(self.package, key),
                    rust_element_type(self.package, value),
                );
            }
            let map_constructor = match value_type.clone() {
                ValueType::Map(key, value) if self.is_builtin(callee, "/core/collections::map") => {
                    Some(("Map", key, value))
                }
                ValueType::UnorderedMap(key, value)
                    if self.is_builtin(callee, "/core/collections::unordered-map") =>
                {
                    Some(("UnorderedMap", key, value))
                }
                _ => None,
            };
            if let Some((kind, key, value)) = map_constructor {
                return self.map_constructor(arguments, kind, key, value);
            }
        }
        if let ValueType::Optional(inner) = value_type {
            let actual = self.value_type(node);
            return if actual == Some(ValueType::Optional(inner.clone())) {
                self.expression(node)
            } else if self.text(node).trim() == "none"
                || actual == Some(ValueType::Scalar(ScalarType::None))
            {
                "None".to_owned()
            } else {
                format!("Some({})", self.expression_as(node, *inner))
            };
        }
        if matches!(value_type, ValueType::Reference(_))
            && node.kind == SyntaxKind::UnaryExpression
            && self.unary_operator(node).as_deref() == Some("ref")
        {
            return self.expression(node);
        }
        if matches!(
            value_type,
            ValueType::Reference(_) | ValueType::SharedReference(_)
        ) && self.value_type(node) == Some(value_type.clone())
        {
            return format!("({}).clone()", self.expression(node));
        }
        if let ValueType::Object(expected) = &value_type
            && self.text(node) == "this"
            && let Some(actual) = &self.current_object
            && let Some(destination) = self
                .unit
                .objects
                .iter()
                .find(|object| object.identity == *expected)
        {
            let copy = if self.object_requires_separation(actual) {
                "self.terrane_separate()"
            } else {
                "self.clone()"
            };
            if actual == expected && !object_descendants(self.unit, destination).is_empty() {
                return format!(
                    "{}::Own({copy})",
                    rust_object_type_name(self.package, expected)
                );
            }
            if let Some(descendant) = object_descendants(self.unit, destination)
                .iter()
                .find(|descendant| descendant.identity == *actual)
            {
                return format!(
                    "{}::{}({copy})",
                    rust_object_type_name(self.package, expected),
                    rust_object_type_name(self.package, &descendant.identity)
                );
            }
        }
        if let ValueType::Object(expected) = &value_type
            && let Some(ValueType::Object(actual)) = self.value_type(node)
            && actual != *expected
            && let Some(destination) = self
                .unit
                .objects
                .iter()
                .find(|object| object.identity == *expected)
                .or_else(|| {
                    self.package
                        .resolve_name_at(self.unit, node.span.start, &expected.name)
                        .and_then(|symbol| {
                            self.package
                                .units
                                .iter()
                                .find(|unit| unit.namespace == symbol.namespace)
                                .and_then(|unit| {
                                    unit.objects
                                        .iter()
                                        .find(|object| object.name == symbol.name)
                                })
                        })
                })
                .or_else(|| {
                    self.package.units.iter().find_map(|unit| {
                        unit.objects.iter().find(|object| {
                            object.identity == *expected && object.kind == ObjectKind::Interface
                        })
                    })
                })
        {
            let expression = if self.text(node) == "this" {
                if self.object_requires_separation(&actual) {
                    "self.terrane_separate()".to_owned()
                } else {
                    "self.clone()".to_owned()
                }
            } else {
                self.expression_as(node, ValueType::Object(actual.clone()))
            };
            if destination.kind == ObjectKind::Interface {
                return format!(
                    "{}::from({expression})",
                    rust_object_type_name(self.package, expected)
                );
            }
            if object_descendants(self.unit, destination)
                .iter()
                .any(|descendant| descendant.identity == actual)
            {
                return format!(
                    "{}::{}({expression})",
                    rust_object_type_name(self.package, expected),
                    rust_object_type_name(self.package, &actual)
                );
            }
        }
        if let ValueType::Scalar(destination) = value_type
            && (node.kind != SyntaxKind::Literal
                || self.value_type(node) != Some(ValueType::Scalar(destination)))
            && let Some(Ok(constant)) = contextual_constant(self.source, node, destination)
        {
            return match constant {
                ContextualConstant::Integer(value) if destination == ScalarType::Int => {
                    adaptive_literal(&value.to_string())
                }
                ContextualConstant::Integer(value) => value.to_string(),
                ContextualConstant::Float32(value) => float32_literal(value),
                ContextualConstant::Float64(value) => float64_literal(value),
            };
        }
        if let ValueType::Scalar(destination) = value_type
            && let Some(ValueType::Scalar(source)) = self.value_type(node)
            && source != destination
            && is_numeric(source)
            && is_numeric(destination)
        {
            return self.numeric_destination(node, source, destination);
        }
        if let ValueType::Scalar(scalar) = value_type
            && scalar != ScalarType::Int
            && scalar.is_integer()
            && node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
        {
            let operator = self.unary_operator(node).unwrap_or_default();
            return format!("{operator}{}", self.expression(operand));
        }
        if node.kind == SyntaxKind::MemberExpression
            && matches!(
                &value_type,
                ValueType::Scalar(ScalarType::String | ScalarType::Bytes)
            )
        {
            return format!("({}).clone()", self.expression(node));
        }
        match value_type {
            ValueType::Scalar(ScalarType::Int) => self.adaptive_expression(node),
            ValueType::Scalar(ScalarType::Float32)
                if self.value_type(node) == Some(ValueType::Scalar(ScalarType::Float64)) =>
            {
                format!("({}) as f32", self.expression(node))
            }
            ValueType::Scalar(ScalarType::String)
                if node.kind == SyntaxKind::Name
                    && self.lazy_namespace_binding_type(node).is_some() =>
            {
                format!("(*{}).clone()", self.namespace_name(node))
            }
            ValueType::List(item)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::list") =>
            {
                format!(
                    "terrane_collection_support::List::<{}>::new(Vec::new())",
                    rust_element_type(self.package, item)
                )
            }
            ValueType::Tuple(item, _)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::tuple") =>
            {
                format!(
                    "terrane_collection_support::Tuple::<{}>::new(Vec::new())",
                    rust_element_type(self.package, item)
                )
            }
            ValueType::Set(item)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::set") =>
            {
                format!(
                    "terrane_collection_support::Set::<{}>::new(Vec::new())",
                    rust_element_type(self.package, item)
                )
            }
            ValueType::UnorderedSet(item)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::unordered-set") =>
            {
                format!(
                    "terrane_collection_support::UnorderedSet::<{}>::new(Vec::new())",
                    rust_element_type(self.package, item)
                )
            }
            ValueType::Map(key, value)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::map") =>
            {
                format!(
                    "terrane_collection_support::Map::<{}, {}>::new(Vec::new())",
                    rust_element_type(self.package, key),
                    rust_element_type(self.package, value)
                )
            }
            ValueType::UnorderedMap(key, value)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::unordered-map") =>
            {
                format!(
                    "terrane_collection_support::UnorderedMap::<{}, {}>::new(Vec::new())",
                    rust_element_type(self.package, key),
                    rust_element_type(self.package, value)
                )
            }
            ValueType::PlatformStreamHandle if node.kind == SyntaxKind::MemberExpression => {
                format!("({}).clone()", self.expression(node))
            }
            ValueType::Object(name)
                if node.kind == SyntaxKind::Name && self.object_owns_resource(&name) =>
            {
                self.expression(node)
            }
            ValueType::Object(name)
                if node.kind == SyntaxKind::Name && self.object_requires_separation(&name) =>
            {
                format!("({}).terrane_separate()", self.expression(node))
            }
            ValueType::List(_)
            | ValueType::Map(_, _)
            | ValueType::Set(_)
            | ValueType::Tuple(_, _)
            | ValueType::Range
            | ValueType::Entry(_, _)
            | ValueType::UnorderedMap(_, _)
            | ValueType::UnorderedSet(_)
            | ValueType::Object(_)
                if node.kind == SyntaxKind::Name && !self.is_only_binding_use(node) =>
            {
                format!("({}).clone()", self.expression(node))
            }
            ValueType::List(_)
            | ValueType::Map(_, _)
            | ValueType::Set(_)
            | ValueType::Tuple(_, _)
            | ValueType::Range
            | ValueType::Entry(_, _)
            | ValueType::UnorderedMap(_, _)
            | ValueType::UnorderedSet(_)
            | ValueType::Object(_)
                if node.kind == SyntaxKind::Name =>
            {
                self.expression(node)
            }
            ValueType::AsyncFunction(parameters, _)
                if node.kind == SyntaxKind::MemberExpression =>
            {
                let [receiver, member] = node.children.as_slice() else {
                    return String::new();
                };
                let receiver_type = self
                    .value_type(receiver)
                    .expect("bound object method receiver must have a static type");
                let receiver = self.expression_as(receiver, receiver_type);
                let declarations = parameters
                    .iter()
                    .enumerate()
                    .map(|(index, parameter)| {
                        format!(
                            "argument_{index}: {}",
                            rust_element_type(self.package, parameter.clone())
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let arguments = (0..parameters.len())
                    .map(|index| format!("argument_{index}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{{ let receiver = {receiver}; std::sync::Arc::new(move |{declarations}| -> std::pin::Pin<Box<dyn Future<Output = _>>> {{ let receiver = receiver.clone(); Box::pin(async move {{ receiver.{}({arguments}).await }}) }}) }}",
                    rust_name(self.text(member))
                )
            }
            ValueType::Function(parameters, _) if node.kind == SyntaxKind::MemberExpression => {
                let [receiver, member] = node.children.as_slice() else {
                    return String::new();
                };
                let callable_field = self.callable_object_field(receiver, self.text(member));
                let receiver_type = self
                    .value_type(receiver)
                    .expect("bound object method receiver must have a static type");
                let receiver = self.expression_as(receiver, receiver_type);
                let declarations = parameters
                    .iter()
                    .enumerate()
                    .map(|(index, parameter)| {
                        format!(
                            "argument_{index}: {}",
                            rust_element_type(self.package, parameter.clone())
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let arguments = (0..parameters.len())
                    .map(|index| format!("argument_{index}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                if callable_field {
                    format!(
                        "{{ let receiver = {receiver}; std::sync::Arc::new(move |{declarations}| (receiver.{})({arguments})) }}",
                        rust_name(self.text(member))
                    )
                } else {
                    format!(
                        "{{ let receiver = {receiver}; std::sync::Arc::new(move |{declarations}| receiver.{}({arguments})) }}",
                        rust_name(self.text(member))
                    )
                }
            }
            ValueType::Function(_, _) if node.kind == SyntaxKind::Name => {
                if let Some(contract) = self
                    .unit
                    .functions
                    .iter()
                    .find(|contract| contract.name == self.text(node))
                {
                    format!("std::sync::Arc::new({})", function_name(contract))
                } else {
                    format!("({}).clone()", self.expression(node))
                }
            }
            _ => self.expression(node),
        }
    }

    fn map_constructor(
        &mut self,
        arguments: &SyntaxNode,
        kind: &str,
        key: ElementType,
        value: ElementType,
    ) -> String {
        let entries = arguments
            .children
            .iter()
            .map(|argument| {
                let value_node = argument.children.last().unwrap_or(argument);
                if argument.children.len() < 2
                    && matches!(self.value_type(value_node), Some(ValueType::Entry(_, _)))
                {
                    return self.expression_as(
                        value_node,
                        ValueType::Entry(key.clone(), value.clone()),
                    );
                }
                let name = argument
                    .children
                    .first()
                    .map(|name| self.text(name).to_owned())
                    .expect("validated named map entry");
                let value_expression = self.expression_as(value_node, value.value_type());
                format!(
                    "terrane_collection_support::Entry::new(String::from({name:?}), {value_expression})"
                )
            })
            .collect::<Vec<_>>();
        format!(
            "terrane_collection_support::{kind}::<{}, {}>::new(vec![{}])",
            rust_element_type(self.package, key),
            rust_element_type(self.package, value),
            entries.join(", ")
        )
    }

    fn adaptive_expression(&mut self, node: &SyntaxNode) -> String {
        match node.kind {
            SyntaxKind::Literal => adaptive_literal(self.text(node)),
            SyntaxKind::Name if self.lazy_namespace_binding_type(node).is_some() => {
                format!("(*{}).clone()", self.namespace_name(node))
            }
            SyntaxKind::Name if self.small_int_binding(node).is_some() => {
                format!(
                    "terrane_int_support::Int::from(({}) as i128)",
                    self.name(node)
                )
            }
            SyntaxKind::Name => format!("{}.clone()", self.name(node)),
            SyntaxKind::GroupExpression => {
                node.children.first().map_or_else(String::new, |child| {
                    format!("({})", self.adaptive_expression(child))
                })
            }
            SyntaxKind::UnaryExpression => {
                let Some(operand) = node.children.last() else {
                    return String::new();
                };
                let operator = self.unary_operator(node).unwrap_or_default();
                if operator == "await" {
                    format!("__terrane_await({}).await", self.expression(operand))
                } else {
                    format!("{operator}{}", self.adaptive_expression(operand))
                }
            }
            SyntaxKind::BinaryExpression => self.adaptive_binary(node),
            SyntaxKind::MemberExpression
                if node
                    .children
                    .get(1)
                    .is_some_and(|member| self.text(member) == "length") =>
            {
                format!("terrane_int_support::Int::from({})", self.expression(node))
            }
            SyntaxKind::MemberExpression
                if node.children.first().is_some_and(|receiver| {
                    matches!(
                        self.receiver_value_type(receiver),
                        Some(ValueType::Object(_))
                    )
                }) =>
            {
                format!("({}).clone()", self.expression(node))
            }
            _ => self.expression(node),
        }
    }

    fn adaptive_binary(&mut self, node: &SyntaxNode) -> String {
        let [left, right] = node.children.as_slice() else {
            return String::new();
        };
        let operator = self.source.text()[left.span.end..right.span.start].trim();
        let left = self.adaptive_expression(left);
        let right = self.adaptive_expression(right);
        match operator {
            "/" => self.fallible(format!("({left}).euclidean_div(&({right}))"), node),
            "%" => self.fallible(format!("({left}).modulo(&({right}))"), node),
            _ => format!("({left} {operator} {right})"),
        }
    }

    fn adaptive_binary_as(&mut self, node: &SyntaxNode) -> String {
        let [left, right] = node.children.as_slice() else {
            return String::new();
        };
        let operator = self.source.text()[left.span.end..right.span.start].trim();
        let left = self.expression_as(left, ValueType::Scalar(ScalarType::Int));
        let right = self.expression_as(right, ValueType::Scalar(ScalarType::Int));
        match operator {
            "/" => self.fallible(format!("({left}).euclidean_div(&({right}))"), node),
            "%" => self.fallible(format!("({left}).modulo(&({right}))"), node),
            _ => format!("({left} {operator} {right})"),
        }
    }

    fn is_adaptive_expression(&self, node: &SyntaxNode) -> bool {
        self.value_type(node) == Some(ValueType::Scalar(ScalarType::Int))
    }

    fn numeric_operation_type(&self, left: &SyntaxNode, right: &SyntaxNode) -> Option<ScalarType> {
        let scalar = |value_type| match value_type {
            ValueType::Scalar(scalar) => Some(scalar),
            _ => None,
        };
        let left_type = self.value_type(left).and_then(scalar);
        let right_type = self.value_type(right).and_then(scalar);
        if let Some(left_type) = left_type
            && is_numeric(left_type)
            && matches!(
                contextual_constant(self.source, right, left_type),
                Some(Ok(_))
            )
        {
            return Some(left_type);
        }
        if let Some(right_type) = right_type
            && is_numeric(right_type)
            && matches!(
                contextual_constant(self.source, left, right_type),
                Some(Ok(_))
            )
        {
            return Some(right_type);
        }
        match (left_type, right_type) {
            (Some(left), Some(right)) if left == right && is_numeric(left) => Some(left),
            (Some(left), Some(right)) if left.is_integer() && right.is_integer() => {
                Some(promoted_integer_type(left, right))
            }
            _ => None,
        }
    }

    fn optional_none_comparison(
        &mut self,
        left: &SyntaxNode,
        operator: &str,
        right: &SyntaxNode,
    ) -> Option<String> {
        if !matches!(operator, "==" | "!=") {
            return None;
        }
        let left_type = self.value_type(left);
        let right_type = self.value_type(right);
        let is_optional = |value_type| matches!(value_type, Some(ValueType::Optional(_)));
        let left_is_none = self.text(left).trim() == "none"
            || left_type == Some(ValueType::Scalar(ScalarType::None));
        let right_is_none = self.text(right).trim() == "none"
            || right_type == Some(ValueType::Scalar(ScalarType::None));
        let presence_check = |value: String| {
            if operator == "==" {
                format!("({value}).is_none()")
            } else {
                format!("({value}).is_some()")
            }
        };
        if is_optional(left_type) && right_is_none {
            return Some(presence_check(self.expression(left)));
        }
        if left_is_none && is_optional(right_type) {
            return Some(presence_check(self.expression(right)));
        }
        None
    }

    fn binary(&mut self, node: &SyntaxNode) -> String {
        let [left, right] = node.children.as_slice() else {
            return String::new();
        };
        let source_operator = self.source.text()[left.span.end..right.span.start].trim();
        if source_operator == "is" {
            let result = matches!(
                (
                    self.descriptor_identity(left),
                    self.descriptor_identity(right),
                ),
                (Some(left), Some(right)) if left == right
            );
            let mut effects = Vec::new();
            if let Some(effect) = self.identity_operand_effect(left) {
                effects.push(effect);
            }
            if let Some(effect) = self.identity_operand_effect(right) {
                effects.push(effect);
            }
            return format!("{{ {} {result} }}", effects.join(" "));
        }
        if let Some(comparison) = self.optional_none_comparison(left, source_operator, right) {
            return comparison;
        }
        let comparison = matches!(source_operator, "==" | "!=" | "<" | "<=" | ">" | ">=");
        let left_is_small = self.small_int_binding(left).is_some()
            || matches!(
                contextual_constant(self.source, left, ScalarType::Int64),
                Some(Ok(_))
            );
        let right_is_small = self.small_int_binding(right).is_some()
            || matches!(
                contextual_constant(self.source, right, ScalarType::Int64),
                Some(Ok(_))
            );
        if comparison && left_is_small && right_is_small {
            let left = if self.small_int_binding(left).is_some() {
                self.expression(left)
            } else {
                self.expression_as(left, ValueType::Scalar(ScalarType::Int64))
            };
            let right = if self.small_int_binding(right).is_some() {
                self.expression(right)
            } else {
                self.expression_as(right, ValueType::Scalar(ScalarType::Int64))
            };
            return format!("({left} {source_operator} {right})");
        }
        if self.is_adaptive_expression(left)
            && matches!(source_operator, "==" | "!=" | "<" | "<=" | ">" | ">=")
        {
            return self.adaptive_binary(node);
        }
        if self.value_type(node) == Some(ValueType::Scalar(ScalarType::Int)) {
            return self.adaptive_binary_as(node);
        }
        if let Some(ValueType::Scalar(operation_type)) = self.value_type(node)
            && operation_type.is_integer()
            && operation_type != ScalarType::Int
            && let Some(operation) = match source_operator {
                "+" => Some("addition"),
                "-" => Some("subtraction"),
                "*" => Some("multiplication"),
                "/" => Some("division"),
                "%" => Some("remainder"),
                "<<" => Some("shift_left"),
                ">>" => Some("shift_right"),
                _ => None,
            }
        {
            let positive_literal_remainder = source_operator == "%"
                && matches!(
                    contextual_constant(self.source, right, operation_type),
                    Some(Ok(ContextualConstant::Integer(value))) if value > BigInt::from(0_u8)
                );
            let operation_type = ValueType::Scalar(operation_type);
            let left = Self::unwrapped_expression(self.expression_as(left, operation_type.clone()));
            let right = if matches!(source_operator, "<<" | ">>") {
                self.receiver_expression(right)
            } else {
                Self::unwrapped_expression(self.expression_as(right, operation_type))
            };
            let right = if matches!(source_operator, "<<" | ">>") {
                format!("&{right}")
            } else {
                right
            };
            if positive_literal_remainder {
                return format!("({left}).rem_euclid({right})");
            }
            let call = format!("terrane_int_support::fixed_{operation}({left}, {right})");
            return self.fallible(call, node);
        }
        if let Some(operation_type) = self.numeric_operation_type(left, right) {
            let left = self.expression_as(left, ValueType::Scalar(operation_type));
            let right = self.expression_as(right, ValueType::Scalar(operation_type));
            return format!("({left} {source_operator} {right})");
        }
        let operator = match source_operator {
            "and" => "&&",
            "or" => "||",
            other => other,
        };
        format!(
            "({} {operator} {})",
            self.receiver_expression(left),
            self.receiver_expression(right)
        )
    }

    fn type_membership(&mut self, node: &SyntaxNode) -> String {
        let [value, descriptor] = node.children.as_slice() else {
            return String::new();
        };
        let descriptor_type = self.descriptor_type(descriptor);
        let descriptor_category =
            crate::semantics::descriptor_expression_category(self.package, self.unit, descriptor);
        if let Some(binding) = self.union_binding(value)
            && let Some(category) = descriptor_category
        {
            let union_name = union_type_name(&binding);
            let expression = self.expression(value);
            let matching = binding
                .destination_arms
                .iter()
                .enumerate()
                .filter(|(_, arm)| arm.conforms_to(category))
                .map(|(index, _)| format!("{union_name}::Arm{index}(_)"))
                .collect::<Vec<_>>();
            return if matching.is_empty() {
                format!("{{ let _ = &{expression}; false }}")
            } else {
                format!("matches!(&{expression}, {})", matching.join(" | "))
            };
        }
        if let Some(binding) = self.union_binding(value)
            && let Some(descriptor) = descriptor_type
            && let Some(index) = binding
                .destination_arms
                .iter()
                .position(|arm| *arm == descriptor)
        {
            let union_name = union_type_name(&binding);
            let expression = self.expression(value);
            return format!("matches!(&{expression}, {union_name}::Arm{index}(_))");
        }
        let value_type = self.value_type(value);
        if let Some(category) = descriptor_category {
            return self.category_membership(value, value_type, category);
        }
        if let Some(destination) = descriptor_type
            && let Some(result) = contextual_constant(self.source, value, destination)
        {
            return result.is_ok().to_string();
        }
        let optional_inner = match value_type.clone() {
            Some(ValueType::Optional(inner)) => Some(*inner),
            _ => None,
        };
        if let Some(inner) = optional_inner {
            let value = self.expression(value);
            return match descriptor_type {
                Some(ScalarType::None) => format!("({value}).is_none()"),
                Some(descriptor) if inner == ValueType::Scalar(descriptor) => {
                    format!("({value}).is_some()")
                }
                _ => format!("{{ let _ = {value}; false }}"),
            };
        }
        let result = matches!(
            (value_type, descriptor_type),
            (Some(ValueType::Scalar(value)), Some(descriptor)) if value == descriptor
        );
        let effect = if value.kind == SyntaxKind::Name {
            let expression = Self::unwrapped_expression(self.expression(value));
            format!("let _ = &{expression};")
        } else {
            Self::discarded_expression(self.expression(value))
        };
        format!("{{ {effect} {result} }}")
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "category membership matches the optional recursive value type as one decision"
    )]
    fn category_membership(
        &mut self,
        node: &SyntaxNode,
        value_type: Option<ValueType>,
        category: TypeCategory,
    ) -> String {
        let optional_inner = match value_type.clone() {
            Some(ValueType::Optional(inner)) => Some(*inner),
            _ => None,
        };
        if let Some(inner) = optional_inner {
            let value = self.expression(node);
            let conforms = match inner {
                ValueType::Scalar(scalar) => scalar.conforms_to(category),
                ValueType::Object(_) => {
                    matches!(category, TypeCategory::Value | TypeCategory::Object)
                }
                _ => false,
            };
            return if conforms {
                format!("({value}).is_some()")
            } else {
                format!("{{ let _ = {value}; false }}")
            };
        }
        let result = matches!(
            value_type,
            Some(ValueType::Scalar(value)) if value.conforms_to(category)
        );
        let effect = if node.kind == SyntaxKind::Name {
            let expression = Self::unwrapped_expression(self.expression(node));
            format!("let _ = &{expression};")
        } else {
            Self::discarded_expression(self.expression(node))
        };
        format!("{{ {effect} {result} }}")
    }

    fn identity_operand_effect(&mut self, node: &SyntaxNode) -> Option<String> {
        let effect = if node.kind == SyntaxKind::MemberExpression
            && node
                .children
                .get(1)
                .is_some_and(|member| self.text(member) == "type")
        {
            node.children.first()?
        } else {
            node
        };
        matches!(self.value_type(effect), Some(ValueType::Scalar(_)))
            .then(|| Self::discarded_expression(self.expression(effect)))
    }

    fn unwrapped_expression(mut expression: String) -> String {
        loop {
            let bytes = expression.as_bytes();
            if bytes.len() <= 2 || bytes.first() != Some(&b'(') || bytes.last() != Some(&b')') {
                break;
            }
            let mut depth = 0_usize;
            let wraps_expression = bytes.iter().enumerate().all(|(index, byte)| {
                match byte {
                    b'(' => depth += 1,
                    b')' => depth -= 1,
                    _ => {}
                }
                depth != 0 || index == bytes.len() - 1
            });
            if !wraps_expression {
                break;
            }
            expression = expression[1..expression.len() - 1].to_owned();
        }
        expression
    }

    fn discarded_expression(expression: String) -> String {
        format!("let _ = {};", Self::unwrapped_expression(expression))
    }

    fn is_only_binding_use(&self, node: &SyntaxNode) -> bool {
        fn count_references(
            emitter: &Emitter<'_>,
            node: &SyntaxNode,
            binding_span: crate::Span,
            name: &str,
        ) -> usize {
            let here = usize::from(
                node.kind == SyntaxKind::Name
                    && emitter.text(node).trim() == name
                    && emitter
                        .unit
                        .typed_bindings
                        .iter()
                        .rev()
                        .find(|candidate| {
                            candidate.name == name
                                && candidate.is_visible_at(emitter.source.id(), node.span.start)
                        })
                        .is_some_and(|candidate| candidate.span == binding_span),
            );
            here + node
                .children
                .iter()
                .map(|child| count_references(emitter, child, binding_span, name))
                .sum::<usize>()
        }
        let name = self.text(node).trim();
        if name == "this" {
            return false;
        }
        let Some(binding) = self.unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == name && binding.is_visible_at(self.source.id(), node.span.start)
        }) else {
            return false;
        };

        count_references(self, &self.unit.tree.root, binding.span, name) == 1
    }

    fn value_type(&self, node: &SyntaxNode) -> Option<ValueType> {
        if let Some(value_type) = self.unit.inferred_value_type(node) {
            return Some(value_type);
        }
        match node.kind {
            SyntaxKind::Literal => match self.text(node).trim() {
                "true" | "false" => Some(ValueType::Scalar(ScalarType::Bool)),
                text if text.starts_with("b'") => Some(ValueType::Scalar(ScalarType::Bytes)),
                text if text.starts_with('\'') || text.starts_with('>') => {
                    Some(ValueType::Scalar(ScalarType::String))
                }
                text if text.contains('.') => Some(ValueType::Scalar(ScalarType::Float64)),
                text if text.chars().all(|character| {
                    character.is_ascii_hexdigit() || matches!(character, '_' | 'x' | 'o' | 'b')
                }) =>
                {
                    Some(ValueType::Scalar(ScalarType::Int))
                }
                _ => None,
            },
            SyntaxKind::Name => {
                let name = self.text(node).trim();
                self.unit
                    .typed_bindings
                    .iter()
                    .rev()
                    .find(|binding| {
                        binding.name == name
                            && binding.is_visible_at(self.source.id(), node.span.start)
                    })
                    .map(|binding| binding.value_type.clone())
                    .or_else(|| {
                        self.parameter_types
                            .iter()
                            .find(|(parameter, _)| parameter == name)
                            .map(|(_, value_type)| value_type.clone())
                    })
            }
            SyntaxKind::TypeExpression
            | SyntaxKind::GroupExpression
            | SyntaxKind::UnaryExpression => node
                .children
                .last()
                .and_then(|child| self.value_type(child)),
            _ => None,
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "string-view dispatch matches the optional recursive receiver type as one decision"
    )]
    fn direct_string_view_length(
        &mut self,
        receiver: &SyntaxNode,
        receiver_type: Option<ValueType>,
        member: &SyntaxNode,
    ) -> Option<String> {
        if self.text(member) != "length" {
            return None;
        }
        let Some(ValueType::StringView(unit)) = receiver_type else {
            return None;
        };
        receiver.children.first().map(|source| {
            let source = self.expression(source);
            match unit {
                crate::semantics::TextUnit::Bytes => format!("({source}).len() as i128"),
                crate::semantics::TextUnit::Scalars => {
                    format!("({source}).chars().count() as i128")
                }
                crate::semantics::TextUnit::Graphemes => {
                    format!("terrane_string_support::length(&({source})) as i128")
                }
            }
        })
    }

    fn index(&mut self, node: &SyntaxNode) -> String {
        let [receiver, index] = node.children.as_slice() else {
            return String::new();
        };
        let receiver_type = self.receiver_value_type(receiver);
        let receiver = self.receiver_expression(receiver);
        match receiver_type {
            Some(ValueType::List(_) | ValueType::Tuple(_, _) | ValueType::StringList) => {
                let index = if self.value_type(index) == Some(ValueType::Scalar(ScalarType::Int)) {
                    let index_value = self.expression_as(index, ValueType::Scalar(ScalarType::Int));
                    self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index_value}))"),
                        node,
                    )
                } else {
                    format!("({}) as usize", self.expression(index))
                };
                if receiver_type == Some(ValueType::StringList) {
                    self.fallible(
                        format!(
                            "({receiver}).get({index}).cloned().ok_or(terrane_collection_support::IndexError {{ index: {index} }})"
                        ),
                        node,
                    )
                } else {
                    self.fallible(format!("({receiver}).get_or_error({index})"), node)
                }
            }
            Some(ValueType::Map(key, _) | ValueType::UnorderedMap(key, _)) => {
                let index_value = self.expression_as(index, key.value_type());
                self.fallible(format!("({receiver}).get_or_error(&({index_value}))"), node)
            }
            _ => String::new(),
        }
    }

    fn receiver_value_type(&self, receiver: &SyntaxNode) -> Option<ValueType> {
        self.value_type(receiver)
            .map(|value_type| match value_type {
                ValueType::Reference(item) | ValueType::SharedReference(item) => item.value_type(),
                value_type => value_type,
            })
    }

    fn receiver_expression(&mut self, receiver: &SyntaxNode) -> String {
        match self.value_type(receiver) {
            Some(ValueType::SharedReference(_)) => format!(
                "({{ let __terrane_value = {}.lock().expect(\"shared reference lock poisoned\").clone(); __terrane_value }})",
                self.expression(receiver)
            ),
            Some(ValueType::Reference(_)) => format!(
                "({{ let __terrane_owner = {}.upgrade().expect(\"reference expired\"); let __terrane_value = __terrane_owner.lock().expect(\"reference lock poisoned\").clone(); __terrane_value }})",
                self.expression(receiver)
            ),
            _ => self.expression(receiver),
        }
    }

    fn receiver_guard_expression(&mut self, receiver: &SyntaxNode) -> String {
        match self.value_type(receiver) {
            Some(ValueType::SharedReference(_)) => format!(
                "{}.lock().expect(\"shared reference lock poisoned\")",
                self.expression(receiver)
            ),
            Some(ValueType::Reference(_)) => {
                "__terrane_owner.lock().expect(\"reference lock poisoned\")".to_owned()
            }
            _ if self.reference_backed_name(receiver).is_some() => {
                format!(
                    "{}.lock().expect(\"reference lock poisoned\")",
                    self.name(receiver)
                )
            }
            _ => self.expression(receiver),
        }
    }

    fn wrap_receiver_guard(&mut self, receiver: &SyntaxNode, expression: String) -> String {
        if matches!(self.value_type(receiver), Some(ValueType::Reference(_))) {
            format!(
                "({{ let __terrane_owner = {}.upgrade().expect(\"reference expired\"); {expression} }})",
                self.expression(receiver)
            )
        } else {
            expression
        }
    }

    fn borrowed_expression(&mut self, node: &SyntaxNode) -> String {
        if node.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = node.children.as_slice()
            && matches!(self.value_type(receiver), Some(ValueType::Entry(_, _)))
            && matches!(self.text(member), "key" | "value")
        {
            return format!("({}).{}", self.expression(receiver), self.text(member));
        }
        self.expression(node)
    }

    fn display_expression(&mut self, node: &SyntaxNode) -> String {
        if matches!(self.value_type(node), Some(ValueType::Descriptor(_))) {
            format!("({}).name", self.borrowed_expression(node))
        } else if matches!(
            self.value_type(node),
            Some(ValueType::Reference(_) | ValueType::SharedReference(_))
        ) {
            self.receiver_expression(node)
        } else {
            self.borrowed_expression(node)
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "member lowering keeps one ordered dispatch across scalar and collection surfaces"
    )]
    fn member(&mut self, node: &SyntaxNode) -> String {
        let [receiver, member] = node.children.as_slice() else {
            return String::new();
        };
        let receiver_type = self.receiver_value_type(receiver);
        if let Some(ValueType::Descriptor(_)) = &receiver_type {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "name" => format!("({receiver}).name.to_owned()"),
                "kind" => format!("({receiver}).kind.to_owned()"),
                "identity" => format!("({receiver}).identity.to_owned()"),
                _ => String::new(),
            };
        }
        if matches!(
            receiver_type,
            Some(ValueType::Function(_, _) | ValueType::AsyncFunction(_, _))
        ) && matches!(
            self.text(member),
            "contracts" | "throwable-contract" | "escaping-throwables"
        ) {
            let reflected = self
                .unit
                .functions
                .iter()
                .find(|contract| contract.name == self.text(receiver))
                .map(|contract| match self.text(member) {
                    "escaping-throwables" => contract
                        .escaping_throwables
                        .iter()
                        .map(|identity| {
                            identity
                                .rsplit_once("::")
                                .map_or(identity.as_str(), |(_, name)| name)
                        })
                        .collect::<Vec<_>>()
                        .join("|"),
                    "throwable-contract" => contract
                        .thrown_types
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join("|"),
                    _ if contract.throws => "throws".to_owned(),
                    _ => String::new(),
                })
                .unwrap_or_default();
            return format!(
                "{{ let _ = {}; {:?}.to_owned() }}",
                self.expression(receiver),
                reflected
            );
        }
        if let Some(ValueType::TaskOutcome(_)) = &receiver_type {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "completed" => format!("({receiver}).completed"),
                "cancelled" => format!("({receiver}).cancelled"),
                "value" => format!("({receiver}).value.clone()"),
                "error" => format!("({receiver}).error"),
                _ => String::new(),
            };
        }
        if matches!(
            receiver_type,
            Some(
                ValueType::PlatformOpenResult
                    | ValueType::PlatformReadResult
                    | ValueType::PlatformWriteResult
                    | ValueType::PlatformUnitResult
            )
        ) {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "handle" | "data" | "completed" | "message" => {
                    format!("({receiver}).{}.clone()", self.text(member))
                }
                name => format!("({receiver}).{name}"),
            };
        }
        if let Some(length) =
            self.direct_string_view_length(receiver, receiver_type.clone(), member)
        {
            return length;
        }
        let wrapped_field = self.wrapped_object_field(receiver, self.text(member));
        if matches!(
            self.value_type(receiver),
            Some(ValueType::Reference(_) | ValueType::SharedReference(_))
        ) && matches!(receiver_type, Some(ValueType::Object(_)))
        {
            let guard = self.receiver_guard_expression(receiver);
            let field = rust_name(self.text(member));
            let access = if wrapped_field {
                format!("({guard}).terrane_field_{field}().clone()")
            } else {
                format!("({guard}).{field}.clone()")
            };
            return self.wrap_receiver_guard(receiver, access);
        }
        let receiver = self.receiver_expression(receiver);
        if self.text(member) == "length"
            && matches!(
                self.value_type(node),
                Some(ValueType::Function(_, _) | ValueType::AsyncFunction(_, _))
            )
        {
            return format!("({receiver}).length");
        }
        match self.text(member) {
            "bytes" if receiver_type == Some(ValueType::Scalar(ScalarType::String)) => {
                format!("({receiver}).as_bytes().to_vec()")
            }
            "scalars" if receiver_type == Some(ValueType::Scalar(ScalarType::String)) => {
                format!("({receiver}).chars().map(|value| value.to_string()).collect::<Vec<_>>()")
            }
            "graphemes" if receiver_type == Some(ValueType::Scalar(ScalarType::String)) => {
                format!("terrane_string_support::graphemes(&({receiver})).collect::<Vec<_>>()")
            }
            "text" if receiver_type == Some(ValueType::TextRange) => {
                format!("({receiver}).text().to_owned()")
            }
            "bytes" | "scalars" | "graphemes" if receiver_type == Some(ValueType::TextRange) => {
                receiver
            }
            boundary @ ("start" | "end")
                if matches!(receiver_type, Some(ValueType::TextRangeView(_))) =>
            {
                let method = match (receiver_type.clone(), boundary) {
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Bytes)),
                        "start",
                    ) => "byte_start",
                    (Some(ValueType::TextRangeView(crate::semantics::TextUnit::Bytes)), "end") => {
                        "byte_end"
                    }
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Scalars)),
                        "start",
                    ) => "scalar_start",
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Scalars)),
                        "end",
                    ) => "scalar_end",
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Graphemes)),
                        "start",
                    ) => "grapheme_start",
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Graphemes)),
                        "end",
                    ) => "grapheme_end",
                    _ => unreachable!("semantic analysis validated text-range boundary"),
                };
                format!("({receiver}).{method}() as i128")
            }
            "length" => match receiver_type {
                Some(
                    ValueType::StringView(crate::semantics::TextUnit::Bytes)
                    | ValueType::Scalar(ScalarType::Bytes),
                ) => {
                    format!("({receiver}).len() as i128")
                }
                Some(
                    ValueType::StringView(
                        crate::semantics::TextUnit::Scalars | crate::semantics::TextUnit::Graphemes,
                    )
                    | ValueType::StringList
                    | ValueType::TextRangeList,
                ) => format!("({receiver}).len() as i128"),
                Some(
                    ValueType::List(_)
                    | ValueType::Map(_, _)
                    | ValueType::Set(_)
                    | ValueType::Tuple(_, _)
                    | ValueType::UnorderedMap(_, _)
                    | ValueType::UnorderedSet(_),
                ) => format!("terrane_int_support::Int::from(({receiver}).length())"),
                _ => format!("terrane_string_support::length(&{receiver}) as i128"),
            },
            "key" if matches!(receiver_type, Some(ValueType::Entry(_, _))) => {
                format!("({receiver}).key.clone()")
            }
            "value" if matches!(receiver_type, Some(ValueType::Entry(_, _))) => {
                format!("({receiver}).value.clone()")
            }
            "type" => "()".to_owned(),
            operation @ ("square-root" | "sine" | "cosine" | "sine-cosine" | "natural-log"
            | "exponential")
                if matches!(
                    receiver_type.clone(),
                    Some(ValueType::Scalar(ScalarType::Float32 | ScalarType::Float64))
                ) =>
            {
                if operation == "sine-cosine" {
                    return format!(
                        "{{ let terrane_sine_cosine = ({receiver}).sin_cos(); \
                         terrane_collection_support::Tuple::new(vec![\
                         terrane_sine_cosine.0, terrane_sine_cosine.1]) }}"
                    );
                }
                let method = match operation {
                    "square-root" => "sqrt",
                    "sine" => "sin",
                    "cosine" => "cos",
                    "natural-log" => "ln",
                    "exponential" => "exp",
                    _ => unreachable!(),
                };
                format!("({receiver}).{method}()")
            }
            mode @ ("round" | "floor" | "ceiling" | "truncate")
                if matches!(
                    receiver_type.clone(),
                    Some(ValueType::Scalar(ScalarType::Float32 | ScalarType::Float64))
                ) =>
            {
                let helper = if receiver_type == Some(ValueType::Scalar(ScalarType::Float32)) {
                    "rounded_f32"
                } else {
                    "rounded_f64"
                };
                let mode = match mode {
                    "round" => "TiesEven",
                    "floor" => "Floor",
                    "ceiling" => "Ceiling",
                    "truncate" => "Truncate",
                    _ => unreachable!(),
                };
                self.fallible(
                    format!(
                        "terrane_int_support::{helper}({receiver}, terrane_int_support::FloatRounding::{mode})"
                    ),
                    node,
                )
            }
            name if wrapped_field => {
                format!("({receiver}).terrane_field_{}().clone()", rust_name(name))
            }
            name => format!("{receiver}.{}", rust_name(name)),
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "all call forms share one ordering and error-propagation path"
    )]
    fn call(&mut self, node: &SyntaxNode) -> String {
        let [callee, arguments] = node.children.as_slice() else {
            return String::new();
        };
        if callee.kind == SyntaxKind::Name && self.text(callee) == "task-scope" {
            let deadline = arguments.children.first().map_or_else(
                || "None".to_owned(),
                |argument| {
                    let value = argument.children.last().unwrap_or(argument);
                    format!("Some(({}) as u64)", self.expression(value))
                },
            );
            return format!("TerraneTaskScope::new({deadline})");
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && self.receiver_value_type(receiver) == Some(ValueType::TaskScope)
        {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "spawn" => arguments.children.first().map_or_else(String::new, |argument| {
                    let callable = argument.children.last().unwrap_or(argument);
                    let throws = self
                        .contract_for_call(callable)
                        .is_some_and(|contract| contract.throws);
                    let callable = if let Some(value_type) = self.value_type(callable) {
                        self.expression_as(callable, value_type)
                    } else {
                        self.expression(callable)
                    };
                    if throws {
                        format!(
                            "{{ let __terrane_scope = ({receiver}).clone(); let __terrane_cancel = __terrane_scope.clone(); TerraneScopedTask::spawn(move || match __terrane_block_on_cancellable(({callable})(), move || __terrane_cancel.should_cancel()) {{ Some(Ok(value)) => TerraneTaskResult::Completed(value), Some(Err(error)) => TerraneTaskResult::Failed(error), None => TerraneTaskResult::Cancelled }}) }}"
                        )
                    } else {
                        format!(
                            "{{ let __terrane_scope = ({receiver}).clone(); let __terrane_cancel = __terrane_scope.clone(); TerraneScopedTask::spawn(move || match __terrane_block_on_cancellable(({callable})(), move || __terrane_cancel.should_cancel()) {{ Some(value) => TerraneTaskResult::Completed(value), None => TerraneTaskResult::Cancelled }}) }}"
                        )
                    }
                }),
                "join" => arguments.children.first().map_or_else(String::new, |argument| {
                    let task = argument.children.last().unwrap_or(argument);
                    format!("({receiver}).join({})", self.expression(task))
                }),
                "cancel" => format!("({receiver}).cancel()"),
                "child-scope" => arguments.children.first().map_or_else(String::new, |argument| {
                    let deadline = argument.children.last().unwrap_or(argument);
                    format!("({receiver}).child_scope(({}) as u64)", self.expression(deadline))
                }),
                _ => String::new(),
            };
        }
        if callee.kind == SyntaxKind::Name
            && let Some(object) =
                self.unit.objects.iter().find(|object| {
                    object.kind == ObjectKind::Class && object.name == self.text(callee)
                })
        {
            let construct = self.contract_for_call(callee).cloned();
            let values = arguments
                .children
                .iter()
                .enumerate()
                .map(|(index, argument)| {
                    let value = argument.children.last().unwrap_or(argument);
                    let destination = construct
                        .as_ref()
                        .and_then(|contract| contract.parameters.get(index))
                        .and_then(|parameter| parameter.value_type.clone());
                    if let Some(ty) = destination {
                        self.expression_as(value, ty)
                    } else {
                        self.expression(value)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!(
                "{}::terrane_construct({values})",
                rust_object_type_name(self.package, &object.identity)
            );
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [family, child] = callee.children.as_slice()
            && self.text(child) == "checked"
            && family.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = family.children.as_slice()
            && self.text(member) == "get"
            && let Some(receiver_type) = self.receiver_value_type(receiver)
            && let Some(argument) = arguments.children.first()
        {
            let argument = argument.children.last().unwrap_or(argument);
            let receiver_value = self.receiver_expression(receiver);
            return match receiver_type {
                ValueType::List(_) | ValueType::Tuple(_, _) => {
                    let index = self.expression_as(argument, ValueType::Scalar(ScalarType::Int));
                    format!(
                        "terrane_collection_support::index_from_int(&({index})).ok().and_then(|index| ({receiver_value}).get(index).cloned())"
                    )
                }
                ValueType::Map(key, _) | ValueType::UnorderedMap(key, _) => {
                    let key = self.expression_as(argument, key.value_type());
                    format!("({receiver_value}).get(&({key})).cloned()")
                }
                _ => String::new(),
            };
        }
        if let Some(string_call) = self.string_call(node, arguments) {
            return string_call;
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && let Some(receiver_type) = self.receiver_value_type(receiver)
        {
            let receiver_value = self.receiver_guard_expression(receiver);
            let member_name = self.text(member).to_owned();
            let values = arguments
                .children
                .iter()
                .map(|argument| argument.children.last().unwrap_or(argument))
                .collect::<Vec<_>>();
            let call = match (receiver_type, member_name.as_str()) {
                (ValueType::List(item), "append") => Some(format!(
                    "({{ let collection = &mut ({receiver_value}); collection.append({}); collection.clone() }})",
                    self.expression_as(values[0], item.value_type())
                )),
                (ValueType::List(item), "set") => {
                    let index = self.expression_as(values[0], ValueType::Scalar(ScalarType::Int));
                    let index = self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index}))"),
                        node,
                    );
                    let value = self.expression_as(values[1], item.value_type());
                    let mutation = self.fallible(format!("collection.set({index}, {value})"), node);
                    Some(format!(
                        "({{ let collection = &mut ({receiver_value}); {mutation}; collection.clone() }})"
                    ))
                }
                (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "set") => {
                    Some(format!(
                        "({{ let collection = &mut ({receiver_value}); collection.set({}, {}); collection.clone() }})",
                        self.expression_as(values[0], key.value_type()),
                        self.expression_as(values[1], value.value_type())
                    ))
                }
                (ValueType::Set(item) | ValueType::UnorderedSet(item), "contains") => {
                    Some(format!(
                        "({receiver_value}).contains(&({}))",
                        self.expression_as(values[0], item.value_type())
                    ))
                }
                (ValueType::Set(item) | ValueType::UnorderedSet(item), "add") => Some(format!(
                    "({{ let collection = &mut ({receiver_value}); collection.add({}); collection.clone() }})",
                    self.expression_as(values[0], item.value_type())
                )),
                (ValueType::Set(item) | ValueType::UnorderedSet(item), "remove") => Some(format!(
                    "({receiver_value}).remove(&({}))",
                    self.expression_as(values[0], item.value_type())
                )),
                (
                    ValueType::Map(_, _) | ValueType::UnorderedMap(_, _),
                    "keys" | "values" | "entries",
                ) => Some(format!("({receiver_value}).{member_name}()")),
                _ => None,
            };
            if let Some(call) = call {
                return self.wrap_receiver_guard(receiver, call);
            }
        }
        if let Some(method) = bound_method(self.source, callee)
            && !matches!(
                find_node_by_span(&self.unit.tree.root, method.receiver)
                    .and_then(|receiver| self.value_type(receiver)),
                Some(ValueType::Object(_))
            )
        {
            let receiver_node = find_node_by_span(&self.unit.tree.root, method.receiver)
                .expect("validated bound method receiver");
            let receiver = self.expression(receiver_node);
            if method.family == MemberFamily::Coerce {
                return self.integer_coercion(&method, receiver_node, callee, arguments);
            }
            if let MemberFamily::Arithmetic(family) = method.family {
                return self.arithmetic_family(
                    family,
                    method.child,
                    receiver_node,
                    arguments,
                    node,
                );
            }
            let argument = arguments
                .children
                .first()
                .and_then(|argument| argument.children.last())
                .map(|value| self.expression(value))
                .unwrap_or_default();
            let callback_throws = method.family == MemberFamily::Parse
                && arguments
                    .children
                    .first()
                    .and_then(|argument| argument.children.last())
                    .and_then(|callback| self.contract_for_call(callback))
                    .is_some_and(|contract| contract.throws);
            let call = match method.family {
                MemberFamily::Parse => format!("{argument}({receiver})"),
                MemberFamily::Radix
                    if self.value_type(receiver_node)
                        == Some(ValueType::Scalar(ScalarType::String)) =>
                {
                    format!("terrane_int_support::parse_radix(&({receiver}), &({argument}))")
                }
                MemberFamily::Radix => {
                    format!("terrane_int_support::format_radix(&({receiver}), &({argument}))")
                }
                MemberFamily::Coerce | MemberFamily::Arithmetic(_) => unreachable!(),
            };
            return if method.family == MemberFamily::Parse && !callback_throws {
                if method.child == "checked" {
                    format!("Some({call})")
                } else {
                    call
                }
            } else if method.child == "checked" {
                format!("({call}).ok()")
            } else {
                self.fallible(call, node)
            };
        }
        if self.is_builtin(callee, "/core/collections::iterator") {
            let item_type = self
                .value_type(node)
                .and_then(|ty| match ty {
                    ValueType::Iterator(item) => Some(item),
                    _ => None,
                })
                .expect("validated iterator constructor has an item type");
            let values = arguments
                .children
                .iter()
                .map(|argument| argument.children.last().unwrap_or(argument))
                .map(|value| self.expression_as(value, item_type.value_type()))
                .collect::<Vec<_>>();
            return format!(
                "terrane_collection_support::Iterator::<{}>::new(vec![{}])",
                rust_element_type(self.package, item_type),
                values.join(", ")
            );
        }
        if let Some(value_type) = self.value_type(node) {
            let constructor = match value_type.clone() {
                ValueType::List(item) if self.is_builtin(callee, "/core/collections::list") => {
                    Some(("List", item))
                }
                ValueType::Tuple(item, _)
                    if self.is_builtin(callee, "/core/collections::tuple") =>
                {
                    Some(("Tuple", item))
                }
                ValueType::Set(item) if self.is_builtin(callee, "/core/collections::set") => {
                    Some(("Set", item))
                }
                ValueType::UnorderedSet(item)
                    if self.is_builtin(callee, "/core/collections::unordered-set") =>
                {
                    Some(("UnorderedSet", item))
                }
                _ => None,
            };
            if let Some((kind, item)) = constructor {
                let values = arguments
                    .children
                    .iter()
                    .map(|argument| argument.children.last().unwrap_or(argument))
                    .map(|value| self.expression_as(value, item.value_type()))
                    .collect::<Vec<_>>();
                return format!(
                    "terrane_collection_support::{kind}::<{}>::new(vec![{}])",
                    rust_element_type(self.package, item),
                    values.join(", ")
                );
            }
            if let ValueType::Entry(key, value) = value_type.clone()
                && self.is_builtin(callee, "/core/collections::entry")
            {
                let [key_argument, value_argument] = arguments.children.as_slice() else {
                    unreachable!("semantic analysis validates entry constructor arity");
                };
                let key_node = key_argument.children.last().unwrap_or(key_argument);
                let value_node = value_argument.children.last().unwrap_or(value_argument);
                let values = [
                    self.expression_as(key_node, key.value_type()),
                    self.expression_as(value_node, value.value_type()),
                ];
                return format!(
                    "terrane_collection_support::Entry::<{}, {}>::new({}, {})",
                    rust_element_type(self.package, key),
                    rust_element_type(self.package, value),
                    values[0],
                    values[1]
                );
            }
            let map_constructor = match value_type.clone() {
                ValueType::Map(key, value) if self.is_builtin(callee, "/core/collections::map") => {
                    Some(("Map", key, value))
                }
                ValueType::UnorderedMap(key, value)
                    if self.is_builtin(callee, "/core/collections::unordered-map") =>
                {
                    Some(("UnorderedMap", key, value))
                }
                _ => None,
            };
            if let Some((kind, key, value)) = map_constructor {
                return self.map_constructor(arguments, kind, key, value);
            }
            if value_type == ValueType::Range {
                let through = callee.kind == SyntaxKind::MemberExpression
                    && callee.children.first().is_some_and(|receiver| {
                        self.is_builtin(receiver, "/core/collections::range")
                    });
                if through || self.is_builtin(callee, "/core/collections::range") {
                    let mut values = arguments
                        .children
                        .iter()
                        .map(|argument| argument.children.last().unwrap_or(argument))
                        .map(|value| self.expression_as(value, ValueType::Scalar(ScalarType::Int)))
                        .collect::<Vec<_>>();
                    if values.len() == 2 {
                        values.push("terrane_int_support::Int::from(1_i64)".to_owned());
                    }
                    let method = if through { "through" } else { "new" };
                    return self.fallible(
                        format!(
                            "terrane_collection_support::Range::{method}({}, {}, {})",
                            values[0], values[1], values[2]
                        ),
                        node,
                    );
                }
            }
        }
        let argument_values = arguments
            .children
            .iter()
            .map(|argument| argument.children.last().unwrap_or(argument))
            .collect::<Vec<_>>();
        if self.is_builtin(callee, "/core/output::print") {
            if argument_values.is_empty() {
                return "println!()".to_owned();
            }
            let values = argument_values
                .iter()
                .map(|value| self.display_expression(value))
                .map(|value| format!("terrane_scalar_support::scalar_text(&({value}))"))
                .collect::<Vec<_>>();
            let format = "{}".repeat(values.len());
            return format!("println!(\"{format}\", {})", values.join(", "));
        }
        let data_call = [
            ("empty-document", "empty_document"),
            ("make-document-none", "make_document_none"),
            ("make-document-bool", "make_document_bool"),
            ("make-document-string", "make_document_string"),
            ("make-document-integer", "make_document_integer"),
            ("make-document-decimal", "make_document_decimal"),
            ("make-document-list", "make_document_list"),
            ("make-document-map", "make_document_map"),
            ("document-list-append", "document_list_append"),
            ("document-map-insert", "document_map_insert"),
            ("json-parse", "json_parse"),
            ("json-canonical", "json_canonical"),
            ("yaml-parse", "yaml_parse"),
            ("data-failed", "data_failed"),
            ("data-message", "data_message"),
            ("data-path", "data_path"),
            ("data-expected", "data_expected"),
            ("data-encoded", "data_encoded"),
            ("document-kind", "document_kind"),
            ("document-text", "document_text"),
            ("document-coefficient", "document_coefficient"),
            ("document-exponent", "document_exponent"),
            ("document-length", "document_length"),
            ("document-item", "document_item"),
            ("document-key", "document_key"),
            ("document-field", "document_field"),
            ("validate-mapping", "validate_mapping"),
            ("url-parse", "url_parse"),
            ("url-failed", "url_failed"),
            ("url-message", "url_message"),
            ("url-serialized", "url_serialized"),
            ("url-display", "url_display"),
            ("url-scheme", "url_scheme"),
            ("url-username", "url_username"),
            ("url-password", "url_password"),
            ("url-host", "url_host"),
            ("url-port", "url_port"),
            ("url-path", "url_path"),
            ("url-query-length", "url_query_length"),
            ("url-query-key", "url_query_key"),
            ("url-query-value", "url_query_value"),
            ("url-fragment", "url_fragment"),
            ("url-origin", "url_origin"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("/core/platform-data::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = data_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let integer_argument = matches!(
                        (function, index),
                        ("json_parse", 1 | 2)
                            | ("yaml_parse", 1..=3)
                            | (
                                "document_item"
                                    | "document_key"
                                    | "url_query_key"
                                    | "url_query_value",
                                1,
                            )
                    );
                    let value = if integer_argument {
                        self.expression_as(value, ValueType::Scalar(ScalarType::Int))
                    } else {
                        self.expression(value)
                    };
                    let borrowed_result = (index == 0
                        && (function.starts_with("data_")
                            || function.starts_with("document_")
                            || function == "json_canonical"
                            || function == "validate_mapping"
                            || function.starts_with("url_") && function != "url_parse"))
                        || matches!(
                            (function, index),
                            ("document_list_append", 1) | ("document_map_insert", 2)
                        );
                    if borrowed_result {
                        format!("&({value})")
                    } else {
                        value
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_{function}({values})");
        }
        let capability_call = [
            ("secure-random", "platform_secure_random"),
            ("pseudo-random", "platform_pseudo_random"),
            ("secret-buffer", "platform_secret_buffer"),
            ("random-bytes", "platform_random_bytes"),
            ("random-bounded", "platform_random_bounded"),
            ("random-split", "platform_random_split"),
            ("digest", "platform_digest"),
            ("destroy-secret", "platform_destroy_secret"),
            ("cancellation-token", "platform_cancellation_token"),
            ("no-resource", "platform_no_resource"),
            ("failed-result", "platform_failed_result"),
            ("cancel", "platform_cancel"),
            ("hmac", "platform_hmac"),
            ("constant-time-equal", "platform_constant_time_equal"),
            ("hex-encode", "platform_hex_encode"),
            ("hex-decode", "platform_hex_decode"),
            ("base64-encode", "platform_base64_encode"),
            ("base64-decode", "platform_base64_decode"),
            ("uuid-parse", "platform_uuid_parse"),
            ("uuid-v4", "platform_uuid_v4"),
            ("uuid-v7", "platform_uuid_v7"),
            ("compress", "platform_compress"),
            ("decompress", "platform_decompress"),
            ("parse-ip", "platform_parse_ip"),
            ("parse-host-name", "platform_parse_host_name"),
            ("parse-socket", "platform_parse_socket"),
            ("parse-socket-text", "platform_parse_socket_text"),
            ("tcp-bind", "platform_tcp_bind"),
            ("tcp-connect", "platform_tcp_connect"),
            ("tcp-connect-host", "platform_tcp_connect_host"),
            ("tcp-accept", "platform_tcp_accept"),
            ("tcp-read", "platform_tcp_read"),
            ("tcp-write", "platform_tcp_write"),
            ("tcp-shutdown", "platform_tcp_shutdown"),
            ("tcp-configure", "platform_tcp_configure"),
            ("udp-bind", "platform_udp_bind"),
            ("udp-configure", "platform_udp_configure"),
            ("udp-send-to", "platform_udp_send_to"),
            ("udp-receive-from", "platform_udp_receive_from"),
            ("dns-lookup", "platform_dns_lookup"),
            ("tls-client", "platform_tls_client"),
            ("tls-read", "platform_tls_read"),
            ("tls-write", "platform_tls_write"),
            ("tls-shutdown", "platform_tls_shutdown"),
            ("close", "platform_close"),
            ("result-failed", "platform_result_failed"),
            ("result-resource-limit", "platform_result_resource_limit"),
            ("result-truncated", "platform_result_truncated"),
            (
                "result-deadline-exceeded",
                "platform_result_deadline_exceeded",
            ),
            ("result-message", "platform_result_message"),
            ("result-text", "platform_result_text"),
            ("result-detail", "platform_result_detail"),
            ("result-bytes", "platform_result_bytes"),
            ("result-int", "platform_result_int"),
            ("result-bool", "platform_result_bool"),
            ("result-entries", "platform_result_entries"),
            ("result-capability", "platform_result_capability"),
            ("result-resource", "platform_result_capability"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("/core/platform-capabilities::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = capability_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let value = self.expression(value);
                    let borrowed = matches!(
                        (function, index),
                        (
                            "platform_random_bytes"
                                | "platform_random_bounded"
                                | "platform_random_split"
                                | "platform_uuid_v4"
                                | "platform_uuid_v7"
                                | "platform_tcp_accept"
                                | "platform_tcp_read"
                                | "platform_tcp_write"
                                | "platform_tcp_shutdown"
                                | "platform_tcp_configure"
                                | "platform_udp_send_to"
                                | "platform_udp_receive_from"
                                | "platform_udp_configure"
                                | "platform_tls_client"
                                | "platform_tls_read"
                                | "platform_tls_write"
                                | "platform_tls_shutdown"
                                | "platform_close"
                                | "platform_digest"
                                | "platform_hmac"
                                | "platform_parse_socket"
                                | "platform_cancel"
                                | "platform_destroy_secret",
                            0,
                        ) | ("platform_parse_socket" | "platform_hmac", 1)
                            | (
                                "platform_tcp_connect"
                                    | "platform_tcp_accept"
                                    | "platform_tls_shutdown",
                                2
                            )
                            | (
                                "platform_tcp_connect_host"
                                    | "platform_tcp_read"
                                    | "platform_tcp_write"
                                    | "platform_udp_receive_from"
                                    | "platform_dns_lookup"
                                    | "platform_tls_client"
                                    | "platform_tls_read"
                                    | "platform_tls_write",
                                3,
                            )
                            | ("platform_udp_send_to", 4)
                    ) || function.starts_with("platform_result_") && index == 0;
                    if borrowed {
                        format!("&({value})")
                    } else {
                        value
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_{function}({values})");
        }
        let concurrency_call = [
            ("no-capability", "platform_no_resource"),
            ("int-channel", "platform_int_channel"),
            ("int-channel-send", "platform_int_channel_send"),
            ("int-channel-receive", "platform_int_channel_receive"),
            (
                "int-channel-try-receive",
                "platform_int_channel_try_receive",
            ),
            ("int-mutex", "platform_int_mutex"),
            ("int-mutex-load", "platform_int_mutex_load"),
            ("int-mutex-store", "platform_int_mutex_store"),
            ("int-mutex-add", "platform_int_mutex_add"),
            ("int-read-write-lock", "platform_int_rw_lock"),
            ("int-read-write-lock-read", "platform_int_rw_lock_read"),
            ("int-read-write-lock-write", "platform_int_rw_lock_write"),
            ("atomic-int64", "platform_atomic_int64"),
            ("atomic-int64-load", "platform_atomic_int64_load"),
            ("atomic-int64-store", "platform_atomic_int64_store"),
            ("atomic-int64-add", "platform_atomic_int64_add"),
            ("thread-local-int", "platform_thread_local_int"),
            ("thread-local-int-get", "platform_thread_local_int_get"),
            ("thread-local-int-set", "platform_thread_local_int_set"),
            ("result-failed", "platform_result_failed"),
            ("result-message", "platform_result_message"),
            ("result-int", "platform_result_int"),
            ("result-bool", "platform_result_bool"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("/core/platform-concurrency::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = concurrency_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let value = self.expression(value);
                    let borrowed = (index == 0
                        && (function.starts_with("platform_result_")
                            || !matches!(
                                function,
                                "platform_int_channel"
                                    | "platform_int_mutex"
                                    | "platform_int_rw_lock"
                                    | "platform_atomic_int64"
                                    | "platform_thread_local_int"
                            )))
                        || (function == "platform_int_channel_send" && index == 3)
                        || (function == "platform_int_channel_receive" && index == 2);
                    if borrowed {
                        format!("&({value})")
                    } else {
                        value
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_{function}({values})");
        }
        if self.is_builtin(callee, "/core/platform-adapters::system-host-name") {
            return "terrane_platform_support::system_host_name()".to_owned();
        }
        let adapter_result_field = [
            ("result-failed", "failed", false),
            ("result-bool", "flag", false),
            ("result-message", "message", true),
            ("result-text", "text", true),
        ]
        .into_iter()
        .find_map(|(terrane, field, cloned)| {
            self.is_builtin(callee, &format!("/core/platform-adapters::{terrane}"))
                .then_some((field, cloned))
        });
        if let Some((field, cloned)) = adapter_result_field {
            let value = self.expression(argument_values[0]);
            if cloned {
                return format!("({value}).{field}.clone()");
            }
            return format!("({value}).{field}");
        }
        let system_call = [
            (
                "acquire-filesystem-authority",
                "acquire_filesystem_authority",
            ),
            ("filesystem-exists", "filesystem_exists"),
            ("filesystem-metadata", "filesystem_metadata"),
            ("filesystem-realpath", "filesystem_realpath"),
            ("filesystem-read-link", "filesystem_read_link"),
            ("filesystem-read-bounded", "filesystem_read_bounded"),
            ("filesystem-write-atomic", "filesystem_write_atomic"),
            ("filesystem-rename", "filesystem_rename"),
            ("filesystem-remove", "filesystem_remove"),
            ("result-failed", "filesystem_result_failed"),
            ("result-message", "filesystem_result_message"),
            ("result-text", "filesystem_result_text"),
            ("result-detail", "filesystem_result_detail"),
            ("result-bytes", "filesystem_result_bytes"),
            ("result-int", "filesystem_result_int"),
            ("result-bool", "filesystem_result_bool"),
            ("platform-value-is-text", "platform_value_is_text"),
            ("platform-value-text", "platform_value_text"),
            ("platform-value-bytes", "platform_value_bytes"),
            ("process-arguments", "process_arguments"),
            ("environment-entries", "environment_entries"),
            ("process-exit", "process_exit"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("/core/platform-system::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = system_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    if (function == "filesystem_call" && index == 4) || function == "process_exit" {
                        self.expression_as(value, ValueType::Scalar(ScalarType::Int))
                    } else {
                        self.expression(value)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            if function.starts_with("filesystem_result_") || function.starts_with("platform_value_")
            {
                return format!("terrane_{function}(&({values}))");
            }
            return format!("terrane_{function}({values})");
        }
        let platform_call = [
            ("acquire-stdin", "acquire_stdin"),
            ("acquire-stdout", "acquire_stdout"),
            ("acquire-stderr", "acquire_stderr"),
            ("open-file", "open_file"),
            ("open-directory-beneath", "open_directory_beneath"),
            ("open-file-beneath", "open_file_beneath"),
            ("read", "read"),
            ("write", "write"),
            ("flush", "flush"),
            ("sync-data", "sync_data"),
            ("sync-all", "sync_all"),
            ("close", "close"),
            ("release", "release"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("/core/platform-streams::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = platform_call {
            let values = argument_values
                .iter()
                .map(|value| self.expression(value))
                .collect::<Vec<_>>();
            if function.starts_with("acquire_") {
                return format!("terrane_platform_{function}()");
            }
            if matches!(function, "open_file" | "open_directory_beneath") {
                return format!("terrane_platform_{function}({})", values.join(", "));
            }
            if function == "write" && values.len() == 3 {
                return format!(
                    "terrane_platform_write(&({}), &({}), terrane_int_support::Int::from(({}).clone()))",
                    values[0], values[1], values[2]
                );
            }
            let Some((handle, arguments)) = values.split_first() else {
                unreachable!("validated platform operation has a stream handle");
            };
            let arguments = std::iter::once(format!("&({handle})"))
                .chain(arguments.iter().cloned())
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_platform_{function}({arguments})");
        }
        let mut values = argument_values
            .into_iter()
            .map(|value| self.expression(value))
            .collect::<Vec<_>>();
        if callee.kind == SyntaxKind::MemberExpression
            && callee
                .children
                .get(1)
                .is_some_and(|member| self.text(member) == "join")
        {
            let separator = self.receiver_expression(&callee.children[0]);
            let values = values
                .into_iter()
                .map(|value| format!("terrane_scalar_support::scalar_text(&({value}))"))
                .collect::<Vec<_>>();
            if values.is_empty() {
                return format!("{{ let _ = {separator}; String::new() }}");
            }
            return format!("vec![{}].join(&({separator}))", values.join(", "));
        }
        if callee.kind == SyntaxKind::MemberExpression
            && callee
                .children
                .get(1)
                .is_some_and(|member| self.text(member) == "concat")
        {
            let receiver = self.receiver_expression(&callee.children[0]);
            if self.value_type(&callee.children[0]) == Some(ValueType::Scalar(ScalarType::Bytes)) {
                let value = values
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Vec::new()".to_owned());
                return format!("{{ let mut bytes = {receiver}; bytes.extend({value}); bytes }}");
            }
            values.insert(0, receiver);
            let values = values
                .into_iter()
                .map(|value| format!("terrane_scalar_support::scalar_text(&({value}))"))
                .collect::<Vec<_>>();
            let format = "{}".repeat(values.len());
            return format!("format!(\"{format}\", {})", values.join(", "));
        }
        let projected_parameters = self
            .projected_function_for_call(callee)
            .map(|function| function.parameters.clone());
        let contract = self.contract_for_call(callee).cloned();
        if let Some(contract) = &contract {
            let mut ordered = vec![None; contract.parameters.len()];
            let mut positional = 0;
            for argument in &arguments.children {
                let named = argument
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
                let index = named.map_or_else(
                    || {
                        let index = positional;
                        positional += 1;
                        index
                    },
                    |name| {
                        contract
                            .parameters
                            .iter()
                            .position(|parameter| parameter.name == self.text(name))
                            .expect("validated named argument")
                    },
                );
                let value = argument.children.last().unwrap_or(argument);
                let parameter = &contract.parameters[index];
                let expression = if let Some(ty) = parameter.value_type.clone() {
                    self.expression_as(value, ty)
                } else {
                    self.expression(value)
                };
                ordered[index] = Some(
                    projected_parameters
                        .as_ref()
                        .and_then(|parameters| parameters.get(index))
                        .filter(|parameter| {
                            parameter.borrowed
                                && matches!(
                                    parameter.ty,
                                    crate::projection::ProjectedType::Foreign { .. }
                                )
                        })
                        .map_or(expression.clone(), |parameter| {
                            if parameter.mutable_borrow {
                                format!("&mut {expression}")
                            } else {
                                format!("&{expression}")
                            }
                        }),
                );
            }
            self.append_defaults(contract, &mut ordered);
            values = ordered.into_iter().flatten().collect();
        } else if let Some(ValueType::Function(parameters, _)) = self.value_type(callee) {
            values = arguments
                .children
                .iter()
                .zip(parameters)
                .map(|(argument, parameter)| {
                    self.expression_as(
                        argument.children.last().unwrap_or(argument),
                        parameter.value_type(),
                    )
                })
                .collect();
        }
        let name = if let Some(contract) = &contract
            && contract.owner.is_none()
        {
            function_name(contract)
        } else if contract
            .as_ref()
            .is_some_and(|contract| contract.owner.is_some())
            && let [receiver, member] = callee.children.as_slice()
        {
            format!(
                "({}).{}",
                self.receiver_expression(receiver),
                rust_name(self.text(member))
            )
        } else {
            self.expression(callee)
        };
        let call = format!("{name}({})", values.join(", "));
        let foreign_method = contract.as_ref().and_then(|contract| {
            let [receiver, _member] = callee.children.as_slice() else {
                return None;
            };
            let ValueType::Object(identity) = self.value_type(receiver)? else {
                return None;
            };
            self.package
                .projection
                .method(&identity.namespace, &identity.name, &contract.name)
        });
        let foreign_error = foreign_method.is_some()
            || (callee.kind == SyntaxKind::Name
                && self
                    .package
                    .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                    .and_then(|symbol| symbol.identity.rsplit_once("::"))
                    .and_then(|(namespace, name)| self.package.projection.item(namespace, name))
                    .is_some_and(|item| {
                        matches!(&item.kind, crate::projection::ProjectedKind::Function(_))
                    }));
        let call = if let Some(method) = foreign_method {
            let [receiver, _member] = callee.children.as_slice() else {
                unreachable!("projected methods have a receiver")
            };
            let Some(ValueType::Object(identity)) = self.value_type(receiver) else {
                unreachable!("projected method receiver has an object type")
            };
            let type_path = self
                .package
                .projection
                .foreign_rust_path(&identity.namespace, &identity.name)
                .expect("foreign method owner has a projected Rust path");
            let dependency = type_path.split("::").next().unwrap_or("dependency");
            let member = format!("{type_path}::{}", method.name);
            let catch_unwind = |body: &str| {
                if method.receiver.is_some() {
                    format!("std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {body}))")
                } else {
                    format!("std::panic::catch_unwind(|| {body})")
                }
            };
            if self.package.profile.panic == crate::package::PanicProfile::Abort {
                if method.error.is_some() {
                    format!(
                        "match {call} {{ Ok(value) => Ok(value), Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{dependency}` member `{member}` failed: {{error}}\"), crate::TERRANE_NO_SITE))) }}"
                    )
                } else if self.discarded_call == Some(node.span) {
                    format!("{{ {call}; Ok(()) }}")
                } else {
                    format!("Ok({call})")
                }
            } else if method.error.is_some() {
                let caught = catch_unwind(&call);
                format!(
                    "match {caught} {{ Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{dependency}` member `{member}` failed: {{error}}\"), crate::TERRANE_NO_SITE))), Err(payload) => Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
                )
            } else {
                let unwind_body = if self.discarded_call == Some(node.span) {
                    format!("{{ {call}; }}")
                } else {
                    call
                };
                let caught = catch_unwind(&unwind_body);
                format!(
                    "match {caught} {{ Ok(value) => Ok(value), Err(payload) => Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
                )
            }
        } else {
            call
        };
        let call = if contract.as_ref().is_some_and(|contract| contract.is_async)
            && matches!(self.value_type(node), Some(ValueType::Task(_)))
        {
            format!("Box::pin({call})")
        } else {
            call
        };
        if contract.is_some_and(|contract| contract.throws) || foreign_error {
            let site = self.error_site(node);
            let dependency_boundary = self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .is_some_and(|symbol| symbol.identity.starts_with("/deps/"));
            if foreign_error || dependency_boundary {
                if self.try_completion {
                    format!("__terrane_raised_completion!({call}, {site})")
                } else if self.propagate_errors {
                    format!("__terrane_raised_err({call}, {site})?")
                } else {
                    format!("__terrane_raised({call}, {site})")
                }
            } else if self.try_completion {
                format!("__terrane_traced_completion!({call}, {site})")
            } else if self.propagate_errors {
                format!("__terrane_traced_err({call}, {site})?")
            } else {
                format!("__terrane_traced({call}, {site})")
            }
        } else {
            call
        }
    }

    fn fallible(&self, call: impl AsRef<str>, node: &SyntaxNode) -> String {
        let call = call.as_ref();
        let site = self.error_site(node);
        if self.try_completion {
            format!("__terrane_raised_completion!({call}, {site})")
        } else if self.propagate_errors {
            format!("__terrane_raised_err({call}, {site})?")
        } else {
            format!("__terrane_raised({call}, {site})")
        }
    }

    fn error_site(&self, node: &SyntaxNode) -> String {
        let function = self
            .current_function
            .as_deref()
            .unwrap_or(self.unit.namespace.as_str());
        let site =
            self.registry
                .register_site(&self.unit.source_path, function, self.source, node.span);
        let (line, column) = self.source.line_column(node.span.start);
        let (end_line, end_column) = self.source.line_column(node.span.end);
        let comment = format!(
            "{}:{line}:{column}-{end_line}:{end_column}",
            self.unit.source_path
        );
        format!("__terrane_comment!({site}, {comment:?})")
    }

    fn numeric_destination(
        &mut self,
        node: &SyntaxNode,
        source: ScalarType,
        destination: ScalarType,
    ) -> String {
        let value = self.receiver_expression(node);
        if destination == ScalarType::Int {
            if matches!(source, ScalarType::Float32 | ScalarType::Float64) {
                let helper = if source == ScalarType::Float32 {
                    "exact_int_f32"
                } else {
                    "exact_int_f64"
                };
                return self.fallible(format!("terrane_int_support::{helper}({value})"), node);
            }
            return if source == ScalarType::Uint128 {
                format!("terrane_int_support::adaptive(&({value}))")
            } else {
                format!("terrane_int_support::Int::from(({value}) as i128)")
            };
        }
        if source == ScalarType::Int && destination.is_integer() {
            return self.fallible(
                format!(
                    "terrane_int_support::coerce::<{}>(&({value}))",
                    rust_type(destination)
                ),
                node,
            );
        }
        if source == ScalarType::Int {
            let helper = if destination == ScalarType::Float32 {
                "exact_f32"
            } else {
                "exact_f64"
            };
            return self.fallible(format!("terrane_int_support::{helper}(&({value}))"), node);
        }
        if source.is_integer() && destination.is_integer() {
            if integer_range_contains(destination, source) {
                return format!("(({value}) as {})", rust_type(destination));
            }
            let conversion = self.fallible(
                format!(
                    "{}::try_from(source_value).map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(&source_value, \"{source}\", \"{destination}\", \"the value is outside the destination range\"))",
                    rust_type(destination)
                ),
                node,
            );
            return format!("{{ let source_value = {value}; {conversion} }}");
        }
        if source == ScalarType::Float32 && destination == ScalarType::Float64 {
            return format!("(({value}) as f64)");
        }
        if source.is_integer() {
            if exact_integer_float_widening(source, destination)
                || self.bounded_float_conversion_is_exact(node, destination)
            {
                return format!("(({value}) as {})", rust_type(destination));
            }
            let helper = if destination == ScalarType::Float32 {
                "exact_fixed_f32"
            } else {
                "exact_fixed_f64"
            };
            return self.fallible(format!("terrane_int_support::{helper}({value})"), node);
        }
        if destination.is_integer() {
            let helper = if source == ScalarType::Float32 {
                "exact_from_f32"
            } else {
                "exact_from_f64"
            };
            return self.fallible(
                format!(
                    "terrane_int_support::{helper}::<{}>({value})",
                    rust_type(destination)
                ),
                node,
            );
        }
        let conversion = self.fallible(
            "Err(terrane_int_support::ArithmeticError::conversion_overflow(&source_value, \"float64\", \"float32\", \"the floating value is not exactly representable\"))",
            node,
        );
        format!(
            "{{ let source_value = {value}; let converted = source_value as f32; if (converted as f64) == source_value {{ converted }} else {{ {conversion} }} }}"
        )
    }

    #[allow(clippy::too_many_lines)]
    fn string_call(&mut self, node: &SyntaxNode, arguments: &SyntaxNode) -> Option<String> {
        let selection = string_call_selection(self.source, node)?;
        let subject = find_node_by_span(&self.unit.tree.root, selection.receiver)
            .expect("selected string receiver belongs to this syntax tree");
        let family = selection.family.source_name();
        let child = selection.child.as_str();
        let receiver = self.receiver_expression(subject);
        let values = arguments
            .children
            .iter()
            .map(|argument| argument.children.last().unwrap_or(argument))
            .map(|value| self.expression(value))
            .collect::<Vec<_>>();
        let result = match (family, child) {
            ("trim", "default") => {
                format!("terrane_string_support::trim(&({receiver}))")
            }
            ("trim", mode @ ("start" | "end")) => {
                let helper = if mode == "start" {
                    "trim_start"
                } else {
                    "trim_end"
                };
                let pattern = values
                    .first()
                    .map_or_else(|| "None".to_owned(), |value| format!("Some(&({value}))"));
                format!("terrane_string_support::{helper}(&({receiver}), {pattern})")
            }
            ("contains", "default") => format!("({receiver}).contains(&({}))", values[0]),
            ("contains", "start") => format!("({receiver}).starts_with(&({}))", values[0]),
            ("contains", "end") => format!("({receiver}).ends_with(&({}))", values[0]),
            ("find", "default") => {
                format!(
                    "terrane_string_support::find(&({receiver}), &({}))",
                    values[0]
                )
            }
            ("find", "all") => {
                format!(
                    "terrane_string_support::find_all(&({receiver}), &({}))",
                    values[0]
                )
            }
            ("find", "count") => format!(
                "terrane_int_support::Int::from(terrane_string_support::find_all(&({receiver}), &({})).len() as i128)",
                values[0]
            ),
            ("upper", mode) => {
                let helper = match mode {
                    "default" => "upper",
                    "first" => "upper_first",
                    "words" => "upper_words",
                    _ => unreachable!(),
                };
                format!("terrane_string_support::{helper}(&({receiver}))")
            }
            ("lower", mode) => {
                let helper = match mode {
                    "default" => "lower",
                    "first" => "lower_first",
                    _ => unreachable!(),
                };
                format!("terrane_string_support::{helper}(&({receiver}))")
            }
            ("case-fold", _) => format!("terrane_string_support::case_fold(&({receiver}))"),
            ("normalise", form) => {
                format!("terrane_string_support::normalise(&({receiver}), {form:?})")
            }
            ("split", _) => {
                format!(
                    "terrane_string_support::split(&({receiver}), &({}))",
                    values[0]
                )
            }
            ("replace", _) => format!(
                "terrane_string_support::replace(&({receiver}), &({}), &({}))",
                values[0], values[1]
            ),
            ("encode", _) => {
                format!(
                    "terrane_string_support::encode(&({receiver}), {})",
                    values[0]
                )
            }
            ("decode", _) => self.fallible(
                format!(
                    "terrane_string_support::decode(&({receiver}), {})",
                    values[0]
                ),
                node,
            ),
            _ => unreachable!("semantic analysis validated string family"),
        };
        Some(result)
    }
    #[allow(clippy::too_many_lines)]
    fn arithmetic_family(
        &mut self,
        family: ArithmeticFamily,
        child: &str,
        receiver_node: &SyntaxNode,
        arguments: &SyntaxNode,
        call: &SyntaxNode,
    ) -> String {
        let Some(ValueType::Scalar(receiver_type)) = self.receiver_value_type(receiver_node) else {
            unreachable!("validated arithmetic receiver");
        };
        let receiver = if receiver_type == ScalarType::Int {
            self.expression_as(receiver_node, ValueType::Scalar(ScalarType::Int))
        } else {
            Self::unwrapped_expression(self.receiver_expression(receiver_node))
        };
        let argument = arguments
            .children
            .first()
            .and_then(|argument| argument.children.last())
            .map(|argument| {
                if receiver_type == ScalarType::Int {
                    self.adaptive_expression(argument)
                } else if matches!(
                    family,
                    ArithmeticFamily::ShiftLeft | ArithmeticFamily::ShiftRight
                ) {
                    self.expression(argument)
                } else {
                    self.expression_as(argument, ValueType::Scalar(receiver_type))
                }
            });
        if receiver_type == ScalarType::Int {
            let expression = match family {
                ArithmeticFamily::Add => format!("({receiver} + {})", argument.unwrap()),
                ArithmeticFamily::Subtract => format!("({receiver} - {})", argument.unwrap()),
                ArithmeticFamily::Multiply => format!("({receiver} * {})", argument.unwrap()),
                ArithmeticFamily::Divide => {
                    format!("({receiver}).euclidean_div(&({}))", argument.unwrap())
                }
                ArithmeticFamily::Remainder => {
                    format!("({receiver}).modulo(&({}))", argument.unwrap())
                }
                ArithmeticFamily::DivRem => {
                    format!("({receiver}).div_rem(&({}))", argument.unwrap())
                }
                ArithmeticFamily::Negate => format!("-({receiver})"),
                ArithmeticFamily::ShiftLeft => {
                    format!("({receiver}).shift_left(&({}))", argument.unwrap())
                }
                ArithmeticFamily::ShiftRight => {
                    format!("({receiver}).shift_right(&({}))", argument.unwrap())
                }
            };
            return if child == "checked" {
                format!("({expression}).ok()")
            } else if matches!(
                family,
                ArithmeticFamily::Divide
                    | ArithmeticFamily::Remainder
                    | ArithmeticFamily::DivRem
                    | ArithmeticFamily::ShiftLeft
                    | ArithmeticFamily::ShiftRight
            ) {
                self.fallible(expression, call)
            } else {
                expression
            };
        }
        let operation = match family {
            ArithmeticFamily::Add => "addition",
            ArithmeticFamily::Subtract => "subtraction",
            ArithmeticFamily::Multiply => "multiplication",
            ArithmeticFamily::Divide => "division",
            ArithmeticFamily::Remainder => "remainder",
            ArithmeticFamily::DivRem => "div_rem",
            ArithmeticFamily::Negate => "negation",
            ArithmeticFamily::ShiftLeft => "shift_left",
            ArithmeticFamily::ShiftRight => "shift_right",
        };
        let helper = if child == "default" {
            format!("fixed_{operation}")
        } else {
            format!("fixed_{operation}_{child}")
        };
        let expression = if family == ArithmeticFamily::Negate {
            format!("terrane_int_support::{helper}({receiver})")
        } else if matches!(
            family,
            ArithmeticFamily::ShiftLeft | ArithmeticFamily::ShiftRight
        ) {
            format!(
                "terrane_int_support::{helper}({receiver}, &({}))",
                argument.unwrap()
            )
        } else {
            format!(
                "terrane_int_support::{helper}({receiver}, {})",
                argument.unwrap()
            )
        };
        let fallible = child == "default"
            || matches!(
                family,
                ArithmeticFamily::Divide
                    | ArithmeticFamily::Remainder
                    | ArithmeticFamily::DivRem
                    | ArithmeticFamily::ShiftLeft
                    | ArithmeticFamily::ShiftRight
            );
        if fallible {
            self.fallible(expression, call)
        } else {
            expression
        }
    }

    fn integer_coercion(
        &mut self,
        method: &crate::BoundMethod,
        receiver: &SyntaxNode,
        callee: &SyntaxNode,
        arguments: &SyntaxNode,
    ) -> String {
        let policy = match method.child {
            "default" => CoercionPolicy::Default,
            child => CoercionPolicy::from_member(child)
                .expect("validated coercion family child must select a policy"),
        };
        let destination = arguments
            .children
            .first()
            .and_then(|argument| argument.children.last())
            .unwrap_or_else(|| &arguments.children[0]);
        let destination = self
            .descriptor_type(destination)
            .expect("validated coercion destination must resolve to a scalar descriptor");
        let receiver_is_borrowed = receiver.kind == SyntaxKind::Name
            && self.lazy_namespace_binding_type(receiver).is_some();
        if policy == CoercionPolicy::Default
            && !receiver_is_borrowed
            && let Some(ValueType::Scalar(source)) = self.receiver_value_type(receiver)
        {
            if source == destination {
                return if destination == ScalarType::Int
                    && self.small_int_binding(receiver).is_some()
                {
                    self.expression_as(receiver, ValueType::Scalar(ScalarType::Int))
                } else {
                    self.receiver_expression(receiver)
                };
            }
            return self.numeric_destination(receiver, source, destination);
        }
        let helper = match policy {
            CoercionPolicy::Default => "coerce",
            CoercionPolicy::Checked => "checked_coerce",
            CoercionPolicy::Wrap => "wrapping_coerce",
            CoercionPolicy::Saturate => "saturating_coerce",
        };
        let receiver = self.receiver_expression(receiver);
        let source = if receiver_is_borrowed {
            receiver
        } else {
            format!("&({receiver})")
        };
        let call = format!(
            "terrane_int_support::{helper}::<{}>({source})",
            rust_type(destination)
        );
        if policy == CoercionPolicy::Default {
            self.fallible(call, callee)
        } else {
            call
        }
    }

    fn descriptor_identity(&self, node: &SyntaxNode) -> Option<String> {
        if node.kind == SyntaxKind::TypeExpression {
            return node
                .children
                .first()
                .and_then(|child| self.descriptor_identity(child));
        }
        if node.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = node.children.as_slice()
            && self.text(member) == "type"
        {
            return self
                .value_type(receiver)
                .and_then(|value_type| match value_type {
                    ValueType::Scalar(value_type) => Some(format!("type:{value_type}")),
                    _ => None,
                });
        }
        crate::semantics::descriptor_expression_type(self.package, self.unit, node)
            .map(|scalar| format!("type:{scalar}"))
    }

    fn descriptor_type(&self, node: &SyntaxNode) -> Option<ScalarType> {
        let resolved = crate::semantics::descriptor_expression_type(self.package, self.unit, node);
        resolved.or_else(|| {
            (node.kind == SyntaxKind::TypeExpression)
                .then(|| {
                    node.children
                        .first()
                        .and_then(|child| self.descriptor_type(child))
                })
                .flatten()
        })
    }

    fn projected_function_for_call(
        &self,
        callee: &SyntaxNode,
    ) -> Option<&crate::projection::ProjectedFunction> {
        if callee.kind != SyntaxKind::Name {
            return None;
        }
        let symbol =
            self.package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))?;
        self.package
            .projection
            .item(&symbol.namespace, &symbol.name)
            .and_then(|item| match &item.kind {
                crate::projection::ProjectedKind::Function(function) => Some(function),
                _ => None,
            })
    }

    fn contract_for_call(&self, callee: &SyntaxNode) -> Option<&FunctionContract> {
        if let [receiver, member] = callee.children.as_slice()
            && callee.kind == SyntaxKind::MemberExpression
            && let Some(ValueType::Object(object_name)) = self.receiver_value_type(receiver)
            && let Some(object) = self
                .unit
                .objects
                .iter()
                .find(|object| object.identity == object_name)
        {
            return effective_object_methods(self.unit, object)
                .into_iter()
                .find(|contract| contract.name == self.text(member));
        }
        if callee.kind != SyntaxKind::Name {
            return None;
        }
        let symbol =
            self.package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))?;
        let span = symbol.declaration_span?;
        if let Some((namespace, object_name)) = symbol.identity.rsplit_once("::")
            && let Some(unit) = self
                .package
                .units
                .iter()
                .find(|unit| unit.namespace == namespace)
            && unit.objects.iter().any(|object| object.name == object_name)
        {
            return unit.functions.iter().find(|contract| {
                contract.owner.as_deref() == Some(object_name) && contract.name == "construct"
            });
        }
        self.package
            .units
            .iter()
            .flat_map(|unit| &unit.functions)
            .find(|contract| contract.span == span)
    }

    fn name(&self, node: &SyntaxNode) -> String {
        let source_name = self.text(node);
        if source_name == "none" {
            return "()".to_owned();
        }
        if source_name == "this" {
            return if self.closure_depth == 0 {
                "self"
            } else {
                "this"
            }
            .to_owned();
        }
        let narrowed =
            narrowed_value_type(self.unit, node, &self.unit.typed_bindings).or_else(|| {
                self.parameter_types
                    .iter()
                    .rev()
                    .find(|(name, _)| name == source_name)
                    .and_then(|(_, value_type)| {
                        narrowed_optional_type(self.unit, node, value_type.clone())
                    })
            });
        if let Some(narrowed) = narrowed {
            let access = format!(
                "{}.as_ref().expect(\"semantic optional narrowing\")",
                rust_name(source_name)
            );
            return if matches!(narrowed, ValueType::Scalar(_)) {
                format!("*{access}")
            } else {
                access
            };
        }
        if let Some((_, local)) = self
            .namespace_initializer
            .as_ref()
            .filter(|(name, _)| name == source_name)
        {
            return local.clone();
        }
        let resolved = self
            .package
            .resolve_name_at(self.unit, node.span.start, source_name);
        let encoding = resolved
            .and_then(|symbol| symbol.identity.strip_prefix("/core/encodings::"))
            .and_then(|name| match name {
                "utf8" => Some("Utf8"),
                "utf16-le" => Some("Utf16Le"),
                "utf16-be" => Some("Utf16Be"),
                "utf32-le" => Some("Utf32Le"),
                "utf32-be" => Some("Utf32Be"),
                _ => None,
            });
        if let Some(encoding) = encoding {
            return format!("terrane_string_support::Encoding::{encoding}");
        }
        let Some(symbol) = self
            .package
            .resolve_name_at(self.unit, node.span.start, source_name)
        else {
            return rust_name(source_name);
        };
        if symbol.kind != SymbolKind::Binding {
            return rust_name(source_name);
        }
        if symbol.global {
            let storage = global_binding_name(&symbol.name);
            let failure = self.uninitialized_global_failure(node);
            return format!(
                "{storage}.lock().expect(\"program-global lock poisoned\").clone().unwrap_or_else(|| {failure})"
            );
        }
        let Some(span) = symbol.declaration_span else {
            return rust_name(source_name);
        };
        if let Some(binding) = self
            .unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span)
            && self.reference_backed(binding)
        {
            return format!(
                "({{ let __terrane_value = {}.lock().expect(\"reference lock poisoned\").clone(); __terrane_value }})",
                rust_name(source_name)
            );
        }
        let name = namespace_binding_name(span.file, &symbol.name);
        if self.lazy_namespace_binding_type(node).is_some() {
            format!("&*{name}")
        } else if self.is_namespace_binding_span(span) {
            name
        } else {
            rust_name(source_name)
        }
    }

    fn uninitialized_global_failure(&self, node: &SyntaxNode) -> String {
        let (line, column) = self.source.line_column(node.span.start);
        format!(
            "__terrane_uninitialized_global({:?}, {:?}, {line}, {column})",
            self.text(node),
            display_path(self.source.path())
        )
    }

    fn namespace_name(&self, node: &SyntaxNode) -> String {
        self.package
            .resolve_name_at(self.unit, node.span.start, self.text(node))
            .and_then(|symbol| {
                symbol
                    .declaration_span
                    .map(|span| namespace_binding_name(span.file, &symbol.name))
            })
            .unwrap_or_else(|| rust_name(self.text(node)))
    }

    fn local_typed_binding(&self, node: &SyntaxNode) -> Option<&TypedBinding> {
        (node.kind == SyntaxKind::Name)
            .then(|| {
                self.unit.typed_bindings.iter().rev().find(|binding| {
                    binding.name == self.text(node)
                        && binding.is_visible_at(self.source.id(), node.span.start)
                        && !self.is_namespace_binding_span(binding.span)
                })
            })
            .flatten()
    }

    fn binding_has_bounded_integer_range(&self, node: &SyntaxNode) -> bool {
        self.local_typed_binding(node).is_some_and(|binding| {
            self.bounded_integer_ranges
                .iter()
                .rev()
                .any(|range| range.binding == binding.span)
        })
    }

    fn bounded_integer_range(
        &self,
        condition: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Option<BoundedIntegerRange> {
        let [left, right] = condition.children.as_slice() else {
            return None;
        };
        (self.source.text()[left.span.end..right.span.start].trim() == "<").then_some(())?;
        let binding = self.local_typed_binding(left)?;
        let ValueType::Scalar(storage) = binding.value_type else {
            return None;
        };
        fixed_integer_shape(storage)?;
        let ContextualConstant::Integer(upper) =
            contextual_constant(self.source, right, storage)?.ok()?
        else {
            return None;
        };
        let declaration = find_node_by_span(&self.unit.tree.root, binding.span)?;
        let name_index = declaration
            .children
            .iter()
            .position(|child| child.kind == SyntaxKind::Name)?;
        let initializer = binding_initializer(declaration, name_index)?;
        let ContextualConstant::Integer(lower) =
            contextual_constant(self.source, initializer, storage)?.ok()?
        else {
            return None;
        };
        (lower <= upper).then_some(())?;

        let direct_increment_count = block
            .children
            .iter()
            .filter(|statement| {
                statement.kind == SyntaxKind::PostfixExpression
                    && statement.children.first().is_some_and(|target| {
                        self.local_typed_binding(target)
                            .is_some_and(|target_binding| target_binding.span == binding.span)
                    })
                    && self.source.text()[statement.span.start..statement.span.end]
                        .trim_end()
                        .ends_with("++")
            })
            .count();
        (direct_increment_count == 1).then_some(())?;

        fn mutation_count(
            emitter: &Emitter<'_>,
            node: &SyntaxNode,
            binding: &TypedBinding,
        ) -> usize {
            let own = usize::from(
                matches!(
                    node.kind,
                    SyntaxKind::Assignment | SyntaxKind::PostfixExpression
                ) && node.children.first().is_some_and(|target| {
                    emitter
                        .local_typed_binding(target)
                        .is_some_and(|target_binding| target_binding.span == binding.span)
                }),
            );
            own + node
                .children
                .iter()
                .map(|child| mutation_count(emitter, child, binding))
                .sum::<usize>()
        }
        (mutation_count(self, block, binding) == 1).then_some(())?;
        Some(BoundedIntegerRange {
            binding: binding.span,
            lower,
            upper,
        })
    }

    fn bounded_float_conversion_is_exact(
        &self,
        node: &SyntaxNode,
        destination: ScalarType,
    ) -> bool {
        let Some(binding) = self.local_typed_binding(node) else {
            return false;
        };
        let precision = match destination {
            ScalarType::Float32 => 24,
            ScalarType::Float64 => 53,
            _ => return false,
        };
        let limit = BigInt::from(1_u8) << precision;
        self.bounded_integer_ranges.iter().rev().any(|range| {
            range.binding == binding.span && range.lower >= -&limit && range.upper <= limit
        })
    }

    fn small_int_binding(&self, node: &SyntaxNode) -> Option<ScalarType> {
        (node.kind == SyntaxKind::Name)
            .then(|| {
                self.unit
                    .typed_bindings
                    .iter()
                    .rev()
                    .find(|binding| {
                        binding.name == self.text(node)
                            && binding.is_visible_at(self.source.id(), node.span.start)
                            && !self.is_namespace_binding_span(binding.span)
                            && !binding_span_is_mutated(self.package, self.unit, binding.span, true)
                    })
                    .and_then(|binding| binding.storage_type)
            })
            .flatten()
    }

    fn lazy_namespace_binding_type(&self, node: &SyntaxNode) -> Option<ValueType> {
        if self
            .namespace_initializer
            .as_ref()
            .is_some_and(|(name, _)| name == self.text(node))
        {
            return None;
        }
        let symbol = self
            .package
            .resolve_name_at(self.unit, node.span.start, self.text(node))?;
        if symbol.global {
            return None;
        }
        let span = symbol.declaration_span?;
        if !self.is_namespace_binding_span(span) {
            return None;
        }
        let owner = self
            .package
            .units
            .iter()
            .find(|unit| unit.source.id() == span.file)?;
        owner
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span)
            .map(|binding| binding.value_type.clone())
    }

    fn is_namespace_binding_span(&self, span: crate::Span) -> bool {
        self.package
            .units
            .iter()
            .find(|unit| unit.source.id() == span.file)
            .is_some_and(|unit| {
                unit.tree.root.children.iter().any(|candidate| {
                    candidate.span == span
                        && matches!(candidate.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
                })
            })
    }

    fn append_defaults(&self, contract: &FunctionContract, values: &mut [Option<String>]) {
        if values.iter().all(Option::is_some) {
            return;
        }
        let Some(owner) = self
            .package
            .units
            .iter()
            .find(|unit| unit.source.id() == contract.span.file)
        else {
            return;
        };
        let Some(function) = find_node(
            &owner.tree.root,
            SyntaxKind::FunctionDeclaration,
            contract.span,
        ) else {
            return;
        };
        let Some(parameters) = function
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::ParameterList)
        else {
            return;
        };
        for (index, parameter) in parameters.children.iter().enumerate() {
            if values[index].is_some() {
                continue;
            }
            if let Some(default) = parameter.children.last().filter(|child| {
                !matches!(child.kind, SyntaxKind::Name | SyntaxKind::TypeExpression)
            }) {
                let destination = contract.parameters[index].value_type.clone();
                let value = destination
                    .and_then(|destination| match destination {
                        ValueType::Scalar(destination) => {
                            contextual_constant(&owner.source, default, destination)
                                .and_then(Result::ok)
                                .map(|constant| lower_contextual_constant(constant, destination))
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| literal_or_text(&owner.source, default));
                values[index] = Some(value);
            }
        }
    }

    fn is_builtin(&self, node: &SyntaxNode, identity: &str) -> bool {
        let SyntaxKind::Name = node.kind else {
            return false;
        };
        self.package
            .resolve_name_at(self.unit, node.span.start, self.text(node))
            .is_some_and(|symbol| symbol.identity == identity)
    }

    fn unary_operator(&self, node: &SyntaxNode) -> Option<String> {
        node.children
            .iter()
            .find(|child| child.kind == SyntaxKind::UnaryOperator)
            .map(|operator| {
                self.text(operator)
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
    }

    fn callable_object_field(&self, receiver: &SyntaxNode, name: &str) -> bool {
        let Some(ValueType::Object(identity)) = self.receiver_value_type(receiver) else {
            return false;
        };
        self.unit
            .objects
            .iter()
            .find(|object| object.identity == identity)
            .is_some_and(|object| {
                !self
                    .package
                    .units
                    .iter()
                    .flat_map(|unit| &unit.functions)
                    .any(|method| method.owner.is_some() && method.name == name)
                    && effective_object_fields(self.unit, object)
                        .iter()
                        .any(|field| {
                            field.name == name
                                && matches!(field.value_type, ValueType::Function(_, _))
                        })
            })
    }

    fn wrapped_object_field(&self, receiver: &SyntaxNode, name: &str) -> bool {
        let Some(ValueType::Object(identity)) = self.receiver_value_type(receiver) else {
            return false;
        };
        let Some(object) = self
            .unit
            .objects
            .iter()
            .find(|object| object.identity == identity)
        else {
            return false;
        };
        !(object_descendants(self.unit, object).is_empty()
            || self.text(receiver) == "this" && self.current_object.is_some())
            && effective_object_fields(self.unit, object)
                .iter()
                .any(|field| field.name == name)
    }

    fn value_type_owns_resource(&self, value_type: &ValueType) -> bool {
        let ValueType::Object(identity) = value_type else {
            return false;
        };
        self.package
            .units
            .iter()
            .flat_map(|unit| &unit.objects)
            .any(|object| object.identity == *identity && object.resource_owning)
    }

    fn object_owns_resource(&self, identity: &ObjectIdentity) -> bool {
        self.package
            .units
            .iter()
            .flat_map(|unit| &unit.objects)
            .find(|object| object.identity == *identity)
            .is_some_and(|object| object.resource_owning)
    }

    fn object_requires_separation(&self, identity: &ObjectIdentity) -> bool {
        let Some(object) = self
            .unit
            .objects
            .iter()
            .find(|object| object.identity == *identity)
        else {
            return false;
        };
        effective_object_methods(self.unit, object)
            .iter()
            .any(|method| method.name == "destruct")
            || object_descendants(self.unit, object)
                .iter()
                .any(|descendant| {
                    effective_object_methods(self.unit, descendant)
                        .iter()
                        .any(|method| method.name == "destruct")
                })
            || (object.kind == ObjectKind::Interface
                && self.unit.objects.iter().any(|candidate| {
                    candidate.interfaces.contains(&object.identity)
                        && (effective_object_methods(self.unit, candidate)
                            .iter()
                            .any(|method| method.name == "destruct")
                            || object_descendants(self.unit, candidate)
                                .iter()
                                .any(|descendant| {
                                    effective_object_methods(self.unit, descendant)
                                        .iter()
                                        .any(|method| method.name == "destruct")
                                }))
                }))
    }

    fn reference_storage_expression(&mut self, operand: &SyntaxNode) -> String {
        if self.reference_backed_name(operand).is_some() {
            rust_name(self.text(operand))
        } else {
            self.expression(operand)
        }
    }

    fn reference_backed_name(&self, node: &SyntaxNode) -> Option<&TypedBinding> {
        if node.kind != SyntaxKind::Name {
            return None;
        }
        let span = self
            .package
            .resolve_name_at(self.unit, node.span.start, self.text(node))?
            .declaration_span?;
        self.unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span && self.reference_backed(binding))
    }

    fn reference_backed(&self, binding: &TypedBinding) -> bool {
        if matches!(
            binding.value_type,
            ValueType::Reference(_) | ValueType::SharedReference(_)
        ) {
            return false;
        }
        self.node_references_binding(&self.unit.tree.root, binding)
    }

    fn node_references_binding(&self, node: &SyntaxNode, binding: &TypedBinding) -> bool {
        if node.kind == SyntaxKind::UnaryExpression
            && matches!(
                self.unary_operator(node).as_deref(),
                Some("ref" | "shared ref")
            )
            && let Some(operand) = node.children.last()
            && operand.kind == SyntaxKind::Name
            && self
                .package
                .resolve_name_at(self.unit, operand.span.start, self.text(operand))
                .and_then(|symbol| symbol.declaration_span)
                == Some(binding.span)
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| self.node_references_binding(child, binding))
    }

    fn text(&self, node: &SyntaxNode) -> &str {
        &self.source.text()[node.span.start..node.span.end]
    }

    fn control_condition(&mut self, mut node: &SyntaxNode) -> String {
        while node.kind == SyntaxKind::GroupExpression
            && let [grouped] = node.children.as_slice()
        {
            node = grouped;
        }
        let expression = self.expression(node);
        if node.kind == SyntaxKind::BinaryExpression {
            Self::unwrapped_expression(expression)
        } else {
            expression
        }
    }

    fn line_start(&mut self) {
        for _ in 0..self.indent {
            self.output.push_str("    ");
        }
    }

    fn line(&mut self, text: &str) {
        self.line_start();
        self.output.push_str(text);
        self.output.push('\n');
    }
}

fn find_node(node: &SyntaxNode, kind: SyntaxKind, span: crate::Span) -> Option<&SyntaxNode> {
    if node.kind == kind && node.span == span {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_node(child, kind, span))
}

fn literal_or_text(source: &SourceFile, node: &SyntaxNode) -> String {
    let text = &source.text()[node.span.start..node.span.end];
    if node.kind == SyntaxKind::Literal {
        literal(text)
    } else {
        text.trim().to_owned()
    }
}

fn literal(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed == "true" || trimmed == "false" {
        return trimmed.to_owned();
    }
    if trimmed.starts_with("b'") && trimmed.ends_with('\'') {
        let value = crate::lexer::unescape_bytes(&trimmed[2..trimmed.len() - 1])
            .expect("lexer rejects malformed byte escapes before lowering");
        return format!("Vec::from({value:?})");
    }
    let compact = trimmed.replace('_', "");
    if let Some(value) = integer_literal(&compact) {
        return value.to_string();
    }
    if compact.parse::<f64>().is_ok() {
        return compact;
    }
    let value = if let Some(value) = trimmed.strip_prefix('>') {
        if let Some(block) = value.strip_prefix('>') {
            block_string(block)
        } else {
            value.to_owned()
        }
    } else if trimmed.len() >= 2
        && ((trimmed.starts_with('\'') && trimmed.ends_with('\''))
            || (trimmed.starts_with('"') && trimmed.ends_with('"')))
    {
        unescape(&trimmed[1..trimmed.len() - 1])
    } else {
        trimmed.to_owned()
    };
    format!("String::from({value:?})")
}

fn adaptive_literal(text: &str) -> String {
    let compact = text.trim().replace('_', "");
    let value = integer_literal(&compact)
        .expect("semantic analysis accepted a non-integer adaptive literal");
    let decimal = value.to_string();
    if decimal.parse::<i128>().is_ok() {
        format!("terrane_int_support::Int::from({decimal}_i128)")
    } else {
        format!("terrane_int_support::Int::from_decimal({decimal:?})")
    }
}

fn lower_contextual_constant(constant: ContextualConstant, destination: ScalarType) -> String {
    match constant {
        ContextualConstant::Integer(value) if destination == ScalarType::Int => {
            adaptive_literal(&value.to_string())
        }
        ContextualConstant::Integer(value) => value.to_string(),
        ContextualConstant::Float32(value) => float32_literal(value),
        ContextualConstant::Float64(value) => float64_literal(value),
    }
}

fn float32_literal(value: f32) -> String {
    if value.is_nan() {
        "f32::NAN".to_owned()
    } else if value == f32::INFINITY {
        "f32::INFINITY".to_owned()
    } else if value == f32::NEG_INFINITY {
        "f32::NEG_INFINITY".to_owned()
    } else {
        format!("{value:?}_f32")
    }
}

fn float64_literal(value: f64) -> String {
    if value.is_nan() {
        "f64::NAN".to_owned()
    } else if value == f64::INFINITY {
        "f64::INFINITY".to_owned()
    } else if value == f64::NEG_INFINITY {
        "f64::NEG_INFINITY".to_owned()
    } else {
        format!("{value:?}_f64")
    }
}

fn integer_literal(text: &str) -> Option<BigInt> {
    let (radix, digits) =
        if let Some(digits) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
            (16, digits)
        } else if let Some(digits) = text.strip_prefix("0o").or_else(|| text.strip_prefix("0O")) {
            (8, digits)
        } else if let Some(digits) = text.strip_prefix("0b").or_else(|| text.strip_prefix("0B")) {
            (2, digits)
        } else {
            (10, text)
        };
    BigInt::parse_bytes(digits.as_bytes(), radix)
}

fn block_string(text: &str) -> String {
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default();
    if !first.trim().is_empty() {
        return first.to_owned();
    }
    let collected = lines.collect::<Vec<_>>();
    let indent = collected
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    collected
        .iter()
        .map(|line| line.get(indent..).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}

fn unescape(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character == '\\' {
            match chars.next() {
                Some('n') => output.push('\n'),
                Some('r') => output.push('\r'),
                Some('t') => output.push('\t'),
                Some('\\') | None => output.push('\\'),
                Some('\'') => output.push('\''),
                Some('"') => output.push('"'),
                Some(other) => output.push(other),
            }
        } else {
            output.push(character);
        }
    }
    output
}

fn effective_object_fields<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a ObjectField> {
    fn collect<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        fields: &mut Vec<&'a ObjectField>,
    ) {
        if let Some(base) = object.base.as_ref().and_then(|identity| {
            unit.objects
                .iter()
                .find(|object| object.identity == *identity)
        }) {
            collect(unit, base, fields);
        }
        for reused in &object.traits {
            if let Some(reused) = unit
                .objects
                .iter()
                .find(|candidate| candidate.identity == *reused)
            {
                collect(unit, reused, fields);
            }
        }
        for field in &object.fields {
            if let Some(index) = fields
                .iter()
                .position(|existing| existing.name == field.name)
            {
                fields[index] = field;
            } else {
                fields.push(field);
            }
        }
    }
    let mut fields = Vec::new();
    collect(unit, object, &mut fields);
    fields
}

fn object_descendants<'a>(
    unit: &'a SemanticUnit,
    object: &ObjectContract,
) -> Vec<&'a ObjectContract> {
    unit.objects
        .iter()
        .filter(|candidate| {
            let mut base = candidate.base.as_ref();
            while let Some(identity) = base {
                if identity == &object.identity {
                    return true;
                }
                base = unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.identity == *identity)
                    .and_then(|candidate| candidate.base.as_ref());
            }
            false
        })
        .collect()
}

fn effective_object_interfaces<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a ObjectIdentity> {
    let mut interfaces = object
        .base
        .as_ref()
        .and_then(|identity| {
            unit.objects
                .iter()
                .find(|candidate| candidate.identity == *identity)
        })
        .map_or_else(Vec::new, |base| effective_object_interfaces(unit, base));
    for interface in &object.interfaces {
        if !interfaces.contains(&interface) {
            interfaces.push(interface);
        }
    }
    interfaces
}

fn object_destructor_chain<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a FunctionContract> {
    let mut destructors = object
        .base
        .as_ref()
        .and_then(|identity| {
            unit.objects
                .iter()
                .find(|candidate| candidate.identity == *identity)
        })
        .map_or_else(Vec::new, |base| object_destructor_chain(unit, base));
    if let Some(destructor) = unit.functions.iter().find(|method| {
        method.owner.as_deref() == Some(object.name.as_str()) && method.name == "destruct"
    }) {
        destructors.push(destructor);
    }
    destructors
}

fn effective_object_methods<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a FunctionContract> {
    fn collect<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        methods: &mut Vec<&'a FunctionContract>,
    ) {
        if let Some(base) = object.base.as_ref().and_then(|identity| {
            unit.objects
                .iter()
                .find(|object| object.identity == *identity)
        }) {
            collect(unit, base, methods);
        }
        for reused in &object.traits {
            if let Some(reused) = unit
                .objects
                .iter()
                .find(|candidate| candidate.identity == *reused)
            {
                collect(unit, reused, methods);
            }
        }
        for method in unit
            .functions
            .iter()
            .filter(|method| method.owner.as_deref() == Some(object.identity.name.as_str()))
        {
            if let Some(index) = methods
                .iter()
                .position(|existing| existing.name == method.name)
            {
                methods[index] = method;
            } else {
                methods.push(method);
            }
        }
    }
    let mut methods = Vec::new();
    collect(unit, object, &mut methods);
    methods
}

fn union_type_name(binding: &TypedBinding) -> String {
    format!("TerraneUnionF{}S{}", binding.span.file, binding.span.start)
}
fn find_node_by_span(node: &SyntaxNode, span: crate::Span) -> Option<&SyntaxNode> {
    (node.span == span).then_some(node).or_else(|| {
        node.children
            .iter()
            .find_map(|child| find_node_by_span(child, span))
    })
}

fn binding_initializer(node: &SyntaxNode, name_index: usize) -> Option<&SyntaxNode> {
    node.children
        .iter()
        .enumerate()
        .rev()
        .find(|(index, child)| {
            *index != name_index
                && !matches!(
                    child.kind,
                    SyntaxKind::TypeExpression
                        | SyntaxKind::Visibility
                        | SyntaxKind::DeclarationQualifier
                )
        })
        .map(|(_, child)| child)
}

fn rust_type(ty: ScalarType) -> &'static str {
    ty.lowering_type()
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "element lowering owns the recursively described value type"
)]
fn rust_element_type(package: &SemanticPackage, ty: ElementType) -> String {
    rust_value_type(package, ty.value_type())
}

#[expect(
    clippy::too_many_lines,
    reason = "the closed semantic value-type enum has one exhaustive Rust representation mapping"
)]
fn rust_value_type(package: &SemanticPackage, ty: ValueType) -> String {
    match ty {
        ValueType::Scalar(scalar) => rust_type(scalar).to_owned(),
        ValueType::Optional(inner) => {
            format!("Option<{}>", rust_value_type(package, *inner))
        }
        ValueType::OverflowResult(scalar) => {
            format!("terrane_int_support::OverflowResult<{}>", rust_type(scalar))
        }
        ValueType::DivRemResult(scalar) => {
            format!("terrane_int_support::DivRemResult<{}>", rust_type(scalar))
        }
        ValueType::StringView(crate::semantics::TextUnit::Bytes) => "Vec<u8>".to_owned(),
        ValueType::StringView(_) | ValueType::TextRangeView(_) => "String".to_owned(),
        ValueType::StringList => "Vec<String>".to_owned(),
        ValueType::Encoding => "terrane_string_support::Encoding".to_owned(),
        ValueType::TextRange => "terrane_string_support::TextRange".to_owned(),
        ValueType::Iterator(item) => {
            format!(
                "terrane_collection_support::Iterator<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::IterationStep(item) => {
            format!(
                "terrane_collection_support::IterationStep<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::List(item) => {
            format!(
                "terrane_collection_support::List<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::Map(key, value) => format!(
            "terrane_collection_support::Map<{}, {}>",
            rust_element_type(package, key),
            rust_element_type(package, value)
        ),
        ValueType::Set(item) => {
            format!(
                "terrane_collection_support::Set<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::Tuple(item, _) => {
            format!(
                "terrane_collection_support::Tuple<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::Range => "terrane_collection_support::Range".to_owned(),
        ValueType::Entry(key, value) => format!(
            "terrane_collection_support::Entry<{}, {}>",
            rust_element_type(package, key),
            rust_element_type(package, value)
        ),
        ValueType::UnorderedMap(key, value) => format!(
            "terrane_collection_support::UnorderedMap<{}, {}>",
            rust_element_type(package, key),
            rust_element_type(package, value)
        ),
        ValueType::UnorderedSet(item) => {
            format!(
                "terrane_collection_support::UnorderedSet<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::TextRangeList => "Vec<terrane_string_support::TextRange>".to_owned(),
        ValueType::Function(parameters, result) => format!(
            "std::sync::Arc<dyn Fn({}) -> {} + Send + Sync>",
            parameters
                .into_iter()
                .map(|parameter| rust_element_type(package, parameter))
                .collect::<Vec<_>>()
                .join(", "),
            rust_element_type(package, result)
        ),
        ValueType::AsyncFunction(parameters, result) => format!(
            "std::sync::Arc<dyn Fn({}) -> std::pin::Pin<Box<dyn Future<Output = {}>>> + Send + Sync>",
            parameters
                .into_iter()
                .map(|parameter| rust_element_type(package, parameter))
                .collect::<Vec<_>>()
                .join(", "),
            rust_element_type(package, result)
        ),
        ValueType::Task(result) => {
            format!(
                "std::pin::Pin<Box<dyn Future<Output = {}>>>",
                rust_element_type(package, result)
            )
        }
        ValueType::ScopedTask(result) => {
            format!("TerraneScopedTask<{}>", rust_element_type(package, result))
        }
        ValueType::TaskScope => "TerraneTaskScope".to_owned(),
        ValueType::TaskOutcome(result) => {
            format!("TerraneTaskOutcome<{}>", rust_element_type(package, result))
        }
        ValueType::PlatformStreamHandle => "TerranePlatformStreamHandle".to_owned(),
        ValueType::FilesystemAuthority => "TerraneFilesystemAuthority".to_owned(),
        ValueType::PlatformFilesystemResult => "TerraneFilesystemResult".to_owned(),
        ValueType::PlatformOpenResult => "TerranePlatformOpenResult".to_owned(),
        ValueType::PlatformReadResult => "TerranePlatformReadResult".to_owned(),
        ValueType::PlatformWriteResult => "TerranePlatformWriteResult".to_owned(),
        ValueType::PlatformUnitResult => "TerranePlatformUnitResult".to_owned(),
        ValueType::PlatformDataResult => "terrane_document_support::DataResult".to_owned(),
        ValueType::PlatformUrlResult => "terrane_document_support::UrlResult".to_owned(),
        ValueType::Descriptor(_) => "TerraneDescriptor".to_owned(),
        ValueType::PlatformCapability | ValueType::PlatformResourceHandle => {
            "TerranePlatformCapability".to_owned()
        }
        ValueType::PlatformResult => "TerranePlatformResult".to_owned(),
        ValueType::Object(identity) => rust_object_type_name(package, &identity),
        ValueType::SharedReference(item) => format!(
            "std::sync::Arc<std::sync::Mutex<{}>>",
            rust_element_type(package, item)
        ),
        ValueType::Reference(item) => format!(
            "std::sync::Weak<std::sync::Mutex<{}>>",
            rust_element_type(package, item)
        ),
    }
}

const fn is_numeric(ty: ScalarType) -> bool {
    ty.is_integer() || matches!(ty, ScalarType::Float32 | ScalarType::Float64)
}

fn integer_range_contains(destination: ScalarType, source: ScalarType) -> bool {
    let Some((destination_signed, destination_bits)) = fixed_integer_shape(destination) else {
        return false;
    };
    let Some((source_signed, source_bits)) = fixed_integer_shape(source) else {
        return false;
    };
    match (destination_signed, source_signed) {
        (true, true) | (false, false) => destination_bits >= source_bits,
        (true, false) => destination_bits > source_bits,
        (false, true) => false,
    }
}

fn exact_integer_float_widening(source: ScalarType, destination: ScalarType) -> bool {
    let Some((_, bits)) = fixed_integer_shape(source) else {
        return false;
    };
    match destination {
        ScalarType::Float32 => bits <= 16,
        ScalarType::Float64 => bits <= 32,
        _ => false,
    }
}

const fn fixed_integer_shape(ty: ScalarType) -> Option<(bool, u16)> {
    match ty {
        ScalarType::Int8 => Some((true, 8)),
        ScalarType::Int16 => Some((true, 16)),
        ScalarType::Int32 => Some((true, 32)),
        ScalarType::Int64 => Some((true, 64)),
        ScalarType::Int128 => Some((true, 128)),
        ScalarType::Uint8 => Some((false, 8)),
        ScalarType::Uint16 => Some((false, 16)),
        ScalarType::Uint32 => Some((false, 32)),
        ScalarType::Uint64 => Some((false, 64)),
        ScalarType::Uint128 => Some((false, 128)),
        _ => None,
    }
}

fn block_may_fall_through(block: &SyntaxNode) -> bool {
    block.children.last().is_none_or(statement_may_fall_through)
}

fn statement_may_fall_through(statement: &SyntaxNode) -> bool {
    match statement.kind {
        SyntaxKind::ReturnStatement
        | SyntaxKind::ThrowStatement
        | SyntaxKind::BreakStatement
        | SyntaxKind::ContinueStatement => false,
        SyntaxKind::IfStatement => {
            let mut branches = statement.children.iter().skip(1);
            let Some(first) = branches.next() else {
                return true;
            };
            let first_falls_through = first
                .children
                .last()
                .filter(|child| child.kind == SyntaxKind::Block)
                .map_or_else(|| block_may_fall_through(first), block_may_fall_through);
            let mut has_else = false;
            let mut any_falls_through = first_falls_through;
            for branch in branches {
                has_else |= branch.kind == SyntaxKind::ElseClause;
                any_falls_through |= branch
                    .children
                    .last()
                    .filter(|child| child.kind == SyntaxKind::Block)
                    .is_none_or(block_may_fall_through);
            }
            !has_else || any_falls_through
        }
        SyntaxKind::TryStatement => {
            let try_falls_through = statement
                .children
                .first()
                .is_none_or(block_may_fall_through);
            let catch_falls_through = statement
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
                .filter_map(|clause| {
                    clause
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block)
                })
                .any(block_may_fall_through);
            let finally_returns = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::FinallyClause)
                .and_then(|clause| clause.children.first())
                .is_some_and(|block| !block_may_fall_through(block));
            !finally_returns && (try_falls_through || catch_falls_through)
        }
        _ => true,
    }
}

fn rust_builtin_error_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "arithmetic-overflow" => Some("ArithmeticOverflow"),
        "division-by-zero" => Some("DivisionByZero"),
        "integer-conversion-overflow" => Some("IntegerConversionOverflow"),
        "negative-shift-count" => Some("NegativeShiftCount"),
        "coercion-error" => Some("CoercionError"),
        "decode-error" => Some("DecodeError"),
        "index-error" => Some("IndexError"),
        "missing-key" => Some("MissingKey"),
        "resource-error" => Some("ResourceError"),
        "error" | "throwable" => Some("SourceError"),
        _ => None,
    }
}

fn function_name(contract: &FunctionContract) -> String {
    if contract.name == "main" {
        "main".to_owned()
    } else {
        rust_name(&contract.name)
    }
}

fn namespace_binding_name(file: u32, name: &str) -> String {
    format!("__TERRANE_F{file}_{}", rust_name(name).to_uppercase())
}

fn global_binding_name(name: &str) -> String {
    format!("__TERRANE_GLOBAL_{}", rust_name(name).to_uppercase())
}

fn rust_object_name(name: &str) -> String {
    let mut uppercase = true;
    name.chars()
        .filter_map(|character| {
            if character == '-' {
                uppercase = true;
                None
            } else if uppercase {
                uppercase = false;
                Some(character.to_ascii_uppercase())
            } else {
                Some(character)
            }
        })
        .collect()
}

/// Qualifies colliding names with source-byte-length-prefixed CamelCase namespace segments.
///
/// Counting the source segment keeps the encoding injective when case conversion erases spelling
/// differences; the following CamelCase letter also makes adjacent decimal lengths unambiguous.
fn rust_object_type_name(package: &SemanticPackage, identity: &ObjectIdentity) -> String {
    let collides = package
        .units
        .iter()
        .flat_map(|unit| &unit.objects)
        .filter(|object| object.identity.name == identity.name)
        .map(|object| &object.identity)
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        > 1;
    if !collides {
        return rust_object_name(&identity.name);
    }
    let mut namespace = String::new();
    for segment in identity.namespace.trim_start_matches('/').split('/') {
        write!(namespace, "{}{}", segment.len(), rust_object_name(segment))
            .expect("writing to a string cannot fail");
    }
    format!("TerraneNs{namespace}{}", rust_object_name(&identity.name))
}

fn rust_name(name: &str) -> String {
    let readable_identifier = name.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_');
    let keyword = matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    );
    if readable_identifier && !keyword {
        return name.replace('-', "_");
    }
    let mut output = String::from("__trn_");
    for byte in name.bytes() {
        write!(output, "{byte:02x}").expect("writing to a String cannot fail");
    }
    output
}

fn display_path(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("<memory>")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{LoweringRegistry, emit_error_support};
    use crate::{SourceFile, Span};

    #[test]
    fn error_support_is_canonical_rust() {
        for (has_custom_throwable, has_dependency) in [(false, false), (true, false), (true, true)]
        {
            let mut emitted = String::new();
            let registry = LoweringRegistry::default();
            if has_dependency {
                registry.register_descriptor("/core/errors::dependency-error", "dependency-error");
                registry.register_descriptor("/core/errors::dependency-panic", "dependency-panic");
            }
            emit_error_support(
                &mut emitted,
                has_custom_throwable,
                has_dependency,
                &registry,
            );
            let canonical = crate::rust_ir::canonicalize_rust(&emitted).unwrap();
            assert_eq!(
                crate::rust_ir::canonicalize_rust(&canonical).unwrap(),
                canonical
            );
        }
    }

    #[test]
    fn lowering_registry_reuses_identical_semantic_sites() {
        let registry = LoweringRegistry::default();
        let source = SourceFile::new(0, PathBuf::from("case.trn"), "value".to_owned());
        let span = Span::new(0, 0, 5);

        let first = registry.register_site("case.trn", "/demo::main", &source, span);
        let second = registry.register_site("case.trn", "/demo::main", &source, span);

        assert_eq!(first, second);
        assert_eq!(registry.sites.borrow().len(), 1);
    }
}
