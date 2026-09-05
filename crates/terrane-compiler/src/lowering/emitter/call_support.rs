use super::super::prelude::*;

impl Emitter<'_> {
    pub(super) fn fallible(&self, call: impl AsRef<str>, node: &SyntaxNode) -> String {
        let call = call.as_ref();
        let site = self.error_site(node);
        if self.try_completion {
            format!("__terrane_raised_completion!({call}, {site})")
        } else if self.propagate_errors {
            format!("__terrane_raised_err({call}, {site})?")
        } else {
            format!("__terrane_raised({call}, {site})")
        }
    }

    pub(super) fn error_site(&self, node: &SyntaxNode) -> String {
        let function = self
            .current_function
            .as_deref()
            .unwrap_or(self.unit.namespace.as_str());
        let site =
            self.registry
                .register_site(&self.unit.source_path, function, self.source, node.span);
        let (line, column) = self.source.line_column(node.span.start);
        let (end_line, end_column) = self.source.line_column(node.span.end);
        let comment = format!(
            "{}:{line}:{column}-{end_line}:{end_column}",
            self.unit.source_path
        );
        format!("__terrane_comment!({site}, {comment:?})")
    }

    pub(super) fn numeric_destination(
        &mut self,
        node: &SyntaxNode,
        source: ScalarType,
        destination: ScalarType,
    ) -> String {
        let value = self.receiver_expression(node);
        if destination == ScalarType::Int {
            if matches!(source, ScalarType::Float32 | ScalarType::Float64) {
                let helper = if source == ScalarType::Float32 {
                    "exact_int_f32"
                } else {
                    "exact_int_f64"
                };
                return self.fallible(format!("terrane_int_support::{helper}({value})"), node);
            }
            return if source == ScalarType::Uint128 {
                format!("terrane_int_support::adaptive(&({value}))")
            } else {
                format!("terrane_int_support::Int::from(({value}) as i128)")
            };
        }
        if source == ScalarType::Int && destination.is_integer() {
            return self.fallible(
                format!(
                    "terrane_int_support::coerce::<{}>(&({value}))",
                    rust_type(destination)
                ),
                node,
            );
        }
        if source == ScalarType::Int {
            let helper = if destination == ScalarType::Float32 {
                "exact_f32"
            } else {
                "exact_f64"
            };
            return self.fallible(format!("terrane_int_support::{helper}(&({value}))"), node);
        }
        if source.is_integer() && destination.is_integer() {
            if integer_range_contains(destination, source) {
                return format!("(({value}) as {})", rust_type(destination));
            }
            let conversion = self.fallible(
                format!(
                    "{}::try_from(source_value).map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(&source_value, \"{source}\", \"{destination}\", \"the value is outside the destination range\"))",
                    rust_type(destination)
                ),
                node,
            );
            return format!("{{ let source_value = {value}; {conversion} }}");
        }
        if source == ScalarType::Float32 && destination == ScalarType::Float64 {
            return format!("(({value}) as f64)");
        }
        if source.is_integer() {
            if exact_integer_float_widening(source, destination)
                || self.bounded_float_conversion_is_exact(node, destination)
            {
                return format!("(({value}) as {})", rust_type(destination));
            }
            let helper = if destination == ScalarType::Float32 {
                "exact_fixed_f32"
            } else {
                "exact_fixed_f64"
            };
            return self.fallible(format!("terrane_int_support::{helper}({value})"), node);
        }
        if destination.is_integer() {
            let helper = if source == ScalarType::Float32 {
                "exact_from_f32"
            } else {
                "exact_from_f64"
            };
            return self.fallible(
                format!(
                    "terrane_int_support::{helper}::<{}>({value})",
                    rust_type(destination)
                ),
                node,
            );
        }
        let conversion = self.fallible(
            "Err(terrane_int_support::ArithmeticError::conversion_overflow(&source_value, \"float64\", \"float32\", \"the floating value is not exactly representable\"))",
            node,
        );
        format!(
            "{{ let source_value = {value}; let converted = source_value as f32; if (converted as f64) == source_value {{ converted }} else {{ {conversion} }} }}"
        )
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn string_call(
        &mut self,
        node: &SyntaxNode,
        arguments: &SyntaxNode,
    ) -> Option<String> {
        let selection = string_call_selection(self.source, node)?;
        let subject = find_node_by_span(&self.unit.tree.root, selection.receiver)
            .expect("selected string receiver belongs to this syntax tree");
        if matches!(self.value_type(subject), Some(ValueType::Object(_))) {
            return None;
        }
        let family = selection.family.source_name();
        let child = selection.child.as_str();
        let receiver = self.receiver_expression(subject);
        let values = arguments
            .children
            .iter()
            .map(|argument| argument.children.last().unwrap_or(argument))
            .map(|value| self.expression(value))
            .collect::<Vec<_>>();
        let result = match (family, child) {
            ("trim", "default") => {
                format!("terrane_string_support::trim(&({receiver}))")
            }
            ("trim", mode @ ("start" | "end")) => {
                let helper = if mode == "start" {
                    "trim_start"
                } else {
                    "trim_end"
                };
                let pattern = values
                    .first()
                    .map_or_else(|| "None".to_owned(), |value| format!("Some(&({value}))"));
                format!("terrane_string_support::{helper}(&({receiver}), {pattern})")
            }
            ("contains", "default") => format!("({receiver}).contains(&({}))", values[0]),
            ("contains", "start") => format!("({receiver}).starts_with(&({}))", values[0]),
            ("contains", "end") => format!("({receiver}).ends_with(&({}))", values[0]),
            ("find", "default") => {
                format!(
                    "terrane_string_support::find(&({receiver}), &({}))",
                    values[0]
                )
            }
            ("find", "all") => {
                format!(
                    "terrane_string_support::find_all(&({receiver}), &({}))",
                    values[0]
                )
            }
            ("find", "count") => format!(
                "terrane_int_support::Int::from(terrane_string_support::find_all(&({receiver}), &({})).len() as i128)",
                values[0]
            ),
            ("upper", mode) => {
                let helper = match mode {
                    "default" => "upper",
                    "first" => "upper_first",
                    "words" => "upper_words",
                    _ => unreachable!(),
                };
                format!("terrane_string_support::{helper}(&({receiver}))")
            }
            ("lower", mode) => {
                let helper = match mode {
                    "default" => "lower",
                    "first" => "lower_first",
                    _ => unreachable!(),
                };
                format!("terrane_string_support::{helper}(&({receiver}))")
            }
            ("case-fold", _) => format!("terrane_string_support::case_fold(&({receiver}))"),
            ("normalise", form) => {
                format!("terrane_string_support::normalise(&({receiver}), {form:?})")
            }
            ("split", _) => {
                format!(
                    "terrane_string_support::split(&({receiver}), &({}))",
                    values[0]
                )
            }
            ("replace", _) => format!(
                "terrane_string_support::replace(&({receiver}), &({}), &({}))",
                values[0], values[1]
            ),
            ("encode", _) => {
                format!(
                    "terrane_string_support::encode(&({receiver}), {})",
                    values[0]
                )
            }
            ("decode", _) => self.fallible(
                format!(
                    "terrane_string_support::decode(&({receiver}), {})",
                    values[0]
                ),
                node,
            ),
            _ => unreachable!("semantic analysis validated string family"),
        };
        Some(result)
    }
    #[allow(clippy::too_many_lines)]
    pub(super) fn arithmetic_family(
        &mut self,
        family: ArithmeticFamily,
        child: &str,
        receiver_node: &SyntaxNode,
        arguments: &SyntaxNode,
        call: &SyntaxNode,
    ) -> String {
        let Some(ValueType::Scalar(receiver_type)) = self.receiver_value_type(receiver_node) else {
            unreachable!("validated arithmetic receiver");
        };
        let receiver = if receiver_type == ScalarType::Int {
            self.expression_as(receiver_node, ValueType::Scalar(ScalarType::Int))
        } else {
            Self::unwrapped_expression(self.receiver_expression(receiver_node))
        };
        let argument = arguments
            .children
            .first()
            .and_then(|argument| argument.children.last())
            .map(|argument| {
                if receiver_type == ScalarType::Int {
                    self.adaptive_expression(argument)
                } else if matches!(
                    family,
                    ArithmeticFamily::ShiftLeft | ArithmeticFamily::ShiftRight
                ) {
                    self.expression(argument)
                } else {
                    self.expression_as(argument, ValueType::Scalar(receiver_type))
                }
            });
        if receiver_type == ScalarType::Int {
            let expression = match family {
                ArithmeticFamily::Add => format!("({receiver} + {})", argument.unwrap()),
                ArithmeticFamily::Subtract => format!("({receiver} - {})", argument.unwrap()),
                ArithmeticFamily::Multiply => format!("({receiver} * {})", argument.unwrap()),
                ArithmeticFamily::Divide => {
                    format!("({receiver}).euclidean_div(&({}))", argument.unwrap())
                }
                ArithmeticFamily::Remainder => {
                    format!("({receiver}).modulo(&({}))", argument.unwrap())
                }
                ArithmeticFamily::DivRem => {
                    format!("({receiver}).div_rem(&({}))", argument.unwrap())
                }
                ArithmeticFamily::Negate => format!("-({receiver})"),
                ArithmeticFamily::ShiftLeft => {
                    format!("({receiver}).shift_left(&({}))", argument.unwrap())
                }
                ArithmeticFamily::ShiftRight => {
                    format!("({receiver}).shift_right(&({}))", argument.unwrap())
                }
            };
            return if child == "checked" {
                format!("({expression}).ok()")
            } else if matches!(
                family,
                ArithmeticFamily::Divide
                    | ArithmeticFamily::Remainder
                    | ArithmeticFamily::DivRem
                    | ArithmeticFamily::ShiftLeft
                    | ArithmeticFamily::ShiftRight
            ) {
                self.fallible(expression, call)
            } else {
                expression
            };
        }
        let operation = match family {
            ArithmeticFamily::Add => "addition",
            ArithmeticFamily::Subtract => "subtraction",
            ArithmeticFamily::Multiply => "multiplication",
            ArithmeticFamily::Divide => "division",
            ArithmeticFamily::Remainder => "remainder",
            ArithmeticFamily::DivRem => "div_rem",
            ArithmeticFamily::Negate => "negation",
            ArithmeticFamily::ShiftLeft => "shift_left",
            ArithmeticFamily::ShiftRight => "shift_right",
        };
        let helper = if child == "default" {
            format!("fixed_{operation}")
        } else {
            format!("fixed_{operation}_{child}")
        };
        let expression = if family == ArithmeticFamily::Negate {
            format!("terrane_int_support::{helper}({receiver})")
        } else if matches!(
            family,
            ArithmeticFamily::ShiftLeft | ArithmeticFamily::ShiftRight
        ) {
            format!(
                "terrane_int_support::{helper}({receiver}, &({}))",
                argument.unwrap()
            )
        } else {
            format!(
                "terrane_int_support::{helper}({receiver}, {})",
                argument.unwrap()
            )
        };
        let fallible = child == "default"
            || matches!(
                family,
                ArithmeticFamily::Divide
                    | ArithmeticFamily::Remainder
                    | ArithmeticFamily::DivRem
                    | ArithmeticFamily::ShiftLeft
                    | ArithmeticFamily::ShiftRight
            );
        if fallible {
            self.fallible(expression, call)
        } else {
            expression
        }
    }

    fn numeric_coercion_types(
        &self,
        receiver: &SyntaxNode,
        arguments: &SyntaxNode,
    ) -> Option<(ScalarType, ScalarType)> {
        let destination = arguments
            .children
            .first()
            .and_then(|argument| argument.children.last())
            .or_else(|| arguments.children.first())?;
        let ValueType::Scalar(source) = self.receiver_value_type(receiver)? else {
            return None;
        };
        Some((source, self.descriptor_type(destination)?))
    }

    pub(super) fn numeric_coercion(
        &mut self,
        method: &crate::BoundMethod,
        receiver: &SyntaxNode,
        callee: &SyntaxNode,
        arguments: &SyntaxNode,
    ) -> String {
        let policy = match method.child {
            "default" => CoercionPolicy::Default,
            child => CoercionPolicy::from_member(child)
                .expect("validated coercion family child must select a policy"),
        };
        let Some((source, destination)) = self.numeric_coercion_types(receiver, arguments) else {
            unreachable!(
                "semantic analysis admitted a non-scalar numeric coercion at {}..{}",
                callee.span.start, callee.span.end
            );
        };
        let receiver_is_borrowed = receiver.kind == SyntaxKind::Name
            && self.lazy_namespace_binding_type(receiver).is_some();

        if destination.is_integer() {
            if policy == CoercionPolicy::Default && !receiver_is_borrowed {
                if source == destination {
                    return if destination == ScalarType::Int
                        && self.small_int_binding(receiver).is_some()
                    {
                        self.expression_as(receiver, ValueType::Scalar(ScalarType::Int))
                    } else {
                        self.receiver_expression(receiver)
                    };
                }
                return self.numeric_destination(receiver, source, destination);
            }
            let helper = match policy {
                CoercionPolicy::Default => "coerce",
                CoercionPolicy::Checked => "checked_coerce",
                CoercionPolicy::Wrap => "wrapping_coerce",
                CoercionPolicy::Saturate => "saturating_coerce",
            };
            let receiver = self.receiver_expression(receiver);
            let source = if receiver_is_borrowed {
                receiver
            } else {
                format!("&({receiver})")
            };
            let call = format!(
                "terrane_int_support::{helper}::<{}>({source})",
                rust_type(destination)
            );
            return if policy == CoercionPolicy::Default {
                self.fallible(call, callee)
            } else {
                call
            };
        }

        self.numeric_float_coercion(
            source,
            destination,
            receiver,
            callee,
            receiver_is_borrowed,
            policy,
        )
    }

    fn numeric_float_coercion(
        &mut self,
        source: ScalarType,
        destination: ScalarType,
        receiver: &SyntaxNode,
        callee: &SyntaxNode,
        receiver_is_borrowed: bool,
        policy: CoercionPolicy,
    ) -> String {
        let receiver = self.receiver_expression(receiver);
        let value = if receiver_is_borrowed {
            format!("*({receiver})")
        } else {
            receiver.clone()
        };
        let fallible = match (source, destination) {
            (ScalarType::Int, ScalarType::Float32) => Some(format!(
                "terrane_int_support::coerce_to_f32({})",
                if receiver_is_borrowed {
                    receiver
                } else {
                    format!("&({receiver})")
                }
            )),
            (ScalarType::Int, ScalarType::Float64) => Some(format!(
                "terrane_int_support::coerce_to_f64({})",
                if receiver_is_borrowed {
                    receiver
                } else {
                    format!("&({receiver})")
                }
            )),
            (ScalarType::Uint128, ScalarType::Float32) => {
                Some(format!("terrane_int_support::coerce_fixed_to_f32({value})"))
            }
            (ScalarType::Float64, ScalarType::Float32) => {
                Some(format!("terrane_int_support::coerce_f64_to_f32({value})"))
            }
            _ => None,
        };
        if let Some(call) = fallible {
            if policy == CoercionPolicy::Checked {
                return format!("({call}).ok()");
            }
            self.registry.uses_float_coercion_error.set(true);
            return self.fallible(call, callee);
        }

        let converted = if source == destination {
            value
        } else {
            format!("(({value}) as {})", rust_type(destination))
        };
        if policy == CoercionPolicy::Checked {
            format!("Some({converted})")
        } else {
            converted
        }
    }

    pub(super) fn descriptor_identity(&self, node: &SyntaxNode) -> Option<String> {
        if node.kind == SyntaxKind::TypeExpression {
            return node
                .children
                .first()
                .and_then(|child| self.descriptor_identity(child));
        }
        if node.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = node.children.as_slice()
            && self.text(member) == "type"
        {
            return self
                .value_type(receiver)
                .and_then(|value_type| match value_type {
                    ValueType::Scalar(value_type) => Some(format!("type:{value_type}")),
                    _ => None,
                });
        }
        crate::semantics::descriptor_expression_type(self.package, self.unit, node)
            .map(|scalar| format!("type:{scalar}"))
    }

    pub(super) fn descriptor_type(&self, node: &SyntaxNode) -> Option<ScalarType> {
        let resolved = crate::semantics::descriptor_expression_type(self.package, self.unit, node);
        resolved.or_else(|| {
            (node.kind == SyntaxKind::TypeExpression)
                .then(|| {
                    node.children
                        .first()
                        .and_then(|child| self.descriptor_type(child))
                })
                .flatten()
        })
    }

    pub(super) fn projected_function_for_call(
        &self,
        callee: &SyntaxNode,
    ) -> Option<&crate::projection::ProjectedFunction> {
        if callee.kind != SyntaxKind::Name {
            return None;
        }
        let symbol =
            self.package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))?;
        self.package
            .projection
            .item(&symbol.namespace, &symbol.name)
            .and_then(|item| match &item.kind {
                crate::projection::ProjectedKind::Function(function) => Some(function),
                _ => None,
            })
    }

    pub(super) fn contract_for_call(&self, callee: &SyntaxNode) -> Option<&FunctionContract> {
        if let [receiver, member] = callee.children.as_slice()
            && matches!(
                callee.kind,
                SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
            )
        {
            let is_static = callee.kind == SyntaxKind::StaticMemberExpression;
            let object = if is_static {
                self.class_designator(receiver)
            } else {
                self.receiver_value_type(receiver).and_then(|value_type| {
                    let ValueType::Object(identity) = value_type else {
                        return None;
                    };
                    self.unit
                        .objects
                        .iter()
                        .find(|object| object.identity == identity)
                })
            }?;
            return effective_object_methods(self.unit, object)
                .into_iter()
                .find(|contract| {
                    contract.name == self.text(member) && contract.is_static == is_static
                });
        }
        if callee.kind == SyntaxKind::ConstructionExpression {
            let object = callee
                .children
                .first()
                .and_then(|designator| self.class_designator(designator))?;
            return effective_object_methods(self.unit, object)
                .into_iter()
                .find(|contract| contract.name == "construct" && !contract.is_static);
        }
        if callee.kind != SyntaxKind::Name {
            return None;
        }
        let symbol =
            self.package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))?;
        let span = symbol.declaration_span?;
        self.package
            .units
            .iter()
            .flat_map(|unit| &unit.functions)
            .find(|contract| contract.span == span)
    }
}
