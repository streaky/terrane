//! Complete the raw invocation inside its planned loan, panic and error boundaries.
use super::InvocationSite;
use crate::{
    lowering::prelude::*, rust_interop::projection::ProjectedFunction,
    semantics::ProjectedCallSpecialization,
};

pub(super) struct Completion<'a> {
    pub(super) contract: Option<&'a FunctionContract>,
    pub(super) projected_function: Option<&'a ProjectedFunction>,
    pub(super) specialization: Option<&'a ProjectedCallSpecialization>,
    pub(super) native_struct_view: Option<bool>,
    pub(super) macro_graph: Option<bool>,
    pub(super) callable_field: Option<super::target::CallableField>,
    pub(super) projected_interface_dispatch: bool,
    pub(super) uses_direct_native_receiver: bool,
    pub(super) producer: Option<&'a ProjectedFunction>,
    pub(super) native_owned_argument_future: bool,
}

pub(super) struct CallSetup {
    pub(super) guard: Option<String>,
    pub(super) arguments: String,
    pub(super) receiver: String,
    pub(super) inputs: String,
}
impl CallSetup {
    fn into_prefix(self) -> String {
        let mut prefix = if let Some(mut guard) = self.guard {
            guard.push_str(&self.arguments);
            guard
        } else {
            self.arguments
        };
        prefix.push_str(&self.receiver);
        prefix.push_str(&self.inputs);
        prefix
    }
}
impl Emitter<'_> {
    #[expect(
        clippy::too_many_lines,
        reason = "terminal mapping preserves nested result, guard and async completion boundaries"
    )]
    pub(super) fn complete_invocation(
        &mut self,
        site: &InvocationSite<'_>,
        completion: &Completion<'_>,
        setup: CallSetup,
        name: String,
        values: &[String],
        call: String,
    ) -> String {
        let InvocationSite { node, callee, .. } = *site;
        let Completion {
            contract,
            projected_function,
            specialization,
            native_struct_view,
            macro_graph,
            callable_field,
            projected_interface_dispatch,
            uses_direct_native_receiver,
            producer,
            native_owned_argument_future,
        } = *completion;
        let projected_error = projected_function.is_some_and(|f| f.error.is_some());
        let foreign_method = if native_struct_view == Some(false) {
            // Native field assembly already supplies its conversion Result.
            None
        } else if specialization.is_some_and(|specialization| specialization.direct_projected_call)
            || (contract.is_none() && uses_direct_native_receiver)
        {
            projected_function
        } else {
            contract.as_ref().and_then(|contract| {
                if projected_interface_dispatch
                    || !matches!(
                        callee.kind,
                        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
                    )
                {
                    return None;
                }
                projected_function
                    .filter(|method| !contract.is_static || method.enum_operation.is_some())
            })
        };
        let call = if macro_graph == Some(false) && foreign_method.is_none() {
            format!("(|| -> Result<_, crate::TerraneForeignError> {{ Ok({call}) }})()")
        } else {
            call
        };
        let enum_operation = foreign_method
            .and_then(|method| method.enum_operation.as_ref())
            .filter(|operation| !matches!(operation, crate::rust_interop::projection::ProjectedEnumOperation::SelectOptionalVariant { .. }));
        let call = enum_operation
            .and_then(|operation| {
                let [receiver, _member] = callee.children.as_slice() else {
                    return None;
                };
                let identity = self
                    .class_designator(receiver)
                    .map(|object| object.identity.clone())
                    .or_else(|| {
                        let ValueType::Object(identity) = self.value_type(receiver)? else {
                            return None;
                        };
                        Some(identity)
                    })?;
                let owner = identity.native_projection.clone().or_else(|| {
                    self.package
                        .projection
                        .foreign_rust_path(&identity.namespace, &identity.name)
                        .map(str::to_owned)
                })?;
                let owner = owner.replacen('<', "::<", 1);
                let receiver = if contract.as_ref().is_some_and(|contract| contract.is_static) {
                    None
                } else if foreign_method.is_some_and(|method| {
                    method.receiver == Some(crate::rust_interop::projection::Receiver::Move)
                }) {
                    Some(self.consuming_native_receiver_expression(receiver))
                } else {
                    Some(self.receiver_expression(receiver))
                };
                Some(projected_enum_call(
                    operation,
                    &owner,
                    receiver.as_deref(),
                    values,
                ))
            })
            .unwrap_or(call);
        let chain_role = foreign_method
            .and_then(|method| method.chain_role)
            .or_else(|| {
                if callee.kind != SyntaxKind::Name {
                    return None;
                }
                self.package
                    .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                    .and_then(|symbol| {
                        self.package
                            .projection
                            .item(&symbol.namespace, &symbol.name)
                    })
                    .and_then(|item| match &item.kind {
                        crate::rust_interop::projection::ProjectedKind::Function(function) => {
                            function.chain_role
                        }
                        _ => None,
                    })
            });
        if let Some(method) = producer {
            let call = if method.error.is_some() {
                self.wrap_foreign_method(node, callee, method, specialization, name, call)
            } else {
                call
            };
            return self.emit_native_producer(node, &setup.into_prefix(), call, method);
        }
        if matches!(
            chain_role,
            Some(
                crate::rust_interop::projection::ChainRole::Root
                    | crate::rust_interop::projection::ChainRole::Continue
            )
        ) {
            return call;
        }
        let direct_projected_call =
            specialization.is_some_and(|specialization| specialization.direct_projected_call);
        let foreign_error = native_struct_view.is_some()
            || (macro_graph == Some(false))
            || direct_projected_call
            || foreign_method.is_some()
            || (callee.kind == SyntaxKind::Name
                && self
                    .package
                    .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                    .and_then(|symbol| symbol.identity.rsplit_once("::"))
                    .and_then(|(namespace, name)| self.package.projection.item(namespace, name))
                    .is_some_and(|item| {
                        matches!(
                            &item.kind,
                            crate::rust_interop::projection::ProjectedKind::Function(_)
                        )
                    }));
        let call = if let Some(method) = foreign_method {
            self.wrap_foreign_method(node, callee, method, specialization, name, call)
        } else {
            call
        };
        let call = if foreign_method.is_none()
            && (contract.as_ref().is_some_and(|contract| contract.is_unsafe)
                || specialization
                    .is_some_and(|specialization| specialization.direct_projected_call)
                    && projected_function.is_some_and(|function| function.is_unsafe))
        {
            format!("unsafe {{ {call} }}")
        } else {
            call
        };
        let function_value_call = (contract.is_none() || callable_field.is_some())
            && matches!(
                self.value_type(callee),
                Some(ValueType::Function(_, _, effects)) if effects.requires_throwing_abi()
            )
            && (callee.kind == SyntaxKind::Name || callable_field.is_some());
        let dependency_contract = contract.as_ref().is_some_and(|contract| {
            !projected_interface_dispatch
                && self.package.units.iter().any(|unit| {
                    unit.source.id() == contract.span.file && unit.namespace.starts_with("/deps/")
                })
        });
        let needs_error_mapping = contract.as_ref().is_some_and(|contract| contract.throws)
            || projected_error
            || dependency_contract
            || foreign_error
            || function_value_call;
        let site = if needs_error_mapping {
            self.error_site(node)
        } else {
            String::new()
        };
        let dependency_boundary = dependency_contract
            || self
                .package
                .resolve_name_at(self.unit, callee.span.start, self.text(callee))
                .is_some_and(|symbol| symbol.identity.starts_with("/deps/"));
        let map_errors = |call: String| {
            if !needs_error_mapping {
                return call;
            }
            if foreign_error || dependency_boundary {
                self.fallible_at(&call, &site)
            } else if self.try_completion {
                format!("__terrane_traced_completion!({call}, {site})")
            } else if self.propagate_errors {
                format!("__terrane_traced_err({call}, {site})?")
            } else {
                format!("__terrane_traced({call}, {site})")
            }
        };
        let result = if contract.as_ref().is_some_and(|contract| contract.is_async)
            && matches!(self.value_type(node), Some(ValueType::Task(_, _)))
        {
            if foreign_error || dependency_boundary {
                let completed = format!("__terrane_raised_err(__terrane_future.await, {site})");
                let completed = if foreign_method.is_none()
                    && let Some(specialization) = specialization
                {
                    projected_result_expression(&completed, &specialization.projected_result)
                } else {
                    completed
                };
                if native_owned_argument_future {
                    format!("async move {{ let __terrane_future = {call}; {completed} }}")
                } else {
                    format!("{{ let __terrane_future = {call}; async move {{ {completed} }} }}")
                }
            } else {
                call
            }
        } else {
            let mapped = map_errors(call);
            if foreign_method.is_none()
                && let Some(specialization) = specialization
            {
                projected_result_expression(&mapped, &specialization.projected_result)
            } else {
                mapped
            }
        };
        let guarded = setup.guard.is_some();
        let mut prefix = setup.into_prefix();
        if guarded {
            write!(prefix, "let __terrane_call_result = {result}; ").expect("String output");
            Self::render_native_region(&prefix, "__terrane_call_result".to_owned())
        } else {
            Self::render_native_region(&prefix, result)
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "the native failure ABI and panic boundary wrap the same invocation"
    )]
    fn wrap_foreign_method(
        &mut self,
        node: &SyntaxNode,
        callee: &SyntaxNode,
        method: &ProjectedFunction,
        specialization: Option<&ProjectedCallSpecialization>,
        name: String,
        call: String,
    ) -> String {
        let callable_head = callee.type_designator_head().unwrap_or(callee);
        let (dependency, member) = if callable_head.kind == SyntaxKind::Name {
            let member = method.native_path.clone().unwrap_or_else(|| {
                method
                    .native_owner
                    .as_deref()
                    .map_or_else(|| name, |owner| format!("{owner}::{}", method.name))
            });
            let (crate_name, member) = crate::rust_ir::rust_callable_origin(&member)
                .unwrap_or_else(|| ("dependency".to_owned(), member));
            let dependency = self
                .package
                .resolve_name_at(
                    self.unit,
                    callable_head.span.start,
                    self.text(callable_head),
                )
                .and_then(|symbol| {
                    self.package
                        .projection
                        .dependency_for_namespace(&symbol.namespace)
                        .map(|dependency| dependency.package.as_str())
                })
                .or_else(|| self.package.projection.package_for_rust_crate(&crate_name))
                .map_or(crate_name, str::to_owned);
            (dependency, member)
        } else {
            let native_path = method.native_path.clone().unwrap_or_else(|| {
                let owner = method
                    .native_owner
                    .clone()
                    .or_else(|| {
                        self.projected_receiver_identity(&callable_head.children[0])
                            .and_then(|identity| {
                                self.package
                                    .projection
                                    .foreign_rust_path(&identity.namespace, &identity.name)
                                    .map(str::to_owned)
                            })
                    })
                    .unwrap_or_else(|| "dependency::ForeignType".to_owned());
                format!("{owner}::{}", method.name)
            });
            let (crate_name, member) = crate::rust_ir::rust_callable_origin(&native_path)
                .unwrap_or_else(|| {
                    (
                        native_path
                            .split("::")
                            .next()
                            .unwrap_or("dependency")
                            .trim()
                            .to_owned(),
                        native_path,
                    )
                });
            let dependency = self
                .package
                .projection
                .package_for_rust_crate(&crate_name)
                .map_or(crate_name, str::to_owned);
            (dependency, member)
        };
        let call = if method.is_unsafe {
            format!("unsafe {{ {call} }}")
        } else {
            call
        };
        let invocation = if method.into_future {
            "std::future::IntoFuture::into_future(__terrane_call).await".to_owned()
        } else if method.is_async {
            "__terrane_call.await".to_owned()
        } else {
            call.clone()
        };
        let discarded_unit = !method.is_async
            && !method.into_future
            && method.error.is_none()
            && self.discarded_call == Some(node.span);
        let unwind_call = if discarded_unit {
            format!("{{ let _ = {call}; }}")
        } else {
            call.clone()
        };
        let caught = if method.into_future {
            "crate::__terrane_dependency_await_unwind(std::future::IntoFuture::into_future(__terrane_call)).await".to_owned()
        } else if method.is_async {
            "crate::__terrane_dependency_await_unwind(__terrane_call).await".to_owned()
        } else {
            format!(
                "std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<_, crate::TerraneForeignError> {{ Ok({unwind_call}) }}))"
            )
        };
        let projected_result = specialization.map_or(&method.result, |specialization| {
            &specialization.projected_result
        });
        let converted = if discarded_unit {
            "()".to_owned()
        } else if self.borrowed_native_leaf_exit == Some(node.span)
            && matches!(
                projected_result,
                crate::rust_interop::projection::ProjectedType::BorrowedBytes
                    | crate::rust_interop::projection::ProjectedType::BorrowedString
            )
        {
            "value".to_owned()
        } else {
            projected_result_expression("value", projected_result)
        };
        let nested_converted = match projected_result {
            crate::rust_interop::projection::ProjectedType::Optional(inner)
                if method.error_optional_depth == 1 =>
            {
                projected_result_expression("value", inner)
            }
            _ => converted.clone(),
        };
        let invocation = if self.package.profile.panic == crate::package::PanicProfile::Abort {
            invocation
        } else if method.is_async || method.into_future {
            format!(
                "{caught}.map_err(|payload| crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?}))?"
            )
        } else {
            format!(
                "match {caught} {{ Ok(result) => result?, Err(payload) => return Err(crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?})) }}"
            )
        };
        let projected_error_identity = method.error.as_deref().and_then(|rust_path| {
            self.package
                .projection
                .projected_identity_for_rust_path(rust_path)
                .map(|(namespace, name)| (format!("{namespace}::{name}"), name.to_owned()))
        });
        let (error_kind, error_message) = projected_error_identity.map_or_else(
                || {
                    (
                        "TERRANE_DEPENDENCY_ERROR".to_owned(),
                        format!(
                            "format!(\"Rust dependency `{{}}` member `{{}}` failed: {{error}}\", {dependency:?}, {member:?})"
                        ),
                    )
                },
                |(identity, name)| {
                    let descriptor = self.registry.register_descriptor(&identity, &name);
                    (
                        format!("DescriptorId({descriptor})"),
                        "error.to_string()".to_owned(),
                    )
                },
            );
        let mapped = if method.error_optional_depth == 1 {
            format!(
                "match {invocation} {{ None => Ok(None), Some(Ok(value)) => Ok(Some({nested_converted})), Some(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, {error_message}, crate::TERRANE_NO_SITE))) }}"
            )
        } else if method.error.is_some() {
            format!(
                "match {invocation} {{ Ok(value) => Ok({converted}), Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, {error_message}, crate::TERRANE_NO_SITE))) }}"
            )
        } else if discarded_unit {
            format!("{{ let _ = {invocation}; Ok::<_, crate::TerraneForeignError>(()) }}")
        } else {
            format!(
                "{{ let value = {invocation}; Ok::<_, crate::TerraneForeignError>({converted}) }}"
            )
        };
        if method.is_async || method.into_future {
            // Keep future construction in the caller's loan region.
            if method.is_async {
                format!("{{ let __terrane_call = {call}; Box::pin(async move {{ {mapped} }}) }}")
            } else {
                format!("{{ let __terrane_call = {call}; async {{ {mapped} }}.await }}")
            }
        } else {
            format!("(|| -> Result<_, crate::TerraneForeignError> {{ {mapped} }})()")
        }
    }
}
#[expect(
    clippy::too_many_lines,
    reason = "enum construction and extraction keep their mirrored field mapping visible together"
)]
fn projected_enum_call(
    operation: &crate::rust_interop::projection::ProjectedEnumOperation,
    owner: &str,
    receiver: Option<&str>,
    values: &[String],
) -> String {
    match operation {
        crate::rust_interop::projection::ProjectedEnumOperation::SelectOptionalVariant {
            ..
        } => {
            unreachable!("optional variant selection has a scoped receiver")
        }
        crate::rust_interop::projection::ProjectedEnumOperation::Construct {
            variant,
            unit,
            conversion,
            payload,
            ..
        } => {
            if *unit {
                return format!("{owner}::{variant}");
            }
            if let Some(payload) = payload {
                let bindings = (0..payload.fields.len())
                    .map(|index| format!("field_{index}"))
                    .collect::<Vec<_>>();
                let body = match payload.style {
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Tuple => {
                        format!("{owner}::{variant}({})", bindings.join(", "))
                    }
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Struct => {
                        let fields = payload
                            .fields
                            .iter()
                            .zip(&bindings)
                            .map(|(field, binding)| format!("r#{field}: {binding}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{owner}::{variant} {{ {fields} }}")
                    }
                };
                return format!(
                    "{{ let ({},) = ({}).terrane_into_fields(); {body} }}",
                    bindings.join(", "),
                    values[0]
                );
            }
            let arguments = values
                .iter()
                .map(|value| match conversion {
                    crate::rust_interop::projection::ProjectedEnumPayloadConversion::Into => {
                        format!("({value}).into()")
                    }
                    _ => value.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{owner}::{variant}({arguments})")
        }
        crate::rust_interop::projection::ProjectedEnumOperation::VariantName {
            variants,
            exhaustive,
        } => {
            let receiver = receiver.expect("enum inspection has a receiver");
            let mut arms = variants
                .iter()
                .map(|variant| format!("{owner}::{variant} {{ .. }} => {variant:?}.to_owned()"))
                .collect::<Vec<_>>();
            if !exhaustive {
                arms.push("_ => \"unknown\".to_owned()".to_owned());
            }
            format!("match &{receiver} {{ {} }}", arms.join(", "))
        }
        crate::rust_interop::projection::ProjectedEnumOperation::Extract {
            variant,
            conversion,
            payload_rust_type,
            payload,
        } => {
            let receiver = receiver.expect("enum extraction has a receiver");
            if let Some(payload) = payload {
                let bindings = (0..payload.fields.len())
                    .map(|index| format!("field_{index}"))
                    .collect::<Vec<_>>();
                let pattern = match payload.style {
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Tuple => {
                        format!("{owner}::{variant}({})", bindings.join(", "))
                    }
                    crate::rust_interop::projection::ProjectedEnumPayloadStyle::Struct => {
                        let fields = payload
                            .fields
                            .iter()
                            .zip(&bindings)
                            .map(|(field, binding)| format!("r#{field}: {binding}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{owner}::{variant} {{ {fields} }}")
                    }
                };
                let constructor = enum_payload_constructor_name(payload_rust_type);
                return format!(
                    "match {receiver} {{ {pattern} => Some({constructor}({})), _ => None }}",
                    bindings.join(", ")
                );
            }
            let value = match conversion {
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::Identity
                | crate::rust_interop::projection::ProjectedEnumPayloadConversion::Into => {
                    "value".to_owned()
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::AsRefString => {
                    format!("<{payload_rust_type} as AsRef<str>>::as_ref(&value).to_owned()")
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::AsRefBytes => {
                    format!("<{payload_rust_type} as AsRef<[u8]>>::as_ref(&value).to_vec()")
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::DerefString => {
                    format!("<{payload_rust_type} as std::ops::Deref>::deref(&value).to_owned()")
                }
                crate::rust_interop::projection::ProjectedEnumPayloadConversion::DerefBytes => {
                    format!("<{payload_rust_type} as std::ops::Deref>::deref(&value).to_vec()")
                }
            };
            format!("match {receiver} {{ {owner}::{variant}(value) => Some({value}), _ => None }}")
        }
    }
}
