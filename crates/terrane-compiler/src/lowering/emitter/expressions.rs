use super::super::prelude::*;

fn callable_adapter_parameters(
    package: &SemanticPackage,
    parameters: &[ElementType],
    mode: InvocationMode,
) -> (String, String, String) {
    let names = (0..parameters.len())
        .map(|index| format!("argument_{index}"))
        .collect::<Vec<_>>();
    let types = parameters
        .iter()
        .cloned()
        .map(|parameter| rust_element_type(package, parameter))
        .collect::<Vec<_>>();
    let declarations = names
        .iter()
        .zip(&types)
        .map(|(name, ty)| format!("{name}: {ty}"))
        .collect::<Vec<_>>()
        .join(", ");
    let tuple_names = match names.as_slice() {
        [] => "()".to_owned(),
        [name] => format!("({name},)"),
        _ => format!("({})", names.join(", ")),
    };
    let tuple_types = match types.as_slice() {
        [] => "()".to_owned(),
        [ty] => format!("({ty},)"),
        _ => format!("({})", types.join(", ")),
    };
    let closure_parameters = if mode == InvocationMode::Shared {
        declarations
    } else {
        format!("{tuple_names}: {tuple_types}")
    };
    (closure_parameters, names.join(", "), tuple_names)
}

fn callable_constructor(mode: InvocationMode) -> &'static str {
    match mode {
        InvocationMode::Shared => "std::sync::Arc::new",
        InvocationMode::Mutable => "TerraneMutableCallable::new",
        InvocationMode::Consuming => "TerraneConsumingCallable::new",
    }
}

impl Emitter<'_> {
    pub(super) fn descriptor_expression(&self, value_type: &ValueType) -> String {
        let descriptor = crate::semantics::materialized_descriptor(self.unit, value_type)
            .expect("every source-visible descriptor must have one canonical contract");
        let contract = descriptor.contract;
        let kind = match contract.kind {
            ObjectKind::Class => "class",
            ObjectKind::Interface => "interface",
            ObjectKind::Trait => "trait",
            ObjectKind::Type => "type",
        };
        let inherently_identity_bearing = contract.resource_owning
            || matches!(
                value_type,
                ValueType::Reference(_) | ValueType::SharedReference(_)
            );
        let fields = if contract.builtin.is_some() {
            String::new()
        } else {
            effective_object_fields(self.package, contract)
                .into_iter()
                .filter(|field| !field.is_static)
                .map(|field| {
                    format!(
                        "TerraneFieldMetadata {{ name: {:?}, external_name: {:?}, defaulted: {}, optional: {}, secret: {} }}",
                        field.name,
                        field.metadata.external_name,
                        field.metadata.defaulted,
                        field.metadata.optional,
                        field.metadata.secret
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        format!(
            "TerraneDescriptor {{ identity: {:?}, name: {:?}, kind: {kind:?}, inherently_identity_bearing: {inherently_identity_bearing}, fields: &[{fields}] }}",
            descriptor.identity, descriptor.name
        )
    }

    fn await_expression(&mut self, operand: &SyntaxNode) -> String {
        let awaited = format!("__terrane_await({}).await", self.expression(operand));
        let callee = (operand.kind == SyntaxKind::CallExpression)
            .then(|| operand.children.first())
            .flatten();
        let contract = callee.and_then(|callee| self.contract_for_call(callee));
        let projected = callee.is_some_and(|callee| {
            if callee.kind == SyntaxKind::Name {
                return self
                    .package
                    .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                    .and_then(|symbol| symbol.identity.rsplit_once("::"))
                    .and_then(|(namespace, name)| self.package.projection.item(namespace, name))
                    .is_some_and(|item| {
                        matches!(&item.kind, crate::projection::ProjectedKind::Function(_))
                    });
            }
            let Some(contract) = contract else {
                return false;
            };
            let Some(receiver) = callee.children.first() else {
                return false;
            };
            let Some(ValueType::Object(identity)) = self.value_type(receiver) else {
                return false;
            };
            self.package
                .projection
                .method(&identity.namespace, &identity.name, &contract.name, false)
                .is_some()
        });
        let function_value_throws = callee
            .and_then(|callee| self.value_type(callee))
            .is_some_and(|value_type| match value_type {
                ValueType::AsyncFunction(_, _, _, effects) => effects.requires_throwing_abi(),
                _ => false,
            });
        let throws =
            projected || contract.is_some_and(|contract| contract.throws) || function_value_throws;
        if !throws {
            return awaited;
        }
        let site = self.error_site(operand);
        if self.try_completion {
            format!("__terrane_traced_completion!({awaited}, {site})")
        } else if self.propagate_errors {
            format!("__terrane_traced_err({awaited}, {site})?")
        } else {
            format!("__terrane_traced({awaited}, {site})")
        }
    }

    pub(super) fn expression(&mut self, node: &SyntaxNode) -> String {
        match node.kind {
            SyntaxKind::Literal => literal(self.text(node)),
            SyntaxKind::AnonymousFunction => self.anonymous_function(node),
            SyntaxKind::Name if self.text(node) == "self" => {
                let identity = self
                    .current_object
                    .as_ref()
                    .expect("`self` is only lowered in an object method");
                self.descriptor_expression(&ValueType::Object(identity.clone()))
            }
            SyntaxKind::Name => {
                let name = self.text(node);
                if self.async_mutable_captures.contains(name) {
                    let name = rust_name(name);
                    if self.assignment_target {
                        return name;
                    }
                    return format!("{name}.snapshot()");
                }
                let binding = self.unit.typed_bindings.iter().rev().find(|binding| {
                    binding.name == name
                        && binding.is_visible_at(self.unit.source.id(), node.span.start)
                });
                match self.value_type(node) {
                    Some(value_type @ ValueType::Descriptor(_))
                        if binding.is_none_or(|binding| binding.scope.is_none()) =>
                    {
                        self.descriptor_expression(&value_type)
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
                        Some(ValueType::Reference(_)) => self.expression(operand),
                        _ => self.reference_address_expression(operand),
                    };
                }
                if source_operator == "shared ref" {
                    return match self.value_type(operand) {
                        Some(ValueType::SharedReference(_)) => {
                            format!("({}).clone()", self.expression(operand))
                        }
                        Some(ValueType::Reference(_)) => unreachable!(
                            "semantic analysis rejects promoting a non-owning ref to shared ownership"
                        ),
                        _ => self.reference_storage_expression(operand),
                    };
                }
                if source_operator == "await" {
                    return self.await_expression(operand);
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
            SyntaxKind::StaticMemberExpression => self.static_member(node),
            SyntaxKind::ConstructionExpression => Self::escaped_construction(node),
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
    pub(in crate::lowering) fn expression_as(
        &mut self,
        node: &SyntaxNode,
        value_type: ValueType,
    ) -> String {
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
        if !self.assignment_target
            && let Some(actual) = self.value_type(node)
            && let ValueType::Reference(item) | ValueType::SharedReference(item) = actual
            && item.value_type() == value_type
        {
            return self.receiver_expression(node);
        }
        if !self.assignment_target
            && let Some(ValueType::Optional(inner)) = self.value_type(node)
            && *inner == value_type
        {
            return format!(
                "({}).expect(\"semantic optional narrowing\")",
                self.expression(node)
            );
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
                let expression = self.expression(node);
                if node.kind == SyntaxKind::MemberExpression
                    && !rust_value_is_copy(inner.as_ref())
                    && let [receiver, member] = node.children.as_slice()
                    && self.object_field(receiver, self.text(member))
                {
                    format!("({expression}).clone()")
                } else {
                    expression
                }
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
        if matches!(value_type, ValueType::Reference(_))
            && self.value_type(node) == Some(value_type.clone())
        {
            return self.expression(node);
        }
        if matches!(value_type, ValueType::SharedReference(_)) {
            if node.kind == SyntaxKind::UnaryExpression {
                return self.expression(node);
            }
            let existing_storage = match node.kind {
                SyntaxKind::Name
                | SyntaxKind::MemberExpression
                | SyntaxKind::StaticMemberExpression
                | SyntaxKind::IndexExpression
                | SyntaxKind::CallExpression => self.value_type(node) == Some(value_type.clone()),
                SyntaxKind::GroupExpression => node
                    .children
                    .first()
                    .is_some_and(|child| self.value_type(child) == Some(value_type.clone())),
                _ => false,
            };
            return if existing_storage {
                format!("({}).clone()", self.expression(node))
            } else {
                self.reference_storage_expression(node)
            };
        }
        if let ValueType::Object(expected) = &value_type
            && self.text(node) == "this"
            && let Some(actual) = &self.current_object
            && let Some(destination) = self
                .unit
                .descriptors
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
                .descriptors
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
                                    unit.descriptors
                                        .iter()
                                        .find(|object| object.name == symbol.name)
                                })
                        })
                })
                .or_else(|| {
                    self.package.units.iter().find_map(|unit| {
                        unit.descriptors.iter().find(|object| {
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
            && node.children.first().is_none_or(|receiver| {
                !matches!(self.value_type(receiver), Some(ValueType::Descriptor(_)))
            })
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
            ValueType::Scalar(ScalarType::String)
                if node.kind == SyntaxKind::Name && self.binding_value_is_reused(node) =>
            {
                format!("({}).clone()", self.expression(node))
            }
            value_type @ ValueType::List(_)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::list") =>
            {
                rust_empty_collection(self.package, &value_type)
                    .expect("list is an empty collection constructor")
            }
            value_type @ ValueType::Tuple(_, _)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::tuple") =>
            {
                rust_empty_collection(self.package, &value_type)
                    .expect("tuple is an empty collection constructor")
            }
            value_type @ ValueType::Set(_)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::set") =>
            {
                rust_empty_collection(self.package, &value_type)
                    .expect("set is an empty collection constructor")
            }
            value_type @ ValueType::UnorderedSet(_)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::unordered-set") =>
            {
                rust_empty_collection(self.package, &value_type)
                    .expect("unordered set is an empty collection constructor")
            }
            value_type @ ValueType::Map(_, _)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::map") =>
            {
                rust_empty_collection(self.package, &value_type)
                    .expect("map is an empty collection constructor")
            }
            value_type @ ValueType::UnorderedMap(_, _)
                if node.kind == SyntaxKind::Name
                    && self.is_builtin(node, "/core/collections::unordered-map") =>
            {
                rust_empty_collection(self.package, &value_type)
                    .expect("unordered map is an empty collection constructor")
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
            ValueType::AsyncFunction(parameters, _, transferability, expected_effects)
                if node.kind == SyntaxKind::Name =>
            {
                let expected_mode = expected_effects.modes.written;
                let (declarations, arguments, tuple_arguments) =
                    callable_adapter_parameters(self.package, &parameters, expected_mode);
                let constructor = callable_constructor(expected_mode);
                let send = if transferability == TaskTransferability::Transferable {
                    " + Send"
                } else {
                    ""
                };
                let expected_throws = expected_effects.requires_throwing_abi();
                let actual = self.value_type(node);
                let (actual_mode, value_requires_throwing_abi) = match &actual {
                    Some(ValueType::AsyncFunction(_, _, _, effects)) => {
                        (effects.modes.written, effects.requires_throwing_abi())
                    }
                    _ => (InvocationMode::Shared, false),
                };
                if let Some(contract) = self.contract_for_call(node) {
                    let function = function_name(self.package, contract);
                    let actual_throws =
                        self.contract_requires_throwing_abi(contract, value_requires_throwing_abi);
                    let future = if expected_throws && !actual_throws {
                        format!("Box::pin(async move {{ Ok({function}({arguments}).await) }})")
                    } else {
                        format!("Box::pin({function}({arguments}))")
                    };
                    format!(
                        "{constructor}(move |{declarations}| -> std::pin::Pin<Box<dyn Future<Output = _>{send}>> {{ {future} }})"
                    )
                } else if actual_mode == expected_mode
                    && (!expected_throws || value_requires_throwing_abi)
                {
                    let callable = self.expression(node);
                    if expected_mode == InvocationMode::Consuming {
                        callable
                    } else {
                        format!("({callable}).clone()")
                    }
                } else {
                    let callable = self.expression(node);
                    let capture = if actual_mode == InvocationMode::Consuming {
                        callable
                    } else {
                        format!("({callable}).clone()")
                    };
                    let invocation = if actual_mode == InvocationMode::Shared {
                        format!("callable({arguments})")
                    } else {
                        format!("callable.call({tuple_arguments})")
                    };
                    let future = if expected_throws && !value_requires_throwing_abi {
                        format!(
                            "{{ let callable_future = {invocation}; Box::pin(async move {{ Ok(callable_future.await) }}) }}"
                        )
                    } else {
                        format!("Box::pin({invocation})")
                    };
                    format!(
                        "{{ let callable = {capture}; {constructor}(move |{declarations}| -> std::pin::Pin<Box<dyn Future<Output = _>{send}>> {{ {future} }}) }}"
                    )
                }
            }
            ValueType::AsyncFunction(parameters, _, transferability, expected_effects)
                if node.kind == SyntaxKind::MemberExpression =>
            {
                let [receiver, member] = node.children.as_slice() else {
                    return String::new();
                };
                let receiver_type = self
                    .value_type(receiver)
                    .expect("bound object method receiver must have a static type");
                let receiver = self.expression_as(receiver, receiver_type);
                let expected_mode = expected_effects.modes.written;
                let method_contract = self.contract_for_call(node);
                let actual_mode = method_contract.map_or(InvocationMode::Shared, |contract| {
                    contract.written_invocation_mode
                });
                let (declarations, arguments, _) =
                    callable_adapter_parameters(self.package, &parameters, expected_mode);
                let constructor = callable_constructor(expected_mode);
                let send = if transferability == TaskTransferability::Transferable {
                    " + Send"
                } else {
                    ""
                };
                let expected_throws = expected_effects.requires_throwing_abi();
                let value_requires_throwing_abi = matches!(
                    self.value_type(node),
                    Some(ValueType::AsyncFunction(_, _, _, effects))
                        if effects.requires_throwing_abi()
                );
                let actual_throws =
                    method_contract.map_or(value_requires_throwing_abi, |contract| {
                        self.contract_requires_throwing_abi(contract, value_requires_throwing_abi)
                    });
                let method = rust_name(self.text(member));
                let (capture, invocation, call) = match actual_mode {
                    InvocationMode::Shared => (
                        format!("std::sync::Arc::new({receiver})"),
                        "let receiver = receiver.clone(); ",
                        format!("receiver.{method}({arguments}).await"),
                    ),
                    InvocationMode::Mutable => (
                        format!("TerraneAsyncMutableState::new({receiver})"),
                        "let receiver = receiver.share(); ",
                        format!(
                            "receiver.with_receiver(move |receiver| Box::pin(async move {{ receiver.{method}({arguments}).await }})).await"
                        ),
                    ),
                    InvocationMode::Consuming => (
                        receiver,
                        "",
                        format!("receiver.{method}({arguments}).await"),
                    ),
                };
                let body = if expected_throws && !actual_throws {
                    format!("Ok({call})")
                } else {
                    call
                };
                format!(
                    "{{ let receiver = {capture}; {constructor}(move |{declarations}| -> std::pin::Pin<Box<dyn Future<Output = _>{send}>> {{ {invocation}Box::pin(async move {{ {body} }}) }}) }}"
                )
            }
            ValueType::Function(parameters, _, expected_effects)
                if node.kind == SyntaxKind::MemberExpression =>
            {
                let [receiver, member] = node.children.as_slice() else {
                    return String::new();
                };
                let callable_field = self.callable_object_field(receiver, self.text(member));
                let value_type = self.value_type(node);
                let value_requires_throwing_abi = matches!(
                    &value_type,
                    Some(ValueType::Function(_, _, effects)) if effects.requires_throwing_abi()
                );
                let method_contract = self.contract_for_call(node);
                let actual_throws =
                    method_contract.map_or(value_requires_throwing_abi, |contract| {
                        self.contract_requires_throwing_abi(contract, value_requires_throwing_abi)
                    });
                let actual_mode = method_contract.map_or_else(
                    || match &value_type {
                        Some(ValueType::Function(_, _, effects)) => effects.modes.written,
                        _ => InvocationMode::Shared,
                    },
                    |contract| contract.written_invocation_mode,
                );
                let expected_mode = expected_effects.modes.written;
                let expected_throws = expected_effects.requires_throwing_abi();
                let receiver_type = self
                    .value_type(receiver)
                    .expect("bound object method receiver must have a static type");
                let receiver = self.expression_as(receiver, receiver_type.clone());
                let (declarations, arguments, tuple_arguments) =
                    callable_adapter_parameters(self.package, &parameters, expected_mode);
                let argument_values = (0..parameters.len())
                    .map(|index| format!("argument_{index}"))
                    .collect::<Vec<_>>();
                let constructor = callable_constructor(expected_mode);
                let mutable = if actual_mode == InvocationMode::Mutable {
                    "mut "
                } else {
                    ""
                };
                if let ValueType::Scalar(float_type @ (ScalarType::Float32 | ScalarType::Float64)) =
                    receiver_type
                    && let Some(body) = self.float_call(
                        float_type,
                        self.text(member),
                        "receiver",
                        &argument_values,
                        node,
                    )
                {
                    let body = if expected_throws {
                        format!("Ok({body})")
                    } else {
                        body
                    };
                    return format!(
                        "{{ let {mutable}receiver = {receiver}; {constructor}(move |{declarations}| {body}) }}"
                    );
                }
                let call = if callable_field && actual_mode != InvocationMode::Shared {
                    format!(
                        "receiver.{}.call({tuple_arguments})",
                        rust_name(self.text(member))
                    )
                } else if callable_field {
                    format!("(receiver.{})({arguments})", rust_name(self.text(member)))
                } else {
                    format!("receiver.{}({arguments})", rust_name(self.text(member)))
                };
                let body = if expected_throws && !actual_throws {
                    format!("Ok({call})")
                } else {
                    call
                };
                format!(
                    "{{ let {mutable}receiver = {receiver}; {constructor}(move |{declarations}| {body}) }}"
                )
            }
            ValueType::Function(parameters, _, expected_effects)
                if node.kind == SyntaxKind::Name =>
            {
                let expected_mode = expected_effects.modes.written;
                let (declarations, arguments, tuple_arguments) =
                    callable_adapter_parameters(self.package, &parameters, expected_mode);
                let constructor = callable_constructor(expected_mode);
                let expected_throws = expected_effects.requires_throwing_abi();
                let actual = self.value_type(node);
                let (actual_mode, value_requires_throwing_abi) = match &actual {
                    Some(ValueType::Function(_, _, effects)) => {
                        (effects.modes.written, effects.requires_throwing_abi())
                    }
                    _ => (InvocationMode::Shared, false),
                };
                if let Some(contract) = self.contract_for_call(node) {
                    let function = function_name(self.package, contract);
                    let actual_throws =
                        self.contract_requires_throwing_abi(contract, value_requires_throwing_abi);
                    if expected_mode == InvocationMode::Shared
                        && (!expected_throws || actual_throws)
                    {
                        format!("std::sync::Arc::new({function})")
                    } else {
                        let call = format!("{function}({arguments})");
                        let body = if expected_throws && !actual_throws {
                            format!("Ok({call})")
                        } else {
                            call
                        };
                        format!("{constructor}(move |{declarations}| {body})")
                    }
                } else if actual_mode == expected_mode
                    && (!expected_throws || value_requires_throwing_abi)
                {
                    let callable = self.expression(node);
                    if expected_mode == InvocationMode::Consuming {
                        callable
                    } else {
                        format!("({callable}).clone()")
                    }
                } else {
                    let callable = self.expression(node);
                    let capture = if actual_mode == InvocationMode::Consuming {
                        callable
                    } else {
                        format!("({callable}).clone()")
                    };
                    let call = if actual_mode == InvocationMode::Shared {
                        format!("callable({arguments})")
                    } else {
                        format!("callable.call({tuple_arguments})")
                    };
                    let body = if expected_throws && !value_requires_throwing_abi {
                        format!("Ok({call})")
                    } else {
                        call
                    };
                    format!(
                        "{{ let callable = {capture}; {constructor}(move |{declarations}| {body}) }}"
                    )
                }
            }
            ValueType::AsyncFunction(parameters, _, transferability, expected_effects) => {
                let callable = self.expression(node);
                let actual = self.value_type(node);
                let (actual_mode, actual_throws) = match &actual {
                    Some(ValueType::AsyncFunction(_, _, _, effects)) => {
                        (effects.modes.written, effects.requires_throwing_abi())
                    }
                    _ => (InvocationMode::Shared, false),
                };
                let expected_mode = expected_effects.modes.written;
                let expected_throws = expected_effects.requires_throwing_abi();
                if actual_mode == expected_mode && (!expected_throws || actual_throws) {
                    callable
                } else {
                    let (declarations, arguments, tuple_arguments) =
                        callable_adapter_parameters(self.package, &parameters, expected_mode);
                    let constructor = callable_constructor(expected_mode);
                    let capture = if actual_mode == InvocationMode::Consuming {
                        callable
                    } else {
                        format!("({callable}).clone()")
                    };
                    let invocation = if actual_mode == InvocationMode::Shared {
                        format!("callable({arguments})")
                    } else {
                        format!("callable.call({tuple_arguments})")
                    };
                    let future = if expected_throws && !actual_throws {
                        format!(
                            "{{ let callable_future = {invocation}; Box::pin(async move {{ Ok(callable_future.await) }}) }}"
                        )
                    } else {
                        format!("Box::pin({invocation})")
                    };
                    let send = if transferability == TaskTransferability::Transferable {
                        " + Send"
                    } else {
                        ""
                    };
                    format!(
                        "{{ let callable = {capture}; {constructor}(move |{declarations}| -> std::pin::Pin<Box<dyn Future<Output = _>{send}>> {{ {future} }}) }}"
                    )
                }
            }
            ValueType::Function(parameters, _, expected_effects) => {
                let callable = self.expression(node);
                let actual = self.value_type(node);
                let (actual_mode, actual_throws) = match &actual {
                    Some(ValueType::Function(_, _, effects)) => {
                        (effects.modes.written, effects.requires_throwing_abi())
                    }
                    _ => (InvocationMode::Shared, false),
                };
                let expected_mode = expected_effects.modes.written;
                let expected_throws = expected_effects.requires_throwing_abi();
                if actual_mode == expected_mode && (!expected_throws || actual_throws) {
                    callable
                } else {
                    let (declarations, arguments, tuple_arguments) =
                        callable_adapter_parameters(self.package, &parameters, expected_mode);
                    let constructor = callable_constructor(expected_mode);
                    let capture = if actual_mode == InvocationMode::Consuming {
                        callable
                    } else {
                        format!("({callable}).clone()")
                    };
                    let call = if actual_mode == InvocationMode::Shared {
                        format!("callable({arguments})")
                    } else {
                        format!("callable.call({tuple_arguments})")
                    };
                    let body = if expected_throws && !actual_throws {
                        format!("Ok({call})")
                    } else {
                        call
                    };
                    format!(
                        "{{ let callable = {capture}; {constructor}(move |{declarations}| {body}) }}"
                    )
                }
            }
            _ => self.expression(node),
        }
    }

    pub(super) fn map_constructor(
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

    pub(super) fn adaptive_expression(&mut self, node: &SyntaxNode) -> String {
        match node.kind {
            SyntaxKind::Literal => adaptive_literal(self.text(node)),
            SyntaxKind::Name if self.async_mutable_captures.contains(self.text(node)) => {
                self.expression(node)
            }
            SyntaxKind::Name if self.lazy_namespace_binding_type(node).is_some() => {
                format!("(*{}).clone()", self.namespace_name(node))
            }
            SyntaxKind::Name if self.small_int_binding(node).is_some() => {
                format!(
                    "terrane_int_support::Int::from(({}) as i128)",
                    self.name(node)
                )
            }
            SyntaxKind::Name => format!("({}).clone()", self.name(node)),
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
                    self.await_expression(operand)
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

    pub(super) fn adaptive_binary(&mut self, node: &SyntaxNode) -> String {
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

    pub(super) fn adaptive_binary_as(&mut self, node: &SyntaxNode) -> String {
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

    pub(super) fn is_adaptive_expression(&self, node: &SyntaxNode) -> bool {
        self.value_type(node) == Some(ValueType::Scalar(ScalarType::Int))
    }

    pub(super) fn numeric_operation_type(
        &self,
        left: &SyntaxNode,
        right: &SyntaxNode,
    ) -> Option<ScalarType> {
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

    pub(super) fn optional_none_comparison(
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

    #[expect(
        clippy::too_many_lines,
        reason = "binary lowering keeps operand effects and operator selection in source order"
    )]
    pub(super) fn binary(&mut self, node: &SyntaxNode) -> String {
        let [left, right] = node.children.as_slice() else {
            return String::new();
        };
        let source_operator = self.source.text()[left.span.end..right.span.start].trim();
        if source_operator == "is" {
            let left_type = self.value_type(left);
            let right_type = self.value_type(right);
            let reference_items = match (&left_type, &right_type) {
                (
                    Some(ValueType::Reference(left) | ValueType::SharedReference(left)),
                    Some(ValueType::Reference(right) | ValueType::SharedReference(right)),
                ) => Some((left, right)),
                _ => None,
            };
            if let Some((left_item, right_item)) = reference_items
                && left_item == right_item
            {
                let left_expression = self.expression(left);
                let right_expression = self.expression(right);
                let left_pointer = if matches!(left_type, Some(ValueType::Reference(_))) {
                    if self.reference_uses_shared_storage(left) {
                        "std::sync::Weak::as_ptr(__terrane_identity_left)"
                    } else {
                        "std::ptr::from_ref(*__terrane_identity_left)"
                    }
                } else {
                    "std::sync::Arc::as_ptr(__terrane_identity_left)"
                };
                let right_pointer = if matches!(right_type, Some(ValueType::Reference(_))) {
                    if self.reference_uses_shared_storage(right) {
                        "std::sync::Weak::as_ptr(__terrane_identity_right)"
                    } else {
                        "std::ptr::from_ref(*__terrane_identity_right)"
                    }
                } else {
                    "std::sync::Arc::as_ptr(__terrane_identity_right)"
                };
                return format!(
                    "{{ let __terrane_identity_left = &({left_expression}); let __terrane_identity_right = &({right_expression}); std::ptr::eq({left_pointer}, {right_pointer}) }}"
                );
            }
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

    pub(super) fn type_membership(&mut self, node: &SyntaxNode) -> String {
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
                .filter(|(_, arm)| {
                    crate::semantics::descriptor_conforms_to(
                        self.unit,
                        &ValueType::Scalar(**arm),
                        category,
                    )
                })
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
    pub(super) fn category_membership(
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
            let conforms = crate::semantics::descriptor_conforms_to(self.unit, &inner, category);
            return if conforms {
                format!("({value}).is_some()")
            } else {
                format!("{{ let _ = {value}; false }}")
            };
        }
        let result = value_type.as_ref().is_some_and(|value_type| {
            crate::semantics::descriptor_conforms_to(self.unit, value_type, category)
        });
        let effect = if node.kind == SyntaxKind::Name {
            let expression = Self::unwrapped_expression(self.expression(node));
            format!("let _ = &{expression};")
        } else {
            Self::discarded_expression(self.expression(node))
        };
        format!("{{ {effect} {result} }}")
    }

    pub(super) fn identity_operand_effect(&mut self, node: &SyntaxNode) -> Option<String> {
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

    pub(super) fn unwrapped_expression(mut expression: String) -> String {
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

    pub(super) fn discarded_expression(expression: String) -> String {
        format!("let _ = {};", Self::unwrapped_expression(expression))
    }

    pub(super) fn is_only_binding_use(&self, node: &SyntaxNode) -> bool {
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
    fn binding_value_is_reused(&self, node: &SyntaxNode) -> bool {
        let name = self.text(node);
        self.unit
            .typed_bindings
            .iter()
            .rev()
            .find(|binding| {
                binding.name == name
                    && binding.is_visible_at(self.unit.source.id(), node.span.start)
            })
            .is_some_and(|binding| {
                binding_read_value_is_reused(self.package, binding.span, node.span)
            })
    }

    pub(in crate::lowering) fn value_type(&self, node: &SyntaxNode) -> Option<ValueType> {
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
}
