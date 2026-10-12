use std::fmt::Write as _;

use super::Emitter;
use crate::{
    ScalarType,
    lowering::helpers::{function_name, union_type_name},
    semantics::{ValueType, display_method},
    syntax::{SyntaxKind, SyntaxNode},
};

impl Emitter<'_> {
    fn display_receiver(&mut self, node: &SyntaxNode) -> String {
        if let Some(value) = self.narrowed_storage_name(node, false, false) {
            return value;
        }
        if self.reference_backed_name(node).is_some()
            && !matches!(
                self.value_type(node),
                Some(ValueType::Reference(_) | ValueType::SharedReference(_))
            )
        {
            return format!("*({})", self.receiver_guard_expression(node));
        }
        if node.kind == SyntaxKind::Name
            && let Some(binding) = self.local_typed_binding(node)
            && self.binding_may_be_unassigned(binding)
            && !self.reference_backed(binding)
        {
            return self.available_storage_reference(binding, node, false);
        }
        if matches!(self.receiver_value_type(node), Some(ValueType::Scalar(_))) {
            self.native_receiver_expression(node, false)
        } else {
            self.receiver_guard_expression(node)
        }
    }

    pub(super) fn text_display(&mut self, node: &SyntaxNode) -> String {
        if node.kind == SyntaxKind::GroupExpression
            && let [inner] = node.children.as_slice()
        {
            return self.text_display(inner);
        }
        let value_type = self
            .value_type(node)
            .expect("display operand has an inferred type");
        if let Some(method) = display_method(self.unit, &value_type, node.span.start) {
            let name = function_name(self.package, method);
            let throws = method.throws;
            let receiver = self.display_receiver(node);
            let call = format!("({receiver}).{name}()");
            let call = if throws {
                self.display_call_error(&call, node)
            } else {
                call
            };
            return self.wrap_receiver_guard(node, call);
        }
        if node.kind == SyntaxKind::Literal && value_type == ValueType::Scalar(ScalarType::String) {
            return self.expression(node);
        }
        if !matches!(
            self.receiver_value_type(node),
            Some(ValueType::Union(_) | ValueType::Optional(_))
        ) {
            let mut value = if matches!(value_type, ValueType::Descriptor(_)) {
                self.display_expression(node)
            } else {
                self.display_receiver(node)
            };
            let mut display_type = &value_type;
            while let ValueType::Reference(item) | ValueType::SharedReference(item) = display_type {
                value = format!("*({value})");
                display_type = item.value_type_ref();
            }
            let text = format!("terrane_scalar_support::scalar_text(&({value}))");
            return self.wrap_receiver_guard(node, text);
        }
        let write = self.write_display(node, "__terrane_text");
        format!("{{ let mut __terrane_text = String::new(); {write} __terrane_text }}")
    }

    fn write_display_value(
        &mut self,
        node: &SyntaxNode,
        value_type: &ValueType,
        value: &str,
        output: &str,
    ) -> String {
        match value_type {
            ValueType::Reference(item) | ValueType::SharedReference(item) => self
                .write_display_value(node, item.value_type_ref(), &format!("*({value})"), output),
            ValueType::Optional(inner) => {
                let present = self.write_display_value(node, inner, "__terrane_present", output);
                format!(
                    "match &({value}) {{ Some(__terrane_present) => {{ {present} }}, None => {output}.push_str(\"none\"), }}"
                )
            }
            ValueType::Union(reaching) => {
                let binding = self
                    .union_binding(node)
                    .expect("union display has canonical storage");
                let name = union_type_name(&binding);
                let cases = self
                    .union_arms(&binding)
                    .iter()
                    .enumerate()
                    .map(|(index, arm)| {
                        if !reaching.contains(arm) {
                            return format!(
                                "{name}::Arm{index}(_) => unreachable!(\"flow-proven storage refinement\")"
                            );
                        }
                        let write = self.write_display_value(node, arm, "__terrane_arm", output);
                        format!("{name}::Arm{index}(__terrane_arm) => {{ {write} }}")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("match &({value}) {{ {cases} }}")
            }
            _ => {
                if let Some(method) = display_method(self.unit, value_type, node.span.start) {
                    let method_name = function_name(self.package, method);
                    let throws = method.throws;
                    let call = format!("({value}).{method_name}()");
                    let text = if throws {
                        self.display_call_error(&call, node)
                    } else {
                        call
                    };
                    format!("{output}.push_str(&({text}));")
                } else {
                    format!(
                        "terrane_scalar_support::ScalarDisplay::write_scalar(&({value}), &mut {output});"
                    )
                }
            }
        }
    }

    pub(super) fn write_display(&mut self, node: &SyntaxNode, output: &str) -> String {
        if node.kind == SyntaxKind::GroupExpression
            && let [inner] = node.children.as_slice()
        {
            return self.write_display(inner, output);
        }
        let value_type = self
            .value_type(node)
            .expect("display operand has an inferred type");
        if node.kind == SyntaxKind::Literal && value_type == ValueType::Scalar(ScalarType::String) {
            let literal = crate::lowering::helpers::native_macro_literal(self.text(node));
            return format!("{output}.push_str({literal});");
        }
        let value = if matches!(value_type, ValueType::Descriptor(_)) {
            self.display_expression(node)
        } else {
            self.display_receiver(node)
        };
        let write = self.write_display_value(node, &value_type, &value, output);
        self.wrap_receiver_guard(node, write)
    }

    /// Flatten only string additions: numeric subtrees remain arithmetic operands.
    fn concatenation_parts<'a>(&self, node: &'a SyntaxNode, parts: &mut Vec<&'a SyntaxNode>) {
        if node.kind == SyntaxKind::BinaryExpression
            && self.value_type(node) == Some(ValueType::Scalar(ScalarType::String))
            && let [left, right] = node.children.as_slice()
            && self.source.text()[left.span.end..right.span.start].trim() == "+"
        {
            self.concatenation_parts(left, parts);
            self.concatenation_parts(right, parts);
        } else {
            parts.push(node);
        }
    }

    pub(super) fn string_addition(&mut self, node: &SyntaxNode) -> String {
        let mut parts = Vec::new();
        self.concatenation_parts(node, &mut parts);
        self.compose_text(&parts, None)
    }

    fn display_call_error(&self, call: &str, node: &SyntaxNode) -> String {
        let site = self.error_site(node);
        if self.try_completion {
            format!("__terrane_traced_completion!({call}, {site})")
        } else if self.propagate_errors {
            format!("__terrane_traced_err({call}, {site})?")
        } else {
            format!("__terrane_traced({call}, {site})")
        }
    }

    fn literal_text_length(&self, mut node: &SyntaxNode) -> Option<usize> {
        while node.kind == SyntaxKind::GroupExpression
            && let [inner] = node.children.as_slice()
        {
            node = inner;
        }
        if node.kind != SyntaxKind::Literal
            || self.value_type(node) != Some(ValueType::Scalar(ScalarType::String))
        {
            return None;
        }
        let literal = crate::lowering::helpers::native_macro_literal(self.text(node));
        Some(
            syn::parse_str::<syn::LitStr>(&literal)
                .expect("lowered string literal")
                .value()
                .len(),
        )
    }

    pub(super) fn compose_text(
        &mut self,
        parts: &[&SyntaxNode],
        separator: Option<&SyntaxNode>,
    ) -> String {
        let mut capacity = parts
            .iter()
            .filter_map(|part| self.literal_text_length(part))
            .sum::<usize>();
        if let Some(separator) = separator.and_then(|node| self.literal_text_length(node)) {
            capacity += separator * parts.len().saturating_sub(1);
        }
        let separator = separator.map(|node| {
            if self.literal_text_length(node).is_some() {
                let mut literal = node;
                while literal.kind == SyntaxKind::GroupExpression
                    && let [inner] = literal.children.as_slice()
                {
                    literal = inner;
                }
                crate::lowering::helpers::native_macro_literal(self.text(literal))
            } else {
                self.receiver_expression(node)
            }
        });
        let mut body = separator.as_ref().map_or_else(String::new, |value| {
            format!("let __terrane_separator = &({value}); ")
        });
        write!(
            body,
            "let mut __terrane_text = String::with_capacity({capacity}); "
        )
        .unwrap();
        for (index, part) in parts.iter().enumerate() {
            if index != 0 && separator.is_some() {
                body.push_str("__terrane_text.push_str(__terrane_separator); ");
            }
            body.push_str(&self.write_display(part, "__terrane_text"));
            body.push(' ');
        }
        format!("{{ {body} __terrane_text }}")
    }
}
