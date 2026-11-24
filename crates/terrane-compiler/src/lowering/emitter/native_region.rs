//! Stage native receiver loans in the lexical scope of their owned exit.
use std::fmt::Write as _;

use super::Emitter;
use crate::{
    Span,
    rust_interop::projection::{ChainRole, ProjectedFunction, Receiver},
    semantics::{ValueType, ungrouped_expression},
    syntax::{SyntaxKind, SyntaxNode, call_is_unsafe},
};

pub(super) struct NativeRegion<'a> {
    producers: Vec<Producer<'a>>,
    prelude: String,
    stage_views: bool,
}

pub(super) struct Producer<'a> {
    pub(super) operation: &'a ProjectedFunction,
    site: Span,
    mutable: bool,
    pub(super) staged: Option<String>,
}

impl<'a> NativeRegion<'a> {
    pub(super) fn producer(&self, site: Span) -> Option<&Producer<'a>> {
        self.producers.iter().find(|producer| producer.site == site)
    }
}

impl<'a> Emitter<'a> {
    pub(super) fn plan_native_region(
        &self,
        head: &SyntaxNode,
        mut operation: &'a ProjectedFunction,
    ) -> Option<NativeRegion<'a>> {
        let mut receiver = head.type_designator_head()?;
        if receiver.kind != SyntaxKind::MemberExpression
            || ungrouped_expression(receiver.children.first()?).kind != SyntaxKind::CallExpression
        {
            return None;
        }
        if operation.is_async
            || operation.into_future
            || matches!(
                operation.chain_role,
                Some(ChainRole::Root | ChainRole::Continue)
            )
        {
            return None;
        }
        let mut producers = Vec::new();
        let mut has_local_loan = false;
        while receiver.kind == SyntaxKind::MemberExpression {
            let lender = ungrouped_expression(receiver.children.first()?);
            if lender.kind != SyntaxKind::CallExpression {
                break;
            }
            let receiver_head = lender.children.first()?;
            let Some(producer) =
                self.projected_function_for_call(receiver_head, call_is_unsafe(lender))
            else {
                break;
            };
            let chain_producer = matches!(
                producer.chain_role,
                Some(ChainRole::Root | ChainRole::Continue)
            );
            let scoped_receiver = self.value_type(lender);
            let lends_scoped_receiver = matches!(
                operation.receiver,
                Some(Receiver::Borrow | Receiver::MutableBorrow)
            ) && matches!(
                scoped_receiver,
                Some(ValueType::InvocationScopedNative { .. })
            );
            if !chain_producer && !lends_scoped_receiver {
                break;
            }
            has_local_loan |= matches!(
                scoped_receiver,
                Some(ValueType::InvocationScopedNative {
                    expression_scoped: false,
                    ..
                })
            );
            producers.push(Producer {
                operation: producer,
                site: lender.span,
                mutable: operation.receiver == Some(Receiver::MutableBorrow),
                staged: None,
            });
            operation = producer;
            receiver = receiver_head.type_designator_head()?;
        }
        // A local loan chained through another producer can need a lexical lender
        // even without a guard. Already expression-scoped chains keep their joint
        // inference and panic boundary, including calls that mutably borrow their owner.
        let stage_views = has_local_loan
            && producers.len() > 1
            && producers.iter().any(|producer| producer.mutable);
        (!producers.is_empty()).then_some(NativeRegion {
            producers,
            prelude: String::new(),
            stage_views,
        })
    }

    pub(super) fn native_producer(&self, site: Span) -> bool {
        self.native_regions
            .last()
            .and_then(|r| r.producer(site))
            .is_some()
    }

    pub(super) fn emit_native_producer(
        &mut self,
        node: &SyntaxNode,
        setup: &str,
        call: String,
        method: &ProjectedFunction,
    ) -> String {
        let site = node.span;
        let call = if method.is_unsafe {
            format!("unsafe {{ {call} }}")
        } else {
            call
        };
        let region = self.native_regions.last().expect("planned native region");
        // Nested expressions retain joint Rust inference until setup needs a lexical home.
        if !region.stage_views && setup.is_empty() && region.prelude.is_empty() {
            return call;
        }
        let call = if self.package.profile.panic == crate::package::PanicProfile::Abort {
            call
        } else {
            let origin = method.native_path.clone().unwrap_or_else(|| {
                method.native_owner.as_ref().map_or_else(
                    || method.name.clone(),
                    |owner| format!("{owner}::{}", method.name),
                )
            });
            let (dependency, member) = crate::rust_ir::rust_callable_origin(&origin)
                .unwrap_or_else(|| ("dependency".to_owned(), origin));
            let dependency = self
                .package
                .projection
                .package_for_rust_crate(&dependency)
                .unwrap_or(&dependency);
            let caught = format!(
                "std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {call})).map_err(|payload| crate::__terrane_dependency_panic(payload, {dependency:?}, {member:?}))"
            );
            self.fallible(caught, node)
        };
        let region = self
            .native_regions
            .last_mut()
            .expect("planned native region");
        let producer = region
            .producers
            .iter_mut()
            .find(|producer| producer.site == site)
            .expect("planned producer");
        let name = format!("__terrane_native_view_{}", site.start);
        let mutable = if producer.mutable { "mut " } else { "" };
        region.prelude.push_str(setup);
        write!(region.prelude, "let {mutable}{name} = {call}; ").expect("String output");
        producer.staged = Some(name.clone());
        name
    }

    pub(super) fn finish_native_region(&mut self, result: String) -> String {
        let region = self.native_regions.pop().expect("planned native region");
        Self::render_native_region(&region.prelude, result)
    }

    pub(super) fn render_native_region(prelude: &str, result: String) -> String {
        if prelude.is_empty() {
            result
        } else {
            format!("{{ {prelude}{result} }}")
        }
    }

    /// Select the native place before an output boundary can force an owned read.
    pub(super) fn native_receiver_loan(
        &mut self,
        receiver: &SyntaxNode,
        mutable: bool,
        staged: bool,
    ) -> (String, String) {
        use super::members::GuardScope;

        let receiver = ungrouped_expression(receiver);
        let borrow = if mutable { "&mut " } else { "&" };
        if !staged && let Some(place) = self.narrowed_storage_name(receiver, false, mutable) {
            return (String::new(), place);
        }
        if staged
            && receiver.kind == SyntaxKind::MemberExpression
            && let [holder, field] = receiver.children.as_slice()
            && let Some((setup, place, borrowed)) = self.projected_field_place_parts(
                receiver,
                holder,
                field,
                mutable,
                GuardScope::Place,
            )
        {
            return if borrowed {
                (setup, place)
            } else {
                (setup, format!("{borrow}({place})"))
            };
        }
        let place = self.native_receiver_expression(receiver, mutable);
        (String::new(), format!("{borrow}({place})"))
    }

    pub(super) fn projected_receiver_guard(
        &mut self,
        receiver: &SyntaxNode,
        mutable: bool,
    ) -> (String, String) {
        let (mut setup, place) = if mutable {
            self.live_receiver_parts(receiver)
        } else {
            self.borrowed_receiver_parts(receiver)
        };
        let guard = self.fresh_read_binding_name("native_guard");
        let qualifier = if mutable { "mut " } else { "" };
        write!(setup, "let {qualifier}{guard} = {place}; ").expect("String output");
        (setup, guard)
    }
}
