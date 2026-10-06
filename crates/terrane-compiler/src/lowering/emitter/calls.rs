use super::super::prelude::*;

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

impl Emitter<'_> {
    fn native_macro_expression(&mut self, node: &SyntaxNode) -> String {
        use crate::rust_interop::projection::ProjectedType;
        let mut node = node;
        while node.kind == SyntaxKind::GroupExpression && node.children.len() == 1 {
            node = &node.children[0];
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && let Some(item) =
                crate::semantics::projected_macro_for_call(self.package, self.unit, callee)
        {
            let path = item.rust_path.clone();
            let values = arguments
                .children
                .iter()
                .map(|argument| {
                    self.native_macro_expression(argument.children.last().unwrap_or(argument))
                })
                .collect::<Vec<_>>();
            return format!("{path}!({})", values.join(", "));
        }
        let parameter = crate::semantics::macro_argument(self.package, self.unit, node, 0)
            .expect("semantic macro proof records a concrete native argument");
        if node.kind == SyntaxKind::Literal {
            let token = native_macro_literal(self.text(node));
            return match parameter.ty {
                ProjectedType::Int => format!("{token}_i64"),
                ProjectedType::Float => format!("{token}_f64"),
                _ => token,
            };
        }
        if parameter.borrowed {
            let operand = node
                .children
                .last()
                .filter(|_| node.kind == SyntaxKind::UnaryExpression)
                .unwrap_or(node);
            let value = self.native_receiver_expression(operand, parameter.mutable_borrow);
            return format!(
                "&{}({value})",
                if parameter.mutable_borrow { "mut " } else { "" }
            );
        }
        let value = self.expression(node);
        projected_chain_argument_expression(&value, &parameter.ty)
    }

    fn borrowed_native_callback_adapter(
        &self,
        value: &SyntaxNode,
        projected: &crate::rust_interop::projection::ProjectedType,
    ) -> Option<String> {
        let crate::rust_interop::projection::ProjectedType::Callback {
            parameters,
            parameter_rust_types,
            parameter_borrows,
            result,
            retained: false,
            is_async: false,
            ..
        } = projected
        else {
            return None;
        };
        if value.kind != SyntaxKind::Name
            || !parameter_borrows.iter().any(|borrowed| *borrowed)
            || matches!(
                result.as_ref(),
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
            )
            || parameter_rust_types.iter().any(|ty| ty.contains('\''))
        {
            return None;
        }
        let contract = self.contract_for_call(value, false)?;
        let function = function_name(self.package, contract);
        let declarations = parameter_rust_types
            .iter()
            .enumerate()
            .map(|(index, _)| format!("borrowed_argument_{index}"))
            .collect::<Vec<_>>()
            .join(", ");
        let arguments = parameters
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                projected_callback_input_expression(
                    &format!("borrowed_argument_{index}"),
                    ty,
                    &parameter_rust_types[index],
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        let call = format!("{function}({arguments})");
        let call = if self.contract_requires_throwing_abi(contract, false) {
            format!("{call}.map_err(crate::TerraneForeignError)?")
        } else {
            call
        };
        let result = projected_callback_output_expression("callback_value", result);
        Some(format!(
            "move |{declarations}| {{ match (|| -> Result<_, crate::TerraneForeignError> {{ let callback_value = {call}; Ok({result}) }})() {{ Ok(value) => value, Err(error) => std::panic::panic_any(error.0) }} }}"
        ))
    }

    fn invocation_scoped_callback_adapter(
        &mut self,
        value: &SyntaxNode,
        projected: &crate::rust_interop::projection::ProjectedType,
        substitutions: Option<
            &std::collections::BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
        >,
    ) -> Option<String> {
        let crate::rust_interop::projection::ProjectedType::Callback {
            native_substitutions,
            result,
            invocation_mode: InvocationMode::Shared,
            ..
        } = projected
        else {
            return None;
        };
        let crate::rust_interop::projection::ProjectedType::InvocationScoped {
            rust_type: producer_template,
            lifetimes,
            ..
        } = result.as_ref()
        else {
            return None;
        };
        let mut selected = substitutions.cloned().unwrap_or_default();
        selected.extend(native_substitutions.clone());
        if value.kind != SyntaxKind::Name {
            return None;
        }
        let callback = self.expression(value);
        let name = self.text(value);
        let source_node = self
            .unit
            .functions
            .iter()
            .find(|function| function.name == name)
            .and_then(|function| {
                find_node(
                    &self.unit.tree.root,
                    SyntaxKind::FunctionDeclaration,
                    function.span,
                )
            });
        let generic_arguments = source_node
            .map(|source| {
                self.invocation_scoped_type_generics(source, producer_template, lifetimes)
                    .into_iter()
                    .filter_map(|declaration| {
                        let name = declaration
                            .split_once(':')
                            .map_or(declaration.as_str(), |(name, _)| name)
                            .trim();
                        selected
                            .get(name)
                            .map(crate::rust_interop::projection::ProjectedType::rust_type)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        Some(if generic_arguments.is_empty() {
            callback
        } else {
            format!("{callback}::<{}>", generic_arguments.join(", "))
        })
    }

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
        let [source_callee, arguments] = node.children.as_slice() else {
            return String::new();
        };
        let is_unsafe =
            crate::syntax::call_is_unsafe(node) || crate::syntax::call_is_unsafe(source_callee);
        let projected_application = self
            .projected_function_for_call(source_callee, is_unsafe)
            .is_some();
        let mut callee = source_callee;
        if projected_application {
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
            && self.receiver_value_type(receiver) == Some(ValueType::TaskScope)
        {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "spawn" => arguments.children.first().map_or_else(String::new, |argument| {
                    let callable = argument.children.last().unwrap_or(argument);
                    let callable_type = self.value_type(callable);
                    let direct_contract = self.contract_for_call(callable, false);
                    let throws = direct_contract.map_or_else(
                        || {
                            matches!(
                                callable_type,
                                Some(ValueType::AsyncFunction(_, _, _, ref effects))
                                    if effects.requires_throwing_abi()
                            )
                        },
                        |contract| contract.throws,
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
            && member.contains('.')
        {
            if matches!(member.as_str(), "get.checked" | "remove.checked")
                && let Some(argument) = arguments.children.first()
            {
                let remove = member == "remove.checked";
                let argument = argument.children.last().unwrap_or(argument);
                let receiver_value = if remove {
                    self.receiver_guard_expression(receiver)
                } else {
                    self.receiver_expression(receiver)
                };
                let call = match receiver_type {
                    ValueType::List(_) | ValueType::Tuple(_, _) => {
                        let index =
                            self.expression_as(argument, ValueType::Scalar(ScalarType::Int));
                        format!(
                            "terrane_collection_support::index_from_int(&({index})).ok().and_then(|index| ({receiver_value}).get(index).cloned())"
                        )
                    }
                    ValueType::Map(key, _) | ValueType::UnorderedMap(key, _) => {
                        let key = self.expression_as(argument, key.value_type());
                        if remove {
                            format!("({receiver_value}).remove_checked(&({key}))")
                        } else {
                            format!("({receiver_value}).get(&({key})).cloned()")
                        }
                    }
                    _ => String::new(),
                };
                return if remove {
                    self.wrap_receiver_guard(receiver, call)
                } else {
                    call
                };
            }
            if member == "sort.descending"
                && let ValueType::List(item) = receiver_type
            {
                let receiver_value = self.receiver_guard_expression(receiver);
                let comparator = list_sort_comparator(&item, true);
                let call = format!(
                    "({{ let collection = &mut ({receiver_value}); collection.sort_by({comparator}); collection.clone() }})"
                );
                return self.wrap_receiver_guard(receiver, call);
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
                    Some(self.fallible(format!("({receiver_value}).remove({index})"), node))
                }
                (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "set") => {
                    Some(format!(
                        "({{ let collection = &mut ({receiver_value}); collection.set({}, {}); collection.clone() }})",
                        self.expression_as(values[0], key.value_type()),
                        self.expression_as(values[1], value.value_type())
                    ))
                }
                (ValueType::Map(key, _) | ValueType::UnorderedMap(key, _), "remove") => {
                    let key = self.expression_as(values[0], key.value_type());
                    Some(self.fallible(format!("({receiver_value}).remove(&({key}))"), node))
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
            ("platform-value-from-bytes", "platform_value_from_bytes"),
            ("platform-value-from-text", "platform_value_from_text"),
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
        let native_macro_path =
            crate::semantics::projected_macro_for_call(self.package, self.unit, callee)
                .map(|item| item.rust_path.clone());
        let mut values = argument_values
            .into_iter()
            .map(|value| {
                if native_macro_path.is_some() {
                    self.native_macro_expression(value)
                } else {
                    self.expression(value)
                }
            })
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
            values.insert(0, receiver);
            let values = values
                .into_iter()
                .map(|value| format!("terrane_scalar_support::scalar_text(&({value}))"))
                .collect::<Vec<_>>();
            let format = "{}".repeat(values.len());
            return format!("format!(\"{format}\", {})", values.join(", "));
        }
        let specialization = self.unit.projected_call_specializations.get(&(
            node.span.file,
            node.span.start,
            node.span.end,
        ));
        let projected_function = self
            .projected_function_for_call(source_callee, is_unsafe)
            .cloned();
        let (projected_parameters, projected_chain_role, projected_error) = projected_function
            .as_ref()
            .map_or((None, None, false), |function| {
                (
                    Some(specialization.map_or_else(
                        || function.parameters.clone(),
                        |specialization| specialization.projected_parameters.clone(),
                    )),
                    function.chain_role,
                    function.error.is_some(),
                )
            });
        let projected_chain_root =
            projected_chain_role == Some(crate::rust_interop::projection::ChainRole::Root);
        let projected_interface_dispatch = callee
            .children
            .first()
            .and_then(|receiver| self.value_type(receiver))
            .and_then(|value_type| match value_type {
                ValueType::Object(identity) => Some(identity),
                _ => None,
            })
            .is_some_and(|identity| {
                self.package
                    .units
                    .iter()
                    .flat_map(|unit| &unit.descriptors)
                    .any(|descriptor| {
                        descriptor.identity == identity
                            && descriptor.kind == crate::semantics::ObjectKind::Interface
                    })
            });
        let contract = if native_macro_path.is_some() {
            None
        } else {
            crate::semantics::selected_callable_contract(self.package, self.unit, node, is_unsafe)
                .or_else(|| self.contract_for_call(callee, is_unsafe).cloned())
        }
        .map(|mut contract| {
            if let Some(canonical) = self
                .package
                .units
                .iter()
                .flat_map(|unit| &unit.functions)
                .find(|function| function.span == contract.span)
            {
                for (parameter, current) in
                    contract.parameters.iter_mut().zip(&canonical.parameters)
                {
                    parameter.mutable = current.mutable;
                }
            }
            contract
        });
        let native_struct_construction = callee.kind == SyntaxKind::ConstructionExpression
            && callee
                .children
                .first()
                .and_then(|designator| self.class_designator(designator))
                .and_then(|object| {
                    self.package
                        .projection
                        .projected_struct(&object.identity.namespace, &object.identity.name)
                })
                .is_some();
        let mut native_field_values = None;
        let mut native_optional_arguments = Vec::new();
        if let Some(contract) = &contract {
            let mut ordered = vec![None; contract.parameters.len()];
            if native_struct_construction {
                native_optional_arguments.resize(contract.parameters.len(), false);
            }
            let mut variadic_values = Vec::new();
            let variadic_index = contract
                .parameters
                .iter()
                .position(|parameter| parameter.variadic);
            let mut positional = 0;
            for argument in &arguments.children {
                let named = argument
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
                let index = named.map_or_else(
                    || {
                        let index = variadic_index
                            .filter(|variadic| positional >= *variadic)
                            .unwrap_or(positional);
                        positional += 1;
                        index
                    },
                    |name| {
                        contract
                            .parameters
                            .iter()
                            .position(|parameter| parameter.name == self.text(name))
                            .or_else(|| {
                                (callee.kind == SyntaxKind::ConstructionExpression)
                                    .then(|| callee.children.first())
                                    .flatten()
                                    .and_then(|designator| self.class_designator(designator))
                                    .and_then(|object| {
                                        self.package.projection.projected_struct(
                                            &object.identity.namespace,
                                            &object.identity.name,
                                        )
                                    })
                                    .and_then(|(_, fields, _)| {
                                        fields
                                            .iter()
                                            .position(|field| field.name == self.text(name))
                                    })
                            })
                            .expect("validated named argument")
                    },
                );
                let value = argument.children.last().unwrap_or(argument);
                let parameter = &contract.parameters[index];
                let projected_parameter = projected_parameters
                    .as_ref()
                    .and_then(|parameters| parameters.get(index));
                let semantic_parameter = specialization
                    .and_then(|specialization| specialization.value_parameters.get(index))
                    .cloned()
                    .flatten()
                    .or_else(|| parameter.element_value_type());
                let explicit_reference = [argument, value].into_iter().find_map(|candidate| {
                    (candidate.kind == SyntaxKind::UnaryExpression
                        && self.unary_operator(candidate).as_deref() == Some("ref"))
                    .then(|| candidate.children.last())
                    .flatten()
                });
                let invocation_callback = projected_parameter
                    .map(|parameter| parameter.ty.clone())
                    .and_then(|projected| {
                        self.invocation_scoped_callback_adapter(
                            value,
                            &projected,
                            specialization.map(|specialization| &specialization.substitutions),
                        )
                    })
                    .or_else(|| {
                        projected_parameter
                            .map(|parameter| parameter.ty.clone())
                            .and_then(|projected| {
                                self.borrowed_native_callback_adapter(value, &projected)
                            })
                    });
                let has_invocation_callback = invocation_callback.is_some();
                let expression = if let Some(expression) = invocation_callback {
                    expression
                } else if value.kind == SyntaxKind::Name
                    && projected_parameter.is_some_and(|parameter| {
                        parameter.borrowed
                            && matches!(
                                parameter.ty,
                                crate::rust_interop::projection::ProjectedType::String
                                    | crate::rust_interop::projection::ProjectedType::Bytes
                            )
                    })
                    && (specialization
                        .is_some_and(|specialization| specialization.direct_projected_call)
                        || projected_chain_role.is_some()
                        || callee.kind == SyntaxKind::MemberExpression)
                {
                    self.raw_storage_name(value)
                } else if projected_parameter.is_some_and(|parameter| {
                    parameter.generic_parameter.is_some()
                        && (parameter.mutable_borrow || explicit_reference.is_some())
                }) {
                    if let Some(operand) = explicit_reference {
                        if self.narrowed_optional_name(operand).is_some() {
                            format!(
                                "&mut *{}.as_mut().expect(\"semantic optional narrowing\")",
                                self.raw_storage_name(operand)
                            )
                        } else {
                            format!("&mut {}", self.raw_storage_name(operand))
                        }
                    } else {
                        format!("&mut {}", self.expression(value))
                    }
                } else if projected_parameter.is_some_and(|parameter| parameter.mutable_borrow) {
                    format!("&mut {}", self.expression(value))
                } else if projected_parameter.is_some_and(|parameter| {
                    parameter.generic_parameter.is_some()
                        || parameter.borrowed
                            && matches!(
                                parameter.ty,
                                crate::rust_interop::projection::ProjectedType::Foreign { .. }
                            )
                        || matches!(
                            parameter.ty,
                            crate::rust_interop::projection::ProjectedType::BoxedInterface { .. }
                        )
                }) {
                    self.expression(value)
                } else if parameter.mutable
                    && matches!(
                        parameter.binding_value_type(),
                        Some(ValueType::Reference(item))
                            if matches!(
                                item.value_type(),
                                ValueType::Object(identity)
                                    if identity.native_projection.is_some()
                                        || self.package.projection.item(&identity.namespace, &identity.name).is_some()
                            )
                    )
                {
                    if let Some(operand) = explicit_reference {
                        if self.narrowed_optional_name(operand).is_some() {
                            format!(
                                "&mut *{}.as_mut().expect(\"semantic optional narrowing\")",
                                self.raw_storage_name(operand)
                            )
                        } else if matches!(self.value_type(operand), Some(ValueType::Reference(_)))
                        {
                            format!("&mut *{}", self.expression(operand))
                        } else {
                            format!("&mut {}", self.raw_storage_name(operand))
                        }
                    } else {
                        format!("&mut *{}", self.expression(value))
                    }
                } else if let Some(ty) = semantic_parameter.as_ref() {
                    self.expression_as(value, ty.clone())
                } else {
                    self.expression(value)
                };
                let expression = if let Some(projected) = projected_parameters
                    .as_ref()
                    .and_then(|parameters| parameters.get(index))
                    .filter(|parameter| {
                        !has_invocation_callback
                            && !matches!(
                                parameter.ty,
                                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                            )
                            && (parameter.generic_parameter.is_none()
                                || matches!(
                                    parameter.ty,
                                    crate::rust_interop::projection::ProjectedType::Callback { .. }
                                ))
                            && (specialization.is_some_and(|specialization| {
                                specialization.direct_projected_call
                            }) || projected_chain_role.is_some()
                                || callee.kind == SyntaxKind::MemberExpression)
                    })
                {
                    let sequence_conversion = match (&projected.ty, self.value_type(value)) {
                        (
                            crate::rust_interop::projection::ProjectedType::Sequence {
                                rust_path,
                                item,
                            },
                            Some(ValueType::List(actual)),
                        ) if (rust_path.starts_with("alloc::vec::Vec<")
                            || rust_path.starts_with("std::vec::Vec<"))
                            && (item.has_identity_representation()
                                || matches!(actual.value_type_ref(), ValueType::InvocationScopedNative { .. })) =>
                        {
                            let convert_items = match (item.as_ref(), actual.value_type_ref()) {
                                (
                                    crate::rust_interop::projection::ProjectedType::InvocationScoped {
                                        name: expected,
                                        ..
                                    },
                                    ValueType::InvocationScopedNative { family, .. },
                                ) => expected != &family.name,
                                _ => false,
                            };
                            let consume = if matches!(
                                actual.value_type_ref(),
                                ValueType::InvocationScopedNative { .. }
                            ) {
                                "into_unique_vec"
                            } else {
                                "into_vec"
                            };
                            Some(if convert_items {
                                format!(
                                    "{expression}.{consume}().into_iter().map(Into::into).collect::<Vec<_>>()"
                                )
                            } else {
                                format!("{expression}.{consume}()")
                            })
                        }
                        _ => None,
                    };
                    sequence_conversion.unwrap_or_else(|| {
                        projected_chain_argument_expression(&expression, &projected.ty)
                    })
                } else {
                    expression
                };
                let expression = projected_parameters
                    .as_ref()
                    .and_then(|parameters| parameters.get(index))
                    .filter(|parameter| {
                        parameter.borrowed
                            && !parameter.mutable_borrow
                            && !projected_interface_dispatch
                            && (specialization
                                .is_some_and(|specialization| specialization.direct_projected_call)
                                || projected_chain_root
                                || callee.kind == SyntaxKind::MemberExpression
                                || matches!(
                                    parameter.ty,
                                    crate::rust_interop::projection::ProjectedType::Foreign { .. }
                                ))
                    })
                    .map_or(expression.clone(), |parameter| {
                        match (&parameter.ty, parameter.mutable_borrow) {
                            (crate::rust_interop::projection::ProjectedType::String, false)
                                if parameter.generic_parameter.is_some() =>
                            {
                                format!("({expression}).as_str()")
                            }
                            (crate::rust_interop::projection::ProjectedType::Bytes, false)
                                if parameter.generic_parameter.is_some() =>
                            {
                                format!("({expression}).as_slice()")
                            }
                            (_, true) => format!("&mut {expression}"),
                            _ => format!("&{expression}"),
                        }
                    });
                if parameter.variadic {
                    variadic_values.push(expression);
                } else {
                    ordered[index] = Some(expression);
                    if native_struct_construction {
                        native_optional_arguments[index] =
                            matches!(self.value_type(value), Some(ValueType::Optional(_)));
                    }
                }
            }
            if let Some(index) = variadic_index {
                ordered[index] = Some(format!(
                    "terrane_collection_support::List::new(vec![{}])",
                    variadic_values.join(", ")
                ));
            }
            self.append_defaults(contract, &mut ordered);
            if native_struct_construction {
                native_field_values = Some(ordered);
                values.clear();
            } else {
                values = ordered.into_iter().flatten().collect();
            }
        } else if native_macro_path.is_none()
            && let Some(
                ValueType::Function(parameters, _, _)
                | ValueType::AsyncFunction(parameters, _, _, _),
            ) = self.value_type(callee)
        {
            let variadic = parameters
                .last()
                .filter(|parameter| parameter.is_variadic());
            let fixed = parameters.len() - usize::from(variadic.is_some());
            let callable = crate::semantics::callback_contract(self.package, self.unit, callee);
            values = arguments
                .children
                .iter()
                .take(fixed)
                .zip(&parameters)
                .enumerate()
                .map(|(index, (argument, parameter))| {
                    let value = argument.children.last().unwrap_or(argument);
                    let mutable_native_reference = callable.is_some_and(|contract| {
                        index < contract.parameters.len()
                            && self.mutable_native_reference_parameter(
                                contract,
                                index,
                                Some(parameter.value_type_ref()),
                            )
                    });
                    if mutable_native_reference {
                        let receiver = if value.kind == SyntaxKind::UnaryExpression
                            && self.unary_operator(value).as_deref() == Some("ref")
                        {
                            value.children.last().unwrap_or(value)
                        } else {
                            value
                        };
                        let native = self.native_receiver_expression(receiver, true);
                        if matches!(self.value_type(receiver), Some(ValueType::Reference(_))) {
                            format!("&mut *{native}")
                        } else {
                            format!("&mut {native}")
                        }
                    } else {
                        self.expression_as(value, parameter.value_type())
                    }
                })
                .collect();
            if let Some(parameter) = variadic {
                let tail = arguments
                    .children
                    .iter()
                    .skip(fixed)
                    .map(|argument| {
                        self.expression_as(
                            argument.children.last().unwrap_or(argument),
                            parameter.value_type(),
                        )
                    })
                    .collect::<Vec<_>>();
                values.push(format!(
                    "terrane_collection_support::List::new(vec![{}])",
                    tail.join(", ")
                ));
            }
        }
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
        let mut native_member_receiver = None;
        let name = if let Some(path) = &native_macro_path {
            path.clone()
        } else if let Some(function) = self.source_applied_function(callee) {
            function
        } else if callee.kind == SyntaxKind::ConstructionExpression {
            callee
                .children
                .first()
                .and_then(|designator| self.class_designator(designator))
                .map_or_else(String::new, |object| {
                    let identity = match self.value_type(node) {
                        Some(ValueType::Object(identity))
                            if identity.namespace == object.identity.namespace
                                && identity.name == object.identity.name
                                && identity.is_unsafe == object.identity.is_unsafe =>
                        {
                            identity
                        }
                        _ => object.identity.clone(),
                    };
                    format!(
                        "{}::terrane_construct",
                        rust_source_type_application(self.package, &identity)
                            .replacen('<', "::<", 1)
                    )
                })
        } else if let [receiver, member] = callee.children.as_slice()
            && callee.kind == SyntaxKind::StaticMemberExpression
            && let Some(object) = self.class_designator(receiver)
        {
            let rust_type = rust_object_type_name(self.package, &object.identity);
            if specialization.is_some_and(|specialization| specialization.direct_projected_call)
                && let Some(item) = self
                    .package
                    .projection
                    .item(&object.identity.namespace, &object.identity.name)
                && let Some(projected) = self.projected_function_for_call(callee, is_unsafe)
            {
                projected.native_path.clone().unwrap_or_else(|| {
                    format!(
                        "{}::{}",
                        crate::lowering::dependencies::rust_value_path(
                            projected.native_owner.as_deref().unwrap_or(&item.rust_path),
                        ),
                        rust_name(&projected.name)
                    )
                })
            } else if let Some(contract) = contract
                .as_ref()
                .filter(|_| projected_static_owner.is_some())
            {
                let name = crate::lowering::dependencies::projected_static_shim_name(
                    self.package,
                    contract,
                );
                let selected_identity = match self.value_type(node) {
                    Some(ValueType::Object(identity))
                        if contract.owner_identity.as_ref().is_some_and(|owner| {
                            identity.namespace == owner.namespace && identity.name == owner.name
                        }) =>
                    {
                        Some(identity)
                    }
                    _ => None,
                };
                let arguments = selected_identity
                    .as_ref()
                    .and_then(|identity| {
                        self.package
                            .projection
                            .item(&identity.namespace, &identity.name)
                    })
                    .and_then(|item| match &item.kind {
                        crate::rust_interop::projection::ProjectedKind::ForeignType {
                            generic_parameters,
                            ..
                        }
                        | crate::rust_interop::projection::ProjectedKind::Enum {
                            generic_parameters,
                            ..
                        } if !generic_parameters.is_empty() => {
                            selected_identity.as_ref().and_then(|identity| {
                                let generic_parameters = self
                                    .projected_function_for_call(callee, is_unsafe)
                                    .filter(|projected| projected.native_owner.is_some())
                                    .map_or(generic_parameters.as_slice(), |projected| {
                                        projected.operation_owner_generics.as_slice()
                                    });
                                if generic_parameters.is_empty() {
                                    return None;
                                }
                                Some(
                                    generic_parameters
                                        .iter()
                                        .map(|parameter| {
                                            identity
                                                .native_arguments
                                                .get(&parameter.name)
                                                .and_then(|value| {
                                                    crate::semantics::destination_projected_type(
                                                        self.package,
                                                        value,
                                                    )
                                                    .ok()
                                                })
                                                .map_or_else(
                                                    || "_".to_owned(),
                                                    |projected| projected.rust_type(),
                                                )
                                        })
                                        .collect::<Vec<_>>(),
                                )
                            })
                        }
                        _ => None,
                    });
                arguments.map_or(name.clone(), |arguments| {
                    format!("{name}::<{}>", arguments.join(", "))
                })
            } else {
                format!(
                    "{rust_type}::terrane_static_{}",
                    rust_name(self.text(member))
                )
            }
        } else if let Some(contract) = &contract
            && contract.owner.is_none()
        {
            let lookup_name = is_unsafe.then(|| format!("unsafe::{}", self.text(callee)));
            self.package
                .resolve_name_at(
                    self.unit,
                    callee.span.start,
                    lookup_name.as_deref().unwrap_or_else(|| self.text(callee)),
                )
                .and_then(|symbol| {
                    self.package
                        .projection
                        .item(&symbol.namespace, &symbol.name)
                })
                .filter(|item| {
                    matches!(&item.kind, crate::rust_interop::projection::ProjectedKind::Function(function)
                        if specialization.is_some_and(|specialization| specialization.direct_projected_call)
                            || function.chain_role == Some(crate::rust_interop::projection::ChainRole::Root))
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
        } else if let [receiver, _member] = callee.children.as_slice()
            && let Some(projected) = projected_function.as_ref()
            && let Some(native_path) = &projected.native_path
        {
            let receiver = match projected.receiver {
                Some(crate::rust_interop::projection::Receiver::Borrow) => {
                    format!("&{}", self.native_receiver_expression(receiver, false))
                }
                Some(crate::rust_interop::projection::Receiver::MutableBorrow) => {
                    format!("&mut {}", self.native_receiver_expression(receiver, true))
                }
                Some(crate::rust_interop::projection::Receiver::Move) => {
                    self.consuming_native_receiver_expression(receiver)
                }
                _ => self.receiver_expression(receiver),
            };
            native_member_receiver = Some(receiver);
            native_path.clone()
        } else if let [receiver, _member] = callee.children.as_slice()
            && let Some(projected) = projected_function.as_ref()
            && matches!(
                self.value_type(receiver),
                Some(ValueType::InvocationScopedNative {
                    expression_scoped: true,
                    ..
                })
            )
        {
            let mutable = matches!(
                projected.receiver,
                Some(crate::rust_interop::projection::Receiver::MutableBorrow)
            );
            let receiver = self.native_receiver_expression(receiver, mutable);
            format!("({receiver}).{}", projected.name)
        } else if contract
            .as_ref()
            .is_some_and(|contract| contract.owner.is_some())
            && let [receiver, _member] = callee.children.as_slice()
        {
            let contract = contract.as_ref().expect("method contract exists");
            let projected_receiver = self
                .value_type(receiver)
                .and_then(|value_type| match value_type {
                    ValueType::Object(identity) => Some(identity),
                    ValueType::Reference(item) | ValueType::SharedReference(item) => {
                        let ValueType::Object(identity) = item.value_type() else {
                            return None;
                        };
                        Some(identity.clone())
                    }
                    _ => None,
                })
                .and_then(|identity| {
                    self.package
                        .projection
                        .method(&identity.namespace, &identity.name, &contract.name, false)
                        .and_then(|method| method.receiver)
                });
            let receiver_expression = if !contract.is_async
                && matches!(
                    projected_receiver,
                    Some(
                        crate::rust_interop::projection::Receiver::Borrow
                            | crate::rust_interop::projection::Receiver::MutableBorrow
                    )
                ) {
                self.native_receiver_expression(
                    receiver,
                    matches!(
                        projected_receiver,
                        Some(crate::rust_interop::projection::Receiver::MutableBorrow)
                    ),
                )
            } else if projected_receiver == Some(crate::rust_interop::projection::Receiver::Move) {
                self.consuming_native_receiver_expression(receiver)
            } else {
                self.receiver_expression(receiver)
            };
            let receiver = if contract.is_async {
                match projected_receiver {
                    Some(crate::rust_interop::projection::Receiver::MutableBorrow) => {
                        self.mutable_receiver_expression(receiver)
                    }
                    Some(crate::rust_interop::projection::Receiver::Borrow) => {
                        format!("&{receiver_expression}")
                    }
                    Some(crate::rust_interop::projection::Receiver::Move) => receiver_expression,
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
            let method_name = if specialization
                .is_some_and(|specialization| specialization.direct_projected_call)
                && let Some(projected) = projected_function.as_ref()
            {
                rust_name(&projected.name)
            } else {
                function_name(self.package, contract)
            };
            format!("({receiver}).{method_name}")
        } else {
            self.expression(callee)
        };
        let native_path_generics = projected_function
            .as_ref()
            .and_then(|projected| {
                projected
                    .native_path
                    .as_deref()
                    .or(projected.native_owner.as_deref())
            })
            .map(crate::rust_ir::rust_type_parameter_names)
            .unwrap_or_default();
        let name = if let Some(specialization) = specialization
            && !native_path_generics.is_empty()
        {
            let replacements = specialization
                .substitutions
                .iter()
                .map(|(name, projected)| (name.clone(), projected.rust_type()))
                .collect();
            crate::rust_ir::instantiate_rust_generics(&name, &replacements)
        } else {
            name
        };
        // Projected specialization records attach only to free/static paths or projected
        // member access. Each branch above ends in a callable Rust path/member segment, so an
        // explicit turbofish is syntactically valid here; arbitrary callee expressions never
        // receive a specialization record.
        let name = if let Some(specialization) =
            specialization.filter(|specialization| !specialization.generic_arguments.is_empty())
        {
            let contextual_value_type = self
                .unit
                .enclosing_function_spans
                .get(&node.span.start)
                .copied()
                .flatten()
                .and_then(|span| {
                    self.unit
                        .invocation_scoped_function_results
                        .get(&(span.file, span.start, span.end))
                })
                .cloned()
                .or_else(|| self.value_type(node));
            let mut generic_arguments = specialization
                .generic_arguments
                .iter()
                .enumerate()
                .filter_map(|(index, argument)| {
                    use crate::rust_interop::projection::ProjectedType;
                    let projected = projected_function.as_ref();
                    let template =
                        projected.and_then(|function| function.rust_generic_arguments.get(index));
                    // Concrete UFCS owner arguments already belong to the trait path.
                    if matches!(template, Some(ProjectedType::Foreign { .. }))
                        && projected
                            .and_then(|function| function.native_path.as_deref())
                            .is_some_and(|path| path.starts_with('<'))
                    {
                        return None;
                    }
                    let parameter_name = match template {
                        Some(ProjectedType::Generic(name)) => Some(name.as_str()),
                        None => projected
                            .and_then(|function| function.generic_parameters.get(index))
                            .map(|parameter| parameter.name.as_str()),
                        _ => None,
                    };
                    if parameter_name.is_some_and(|name| native_path_generics.contains(name)) {
                        return None;
                    }
                    let callback_generic = match template {
                        Some(ProjectedType::Callback { .. }) => true,
                        Some(ProjectedType::Generic(name)) => projected.is_some_and(|function| {
                            function.parameters.iter().any(|parameter| {
                                parameter.generic_parameter.as_deref() == Some(name.as_str())
                                    && matches!(parameter.ty, ProjectedType::Callback { .. })
                            })
                        }),
                        _ => false,
                    };
                    let native = argument.rust_type();
                    Some(
                        if callback_generic
                            || projected.is_some_and(|function| {
                                function
                                    .generic_parameters
                                    .iter()
                                    .any(|parameter| parameter.name == native)
                            })
                        {
                            "_".to_owned()
                        } else {
                            self.package
                                .projection
                                .canonical_native_type(&native)
                                .into_owned()
                        },
                    )
                })
                .collect::<Vec<_>>();
            if let (
                Some(projected),
                Some(ValueType::InvocationScopedNative {
                    rust_type: actual, ..
                }),
            ) = (projected_function.as_ref(), contextual_value_type)
                && let crate::rust_interop::projection::ProjectedType::InvocationScoped {
                    rust_type: template,
                    ..
                } = &projected.result
            {
                let template_arguments = crate::rust_ir::rust_type_arguments(template);
                let actual_arguments = crate::rust_ir::rust_type_arguments(&actual);
                for argument in &mut generic_arguments {
                    if actual_arguments.contains(argument) {
                        continue;
                    }
                    if let Some((_, replacement)) = template_arguments
                        .iter()
                        .zip(&actual_arguments)
                        .find(|(template, _)| *template == argument)
                    {
                        argument.clone_from(replacement);
                    } else if template_arguments.contains(argument) {
                        "_".clone_into(argument);
                    }
                }
            }
            if generic_arguments.is_empty() {
                name
            } else {
                format!("{name}::<{}>", generic_arguments.join(", "))
            }
        } else {
            name
        };
        if let Some(receiver) = native_member_receiver {
            values.insert(0, receiver);
        }
        let callable_mode =
            self.value_type(callee)
                .and_then(|value_type| match value_type {
                    ValueType::Function(_, _, effects)
                    | ValueType::AsyncFunction(_, _, _, effects) => Some(effects.modes.written),
                    _ => None,
                });
        let projected_native_construction = (callee.kind == SyntaxKind::ConstructionExpression)
            .then(|| {
                callee
                    .children
                    .first()
                    .and_then(|designator| self.class_designator(designator))
            })
            .flatten()
            .and_then(|object| {
                self.package
                    .projection
                    .projected_struct(&object.identity.namespace, &object.identity.name)
            });
        let call = if let Some((native_path, fields, borrowed_view)) = projected_native_construction
        {
            let selected_native = self.value_type(node).and_then(|value| match value {
                ValueType::Object(identity) => identity.native_projection,
                _ => None,
            });
            let native_path = selected_native.as_deref().unwrap_or(native_path);
            let value_for = |field_name: &str| {
                native_field_values
                    .as_ref()
                    .and_then(|slots| {
                        fields
                            .iter()
                            .position(|field| field.name == field_name)
                            .and_then(|index| slots.get(index))
                            .and_then(Clone::clone)
                    })
                    .or_else(|| {
                        contract.as_ref().and_then(|contract| {
                            contract.parameters.iter().zip(&values).find_map(
                                |(parameter, value)| {
                                    (parameter.name == field_name).then(|| value.clone())
                                },
                            )
                        })
                    })
                    .or_else(|| {
                        arguments
                            .children
                            .iter()
                            .zip(&values)
                            .find_map(|(argument, value)| {
                                argument
                                    .children
                                    .first()
                                    .filter(|name| {
                                        name.kind == SyntaxKind::Name
                                            && self.text(name) == field_name
                                    })
                                    .map(|_| value.clone())
                            })
                    })
            };
            let projected_values = fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let value = value_for(&field.name);
                    match (&field.ty, value) {
                        (crate::rust_interop::projection::ProjectedType::Optional(inner), None) => {
                            format!(
                                "None::<{}>",
                                super::super::dependencies::projected_field_abi_type(inner)
                            )
                        }
                        (_, Some(value))
                            if borrowed_view
                                || (projected_function.is_some()
                                    && !(matches!(
                                        &field.ty,
                                        crate::rust_interop::projection::ProjectedType::Optional(_)
                                    ) && native_optional_arguments.get(index)
                                        != Some(&true))) =>
                        {
                            value
                        }
                        (
                            crate::rust_interop::projection::ProjectedType::Optional(_),
                            Some(value),
                        ) if native_optional_arguments.get(index) == Some(&true) => self.fallible(
                            format!(
                                "(|| -> Result<_, crate::TerraneForeignError> {{ Ok({}) }})()",
                                projected_argument_expression(&value, &field.ty)
                            ),
                            node,
                        ),
                        (
                            crate::rust_interop::projection::ProjectedType::Sequence { .. },
                            Some(value),
                        ) => {
                            format!("{value}.into_vec()")
                        }
                        (
                            crate::rust_interop::projection::ProjectedType::Optional(_),
                            Some(value),
                        ) => {
                            format!("{value}.into()")
                        }
                        (_, Some(value)) => value,
                        (_, None) => unreachable!(
                            "semantics requires projected struct field `{}`",
                            field.name
                        ),
                    }
                })
                .collect::<Vec<_>>();
            if borrowed_view {
                format!("{name}({})", projected_values.join(", "))
            } else {
                let assignments = fields
                    .iter()
                    .zip(projected_values)
                    .map(|(field, value)| {
                        let field_name = if field
                            .rust_name
                            .chars()
                            .all(|character| character.is_ascii_digit())
                        {
                            field.rust_name.clone()
                        } else {
                            rust_name(&field.rust_name)
                        };
                        format!("{field_name}: {value}")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let native_path = native_path.replacen('<', "::<", 1);
                format!("{native_path} {{ {assignments} }}")
            }
        } else if contract.is_none()
            && matches!(
                callable_mode,
                Some(InvocationMode::Mutable | InvocationMode::Consuming)
            )
        {
            let arguments = match values.as_slice() {
                [] => "()".to_owned(),
                [value] => format!("({value},)"),
                _ => format!("({})", values.join(", ")),
            };
            format!("{name}.call({arguments})")
        } else if native_macro_path.is_some() {
            let result = &specialization
                .expect("macro result destination is proven")
                .projected_result;
            let mut result_type = result.rust_type();
            if let crate::rust_interop::projection::ProjectedType::InvocationScoped {
                lifetimes,
                ..
            } = result
            {
                for lifetime in lifetimes {
                    result_type = result_type.replace(lifetime, "'_");
                }
            }
            if matches!(
                result,
                crate::rust_interop::projection::ProjectedType::Generic(_)
            ) {
                format!("{name}!({})", values.join(", "))
            } else {
                format!(
                    "{{ let __terrane_macro_result: {} = core::convert::Into::into({name}!({})); __terrane_macro_result }}",
                    result_type,
                    values.join(", ")
                )
            }
        } else {
            format!("{name}({})", values.join(", "))
        };
        let projected_enum_receiver = callee
            .children
            .first()
            .map(|receiver| self.receiver_expression(receiver));
        let foreign_method =
            if specialization.is_some_and(|specialization| specialization.direct_projected_call) {
                projected_function.as_ref()
            } else {
                contract.as_ref().and_then(|contract| {
                    let [receiver, _member] = callee.children.as_slice() else {
                        return None;
                    };
                    let identity = self
                        .class_designator(receiver)
                        .map(|object| object.identity.clone())
                        .or_else(|| match self.value_type(receiver)? {
                            ValueType::Object(identity) => Some(identity),
                            ValueType::Reference(item) | ValueType::SharedReference(item) => {
                                let ValueType::Object(identity) = item.value_type() else {
                                    return None;
                                };
                                Some(identity.clone())
                            }
                            ValueType::InvocationScopedNative { family, .. } => Some(family),
                            _ => None,
                        })?;
                    if projected_interface_dispatch {
                        return None;
                    }
                    self.package
                        .projection
                        .method(
                            &identity.namespace,
                            &identity.name,
                            &contract.name,
                            contract.is_static,
                        )
                        .filter(|method| !contract.is_static || method.enum_operation.is_some())
                })
            };
        let enum_operation = foreign_method.and_then(|method| method.enum_operation.clone());
        let call = enum_operation
            .as_ref()
            .and_then(|operation| {
                let [receiver, _member] = callee.children.as_slice() else {
                    return None;
                };
                let identity = self
                    .class_designator(receiver)
                    .map(|object| object.identity.clone())
                    .or_else(|| {
                        let ValueType::Object(identity) = self.value_type(receiver)? else {
                            return None;
                        };
                        Some(identity)
                    })?;
                let owner = identity.native_projection.clone().or_else(|| {
                    self.package
                        .projection
                        .foreign_rust_path(&identity.namespace, &identity.name)
                        .map(str::to_owned)
                })?;
                let owner = owner.replacen('<', "::<", 1);
                let receiver = (!contract.as_ref().is_some_and(|contract| contract.is_static))
                    .then_some(projected_enum_receiver.as_deref())
                    .flatten();
                Some(projected_enum_call(operation, &owner, receiver, &values))
            })
            .unwrap_or(call);
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
                        crate::rust_interop::projection::ProjectedKind::Function(function) => {
                            function.chain_role
                        }
                        _ => None,
                    })
            });
        if matches!(
            chain_role,
            Some(
                crate::rust_interop::projection::ChainRole::Root
                    | crate::rust_interop::projection::ChainRole::Continue
            )
        ) {
            return call;
        }
        let direct_projected_call =
            specialization.is_some_and(|specialization| specialization.direct_projected_call);
        let foreign_error = direct_projected_call
            || foreign_method.is_some()
            || (callee.kind == SyntaxKind::Name
                && self
                    .package
                    .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                    .and_then(|symbol| symbol.identity.rsplit_once("::"))
                    .and_then(|(namespace, name)| self.package.projection.item(namespace, name))
                    .is_some_and(|item| {
                        matches!(
                            &item.kind,
                            crate::rust_interop::projection::ProjectedKind::Function(_)
                        )
                    }));
        let call = if let Some(method) = foreign_method {
            let (dependency, member) = if callee.kind == SyntaxKind::Name
                || specialization.is_some_and(|specialization| specialization.direct_projected_call)
            {
                let dependency = self
                    .package
                    .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                    .and_then(|symbol| symbol.namespace.strip_prefix("/deps/"))
                    .and_then(|namespace| namespace.split('/').next())
                    .unwrap_or("dependency")
                    .to_owned();
                (dependency, name.clone())
            } else {
                let [receiver, _member] = callee.children.as_slice() else {
                    unreachable!("projected methods have a receiver")
                };
                let identity = self
                    .class_designator(receiver)
                    .map(|object| object.identity.clone())
                    .or_else(|| match self.value_type(receiver)? {
                        ValueType::Object(identity) => Some(identity),
                        ValueType::InvocationScopedNative { family, .. } => Some(family),
                        ValueType::Reference(item) | ValueType::SharedReference(item) => {
                            let ValueType::Object(identity) = item.value_type() else {
                                return None;
                            };
                            Some(identity.clone())
                        }
                        _ => None,
                    })
                    .expect("projected method receiver has an object type");
                let type_path = self
                    .package
                    .projection
                    .foreign_rust_path(&identity.namespace, &identity.name)
                    .expect("foreign method owner has a projected Rust path");
                (
                    type_path
                        .split("::")
                        .next()
                        .unwrap_or("dependency")
                        .to_owned(),
                    format!("{type_path}::{}", method.name),
                )
            };
            let call = if method.is_unsafe {
                format!("unsafe {{ {call} }}")
            } else {
                call
            };
            let invocation = if method.into_future {
                "std::future::IntoFuture::into_future(__terrane_call).await".to_owned()
            } else if method.is_async {
                "__terrane_call.await".to_owned()
            } else {
                call.clone()
            };
            let discarded_unit = !method.is_async
                && !method.into_future
                && method.error.is_none()
                && self.discarded_call == Some(node.span);
            let unwind_call = if discarded_unit {
                format!("{{ let _ = {call}; }}")
            } else {
                call.clone()
            };
            let caught = if method.into_future {
                "crate::__terrane_dependency_await_unwind(std::future::IntoFuture::into_future(__terrane_call)).await".to_owned()
            } else if method.is_async {
                "crate::__terrane_dependency_await_unwind(__terrane_call).await".to_owned()
            } else {
                format!("std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {unwind_call}))")
            };
            let projected_result = specialization.map_or(&method.result, |specialization| {
                &specialization.projected_result
            });
            let converted = if discarded_unit {
                "()".to_owned()
            } else {
                projected_result_expression("value", projected_result)
            };
            let nested_converted = match projected_result {
                crate::rust_interop::projection::ProjectedType::Optional(inner)
                    if method.error_optional_depth == 1 =>
                {
                    projected_result_expression("value", inner)
                }
                _ => converted.clone(),
            };
            let projected_error_identity = method.error.as_deref().and_then(|rust_path| {
                self.package
                    .projection
                    .projected_identity_for_rust_path(rust_path)
                    .map(|(namespace, name)| (format!("{namespace}::{name}"), name.to_owned()))
            });
            let (error_kind, error_message) = projected_error_identity.map_or_else(
                || {
                    (
                        "TERRANE_DEPENDENCY_ERROR".to_owned(),
                        format!(
                            "format!(\"Rust dependency `{dependency}` member `{member}` failed: {{error}}\")"
                        ),
                    )
                },
                |(identity, name)| {
                    let descriptor = self.registry.register_descriptor(&identity, &name);
                    (
                        format!("DescriptorId({descriptor})"),
                        "error.to_string()".to_owned(),
                    )
                },
            );
            let mapped = if self.package.profile.panic == crate::package::PanicProfile::Abort {
                if method.error_optional_depth == 1 {
                    format!(
                        "match {invocation} {{ None => Ok(None), Some(Ok(value)) => Ok(Some({nested_converted})), Some(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, {error_message}, crate::TERRANE_NO_SITE))) }}"
                    )
                } else if method.error.is_some() {
                    format!(
                        "match {invocation} {{ Ok(value) => Ok({converted}), Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, {error_message}, crate::TERRANE_NO_SITE))) }}"
                    )
                } else if self.discarded_call == Some(node.span) {
                    format!("{{ let _ = {invocation}; Ok(()) }}")
                } else {
                    format!(
                        "Ok({})",
                        projected_result_expression(&invocation, projected_result)
                    )
                }
            } else if method.error_optional_depth == 1 {
                format!(
                    "match {caught} {{ Ok(None) => Ok(None), Ok(Some(Ok(value))) => Ok(Some({nested_converted})), Ok(Some(Err(error))) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, {error_message}, crate::TERRANE_NO_SITE))), Err(payload) => Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
                )
            } else if method.error.is_some() {
                format!(
                    "match {caught} {{ Ok(Ok(value)) => Ok({converted}), Ok(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, {error_message}, crate::TERRANE_NO_SITE))), Err(payload) => Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
                )
            } else {
                let value_pattern = if discarded_unit { "()" } else { "value" };
                format!(
                    "match {caught} {{ Ok({value_pattern}) => Ok({converted}), Err(payload) => Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
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
        let call = if foreign_method.is_none()
            && (contract.as_ref().is_some_and(|contract| contract.is_unsafe)
                || specialization
                    .is_some_and(|specialization| specialization.direct_projected_call)
                    && projected_function
                        .as_ref()
                        .is_some_and(|function| function.is_unsafe))
        {
            format!("unsafe {{ {call} }}")
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
            !projected_interface_dispatch
                && self.package.units.iter().any(|unit| {
                    unit.source.id() == contract.span.file && unit.namespace.starts_with("/deps/")
                })
        });
        let needs_error_mapping = contract.as_ref().is_some_and(|contract| contract.throws)
            || projected_error
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
#[expect(
    clippy::too_many_lines,
    reason = "enum construction and extraction keep their mirrored field mapping visible together"
)]
fn projected_enum_call(
    operation: &crate::rust_interop::projection::ProjectedEnumOperation,
    owner: &str,
    receiver: Option<&str>,
    values: &[String],
) -> String {
    match operation {
        crate::rust_interop::projection::ProjectedEnumOperation::Construct {
            variant,
            unit,
            conversion,
            payload,
            ..
        } => {
            if *unit {
                return format!("{owner}::{variant}");
            }
            if let Some(payload) = payload {
                let bindings = (0..payload.fields.len())
                    .map(|index| format!("field_{index}"))
                    .collect::<Vec<_>>();
                let body = match payload.style {
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Tuple => {
                        format!("{owner}::{variant}({})", bindings.join(", "))
                    }
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Struct => {
                        let fields = payload
                            .fields
                            .iter()
                            .zip(&bindings)
                            .map(|(field, binding)| format!("r#{field}: {binding}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{owner}::{variant} {{ {fields} }}")
                    }
                };
                return format!(
                    "{{ let ({},) = ({}).terrane_into_fields(); {body} }}",
                    bindings.join(", "),
                    values[0]
                );
            }
            let arguments = values
                .iter()
                .map(|value| match conversion {
                    crate::rust_interop::projection::ProjectedEnumPayloadConversion::Into => {
                        format!("({value}).into()")
                    }
                    _ => value.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{owner}::{variant}({arguments})")
        }
        crate::rust_interop::projection::ProjectedEnumOperation::VariantName {
            variants,
            exhaustive,
        } => {
            let receiver = receiver.expect("enum inspection has a receiver");
            let mut arms = variants
                .iter()
                .map(|variant| format!("{owner}::{variant} {{ .. }} => {variant:?}.to_owned()"))
                .collect::<Vec<_>>();
            if !exhaustive {
                arms.push("_ => \"unknown\".to_owned()".to_owned());
            }
            format!("match &{receiver} {{ {} }}", arms.join(", "))
        }
        crate::rust_interop::projection::ProjectedEnumOperation::Extract {
            variant,
            conversion,
            payload_rust_type,
            payload,
        } => {
            let receiver = receiver.expect("enum extraction has a receiver");
            if let Some(payload) = payload {
                let bindings = (0..payload.fields.len())
                    .map(|index| format!("field_{index}"))
                    .collect::<Vec<_>>();
                let pattern = match payload.style {
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Tuple => {
                        format!("{owner}::{variant}({})", bindings.join(", "))
                    }
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Struct => {
                        let fields = payload
                            .fields
                            .iter()
                            .zip(&bindings)
                            .map(|(field, binding)| format!("r#{field}: {binding}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{owner}::{variant} {{ {fields} }}")
                    }
                };
                let constructor =
                    super::super::dependencies::enum_payload_constructor_name(payload_rust_type);
                return format!(
                    "match {receiver} {{ {pattern} => Some({constructor}({})), _ => None }}",
                    bindings.join(", ")
                );
            }
            let value = match conversion {
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::Identity
                | crate::rust_interop::projection::ProjectedEnumPayloadConversion::Into => {
                    "value".to_owned()
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::AsRefString => {
                    format!("<{payload_rust_type} as AsRef<str>>::as_ref(&value).to_owned()")
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::AsRefBytes => {
                    format!("<{payload_rust_type} as AsRef<[u8]>>::as_ref(&value).to_vec()")
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::DerefString => {
                    format!("<{payload_rust_type} as std::ops::Deref>::deref(&value).to_owned()")
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::DerefBytes => {
                    format!("<{payload_rust_type} as std::ops::Deref>::deref(&value).to_vec()")
                }
            };
            format!("match {receiver} {{ {owner}::{variant}(value) => Some({value}), _ => None }}")
        }
    }
}
