use super::super::prelude::*;

fn collect_local_functions<'a>(node: &'a SyntaxNode, output: &mut Vec<&'a SyntaxNode>) {
    if node.kind == SyntaxKind::FunctionDeclaration {
        output.push(node);
    } else if node.kind != SyntaxKind::AnonymousFunction {
        for child in &node.children {
            collect_local_functions(child, output);
        }
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

fn contains_unsafe_call(node: &SyntaxNode) -> bool {
    node.kind == SyntaxKind::UnsafeRustBlock
        || crate::syntax::call_is_unsafe(node)
        || node.children.iter().any(contains_unsafe_call)
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

impl<'a> Emitter<'a> {
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
                            super::enums::value_type_uses_parameter(
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
                            .map(|parameter| rust_name(&parameter.name))
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
                        write!(self.output, "{}: {ty}", rust_name(&parameter.name)).unwrap();
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
                        .map(|parameter| rust_name(&parameter.name))
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
                        .map(|parameter| rust_name(&parameter.name))
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
        let crate::rust_interop::projection::ProjectedKind::Interface(interface) =
            &projected_item.kind
        else {
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
                    crate::rust_interop::projection::ProjectedType::rust_type,
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
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Mutable,
                    ) => "mut self",
                    (Some(crate::rust_interop::projection::Receiver::Move), _) => "self",
                    (Some(crate::rust_interop::projection::Receiver::MutableBorrow), _) => {
                        "&mut self"
                    }
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
                if method.result != crate::rust_interop::projection::ProjectedType::None {
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
                            &parameter.ty.rust_type(),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let receiver = match (method.receiver, implementation.written_invocation_mode) {
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Shared,
                    ) => "&self",
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Mutable,
                    ) => "&mut self",
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Consuming,
                    ) => "self",
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
                        Some(crate::rust_interop::projection::Receiver::Borrow) => format!(
                            "{{ let __terrane_receiver = self.clone(); __terrane_projected_async_entry(async move {{ <{class_type}>::{}(&__terrane_receiver, {arguments}).await }}).await }}",
                            rust_name(&method.name)
                        ),
                        Some(crate::rust_interop::projection::Receiver::MutableBorrow) => format!(
                            "{{ let mut __terrane_receiver = self.clone(); let (__terrane_output, __terrane_receiver) = __terrane_projected_async_entry(async move {{ let __terrane_output = <{class_type}>::{}(&mut __terrane_receiver, {arguments}).await; (__terrane_output, __terrane_receiver) }}).await; *self = __terrane_receiver; __terrane_output }}",
                            rust_name(&method.name)
                        ),
                        Some(crate::rust_interop::projection::Receiver::Move) => format!(
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

    pub(super) fn local_function(&mut self, node: &SyntaxNode) {
        let outer_completion = std::mem::replace(&mut self.try_completion, false);
        let outer_loop = std::mem::replace(&mut self.in_loop, false);
        let outer_continue = self.continue_label.take();
        let outer_break = self.break_label.take();
        let outer_error = self.current_error.take();
        let outer_closure_depth = std::mem::replace(&mut self.closure_depth, 0);
        let outer_ranges = std::mem::take(&mut self.bounded_integer_ranges);
        let outer_list_borrows = std::mem::take(&mut self.list_append_borrows);
        let outer_captures = std::mem::take(&mut self.async_mutable_captures);
        let outer_fresh_lists = std::mem::take(&mut self.fresh_empty_lists);
        self.function(node);
        self.try_completion = outer_completion;
        self.in_loop = outer_loop;
        self.continue_label = outer_continue;
        self.break_label = outer_break;
        self.current_error = outer_error;
        self.closure_depth = outer_closure_depth;
        self.bounded_integer_ranges = outer_ranges;
        self.list_append_borrows = outer_list_borrows;
        self.async_mutable_captures = outer_captures;
        self.fresh_empty_lists = outer_fresh_lists;
    }

    fn consuming_method_needs_mutable_self(&self, node: &SyntaxNode) -> bool {
        fn rooted_in_this(emitter: &Emitter<'_>, node: &SyntaxNode) -> bool {
            if node.kind == SyntaxKind::Name {
                return emitter.text(node) == "this";
            }
            node.kind == SyntaxKind::MemberExpression
                && node
                    .children
                    .first()
                    .is_some_and(|receiver| rooted_in_this(emitter, receiver))
        }
        if node.kind == SyntaxKind::Assignment
            && node
                .children
                .first()
                .is_some_and(|target| rooted_in_this(self, target))
        {
            return true;
        }
        if node.kind == SyntaxKind::UnaryExpression
            && self.unary_operator(node).as_deref() == Some("move")
            && node
                .children
                .last()
                .is_some_and(|operand| rooted_in_this(self, operand))
        {
            return true;
        }
        if node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                callee.kind == SyntaxKind::MemberExpression
                    && callee
                        .children
                        .first()
                        .is_some_and(|receiver| rooted_in_this(self, receiver))
            })
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| self.consuming_method_needs_mutable_self(child))
    }

    pub(super) fn object_method(&mut self, node: &SyntaxNode) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("object method must have an analyzed contract");
        let receiver = if contract.name == "destruct" {
            if contract.is_async {
                "mut self: &mut Self"
            } else {
                "&mut self"
            }
        } else {
            match contract.written_invocation_mode {
                InvocationMode::Consuming if self.consuming_method_needs_mutable_self(node) => {
                    "mut self"
                }
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
            if contract.is_async {
                "mut self: &mut Self"
            } else {
                "&mut self"
            }
        } else {
            match contract.written_invocation_mode {
                InvocationMode::Consuming if self.consuming_method_needs_mutable_self(node) => {
                    "mut self"
                }
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

    fn emit_select_cursors(&mut self, function_span: crate::Span) {
        let selections = self
            .unit
            .selections
            .iter()
            .filter(|selection| {
                self.unit
                    .enclosing_function_spans
                    .get(&selection.span.start)
                    .copied()
                    .flatten()
                    == Some(function_span)
            })
            .map(|selection| selection.span.start)
            .collect::<Vec<_>>();
        for selection_start in selections {
            self.line(&format!(
                "let mut __terrane_select_cursor_{selection_start} = 0usize;"
            ));
        }
    }

    pub(super) fn invocation_scoped_type_generics(
        &self,
        node: &SyntaxNode,
        rust_type: &str,
        lifetimes: &[String],
    ) -> Vec<String> {
        let mut declarations = Vec::new();
        if node.kind == SyntaxKind::CallExpression
            && self.unit.selected_expression_types.contains_key(&(
                node.span.file,
                node.span.start,
                node.span.end,
            ))
            && let Some(callee) = node.children.first()
            && let Some(symbol) = self.package.resolve_name_at(
                self.unit,
                callee.span.start,
                &self.unit.source.text()[callee.span.start..callee.span.end],
            )
            && let Some(item) = self
                .package
                .projection
                .item(&symbol.namespace, &symbol.name)
            && let crate::rust_interop::projection::ProjectedKind::Function(function) = &item.kind
        {
            declarations.extend(
                function
                    .generic_parameters
                    .iter()
                    .filter(|parameter| {
                        rust_type
                            .split(|character: char| {
                                !character.is_ascii_alphanumeric() && character != '_'
                            })
                            .any(|token| token == parameter.name)
                    })
                    .map(|parameter| {
                        (
                            parameter.name.clone(),
                            parameter
                                .rust_bounds
                                .iter()
                                .map(|bound| {
                                    lifetimes.iter().enumerate().fold(
                                        bound.clone(),
                                        |bound, (index, lifetime)| {
                                            let binder = format!("for<{lifetime}>");
                                            let marker =
                                                format!("__terrane_lifetime_binder_{index}");
                                            bound
                                                .replace(&binder, &marker)
                                                .replace(lifetime, "'view")
                                                .replace(&marker, &binder)
                                        },
                                    )
                                })
                                .collect::<Vec<_>>(),
                        )
                    }),
            );
        }
        for child in &node.children {
            declarations.extend(
                self.invocation_scoped_type_generics(child, rust_type, lifetimes)
                    .into_iter()
                    .map(|declaration| {
                        if let Some((name, bounds)) = declaration.split_once(':') {
                            (
                                name.trim().to_owned(),
                                bounds
                                    .split(" + ")
                                    .map(str::trim)
                                    .map(str::to_owned)
                                    .collect(),
                            )
                        } else {
                            (declaration, Vec::new())
                        }
                    }),
            );
        }
        let mut merged =
            std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
        for (name, bounds) in declarations {
            merged.entry(name).or_default().extend(bounds);
        }
        if !lifetimes.is_empty() {
            for bounds in merged.values_mut() {
                bounds.insert("'view".to_owned());
            }
        }
        merged
            .into_iter()
            .map(|(name, bounds)| {
                if bounds.is_empty() {
                    name
                } else {
                    format!(
                        "{name}: {}",
                        bounds.into_iter().collect::<Vec<_>>().join(" + ")
                    )
                }
            })
            .collect()
    }
    fn emit_parameter_union_carriers(&mut self, contract: &FunctionContract) {
        for parameter in &contract.parameters {
            let Some(ValueType::Union(arms)) = self.unit.flow_binding_types.get(&parameter.span)
            else {
                continue;
            };
            let Some(source_type) = parameter.binding_value_type() else {
                continue;
            };
            let Some(index) = arms.iter().position(|arm| arm == &source_type) else {
                continue;
            };
            let name = rust_local_name(&parameter.name, parameter.span);
            let carrier = union_type_name_for_span(parameter.span);
            self.line(&format!(
                "let mut {name}: {carrier} = {carrier}::Arm{index}({});",
                rust_name(&parameter.name)
            ));
        }
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
        self.fresh_empty_lists.clear();

        let contract = self
            .unit
            .functions
            .iter()
            .find(|item| item.span == node.span)
            .expect("analyzed function declaration must have a semantic contract");
        let mut return_type = contract.return_type.clone().map(|return_type| {
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
        if matches!(
            return_type,
            Some(ValueType::InvocationScopedNative {
                concrete: false,
                ..
            })
        ) {
            return_type = self
                .unit
                .invocation_scoped_function_results
                .get(&(contract.span.file, contract.span.start, contract.span.end))
                .cloned();
        }
        let scoped_lifetime = if let Some(ValueType::InvocationScopedNative {
            rust_type,
            lifetimes,
            ..
        }) = &mut return_type
        {
            for lifetime in lifetimes.iter() {
                *rust_type = rust_type.replace(lifetime, "'view");
            }
            Some("'view")
        } else {
            None
        };
        let reference_lender = self
            .unit
            .reference_return_lenders
            .get(&(contract.span.file, contract.span.start, contract.span.end))
            .copied();
        let scoped_type_generics = return_type
            .as_ref()
            .and_then(|return_type| match return_type {
                ValueType::InvocationScopedNative {
                    rust_type,
                    lifetimes,
                    ..
                } => Some(self.invocation_scoped_type_generics(node, rust_type, lifetimes)),
                _ => None,
            })
            .unwrap_or_default();
        let function_generics = if reference_lender.is_some() {
            "<'a>".to_owned()
        } else {
            let mut generics = Vec::new();
            if scoped_lifetime == Some("'view") {
                generics.push("'view".to_owned());
            }
            generics.extend(scoped_type_generics);
            generics.extend(rust_generic_parameter_declarations(
                self.package,
                &contract.generic_parameters,
            ));
            if generics.is_empty() {
                String::new()
            } else {
                format!("<{}>", generics.join(", "))
            }
        };
        if contract.is_unsafe || contains_unsafe_call(node) {
            self.line("#[allow(unsafe_code)]");
        }
        if receiver.is_none()
            && contract.owner.is_none()
            && (contract.name != "main"
                || self.package.artifact == crate::package::ArtifactKind::DynamicLibrary)
            && !self.unit.bundled
            && !self.package.function_is_referenced(contract.span)
        {
            self.line("#[allow(dead_code)]");
        }
        if contract.name == "destruct" && contract.is_async {
            self.line("#[allow(unused_mut)]");
        }
        self.line_start();
        let name =
            name_override.map_or_else(|| function_name(self.package, contract), str::to_owned);
        let async_main = contract.is_async && contract.name == "main" && receiver.is_none();
        write!(
            self.output,
            "{}{}{}fn {name}{}(",
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
            if contract.is_unsafe { "unsafe " } else { "" },
            function_generics,
        )
        .unwrap();
        if let Some(receiver) = receiver {
            self.output.push_str(receiver);
        }
        for (index, parameter) in contract.parameters.iter().enumerate() {
            if receiver.is_some() || index != 0 {
                self.output.push_str(", ");
            }
            let binding_type = parameter.binding_value_type();
            let native_mutable_reference = self
                .registry
                .mutable_native_callback_parameters
                .borrow()
                .contains(&(
                    (contract.span.file, contract.span.start, contract.span.end),
                    index,
                ))
                || parameter.mutable
                    && matches!(
                        &binding_type,
                        Some(ValueType::Reference(item))
                            if matches!(
                                item.value_type(),
                                ValueType::Object(identity)
                                    if identity.native_projection.is_some()
                                        || self.package.projection.item(&identity.namespace, &identity.name).is_some()
                            )
                    );
            let ty = match (&binding_type, reference_lender == Some(index)) {
                (Some(ValueType::Reference(item)), _) if native_mutable_reference => {
                    format!(
                        "&{}mut {}",
                        if reference_lender == Some(index) {
                            "'a "
                        } else if scoped_lifetime == Some("'view") {
                            "'view "
                        } else {
                            ""
                        },
                        rust_element_type(self.package, item.clone()),
                    )
                }
                (Some(ValueType::Reference(item)), true) => {
                    format!("&'a {}", rust_element_type(self.package, item.clone()))
                }
                (Some(ValueType::Reference(item)), false) if scoped_lifetime == Some("'view") => {
                    format!("&'view {}", rust_element_type(self.package, item.clone()))
                }
                _ => binding_type.map_or_else(
                    || "i128".to_owned(),
                    |value_type| rust_value_type(self.package, value_type),
                ),
            };
            let mutable = if parameter.mutable && !native_mutable_reference {
                "mut "
            } else {
                ""
            };
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
                        .binding_value_type()
                        .map(|value_type| (parameter.name.clone(), value_type))
                })
                .collect(),
        );
        let previous_active_bindings = std::mem::take(&mut self.active_function_bindings);
        self.active_function_bindings = self
            .unit
            .typed_bindings
            .iter()
            .filter(|binding| {
                binding.scope == Some(contract.span)
                    && !matches!(binding.name.as_str(), "self" | "this")
                    && !contract
                        .parameters
                        .iter()
                        .any(|parameter| parameter.span == binding.span)
            })
            .map(|binding| binding.span)
            .collect();
        for parameter in &contract.parameters {
            if matches!(
                self.unit.flow_binding_types.get(&parameter.span),
                Some(ValueType::Union(_))
            ) {
                self.active_function_bindings.insert(parameter.span);
            }
        }
        let previous_local_functions = std::mem::take(&mut self.active_local_functions);
        let mut local_functions = Vec::new();
        if let Some(block) = block {
            collect_local_functions(block, &mut local_functions);
        }
        self.active_local_functions = local_functions
            .iter()
            .map(|function| function.span)
            .collect();
        self.indent += 1;
        for function in &local_functions {
            self.local_function(function);
        }
        self.emit_parameter_union_carriers(contract);
        self.emit_function_storage(contract);
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
        self.emit_select_cursors(contract.span);
        if let Some(block) = block {
            self.block(block);
        }
        if function_errors
            && contract
                .return_type
                .clone()
                .is_none_or(|ty| ty == ValueType::Scalar(ScalarType::None))
            && block.is_none_or(block_may_fall_through)
        {
            self.line("Ok(())");
        }
        self.active_function_bindings = previous_active_bindings;
        self.active_local_functions = previous_local_functions;
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

    fn emit_function_storage(&mut self, contract: &FunctionContract) {
        let unit = self.unit;
        for binding in &unit.typed_bindings {
            let key = (binding.span.file, binding.span.start, binding.span.end);
            if !self.active_function_bindings.contains(&binding.span)
                || contract
                    .parameters
                    .iter()
                    .any(|parameter| parameter.span == binding.span)
                || unit
                    .flow_binding_ids
                    .get(&key)
                    .is_some_and(|identity| *identity != binding.span)
                || !unit
                    .flow_binding_ids
                    .values()
                    .any(|identity| *identity == binding.span)
            {
                continue;
            }
            let name = rust_binding_name(binding);
            let ty = self.binding_rust_type(
                binding,
                self.binding_scalar_storage_type(binding),
                self.reference_backed(binding),
            );
            if self.binding_may_be_unassigned(binding) {
                self.line(&format!("let mut {name}: Option<{ty}> = None;"));
            } else {
                let mutable = if binding_requires_mutable_storage(
                    self.package,
                    unit,
                    binding.span,
                    false,
                    self.local_binding_closure_writes(),
                ) {
                    "mut "
                } else {
                    ""
                };
                self.line(&format!("let {mutable}{name}: {ty};"));
            }
        }
    }

    pub(super) fn mutable_native_reference_parameter(
        &self,
        contract: &FunctionContract,
        index: usize,
        binding_type: Option<&ValueType>,
    ) -> bool {
        (contract.parameters[index].mutable
            && matches!(
                binding_type,
                Some(ValueType::Reference(item))
                    if matches!(
                        item.value_type_ref(),
                        ValueType::Object(identity)
                            if identity.native_projection.is_some()
                                || self.package.projection.item(&identity.namespace, &identity.name).is_some()
                    )
            ))
            || self
                .registry
                .mutable_native_callback_parameters
                .borrow()
                .contains(&(
                    (contract.span.file, contract.span.start, contract.span.end),
                    index,
                ))
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
        let parameter_layout = contract
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                let binding_type = parameter.binding_value_type();
                let native_mutable_reference =
                    self.mutable_native_reference_parameter(contract, index, binding_type.as_ref());
                let ty = match binding_type {
                    Some(ValueType::Reference(item)) if native_mutable_reference => {
                        format!("&mut {}", rust_element_type(self.package, item))
                    }
                    Some(value_type) => rust_value_type(self.package, value_type),
                    None => "i128".to_owned(),
                };
                let mutable = if parameter.mutable && !native_mutable_reference {
                    "mut "
                } else {
                    ""
                };
                (format!("{mutable}{}", rust_name(&parameter.name)), ty)
            })
            .collect::<Vec<_>>();
        let parameters = parameter_layout
            .iter()
            .map(|(name, ty)| format!("{name}: {ty}"))
            .collect::<Vec<_>>()
            .join(", ");
        let parameter_names = parameter_layout
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        let parameter_types = parameter_layout
            .iter()
            .map(|(_, ty)| ty.clone())
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
                        .binding_value_type()
                        .map(|ty| (parameter.name.clone(), ty))
                })
                .collect(),
        );
        let outer_async_mutable_captures = std::mem::take(&mut self.async_mutable_captures);
        let outer_fresh_empty_lists = std::mem::take(&mut self.fresh_empty_lists);
        let outer_active_bindings = std::mem::take(&mut self.active_function_bindings);
        let outer_active_local_functions = std::mem::take(&mut self.active_local_functions);
        if contract.is_async && contract.written_invocation_mode == InvocationMode::Mutable {
            self.async_mutable_captures
                .extend(contract.captures.iter().cloned());
        }
        self.closure_depth += 1;
        self.indent = outer_indent + 1;
        self.emit_select_cursors(contract.span);
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            self.active_function_bindings = self
                .unit
                .typed_bindings
                .iter()
                .filter(|binding| {
                    binding.scope == Some(contract.span)
                        && !matches!(binding.name.as_str(), "self" | "this")
                        && !contract
                            .parameters
                            .iter()
                            .any(|parameter| parameter.span == binding.span)
                })
                .map(|binding| binding.span)
                .collect();
            for parameter in &contract.parameters {
                if matches!(
                    self.unit.flow_binding_types.get(&parameter.span),
                    Some(ValueType::Union(_))
                ) {
                    self.active_function_bindings.insert(parameter.span);
                }
            }
            let mut local_functions = Vec::new();
            collect_local_functions(block, &mut local_functions);
            self.active_local_functions = local_functions
                .iter()
                .map(|function| function.span)
                .collect();
            for function in &local_functions {
                self.local_function(function);
            }
            self.emit_parameter_union_carriers(contract);
            self.emit_function_storage(contract);
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
        self.fresh_empty_lists = outer_fresh_empty_lists;
        self.active_function_bindings = outer_active_bindings;
        self.active_local_functions = outer_active_local_functions;
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
            InvocationMode::Mutable => {
                self.registry.uses_mutable_callable.set(true);
                "TerraneMutableCallable::new"
            }
            InvocationMode::Consuming => {
                self.registry.uses_consuming_callable.set(true);
                "TerraneConsumingCallable::new"
            }
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
            let binding = self.unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == *capture && binding.is_visible_at(self.source.id(), node.span.start)
            });
            let name = binding.map_or_else(
                || rust_name(capture),
                |binding| {
                    if self.unit.functions.iter().any(|function| {
                        function
                            .parameters
                            .iter()
                            .any(|parameter| parameter.span == binding.span)
                    }) {
                        rust_name(capture)
                    } else {
                        rust_binding_name(binding)
                    }
                },
            );
            let mutable = if contract.written_invocation_mode == InvocationMode::Mutable {
                "mut "
            } else {
                ""
            };
            let source = if capture == "this" { "self" } else { &name };
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
            if statement.kind == SyntaxKind::FunctionDeclaration {
                if !self.active_local_functions.contains(&statement.span) {
                    self.local_function(statement);
                }
            } else {
                self.statement(statement);
            }
        }
    }

    pub(super) fn flow_binding_type(&self, binding: &TypedBinding) -> ValueType {
        self.unit
            .flow_binding_types
            .get(&binding.span)
            .cloned()
            .unwrap_or_else(|| binding.value_type.clone())
    }

    pub(super) fn union_binding(&self, node: &SyntaxNode) -> Option<TypedBinding> {
        self.local_typed_binding(node)
            .filter(|binding| {
                !binding.destination_arms.is_empty()
                    || matches!(self.flow_binding_type(binding), ValueType::Union(_))
            })
            .cloned()
            .map(|mut binding| {
                binding.value_type = self.flow_binding_type(&binding);
                binding
            })
    }

    pub(super) fn union_arms(&self, binding: &TypedBinding) -> Vec<ValueType> {
        if let ValueType::Union(arms) = self.flow_binding_type(binding) {
            arms
        } else {
            binding
                .destination_arms
                .iter()
                .copied()
                .map(ValueType::Scalar)
                .collect()
        }
    }

    pub(super) fn union_value(&mut self, binding: &TypedBinding, value: &SyntaxNode) -> String {
        let arms = self.union_arms(binding);
        let actual = self.value_type(value);
        let selected = actual
            .as_ref()
            .and_then(|actual| arms.iter().find(|arm| *arm == actual))
            .cloned()
            .or_else(|| {
                let actual_scalar = actual.and_then(|actual| match actual {
                    ValueType::Scalar(scalar) => Some(scalar),
                    _ => None,
                });
                arms.iter().find_map(|arm| {
                    let ValueType::Scalar(scalar) = arm else {
                        return None;
                    };
                    (contextual_constant(self.source, value, *scalar).is_some()
                        || actual_scalar
                            .is_some_and(|actual| is_numeric(actual) && is_numeric(*scalar)))
                    .then(|| arm.clone())
                })
            })
            .expect("validated union destination");
        let index = arms
            .iter()
            .position(|arm| *arm == selected)
            .expect("selected union arm belongs to destination");
        format!(
            "{}::Arm{index}({})",
            union_type_name(binding),
            self.expression_as(value, selected)
        )
    }

    pub(super) fn emit_union_types(&mut self) {
        let mut emitted = std::collections::BTreeSet::new();
        let unit = self.unit;
        for binding in &unit.typed_bindings {
            if binding.destination_arms.is_empty()
                && !matches!(self.flow_binding_type(binding), ValueType::Union(_))
            {
                continue;
            }
            let name = union_type_name(binding);
            if !emitted.insert(name.clone()) {
                continue;
            }
            let arms = self.union_arms(binding);
            let arm_types = arms
                .iter()
                .map(|arm| rust_value_type(self.package, arm.clone()))
                .collect::<Vec<_>>();
            let borrowed = arm_types.iter().any(|arm| arm.contains('&'));
            let lifetime = if borrowed { "<'a>" } else { "" };
            self.line("#[allow(dead_code)]");
            if !arms.iter().any(|arm| self.value_type_owns_resource(arm)) {
                self.line("#[derive(Clone)]");
            }
            self.line(&format!("enum {name}{lifetime} {{"));
            self.indent += 1;
            for (index, arm) in arm_types.iter().enumerate() {
                let arm = if borrowed {
                    std::borrow::Cow::Owned(arm.replace('&', "&'a "))
                } else {
                    std::borrow::Cow::Borrowed(arm.as_str())
                };
                self.line(&format!("Arm{index}({arm}),"));
            }
            self.indent -= 1;
            self.line("}");
            let printable = |arm: &ValueType| {
                matches!(arm, ValueType::Scalar(scalar) if *scalar != ScalarType::Bytes)
                    || matches!(arm, ValueType::StringView(_))
            };
            if arms.iter().any(printable) {
                self.line(&format!(
                    "impl{lifetime} terrane_scalar_support::ScalarDisplay for {name}{lifetime} {{"
                ));
                self.indent += 1;
                self.line("fn write_scalar(&self, output: &mut String) {");
                self.indent += 1;
                self.line("match self {");
                self.indent += 1;
                for (index, arm) in arms.iter().enumerate() {
                    if printable(arm) {
                        self.line(&format!(
                            "Self::Arm{index}(value) => terrane_scalar_support::ScalarDisplay::write_scalar(value, output),"
                        ));
                    } else {
                        self.line(&format!(
                            "Self::Arm{index}(_) => unreachable!(\"semantic display refinement excludes this union arm\"),"
                        ));
                    }
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
}
