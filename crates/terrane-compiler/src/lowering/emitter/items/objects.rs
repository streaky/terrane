use std::{collections::BTreeSet, fmt::Write as _};

use crate::{
    ScalarType,
    lowering::{
        dependencies::{projected_callback_input_expression, projected_callback_output_expression},
        emitter::Emitter,
        helpers::{
            effective_object_methods, find_node, find_node_by_span, function_name,
            object_descendants, object_destructor_chain, rust_empty_collection,
            rust_generic_parameters, rust_name, rust_object_type_name,
            rust_source_type_application, rust_static_field_name, rust_type_parameter_name,
            rust_value_type,
        },
        required_init::constructor_proves_fields,
        runtime_support::package_uses_typed_documents,
    },
    semantics::{
        CanonicalDefault, DescriptorContract, EffectiveObjectField, FunctionContract,
        GenericParameterContract, InvocationMode, ObjectKind, SemanticPackage, ValueType,
        canonical_default, effective_object_fields,
    },
    syntax::{SyntaxKind, SyntaxNode},
};

fn canonical_field_default(package: &SemanticPackage, value_type: &ValueType) -> Option<String> {
    match canonical_default(value_type)? {
        CanonicalDefault::BoolFalse => Some("false".to_owned()),
        CanonicalDefault::AdaptiveIntegerZero => {
            Some("terrane_int_support::Int::from(0_i128)".to_owned())
        }
        CanonicalDefault::FixedIntegerZero => Some("0".to_owned()),
        CanonicalDefault::Float32Zero => Some("0.0_f32".to_owned()),
        CanonicalDefault::Float64Zero => Some("0.0_f64".to_owned()),
        CanonicalDefault::EmptyString => Some("String::new()".to_owned()),
        CanonicalDefault::EmptyBytes => Some("Vec::new()".to_owned()),
        CanonicalDefault::AbsentOptional => Some("None".to_owned()),
        CanonicalDefault::EmptyList
        | CanonicalDefault::EmptyMap
        | CanonicalDefault::EmptySet
        | CanonicalDefault::EmptyUnorderedMap
        | CanonicalDefault::EmptyUnorderedSet => rust_empty_collection(package, value_type),
    }
}

fn is_source_type_parameter(
    value_type: &ValueType,
    parameters: &[GenericParameterContract],
) -> bool {
    match value_type {
        ValueType::TypeParameter(name) => {
            parameters.iter().any(|parameter| parameter.name == *name)
        }
        ValueType::Optional(inner) => is_source_type_parameter(inner, parameters),
        _ => false,
    }
}

fn forwarded_method_return_type(
    package: &SemanticPackage,
    method: &FunctionContract,
) -> Option<String> {
    if method.throws {
        let result = method.return_type.clone().map_or_else(
            || "()".to_owned(),
            |value_type| rust_value_type(package, value_type),
        );
        Some(format!("Result<{result}, TerraneError>"))
    } else {
        method
            .return_type
            .clone()
            .filter(|result| *result != ValueType::Scalar(ScalarType::None))
            .map(|result| rust_value_type(package, result))
    }
}

impl<'a> Emitter<'a> {
    #[expect(
        clippy::too_many_lines,
        reason = "one decoder emission pass keeps field ordering and diagnostic accumulation auditable"
    )]
    fn object_document_decoder(
        &mut self,
        object: &DescriptorContract,
        class_type: &str,
        fields: &[EffectiveObjectField<'_>],
    ) {
        let (class_line, class_column) = self.source.line_column(object.span.start);
        let class_source = format!("{}:{class_line}:{class_column}", self.unit.source_path);
        let declared_fields = fields
            .iter()
            .map(|field| format!("{:?}", field.metadata.external_name))
            .collect::<Vec<_>>()
            .join(", ");
        self.line(&format!("impl TerraneDocumentDecode for {class_type} {{"));
        self.indent += 1;
        self.line("fn terrane_decode_document(");
        self.indent += 1;
        self.line("input: &terrane_document_support::DataResult,");
        self.line("path: &str,");
        self.line("allow_unknown: bool,");
        self.line("source: &str,");
        self.line("_field_source: &str,");
        self.indent -= 1;
        self.line(") -> Result<Self, Vec<TerraneDocumentDiagnostic>> {");
        self.indent += 1;
        self.line("if input.failed {");
        self.indent += 1;
        self.line(&format!(
            "return Err(vec![__terrane_document_diagnostic(path, {:?}, \"invalid\", \"parse\", input.message.clone(), source, {:?})]);",
            object.name, class_source
        ));
        self.indent -= 1;
        self.line("}");
        self.line("if terrane_document_support::document_kind(input) != \"map\" {");
        self.indent += 1;
        self.line(&format!(
            "return __terrane_document_type_error(input, path, {:?}, source, {:?});",
            object.name, class_source
        ));
        self.indent -= 1;
        self.line("}");
        self.line("let mut value = Self::terrane_construct();");
        self.line("let mut diagnostics = Vec::new();");
        self.line(&format!(
            "let declared_fields: &[&str] = &[{declared_fields}];"
        ));
        self.line("if !allow_unknown {");
        self.indent += 1;
        self.line("for index in 0..terrane_document_support::document_length(input) {");
        self.indent += 1;
        self.line("let key = terrane_document_support::document_key(input, index);");
        self.line("if !declared_fields.contains(&key.as_str()) {");
        self.indent += 1;
        self.line(&format!(
            "diagnostics.push(__terrane_document_diagnostic(__terrane_document_child_path(path, &key), {:?}, \"present\", \"unknown-field\", format!(\"unknown field `{{key}}`\"), source, {:?}));",
            object.name, class_source
        ));
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
        for field in fields {
            let rust_field = rust_name(&field.name);
            let rust_type = rust_value_type(self.package, field.value_type.clone());
            let external = &field.metadata.external_name;
            let (field_line, field_column) = field.unit.source.line_column(field.span.start);
            let field_source = format!("{}:{field_line}:{field_column}", field.unit.source_path);
            self.line("{");
            self.indent += 1;
            self.line(&format!(
                "let field = terrane_document_support::document_field(input, {external:?});"
            ));
            self.line(&format!(
                "let field_path = __terrane_document_child_path(path, {external:?});"
            ));
            self.line("if field.failed {");
            self.indent += 1;
            if field.metadata.defaulted || field.metadata.optional {
                self.line("// The field initializer supplies the absent default.");
            } else {
                self.line(&format!(
                    "diagnostics.push(__terrane_document_diagnostic(&field_path, {:?}, \"missing\", \"missing-required\", \"required field is missing\", source, {:?}));",
                    field.value_type.to_string(), field_source
                ));
            }
            self.indent -= 1;
            self.line("} else {");
            self.indent += 1;
            if let ValueType::Tuple(_, Some(length)) = field.value_type {
                self.line(&format!(
                    "if terrane_document_support::document_kind(&field) == \"list\" && terrane_document_support::document_length(&field) != {length} {{"
                ));
                self.indent += 1;
                self.line(&format!(
                    "diagnostics.push(__terrane_document_diagnostic(&field_path, {:?}, \"list\", \"tuple-length\", {:?}, source, {:?}));",
                    field.value_type.to_string(),
                    format!("expected exactly {length} tuple items"),
                    field_source
                ));
                self.indent -= 1;
                self.line("} else {");
                self.indent += 1;
            }
            self.line(&format!(
                "match <{rust_type} as TerraneDocumentDecode>::terrane_decode_document(&field, &field_path, allow_unknown, source, {field_source:?}) {{"
            ));
            self.indent += 1;
            self.line(&format!("Ok(decoded) => value.{rust_field} = decoded,"));
            self.line("Err(mut field_diagnostics) => diagnostics.append(&mut field_diagnostics),");
            self.indent -= 1;
            self.line("}");
            if matches!(field.value_type, ValueType::Tuple(_, Some(_))) {
                self.indent -= 1;
                self.line("}");
            }
            self.indent -= 1;
            self.line("}");
            self.indent -= 1;
            self.line("}");
        }
        if object.interfaces.iter().any(|interface| {
            interface.namespace == "/core/documents" && interface.name == "document-validatable"
        }) {
            self.line("if diagnostics.is_empty() {");
            self.indent += 1;
            self.line("if let Some(message) = value.validate_document() {");
            self.indent += 1;
            self.line(&format!(
                "diagnostics.push(__terrane_document_diagnostic(path, {:?}, \"map\", \"validation\", message, source, {:?}));",
                object.name, class_source
            ));
            self.indent -= 1;
            self.line("}");
            self.indent -= 1;
            self.line("}");
        }
        self.line("if diagnostics.is_empty() {");
        self.indent += 1;
        self.line("Ok(value)");
        self.indent -= 1;
        self.line("} else {");
        self.indent += 1;
        self.line("Err(diagnostics)");
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
    }

    #[expect(
        clippy::too_many_lines,
        reason = "object lowering emits one complete, ordered Rust object contract"
    )]
    pub(in crate::lowering::emitter) fn object(&mut self, node: &SyntaxNode) {
        let object = self
            .unit
            .descriptors
            .iter()
            .find(|object| object.span == node.span)
            .expect("analyzed object declaration must have a semantic contract");
        match object.kind {
            ObjectKind::Interface => {
                let name = rust_object_type_name(self.package, &object.identity);
                let protocol = format!("{name}Protocol");
                let methods = effective_object_methods(self.unit, object);
                let projected_requirements = self
                    .package
                    .projection
                    .item(&object.identity.namespace, &object.identity.name)
                    .and_then(|item| match &item.kind {
                        crate::rust_interop::projection::ProjectedKind::Interface(interface) => {
                            Some(interface)
                        }
                        _ => None,
                    });
                let associated_bounds = projected_requirements
                    .and_then(|interface| interface.associated_type.as_ref())
                    .map(|associated| {
                        let bounds = associated.bounds.join(" + ");
                        if bounds.is_empty() {
                            "'static".to_owned()
                        } else {
                            format!("'static + {bounds}")
                        }
                    });
                let (source_declaration, source_use) =
                    rust_generic_parameters(self.package, &object.generic_parameters);
                let associated_declaration = associated_bounds
                    .as_ref()
                    .map_or_else(String::new, |bounds| format!("TerraneAssociated: {bounds}"));
                let source_declaration = source_declaration
                    .trim_start_matches('<')
                    .trim_end_matches('>')
                    .to_owned();
                let source_use = source_use
                    .trim_start_matches('<')
                    .trim_end_matches('>')
                    .to_owned();
                let generic_declaration = match (
                    associated_declaration.is_empty(),
                    source_declaration.is_empty(),
                ) {
                    (true, true) => String::new(),
                    (true, false) => format!("<{source_declaration}>"),
                    (false, true) => format!("<{associated_declaration}>"),
                    (false, false) => format!("<{associated_declaration}, {source_declaration}>"),
                };
                let generic_use = match (associated_bounds.is_none(), source_use.is_empty()) {
                    (true, true) => String::new(),
                    (true, false) => format!("<{source_use}>"),
                    (false, true) => "<TerraneAssociated>".to_owned(),
                    (false, false) => format!("<TerraneAssociated, {source_use}>"),
                };
                let protocol_use = format!("{protocol}{generic_use}");
                let transfer_bounds = if projected_requirements
                    .is_none_or(|item| item.send && item.sync)
                    || object.identity.namespace == "/core/logging"
                        && object.identity.name == "log-value"
                {
                    " : Send + Sync"
                } else if projected_requirements.is_some_and(|item| item.send) {
                    " : Send"
                } else if projected_requirements.is_some_and(|item| item.sync) {
                    " : Sync"
                } else {
                    ""
                };
                let has_unsafe_methods = methods.iter().any(|method| method.is_unsafe);
                if object.is_unsafe || has_unsafe_methods {
                    self.line("#[allow(unsafe_code)]");
                }
                self.line(&format!(
                    "pub {}trait {protocol}{generic_declaration}{transfer_bounds} {{",
                    if object.is_unsafe { "unsafe " } else { "" }
                ));
                self.indent += 1;
                self.line(&format!("fn clone_box(&self) -> Box<dyn {protocol_use}>;"));
                self.line(&format!(
                    "fn separate_box(&self) -> Box<dyn {protocol_use}>;"
                ));
                for method in &methods {
                    self.line_start();
                    let receiver = match method.written_invocation_mode {
                        InvocationMode::Consuming => "self: Box<Self>",
                        InvocationMode::Mutable => "&mut self",
                        InvocationMode::Shared => "&self",
                    };
                    let (method_generics, _) =
                        rust_generic_parameters(self.package, &method.generic_parameters);
                    write!(
                        self.output,
                        "{}fn {}{method_generics}({receiver}",
                        if method.is_unsafe { "unsafe " } else { "" },
                        function_name(self.package, method)
                    )
                    .unwrap();
                    for parameter in &method.parameters {
                        let ty = parameter.binding_value_type().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        let name = self.parameter_source_name(&parameter.name, parameter.span);
                        write!(self.output, ", {name}: {ty}").unwrap();
                    }
                    self.output.push(')');
                    let result = forwarded_method_return_type(self.package, method);
                    if method.is_async {
                        write!(
                            self.output,
                            " -> std::pin::Pin<Box<dyn std::future::Future<Output = {}> + Send + {}>>",
                            result.as_deref().unwrap_or("()"),
                            if method.written_invocation_mode == InvocationMode::Consuming {
                                "'static"
                            } else {
                                "'_"
                            }
                        )
                        .unwrap();
                    } else if let Some(result) = result {
                        write!(self.output, " -> {result}").unwrap();
                    }
                    self.output.push_str(";\n");
                }
                self.indent -= 1;
                self.line("}");
                self.line(&format!(
                    "impl{generic_declaration} Clone for Box<dyn {protocol_use}> {{ fn clone(&self) -> Self {{ self.clone_box() }} }}"
                ));
                if methods.is_empty() {
                    self.line(
                        "#[allow(dead_code, reason = \"marker interface storage is materialized only when a value is erased to that marker\")]",
                    );
                }
                self.line(&format!(
                    "pub struct {name}{generic_declaration}(Box<dyn {protocol_use}>);"
                ));
                self.line(&format!(
                    "impl{generic_declaration} Clone for {name}{generic_use} {{ fn clone(&self) -> Self {{ Self(self.0.clone()) }} }}"
                ));
                if projected_requirements.is_some_and(|item| item.requires_drop) {
                    self.line(&format!(
                        "impl{generic_declaration} Drop for {name}{generic_use} {{ fn drop(&mut self) {{}} }}"
                    ));
                }
                if has_unsafe_methods {
                    self.line("#[allow(unsafe_code)]");
                }
                self.line(&format!("impl{generic_declaration} {name}{generic_use} {{"));
                self.indent += 1;
                for method in &methods {
                    self.line_start();
                    let receiver = match method.written_invocation_mode {
                        InvocationMode::Consuming => "self",
                        InvocationMode::Mutable => "&mut self",
                        InvocationMode::Shared => "&self",
                    };
                    write!(
                        self.output,
                        "pub {}{}fn {}({receiver}",
                        if method.is_async { "async " } else { "" },
                        if method.is_unsafe { "unsafe " } else { "" },
                        function_name(self.package, method)
                    )
                    .unwrap();
                    for parameter in &method.parameters {
                        let ty = parameter.binding_value_type().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        let name = self.parameter_source_name(&parameter.name, parameter.span);
                        write!(self.output, ", {name}: {ty}").unwrap();
                    }
                    self.output.push(')');
                    if let Some(result) = forwarded_method_return_type(self.package, method) {
                        write!(self.output, " -> {result}").unwrap();
                    }
                    self.output.push_str(" {\n");
                    self.indent += 1;
                    let arguments = method
                        .parameters
                        .iter()
                        .map(|parameter| {
                            self.parameter_source_name(&parameter.name, parameter.span)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    let forwarded = format!(
                        "self.0.{}({arguments}){}",
                        function_name(self.package, method),
                        if method.is_async { ".await" } else { "" }
                    );
                    self.line(&if method.is_unsafe {
                        format!("unsafe {{ {forwarded} }}")
                    } else {
                        forwarded
                    });
                    self.indent -= 1;
                    self.line("}");
                }
                if self.object_requires_separation(&object.identity) {
                    self.line("fn terrane_separate(&self) -> Self { Self(self.0.separate_box()) }");
                }
                self.indent -= 1;
                self.line("}");
                if let Some(projected_item) = self
                    .package
                    .projection
                    .item(&object.identity.namespace, &object.identity.name)
                    && let crate::rust_interop::projection::ProjectedKind::Interface(interface) =
                        &projected_item.kind
                {
                    if interface.associated_type.is_some() {
                        let applications = self
                            .unit
                            .descriptors
                            .iter()
                            .filter(|candidate| {
                                candidate.kind == ObjectKind::Interface
                                    && candidate.identity.base() == object.identity
                                    && candidate.identity.application.is_some()
                            })
                            .cloned()
                            .collect::<Vec<_>>();
                        for applied in applications {
                            self.projected_interface_implementation(&applied, &applied.identity);
                        }
                    } else {
                        self.projected_interface_implementation(object, &object.identity);
                    }
                }
            }
            ObjectKind::Enum => self.enum_declaration(node),
            ObjectKind::Trait => {}
            ObjectKind::Type => {
                unreachable!("compiler-owned descriptor templates have no source declaration")
            }
            ObjectKind::Class => {
                let fields = effective_object_fields(self.package, object);
                let instance_fields = fields
                    .iter()
                    .copied()
                    .filter(|field| !field.field.is_static)
                    .collect::<Vec<_>>();
                let static_fields = fields
                    .iter()
                    .copied()
                    .filter(|field| field.field.is_static)
                    .collect::<Vec<_>>();
                let phantom_parameters = object
                    .generic_parameters
                    .iter()
                    .filter(|parameter| {
                        !instance_fields.iter().any(|field| {
                            super::super::enums::value_type_uses_parameter(
                                &field.field.value_type,
                                &parameter.name,
                            )
                        })
                    })
                    .collect::<Vec<_>>();
                let phantom_type = (!phantom_parameters.is_empty()).then(|| {
                    format!(
                        "std::marker::PhantomData<fn({})>",
                        phantom_parameters
                            .iter()
                            .map(|parameter| rust_type_parameter_name(&parameter.name))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                });
                let descendants = object_descendants(self.unit, object);
                let generic_identity = object.identity.base();
                let class_name = rust_object_type_name(self.package, &generic_identity);
                let (generic_declaration, generic_use) =
                    rust_generic_parameters(self.package, &object.generic_parameters);
                let class_type = format!("{class_name}{generic_use}");
                let storage_name = if descendants.is_empty() {
                    class_name.clone()
                } else {
                    format!("{class_name}Storage")
                };
                let storage_type = format!("{storage_name}{generic_use}");
                let all_methods = effective_object_methods(self.unit, object);
                let methods = all_methods
                    .iter()
                    .copied()
                    .filter(|method| !method.is_static)
                    .collect::<Vec<_>>();
                let static_methods = all_methods
                    .iter()
                    .copied()
                    .filter(|method| method.is_static)
                    .collect::<Vec<_>>();
                let has_destructor = methods.iter().any(|method| method.name == "destruct");
                let has_required_fields = instance_fields.iter().any(|field| field.field.required);
                let tracks_construction = has_destructor && has_required_fields;
                let conditional_generic_clone = !has_destructor
                    && !object.generic_parameters.is_empty()
                    && instance_fields.iter().all(|field| {
                        !crate::semantics::application_is_resource_owning(
                            self.package,
                            &field.field.value_type,
                        ) || is_source_type_parameter(
                            &field.field.value_type,
                            &object.generic_parameters,
                        )
                    });
                let can_clone = !object.resource_owning || conditional_generic_clone;

                let previous_object = self.current_object.replace(object.identity.clone());
                for field in &static_fields {
                    let initializer = self.field_initial_value(*field);
                    self.line(&format!(
                        "pub static {}: std::sync::LazyLock<std::sync::Mutex<{}>> = std::sync::LazyLock::new(|| std::sync::Mutex::new({initializer}));",
                        rust_static_field_name(self.package, &object.identity, &field.field.name),
                        rust_value_type(self.package, field.field.value_type.clone())
                    ));
                }
                self.current_object = previous_object;
                if !static_fields.is_empty() {
                    self.output.push('\n');
                }

                if can_clone {
                    if !object.resource_owning
                        && (object.identity.namespace == "/core/time"
                            && matches!(
                                object.identity.name.as_str(),
                                "duration"
                                    | "duration-subtraction"
                                    | "instant"
                                    | "monotonic-instant"
                                    | "deadline"
                            )
                            || object.identity.namespace == "/core/process-signals"
                                && object.identity.name == "process-signal")
                    {
                        if object.identity.namespace == "/core/process-signals" {
                            self.line("#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]");
                        } else {
                            self.line("#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]");
                        }
                    } else {
                        self.line("#[derive(Clone)]");
                    }
                }
                self.line(&format!(
                    "pub struct {storage_name}{generic_declaration} {{"
                ));
                self.indent += 1;
                if tracks_construction {
                    self.line("__terrane_constructed: bool,");
                }
                if has_destructor && !object.resource_owning {
                    self.line("__terrane_lifetime: std::sync::Arc<()>,");
                }
                if let Some(phantom_type) = &phantom_type {
                    self.line(&format!("__terrane_phantom: {phantom_type},"));
                }
                for field in &instance_fields {
                    let field_type = rust_value_type(self.package, field.field.value_type.clone());
                    let field_type = if field.field.required {
                        format!("Option<{field_type}>")
                    } else {
                        field_type
                    };
                    self.line(&format!(
                        "pub {}: {field_type},",
                        rust_name(&field.field.name)
                    ));
                }
                self.indent -= 1;
                self.line("}");
                self.line(&format!("impl{generic_declaration} {storage_type} {{"));
                self.indent += 1;
                if let Some(construct) = methods.iter().find(|method| method.name == "construct") {
                    let required_fields = instance_fields
                        .iter()
                        .filter(|field| field.field.required)
                        .map(|field| field.field.name.as_str())
                        .collect::<BTreeSet<_>>();
                    if !required_fields.is_empty()
                        && !constructor_proves_fields(self.unit, construct, &required_fields)
                    {
                        unreachable!("semantic analysis must prove required field initialization");
                    }
                    self.output.push_str("pub fn terrane_construct(");
                    for (index, parameter) in construct.parameters.iter().enumerate() {
                        if index != 0 {
                            self.output.push_str(", ");
                        }
                        let ty = parameter.binding_value_type().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        let name = self.parameter_source_name(&parameter.name, parameter.span);
                        write!(self.output, "{name}: {ty}").unwrap();
                    }
                    if construct.throws {
                        self.output.push_str(") -> Result<Self, TerraneError> {\n");
                    } else {
                        self.output.push_str(") -> Self {\n");
                    }
                    self.indent += 1;
                    let parameter_names = construct
                        .parameters
                        .iter()
                        .map(|parameter| {
                            self.parameter_source_name(&parameter.name, parameter.span)
                        })
                        .collect::<BTreeSet<_>>();
                    let mut value_name = "__terrane_constructed_value".to_owned();
                    while parameter_names.contains(&value_name) {
                        value_name.push('_');
                    }
                    self.line(&format!("let mut {value_name} = Self {{"));
                    self.indent += 1;
                    if tracks_construction {
                        self.line("__terrane_constructed: false,");
                    }
                    if phantom_type.is_some() {
                        self.line("__terrane_phantom: std::marker::PhantomData,");
                    }
                    for field in &instance_fields {
                        let value = self.field_initial_value(*field);
                        self.line(&format!("{}: {value},", rust_name(&field.field.name)));
                    }
                    if has_destructor && !object.resource_owning {
                        self.line("__terrane_lifetime: std::sync::Arc::new(()),");
                    }
                    self.indent -= 1;
                    self.line("};");
                    let arguments = construct
                        .parameters
                        .iter()
                        .map(|parameter| {
                            self.parameter_source_name(&parameter.name, parameter.span)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.line(&format!(
                        "{value_name}.construct({arguments}){};",
                        if construct.throws { "?" } else { "" }
                    ));
                    if tracks_construction {
                        self.line(&format!("{value_name}.__terrane_constructed = true;"));
                    }
                    self.line(&if construct.throws {
                        format!("Ok({value_name})")
                    } else {
                        value_name
                    });
                    self.indent -= 1;
                    self.line("}");
                } else {
                    self.line("pub fn terrane_construct() -> Self {");
                    self.indent += 1;
                    self.line("Self {");
                    self.indent += 1;
                    if tracks_construction {
                        self.line("__terrane_constructed: true,");
                    }
                    if phantom_type.is_some() {
                        self.line("__terrane_phantom: std::marker::PhantomData,");
                    }
                    for field in &instance_fields {
                        let value = self.field_initial_value(*field);
                        self.line(&format!("{}: {value},", rust_name(&field.field.name)));
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
                if descendants.is_empty() {
                    for method in &static_methods {
                        let method_node = find_node(
                            &self.unit.tree.root,
                            SyntaxKind::FunctionDeclaration,
                            method.span,
                        )
                        .expect("static object method must have a syntax node");
                        self.object_static_method(method_node);
                    }
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
                if package_uses_typed_documents(self.package)
                    && object.interfaces.iter().any(|interface| {
                        interface.namespace == "/core/documents"
                            && interface.name == "document-decodable"
                    })
                    && object.base.is_none()
                    && descendants.is_empty()
                    && !object.resource_owning
                    && !methods.iter().any(|method| method.name == "construct")
                {
                    self.object_document_decoder(object, &class_type, &instance_fields);
                }
                if !descendants.is_empty() {
                    if can_clone {
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
                            let ty = parameter.binding_value_type().map_or_else(
                                || "i128".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            let name = self.parameter_source_name(&parameter.name, parameter.span);
                            write!(self.output, "{name}: {ty}").unwrap();
                        }
                        if construct.throws {
                            self.output.push_str(") -> Result<Self, TerraneError> {\n");
                        } else {
                            self.output.push_str(") -> Self {\n");
                        }
                        self.indent += 1;
                        let arguments = construct
                            .parameters
                            .iter()
                            .map(|parameter| {
                                self.parameter_source_name(&parameter.name, parameter.span)
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        if construct.throws {
                            self.line(&format!(
                                "Ok(Self::Own({storage_type}::terrane_construct({arguments})?))"
                            ));
                        } else {
                            self.line(&format!(
                                "Self::Own({storage_type}::terrane_construct({arguments}))"
                            ));
                        }
                        self.indent -= 1;
                        self.line("}");
                    } else {
                        self.line(&format!(
                            "pub fn terrane_construct() -> Self {{ Self::Own({storage_type}::terrane_construct()) }}"
                        ));
                    }
                    let previous_object = self.current_object.replace(object.identity.clone());
                    for method in &static_methods {
                        let method_node = find_node(
                            &self.unit.tree.root,
                            SyntaxKind::FunctionDeclaration,
                            method.span,
                        )
                        .expect("static object method must have a syntax node");
                        self.object_static_method(method_node);
                    }
                    self.current_object = previous_object;
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
                        if method.is_unsafe {
                            self.line("#[allow(unsafe_code)]");
                        }
                        self.line_start();
                        let receiver = match method.written_invocation_mode {
                            InvocationMode::Consuming => "self",
                            InvocationMode::Mutable => "&mut self",
                            InvocationMode::Shared => "&self",
                        };
                        write!(
                            self.output,
                            "pub {}fn {}({receiver}",
                            if method.is_unsafe { "unsafe " } else { "" },
                            function_name(self.package, method)
                        )
                        .unwrap();
                        for parameter in &method.parameters {
                            let ty = parameter.binding_value_type().map_or_else(
                                || "i128".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            let name = self.parameter_source_name(&parameter.name, parameter.span);
                            write!(self.output, ", {name}: {ty}").unwrap();
                        }
                        self.output.push(')');
                        if let Some(result) = forwarded_method_return_type(self.package, method) {
                            write!(self.output, " -> {result}").unwrap();
                        }
                        self.output.push_str(" {\n");
                        self.indent += 1;
                        self.line("match self {");
                        self.indent += 1;
                        let arguments = method
                            .parameters
                            .iter()
                            .map(|parameter| {
                                self.parameter_source_name(&parameter.name, parameter.span)
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        let mut receiver_binding = "value".to_owned();
                        while method.parameters.iter().any(|parameter| {
                            self.parameter_source_name(&parameter.name, parameter.span)
                                == receiver_binding
                        }) {
                            receiver_binding.push('_');
                        }
                        let method_name = function_name(self.package, method);
                        let forward = |target: String| {
                            if method.is_unsafe {
                                format!("unsafe {{ {target} }}")
                            } else {
                                target
                            }
                        };
                        self.line(&format!(
                            "Self::Own({receiver_binding}) => {},",
                            forward(format!("{receiver_binding}.{method_name}({arguments})"))
                        ));
                        for descendant in &descendants {
                            let descendant_type =
                                rust_object_type_name(self.package, &descendant.identity);
                            self.line(&format!(
                                "Self::{descendant_type}({receiver_binding}) => {},",
                                forward(format!("{receiver_binding}.{method_name}({arguments})"))
                            ));
                        }
                        self.indent -= 1;
                        self.line("}");
                        self.indent -= 1;
                        self.line("}");
                    }
                    for field in &instance_fields {
                        let field_name = rust_name(&field.name);
                        let field_type = rust_value_type(self.package, field.value_type.clone());
                        self.line(&format!(
                            "pub fn terrane_field_{field_name}(&self) -> &{field_type} {{"
                        ));
                        self.indent += 1;
                        self.line("match self {");
                        self.indent += 1;
                        let own_access = if field.field.required {
                            format!(
                                "value.{field_name}.as_ref().expect(\"required field initialized\")"
                            )
                        } else {
                            format!("&value.{field_name}")
                        };
                        self.line(&format!("Self::Own(value) => {own_access},"));
                        for descendant in &descendants {
                            let descendant_type =
                                rust_object_type_name(self.package, &descendant.identity);
                            if self.unit.descriptors.iter().any(|candidate| {
                                candidate.base.as_ref() == Some(&descendant.identity)
                            }) {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => value.terrane_field_{field_name}(),"
                                ));
                            } else if field.field.required {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => value.{field_name}.as_ref().expect(\"required field initialized\"),"
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
                        let own_mut_access = if field.field.required {
                            format!(
                                "value.{field_name}.as_mut().expect(\"required field initialized\")"
                            )
                        } else {
                            format!("&mut value.{field_name}")
                        };
                        self.line(&format!("Self::Own(value) => {own_mut_access},"));
                        for descendant in &descendants {
                            let descendant_type =
                                rust_object_type_name(self.package, &descendant.identity);
                            if self.unit.descriptors.iter().any(|candidate| {
                                candidate.base.as_ref() == Some(&descendant.identity)
                            }) {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => value.terrane_field_{field_name}_mut(),"
                                ));
                            } else if field.field.required {
                                self.line(&format!(
                                    "Self::{descendant_type}(value) => value.{field_name}.as_mut().expect(\"required field initialized\"),"
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
                        if field.field.required {
                            self.line(&format!(
                                "pub fn terrane_field_{field_name}_slot_mut(&mut self) -> &mut Option<{field_type}> {{"
                            ));
                            self.indent += 1;
                            self.line("match self {");
                            self.indent += 1;
                            self.line(&format!("Self::Own(value) => &mut value.{field_name},"));
                            for descendant in &descendants {
                                let descendant_type =
                                    rust_object_type_name(self.package, &descendant.identity);
                                if self.unit.descriptors.iter().any(|candidate| {
                                    candidate.base.as_ref() == Some(&descendant.identity)
                                }) {
                                    self.line(&format!(
                                        "Self::{descendant_type}(value) => value.terrane_field_{field_name}_slot_mut(),"
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
                    }
                    self.indent -= 1;
                    self.line("}");
                }
                for interface_identity in
                    crate::semantics::effective_object_interfaces(self.package, object)
                {
                    if interface_identity.namespace == "/core/errors"
                        && interface_identity.name == "throwable"
                    {
                        continue;
                    }
                    let projected_interface = self
                        .package
                        .projection
                        .item(&interface_identity.namespace, &interface_identity.name)
                        .is_some_and(|item| {
                            matches!(
                                item.kind,
                                crate::rust_interop::projection::ProjectedKind::Interface(_)
                            )
                        });
                    if object.resource_owning && projected_interface {
                        self.projected_interface_implementation(object, interface_identity);
                        continue;
                    }
                    let interface_unit = self
                        .package
                        .units
                        .iter()
                        .find(|candidate| candidate.namespace == interface_identity.namespace)
                        .expect("resolved interface namespace");
                    let interface = interface_unit
                        .descriptors
                        .iter()
                        .find(|candidate| candidate.identity.base() == interface_identity.base())
                        .expect("validated interface contract");
                    let interface_type =
                        rust_source_type_application(self.package, interface_identity);
                    let interface_base =
                        rust_object_type_name(self.package, &interface.identity.base());
                    let protocol = format!(
                        "{interface_base}Protocol{}",
                        if interface_identity.type_arguments.is_empty() {
                            interface_identity.application.as_deref().map_or_else(
                                String::new,
                                |application| {
                                    format!(
                                        "<{}>",
                                        rust_value_type(self.package, application.clone())
                                    )
                                },
                            )
                        } else {
                            format!(
                                "<{}>",
                                interface_identity
                                    .type_arguments
                                    .iter()
                                    .map(|argument| rust_value_type(self.package, argument.clone()))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        }
                    );
                    let class_type = rust_object_type_name(self.package, &object.identity);
                    if interface.is_unsafe
                        || effective_object_methods(interface_unit, interface)
                            .iter()
                            .any(|method| method.is_unsafe)
                    {
                        self.line("#[allow(unsafe_code)]");
                    }
                    self.line(&format!(
                        "{}impl {protocol} for {class_type} {{",
                        if interface.is_unsafe { "unsafe " } else { "" }
                    ));
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
                    let substitutions = interface
                        .generic_parameters
                        .iter()
                        .zip(&interface_identity.type_arguments)
                        .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
                        .collect();
                    let interface_methods = effective_object_methods(interface_unit, interface)
                        .into_iter()
                        .map(|method| {
                            let mut method = crate::semantics::bind_projected_requirement(
                                method,
                                interface_identity.application.as_deref(),
                            );
                            for parameter in &mut method.parameters {
                                parameter.value_type =
                                    parameter.value_type.as_ref().map(|value_type| {
                                        crate::semantics::substitute_value_type(
                                            value_type,
                                            &substitutions,
                                        )
                                    });
                            }
                            method.return_type = method.return_type.as_ref().map(|value_type| {
                                crate::semantics::substitute_value_type(value_type, &substitutions)
                            });
                            method
                        })
                        .collect::<Vec<_>>();
                    for method in &interface_methods {
                        let implementation = effective_object_methods(self.unit, object)
                            .into_iter()
                            .find(|candidate| {
                                candidate.name == method.name
                                    && !candidate.is_static
                                    && candidate.is_unsafe == method.is_unsafe
                            });
                        self.line_start();
                        let implementation_mode = implementation
                            .map_or(method.written_invocation_mode, |implementation| {
                                implementation.written_invocation_mode
                            });
                        let receiver = match (method.written_invocation_mode, implementation_mode) {
                            (InvocationMode::Consuming, InvocationMode::Mutable) => {
                                "mut self: Box<Self>"
                            }
                            (InvocationMode::Consuming, _) => "self: Box<Self>",
                            (InvocationMode::Mutable, _) => "&mut self",
                            (InvocationMode::Shared, _) => "&self",
                        };
                        write!(
                            self.output,
                            "{}fn {}({receiver}",
                            if method.is_unsafe { "unsafe " } else { "" },
                            function_name(self.package, method)
                        )
                        .unwrap();
                        for parameter in &method.parameters {
                            let ty = parameter.binding_value_type().map_or_else(
                                || "i128".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            let name = self.parameter_source_name(&parameter.name, parameter.span);
                            write!(self.output, ", {name}: {ty}").unwrap();
                        }
                        self.output.push(')');
                        let result = forwarded_method_return_type(self.package, method);
                        if method.is_async {
                            write!(
                                self.output,
                                " -> std::pin::Pin<Box<dyn std::future::Future<Output = {}> + Send + {}>>",
                                result.as_deref().unwrap_or("()"),
                                if method.written_invocation_mode == InvocationMode::Consuming {
                                    "'static"
                                } else {
                                    "'_"
                                }
                            )
                            .unwrap();
                        } else if let Some(result) = result {
                            write!(self.output, " -> {result}").unwrap();
                        }
                        self.output.push_str(" {\n");
                        self.indent += 1;
                        let arguments = method
                            .parameters
                            .iter()
                            .map(|parameter| {
                                self.parameter_source_name(&parameter.name, parameter.span)
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        if method.is_async {
                            self.line_start();
                            self.output.push_str("Box::pin(async move { ");
                        }
                        if let Some(implementation) = implementation {
                            let receiver = match implementation.written_invocation_mode {
                                InvocationMode::Shared => "&*self",
                                InvocationMode::Mutable => "&mut *self",
                                InvocationMode::Consuming => {
                                    debug_assert_eq!(
                                        method.written_invocation_mode,
                                        InvocationMode::Consuming
                                    );
                                    "*self"
                                }
                            };
                            let call = format!(
                                "{class_type}::{}({receiver}, {arguments})",
                                function_name(self.package, implementation),
                            );
                            let call = if implementation.is_unsafe {
                                format!("unsafe {{ {call} }}")
                            } else {
                                call
                            };
                            write!(
                                self.output,
                                "{call}{}",
                                if method.is_async { ".await" } else { "" }
                            )
                            .unwrap();
                        } else {
                            let projected_item = self
                                .package
                                .projection
                                .item(&interface.identity.namespace, &interface.identity.name)
                                .expect("projected default interface");
                            let projected_method = self
                                .package
                                .projection
                                .interface_method(
                                    &interface.identity.namespace,
                                    &interface.identity.name,
                                    &method.name,
                                )
                                .expect("projected default method");
                            let rust_arguments = projected_method
                                .function
                                .parameters
                                .iter()
                                .zip(&method.parameters)
                                .map(|(projected, parameter)| {
                                    let converted = projected_callback_output_expression(
                                        &self
                                            .parameter_source_name(&parameter.name, parameter.span),
                                        &projected.ty,
                                    );
                                    if projected.borrowed {
                                        format!(
                                            "&{}{converted}",
                                            if projected.mutable_borrow { "mut " } else { "" }
                                        )
                                    } else {
                                        converted
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(", ");
                            let receiver = match method.written_invocation_mode {
                                InvocationMode::Shared => "&*self",
                                InvocationMode::Mutable => "&mut *self",
                                InvocationMode::Consuming => "*self",
                            };
                            let owner_rust_path = projected_method
                                .owner_rust_path
                                .as_deref()
                                .unwrap_or(&projected_item.rust_path);
                            let call = format!(
                                "<{class_type} as {owner_rust_path}>::{}({receiver}, {rust_arguments})",
                                rust_name(&method.name)
                            );
                            let call = if projected_method.function.is_unsafe {
                                format!("unsafe {{ {call} }}")
                            } else {
                                call
                            };
                            let call = if method.is_async {
                                format!("{call}.await")
                            } else {
                                call
                            };
                            let call = if projected_method.function.error.is_some() {
                                format!(
                                    "match {call} {{ Ok(value) => value, Err(error) => return Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))) }}",
                                    projected_item.rust_path, projected_method.function.name
                                )
                            } else {
                                call
                            };
                            let converted = projected_callback_input_expression(
                                "__terrane_default",
                                &projected_method.function.result,
                                &projected_method.function.result.rust_type(),
                            );
                            let result_type = method.return_type.clone().map_or_else(
                                || "()".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            let boundary = format!(
                                "(|| -> Result<{result_type}, crate::TerraneForeignError> {{ let __terrane_default = {call}; Ok({converted}) }})()"
                            );
                            if method.throws {
                                write!(
                                    self.output,
                                    "{boundary}.map_err(|error| error.raised(crate::TERRANE_NO_SITE))"
                                )
                                .expect("writing to a string cannot fail");
                            } else {
                                write!(
                                    self.output,
                                    "{boundary}.unwrap_or_else(|error| panic!(\"{{}}\", error.render()))"
                                )
                                .expect("writing to a string cannot fail");
                            }
                        }
                        if method.is_async {
                            self.output.push_str(" })");
                        }
                        self.output.push('\n');
                        self.indent -= 1;
                        self.line("}");
                    }
                    self.indent -= 1;
                    self.line("}");
                    self.line(&format!(
                        "impl From<{class_type}> for {interface_type} {{ fn from(value: {class_type}) -> Self {{ Self(Box::new(value)) }} }}"
                    ));
                    self.projected_interface_implementation(object, interface_identity);
                }
                if has_destructor {
                    let destructors = object_destructor_chain(self.unit, object);
                    let destructor_calls = std::iter::once(destructors.last().map_or_else(
                        String::new,
                        |destructor| {
                            format!(
                                "self.destruct(){}",
                                if destructor.is_async { ".await" } else { "" }
                            )
                        },
                    ))
                    .chain(
                        destructors
                            .iter()
                            .take(destructors.len().saturating_sub(1))
                            .enumerate()
                            .rev()
                            .map(|(index, destructor)| {
                                format!(
                                    "self.terrane_destruct_{index}(){}",
                                    if destructor.is_async { ".await" } else { "" }
                                )
                            }),
                    )
                    .collect::<Vec<_>>();
                    let awaits_destruction =
                        destructors.iter().any(|destructor| destructor.is_async);
                    self.line(&format!("impl Drop for {storage_type} {{"));
                    self.indent += 1;
                    self.line("fn drop(&mut self) {");
                    self.indent += 1;
                    if tracks_construction {
                        self.line("if !self.__terrane_constructed { return; }");
                    }
                    if !object.resource_owning {
                        self.line(
                            "if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 { return; }",
                        );
                    }
                    if awaits_destruction {
                        self.line("__terrane_await_destructor(async {");
                        self.indent += 1;
                        for call in &destructor_calls {
                            self.line(&format!("{call};"));
                        }
                        self.indent -= 1;
                        self.line("});");
                    } else {
                        for call in &destructor_calls {
                            self.line(&format!("{call};"));
                        }
                    }
                    self.indent -= 1;
                    self.line("}");
                    self.indent -= 1;
                    self.line("}");
                }
            }
        }
    }

    fn field_initial_value(&mut self, effective: EffectiveObjectField<'a>) -> String {
        let field = effective.field;
        if field.required {
            return "None".to_owned();
        }
        if let Some(initializer_span) = field.initializer_span {
            let initializer = find_node_by_span(&effective.unit.tree.root, initializer_span)
                .expect("semantic field initializer span must resolve in its declaration unit");
            let previous_unit = std::mem::replace(&mut self.unit, effective.unit);
            let previous_source = std::mem::replace(&mut self.source, &effective.unit.source);
            let previous_object = self
                .current_object
                .replace(effective.owner.identity.clone());
            let value = self.expression_as(initializer, field.value_type.clone());
            self.current_object = previous_object;
            self.source = previous_source;
            self.unit = previous_unit;
            return value;
        }
        if let Some(value) = canonical_field_default(self.package, &field.value_type) {
            return value;
        }
        match field.value_type {
            ValueType::PlatformStreamHandle
            | ValueType::PlatformResourceHandle
            | ValueType::FilesystemAuthority => "Default::default()".to_owned(),
            _ => unreachable!("semantic analysis requires a class field initializer"),
        }
    }
}
