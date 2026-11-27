use super::super::prelude::*;
use super::native_bridge::{InvocationSite, NativeBridgeCall};

#[cfg(test)]
mod tests;

pub(super) fn list_sort_comparator(item: &ElementType, descending: bool) -> &'static str {
    match (item.value_type(), descending) {
        (ValueType::Scalar(ScalarType::Float32), false) => {
            "terrane_collection_support::compare_float32_ascending"
        }
        (ValueType::Scalar(ScalarType::Float32), true) => {
            "terrane_collection_support::compare_float32_descending"
        }
        (ValueType::Scalar(ScalarType::Float64), false) => {
            "terrane_collection_support::compare_float64_ascending"
        }
        (ValueType::Scalar(ScalarType::Float64), true) => {
            "terrane_collection_support::compare_float64_descending"
        }
        (ValueType::Scalar(_), false) => "|left, right| left.cmp(right)",
        (ValueType::Scalar(_), true) => "|left, right| right.cmp(left)",
        _ => unreachable!("semantic analysis admits only sortable scalar list items"),
    }
}

// Private host intrinsics have no source FunctionContract. Keep their ABI
// parameter names alongside their resolved identity, rather than pretending
// that source-call selection can supply a contract.
fn intrinsic_arguments<'a>(
    source: &crate::source::SourceFile,
    arguments: &'a SyntaxNode,
    parameters: &[&str],
) -> Vec<Option<&'a SyntaxNode>> {
    let mut ordered = vec![None; parameters.len()];
    let mut positional = 0;
    for argument in &arguments.children {
        let named = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let index = if let Some(name) = named {
            parameters
                .iter()
                .position(|parameter| *parameter == &source.text()[name.span.start..name.span.end])
        } else {
            let index = positional;
            positional += 1;
            Some(index)
        }
        .expect("private host argument names match their ABI");
        ordered[index] = Some(argument.children.last().unwrap_or(argument));
    }
    ordered
}

impl Emitter<'_> {
    fn typed_document_decode_call(
        &mut self,
        node: &SyntaxNode,
        identity: &str,
        arguments: &SyntaxNode,
    ) -> String {
        let values = arguments
            .children
            .iter()
            .map(|argument| argument.children.last().unwrap_or(argument))
            .collect::<Vec<_>>();
        let (source_setup, source) = self.operand_reference_parts(values[0], false);
        let ValueType::DocumentDecodeOutcome(destination) =
            self.value_type(node).expect("typed decode outcome")
        else {
            unreachable!("typed document semantics retained the destination type")
        };
        let destination = destination.value_type();
        let destination_type = rust_value_type(self.package, destination.clone());
        let mut fallback_operand = values[1];
        while fallback_operand.kind == SyntaxKind::GroupExpression
            && let [inner] = fallback_operand.children.as_slice()
        {
            fallback_operand = inner;
        }
        let (fallback_setup, fallback) =
            if matches!(self.value_type(values[1]), Some(ValueType::Descriptor(_))) {
                let fallback = if matches!(destination, ValueType::Object(_)) {
                    format!("{destination_type}::terrane_construct()")
                } else if matches!(destination, ValueType::Scalar(ScalarType::Int)) {
                    adaptive_literal("0")
                } else {
                    format!("<{destination_type}>::default()")
                };
                (String::new(), fallback)
            } else if matches!(destination, ValueType::Scalar(ScalarType::Int))
                && !matches!(
                    fallback_operand.kind,
                    SyntaxKind::Name | SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
                )
            {
                // A literal's borrowed read form may be a Rust integer primitive.
                // Materialize the canonical adaptive representation before capture.
                let value = self.expression_as(values[1], destination.clone());
                (
                    format!("let __terrane_document_fallback = {value}; "),
                    "__terrane_document_fallback.clone()".to_owned(),
                )
            } else {
                let (setup, value) = self.operand_reference_parts(values[1], false);
                (
                    format!("{setup}let __terrane_document_fallback = {value}; "),
                    "(*__terrane_document_fallback).clone()".to_owned(),
                )
            };
        let shape = crate::lowering::helpers::rust_document_shape(&destination);
        let (options_setup, options) = self.operand_reference_parts(values[2], false);
        let allow_unknown = self.expression_as(values[3], ValueType::Scalar(ScalarType::Bool));
        let (line, column) = self.source.line_column(node.span.start);
        let source_site = format!("{}:{line}:{column}", self.unit.source_path);
        let expected = destination.to_string();
        let parse = if identity == "/core/documents/json::decode-typed-json" {
            "terrane_document_support::parse_json_document(__terrane_document_source, terrane_limit(&__terrane_document_options.max_depth), terrane_limit(&__terrane_document_options.max_bytes))".to_owned()
        } else {
            "terrane_document_support::parse_yaml_document(__terrane_document_source, terrane_limit(&__terrane_document_options.max_depth), terrane_limit(&__terrane_document_options.max_bytes), terrane_limit(&__terrane_document_options.max_alias_nodes))".to_owned()
        };
        format!(
            "{{ {source_setup}let __terrane_document_source = {source}; {fallback_setup}{options_setup}let __terrane_document_options = {options}; let __terrane_document_allow_unknown = {allow_unknown}; __terrane_decode_document::<{destination_type}>({parse}, || {fallback}, __terrane_document_allow_unknown, {source_site:?}, {expected:?}, {shape}) }}"
        )
    }
    fn typed_document_encode_call(&mut self, node: &SyntaxNode, arguments: &SyntaxNode) -> String {
        let values = arguments
            .children
            .iter()
            .map(|argument| argument.children.last().unwrap_or(argument))
            .collect::<Vec<_>>();
        let (source_setup, source) = self.operand_reference_parts(values[0], false);
        let source_type = self
            .value_type(values[0])
            .expect("typed encoder source has a type");
        let expected = source_type.to_string();
        let (options_setup, options) = self.operand_reference_parts(values[1], false);
        let (line, column) = self.source.line_column(node.span.start);
        let source_site = format!("{}:{line}:{column}", self.unit.source_path);
        format!(
            "{{ {source_setup}let __terrane_document_source = {source}; {options_setup}let __terrane_document_options = {options}; __terrane_encode_document(__terrane_document_source, terrane_limit(&__terrane_document_options.max_depth), terrane_limit(&__terrane_document_options.max_bytes), {source_site:?}, {expected:?}) }}"
        )
    }
    fn structured_log_emit_call(
        &mut self,
        node: &SyntaxNode,
        identity: &str,
        arguments: &SyntaxNode,
    ) -> String {
        let mut values = arguments
            .children
            .iter()
            .map(|argument| {
                let value = argument.children.last().unwrap_or(argument);
                let value_type = self
                    .value_type(value)
                    .expect("semantic structured log argument has a value type");
                self.expression_as(value, value_type)
            })
            .collect::<Vec<_>>();
        if identity != "/core/logging::emit" {
            let level = identity
                .rsplit_once("::")
                .map(|(_, name)| name)
                .expect("a structured log operation has a qualified identity");
            values.insert(1, format!("{level}_level()"));
        }
        let (line, column) = self.source.line_column(node.span.start);
        values.insert(
            3,
            format!(
                "{:?}.to_owned()",
                format!("{}:{line}:{column}", self.unit.source_path)
            ),
        );
        format!("emit_at({})", values.join(", "))
    }

    pub(super) fn call(&mut self, node: &SyntaxNode) -> String {
        self.lower_bridge_call(node, false)
    }

    pub(super) fn interpolation_renderer_call(&mut self, node: &SyntaxNode) -> String {
        self.lower_bridge_call(node, true)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "all call forms share one ordering and error-propagation path"
    )]
    pub(super) fn call_with_order(
        &mut self,
        node: &SyntaxNode,
        argument_first: bool,
        bridge: &NativeBridgeCall<'_>,
    ) -> String {
        #[cfg(test)]
        tests::count_call(node.span.file);
        let [source_callee, arguments] = node.children.as_slice() else {
            return String::new();
        };
        let projected_application = bridge.operation.is_some();
        let mut callee = source_callee;
        let mut application = source_callee;
        while matches!(
            application.kind,
            SyntaxKind::GroupExpression | SyntaxKind::TypeExpression
        ) {
            let Some(inner) = application.children.first() else {
                break;
            };
            application = inner;
        }
        let source_method_application = (!projected_application
            && application.kind == SyntaxKind::AppliedType
            && application
                .children
                .first()
                .is_some_and(|base| base.kind == SyntaxKind::MemberExpression))
        .then_some(application);
        if projected_application || source_method_application.is_some() {
            while matches!(
                callee.kind,
                SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
            ) {
                let Some(inner) = callee.children.first() else {
                    break;
                };
                callee = inner;
            }
        }
        if callee.kind == SyntaxKind::ConstructionExpression
            && let Some(designator) = callee.children.first()
            && let Some(value) = self.source_enum_construction(designator, arguments, node)
        {
            return value;
        }
        if callee.kind == SyntaxKind::Name
            && let Some(identity) = self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .map(|symbol| symbol.identity.clone())
            && matches!(
                identity.as_str(),
                "/core/documents/json::decode-typed-json"
                    | "/core/documents/yaml::decode-typed-yaml"
            )
        {
            return self.typed_document_decode_call(node, &identity, arguments);
        }
        if callee.kind == SyntaxKind::Name
            && let Some(identity) = self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .map(|symbol| symbol.identity.clone())
            && matches!(
                identity.as_str(),
                "/core/documents/json::encode-typed-json"
                    | "/core/documents/yaml::encode-typed-yaml"
            )
        {
            return self.typed_document_encode_call(node, arguments);
        }
        if callee.kind == SyntaxKind::Name
            && let Some(identity) = self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .map(|symbol| symbol.identity.clone())
            && matches!(
                identity.as_str(),
                "/core/logging::emit"
                    | "/core/logging::debug"
                    | "/core/logging::info"
                    | "/core/logging::warning"
                    | "/core/logging::error"
            )
        {
            return self.structured_log_emit_call(node, &identity, arguments);
        }
        if callee.kind == SyntaxKind::Name
            && let Some(identity) = self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .map(|symbol| symbol.identity.clone())
            && matches!(
                identity.as_str(),
                "/core/logging::field" | "/core/logging::secret-field"
            )
        {
            let values = arguments
                .children
                .iter()
                .map(|argument| argument.children.last().unwrap_or(argument))
                .collect::<Vec<_>>();
            let name = self.expression_as(values[0], ValueType::Scalar(ScalarType::String));
            let value = self.expression_as(
                values[1],
                ValueType::Object(ObjectIdentity::new("/core/logging", "log-value")),
            );
            let secret = identity.ends_with("::secret-field");
            let (line, column) = self.source.line_column(node.span.start);
            let source = format!("{}:{line}:{column}", self.unit.source_path);
            return format!("field_at({name}, {value}, {secret}, {source:?}.to_owned())");
        }
        if callee.kind == SyntaxKind::Name
            && self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .is_some_and(|symbol| symbol.identity == "/core/logging::make-event")
        {
            let values = arguments
                .children
                .iter()
                .map(|argument| {
                    let value = argument.children.last().unwrap_or(argument);
                    let value_type = self
                        .value_type(value)
                        .expect("semantic log event argument has a value type");
                    self.expression_as(value, value_type)
                })
                .collect::<Vec<_>>();
            let (line, column) = self.source.line_column(node.span.start);
            let source = format!("{}:{line}:{column}", self.unit.source_path);
            return format!(
                "make_event_at({}, {}, {source:?}.to_owned(), {})",
                values[0], values[1], values[2]
            );
        }
        if callee.kind == SyntaxKind::Name
            && self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .is_some_and(|symbol| symbol.identity == "/core/logging::log-error")
        {
            let argument = arguments
                .children
                .first()
                .and_then(|argument| argument.children.last())
                .expect("semantic logging error adapter has one argument");
            let value_type = self
                .value_type(argument)
                .expect("semantic logging error adapter has a throwable argument");
            let value = self.expression_as(argument, value_type);
            return format!("terrane_log_error({value})");
        }
        if callee.kind == SyntaxKind::Name
            && self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .is_some_and(|symbol| symbol.identity == "/core/async::task-group")
            && let Some(ValueType::TaskGroup(item)) = self.value_type(node)
        {
            return format!(
                "TerraneTaskGroup::<{}>::new()",
                rust_element_type(self.package, item)
            );
        }
        if callee.kind == SyntaxKind::Name && self.text(callee) == "task-scope" {
            let deadline = arguments.children.first().map_or_else(
                || "None".to_owned(),
                |argument| {
                    let value = argument.children.last().unwrap_or(argument);
                    format!(
                        "Some(({}).expires_at.elapsed_nanoseconds.clone())",
                        self.expression(value)
                    )
                },
            );
            return format!("TerraneTaskScope::new({deadline})");
        }
        if callee.kind == SyntaxKind::Name
            && self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .is_some_and(|symbol| symbol.identity == "/core/concurrency::channel")
        {
            let values = arguments
                .children
                .iter()
                .map(|argument| argument.children.last().unwrap_or(argument))
                .collect::<Vec<_>>();
            let capacity = self.expression_as(values[1], ValueType::Scalar(ScalarType::Int));
            let overflow = self.expression(values[2]);
            return format!(
                "TerraneChannelPair::new(terrane_collection_support::index_from_int(&({capacity})).expect(\"semantic channel capacity\"), {overflow})"
            );
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && matches!(
                self.receiver_value_type(receiver),
                Some(ValueType::TaskScope | ValueType::TaskGroup(_))
            )
        {
            let is_group = matches!(
                self.receiver_value_type(receiver),
                Some(ValueType::TaskGroup(_))
            );
            let receiver = self.expression(receiver);
            let scope = if is_group {
                format!("({receiver}).scope()")
            } else {
                format!("({receiver}).clone()")
            };
            let deadline = if is_group {
                "__terrane_scope.deadline.clone()"
            } else {
                "__terrane_scope.deadline"
            };
            let cancel_failure = if is_group {
                "__terrane_scope.cancel(); "
            } else {
                ""
            };
            return match self.text(member) {
                "spawn" => arguments.children.first().map_or_else(String::new, |argument| {
                    let callable = argument.children.last().unwrap_or(argument);
                    let callable_type = self.value_type(callable);
                    let direct_contract = self.contract_for_call(callable, false);
                    let throws = if matches!(callable_type, Some(ValueType::Task(_, _))) {
                        self.awaited_throws(callable)
                    } else { direct_contract.map_or_else(
                        || {
                            matches!(
                                &callable_type,
                                Some(ValueType::AsyncFunction(_, _, _, effects))
                                    if effects.requires_throwing_abi()
                            )
                        },
                        |contract| contract.throws,
                    ) };
                    let foreign_error = callable.kind == SyntaxKind::Name
                        && self
                            .package
                            .resolve_name_at(self.unit, callable.span.start, self.text(callable))
                            .is_some_and(|symbol| symbol.identity.starts_with("/deps/"));
                    let callable = if let Some(value_type) = callable_type.clone() {
                        self.expression_as(callable, value_type)
                    } else {
                        self.expression(callable)
                    };
                    let (task_setup, invocation) =
                        if matches!(callable_type, Some(ValueType::Task(_, _))) {
                            (
                                format!("let __terrane_spawned_task = {callable}; "),
                                "__terrane_spawned_task".to_owned(),
                            )
                        } else {
                            (String::new(), format!("({callable})()"))
                        };
                    let spawn = if foreign_error {
                        format!(
                            "{{ let __terrane_scope = {scope}; let __terrane_cancel = __terrane_scope.cancellation(); let __terrane_deadline = {deadline}; {task_setup}TerraneScopedTask::spawn(async move {{ match __terrane_cancellable({invocation}, __terrane_cancel, __terrane_deadline).await {{ Some(Ok(value)) => TerraneTaskResult::Completed(value), Some(Err(error)) => {{ {cancel_failure}TerraneTaskResult::Failed(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)) }}, None => TerraneTaskResult::Cancelled }} }}) }}"
                        )
                    } else if throws {
                        format!(
                            "{{ let __terrane_scope = {scope}; let __terrane_cancel = __terrane_scope.cancellation(); let __terrane_deadline = {deadline}; {task_setup}TerraneScopedTask::spawn(async move {{ match __terrane_cancellable({invocation}, __terrane_cancel, __terrane_deadline).await {{ Some(Ok(value)) => TerraneTaskResult::Completed(value), Some(Err(error)) => {{ {cancel_failure}TerraneTaskResult::Failed(error) }}, None => TerraneTaskResult::Cancelled }} }}) }}"
                        )
                    } else {
                        format!(
                            "{{ let __terrane_scope = {scope}; let __terrane_cancel = __terrane_scope.cancellation(); let __terrane_deadline = {deadline}; {task_setup}TerraneScopedTask::spawn(async move {{ match __terrane_cancellable({invocation}, __terrane_cancel, __terrane_deadline).await {{ Some(value) => TerraneTaskResult::Completed(value), None => TerraneTaskResult::Cancelled }} }}) }}"
                        )
                    };
                    if is_group {
                        format!("{{ let __terrane_child = {spawn}; ({receiver}).spawn(__terrane_child); }}")
                    } else {
                        spawn
                    }
                }),
                "drain" if is_group => format!("std::boxed::Box::pin(({receiver}).prepare_drain())"),
                "join" => arguments.children.first().map_or_else(String::new, |argument| {
                    let task = argument.children.last().unwrap_or(argument);
                    format!("({receiver}).join({})", self.expression(task))
                }),
                "cancel" => format!("({receiver}).cancel()"),
                "child-scope" => arguments.children.first().map_or_else(String::new, |argument| {
                    let deadline = argument.children.last().unwrap_or(argument);
                    format!(
                        "({receiver}).child_scope(&({}).expires_at.elapsed_nanoseconds)",
                        self.expression(deadline)
                    )
                }),
                _ => String::new(),
            };
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && receiver.kind == SyntaxKind::Name
            && let Some(receiver_type) = self.receiver_value_type(receiver)
            && matches!(
                receiver_type,
                ValueType::ChannelSender(_) | ValueType::ChannelReceiver(_)
            )
        {
            let receiver = self.expression(receiver);
            return match (receiver_type, self.text(member)) {
                (ValueType::ChannelSender(item), "send") => {
                    let value = arguments
                        .children
                        .first()
                        .map(|argument| argument.children.last().unwrap_or(argument))
                        .map_or_else(String::new, |value| {
                            self.expression_as(value, item.value_type())
                        });
                    format!("Box::pin(({receiver}).send({value}))")
                }
                (ValueType::ChannelReceiver(_), "receive") => {
                    format!("Box::pin(({receiver}).receive())")
                }
                (ValueType::ChannelSender(_) | ValueType::ChannelReceiver(_), "close") => {
                    format!("({receiver}).close()")
                }
                _ => String::new(),
            };
        }

        if let Ok(Some((receiver, receiver_type, member))) =
            collection_member_call(self.unit, callee, &self.unit.typed_bindings)
            && (member.contains('.') || member == "get")
        {
            if matches!(member.as_str(), "get" | "get.checked" | "remove.checked")
                && let Some(argument) = arguments.children.first()
            {
                let remove = member == "remove.checked";
                let checked = member != "get";
                let argument = argument.children.last().unwrap_or(argument);
                let (setup, receiver_value) = if remove {
                    self.live_receiver_parts(receiver)
                } else {
                    (String::new(), self.receiver_expression(receiver))
                };
                let unique =
                    crate::semantics::collection_uses_unique_storage(self.package, &receiver_type);
                let call = match receiver_type {
                    ValueType::List(_) | ValueType::Tuple(_, _) => {
                        let index =
                            self.expression_as(argument, ValueType::Scalar(ScalarType::Int));
                        if checked {
                            format!(
                                "terrane_collection_support::index_from_int(&({index})).ok().and_then(|index| ({receiver_value}).get(index).cloned())"
                            )
                        } else {
                            self.fallible(
                                format!("terrane_collection_support::index_from_int(&({index})).and_then(|index| ({receiver_value}).get_or_error(index))"),
                                node,
                            )
                        }
                    }
                    ValueType::Map(key, _) | ValueType::UnorderedMap(key, _) => {
                        let key = self.expression_as(argument, key.value_type());
                        if remove {
                            let method = if unique {
                                "remove_checked_unique"
                            } else {
                                "remove_checked"
                            };
                            format!(
                                "{{ let __terrane_key = {key}; ({receiver_value}).{method}(&__terrane_key) }}"
                            )
                        } else if checked {
                            format!("({receiver_value}).get(&({key})).cloned()")
                        } else {
                            self.fallible(
                                format!("({receiver_value}).get_or_error(&({key}))"),
                                node,
                            )
                        }
                    }
                    _ => String::new(),
                };
                return if remove {
                    format!("{{ {setup} let __terrane_result = {call}; __terrane_result }}")
                } else {
                    call
                };
            }
            if member == "sort.descending"
                && let ValueType::List(item) = receiver_type
            {
                let (setup, receiver_value) = if self.receiver_needs_live_guard(receiver) {
                    self.live_receiver_parts(receiver)
                } else {
                    (String::new(), self.receiver_expression(receiver))
                };
                let comparator = list_sort_comparator(&item, true);
                let call = format!(
                    "({{ let collection = &mut ({receiver_value}); collection.sort_by({comparator}); collection.clone() }})"
                );
                return format!("{{ {setup} let __terrane_result = {call}; __terrane_result }}");
            }
        }
        if let Some(string_call) = self.string_call(node, arguments) {
            return string_call;
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && self.is_throwable_value(receiver)
            && self.text(member) == "render"
        {
            let (setup, receiver) = self.borrowed_receiver_parts(receiver);
            let value = format!("({receiver}).render()");
            return if setup.is_empty() {
                value
            } else {
                format!("{{ {setup} {value} }}")
            };
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && let Some(receiver_type) = self.receiver_value_type(receiver)
        {
            let member_name = self.text(member).to_owned();
            let remove_from_collection = member_name == "remove"
                && matches!(
                    receiver_type,
                    ValueType::List(_) | ValueType::Map(_, _) | ValueType::UnorderedMap(_, _)
                );
            let (setup, receiver_value) = if remove_from_collection
                || !matches!(receiver_type, ValueType::Scalar(_))
                    && self.receiver_needs_live_guard(receiver)
            {
                self.live_receiver_parts(receiver)
            } else {
                (String::new(), self.receiver_expression(receiver))
            };
            let values = arguments
                .children
                .iter()
                .map(|argument| argument.children.last().unwrap_or(argument))
                .collect::<Vec<_>>();
            let unique =
                crate::semantics::collection_uses_unique_storage(self.package, &receiver_type);
            let call = match (receiver_type, member_name.as_str()) {
                (
                    ValueType::Scalar(receiver_type @ (ScalarType::Float32 | ScalarType::Float64)),
                    operation,
                ) if float_member_contract(operation)
                    .is_some_and(|contract| contract.parameters.is_some()) =>
                {
                    let contract =
                        float_member_contract(operation).expect("validated floating operation");
                    let arguments = values
                        .iter()
                        .zip(contract.parameters.expect("callable floating operation"))
                        .map(|(value, parameter)| {
                            self.expression_as(
                                value,
                                ValueType::Scalar(match parameter {
                                    FloatMemberArgument::Receiver => receiver_type,
                                    FloatMemberArgument::Int32 => ScalarType::Int32,
                                }),
                            )
                        })
                        .collect::<Vec<_>>();
                    self.float_call(receiver_type, operation, &receiver_value, &arguments, node)
                }
                (ValueType::Iterator(_), "next") => {
                    let call = format!("({receiver_value}).next()");
                    Some(if self.discarded_call == Some(node.span) {
                        format!("{{ let _ = {call}; }}")
                    } else {
                        call
                    })
                }
                (ValueType::List(item), "append") => {
                    let value = self.expression_as(values[0], item.value_type());
                    Some(format!(
                        "({{ let __terrane_value = {value}; let collection = &mut ({receiver_value}); collection.append(__terrane_value); collection.clone() }})"
                    ))
                }
                (ValueType::List(item), "set") => {
                    let index = self.expression_as(values[0], ValueType::Scalar(ScalarType::Int));
                    let index = self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index}))"),
                        node,
                    );
                    let value = self.expression_as(values[1], item.value_type());
                    let mutation =
                        self.fallible("collection.set(__terrane_index, __terrane_value)", node);
                    Some(format!(
                        "({{ let __terrane_index = {index}; let __terrane_value = {value}; let collection = &mut ({receiver_value}); {mutation}; collection.clone() }})"
                    ))
                }
                (ValueType::List(_), "clear") => Some(format!(
                    "({{ let collection = &mut ({receiver_value}); collection.clear(); collection.clone() }})"
                )),
                (ValueType::List(item), "sort") => {
                    let comparator = list_sort_comparator(&item, false);
                    Some(format!(
                        "({{ let collection = &mut ({receiver_value}); collection.sort_by({comparator}); collection.clone() }})"
                    ))
                }
                (ValueType::List(_), "remove") => {
                    let index = self.expression_as(values[0], ValueType::Scalar(ScalarType::Int));
                    let index = self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index}))"),
                        node,
                    );
                    let method = if unique { "remove_unique" } else { "remove" };
                    let mutation = self.fallible(
                        format!("({receiver_value}).{method}(__terrane_index)"),
                        node,
                    );
                    Some(format!("{{ let __terrane_index = {index}; {mutation} }}"))
                }
                (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "set") => {
                    let key = self.expression_as(values[0], key.value_type());
                    let value = self.expression_as(values[1], value.value_type());
                    Some(format!(
                        "({{ let __terrane_key = {key}; let __terrane_value = {value}; let collection = &mut ({receiver_value}); collection.set(__terrane_key, __terrane_value); collection.clone() }})"
                    ))
                }
                (ValueType::Map(key, _) | ValueType::UnorderedMap(key, _), "remove") => {
                    let key = self.expression_as(values[0], key.value_type());
                    let method = if unique { "remove_unique" } else { "remove" };
                    let mutation =
                        self.fallible(format!("({receiver_value}).{method}(&__terrane_key)"), node);
                    Some(format!("{{ let __terrane_key = {key}; {mutation} }}"))
                }
                (ValueType::Set(item) | ValueType::UnorderedSet(item), "contains") => {
                    Some(format!(
                        "({{ let __terrane_value = {}; ({receiver_value}).contains(&__terrane_value) }})",
                        self.expression_as(values[0], item.value_type())
                    ))
                }
                (ValueType::Set(item) | ValueType::UnorderedSet(item), "add") => Some(format!(
                    "({{ let __terrane_value = {}; let collection = &mut ({receiver_value}); collection.add(__terrane_value); collection.clone() }})",
                    self.expression_as(values[0], item.value_type())
                )),
                (ValueType::Set(item) | ValueType::UnorderedSet(item), "remove") => Some(format!(
                    "({{ let __terrane_value = {}; ({receiver_value}).remove(&__terrane_value) }})",
                    self.expression_as(values[0], item.value_type())
                )),
                (
                    ValueType::Map(_, _) | ValueType::UnorderedMap(_, _),
                    "keys" | "values" | "entries",
                ) => Some(format!("({receiver_value}).{member_name}()")),
                _ => None,
            };
            if let Some(call) = call {
                return format!("{{ {setup} let __terrane_result = {call}; __terrane_result }}");
            }
        }
        if let Some(method) = bound_method(self.source, callee)
            && !matches!(
                find_node_by_span(&self.unit.tree.root, method.receiver)
                    .and_then(|receiver| self.receiver_value_type(receiver)),
                Some(ValueType::Object(_) | ValueType::TypeParameter(_))
            )
        {
            let receiver_node = find_node_by_span(&self.unit.tree.root, method.receiver)
                .expect("validated bound method receiver");
            let receiver = if method.family == MemberFamily::Parse {
                self.expression_as(
                    receiver_node,
                    self.receiver_value_type(receiver_node)
                        .expect("validated scalar parser input"),
                )
            } else {
                self.expression(receiver_node)
            };
            if method.family == MemberFamily::Coerce {
                return self.numeric_coercion(&method, receiver_node, callee, arguments);
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
                .map_or_default(|value| self.expression(value));
            let callback_throws = method.family == MemberFamily::Parse
                && arguments
                    .children
                    .first()
                    .and_then(|argument| argument.children.last())
                    .and_then(|callback| self.contract_for_call(callback, false))
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
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && self.text(member) == "end"
            && self.is_builtin(receiver, "/core/collections::iteration-step")
        {
            return "terrane_collection_support::IterationStep::End".to_owned();
        }
        if self.is_builtin(callee, "/core/collections::iteration-step") {
            let item_type = self
                .value_type(node)
                .and_then(|ty| match ty {
                    ValueType::IterationStep(item) => Some(item),
                    _ => None,
                })
                .expect("validated iteration-step constructor has an item type");
            let argument = arguments
                .children
                .first()
                .expect("validated iteration-step constructor has one argument");
            let item = argument.children.last().unwrap_or(argument);
            return format!(
                "terrane_collection_support::IterationStep::<{}>::Item({})",
                rust_element_type(self.package, item_type.clone()),
                self.expression_as(item, item_type.value_type())
            );
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
            if argument_values
                .iter()
                .any(|value| self.throwable_text_member(value).is_some())
            {
                let mut setup = String::new();
                let mut values = Vec::with_capacity(argument_values.len());
                for value in &argument_values {
                    let snapshot = argument_values.len() > 1
                        && (self.contains_guarded_read(value)
                            || !self.read_borrow_is_stable_in(value, arguments));
                    if let Some((value_setup, text)) = self.throwable_text_parts(value, snapshot) {
                        setup.push_str(&value_setup);
                        values.push(text);
                    } else {
                        let text = self.text_display(value);
                        let name = self.fresh_read_binding_name("print_value");
                        write!(setup, "let {name} = {text}; ")
                            .expect("writing to a String cannot fail");
                        values.push(name);
                    }
                }
                let format = "{}".repeat(values.len());
                return format!(
                    "{{ {setup}println!(\"{format}\", {}); }}",
                    values.join(", ")
                );
            }
            let values = argument_values
                .iter()
                .map(|value| self.text_display(value))
                .collect::<Vec<_>>();
            let format = "{}".repeat(values.len());
            return format!("println!(\"{format}\", {})", values.join(", "));
        }
        if self.is_builtin(callee, "intrinsic:logging::log-empty-fields") {
            return "terrane_collection_support::List::new(Vec::new())".to_owned();
        }
        if self.is_builtin(callee, "intrinsic:logging::log-empty-spans") {
            return "terrane_collection_support::List::<String>::new(Vec::new())".to_owned();
        }
        if self.is_builtin(callee, "intrinsic:logging::log-no-sink") {
            return "terrane_platform_support::logging_no_sink()".to_owned();
        }
        if self.is_builtin(callee, "intrinsic:logging::log-memory-sink") {
            let integer = |emitter: &mut Self, index: usize| {
                let value = emitter
                    .expression_as(argument_values[index], ValueType::Scalar(ScalarType::Int));
                format!("terrane_int_support::checked_coerce::<i128>(&({value}))")
            };
            let capacity = integer(self, 0);
            let overflow = self.expression(argument_values[1]);
            let start = integer(self, 2);
            let step = integer(self, 3);
            let reveal = self.expression(argument_values[4]);
            return format!(
                "terrane_platform_support::logging_memory_sink({capacity}, &({overflow}), {start}, {step}, {reveal})"
            );
        }
        if self.is_builtin(callee, "intrinsic:logging::log-console-sink") {
            let reveal = self.expression(argument_values[0]);
            return format!("terrane_platform_support::logging_console_sink({reveal})");
        }
        if self.is_builtin(callee, "intrinsic:logging::log-failing-sink") {
            return "terrane_platform_support::logging_failing_sink()".to_owned();
        }
        for (identity, function) in [
            ("log-result-failed", "terrane_platform_result_failed"),
            ("log-result-message", "terrane_platform_result_message"),
            (
                "log-result-capability",
                "terrane_platform_result_capability",
            ),
        ] {
            if self.is_builtin(callee, &format!("intrinsic:logging::{identity}")) {
                let value = self.expression(argument_values[0]);
                return format!("{function}(&({value}))");
            }
        }
        if self.is_builtin(callee, "intrinsic:logging::log-result-entries") {
            let value = self.expression(argument_values[0]);
            return format!(
                "terrane_collection_support::List::new(terrane_platform_result_entries(&({value})))"
            );
        }
        if self.is_builtin(callee, "intrinsic:logging::log-discarded-count") {
            let sink = self.expression(argument_values[0]);
            return format!(
                "terrane_int_support::Int::from(i128::from(terrane_platform_support::logging_discarded_count(&({sink}))))"
            );
        }
        if self.is_builtin(callee, "intrinsic:logging::log-drain") {
            let sink = self.expression(argument_values[0]);
            return format!("terrane_platform_support::logging_drain(&({sink}))");
        }
        if self.is_builtin(callee, "intrinsic:logging::log-drain-fallback") {
            return "terrane_platform_support::logging_drain_fallback()".to_owned();
        }
        if self.is_builtin(callee, "intrinsic:logging::log-install-dependency-bridge") {
            let sink = self.expression(argument_values[0]);
            return format!(
                "terrane_platform_support::logging_install_dependency_bridge(&({sink}))"
            );
        }
        if self.is_builtin(callee, "intrinsic:logging::log-write") {
            let sink = self.expression(argument_values[0]);
            let severity = self.expression(argument_values[1]);
            let target = self.expression(argument_values[2]);
            let message = self.expression(argument_values[3]);
            let fields = self.expression(argument_values[4]);
            let source = self.expression(argument_values[5]);
            let spans = self.expression(argument_values[6]);
            let max_fields =
                self.expression_as(argument_values[7], ValueType::Scalar(ScalarType::Int));
            let max_bytes =
                self.expression_as(argument_values[8], ValueType::Scalar(ScalarType::Int));
            return format!(
                "{{ let sink = {sink}; let severity = {severity}; let target = {target}; let message = {message}; let raw_fields = {fields}; let source = {source}; let spans = {spans}; let max_fields_value = {max_fields}; let max_bytes_value = {max_bytes}; match (terrane_collection_support::index_from_int(&max_fields_value), terrane_collection_support::index_from_int(&max_bytes_value)) {{ (Ok(max_fields), Ok(max_bytes)) => {{ let reveal_secrets = terrane_platform_support::logging_reveals_secrets(&sink); let fields = raw_fields.into_vec().into_iter().map(|field| {{ let value_json = if field.secret && !reveal_secrets {{ \"null\".to_owned() }} else {{ field.value.render().encoded.clone() }}; terrane_platform_support::LogFieldInput {{ name: field.name, value_json, secret: field.secret, source: field.source }} }}).collect::<Vec<_>>(); terrane_platform_support::logging_emit(&sink, terrane_platform_support::LogEventInput {{ severity, target, message, fields, source, spans: spans.into_vec(), max_fields, max_bytes, origin: \"terrane\".to_owned() }}) }}, (Err(_), _) => terrane_platform_support::ResultValue::error(\"logging field limit must be non-negative and fit this target\"), (_, Err(_)) => terrane_platform_support::ResultValue::error(\"logging byte limit must be non-negative and fit this target\") }} }}"
            );
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
            ("json-parse", "json_parse"),
            ("json-canonical", "json_canonical"),
            ("json-stringify", "json_stringify"),
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
            self.is_builtin(callee, &format!("intrinsic:data::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = data_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let integer_argument = matches!(
                        (function, index),
                        ("json_parse" | "json_stringify", 1 | 2)
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
                    match function {
                        "make_document_list" => {
                            // `core/documents.trn` passes a `list of document-value`; borrow its raw results.
                            return format!("({value}).as_slice().iter().map(|item| &item.raw)");
                        }
                        "make_document_map" => {
                            // `core/documents.trn` stores a `list of entry of string, host-platform-data-result`.
                            return format!("({value}).as_slice().iter().map(|entry| (entry.key.as_str(), &entry.value))");
                        }
                        _ => {}
                    }
                    let borrowed_result = index == 0
                        && (function.starts_with("data_")
                            || function.starts_with("document_")
                            || matches!(function, "json_canonical" | "json_stringify")
                            || function == "validate_mapping"
                            || function.starts_with("url_") && function != "url_parse");
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
        let child_call = if callee.kind == SyntaxKind::Name {
            self.package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .and_then(|symbol| {
                    symbol
                        .compiler_identity()
                        .strip_prefix("intrinsic:child::child-")
                })
        } else {
            None
        };
        if child_call == Some("result-capture") {
            let capture = rust_value_type(
                self.package,
                ValueType::Object(ObjectIdentity::new(
                    "/core/process/child",
                    "process-capture",
                )),
            );
            let status = self.expression(argument_values[0]);
            let raw = self.expression(argument_values[1]);
            return format!(
                "{{ let __terrane_status = {status}; let __terrane_capture = {raw}; \
                 {capture}::terrane_construct(__terrane_status, __terrane_capture.data, \
                 __terrane_capture.secondary_data, __terrane_capture.truncated, \
                 __terrane_capture.secondary_flag) }}"
            );
        }
        if let Some(name) = child_call {
            let function = name.replace('-', "_");
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let value = self.expression(value);
                    match (name, index) {
                        ("spawn" | "capture" | "capture-async", 1 | 3 | 4)
                        | ("capture" | "capture-async", 7)
                        | ("write", 1) => format!("({value}).as_slice()"),
                        ("result-data", _) => value,
                        ("spawn" | "capture" | "capture-async", 2)
                        | ("send-signal", 1)
                        | (_, 0) => {
                            format!("&({value})")
                        }
                        _ => value,
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_platform_child_{function}({values})");
        }
        if self.is_builtin(
            callee,
            "intrinsic:process-signals::process-signal-no-capability",
        ) {
            return "TerranePlatformCapability::default()".to_owned();
        }
        if self.is_builtin(
            callee,
            "intrinsic:process-signals::process-signal-subscribe",
        ) {
            return format!(
                "terrane_process_signal_subscribe({})",
                self.expression(argument_values[0])
            );
        }
        if self.is_builtin(callee, "intrinsic:process-signals::process-signal-next") {
            return format!(
                "terrane_process_signal_next(&({}))",
                self.expression(argument_values[0])
            );
        }
        if self.is_builtin(callee, "intrinsic:process-signals::process-signal-close") {
            return format!(
                "terrane_process_signal_close(&({}))",
                self.expression(argument_values[0])
            );
        }
        let signal_result_call = [
            ("process-signal-result-failed", "platform_result_failed"),
            ("process-signal-result-message", "platform_result_message"),
            ("process-signal-result-detail", "platform_result_detail"),
            ("process-signal-result-int", "platform_result_int"),
            ("process-signal-result-bool", "platform_result_bool"),
            (
                "process-signal-result-capability",
                "platform_result_capability",
            ),
            (
                "process-signal-result-exact-int",
                "process_signal_result_exact_int",
            ),
            (
                "process-signal-result-observed",
                "process_signal_result_observed",
            ),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("intrinsic:process-signals::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = signal_result_call {
            return format!(
                "terrane_{function}(&({}))",
                self.expression(argument_values[0])
            );
        }
        if callee.kind == SyntaxKind::StaticMemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && self.text(member) == "sleep"
            && self
                .package
                .resolve_name_at(self.unit, receiver.span.start, self.text(receiver))
                .is_some_and(|symbol| symbol.compiler_identity() == "/core/time::clock")
        {
            let elapsed = argument_values
                .first()
                .expect("time sleep arity is checked semantically");
            return format!(
                "terrane_time_sleep_after(({}).total_nanoseconds.clone())",
                self.expression(elapsed)
            );
        }
        if self.is_builtin(callee, "intrinsic:capabilities::bytes-from-octets") {
            let octets = argument_values
                .first()
                .expect("bytes-from-octets arity is checked semantically");
            return format!(
                "({}).into_vec()",
                self.expression_as(
                    octets,
                    ValueType::List(ElementType::new(ValueType::Scalar(ScalarType::Uint8))),
                )
            );
        }
        let time_call = [
            ("time-wall", "platform_time_wall"),
            ("time-wall-seconds", "platform_time_wall_seconds"),
            ("time-wall-nanoseconds", "platform_time_wall_nanoseconds"),
            ("time-domain", "platform_time_domain"),
            ("time-monotonic", "platform_time_monotonic"),
            ("time-sleep-until", "platform_time_sleep_until"),
            ("time-div", "platform_time_div"),
            ("time-mod", "platform_time_mod"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("intrinsic:time::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = time_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let value = self.expression(value);
                    if matches!(
                        function,
                        "platform_time_wall_seconds" | "platform_time_wall_nanoseconds"
                    ) || index == 0
                        && matches!(function, "platform_time_div" | "platform_time_mod")
                    {
                        format!("&({value})")
                    } else if index == 1
                        && matches!(function, "platform_time_div" | "platform_time_mod")
                    {
                        format!("({value}).clone()")
                    } else {
                        value
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_{function}({values})");
        }
        let buffer_call = [
            (
                "intrinsic:capabilities::buffer-create",
                "platform_buffer_create",
                &["capacity"][..],
            ),
            (
                "intrinsic:capabilities::buffer-clear",
                "platform_buffer_clear",
                &["buffer"][..],
            ),
            (
                "intrinsic:capabilities::buffer-close",
                "platform_buffer_close",
                &["buffer"][..],
            ),
            (
                "intrinsic:capabilities::buffer-capacity",
                "platform_buffer_capacity",
                &["buffer"][..],
            ),
            (
                "intrinsic:capabilities::buffer-completed",
                "platform_buffer_completed",
                &["buffer"][..],
            ),
            (
                "intrinsic:capabilities::buffer-filled-bytes",
                "platform_buffer_filled_bytes",
                &["buffer"][..],
            ),
            (
                "intrinsic:capabilities::buffer-filled-text",
                "platform_buffer_filled_text",
                &["buffer"][..],
            ),
            (
                "intrinsic:capabilities::tcp-read-into-async",
                "platform_tcp_read_into_async",
                &["handle", "buffer", "count", "deadline", "cancellation"][..],
            ),
            (
                "intrinsic:capabilities::tls-read-into-async",
                "platform_tls_read_into_async",
                &["handle", "buffer", "count", "deadline", "cancellation"][..],
            ),
            (
                "intrinsic:capabilities::stream-read-into-async",
                "platform_stream_read_into_async",
                &["handle", "buffer", "count", "deadline", "cancellation"][..],
            ),
            (
                "intrinsic:capabilities::tcp-write-async",
                "platform_tcp_write_async",
                &["handle", "data", "offset", "deadline", "cancellation"][..],
            ),
            (
                "intrinsic:capabilities::tls-write-async",
                "platform_tls_write_async",
                &["handle", "data", "offset", "deadline", "cancellation"][..],
            ),
        ]
        .into_iter()
        .find(|(identity, _, _)| self.is_builtin(callee, identity));
        if let Some((identity, function, parameters)) = buffer_call {
            let values = intrinsic_arguments(&self.unit.source, arguments, parameters)
                .into_iter()
                .zip(parameters)
                .filter_map(|(argument, parameter)| {
                    let argument = argument?;
                    Some(match *parameter {
                        "capacity" | "handle" | "buffer" => {
                            format!("&({})", self.native_receiver_expression(argument, false))
                        }
                        "cancellation" => format!("({}).as_ref()", self.expression(argument)),
                        "data"
                            if matches!(
                                identity,
                                "intrinsic:capabilities::tcp-write-async"
                                    | "intrinsic:capabilities::tls-write-async"
                            ) =>
                        {
                            format!("&({})", self.native_receiver_expression(argument, false))
                        }
                        "offset" | "count" | "deadline" => {
                            self.expression_as(argument, ValueType::Scalar(ScalarType::Int))
                        }
                        _ => self.expression(argument),
                    })
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_{function}({values})");
        }
        if self.unit.namespace == "/core/regex"
            && callee.kind == SyntaxKind::Name
            && let Some(name) = self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .and_then(|symbol| symbol.compiler_identity().strip_prefix("intrinsic:regex::"))
        {
            let function = crate::bundled::REGEX_HOST_FUNCTIONS
                .iter()
                .find_map(|&(source, rust)| (source == name).then_some(rust))
                .expect("registered regex host function");
            // Host search inputs lend storage. Only the checked outcome is
            // consumed; numeric group indices cross through a checked usize.
            let args = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| match (function, index) {
                    ("result_value", _) | ("compile", 1) => self.expression(value),
                    _ => {
                        if let Some(name) = self.borrowed_key_name(value) {
                            return name.to_owned();
                        }
                        let (setup, reference) = self.operand_reference_parts(value, false);
                        let expression = if function == "capture" && index == 1 {
                            format!("terrane_int_support::coerce::<usize>({reference}).ok()")
                        } else {
                            reference
                        };
                        if setup.is_empty() {
                            expression
                        } else {
                            format!("{{ {setup}{expression} }}")
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_regex_support::{function}({args})");
        }
        let capability_call = [
            ("no-capability", "platform_no_resource"),
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
            (
                "tcp-listener-into-native",
                "platform_tcp_listener_into_native",
            ),
            ("tcp-stream-into-native", "platform_tcp_stream_into_native"),
            ("native-result-failed", "platform_native_result_failed"),
            ("native-result-message", "platform_native_result_message"),
            ("native-listener-value", "platform_native_result_value"),
            ("native-stream-value", "platform_native_result_value"),
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
            ("tcp-listen-async", "platform_tcp_listen_async"),
            ("tcp-connect-async", "platform_tcp_connect_async"),
            ("tcp-connect-host-async", "platform_tcp_connect_host_async"),
            ("tcp-accept-async", "platform_tcp_accept_async"),
            ("tcp-read-async", "platform_tcp_read_async"),
            ("tcp-write-async", "platform_tcp_write_async"),
            ("udp-send-to-async", "platform_udp_send_to_async"),
            ("udp-receive-from-async", "platform_udp_receive_from_async"),
            ("dns-lookup-async", "platform_dns_lookup_async"),
            ("tls-client", "platform_tls_client"),
            ("tls-read", "platform_tls_read"),
            ("tls-write", "platform_tls_write"),
            ("tls-shutdown", "platform_tls_shutdown"),
            ("tls-client-async", "platform_tls_client_async"),
            ("tls-read-async", "platform_tls_read_async"),
            ("tls-write-async", "platform_tls_write_async"),
            ("tls-shutdown-async", "platform_tls_shutdown_async"),
            ("close", "platform_capability_close"),
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
            self.is_builtin(callee, &format!("intrinsic:capabilities::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = capability_call {
            if function == "platform_result_deadline_exceeded"
                && let Some(value) = argument_values.first()
                && matches!(
                    self.value_type(value),
                    Some(ValueType::PlatformReadResult | ValueType::PlatformWriteResult)
                )
            {
                return format!(
                    "({}).deadline_exceeded",
                    self.native_receiver_expression(value, false)
                );
            }
            let values = argument_values.iter().enumerate().map(|(index, value)| {
                if self.value_type(value).is_some_and(|ty|
                    matches!(&ty, ValueType::Optional(inner) if matches!(**inner, ValueType::PlatformCapability))) {
                    return format!("({}).as_ref()", self.expression(value));
                }
                let expression = self.expression(value);
                let borrowed = matches!((function, index),
                    ("platform_random_bytes" | "platform_random_bounded" | "platform_random_split"
                    | "platform_uuid_v4" | "platform_uuid_v7" | "platform_tcp_accept_async"
                    | "platform_tcp_read_async" | "platform_tcp_shutdown" | "platform_tcp_configure"
                    | "platform_udp_send_to" | "platform_udp_send_to_async" | "platform_udp_receive_from"
                    | "platform_udp_receive_from_async" | "platform_udp_configure" | "platform_tls_client"
                    | "platform_tls_client_async" | "platform_tls_read" | "platform_tls_read_async"
                    | "platform_tls_write" | "platform_tls_shutdown" | "platform_tls_shutdown_async"
                    | "platform_capability_close" | "platform_digest" | "platform_hmac"
                    | "platform_parse_socket" | "platform_cancel" | "platform_destroy_secret"
                    | "platform_tcp_listener_into_native" | "platform_tcp_stream_into_native"
                    | "platform_native_result_failed" | "platform_native_result_message", 0)
                    | ("platform_parse_socket" | "platform_hmac", 1))
                    || function.starts_with("platform_result_") && index == 0;
                if borrowed { format!("&({expression})") } else { expression }
            })
                .collect::<Vec<_>>()
                .join(", ");
            return format!("terrane_{function}({values})");
        }
        let concurrency_call = [
            ("no-capability", "platform_no_resource"),
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
            self.is_builtin(callee, &format!("intrinsic:concurrency::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = concurrency_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let value = self.expression(value);
                    let borrowed = index == 0
                        && (function.starts_with("platform_result_")
                            || !matches!(
                                function,
                                "platform_int_mutex"
                                    | "platform_int_rw_lock"
                                    | "platform_atomic_int64"
                                    | "platform_thread_local_int"
                            ));
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
        if self.is_builtin(callee, "intrinsic:adapters::system-host-name") {
            return "terrane_platform_support::system_host_name()".to_owned();
        }
        if self.is_builtin(callee, "intrinsic:adapters::process-current-executable") {
            return "terrane_platform_support::process_current_executable()".to_owned();
        }
        let adapter_result_field = [
            ("result-failed", "failed", false),
            ("result-bool", "flag", false),
            ("result-message", "message", true),
            ("result-text", "text", true),
        ]
        .into_iter()
        .find_map(|(terrane, field, cloned)| {
            self.is_builtin(callee, &format!("intrinsic:adapters::{terrane}"))
                .then_some((field, cloned))
        });
        if let Some((field, cloned)) = adapter_result_field {
            let value = self.expression(argument_values[0]);
            if cloned {
                return format!("({value}).{field}.clone()");
            }
            return format!("({value}).{field}");
        }
        let testing_call = [
            ("test-spawn", "test_spawn"),
            ("test-result-failed", "test_result_failed"),
            (
                "test-result-deadline-exceeded",
                "test_result_deadline_exceeded",
            ),
            ("test-result-message", "test_result_message"),
            ("test-result-exit-code", "test_result_exit_code"),
            ("test-result-crashed", "test_result_crashed"),
            ("test-result-stdout", "test_result_stdout"),
            ("test-result-stderr", "test_result_stderr"),
            (
                "test-result-stdout-truncated",
                "test_result_stdout_truncated",
            ),
            (
                "test-result-stderr-truncated",
                "test_result_stderr_truncated",
            ),
            ("test-time-advance", "test_time_advance"),
            ("test-render-int", "test_render_int"),
            ("test-render-float64", "test_render_float64"),
            ("test-render-bytes", "test_render_bytes"),
            ("test-render-bool", "test_render_bool"),
            ("test-deadline-nanoseconds", "test_deadline_nanoseconds"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("intrinsic:testing::{terrane}"))
                .then_some(rust)
        });
        if let Some(function) = testing_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    if (function == "test_spawn" && index == 4)
                        || matches!(function, "test_time_advance" | "test_render_int") && index == 0
                    {
                        self.expression_as(value, ValueType::Scalar(ScalarType::Int))
                    } else {
                        self.expression(value)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            if function.starts_with("test_result_") {
                return format!("terrane_{function}(&({values}))");
            }
            return format!("terrane_{function}({values})");
        }
        let system_call = [
            (
                "acquire-filesystem-authority",
                "acquire_filesystem_authority",
            ),
            ("filesystem-temporary-create", "temporary_create"),
            ("filesystem-temporary-delete", "temporary_delete"),
            ("filesystem-temporary-keep", "temporary_keep"),
            ("filesystem-create-directory", "filesystem_create_directory"),
            (
                "filesystem-create-directories",
                "filesystem_create_directories",
            ),
            ("filesystem-list-directory", "filesystem_list_directory"),
            ("filesystem-remove-directory", "filesystem_remove_directory"),
            (
                "filesystem-remove-directory-tree",
                "filesystem_remove_directory_tree",
            ),
            ("filesystem-create-symlink", "filesystem_create_symlink"),
            ("filesystem-create-hard-link", "filesystem_create_hard_link"),
            ("filesystem-exists", "filesystem_exists"),
            ("filesystem-metadata", "filesystem_metadata"),
            ("filesystem-realpath", "filesystem_realpath"),
            ("filesystem-read-link", "filesystem_read_link"),
            ("filesystem-read-bounded", "filesystem_read_bounded"),
            ("filesystem-write-atomic", "filesystem_write_atomic"),
            ("filesystem-rename", "filesystem_rename"),
            ("filesystem-remove", "filesystem_remove"),
            ("process-arguments", "process_arguments"),
            ("process-id", "process_id"),
            ("process-group-id", "process_group_id"),
            ("environment-entries", "environment_entries"),
            ("process-exit", "process_exit"),
        ];
        let borrowed_system_call = [
            ("filesystem-temporary-status", "temporary_status"),
            ("filesystem-temporary-location", "temporary_location"),
            ("result-failed", "filesystem_result_failed"),
            ("result-error-kind", "filesystem_result_error_kind"),
            ("result-message", "filesystem_result_message"),
            ("result-text", "filesystem_result_text"),
            ("result-detail", "filesystem_result_detail"),
            ("result-entries", "filesystem_result_entries"),
            ("result-bytes", "filesystem_result_bytes"),
            ("result-int", "filesystem_result_int"),
            ("result-bool", "filesystem_result_bool"),
            ("platform-value-is-text", "platform_value_is_text"),
            ("platform-value-text", "platform_value_text"),
            ("platform-value-bytes", "platform_value_bytes"),
            ("platform-value-from-bytes", "platform_value_from_bytes"),
            ("platform-value-from-text", "platform_value_from_text"),
        ];
        let system_call = system_call
            .into_iter()
            .map(|(source, rust)| (source, rust, false))
            .chain(
                borrowed_system_call
                    .into_iter()
                    .map(|(source, rust)| (source, rust, true)),
            )
            .find_map(|(source, rust, borrowed)| {
                self.is_builtin(callee, &format!("intrinsic:system::{source}"))
                    .then_some((rust, borrowed))
            });
        if let Some((function, borrowed)) = system_call {
            let values = argument_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    if (function == "filesystem_list_directory" && index == 1)
                        || (function == "filesystem_call" && index == 4)
                        || function == "process_exit"
                    {
                        self.expression_as(value, ValueType::Scalar(ScalarType::Int))
                    } else {
                        self.expression(value)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            if borrowed {
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
            ("read-async", "read_async"),
            ("write", "write"),
            ("write-async", "write_async"),
            ("flush", "flush"),
            ("sync-data", "sync_data"),
            ("sync-all", "sync_all"),
            ("close", "close"),
            ("release", "release"),
        ]
        .into_iter()
        .find_map(|(terrane, rust)| {
            self.is_builtin(callee, &format!("intrinsic:streams::{terrane}"))
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
            if matches!(function, "read_async" | "write_async") {
                let mut arguments = vec![format!("&({})", values[0])];
                arguments.extend(values[1..values.len() - 1].iter().cloned());
                if function == "write_async" {
                    arguments[2] = self.adaptive_expression(argument_values[2]);
                }
                let cancellation = self.expression_as(
                    argument_values.last().expect("async stream cancellation"),
                    ValueType::Optional(Box::new(ValueType::PlatformCapability)),
                );
                arguments.push(format!("({cancellation}).as_ref()"));
                return format!("terrane_platform_{function}({})", arguments.join(", "));
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
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
        {
            let member_name = self.text(member);
            let receiver_type = self.receiver_value_type(receiver);
            if member_name == "append-utf8-ranges"
                && receiver_type == Some(ValueType::Scalar(ScalarType::Bytes))
                && let [output, delimiter, separator, accepts] = argument_values.as_slice()
            {
                return self.append_utf8_ranges(
                    receiver, arguments, output, delimiter, separator, accepts,
                );
            }
            if member_name == "to-string"
                && matches!(receiver_type, Some(ValueType::Scalar(scalar)) if scalar != ScalarType::Bytes)
            {
                return self.text_display(receiver);
            }
            if receiver_type == Some(ValueType::Scalar(ScalarType::String))
                && matches!(member_name, "concat" | "join")
            {
                let parts = arguments
                    .children
                    .iter()
                    .map(|argument| argument.children.last().unwrap_or(argument))
                    .collect::<Vec<_>>();
                if member_name == "join" {
                    return self.compose_text(&parts, Some(receiver));
                }
                let parts = std::iter::once(receiver).chain(parts).collect::<Vec<_>>();
                return self.compose_text(&parts, None);
            }
        }
        if callee.kind == SyntaxKind::MemberExpression
            && callee
                .children
                .get(1)
                .is_some_and(|member| self.text(member) == "concat")
            && self.value_type(&callee.children[0]) == Some(ValueType::Scalar(ScalarType::Bytes))
        {
            let values = argument_values
                .iter()
                .map(|value| self.expression(value))
                .collect::<Vec<_>>();
            let receiver = self.receiver_expression(&callee.children[0]);
            let bindings = values
                .iter()
                .enumerate()
                .map(|(index, value)| format!("let part_{index}: Vec<u8> = {value};"))
                .collect::<Vec<_>>()
                .join(" ");
            let part_lengths = (0..values.len())
                .map(|index| format!("part_{index}.len()"))
                .collect::<Vec<_>>()
                .join(", ");
            let extensions = (0..values.len())
                .map(|index| format!("bytes.extend(part_{index});"))
                .collect::<Vec<_>>()
                .join(" ");
            // `bytes.concat` is infallible at the Terrane surface. A combined length outside
            // `usize` or a failed reservation therefore cannot become a normal language
            // result. Abort before extension so neither Rust unwinding nor partial mutation
            // crosses the generated-code boundary.
            return format!(
                "{{ let mut bytes = {receiver}; {bindings} let additional = match [{part_lengths}].into_iter().try_fold(0usize, usize::checked_add) {{ Some(length) => length, None => std::process::abort() }}; if bytes.try_reserve(additional).is_err() {{ std::process::abort(); }} {extensions} bytes }}"
            );
        }
        self.emit_invocation(
            &InvocationSite {
                node,
                source_callee,
                callee,
                arguments,
                source_method_application,
            },
            bridge,
            argument_values,
            argument_first,
        )
    }
}
