//! Shared invocation pipeline after builtin/intrinsic dispatch. Source calls take the
//! same ordering path, without a projected ABI or a native completion boundary.
use super::{
    InvocationSite, NativeBridgeCall,
    construct::{StructConstruction, macro_invocation},
    inputs::{OperandAbi, PreparedInputs},
    result::{CallSetup, Completion},
    target::{CallableField, InvocationTarget, TargetContext},
};
use crate::{lowering::prelude::*, syntax::call_is_unsafe};

impl Emitter<'_> {
    #[expect(
        clippy::too_many_lines,
        reason = "sequence the selected ABI, ordered operands, target, invocation and completion"
    )]
    pub(in crate::lowering::emitter) fn emit_invocation(
        &mut self,
        site: &InvocationSite<'_>,
        bridge: &NativeBridgeCall<'_>,
        argument_values: Vec<&SyntaxNode>,
        argument_first: bool,
    ) -> String {
        let InvocationSite {
            node,
            callee,
            source_callee,
            ..
        } = *site;
        let is_unsafe = call_is_unsafe(node) || call_is_unsafe(source_callee);
        let native_macro_path =
            crate::semantics::projected_macro_for_call(self.package, self.unit, callee)
                .map(|item| item.rust_path.as_str());
        let macro_graph = native_macro_path.map(|_| {
            bridge.selection.is_some_and(|s| {
                matches!(
                    s.value_type,
                    ValueType::InvocationScopedNative { concrete: true, .. }
                )
            }) || self.unit.invocation_scoped_result_at(node.span).is_some()
        });
        let specialization = bridge.selection;
        let projected_operation = bridge.instantiated_operation(self, source_callee);
        let projected_function = projected_operation.as_deref();
        let mut projected_parameters = projected_function.map(|function| {
            std::borrow::Cow::Borrowed(
                specialization.map_or(function.parameters.as_slice(), |plan| {
                    plan.projected_parameters.as_slice()
                }),
            )
        });
        let projected_interface_dispatch = callee
            .children
            .first()
            .and_then(|receiver| self.value_type(receiver))
            .and_then(|value_type| match value_type {
                ValueType::Object(identity) => Some(identity),
                ValueType::Reference(item)
                | ValueType::MutableReference(item)
                | ValueType::SharedReference(item) => match item.value_type() {
                    ValueType::Object(identity) => Some(identity),
                    _ => None,
                },
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
        let callable_field = if self.callable_static_field(callee) {
            Some(CallableField::Static)
        } else if callee.kind == SyntaxKind::MemberExpression
            && matches!(
                callee.children.as_slice(),
                [receiver, member] if self.callable_object_field(receiver, self.text(member))
            )
        {
            Some(CallableField::Instance)
        } else {
            None
        };
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
                .filter(|owner| owner.source.id() == contract.span.file)
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
        let native_struct = (callee.kind == SyntaxKind::ConstructionExpression)
            .then(|| {
                let designator = callee.children.first()?;
                let object = self.class_designator(designator)?;
                let ValueType::Object(identity) = self.value_type(node)? else {
                    return None;
                };
                (identity.namespace == object.identity.namespace
                    && identity.name == object.identity.name
                    && identity.is_unsafe == object.identity.is_unsafe)
                    .then(|| {
                        self.package
                            .projection
                            .projected_struct(&object.identity.namespace, &object.identity.name)
                    })
                    .flatten()
            })
            .flatten();
        let native_struct_view = native_struct.map(|(path, _, borrowed_view)| {
            borrowed_view && self.package.projection.enum_payload(path).is_none()
        });
        if projected_parameters.is_none()
            && native_struct_view == Some(false)
            && let Some((_, fields, _)) = native_struct
        {
            projected_parameters = Some(std::borrow::Cow::Owned(
                fields
                    .iter()
                    .map(
                        |field| crate::rust_interop::projection::ProjectedParameter {
                            name: field.name.clone(),
                            ty: field.ty.clone(),
                            borrow_lifetime: None,
                            borrowed: false,
                            mutable_borrow: false,
                            borrowed_str: false,
                            generic_parameter: None,
                            generic_bounds: Vec::new(),
                            generic_interface: None,
                            associated_type: None,
                        },
                    )
                    .collect(),
            ));
        }
        let region_producer = self.native_producer(node.span);
        let PreparedInputs {
            mut values,
            mut shared_argument_setup,
            native_conversion_inputs,
            native_field_values,
            native_field_converted,
            shared_async_receiver,
            guarded_method_receiver,
            native_owned_argument_future,
        } = self.prepare_invocation_inputs(
            site,
            &OperandAbi {
                projected_function,
                projected_parameters: projected_parameters.as_deref(),
                specialization,
                contract: contract.as_ref(),
                struct_fields: native_struct.map(|(_, fields, _)| fields),
                native_struct_view,
                projected_interface_dispatch,
            },
            argument_values,
            argument_first,
            macro_graph,
            region_producer,
        );
        let InvocationTarget {
            name,
            mut native_input_setup,
            native_receiver_setup,
            native_member_receiver,
        } = self.invocation_target(
            site,
            &TargetContext {
                contract: contract.as_ref(),
                native_macro_path,
                callable_field,
                operation: bridge.operation,
                projected_function,
                specialization,
                shared_async_receiver: shared_async_receiver.is_some(),
                guarded_method_receiver: guarded_method_receiver.as_ref(),
                native_owned_argument_future,
                is_unsafe,
            },
            &mut shared_argument_setup,
        );
        if let Some(handle_setup) = &shared_async_receiver {
            values.insert(0, "__terrane_shared_call_handle".to_owned());
            shared_argument_setup.insert_str(0, handle_setup);
        }
        let uses_direct_native_receiver = native_member_receiver.is_some();
        if let Some(receiver) = native_member_receiver {
            values.insert(0, receiver);
        }
        for (name, expression) in native_conversion_inputs {
            writeln!(native_input_setup, "let {name} = {expression};")
                .expect("writing cannot fail");
        }
        let callable_mode =
            self.value_type(callee)
                .and_then(|value_type| match value_type {
                    ValueType::Function(_, _, effects)
                    | ValueType::AsyncFunction(_, _, _, effects) => Some(effects.modes.written),
                    _ => None,
                });
        let call = if let Some((native_path, fields, borrowed_view)) = native_struct {
            self.render_struct_invocation(
                site,
                &StructConstruction {
                    fields,
                    native_path,
                    borrowed_view,
                    native_struct_view: native_struct_view == Some(true),
                    ordered_fields: native_field_values.as_deref(),
                    converted_fields: &native_field_converted,
                },
                contract.as_ref(),
                &name,
                &values,
            )
        } else if (contract.is_none() || callable_field.is_some())
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
            macro_invocation(
                &name,
                &values,
                &specialization
                    .expect("macro result destination is proven")
                    .projected_result,
            )
        } else {
            format!("{name}({})", values.join(", "))
        };
        self.complete_invocation(
            site,
            &Completion {
                contract: contract.as_ref(),
                projected_function,
                specialization,
                native_struct_view,
                macro_graph,
                callable_field,
                projected_interface_dispatch,
                uses_direct_native_receiver,
                producer: region_producer
                    .then(|| projected_function.expect("planned native producer")),
                native_owned_argument_future,
            },
            CallSetup {
                guard: guarded_method_receiver.map(|(setup, _)| setup),
                arguments: shared_argument_setup,
                receiver: native_receiver_setup,
                inputs: native_input_setup,
            },
            name,
            &values,
            call,
        )
    }
}
