use super::prelude::*;

#[expect(
    clippy::too_many_lines,
    reason = "value inference centralizes the precedence among syntax forms and typed member families"
)]
pub(super) fn infer_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    if node.kind == SyntaxKind::Literal {
        return Ok(infer_literal_type(unit, node).map(ValueType::Scalar));
    }
    if node.kind == SyntaxKind::AnonymousFunction {
        let contract = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed closure must have a semantic contract");
        let parameters = contract
            .parameters
            .iter()
            .map(|parameter| {
                parameter
                    .value_type
                    .clone()
                    .map(ElementType::new)
                    .ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0052",
                            "stored function parameters require explicit types",
                            parameter.span,
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let result = ElementType::new(
            contract
                .return_type
                .clone()
                .unwrap_or(ValueType::Scalar(ScalarType::None)),
        );
        return Ok(Some(ValueType::Function(parameters, result)));
    }
    if node.kind == SyntaxKind::GroupExpression {
        return match node.children.first() {
            Some(child) => infer_value_type(unit, child, bindings),
            None => Ok(None),
        };
    }
    if node.kind == SyntaxKind::UnaryExpression {
        return infer_unary_type(unit, node, bindings).map(Some);
    }
    if node.kind == SyntaxKind::BinaryExpression {
        return infer_binary_type(unit, node, bindings).map(Some);
    }
    if node.kind == SyntaxKind::TypeMembershipExpression {
        return Ok(Some(ValueType::Scalar(ScalarType::Bool)));
    }
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        if name == "none" {
            return Ok(Some(ValueType::Scalar(ScalarType::None)));
        }
        if let Some(scalar) = ScalarType::from_source_name(name).or_else(|| {
            visible_descriptor_aliases(&unit.descriptor_aliases, unit.source.id(), node.span.start)
                .get(name)
                .copied()
        }) {
            return Ok(Some(ValueType::Descriptor(scalar.source_name().to_owned())));
        }
        if unit.objects.iter().any(|object| object.name == name) {
            return Ok(Some(ValueType::Descriptor(name.to_owned())));
        }
        if let Some(binding) = bindings.iter().rev().find(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), node.span.start)
        }) {
            return Ok(Some(
                narrowed_value_type(unit, node, bindings).unwrap_or(binding.value_type.clone()),
            ));
        }
        if let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.owner.is_none() && contract.name == name)
        {
            let parameters = contract
                .parameters
                .iter()
                .map(|parameter| parameter.value_type.clone().map(ElementType::new))
                .collect::<Option<Vec<_>>>();
            if let Some(parameters) = parameters {
                let result = ElementType::new(
                    contract
                        .return_type
                        .clone()
                        .unwrap_or(ValueType::Scalar(ScalarType::None)),
                );
                return Ok(Some(if contract.is_async {
                    ValueType::AsyncFunction(parameters, result)
                } else {
                    ValueType::Function(parameters, result)
                }));
            }
        }
        let resolved_symbol = lexical_scope_chain(unit, node.span.start).find_map(|scope| {
            scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol
                    .declaration_span
                    .is_none_or(|span| span.end <= node.span.start)
            })
        });
        let resolved_encoding = resolved_symbol
            .map(|symbol| symbol.identity.as_str())
            .or_else(|| {
                unit.prelude.then_some(name).and_then(|name| {
                    matches!(
                        name,
                        "utf8" | "utf16-le" | "utf16-be" | "utf32-le" | "utf32-be"
                    )
                    .then_some(name)
                })
            })
            .is_some_and(|identity| {
                identity.starts_with("/core/encodings::")
                    || matches!(
                        identity,
                        "utf8" | "utf16-le" | "utf16-be" | "utf32-le" | "utf32-be"
                    )
            });
        return Ok(resolved_encoding.then_some(ValueType::Encoding));
    }
    if member_family_receiver(unit, node) {
        return Err(failure(
            &unit.source,
            "T0018",
            "member-family selections must be invoked in the same expression",
            node.span,
        ));
    }
    if node.kind == SyntaxKind::IndexExpression {
        let Some(receiver) = node.children.first() else {
            return Ok(None);
        };
        return match infer_receiver_value_type(unit, receiver, bindings)? {
            Some(ValueType::List(item) | ValueType::Tuple(item, _)) => Ok(Some(item.value_type())),
            Some(ValueType::StringList) => Ok(Some(ValueType::Scalar(ScalarType::String))),
            Some(ValueType::Map(_, value) | ValueType::UnorderedMap(_, value)) => {
                Ok(Some(value.value_type()))
            }
            Some(ValueType::Scalar(ScalarType::String)) => Err(failure(
                &unit.source,
                "T0050",
                "string indexing is not implemented yet",
                receiver.span,
            )),
            Some(other) => Err(failure(
                &unit.source,
                "T0050",
                format!("indexing is not supported for `{other}`"),
                receiver.span,
            )),
            None => Err(failure(
                &unit.source,
                "T0050",
                "indexing requires a receiver with a statically known collection type",
                receiver.span,
            )),
        };
    }
    if node.kind == SyntaxKind::MemberExpression {
        return infer_member_value_type(unit, node, bindings);
    }
    if node.kind == SyntaxKind::StaticMemberExpression {
        let [receiver, member] = node.children.as_slice() else {
            return Ok(None);
        };
        let identity = class_designator_identity(unit, receiver).ok_or_else(|| {
            failure(
                &unit.source,
                "T0104",
                "the left side of `::` must resolve to a class",
                receiver.span,
            )
        })?;
        return object_member_type(unit, &identity, node_text(&unit.source, member), true)
            .map(Some)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0105",
                    format!(
                        "class `{}` has no static member `{}`",
                        identity.name,
                        node_text(&unit.source, member)
                    ),
                    member.span,
                )
            });
    }
    if node.kind == SyntaxKind::CallExpression {
        if let [callee, arguments] = node.children.as_slice() {
            if callee.kind == SyntaxKind::ConstructionExpression {
                let class = callee.children.first().ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0103",
                        "construction requires a class",
                        callee.span,
                    )
                })?;
                let identity = class_designator_identity(unit, class).ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0103",
                        format!(
                            "`{}` does not resolve to a constructible class",
                            node_text(&unit.source, class)
                        ),
                        class.span,
                    )
                })?;
                return Ok(Some(ValueType::Object(identity)));
            }
            if callee.kind == SyntaxKind::StaticMemberExpression {
                let [receiver, member] = callee.children.as_slice() else {
                    return Ok(None);
                };
                let identity = class_designator_identity(unit, receiver).ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0104",
                        "the left side of `::` must resolve to a class",
                        receiver.span,
                    )
                })?;
                let member_type =
                    object_member_type(unit, &identity, node_text(&unit.source, member), true)
                        .ok_or_else(|| {
                            failure(
                                &unit.source,
                                "T0105",
                                format!(
                                    "class `{}` has no static member `{}`",
                                    identity.name,
                                    node_text(&unit.source, member)
                                ),
                                member.span,
                            )
                        })?;
                return match member_type {
                    ValueType::Function(_, result) => {
                        let result = result.value_type();
                        let result = object_method_contract(
                            unit,
                            &identity,
                            node_text(&unit.source, member),
                            true,
                        )
                        .filter(|method| {
                            matches!(
                                &result,
                                ValueType::Object(returned)
                                    if method.owner.as_deref() == Some(returned.name.as_str())
                            )
                        })
                        .map_or(result, |_| ValueType::Object(identity));
                        Ok(Some(result))
                    }
                    ValueType::AsyncFunction(_, result) => Ok(Some(ValueType::Task(result))),
                    _ => Err(failure(
                        &unit.source,
                        "T0039",
                        format!(
                            "`{}::{}` is a property and cannot be invoked",
                            identity.name,
                            node_text(&unit.source, member)
                        ),
                        callee.span,
                    )),
                };
            }
            if callee.kind == SyntaxKind::Name
                && resolved_compiler_identity(unit, callee)
                    .is_some_and(|identity| identity == "/core/async::task-scope")
            {
                return Ok(Some(ValueType::TaskScope));
            }
            if callee.kind == SyntaxKind::Name
                && let Some(identity) = resolved_compiler_identity(unit, callee)
            {
                let platform_result = match identity {
                    "intrinsic:streams::acquire-stdin"
                    | "intrinsic:streams::acquire-stdout"
                    | "intrinsic:streams::acquire-stderr" => Some(ValueType::PlatformStreamHandle),
                    "intrinsic:system::acquire-filesystem-authority" => {
                        Some(ValueType::FilesystemAuthority)
                    }
                    "intrinsic:streams::open-file"
                    | "intrinsic:streams::open-directory-beneath"
                    | "intrinsic:streams::open-file-beneath" => Some(ValueType::PlatformOpenResult),
                    "intrinsic:streams::read" => Some(ValueType::PlatformReadResult),
                    "intrinsic:streams::write" => Some(ValueType::PlatformWriteResult),
                    "intrinsic:streams::flush"
                    | "intrinsic:streams::sync-data"
                    | "intrinsic:streams::sync-all"
                    | "intrinsic:streams::close"
                    | "intrinsic:streams::release" => Some(ValueType::PlatformUnitResult),
                    "intrinsic:data::empty-document"
                    | "intrinsic:data::make-document-none"
                    | "intrinsic:data::make-document-bool"
                    | "intrinsic:data::make-document-string"
                    | "intrinsic:data::make-document-integer"
                    | "intrinsic:data::make-document-decimal"
                    | "intrinsic:data::make-document-list"
                    | "intrinsic:data::make-document-map"
                    | "intrinsic:data::document-list-append"
                    | "intrinsic:data::document-map-insert"
                    | "intrinsic:data::json-parse"
                    | "intrinsic:data::json-canonical"
                    | "intrinsic:data::yaml-parse"
                    | "intrinsic:data::document-item"
                    | "intrinsic:data::document-field"
                    | "intrinsic:data::validate-mapping" => Some(ValueType::PlatformDataResult),
                    "intrinsic:data::url-parse" => Some(ValueType::PlatformUrlResult),
                    "intrinsic:capabilities::secure-random"
                    | "intrinsic:capabilities::cancellation-token"
                    | "intrinsic:capabilities::pseudo-random"
                    | "intrinsic:capabilities::secret-buffer"
                    | "intrinsic:capabilities::result-capability"
                    | "intrinsic:concurrency::platform-capability"
                    | "intrinsic:concurrency::no-capability" => Some(ValueType::PlatformCapability),
                    "intrinsic:capabilities::result-resource"
                    | "intrinsic:capabilities::no-resource" => {
                        Some(ValueType::PlatformResourceHandle)
                    }
                    "intrinsic:capabilities::failed-result"
                    | "intrinsic:capabilities::random-bytes"
                    | "intrinsic:capabilities::random-bounded"
                    | "intrinsic:capabilities::random-split"
                    | "intrinsic:capabilities::digest"
                    | "intrinsic:capabilities::hmac"
                    | "intrinsic:capabilities::destroy-secret"
                    | "intrinsic:capabilities::hex-decode"
                    | "intrinsic:capabilities::base64-decode"
                    | "intrinsic:capabilities::uuid-parse"
                    | "intrinsic:capabilities::uuid-v4"
                    | "intrinsic:capabilities::uuid-v7"
                    | "intrinsic:capabilities::compress"
                    | "intrinsic:capabilities::decompress"
                    | "intrinsic:capabilities::parse-ip"
                    | "intrinsic:capabilities::parse-host-name"
                    | "intrinsic:capabilities::parse-socket"
                    | "intrinsic:capabilities::parse-socket-text"
                    | "intrinsic:capabilities::tcp-bind"
                    | "intrinsic:capabilities::tcp-connect"
                    | "intrinsic:capabilities::tcp-connect-host"
                    | "intrinsic:capabilities::tcp-accept"
                    | "intrinsic:capabilities::tcp-read"
                    | "intrinsic:capabilities::tcp-write"
                    | "intrinsic:capabilities::tcp-shutdown"
                    | "intrinsic:capabilities::tcp-configure"
                    | "intrinsic:capabilities::udp-bind"
                    | "intrinsic:capabilities::udp-configure"
                    | "intrinsic:capabilities::udp-send-to"
                    | "intrinsic:capabilities::udp-receive-from"
                    | "intrinsic:capabilities::dns-lookup"
                    | "intrinsic:capabilities::tls-client"
                    | "intrinsic:capabilities::tls-read"
                    | "intrinsic:capabilities::tls-write"
                    | "intrinsic:capabilities::tls-shutdown"
                    | "intrinsic:capabilities::cancel"
                    | "intrinsic:capabilities::close"
                    | "intrinsic:concurrency::platform-result"
                    | "intrinsic:concurrency::int-channel"
                    | "intrinsic:concurrency::int-mutex"
                    | "intrinsic:concurrency::int-read-write-lock"
                    | "intrinsic:concurrency::atomic-int64"
                    | "intrinsic:concurrency::thread-local-int"
                    | "intrinsic:concurrency::int-channel-send"
                    | "intrinsic:concurrency::int-channel-receive"
                    | "intrinsic:concurrency::int-channel-try-receive"
                    | "intrinsic:concurrency::int-mutex-load"
                    | "intrinsic:concurrency::int-mutex-store"
                    | "intrinsic:concurrency::int-mutex-add"
                    | "intrinsic:concurrency::int-read-write-lock-read"
                    | "intrinsic:concurrency::int-read-write-lock-write"
                    | "intrinsic:concurrency::atomic-int64-load"
                    | "intrinsic:concurrency::atomic-int64-store"
                    | "intrinsic:concurrency::atomic-int64-add"
                    | "intrinsic:concurrency::thread-local-int-get"
                    | "intrinsic:concurrency::thread-local-int-set"
                    | "intrinsic:adapters::platform-result"
                    | "intrinsic:adapters::system-host-name" => Some(ValueType::PlatformResult),
                    "intrinsic:system::filesystem-exists"
                    | "intrinsic:system::filesystem-metadata"
                    | "intrinsic:system::filesystem-realpath"
                    | "intrinsic:system::filesystem-read-link"
                    | "intrinsic:system::filesystem-read-bounded"
                    | "intrinsic:system::filesystem-write-atomic"
                    | "intrinsic:system::filesystem-rename"
                    | "intrinsic:system::filesystem-remove" => {
                        Some(ValueType::PlatformFilesystemResult)
                    }
                    "intrinsic:system::result-failed"
                    | "intrinsic:system::result-bool"
                    | "intrinsic:system::platform-value-is-text"
                    | "intrinsic:data::data-failed"
                    | "intrinsic:data::url-failed"
                    | "intrinsic:capabilities::constant-time-equal"
                    | "intrinsic:capabilities::result-failed"
                    | "intrinsic:capabilities::result-resource-limit"
                    | "intrinsic:capabilities::result-truncated"
                    | "intrinsic:capabilities::result-deadline-exceeded"
                    | "intrinsic:capabilities::result-bool"
                    | "intrinsic:concurrency::result-failed"
                    | "intrinsic:concurrency::result-bool"
                    | "intrinsic:adapters::result-failed"
                    | "intrinsic:adapters::result-bool" => {
                        Some(ValueType::Scalar(ScalarType::Bool))
                    }
                    "intrinsic:system::result-message"
                    | "intrinsic:system::result-text"
                    | "intrinsic:system::result-detail"
                    | "intrinsic:system::platform-value-text"
                    | "intrinsic:data::data-message"
                    | "intrinsic:data::data-path"
                    | "intrinsic:data::data-expected"
                    | "intrinsic:data::data-encoded"
                    | "intrinsic:data::document-kind"
                    | "intrinsic:data::document-text"
                    | "intrinsic:data::document-coefficient"
                    | "intrinsic:data::document-key"
                    | "intrinsic:data::url-message"
                    | "intrinsic:data::url-serialized"
                    | "intrinsic:data::url-display"
                    | "intrinsic:data::url-scheme"
                    | "intrinsic:data::url-username"
                    | "intrinsic:data::url-password"
                    | "intrinsic:data::url-host"
                    | "intrinsic:data::url-port"
                    | "intrinsic:data::url-path"
                    | "intrinsic:data::url-query-key"
                    | "intrinsic:data::url-query-value"
                    | "intrinsic:data::url-fragment"
                    | "intrinsic:data::url-origin"
                    | "intrinsic:capabilities::hex-encode"
                    | "intrinsic:capabilities::base64-encode"
                    | "intrinsic:capabilities::result-message"
                    | "intrinsic:capabilities::result-text"
                    | "intrinsic:capabilities::result-detail"
                    | "intrinsic:concurrency::result-message"
                    | "intrinsic:adapters::result-message"
                    | "intrinsic:adapters::result-text" => {
                        Some(ValueType::Scalar(ScalarType::String))
                    }
                    "intrinsic:system::result-bytes"
                    | "intrinsic:system::platform-value-bytes"
                    | "intrinsic:capabilities::result-bytes" => {
                        Some(ValueType::Scalar(ScalarType::Bytes))
                    }
                    "intrinsic:system::result-int"
                    | "intrinsic:data::document-exponent"
                    | "intrinsic:data::document-length"
                    | "intrinsic:data::url-query-length"
                    | "intrinsic:capabilities::result-int"
                    | "intrinsic:concurrency::result-int" => {
                        Some(ValueType::Scalar(ScalarType::Int))
                    }
                    "intrinsic:system::process-arguments"
                    | "intrinsic:system::environment-entries"
                    | "intrinsic:capabilities::result-entries" => Some(ValueType::StringList),
                    "intrinsic:system::process-exit" => Some(ValueType::Scalar(ScalarType::None)),
                    _ => None,
                };
                if platform_result.is_some() {
                    return Ok(platform_result);
                }
            }
            if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
                && matches!(
                    node_text(&unit.source, member),
                    "spawn" | "join" | "cancel" | "child-scope"
                )
                && infer_value_type(unit, receiver, bindings)? == Some(ValueType::TaskScope)
            {
                return match node_text(&unit.source, member) {
                    "spawn" => {
                        let Some(callable) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.spawn` requires one async callable",
                                node.span,
                            ));
                        };
                        let callable = callable.children.last().unwrap_or(callable);
                        match infer_value_type(unit, callable, bindings)? {
                            Some(ValueType::AsyncFunction(_, result)) => {
                                Ok(Some(ValueType::ScopedTask(result)))
                            }
                            _ => Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.spawn` requires an async callable value",
                                callable.span,
                            )),
                        }
                    }
                    "join" => {
                        let Some(task) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.join` requires one scoped task",
                                node.span,
                            ));
                        };
                        let task = task.children.last().unwrap_or(task);
                        match infer_value_type(unit, task, bindings)? {
                            Some(ValueType::ScopedTask(result)) => {
                                Ok(Some(ValueType::TaskOutcome(result)))
                            }
                            _ => Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.join` requires a scoped task",
                                task.span,
                            )),
                        }
                    }
                    "cancel" => Ok(Some(ValueType::Scalar(ScalarType::None))),
                    "child-scope" => {
                        let Some(argument) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.child-scope` requires one deadline",
                                node.span,
                            ));
                        };
                        let child = argument.children.last().unwrap_or(argument);
                        let parent_deadline =
                            task_scope_deadline_ms(unit, receiver, bindings, &mut BTreeSet::new());
                        let child_deadline =
                            constant_deadline_ms(unit, child, bindings, &mut BTreeSet::new());
                        if matches!(
                            (parent_deadline, child_deadline),
                            (Some(parent), Some(child)) if child > parent
                        ) {
                            return Err(failure(
                                &unit.source,
                                "T0075",
                                "a child scope cannot extend its parent deadline",
                                child.span,
                            ));
                        }
                        Ok(Some(ValueType::TaskScope))
                    }
                    _ => Ok(None),
                };
            }
        }
        if let Some(value_type) = infer_collection_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_iterator_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_string_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_float_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_arithmetic_family_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_parse_or_radix_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_numeric_coercion_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && matches!(node_text(&unit.source, member), "concat" | "join")
        {
            let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
            if receiver_type == Some(ValueType::Scalar(ScalarType::String)) {
                return Ok(Some(ValueType::Scalar(ScalarType::String)));
            }
            if receiver_type == Some(ValueType::Scalar(ScalarType::Bytes))
                && node_text(&unit.source, member) == "concat"
            {
                return Ok(Some(ValueType::Scalar(ScalarType::Bytes)));
            }
            return Err(failure(
                &unit.source,
                "T0013",
                format!(
                    "`.{}` requires a `string` receiver{}; found `{}`",
                    node_text(&unit.source, member),
                    if node_text(&unit.source, member) == "concat" {
                        " or `bytes` receiver"
                    } else {
                        ""
                    },
                    receiver_type
                        .map_or_else(|| "unknown".to_owned(), |value_type| value_type.to_string())
                ),
                receiver.span,
            ));
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let Some(member_type) = infer_member_value_type(unit, callee, bindings)?
        {
            return match member_type {
                ValueType::Function(_, result) => Ok(Some(result.value_type())),
                ValueType::AsyncFunction(_, result) => Ok(Some(ValueType::Task(result))),
                _ => Err(failure(
                    &unit.source,
                    "T0039",
                    format!(
                        "`.{}` is a property and cannot be invoked",
                        node_text(
                            &unit.source,
                            callee.children.get(1).expect("member expression")
                        )
                    ),
                    callee.span,
                )),
            };
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
        {
            let name = node_text(&unit.source, callee);
            if unit
                .objects
                .iter()
                .any(|object| object.name == name && object.kind == ObjectKind::Class)
            {
                return Err(failure(
                    &unit.source,
                    "T0102",
                    format!("class `{name}` is not callable; construct it with `instance {name};`"),
                    callee.span,
                ));
            }
            if let Some(contract) = resolved_function_contract(unit, name, callee.span.start) {
                let result = ElementType::new(
                    contract
                        .return_type
                        .clone()
                        .unwrap_or(ValueType::Scalar(ScalarType::None)),
                );
                return Ok(Some(if contract.is_async {
                    ValueType::Task(result)
                } else {
                    result.value_type()
                }));
            }
            if let Some(binding) = bindings.iter().rev().find(|binding| {
                binding.name == name && binding.is_visible_at(unit.source.id(), callee.span.start)
            }) {
                return match &binding.value_type {
                    ValueType::Function(_, result) => Ok(Some(result.value_type())),
                    ValueType::AsyncFunction(_, result) => {
                        Ok(Some(ValueType::Task(result.clone())))
                    }
                    _ => Err(failure(
                        &unit.source,
                        "T0039",
                        format!("`{name}` is a value and cannot be called"),
                        callee.span,
                    )),
                };
            }
            return Ok(None);
        }
        return Ok(None);
    }
    Ok(None)
}
