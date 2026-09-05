use super::super::prelude::*;

impl Emitter<'_> {
    #[expect(
        clippy::too_many_lines,
        reason = "all call forms share one ordering and error-propagation path"
    )]
    pub(super) fn call(&mut self, node: &SyntaxNode) -> String {
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
                    .is_some_and(|contract| contract.arity.is_some()) =>
                {
                    let arguments = values
                        .iter()
                        .map(|value| self.expression_as(value, ValueType::Scalar(receiver_type)))
                        .collect::<Vec<_>>();
                    self.float_call(receiver_type, operation, &receiver_value, &arguments, node)
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
            ("tls-client", "platform_tls_client"),
            ("tls-read", "platform_tls_read"),
            ("tls-write", "platform_tls_write"),
            ("tls-shutdown", "platform_tls_shutdown"),
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
            self.is_builtin(callee, &format!("intrinsic:concurrency::{terrane}"))
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
            format!(
                "{}::terrane_static_{}",
                rust_object_type_name(self.package, &object.identity),
                rust_name(self.text(member))
            )
        } else if let Some(contract) = &contract
            && contract.owner.is_none()
        {
            function_name(self.package, contract)
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
        let function_value_call = contract.is_none()
            && matches!(self.value_type(callee), Some(ValueType::Function(_, _)));
        if contract.is_some_and(|contract| contract.throws) || foreign_error || function_value_call
        {
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
}
