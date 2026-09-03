use super::prelude::*;

impl Emitter<'_> {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "string-view dispatch matches the optional recursive receiver type as one decision"
    )]
    pub(super) fn direct_string_view_length(
        &mut self,
        receiver: &SyntaxNode,
        receiver_type: Option<ValueType>,
        member: &SyntaxNode,
    ) -> Option<String> {
        if self.text(member) != "length" {
            return None;
        }
        let Some(ValueType::StringView(unit)) = receiver_type else {
            return None;
        };
        receiver.children.first().map(|source| {
            let source = self.expression(source);
            match unit {
                crate::semantics::TextUnit::Bytes => format!("({source}).len() as i128"),
                crate::semantics::TextUnit::Scalars => {
                    format!("({source}).chars().count() as i128")
                }
                crate::semantics::TextUnit::Graphemes => {
                    format!("terrane_string_support::length(&({source})) as i128")
                }
            }
        })
    }

    pub(super) fn index(&mut self, node: &SyntaxNode) -> String {
        let [receiver, index] = node.children.as_slice() else {
            return String::new();
        };
        let receiver_type = self.receiver_value_type(receiver);
        let receiver = self.receiver_expression(receiver);
        match receiver_type {
            Some(ValueType::List(_) | ValueType::Tuple(_, _) | ValueType::StringList) => {
                let index = if self.value_type(index) == Some(ValueType::Scalar(ScalarType::Int)) {
                    let index_value = self.expression_as(index, ValueType::Scalar(ScalarType::Int));
                    self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index_value}))"),
                        node,
                    )
                } else {
                    format!("({}) as usize", self.expression(index))
                };
                if receiver_type == Some(ValueType::StringList) {
                    self.fallible(
                        format!(
                            "({receiver}).get({index}).cloned().ok_or(terrane_collection_support::IndexError {{ index: {index} }})"
                        ),
                        node,
                    )
                } else {
                    self.fallible(format!("({receiver}).get_or_error({index})"), node)
                }
            }
            Some(ValueType::Map(key, _) | ValueType::UnorderedMap(key, _)) => {
                let index_value = self.expression_as(index, key.value_type());
                self.fallible(format!("({receiver}).get_or_error(&({index_value}))"), node)
            }
            _ => String::new(),
        }
    }

    pub(super) fn receiver_value_type(&self, receiver: &SyntaxNode) -> Option<ValueType> {
        self.value_type(receiver)
            .map(|value_type| match value_type {
                ValueType::Reference(item) | ValueType::SharedReference(item) => item.value_type(),
                value_type => value_type,
            })
    }

    pub(super) fn receiver_expression(&mut self, receiver: &SyntaxNode) -> String {
        match self.value_type(receiver) {
            Some(ValueType::SharedReference(_)) => format!(
                "({{ let __terrane_value = {}.lock().expect(\"shared reference lock poisoned\").clone(); __terrane_value }})",
                self.expression(receiver)
            ),
            Some(ValueType::Reference(_)) => format!(
                "({{ let __terrane_owner = {}.upgrade().expect(\"reference expired\"); let __terrane_value = __terrane_owner.lock().expect(\"reference lock poisoned\").clone(); __terrane_value }})",
                self.expression(receiver)
            ),
            _ => self.expression(receiver),
        }
    }

    pub(super) fn receiver_guard_expression(&mut self, receiver: &SyntaxNode) -> String {
        match self.value_type(receiver) {
            Some(ValueType::SharedReference(_)) => format!(
                "{}.lock().expect(\"shared reference lock poisoned\")",
                self.expression(receiver)
            ),
            Some(ValueType::Reference(_)) => {
                "__terrane_owner.lock().expect(\"reference lock poisoned\")".to_owned()
            }
            _ if self.reference_backed_name(receiver).is_some() => {
                format!(
                    "{}.lock().expect(\"reference lock poisoned\")",
                    self.name(receiver)
                )
            }
            _ => self.expression(receiver),
        }
    }

    pub(super) fn wrap_receiver_guard(
        &mut self,
        receiver: &SyntaxNode,
        expression: String,
    ) -> String {
        if matches!(self.value_type(receiver), Some(ValueType::Reference(_))) {
            format!(
                "({{ let __terrane_owner = {}.upgrade().expect(\"reference expired\"); {expression} }})",
                self.expression(receiver)
            )
        } else {
            expression
        }
    }

    pub(super) fn borrowed_expression(&mut self, node: &SyntaxNode) -> String {
        if node.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = node.children.as_slice()
            && matches!(self.value_type(receiver), Some(ValueType::Entry(_, _)))
            && matches!(self.text(member), "key" | "value")
        {
            return format!("({}).{}", self.expression(receiver), self.text(member));
        }
        self.expression(node)
    }

    pub(super) fn display_expression(&mut self, node: &SyntaxNode) -> String {
        if matches!(self.value_type(node), Some(ValueType::Descriptor(_))) {
            format!("({}).name", self.borrowed_expression(node))
        } else if matches!(
            self.value_type(node),
            Some(ValueType::Reference(_) | ValueType::SharedReference(_))
        ) {
            self.receiver_expression(node)
        } else {
            self.borrowed_expression(node)
        }
    }

    pub(super) fn class_designator(&self, node: &SyntaxNode) -> Option<&ObjectContract> {
        let name = self.text(node);
        if name == "self" {
            let identity = self.current_object.as_ref()?;
            return self
                .unit
                .objects
                .iter()
                .find(|object| object.identity == *identity && object.kind == ObjectKind::Class);
        }
        if self.unit.typed_bindings.iter().rev().any(|binding| {
            binding.name == name && binding.is_visible_at(self.unit.source.id(), node.span.start)
        }) {
            return None;
        }
        self.unit
            .objects
            .iter()
            .find(|object| object.name == name && object.kind == ObjectKind::Class)
    }

    pub(super) fn static_member(&mut self, node: &SyntaxNode) -> String {
        let [receiver, member] = node.children.as_slice() else {
            return String::new();
        };
        let Some(object) = self.class_designator(receiver) else {
            return String::new();
        };
        let member_name = self.text(member);
        if effective_object_fields(self.unit, object)
            .iter()
            .any(|field| field.is_static && field.name == member_name)
        {
            return format!(
                "({}.lock().expect(\"static field lock poisoned\")).clone()",
                rust_static_field_name(self.package, &object.identity, member_name)
            );
        }
        format!(
            "{}::terrane_static_{}",
            rust_object_type_name(self.package, &object.identity),
            rust_name(member_name)
        )
    }

    #[expect(
        clippy::too_many_lines,
        reason = "member lowering keeps one ordered dispatch across scalar and collection surfaces"
    )]
    pub(super) fn member(&mut self, node: &SyntaxNode) -> String {
        let [receiver, member] = node.children.as_slice() else {
            return String::new();
        };
        let receiver_type = self.receiver_value_type(receiver);
        if let Some(ValueType::Descriptor(_)) = &receiver_type {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "name" => format!("({receiver}).name.to_owned()"),
                "kind" => format!("({receiver}).kind.to_owned()"),
                "identity" => format!("({receiver}).identity.to_owned()"),
                _ => String::new(),
            };
        }
        if matches!(
            receiver_type,
            Some(ValueType::Function(_, _) | ValueType::AsyncFunction(_, _))
        ) && matches!(
            self.text(member),
            "contracts" | "throwable-contract" | "escaping-throwables"
        ) {
            let reflected = self
                .unit
                .functions
                .iter()
                .find(|contract| contract.name == self.text(receiver))
                .map(|contract| match self.text(member) {
                    "escaping-throwables" => contract
                        .escaping_throwables
                        .iter()
                        .map(|identity| {
                            identity
                                .rsplit_once("::")
                                .map_or(identity.as_str(), |(_, name)| name)
                        })
                        .collect::<Vec<_>>()
                        .join("|"),
                    "throwable-contract" => contract
                        .thrown_types
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join("|"),
                    _ if contract.throws => "throws".to_owned(),
                    _ => String::new(),
                })
                .unwrap_or_default();
            return format!(
                "{{ let _ = {}; {:?}.to_owned() }}",
                self.expression(receiver),
                reflected
            );
        }
        if let Some(ValueType::TaskOutcome(_)) = &receiver_type {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "completed" => format!("({receiver}).completed"),
                "cancelled" => format!("({receiver}).cancelled"),
                "value" => format!("({receiver}).value.clone()"),
                "error" => format!("({receiver}).error"),
                _ => String::new(),
            };
        }
        if matches!(
            receiver_type,
            Some(
                ValueType::PlatformOpenResult
                    | ValueType::PlatformReadResult
                    | ValueType::PlatformWriteResult
                    | ValueType::PlatformUnitResult
            )
        ) {
            let receiver = self.expression(receiver);
            return match self.text(member) {
                "handle" | "data" | "completed" | "message" => {
                    format!("({receiver}).{}.clone()", self.text(member))
                }
                name => format!("({receiver}).{name}"),
            };
        }
        if let Some(length) =
            self.direct_string_view_length(receiver, receiver_type.clone(), member)
        {
            return length;
        }
        let wrapped_field = self.wrapped_object_field(receiver, self.text(member));
        if matches!(
            self.value_type(receiver),
            Some(ValueType::Reference(_) | ValueType::SharedReference(_))
        ) && matches!(receiver_type, Some(ValueType::Object(_)))
        {
            let guard = self.receiver_guard_expression(receiver);
            let field = rust_name(self.text(member));
            let access = if wrapped_field {
                format!("({guard}).terrane_field_{field}().clone()")
            } else {
                format!("({guard}).{field}.clone()")
            };
            return self.wrap_receiver_guard(receiver, access);
        }
        let receiver = self.receiver_expression(receiver);
        if self.text(member) == "length"
            && matches!(
                self.value_type(node),
                Some(ValueType::Function(_, _) | ValueType::AsyncFunction(_, _))
            )
        {
            return format!("({receiver}).length");
        }
        match self.text(member) {
            "bytes" if receiver_type == Some(ValueType::Scalar(ScalarType::String)) => {
                format!("({receiver}).as_bytes().to_vec()")
            }
            "scalars" if receiver_type == Some(ValueType::Scalar(ScalarType::String)) => {
                format!("({receiver}).chars().map(|value| value.to_string()).collect::<Vec<_>>()")
            }
            "graphemes" if receiver_type == Some(ValueType::Scalar(ScalarType::String)) => {
                format!("terrane_string_support::graphemes(&({receiver})).collect::<Vec<_>>()")
            }
            "text" if receiver_type == Some(ValueType::TextRange) => {
                format!("({receiver}).text().to_owned()")
            }
            "bytes" | "scalars" | "graphemes" if receiver_type == Some(ValueType::TextRange) => {
                receiver
            }
            boundary @ ("start" | "end")
                if matches!(receiver_type, Some(ValueType::TextRangeView(_))) =>
            {
                let method = match (receiver_type.clone(), boundary) {
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Bytes)),
                        "start",
                    ) => "byte_start",
                    (Some(ValueType::TextRangeView(crate::semantics::TextUnit::Bytes)), "end") => {
                        "byte_end"
                    }
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Scalars)),
                        "start",
                    ) => "scalar_start",
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Scalars)),
                        "end",
                    ) => "scalar_end",
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Graphemes)),
                        "start",
                    ) => "grapheme_start",
                    (
                        Some(ValueType::TextRangeView(crate::semantics::TextUnit::Graphemes)),
                        "end",
                    ) => "grapheme_end",
                    _ => unreachable!("semantic analysis validated text-range boundary"),
                };
                format!("({receiver}).{method}() as i128")
            }
            "length" => match receiver_type {
                Some(
                    ValueType::StringView(crate::semantics::TextUnit::Bytes)
                    | ValueType::Scalar(ScalarType::Bytes),
                ) => {
                    format!("({receiver}).len() as i128")
                }
                Some(
                    ValueType::StringView(
                        crate::semantics::TextUnit::Scalars | crate::semantics::TextUnit::Graphemes,
                    )
                    | ValueType::StringList
                    | ValueType::TextRangeList,
                ) => format!("({receiver}).len() as i128"),
                Some(
                    ValueType::List(_)
                    | ValueType::Map(_, _)
                    | ValueType::Set(_)
                    | ValueType::Tuple(_, _)
                    | ValueType::UnorderedMap(_, _)
                    | ValueType::UnorderedSet(_),
                ) => format!("terrane_int_support::Int::from(({receiver}).length())"),
                _ => format!("terrane_string_support::length(&{receiver}) as i128"),
            },
            "key" if matches!(receiver_type, Some(ValueType::Entry(_, _))) => {
                format!("({receiver}).key.clone()")
            }
            "value" if matches!(receiver_type, Some(ValueType::Entry(_, _))) => {
                format!("({receiver}).value.clone()")
            }
            "type" => "()".to_owned(),
            name if matches!(
                receiver_type,
                Some(ValueType::Scalar(ScalarType::Float32 | ScalarType::Float64))
            ) && float_member_contract(name)
                .is_some_and(|contract| contract.arity.is_none()) =>
            {
                let operation = float_member_contract(name)
                    .expect("validated floating property")
                    .operation;
                let method = match operation {
                    FloatMemberOperation::Finite => "is_finite",
                    FloatMemberOperation::Infinite => "is_infinite",
                    FloatMemberOperation::NotANumber => "is_nan",
                    _ => unreachable!("callable floating member used as a property"),
                };
                format!("({receiver}).{method}()")
            }
            name if wrapped_field => {
                format!("({receiver}).terrane_field_{}().clone()", rust_name(name))
            }
            name => format!("{receiver}.{}", rust_name(name)),
        }
    }

    pub(super) fn float_call(
        &self,
        float_type: ScalarType,
        operation: &str,
        receiver: &str,
        arguments: &[String],
        node: &SyntaxNode,
    ) -> Option<String> {
        let contract = float_member_contract(operation)?;
        (contract.arity == Some(arguments.len())).then_some(())?;
        let call = match contract.operation {
            FloatMemberOperation::SineCosine => format!(
                "{{ let terrane_sine_cosine = ({receiver}).sin_cos(); \
                 terrane_collection_support::Tuple::new(vec![\
                 terrane_sine_cosine.0, terrane_sine_cosine.1]) }}"
            ),
            operation @ (FloatMemberOperation::Round
            | FloatMemberOperation::Floor
            | FloatMemberOperation::Ceiling
            | FloatMemberOperation::Truncate) => {
                let helper = if float_type == ScalarType::Float32 {
                    "rounded_f32"
                } else {
                    "rounded_f64"
                };
                let mode = match operation {
                    FloatMemberOperation::Round => "TiesEven",
                    FloatMemberOperation::Floor => "Floor",
                    FloatMemberOperation::Ceiling => "Ceiling",
                    FloatMemberOperation::Truncate => "Truncate",
                    _ => unreachable!(),
                };
                self.fallible(
                    format!(
                        "terrane_int_support::{helper}({receiver}, \
                         terrane_int_support::FloatRounding::{mode})"
                    ),
                    node,
                )
            }
            operation @ (FloatMemberOperation::SquareRoot
            | FloatMemberOperation::Sine
            | FloatMemberOperation::Cosine
            | FloatMemberOperation::NaturalLog
            | FloatMemberOperation::Exponential
            | FloatMemberOperation::Absolute) => {
                let method = match operation {
                    FloatMemberOperation::SquareRoot => "sqrt",
                    FloatMemberOperation::Sine => "sin",
                    FloatMemberOperation::Cosine => "cos",
                    FloatMemberOperation::NaturalLog => "ln",
                    FloatMemberOperation::Exponential => "exp",
                    FloatMemberOperation::Absolute => "abs",
                    _ => unreachable!(),
                };
                format!("({receiver}).{method}()")
            }
            operation @ (FloatMemberOperation::Minimum | FloatMemberOperation::Maximum) => {
                let other = &arguments[0];
                let zero_selection = if operation == FloatMemberOperation::Minimum {
                    "if terrane_receiver.is_sign_negative() || \
                     terrane_argument.is_sign_negative() { -0.0 } else { 0.0 }"
                } else {
                    "if terrane_receiver.is_sign_positive() || \
                     terrane_argument.is_sign_positive() { 0.0 } else { -0.0 }"
                };
                let method = if operation == FloatMemberOperation::Minimum {
                    "min"
                } else {
                    "max"
                };
                format!(
                    "{{ let terrane_receiver = {receiver}; let terrane_argument = {other}; \
                     if terrane_receiver == 0.0 && terrane_argument == 0.0 {{ \
                     {zero_selection} }} else {{ terrane_receiver.{method}(terrane_argument) }} }}"
                )
            }
            FloatMemberOperation::MultiplyAdd => {
                format!("({receiver}).mul_add({}, {})", arguments[0], arguments[1])
            }
            FloatMemberOperation::Finite
            | FloatMemberOperation::Infinite
            | FloatMemberOperation::NotANumber => return None,
        };
        Some(call)
    }
}
