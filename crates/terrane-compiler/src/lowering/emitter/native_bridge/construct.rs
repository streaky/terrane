//! Render the raw invocation. Completion and native error handling happen afterwards.
use super::InvocationSite;
use crate::{
    lowering::prelude::*,
    rust_interop::projection::{ProjectedField, ProjectedType},
};

pub(super) struct StructConstruction<'a> {
    pub(super) fields: &'a [ProjectedField],
    pub(super) native_path: &'a str,
    pub(super) borrowed_view: bool,
    pub(super) native_struct_view: bool,
    pub(super) ordered_fields: Option<&'a [Option<String>]>,
    pub(super) converted_fields: &'a [bool],
}
impl Emitter<'_> {
    #[expect(
        clippy::too_many_lines,
        reason = "construction maps the selected source slots into the projected native fields"
    )]
    pub(super) fn render_struct_invocation(
        &self,
        site: &InvocationSite<'_>,
        construction: &StructConstruction<'_>,
        contract: Option<&FunctionContract>,
        name: &str,
        values: &[String],
    ) -> String {
        let InvocationSite {
            node,
            callee,
            arguments,
            ..
        } = *site;
        let StructConstruction {
            fields,
            native_path,
            borrowed_view,
            native_struct_view,
            ordered_fields,
            converted_fields,
        } = *construction;
        let selected_native = self.value_type(node).and_then(|value| match value {
            ValueType::Object(identity) => identity.into_native_projection(),
            _ => None,
        });
        let native_path = selected_native.as_deref().unwrap_or(native_path);
        let value_for = |field_name: &str| {
            ordered_fields
                .and_then(|slots| {
                    fields
                        .iter()
                        .position(|field| field.name == field_name)
                        .and_then(|index| slots.get(index))
                        .and_then(Clone::clone)
                })
                .or_else(|| {
                    contract.as_ref().and_then(|contract| {
                        contract
                            .parameters
                            .iter()
                            .zip(values)
                            .find_map(|(parameter, value)| {
                                (parameter.name == field_name).then(|| value.clone())
                            })
                    })
                })
                .or_else(|| {
                    arguments
                        .children
                        .iter()
                        .zip(values)
                        .find_map(|(argument, value)| {
                            argument
                                .children
                                .first()
                                .filter(|name| {
                                    name.kind == SyntaxKind::Name && self.text(name) == field_name
                                })
                                .map(|_| value.clone())
                        })
                })
        };
        let convert_field = |value: &str, ty: &ProjectedType| {
            projected_argument_expression(value, ty, self.package)
        };
        let projected_values = fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let value = value_for(&field.name);
                match (&field.ty, value) {
                    (ProjectedType::Optional(_), None) if native_struct_view => {
                        let Some(ValueType::Optional(inner)) = contract
                            .as_ref()
                            .and_then(|contract| contract.parameters.get(index))
                            .and_then(|parameter| parameter.value_type.as_ref())
                        else {
                            unreachable!("borrowed field has an optional source parameter");
                        };
                        format!(
                            "None::<{}>",
                            rust_value_type(self.package, inner.as_ref().clone())
                        )
                    }
                    (ProjectedType::Optional(inner), None) => {
                        format!("None::<{}>", inner.rust_type())
                    }
                    (ProjectedType::Optional(_), Some(value))
                        if !borrowed_view && converted_fields.get(index) == Some(&true) =>
                    {
                        format!("({value}).into()")
                    }
                    (_, Some(value))
                        if borrowed_view || converted_fields.get(index) == Some(&true) =>
                    {
                        value
                    }
                    (ProjectedType::Optional(_), Some(value)) => {
                        if field.ty.has_identity_representation() {
                            value
                        } else {
                            let mut source_type = contract
                                .as_ref()
                                .and_then(|contract| contract.parameters.get(index))
                                .and_then(crate::ParameterContract::element_value_type)
                                .or_else(|| {
                                    callee
                                        .children
                                        .first()
                                        .and_then(|designator| self.class_designator(designator))
                                        .and_then(|object| {
                                            object
                                                .fields
                                                .iter()
                                                .find(|candidate| candidate.name == field.name)
                                                .map(|field| field.value_type.clone())
                                        })
                                })
                                .expect("native struct field has a source parameter type");
                            crate::semantics::canonicalize_native_value_type(
                                self.package,
                                &mut source_type,
                            );
                            format!(
                                "{{ let value: {} = {value}; {} }}",
                                rust_value_type(self.package, source_type),
                                convert_field("value", &field.ty),
                            )
                        }
                    }
                    (_, Some(value)) => convert_field(&value, &field.ty),
                    (_, None) => {
                        unreachable!("semantics requires projected struct field `{}`", field.name)
                    }
                }
            })
            .collect::<Vec<_>>();
        if borrowed_view {
            let call = format!("{name}({})", projected_values.join(", "));
            if native_struct_view {
                call
            } else {
                format!("(|| -> Result<_, crate::TerraneForeignError> {{ Ok({call}) }})()")
            }
        } else {
            let assignments = fields
                .iter()
                .zip(projected_values)
                .map(|(field, value)| {
                    let field_name = crate::rust_ir::native_member_name(&field.rust_name);
                    format!("{field_name}: {value}")
                })
                .collect::<Vec<_>>()
                .join(", ");
            let native_path = native_path.replacen('<', "::<", 1);
            format!(
                "(|| -> Result<_, crate::TerraneForeignError> {{ Ok({native_path} {{ {assignments} }}) }})()"
            )
        }
    }
}

pub(super) fn macro_invocation(name: &str, values: &[String], result: &ProjectedType) -> String {
    let mut result_type = result.rust_type();
    if let ProjectedType::InvocationScoped { lifetimes, .. } = result {
        for lifetime in lifetimes {
            result_type = result_type.replace(lifetime, "'_");
        }
    }
    if matches!(result, ProjectedType::Generic(_)) {
        format!("{name}!({})", values.join(", "))
    } else {
        format!(
            "{{ let __terrane_macro_result: {} = core::convert::Into::into({name}!({})); __terrane_macro_result }}",
            result_type,
            values.join(", ")
        )
    }
}
