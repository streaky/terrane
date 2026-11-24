//! Connect a projected operation and its checked ABI to its lexical lowering region.
mod construct;
mod inputs;
mod invocation;
mod result;
mod target;

use std::borrow::Cow;

use super::Emitter;
use crate::{
    rust_interop::projection::{ProjectedFunction, instantiate_receiver_lifetimes},
    semantics::{ObjectIdentity, ProjectedCallSpecialization, ValueType},
    syntax::{SyntaxNode, call_is_unsafe},
};

pub(super) struct InvocationSite<'a> {
    pub(super) node: &'a SyntaxNode,
    pub(super) source_callee: &'a SyntaxNode,
    pub(super) callee: &'a SyntaxNode,
    pub(super) arguments: &'a SyntaxNode,
    pub(super) source_method_application: Option<&'a SyntaxNode>,
}

pub(super) struct NativeBridgeCall<'a> {
    pub(super) operation: Option<&'a ProjectedFunction>,
    pub(super) selection: Option<&'a ProjectedCallSpecialization>,
    is_unsafe: bool,
}

impl Emitter<'_> {
    fn projected_receiver_identity(&self, receiver: &SyntaxNode) -> Option<ObjectIdentity> {
        self.class_designator(receiver)
            .map(|object| object.identity.clone())
            .or_else(|| match self.value_type(receiver)? {
                ValueType::Object(identity) => Some(identity),
                ValueType::Reference(item)
                | ValueType::MutableReference(item)
                | ValueType::SharedReference(item) => match item.value_type() {
                    ValueType::Object(identity)
                    | ValueType::InvocationScopedNative {
                        family: identity, ..
                    } => Some(identity),
                    _ => None,
                },
                ValueType::InvocationScopedNative { family, .. } => Some(family),
                _ => None,
            })
    }

    pub(super) fn lower_bridge_call(&mut self, node: &SyntaxNode, argument_first: bool) -> String {
        let [callee, _] = node.children.as_slice() else {
            return String::new();
        };
        let producer = (!argument_first)
            .then(|| self.native_regions.last()?.producer(node.span))
            .flatten();
        if let Some(binding) = producer.and_then(|p| p.staged.as_ref()) {
            return binding.clone();
        }
        let is_unsafe = call_is_unsafe(node) || call_is_unsafe(callee);
        let operation = producer
            .map(|p| p.operation)
            .or_else(|| self.projected_function_for_call(callee, is_unsafe));
        let plan = NativeBridgeCall {
            operation,
            is_unsafe,
            selection: self.unit.projected_call_specializations.get(&(
                node.span.file,
                node.span.start,
                node.span.end,
            )),
        };
        let region = if argument_first || producer.is_some() {
            None
        } else {
            operation.and_then(|op| self.plan_native_region(callee, op))
        };
        let has_region = region.is_some();
        if let Some(region) = region {
            self.native_regions.push(region);
        }
        let result = self.call_with_order(node, argument_first, &plan);
        if has_region {
            self.finish_native_region(result)
        } else {
            result
        }
    }
}

impl<'a> NativeBridgeCall<'a> {
    pub(super) fn instantiated_operation(
        &self,
        emitter: &Emitter<'_>,
        callee: &SyntaxNode,
    ) -> Option<Cow<'a, ProjectedFunction>> {
        let function = self.operation?;
        let receiver = callee
            .children
            .first()
            .and_then(|r| emitter.receiver_value_type(r));
        let Some(ValueType::InvocationScopedNative { rust_type, .. }) = receiver else {
            return Some(Cow::Borrowed(function));
        };
        let arguments = self
            .selection
            .into_iter()
            .flat_map(|s| &s.substitutions)
            .map(|(name, ty)| (name.clone(), ty.rust_type()))
            .collect();
        Some(
            instantiate_receiver_lifetimes(function, &rust_type, &arguments)
                .expect("analyzed native receiver lifetime instantiation"),
        )
    }
}
