use crate::{
    ScalarType,
    lowering::{
        emitter::Emitter,
        helpers::{rust_value_type, union_type_name},
    },
    semantics::{TypedBinding, ValueType, contextual_constant, is_numeric},
    syntax::SyntaxNode,
};

impl Emitter<'_> {
    pub(in crate::lowering::emitter) fn flow_binding_type(
        &self,
        binding: &TypedBinding,
    ) -> ValueType {
        self.unit
            .flow_binding_types
            .get(&binding.span)
            .cloned()
            .unwrap_or_else(|| binding.value_type.clone())
    }

    pub(in crate::lowering::emitter) fn union_binding(
        &self,
        node: &SyntaxNode,
    ) -> Option<TypedBinding> {
        self.local_typed_binding(node)
            .filter(|binding| {
                !binding.destination_arms.is_empty()
                    || matches!(self.flow_binding_type(binding), ValueType::Union(_))
            })
            .cloned()
            .map(|mut binding| {
                binding.value_type = self.flow_binding_type(&binding);
                binding
            })
    }

    pub(in crate::lowering::emitter) fn union_arms(
        &self,
        binding: &TypedBinding,
    ) -> Vec<ValueType> {
        if let ValueType::Union(arms) = self.flow_binding_type(binding) {
            arms
        } else {
            binding
                .destination_arms
                .iter()
                .copied()
                .map(ValueType::Scalar)
                .collect()
        }
    }

    pub(in crate::lowering::emitter) fn union_value(
        &mut self,
        binding: &TypedBinding,
        value: &SyntaxNode,
    ) -> String {
        let arms = self.union_arms(binding);
        let actual = self.value_type(value);
        let selected = actual
            .as_ref()
            .and_then(|actual| arms.iter().find(|arm| *arm == actual))
            .cloned()
            .or_else(|| {
                let actual_scalar = actual.and_then(|actual| match actual {
                    ValueType::Scalar(scalar) => Some(scalar),
                    _ => None,
                });
                arms.iter().find_map(|arm| {
                    let ValueType::Scalar(scalar) = arm else {
                        return None;
                    };
                    (contextual_constant(self.source, value, *scalar).is_some()
                        || actual_scalar
                            .is_some_and(|actual| is_numeric(actual) && is_numeric(*scalar)))
                    .then(|| arm.clone())
                })
            })
            .expect("validated union destination");
        let index = arms
            .iter()
            .position(|arm| *arm == selected)
            .expect("selected union arm belongs to destination");
        format!(
            "{}::Arm{index}({})",
            union_type_name(binding),
            self.expression_as(value, selected)
        )
    }

    pub(in crate::lowering::emitter) fn emit_union_types(&mut self) {
        let mut emitted = std::collections::BTreeSet::new();
        let unit = self.unit;
        for binding in &unit.typed_bindings {
            if binding.destination_arms.is_empty()
                && !matches!(self.flow_binding_type(binding), ValueType::Union(_))
            {
                continue;
            }
            let name = union_type_name(binding);
            if !emitted.insert(name.clone()) {
                continue;
            }
            let arms = self.union_arms(binding);
            let arm_types = arms
                .iter()
                .map(|arm| rust_value_type(self.package, arm.clone()))
                .collect::<Vec<_>>();
            let borrowed = arm_types.iter().any(|arm| arm.contains('&'));
            let lifetime = if borrowed { "<'a>" } else { "" };
            self.line("#[allow(dead_code)]");
            if !arms.iter().any(|arm| self.value_type_owns_resource(arm)) {
                self.line("#[derive(Clone)]");
            }
            self.line(&format!("enum {name}{lifetime} {{"));
            self.indent += 1;
            for (index, arm) in arm_types.iter().enumerate() {
                let arm = if borrowed {
                    std::borrow::Cow::Owned(arm.replace('&', "&'a "))
                } else {
                    std::borrow::Cow::Borrowed(arm.as_str())
                };
                self.line(&format!("Arm{index}({arm}),"));
            }
            self.indent -= 1;
            self.line("}");
            let printable = |arm: &ValueType| {
                matches!(arm, ValueType::Scalar(scalar) if *scalar != ScalarType::Bytes)
                    || matches!(arm, ValueType::StringView(_))
            };
            if arms.iter().any(printable) {
                self.line(&format!(
                    "impl{lifetime} terrane_scalar_support::ScalarDisplay for {name}{lifetime} {{"
                ));
                self.indent += 1;
                self.line("fn write_scalar(&self, output: &mut String) {");
                self.indent += 1;
                self.line("match self {");
                self.indent += 1;
                for (index, arm) in arms.iter().enumerate() {
                    if printable(arm) {
                        self.line(&format!(
                            "Self::Arm{index}(value) => terrane_scalar_support::ScalarDisplay::write_scalar(value, output),"
                        ));
                    } else {
                        self.line(&format!(
                            "Self::Arm{index}(_) => unreachable!(\"semantic display refinement excludes this union arm\"),"
                        ));
                    }
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
}
