use super::prelude::*;

impl Emitter<'_> {
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
        reason = "object lowering emits one complete, ordered Rust object contract"
    )]
    pub(super) fn object(&mut self, node: &SyntaxNode) {
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
                let instance_fields = fields
                    .iter()
                    .copied()
                    .filter(|field| !field.is_static)
                    .collect::<Vec<_>>();
                let static_fields = fields
                    .iter()
                    .copied()
                    .filter(|field| field.is_static)
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
                    let initializer = find_node_by_span(&self.unit.tree.root, field.span)
                        .and_then(|binding| {
                            binding
                                .children
                                .iter()
                                .position(|child| child.kind == SyntaxKind::Name)
                                .and_then(|index| binding_initializer(binding, index))
                        })
                        .map_or_else(
                            || "panic!(\"static object field was not initialized\")".to_owned(),
                            |value| self.expression_as(value, field.value_type.clone()),
                        );
                    self.line(&format!(
                        "pub static {}: std::sync::LazyLock<std::sync::Mutex<{}>> = std::sync::LazyLock::new(|| std::sync::Mutex::new({initializer}));",
                        rust_static_field_name(self.package, &object.identity, &field.name),
                        rust_value_type(self.package, field.value_type.clone())
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
                    for field in &instance_fields {
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
                    for field in &instance_fields {
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
        let receiver = if contract.consumes_receiver {
            "self"
        } else if contract.mutates_receiver {
            "&mut self"
        } else {
            "&self"
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
        let receiver = if contract.consumes_receiver {
            "self"
        } else if contract.mutates_receiver {
            "&mut self"
        } else {
            "&self"
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
        self.line_start();
        let name =
            name_override.map_or_else(|| function_name(self.package, contract), str::to_owned);
        let async_main = contract.is_async && contract.name == "main" && receiver.is_none();
        write!(
            self.output,
            "{}{}fn {name}(",
            if contract.owner.is_some() || (receiver.is_none() && self.unit.bundled) {
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
            let result = return_type.clone().map_or_else(
                || "()".to_owned(),
                |value_type| rust_value_type(self.package, value_type),
            );
            write!(self.output, " -> Result<{result}, TerraneError>").unwrap();
        } else if let Some(return_type) = return_type.clone()
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
