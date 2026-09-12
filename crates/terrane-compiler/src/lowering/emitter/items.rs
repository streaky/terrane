use super::super::prelude::*;

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

impl<'a> Emitter<'a> {
    fn field_initial_value(&mut self, effective: EffectiveObjectField<'a>) -> String {
        let field = effective.field;
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

    pub(super) fn global_storage(&self, node: &SyntaxNode) -> Option<String> {
        (node.kind == SyntaxKind::Name)
            .then(|| {
                self.package
                    .resolve_name_at(self.unit, node.span.start, self.text(node))
            })
            .flatten()
            .filter(|symbol| symbol.global && symbol.kind == SymbolKind::Binding)
            .map(|symbol| global_binding_name(&symbol.name))
    }

    pub(super) fn global_assignment(&mut self, node: &SyntaxNode) -> bool {
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
    pub(super) fn namespace_binding(&mut self, node: &SyntaxNode) {
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
    pub(super) fn object(&mut self, node: &SyntaxNode) {
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
                        crate::projection::ProjectedKind::Interface(interface) => Some(interface),
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
                let generic_declaration = associated_bounds
                    .as_ref()
                    .map_or_else(String::new, |bounds| {
                        format!("<TerraneAssociated: {bounds}>")
                    });
                let generic_use = associated_bounds
                    .as_ref()
                    .map_or("", |_| "<TerraneAssociated>");
                let protocol_use = format!("{protocol}{generic_use}");
                let transfer_bounds = if projected_requirements
                    .is_some_and(|item| item.send && item.sync)
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
                self.line(&format!(
                    "pub trait {protocol}{generic_declaration}{transfer_bounds} {{"
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
                    write!(self.output, "fn {}({receiver}", rust_name(&method.name)).unwrap();
                    for parameter in &method.parameters {
                        let ty = parameter.value_type.clone().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        write!(self.output, ", {}: {ty}", rust_name(&parameter.name)).unwrap();
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
                        "pub {}fn {}({receiver}",
                        if method.is_async { "async " } else { "" },
                        rust_name(&method.name)
                    )
                    .unwrap();
                    for parameter in &method.parameters {
                        let ty = parameter.value_type.clone().map_or_else(
                            || "i128".to_owned(),
                            |value_type| rust_value_type(self.package, value_type),
                        );
                        write!(self.output, ", {}: {ty}", rust_name(&parameter.name)).unwrap();
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
                        .map(|parameter| rust_name(&parameter.name))
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.line(&format!(
                        "self.0.{}({arguments}){}",
                        rust_name(&method.name),
                        if method.is_async { ".await" } else { "" }
                    ));
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
                    && let crate::projection::ProjectedKind::Interface(interface) =
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
                let descendants = object_descendants(self.unit, object);
                let class_type = rust_object_type_name(self.package, &object.identity);
                let storage_type = if descendants.is_empty() {
                    class_type.clone()
                } else {
                    format!("{class_type}Storage")
                };
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

                if !object.resource_owning {
                    self.line("#[derive(Clone)]");
                }
                self.line(&format!("pub struct {storage_type} {{"));
                self.indent += 1;
                if has_destructor && !object.resource_owning {
                    self.line("__terrane_lifetime: std::sync::Arc<()>,");
                }
                for field in &instance_fields {
                    self.line(&format!(
                        "pub {}: {},",
                        rust_name(&field.field.name),
                        rust_value_type(self.package, field.field.value_type.clone())
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
                    if construct.throws {
                        self.output.push_str(") -> Result<Self, TerraneError> {\n");
                    } else {
                        self.output.push_str(") -> Self {\n");
                    }
                    self.indent += 1;
                    self.line("let mut value = Self {");
                    self.indent += 1;
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
                        .map(|parameter| rust_name(&parameter.name))
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.line(&format!(
                        "value.construct({arguments}){};",
                        if construct.throws { "?" } else { "" }
                    ));
                    self.line(if construct.throws {
                        "Ok(value)"
                    } else {
                        "value"
                    });
                    self.indent -= 1;
                    self.line("}");
                } else {
                    self.line("pub fn terrane_construct() -> Self {");
                    self.indent += 1;
                    self.line("Self {");
                    self.indent += 1;
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
                        if construct.throws {
                            self.output.push_str(") -> Result<Self, TerraneError> {\n");
                        } else {
                            self.output.push_str(") -> Self {\n");
                        }
                        self.indent += 1;
                        let arguments = construct
                            .parameters
                            .iter()
                            .map(|parameter| rust_name(&parameter.name))
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
                        self.line_start();
                        let receiver = match method.written_invocation_mode {
                            InvocationMode::Consuming => "self",
                            InvocationMode::Mutable => "&mut self",
                            InvocationMode::Shared => "&self",
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
                    for field in &instance_fields {
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
                            if self.unit.descriptors.iter().any(|candidate| {
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
                            if self.unit.descriptors.iter().any(|candidate| {
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
                            matches!(item.kind, crate::projection::ProjectedKind::Interface(_))
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
                        .find(|candidate| candidate.identity == *interface_identity)
                        .expect("validated interface contract");
                    let interface_type = rust_object_type_name(self.package, &interface.identity);
                    let interface_base =
                        rust_object_type_name(self.package, &interface.identity.base());
                    let protocol = format!(
                        "{interface_base}Protocol{}",
                        interface_identity.application.as_deref().map_or_else(
                            String::new,
                            |application| format!(
                                "<{}>",
                                rust_value_type(self.package, application.clone())
                            )
                        )
                    );
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
                    let interface_methods = effective_object_methods(interface_unit, interface)
                        .into_iter()
                        .map(|method| {
                            crate::semantics::bind_projected_requirement(
                                method,
                                interface_identity.application.as_deref(),
                            )
                        })
                        .collect::<Vec<_>>();
                    for method in &interface_methods {
                        let implementation = effective_object_methods(self.unit, object)
                            .into_iter()
                            .find(|candidate| {
                                candidate.name == method.name && !candidate.is_static
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
                        write!(self.output, "fn {}({receiver}", rust_name(&method.name)).unwrap();
                        for parameter in &method.parameters {
                            let ty = parameter.value_type.clone().map_or_else(
                                || "i128".to_owned(),
                                |value_type| rust_value_type(self.package, value_type),
                            );
                            write!(self.output, ", {}: {ty}", rust_name(&parameter.name)).unwrap();
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
                            .map(|parameter| rust_name(&parameter.name))
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
                            write!(
                                self.output,
                                "{class_type}::{}({receiver}, {arguments}){}",
                                rust_name(&implementation.name),
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
                                        &rust_name(&parameter.name),
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

    #[expect(
        clippy::too_many_lines,
        reason = "foreign interface method signatures and boundary conversions are emitted together"
    )]
    fn projected_interface_implementation(
        &mut self,
        object: &DescriptorContract,
        interface_identity: &ObjectIdentity,
    ) {
        let Some(projected_item) = self
            .package
            .projection
            .item(&interface_identity.namespace, &interface_identity.name)
        else {
            return;
        };
        let crate::projection::ProjectedKind::Interface(interface) = &projected_item.kind else {
            return;
        };
        let trait_path = projected_item.rust_path.clone();
        let mut interface = interface.clone();
        let associated_application = interface_identity
            .application
            .as_deref()
            .map(|application| {
                crate::semantics::destination_projected_type(self.package, application)
                    .expect("validated projected associated type")
            });
        if let Some(application) = &associated_application {
            for method in &mut interface.methods {
                for parameter in &mut method.function.parameters {
                    parameter.ty.bind_associated(application);
                }
                method.function.result.bind_associated(application);
            }
        }
        let generic_bounds = interface.associated_type.as_ref().map(|associated| {
            let bounds = associated.bounds.join(" + ");
            if bounds.is_empty() {
                "'static".to_owned()
            } else {
                format!("'static + {bounds}")
            }
        });
        let generic_declaration =
            if object.kind == ObjectKind::Interface && interface_identity.application.is_none() {
                generic_bounds.as_ref().map_or_else(String::new, |bounds| {
                    format!("<TerraneAssociated: {bounds}>")
                })
            } else {
                String::new()
            };
        let mut class_type = rust_object_type_name(self.package, &object.identity);
        if !generic_declaration.is_empty() {
            class_type.push_str("<TerraneAssociated>");
        }
        let implementation_paths = if object.kind == ObjectKind::Interface {
            interface
                .supertraits
                .iter()
                .map(|supertrait| supertrait.rust_path.clone())
                .chain(std::iter::once(trait_path))
                .collect::<Vec<_>>()
        } else {
            vec![trait_path]
        };
        for implementation_path in implementation_paths {
            self.line(&format!(
                "impl{generic_declaration} {implementation_path} for {class_type} {{"
            ));
            self.indent += 1;
            if let Some(associated) = &interface.associated_type
                && associated
                    .rust_path
                    .rsplit_once("::")
                    .map(|(owner, _)| owner)
                    == Some(implementation_path.as_str())
            {
                let application = associated_application.as_ref().map_or_else(
                    || "TerraneAssociated".to_owned(),
                    crate::projection::ProjectedType::rust_type,
                );
                self.line(&format!("type {} = {application};", associated.name));
            }
            for projected in interface
                .methods
                .iter()
                .filter(|method| {
                    method.owner_rust_path.as_deref() == Some(implementation_path.as_str())
                })
                .cloned()
            {
                if object.kind == ObjectKind::Interface && projected.provided {
                    continue;
                }
                let method = projected.function;
                let Some(implementation) = effective_object_methods(self.unit, object)
                    .into_iter()
                    .find(|candidate| candidate.name == method.name && !candidate.is_static)
                else {
                    continue;
                };
                self.line_start();
                if method.is_async {
                    self.output.push_str("async ");
                }
                let receiver = match (method.receiver, implementation.written_invocation_mode) {
                    (Some(crate::projection::Receiver::Move), InvocationMode::Mutable) => {
                        "mut self"
                    }
                    (Some(crate::projection::Receiver::Move), _) => "self",
                    (Some(crate::projection::Receiver::MutableBorrow), _) => "&mut self",
                    _ => "&self",
                };
                write!(self.output, "fn {}({receiver}", rust_name(&method.name))
                    .expect("writing to a string cannot fail");
                for parameter in &method.parameters {
                    let mut ty = parameter.ty.rust_type();
                    if parameter.borrowed {
                        ty = format!(
                            "&{}{}",
                            if parameter.mutable_borrow { "mut " } else { "" },
                            ty
                        );
                    }
                    write!(self.output, ", {}: {ty}", rust_name(&parameter.name))
                        .expect("writing to a string cannot fail");
                }
                self.output.push(')');
                let rust_result = method.result.rust_type();
                if method.result != crate::projection::ProjectedType::None {
                    write!(self.output, " -> {rust_result}")
                        .expect("writing to a string cannot fail");
                }
                self.output.push_str(" {\n");
                self.indent += 1;
                let arguments = method
                    .parameters
                    .iter()
                    .map(|parameter| {
                        projected_callback_input_expression(
                            &rust_name(&parameter.name),
                            &parameter.ty,
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let receiver = match (method.receiver, implementation.written_invocation_mode) {
                    (Some(crate::projection::Receiver::Move), InvocationMode::Shared) => "&self",
                    (Some(crate::projection::Receiver::Move), InvocationMode::Mutable) => {
                        "&mut self"
                    }
                    (Some(crate::projection::Receiver::Move), InvocationMode::Consuming) => "self",
                    (_, InvocationMode::Shared) => "&*self",
                    (_, InvocationMode::Mutable) => "&mut *self",
                    (_, InvocationMode::Consuming) => {
                        unreachable!("validated projected interface mode compatibility")
                    }
                };
                let await_ = if method.is_async { ".await" } else { "" };
                let terrane_call = format!(
                    "<{class_type}>::{}({receiver}, {arguments}){await_}",
                    rust_name(&method.name)
                );
                let terrane_call = if method.is_async && interface.send {
                    match method.receiver {
                        Some(crate::projection::Receiver::Borrow) => format!(
                            "{{ let __terrane_receiver = self.clone(); __terrane_projected_async_entry(async move {{ <{class_type}>::{}(&__terrane_receiver, {arguments}).await }}).await }}",
                            rust_name(&method.name)
                        ),
                        Some(crate::projection::Receiver::MutableBorrow) => format!(
                            "{{ let mut __terrane_receiver = self.clone(); let (__terrane_output, __terrane_receiver) = __terrane_projected_async_entry(async move {{ let __terrane_output = <{class_type}>::{}(&mut __terrane_receiver, {arguments}).await; (__terrane_output, __terrane_receiver) }}).await; *self = __terrane_receiver; __terrane_output }}",
                            rust_name(&method.name)
                        ),
                        Some(crate::projection::Receiver::Move) => format!(
                            "__terrane_projected_async_entry(async move {{ {terrane_call} }}).await"
                        ),
                        None => terrane_call,
                    }
                } else {
                    terrane_call
                };
                let converted =
                    projected_callback_output_expression("__terrane_value", &method.result);
                if method.is_async {
                    self.line(&format!(
                    "let __terrane_boundary: Result<{rust_result}, crate::TerraneForeignError> = async {{ let __terrane_value = {terrane_call}; Ok({converted}) }}.await;"
                ));
                } else {
                    self.line(&format!(
                    "let __terrane_boundary: Result<{rust_result}, crate::TerraneForeignError> = (|| {{ let __terrane_value = {terrane_call}; Ok({converted}) }})();"
                ));
                }
                self.line(
                    "__terrane_boundary.unwrap_or_else(|error| panic!(\"{}\", error.render()))",
                );
                self.indent -= 1;
                self.line("}");
            }
            self.indent -= 1;
            self.line("}");
        }
    }

    pub(super) fn function(&mut self, node: &SyntaxNode) {
        self.emit_function(node, None);
    }

    pub(super) fn object_method(&mut self, node: &SyntaxNode) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("object method must have an analyzed contract");
        let receiver = if contract.name == "destruct" {
            "&mut self"
        } else {
            match contract.written_invocation_mode {
                InvocationMode::Consuming => "self",
                InvocationMode::Mutable => "&mut self",
                InvocationMode::Shared => "&self",
            }
        };
        self.emit_function_as(node, Some(receiver), None);
    }

    pub(super) fn object_static_method(&mut self, node: &SyntaxNode) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed static method must have a semantic contract");
        self.emit_function_as(
            node,
            None,
            Some(&format!("terrane_static_{}", rust_name(&contract.name))),
        );
    }
    pub(super) fn object_method_as(&mut self, node: &SyntaxNode, name: &str) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("object method must have an analyzed contract");
        let receiver = if contract.name == "destruct" {
            "&mut self"
        } else {
            match contract.written_invocation_mode {
                InvocationMode::Consuming => "self",
                InvocationMode::Mutable => "&mut self",
                InvocationMode::Shared => "&self",
            }
        };
        self.emit_function_as(node, Some(receiver), Some(name));
    }

    pub(super) fn emit_function(&mut self, node: &SyntaxNode, receiver: Option<&str>) {
        self.emit_function_as(node, receiver, None);
    }

    #[expect(
        clippy::too_many_lines,
        reason = "function lowering preserves one ordered signature and body pipeline"
    )]
    pub(super) fn emit_function_as(
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
        let return_type = contract.return_type.clone().map(|return_type| {
            if contract.is_static
                && let ValueType::Object(returned) = &return_type
                && contract.owner.as_deref() == Some(returned.name.as_str())
                && let Some(effective) = &self.current_object
            {
                ValueType::Object(effective.clone())
            } else {
                return_type
            }
        });
        let reference_lender = self
            .unit
            .reference_return_lenders
            .get(&(contract.span.file, contract.span.start, contract.span.end))
            .copied();
        if receiver.is_none()
            && contract.owner.is_none()
            && contract.name != "main"
            && !self.unit.bundled
            && !self.package.function_is_referenced(contract.span)
        {
            self.line("#[allow(dead_code)]");
        }
        self.line_start();
        let name =
            name_override.map_or_else(|| function_name(self.package, contract), str::to_owned);
        let async_main = contract.is_async && contract.name == "main" && receiver.is_none();
        write!(
            self.output,
            "{}{}fn {name}{}(",
            if contract.owner.is_some() || (receiver.is_none() && self.unit.bundled) {
                "pub "
            } else {
                ""
            },
            if contract.is_async && !async_main {
                "async "
            } else {
                ""
            },
            if reference_lender.is_some() {
                "<'a>"
            } else {
                ""
            },
        )
        .unwrap();
        if let Some(receiver) = receiver {
            self.output.push_str(receiver);
        }
        for (index, parameter) in contract.parameters.iter().enumerate() {
            if receiver.is_some() || index != 0 {
                self.output.push_str(", ");
            }
            let ty = match (&parameter.value_type, reference_lender == Some(index)) {
                (Some(ValueType::Reference(item)), true) => {
                    format!("&'a {}", rust_element_type(self.package, item.clone()))
                }
                _ => parameter.value_type.clone().map_or_else(
                    || "i128".to_owned(),
                    |value_type| rust_value_type(self.package, value_type),
                ),
            };
            let mutable = if parameter.mutable { "mut " } else { "" };
            write!(self.output, "{mutable}{}: {ty}", rust_name(&parameter.name)).unwrap();
        }
        self.output.push(')');
        let function_errors = contract.throws && contract.name != "main";
        if function_errors {
            let result = return_type.clone().map_or_else(
                || "()".to_owned(),
                |value_type| match value_type {
                    ValueType::Reference(item) if reference_lender.is_some() => {
                        format!("&'a {}", rust_element_type(self.package, item))
                    }
                    value_type => rust_value_type(self.package, value_type),
                },
            );
            write!(self.output, " -> Result<{result}, TerraneError>").unwrap();
        } else if let Some(return_type) = return_type.clone()
            && return_type != ValueType::Scalar(ScalarType::None)
        {
            let return_type = match return_type {
                ValueType::Reference(item) if reference_lender.is_some() => {
                    format!("&'a {}", rust_element_type(self.package, item))
                }
                return_type => rust_value_type(self.package, return_type),
            };
            write!(self.output, " -> {return_type}").unwrap();
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
            self.line("__terrane_run(async move {});");
            self.indent -= 1;
            self.line("}");
            return;
        }
        self.output.push_str(" {\n");
        if async_main {
            self.indent += 1;
            self.line("__terrane_run(async move {");
        }
        let outer_return_type = std::mem::replace(&mut self.return_type, return_type);
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

    #[expect(
        clippy::too_many_lines,
        reason = "closure ownership, contracts, captures, and body lowering form one emission path"
    )]
    pub(super) fn anonymous_function(&mut self, node: &SyntaxNode) -> String {
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
        let parameter_names = contract
            .parameters
            .iter()
            .map(|parameter| rust_name(&parameter.name))
            .collect::<Vec<_>>();
        let parameter_types = contract
            .parameters
            .iter()
            .map(|parameter| {
                parameter.value_type.clone().map_or_else(
                    || "i128".to_owned(),
                    |value_type| rust_value_type(self.package, value_type),
                )
            })
            .collect::<Vec<_>>();
        let tuple_pattern = match parameter_names.as_slice() {
            [] => "()".to_owned(),
            [name] => format!("({name},)"),
            _ => format!("({})", parameter_names.join(", ")),
        };
        let tuple_type = match parameter_types.as_slice() {
            [] => "()".to_owned(),
            [ty] => format!("({ty},)"),
            _ => format!("({})", parameter_types.join(", ")),
        };
        let stateful_parameters = format!("{tuple_pattern}: {tuple_type}");
        let result = contract
            .return_type
            .clone()
            .unwrap_or(ValueType::Scalar(ScalarType::None));
        let result_type = if contract.throws {
            format!(
                "Result<{}, TerraneError>",
                rust_value_type(self.package, result.clone())
            )
        } else {
            rust_value_type(self.package, result.clone())
        };
        let outer_output = std::mem::take(&mut self.output);
        let outer_indent = self.indent;
        let outer_return_type = self.return_type.replace(result.clone());
        let outer_function_errors = std::mem::replace(&mut self.function_errors, contract.throws);
        let outer_propagation = std::mem::replace(&mut self.propagate_errors, contract.throws);
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
        let outer_async_mutable_captures = std::mem::take(&mut self.async_mutable_captures);
        if contract.is_async && contract.written_invocation_mode == InvocationMode::Mutable {
            self.async_mutable_captures
                .extend(contract.captures.iter().cloned());
        }
        self.closure_depth += 1;
        self.indent = outer_indent + 1;
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            self.block(block);
            if result == ValueType::Scalar(ScalarType::None) && block_may_fall_through(block) {
                self.line(if contract.throws { "Ok(())" } else { "()" });
            }
        }
        self.closure_depth -= 1;
        let body = std::mem::replace(&mut self.output, outer_output);
        self.indent = outer_indent;
        self.return_type = outer_return_type;
        self.function_errors = outer_function_errors;
        self.propagate_errors = outer_propagation;
        self.parameter_types = outer_parameter_types;
        self.async_mutable_captures = outer_async_mutable_captures;
        let (mut captures, mut invocation_captures) =
            self.anonymous_function_captures(node, contract);
        let invocation_guard =
            if contract.is_async && contract.written_invocation_mode == InvocationMode::Mutable {
                captures.push_str("let __terrane_invocation = TerraneAsyncInvocationGate::new(); ");
                invocation_captures
                    .push_str("let __terrane_invocation = __terrane_invocation.share(); ");
                "let _invocation = __terrane_invocation.enter().await;\n"
            } else {
                ""
            };
        let constructor = match contract.written_invocation_mode {
            InvocationMode::Shared => "std::sync::Arc::new",
            InvocationMode::Mutable => "TerraneMutableCallable::new",
            InvocationMode::Consuming => "TerraneConsumingCallable::new",
        };
        let closure_parameters = if contract.written_invocation_mode == InvocationMode::Shared {
            parameters
        } else {
            stateful_parameters
        };
        if contract.is_async {
            format!(
                "{{ {captures}{constructor}(move |{closure_parameters}| -> std::pin::Pin<Box<dyn Future<Output = {result_type}> + Send>> {{ {invocation_captures}Box::pin(async move {{\n{invocation_guard}{body}{}}}) }}) }}",
                "    ".repeat(outer_indent)
            )
        } else {
            format!(
                "{{ {captures}{constructor}(move |{closure_parameters}| -> {result_type} {{\n{body}{}}}) }}",
                "    ".repeat(outer_indent)
            )
        }
    }

    fn anonymous_function_captures(
        &self,
        node: &SyntaxNode,
        contract: &FunctionContract,
    ) -> (String, String) {
        let mut captures = String::new();
        let mut invocation_captures = String::new();
        for capture in &contract.captures {
            let name = rust_name(capture);
            let mutable = if contract.written_invocation_mode == InvocationMode::Mutable {
                "mut "
            } else {
                ""
            };
            let source = if capture == "this" { "self" } else { &name };
            let binding = self.unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == *capture && binding.is_visible_at(self.source.id(), node.span.start)
            });
            let transfer =
                binding.is_some_and(|binding| self.value_type_owns_resource(&binding.value_type));
            let borrowed = binding.is_some_and(|binding| {
                matches!(binding.value_type, ValueType::Reference(_))
                    && self
                        .unit
                        .reference_provenance
                        .get(&(binding.span.start, binding.span.end))
                        .is_some_and(|provenance| {
                            !self.reference_owner_uses_shared_storage(provenance.owner)
                        })
            });
            if contract.is_async && contract.written_invocation_mode == InvocationMode::Mutable {
                write!(
                    captures,
                    "let {name} = TerraneAsyncMutableState::new({source}.clone()); "
                )
                .expect("writing to a String cannot fail");
                write!(invocation_captures, "let {name} = {name}.share(); ")
                    .expect("writing to a String cannot fail");
                continue;
            }
            if transfer || borrowed {
                write!(captures, "let {mutable}{name} = {source}; ")
                    .expect("writing to a String cannot fail");
            } else {
                write!(captures, "let {mutable}{name} = {source}.clone(); ")
                    .expect("writing to a String cannot fail");
            }
            if borrowed {
                write!(invocation_captures, "let {mutable}{name} = {name}; ")
                    .expect("writing to a String cannot fail");
            } else {
                write!(
                    invocation_captures,
                    "let {mutable}{name} = {name}.clone(); "
                )
                .expect("writing to a String cannot fail");
            }
        }
        (captures, invocation_captures)
    }

    pub(super) fn block(&mut self, block: &SyntaxNode) {
        for statement in &block.children {
            self.statement(statement);
        }
    }

    pub(super) fn union_binding(&self, node: &SyntaxNode) -> Option<TypedBinding> {
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

    pub(super) fn union_value(&mut self, binding: &TypedBinding, value: &SyntaxNode) -> String {
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

    pub(super) fn emit_union_types(&mut self) {
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
}
