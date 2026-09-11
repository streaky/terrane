use super::super::prelude::*;

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
        let source = self.expression_as(values[0], ValueType::Scalar(ScalarType::String));
        let destination = self.text(values[1]);
        let object = self
            .unit
            .descriptors
            .iter()
            .find(|object| object.name == destination)
            .expect("typed document semantics retained the destination class");
        let destination_type = rust_object_type_name(self.package, &object.identity);
        let options = self.expression(values[2]);
        let allow_unknown = self.expression_as(values[3], ValueType::Scalar(ScalarType::Bool));
        let (line, column) = self.source.line_column(node.span.start);
        let source_site = format!("{}:{line}:{column}", self.unit.source_path);
        let parse = if identity == "/core/documents/json::decode-typed-json" {
            "terrane_document_support::parse_json(&source, terrane_limit(&options.max_depth), terrane_limit(&options.max_bytes))".to_owned()
        } else {
            "terrane_document_support::parse_yaml(&source, terrane_limit(&options.max_depth), terrane_limit(&options.max_bytes), terrane_limit(&options.max_alias_nodes))".to_owned()
        };
        format!(
            "{{ let source = {source}; let options = {options}; let input = {parse}; match <{destination_type} as TerraneDocumentDecode>::terrane_decode_document(&input, \"$\", {allow_unknown}, {source_site:?}, {source_site:?}) {{ Ok(value) => TerraneDocumentDecodeOutcome {{ value, diagnostics: terrane_collection_support::List::new(Vec::new()) }}, Err(diagnostics) => TerraneDocumentDecodeOutcome {{ value: {destination_type}::terrane_construct(), diagnostics: terrane_collection_support::List::new(diagnostics) }} }} }}"
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
                format!("({}).clone()", self.expression(value))
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

    #[expect(
        clippy::too_many_lines,
        reason = "all call forms share one ordering and error-propagation path"
    )]
    pub(super) fn call(&mut self, node: &SyntaxNode) -> String {
        let [callee, arguments] = node.children.as_slice() else {
            return String::new();
        };
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
            let name = format!("({}).clone()", self.expression(values[0]));
            let value = self.expression_as(
                values[1],
                ValueType::Object(ObjectIdentity {
                    namespace: "/core/logging".to_owned(),
                    name: "log-value".to_owned(),
                }),
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
                    format!("({}).clone()", self.expression(value))
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
            return format!("terrane_log_error(({}).clone())", self.expression(argument));
        }
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
            && self.receiver_value_type(receiver) == Some(ValueType::TaskScope)
        {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "spawn" => arguments.children.first().map_or_else(String::new, |argument| {
                    let callable = argument.children.last().unwrap_or(argument);
                    let callable_type = self.value_type(callable);
                    let throws = self
                        .contract_for_call(callable)
                        .is_some_and(|contract| contract.throws)
                        || matches!(
                            callable_type,
                            Some(ValueType::AsyncFunction(_, _, _, ref effects))
                                if effects.requires_throwing_abi()
                        );
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
                    if foreign_error {
                        format!(
                            "{{ let __terrane_scope = ({receiver}).clone(); let __terrane_cancel = __terrane_scope.cancellation(); let __terrane_deadline = __terrane_scope.deadline; {task_setup}TerraneScopedTask::spawn(async move {{ match __terrane_cancellable({invocation}, __terrane_cancel, __terrane_deadline).await {{ Some(Ok(value)) => TerraneTaskResult::Completed(value), Some(Err(error)) => TerraneTaskResult::Failed(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)), None => TerraneTaskResult::Cancelled }} }}) }}"
                        )
                    } else if throws {
                        format!(
                            "{{ let __terrane_scope = ({receiver}).clone(); let __terrane_cancel = __terrane_scope.cancellation(); let __terrane_deadline = __terrane_scope.deadline; {task_setup}TerraneScopedTask::spawn(async move {{ match __terrane_cancellable({invocation}, __terrane_cancel, __terrane_deadline).await {{ Some(Ok(value)) => TerraneTaskResult::Completed(value), Some(Err(error)) => TerraneTaskResult::Failed(error), None => TerraneTaskResult::Cancelled }} }}) }}"
                        )
                    } else {
                        format!(
                            "{{ let __terrane_scope = ({receiver}).clone(); let __terrane_cancel = __terrane_scope.cancellation(); let __terrane_deadline = __terrane_scope.deadline; {task_setup}TerraneScopedTask::spawn(async move {{ match __terrane_cancellable({invocation}, __terrane_cancel, __terrane_deadline).await {{ Some(value) => TerraneTaskResult::Completed(value), None => TerraneTaskResult::Cancelled }} }}) }}"
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
            && self.is_throwable_value(receiver)
            && self.text(member) == "render"
        {
            return format!("({}).render()", self.expression(receiver));
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
                (ValueType::List(_), "clear") => Some(format!(
                    "({{ let collection = &mut ({receiver_value}); collection.clear(); collection.clone() }})"
                )),
                (ValueType::List(_), "remove") => {
                    let index = self.expression_as(values[0], ValueType::Scalar(ScalarType::Int));
                    let index = self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index}))"),
                        node,
                    );
                    Some(self.fallible(format!("({receiver_value}).remove({index})"), node))
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
            let values = argument_values
                .iter()
                .map(|value| self.display_expression(value))
                .map(|value| format!("terrane_scalar_support::scalar_text(&({value}))"))
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
                                | "platform_tcp_accept_async"
                                | "platform_tcp_read"
                                | "platform_tcp_read_async"
                                | "platform_tcp_write"
                                | "platform_tcp_write_async"
                                | "platform_tcp_shutdown"
                                | "platform_tcp_configure"
                                | "platform_udp_send_to"
                                | "platform_udp_send_to_async"
                                | "platform_udp_receive_from"
                                | "platform_udp_receive_from_async"
                                | "platform_udp_configure"
                                | "platform_tls_client"
                                | "platform_tls_client_async"
                                | "platform_tls_read"
                                | "platform_tls_read_async"
                                | "platform_tls_write"
                                | "platform_tls_write_async"
                                | "platform_tls_shutdown"
                                | "platform_tls_shutdown_async"
                                | "platform_capability_close"
                                | "platform_digest"
                                | "platform_hmac"
                                | "platform_parse_socket"
                                | "platform_cancel"
                                | "platform_destroy_secret",
                            0,
                        ) | ("platform_parse_socket" | "platform_hmac", 1)
                            | (
                                "platform_tcp_connect"
                                    | "platform_tcp_connect_async"
                                    | "platform_tcp_accept"
                                    | "platform_tcp_accept_async"
                                    | "platform_tls_shutdown"
                                    | "platform_tls_shutdown_async",
                                2
                            )
                            | (
                                "platform_tcp_connect_host"
                                    | "platform_tcp_connect_host_async"
                                    | "platform_tcp_read"
                                    | "platform_tcp_read_async"
                                    | "platform_tcp_write"
                                    | "platform_tcp_write_async"
                                    | "platform_udp_receive_from"
                                    | "platform_udp_receive_from_async"
                                    | "platform_dns_lookup"
                                    | "platform_dns_lookup_async"
                                    | "platform_tls_client"
                                    | "platform_tls_client_async"
                                    | "platform_tls_read"
                                    | "platform_tls_read_async"
                                    | "platform_tls_write"
                                    | "platform_tls_write_async",
                                3,
                            )
                            | ("platform_udp_send_to" | "platform_udp_send_to_async", 4)
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
            self.is_builtin(callee, &format!("intrinsic:system::{terrane}"))
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
            ("read-async", "read_async"),
            ("write", "write"),
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
        let projected_chain_role = self
            .projected_function_for_call(callee)
            .and_then(|function| function.chain_role);
        let projected_chain_root = projected_chain_role == Some(crate::projection::ChainRole::Root);
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
                let expression = if let Some(projected) = projected_parameters
                    .as_ref()
                    .and_then(|parameters| parameters.get(index))
                    .filter(|_| {
                        projected_chain_role.is_some()
                            || callee.kind == SyntaxKind::MemberExpression
                    }) {
                    projected_chain_argument_expression(&expression, &projected.ty)
                } else {
                    expression
                };
                ordered[index] = Some(
                    projected_parameters
                        .as_ref()
                        .and_then(|parameters| parameters.get(index))
                        .filter(|parameter| {
                            parameter.borrowed
                                && (projected_chain_root
                                    || matches!(
                                        parameter.ty,
                                        crate::projection::ProjectedType::Foreign { .. }
                                    ))
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
        } else if let Some(
            ValueType::Function(parameters, _, _) | ValueType::AsyncFunction(parameters, _, _, _),
        ) = self.value_type(callee)
        {
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
        let specialization = self.unit.projected_call_specializations.get(&(
            node.span.file,
            node.span.start,
            node.span.end,
        ));
        let projected_static_owner = contract.as_ref().and_then(|contract| {
            let owner = contract.owner.as_deref()?;
            let unit = self.package.units.iter().find(|unit| {
                unit.source.id() == contract.span.file && unit.namespace.starts_with("/deps/")
            })?;
            Some(
                unit.descriptors
                    .iter()
                    .find(|object| object.identity.name == owner)
                    .map_or(owner, |object| object.name.as_str()),
            )
        });
        let name = if callee.kind == SyntaxKind::ConstructionExpression {
            callee
                .children
                .first()
                .and_then(|designator| self.class_designator(designator))
                .map_or_else(String::new, |object| {
                    format!(
                        "{}::terrane_construct",
                        rust_object_type_name(self.package, &object.identity)
                    )
                })
        } else if let [receiver, member] = callee.children.as_slice()
            && callee.kind == SyntaxKind::StaticMemberExpression
            && let Some(object) = self.class_designator(receiver)
        {
            let rust_type = rust_object_type_name(self.package, &object.identity);
            if let Some(owner) = projected_static_owner {
                crate::lowering::dependencies::projected_static_shim_name(owner, self.text(member))
            } else {
                format!(
                    "{rust_type}::terrane_static_{}",
                    rust_name(self.text(member))
                )
            }
        } else if let Some(contract) = &contract
            && contract.owner.is_none()
        {
            self.package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .and_then(|symbol| {
                    self.package
                        .projection
                        .item(&symbol.namespace, &symbol.name)
                })
                .filter(|item| {
                    matches!(
                        &item.kind,
                        crate::projection::ProjectedKind::Function(function)
                            if function.chain_role == Some(crate::projection::ChainRole::Root)
                    )
                })
                .map_or_else(
                    || function_name(self.package, contract),
                    |item| item.rust_path.clone(),
                )
        } else if let [receiver, member] = callee.children.as_slice()
            && self.callable_object_field(receiver, self.text(member))
        {
            format!(
                "({}.{})",
                self.expression(receiver),
                rust_name(self.text(member))
            )
        } else if contract
            .as_ref()
            .is_some_and(|contract| contract.owner.is_some())
            && let [receiver, member] = callee.children.as_slice()
        {
            let contract = contract.as_ref().expect("method contract exists");
            let receiver_expression = self.receiver_expression(receiver);
            let projected_receiver = self.value_type(receiver).and_then(|value_type| {
                let ValueType::Object(identity) = value_type else {
                    return None;
                };
                self.package
                    .projection
                    .method(&identity.namespace, &identity.name, &contract.name, false)
                    .and_then(|method| method.receiver)
            });
            let receiver = if contract.is_async {
                match projected_receiver {
                    Some(crate::projection::Receiver::MutableBorrow) => {
                        format!("&mut {receiver_expression}")
                    }
                    Some(crate::projection::Receiver::Borrow) => {
                        format!("&{receiver_expression}")
                    }
                    Some(crate::projection::Receiver::Move) => receiver_expression,
                    _ if contract.written_invocation_mode == InvocationMode::Mutable => {
                        format!("&mut {receiver_expression}")
                    }
                    _ if contract.written_invocation_mode == InvocationMode::Shared => {
                        format!("&{receiver_expression}")
                    }
                    _ => receiver_expression,
                }
            } else {
                receiver_expression
            };
            format!("({receiver}).{}", rust_name(self.text(member)))
        } else {
            self.expression(callee)
        };
        // Destination specializations only attach to projected free/static paths or projected
        // member access. Each branch above ends in a callable Rust path/member segment, so an
        // explicit turbofish is syntactically valid here; arbitrary callee expressions never
        // receive a specialization record.
        let name = if let Some(specialization) = specialization {
            format!("{name}::<{}>", specialization.rust_type)
        } else {
            name
        };
        let callable_mode =
            self.value_type(callee)
                .and_then(|value_type| match value_type {
                    ValueType::Function(_, _, effects)
                    | ValueType::AsyncFunction(_, _, _, effects) => Some(effects.modes.written),
                    _ => None,
                });
        let call = if contract.is_none()
            && matches!(
                callable_mode,
                Some(InvocationMode::Mutable | InvocationMode::Consuming)
            ) {
            let arguments = match values.as_slice() {
                [] => "()".to_owned(),
                [value] => format!("({value},)"),
                _ => format!("({})", values.join(", ")),
            };
            format!("{name}.call({arguments})")
        } else {
            format!("{name}({})", values.join(", "))
        };
        let foreign_method = contract.as_ref().and_then(|contract| {
            let [receiver, _member] = callee.children.as_slice() else {
                return None;
            };
            let ValueType::Object(identity) = self.value_type(receiver)? else {
                return None;
            };
            self.package.projection.method(
                &identity.namespace,
                &identity.name,
                &contract.name,
                false,
            )
        });
        let chain_role = foreign_method
            .and_then(|method| method.chain_role)
            .or_else(|| {
                if callee.kind != SyntaxKind::Name {
                    return None;
                }
                self.package
                    .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                    .and_then(|symbol| {
                        self.package
                            .projection
                            .item(&symbol.namespace, &symbol.name)
                    })
                    .and_then(|item| match &item.kind {
                        crate::projection::ProjectedKind::Function(function) => function.chain_role,
                        _ => None,
                    })
            });
        if matches!(
            chain_role,
            Some(crate::projection::ChainRole::Root | crate::projection::ChainRole::Continue)
        ) {
            return call;
        }
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
            let invocation = if method.is_async {
                "__terrane_call.await".to_owned()
            } else {
                call.clone()
            };
            let unwind_call = if !method.is_async
                && method.error.is_none()
                && self.discarded_call == Some(node.span)
            {
                format!("{{ {call}; }}")
            } else {
                call.clone()
            };
            let caught = if method.is_async {
                "crate::__terrane_dependency_await_unwind(__terrane_call).await".to_owned()
            } else if method.receiver.is_some() {
                format!("std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {unwind_call}))")
            } else {
                format!("std::panic::catch_unwind(|| {unwind_call})")
            };
            let projected_result = specialization.map_or(&method.result, |specialization| {
                &specialization.projected_result
            });
            let converted = projected_result_expression("value", projected_result);
            let mapped = if self.package.profile.panic == crate::package::PanicProfile::Abort {
                if method.error.is_some() {
                    format!(
                        "match {invocation} {{ Ok(value) => Ok({converted}), Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{dependency}` member `{member}` failed: {{error}}\"), crate::TERRANE_NO_SITE))) }}"
                    )
                } else if self.discarded_call == Some(node.span) {
                    format!("{{ {invocation}; Ok(()) }}")
                } else {
                    format!(
                        "Ok({})",
                        projected_result_expression(&invocation, projected_result)
                    )
                }
            } else if method.error.is_some() {
                format!(
                    "match {caught} {{ Ok(Ok(value)) => Ok({converted}), Ok(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{dependency}` member `{member}` failed: {{error}}\"), crate::TERRANE_NO_SITE))), Err(payload) => Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
                )
            } else {
                format!(
                    "match {caught} {{ Ok(value) => Ok({converted}), Err(payload) => Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
                )
            };
            if method.is_async {
                format!("{{ let __terrane_call = {call}; async move {{ {mapped} }} }}")
            } else {
                mapped
            }
        } else {
            call
        };
        let function_value_call = contract.is_none()
            && matches!(
                self.value_type(callee),
                Some(ValueType::Function(_, _, effects)) if effects.requires_throwing_abi()
            )
            && (callee.kind == SyntaxKind::Name
                || matches!(
                    callee.children.as_slice(),
                    [receiver, member]
                        if self.callable_object_field(receiver, self.text(member))
                ));
        let dependency_contract = contract.as_ref().is_some_and(|contract| {
            self.package.units.iter().any(|unit| {
                unit.source.id() == contract.span.file && unit.namespace.starts_with("/deps/")
            })
        });
        let needs_error_mapping = contract.as_ref().is_some_and(|contract| contract.throws)
            || dependency_contract
            || foreign_error
            || function_value_call;
        let site = if needs_error_mapping {
            self.error_site(node)
        } else {
            String::new()
        };
        let dependency_boundary = dependency_contract
            || self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .is_some_and(|symbol| symbol.identity.starts_with("/deps/"));
        let map_errors = |call: &str| {
            if !needs_error_mapping {
                return call.to_owned();
            }
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
        };
        if contract.as_ref().is_some_and(|contract| contract.is_async)
            && matches!(self.value_type(node), Some(ValueType::Task(_, _)))
        {
            if foreign_error || dependency_boundary {
                let completed = format!("__terrane_raised_err(__terrane_future.await, {site})");
                let completed = if foreign_method.is_none()
                    && let Some(specialization) = specialization
                {
                    projected_result_expression(&completed, &specialization.projected_result)
                } else {
                    completed
                };
                format!("{{ let __terrane_future = {call}; async move {{ {completed} }} }}")
            } else {
                call
            }
        } else {
            let mapped = map_errors(&call);
            if foreign_method.is_none()
                && let Some(specialization) = specialization
            {
                projected_result_expression(&mapped, &specialization.projected_result)
            } else {
                mapped
            }
        }
    }
}
