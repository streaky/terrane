use super::super::prelude::*;

impl Emitter<'_> {
    /// A stored callable has the source value ABI. Its native facade still lends
    /// those invocation-owned arguments rather than changing the callable type.
    pub(super) fn native_callable_invocation(
        &mut self,
        node: &SyntaxNode,
        parameters: &[CallableParameterType],
        throwing: bool,
        asynchronous: bool,
    ) -> Option<String> {
        let projected = self.projected_function_for_call(node, false)?.clone();
        if !projected.parameters.iter().any(|parameter| {
            parameter.preserves_borrowed_storage()
                && matches!(
                    parameter.ty,
                    crate::rust_interop::projection::ProjectedType::String
                        | crate::rust_interop::projection::ProjectedType::Bytes
                        | crate::rust_interop::projection::ProjectedType::Sequence { .. }
                )
        }) {
            return None;
        }
        let contract = self.contract_for_call(node, false)?;
        let function = function_name(self.package, contract);
        let arguments = projected
            .parameters
            .iter()
            .zip(parameters)
            .enumerate()
            .map(|(index, (projected, parameter))| {
                let name = format!("argument_{index}");
                if projected.preserves_borrowed_storage()
                    && matches!(
                        projected.ty,
                        crate::rust_interop::projection::ProjectedType::Sequence { .. }
                    )
                {
                    return format!("({name}).as_vec()");
                }
                if projected.preserves_borrowed_storage()
                    && !matches!(
                        parameter.value_type_ref(),
                        ValueType::Reference(_) | ValueType::SharedReference(_)
                    )
                {
                    format!(
                        "&{}{name}",
                        if projected.mutable_borrow { "mut " } else { "" }
                    )
                } else {
                    name
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let call = format!(
            "{function}({arguments}){}",
            if asynchronous { ".await" } else { "" }
        );
        let site = self.error_site(node);
        Some(if throwing {
            format!("__terrane_raised_err({call}, {site})")
        } else {
            format!("__terrane_raised({call}, {site})")
        })
    }

    pub(super) fn interface_read_parameter(
        &self,
        method: &FunctionContract,
        index: usize,
    ) -> Option<&crate::rust_interop::projection::ProjectedParameter> {
        let owner = method.owner_identity.as_ref()?;
        let projected = self.package.projection.interface_method(
            &owner.namespace,
            &owner.name,
            &method.name,
        )?;
        if !projected.provided || projected.function.is_async {
            return None;
        }
        projected
            .function
            .parameters
            .get(index)
            .filter(|parameter| parameter.preserves_borrowed_storage() && !parameter.mutable_borrow)
    }

    pub(super) fn forwarded_parameter_type(
        &self,
        method: &FunctionContract,
        index: usize,
    ) -> String {
        let parameter = &method.parameters[index];
        let source = parameter.binding_value_type().map_or_else(
            || "i128".to_owned(),
            |value| rust_value_type(self.package, value),
        );
        let Some(projected) = self.interface_read_parameter(method, index) else {
            return source;
        };
        if projected.borrowed_str {
            "&str".to_owned()
        } else if matches!(
            projected.ty,
            crate::rust_interop::projection::ProjectedType::Sequence { .. }
        ) {
            format!("&{}", projected.ty.rust_type())
        } else if matches!(
            parameter.binding_value_type(),
            Some(ValueType::Reference(_))
        ) {
            source
        } else {
            format!("&{source}")
        }
    }
}
