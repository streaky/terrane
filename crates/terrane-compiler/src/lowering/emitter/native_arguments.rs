use super::super::prelude::*;
use crate::rust_interop::projection::{ProjectedParameter, ProjectedType};

pub(super) struct NativeArgumentContext<'a> {
    pub contract: &'a FunctionContract,
    pub parameter_index: usize,
    pub operand: &'a SyntaxNode,
    pub arguments: &'a SyntaxNode,
    pub projected: Option<&'a ProjectedParameter>,
    pub semantic_type: Option<&'a ValueType>,
    pub substitutions: Option<&'a BTreeMap<String, ProjectedType>>,
    pub borrowed_input_abi: bool,
    pub guarded_receiver: bool,
    pub lifetime: NativeArgumentLifetime,
}

#[derive(Clone, Copy)]
pub(super) enum NativeArgumentLifetime {
    Invocation,
    RetainedBorrow,
    OwnedFuture,
}

#[derive(Clone, Copy)]
pub(super) enum NativeArgumentMode {
    SourceValue,
    NativeValue,
    ForwardReference,
    Borrow { mutable: bool },
    Snapshot,
    RetainedValue,
}

impl NativeArgumentMode {
    pub(super) fn borrows_input(self) -> bool {
        matches!(
            self,
            Self::Borrow { .. } | Self::Snapshot | Self::RetainedValue
        )
    }

    pub(super) fn owns_lender(self) -> bool {
        matches!(self, Self::Snapshot | Self::RetainedValue)
    }
}

pub(super) struct NativeArgumentResult {
    pub expression: String,
    pub mode: NativeArgumentMode,
    pub has_invocation_callback: bool,
}

impl Emitter<'_> {
    pub(super) fn lower_native_argument(
        &mut self,
        context: &NativeArgumentContext<'_>,
        setup: &mut String,
        next_reference: &mut usize,
    ) -> NativeArgumentResult {
        let callback = context.projected.and_then(|parameter| {
            self.invocation_scoped_callback_adapter(
                context.operand,
                &parameter.ty,
                context.substitutions,
            )
            .or_else(|| self.borrowed_native_callback_adapter(context.operand, &parameter.ty))
        });
        if let Some(expression) = callback {
            return NativeArgumentResult {
                expression,
                mode: NativeArgumentMode::SourceValue,
                has_invocation_callback: true,
            };
        }
        if context.borrowed_input_abi
            && context.projected.is_some_and(|parameter| {
                parameter.borrowed_str && matches!(parameter.ty, ProjectedType::String)
            })
            && let Some(expression) = self.borrowed_string_literal(context.operand)
        {
            return NativeArgumentResult {
                expression,
                mode: NativeArgumentMode::Borrow { mutable: false },
                has_invocation_callback: false,
            };
        }
        let mode = self.native_argument_mode(context);
        let expression = match mode {
            NativeArgumentMode::Borrow { mutable } => self.staged_native_argument_reference(
                context.operand,
                mutable,
                setup,
                next_reference,
            ),
            NativeArgumentMode::Snapshot => {
                self.staged_read_snapshot(context.operand, setup, next_reference)
            }
            NativeArgumentMode::RetainedValue => {
                let value_type = self
                    .receiver_value_type(context.operand)
                    .expect("storage-preserving native input has a value type");
                self.staged_native_value_argument(
                    context.operand,
                    value_type,
                    setup,
                    next_reference,
                )
            }
            NativeArgumentMode::NativeValue => self.owned_native_argument(context.operand),
            NativeArgumentMode::ForwardReference => self.expression(context.operand),
            NativeArgumentMode::SourceValue => match context.semantic_type {
                Some(value_type) => self.expression_as(context.operand, value_type.clone()),
                None => self.expression(context.operand),
            },
        };
        NativeArgumentResult {
            expression,
            mode,
            has_invocation_callback: false,
        }
    }

    fn native_argument_mode(&self, context: &NativeArgumentContext<'_>) -> NativeArgumentMode {
        let projected = context.projected;
        if context.borrowed_input_abi
            && projected.is_some_and(|parameter| {
                parameter.preserves_borrowed_storage() && !parameter.mutable_borrow
            })
        {
            let retain_value = matches!(context.lifetime, NativeArgumentLifetime::OwnedFuture)
                && context.semantic_type.is_some_and(|ty| {
                    !matches!(ty, ValueType::Reference(_) | ValueType::SharedReference(_))
                });
            if !matches!(
                projected.map(|parameter| &parameter.ty),
                Some(ProjectedType::Foreign { .. })
            ) && (retain_value
                || self.native_read_needs_snapshot(
                    context.operand,
                    context.arguments,
                    context.guarded_receiver,
                    !matches!(context.lifetime, NativeArgumentLifetime::Invocation),
                ))
            {
                return if retain_value
                    && !self
                        .native_field_type(context.operand)
                        .is_some_and(|field| {
                            matches!(field, ProjectedType::Sequence { item, .. }
                        if item.has_identity_representation())
                        })
                {
                    NativeArgumentMode::RetainedValue
                } else {
                    NativeArgumentMode::Snapshot
                };
            }
            return NativeArgumentMode::Borrow { mutable: false };
        }
        if projected.is_some_and(ProjectedParameter::requires_mutable_reference)
            || self.mutable_native_reference_parameter(
                context.contract,
                context.parameter_index,
                context.contract.parameters[context.parameter_index]
                    .binding_value_type()
                    .as_ref(),
            )
        {
            return NativeArgumentMode::Borrow { mutable: true };
        }
        if projected.is_some_and(|parameter| {
            parameter.generic_parameter.is_some()
                || parameter.borrowed && matches!(parameter.ty, ProjectedType::Foreign { .. })
                || matches!(
                    parameter.ty,
                    ProjectedType::BoxedInterface { .. } | ProjectedType::Reference { .. }
                )
        }) {
            return if projected.is_some_and(ProjectedParameter::borrows_argument) {
                if projected.is_some_and(|parameter| {
                    matches!(parameter.ty, ProjectedType::String | ProjectedType::Bytes)
                }) {
                    NativeArgumentMode::ForwardReference
                } else {
                    NativeArgumentMode::Borrow { mutable: false }
                }
            } else {
                NativeArgumentMode::NativeValue
            };
        }
        NativeArgumentMode::SourceValue
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
}
