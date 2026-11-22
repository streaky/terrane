//! File-local mutation candidates, reconciled against the current semantic snapshot.
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    bindings::{callback_contract, projected_function_for_call},
    diagnostics::{member_invocation_mode_at, node_text, typed_member_call, unary_operator_text},
    expressions::infer_value_type,
    member_inference::object_field_type,
    model::{CallableParameterType, SemanticPackage, SemanticUnit, ValueType},
    objects::{application_is_resource_owning, effective_object_fields, value_type_is_cloneable},
    types::ungrouped_expression,
};
use crate::{
    InvocationMode, Span,
    syntax::{SyntaxKind, SyntaxNode},
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum ClosureWrites {
    Include,
    Exclude,
}

#[derive(Clone, Debug, Default)]
pub(super) struct MutationState {
    pub(super) normalized: bool,
    sites: MutationSites,
    index: Option<MutationIndex>,
}

/// Paths are relative to one syntax tree, independent of composition-local file IDs.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct MutationSites(Vec<Site>);

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Site {
    path: Box<[usize]>,
    inside_closure: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    rust_borrows: Vec<String>,
}

fn candidate(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::Assignment
            | SyntaxKind::PostfixExpression
            | SyntaxKind::CallExpression
            | SyntaxKind::ForStatement
            | SyntaxKind::UnaryExpression
            | SyntaxKind::PipelineStage
            | SyntaxKind::RustBlock
            | SyntaxKind::UnsafeRustBlock
    )
}

fn rust_block(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::RustBlock | SyntaxKind::UnsafeRustBlock
    )
}

impl MutationSites {
    fn collect(unit: &SemanticUnit) -> Self {
        fn visit(
            unit: &SemanticUnit,
            node: &SyntaxNode,
            path: &mut Vec<usize>,
            inside_closure: bool,
            sites: &mut Vec<Site>,
        ) {
            let inside_closure = inside_closure || node.kind == SyntaxKind::AnonymousFunction;
            if candidate(node) {
                let rust_borrows = if rust_block(node) {
                    crate::rust_ir::rust_mutably_borrowed_value_identifiers(node_text(
                        &unit.source,
                        node,
                    ))
                    .into_iter()
                    .collect()
                } else {
                    Vec::new()
                };
                if !rust_block(node) || !rust_borrows.is_empty() {
                    sites.push(Site {
                        path: path.as_slice().into(),
                        inside_closure,
                        rust_borrows,
                    });
                }
            }
            if !rust_block(node) {
                for (index, child) in node.children.iter().enumerate() {
                    path.push(index);
                    visit(unit, child, path, inside_closure, sites);
                    path.pop();
                }
            }
        }
        let mut sites = Vec::new();
        visit(unit, &unit.tree.root, &mut Vec::new(), false, &mut sites);
        Self(sites)
    }

    fn load_or_collect(root: &std::path::Path, unit: &SemanticUnit) -> Self {
        if !crate::cache_scope::enabled() {
            return Self::collect(unit);
        }
        let mut slot = crate::cache_io::HashWriter::new();
        slot.part(unit.source.path().as_os_str().as_encoded_bytes());
        let path = root
            .join(".trn/cache/mutation-sites")
            .join(slot.finish_hex());
        let mut key = crate::cache_io::HashWriter::new();
        key.part(unit.source.text().as_bytes());
        if unit.mutations.normalized {
            // Some syntax rewrites depend on other files' declarations. For those
            // files, also verify the shape and the byte offsets the paths address.
            fn shape(node: &SyntaxNode, key: &mut crate::cache_io::HashWriter) {
                key.part(&(node.kind as u32).to_le_bytes());
                key.part(&node.span.start.to_le_bytes());
                key.part(&node.span.end.to_le_bytes());
                key.part(&node.children.len().to_le_bytes());
                for child in &node.children {
                    shape(child, key);
                }
            }
            shape(&unit.tree.root, &mut key);
        }
        let key = key.finish_hex();
        if let Some(cached) = crate::compiler::cache::load_file(&path, &key) {
            crate::compilation_progress::note(
                "reusing file mutation sites",
                unit.source.path().display(),
            );
            return cached;
        }
        let sites = Self::collect(unit);
        crate::compiler::cache::store_file(&path, &key, &sites);
        crate::compilation_progress::note(
            "collected file mutation sites",
            unit.source.path().display(),
        );
        sites
    }
}

pub(super) fn prepare_sites(package: &mut SemanticPackage) {
    let phase =
        crate::compilation_progress::start("preparing file mutation sites", package.root.display());
    for unit in &mut package.units {
        unit.mutations.sites = MutationSites::load_or_collect(&package.root, unit);
    }
    phase.finish();
}

#[derive(Clone, Debug, Default)]
pub(super) struct MutationIndex(HashMap<Span, MutationCounts>);

#[derive(Clone, Copy, Debug, Default)]
struct MutationCounts {
    all: [usize; 2],
    interior: u8,
}

impl MutationIndex {
    pub(super) fn resolve(package: &SemanticPackage, unit: &SemanticUnit) -> Self {
        let mut index = Self::default();
        let mut writes = SiteWrites::default();
        for site in &unit.mutations.sites.0 {
            let node = site
                .path
                .iter()
                .fold(&unit.tree.root, |node, &index| &node.children[index]);
            writes.resolve(package, unit, node, &site.rust_borrows);
            for (span, flags) in writes.0.drain(..) {
                let counts = index.0.entry(span).or_default();
                counts.all[0] += 1;
                counts.interior |= flags & INTERIOR;
                if !site.inside_closure && flags & OUTSIDE_CLOSURES != 0 {
                    counts.all[1] += 1;
                    counts.interior |= flags & INTERIOR_OUTSIDE_CLOSURES;
                }
            }
        }
        index
    }

    pub(super) fn is_mutated(
        &self,
        span: Span,
        initially_assigned: bool,
        closure_writes: ClosureWrites,
    ) -> bool {
        self.0.get(&span).is_some_and(|counts| {
            counts.all[usize::from(closure_writes == ClosureWrites::Exclude)]
                > usize::from(!initially_assigned)
        })
    }
}

pub(super) fn finalize(package: &mut SemanticPackage) {
    let phase =
        crate::compilation_progress::start("indexing final mutation facts", package.root.display());
    let indexes: Vec<_> = package
        .units
        .iter()
        .map(|unit| MutationIndex::resolve(package, unit))
        .collect();
    for (unit, index) in package.units.iter_mut().zip(indexes) {
        unit.mutations.sites = MutationSites::default();
        unit.mutations.index = Some(index);
    }
    phase.finish();
}

pub(crate) fn binding_span_is_mutated(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    initially_assigned: bool,
    closure_writes: ClosureWrites,
) -> bool {
    if let Some(index) = &unit.mutations.index {
        return index.is_mutated(declaration_span, initially_assigned, closure_writes);
    }
    binding_span_is_mutated_in(
        package,
        unit,
        &unit.tree.root,
        declaration_span,
        initially_assigned,
        closure_writes,
    )
}

pub(crate) fn binding_span_is_mutated_in(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    scope: &SyntaxNode,
    declaration_span: Span,
    initially_assigned: bool,
    closure_writes: ClosureWrites,
) -> bool {
    let mut count = 0;
    subtree_mutates(
        package,
        unit,
        scope,
        closure_writes,
        &mut SiteWrites::default(),
        &mut |writes| {
            count += usize::from(writes.contains(
                declaration_span,
                if closure_writes == ClosureWrites::Include {
                    0
                } else {
                    OUTSIDE_CLOSURES
                },
            ));
            count > usize::from(!initially_assigned)
        },
    )
}

pub(super) fn binding_span_has_interior_mutation(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    closure_writes: ClosureWrites,
) -> bool {
    let mask = if closure_writes == ClosureWrites::Include {
        INTERIOR
    } else {
        INTERIOR_OUTSIDE_CLOSURES
    };
    if let Some(index) = &unit.mutations.index {
        return index
            .0
            .get(&declaration_span)
            .is_some_and(|counts| counts.interior & mask != 0);
    }
    subtree_mutates(
        package,
        unit,
        &unit.tree.root,
        closure_writes,
        &mut SiteWrites::default(),
        &mut |writes| writes.contains(declaration_span, mask),
    )
}

// A scoped caller can provide a transformed subtree. Use the same classifier,
// but respect the actual subtree and its relative closure boundaries.
fn subtree_mutates(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    closure_writes: ClosureWrites,
    writes: &mut SiteWrites,
    selected: &mut impl FnMut(&SiteWrites) -> bool,
) -> bool {
    if closure_writes == ClosureWrites::Exclude && node.kind == SyntaxKind::AnonymousFunction {
        return false;
    }
    if candidate(node) {
        writes.0.clear();
        if rust_block(node) {
            for name in crate::rust_ir::rust_mutably_borrowed_value_identifiers(node_text(
                &unit.source,
                node,
            )) {
                writes.rust_borrow(package, unit, node, &name);
            }
            return selected(writes);
        }
        writes.resolve(package, unit, node, &[]);
        if selected(writes) {
            return true;
        }
    }
    node.children
        .iter()
        .any(|child| subtree_mutates(package, unit, child, closure_writes, writes, selected))
}

const OUTSIDE_CLOSURES: u8 = 1;
const INTERIOR: u8 = 2;
const INTERIOR_OUTSIDE_CLOSURES: u8 = 4;

/// Count one write per binding per syntax node, even if several operands or
/// classifiers select that binding. Interior/excluded queries are independent.
#[derive(Default)]
struct SiteWrites(Vec<(Span, u8)>);

impl SiteWrites {
    fn add(&mut self, span: Span, interior: bool, outside_closures: bool) {
        let mut flags = 0;
        if interior {
            flags |= INTERIOR;
        }
        if outside_closures {
            flags |= OUTSIDE_CLOSURES;
        }
        if interior && outside_closures {
            flags |= INTERIOR_OUTSIDE_CLOSURES;
        }
        if let Some((_, current)) = self.0.iter_mut().find(|(binding, _)| *binding == span) {
            *current |= flags;
        } else {
            self.0.push((span, flags));
        }
    }

    fn contains(&self, span: Span, mask: u8) -> bool {
        self.0
            .iter()
            .any(|&(binding, flags)| binding == span && (mask == 0 || flags & mask != 0))
    }

    fn rust_borrow(
        &mut self,
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        name: &str,
    ) {
        if !package.is_lexical_replacement(unit, node.span, name)
            && let Some(binding) = package
                .resolve_name_at(unit, node.span.start, name)
                .and_then(|symbol| symbol.declaration_span)
        {
            self.add(binding, true, true);
        }
    }

    fn resolve(
        &mut self,
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        rust_borrows: &[String],
    ) {
        let root_binding = |target: &SyntaxNode| {
            let root = object_mutation_root(unit, target)?;
            let name = node_text(&unit.source, root);
            if package.is_lexical_replacement(unit, node.span, name) {
                return None;
            }
            package
                .resolve_name_at(unit, root.span.start, name)?
                .declaration_span
        };
        match node.kind {
            SyntaxKind::RustBlock | SyntaxKind::UnsafeRustBlock => {
                for name in rust_borrows {
                    self.rust_borrow(package, unit, node, name);
                }
            }
            SyntaxKind::Assignment | SyntaxKind::PostfixExpression => {
                if let Some(target) = node.children.first().map(ungrouped_expression) {
                    let interior = node.kind == SyntaxKind::PostfixExpression
                        || target.kind != SyntaxKind::Name;
                    let receiver = target.children.first().filter(|_| {
                        matches!(
                            target.kind,
                            SyntaxKind::IndexExpression | SyntaxKind::MemberExpression
                        )
                    });
                    for binding in [root_binding(target), receiver.and_then(root_binding)]
                        .into_iter()
                        .flatten()
                    {
                        if node.span != binding {
                            self.add(binding, interior, true);
                        }
                    }
                }
            }
            SyntaxKind::CallExpression => {
                self.call_arguments(package, unit, node);
                if let Some(callee) = node.children.first() {
                    self.call_receiver(package, unit, callee, &root_binding);
                    self.borrowed_callable(unit, callee, &root_binding);
                }
            }
            SyntaxKind::ForStatement => {
                if let Some(binding) = node.children.get(1).and_then(root_binding)
                    && unit.typed_bindings.iter().any(|typed| {
                        typed.span == binding && matches!(typed.value_type, ValueType::Iterator(_))
                    })
                {
                    self.add(binding, true, true);
                }
            }
            SyntaxKind::UnaryExpression => {
                if let Some(target) = node.children.last() {
                    match unary_operator_text(unit, node).as_deref() {
                        Some("mutable ref") => {
                            if let Some(binding) = root_binding(target) {
                                self.add(binding, true, true);
                            }
                        }
                        Some("move") if target.kind == SyntaxKind::MemberExpression => {
                            if let Some(binding) = target.children.first().and_then(root_binding)
                                && explicit_field_extraction(package, unit, target)
                            {
                                self.add(binding, true, true);
                            }
                        }
                        _ => {}
                    }
                }
            }
            SyntaxKind::PipelineStage => {
                if node.children.first().is_some_and(|operation| {
                    matches!(node_text(&unit.source, operation), "filter" | "map")
                }) && let Some(callee) = node
                    .children
                    .get(1)
                    .and_then(|arguments| arguments.children.first())
                {
                    self.borrowed_callable(unit, callee, &root_binding);
                }
            }
            _ => {}
        }
    }

    fn borrowed_callable(
        &mut self,
        unit: &SemanticUnit,
        callee: &SyntaxNode,
        binding: &impl Fn(&SyntaxNode) -> Option<Span>,
    ) {
        if let Some(binding) = binding(callee)
            && matches!(unit.inferred_value_type(callee),
                Some(ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects))
                if effects.borrowed_environment && effects.modes.written == InvocationMode::Mutable)
        {
            self.add(binding, true, true);
        }
    }

    fn call_receiver(
        &mut self,
        package: &SemanticPackage,
        unit: &SemanticUnit,
        callee: &SyntaxNode,
        binding: &impl Fn(&SyntaxNode) -> Option<Span>,
    ) {
        let Some((receiver, receiver_type, member)) = typed_member_call(unit, callee) else {
            return;
        };
        let family = member
            .split_once('.')
            .map_or(member.as_str(), |(family, _)| family);
        if let Some(binding) = binding(receiver)
            && member_invocation_mode_at(package, unit, receiver, &receiver_type, family)
                == InvocationMode::Mutable
        {
            let callable_field = matches!(&receiver_type, ValueType::Object(identity)
                if matches!(object_field_type(unit, identity, family, false), Some(ValueType::Function(..) | ValueType::AsyncFunction(..))));
            self.add(binding, true, !callable_field);
        }
        let place = ungrouped_expression(receiver);
        if place.kind == SyntaxKind::MemberExpression
            && let [owner, field] = place.children.as_slice()
            && let Some(binding) = binding(owner)
            && member_invocation_mode_at(package, unit, receiver, &receiver_type, &member)
                == InvocationMode::Consuming
            && super::field_moves::field_access_transfers(package, unit, place, true)
                .unwrap_or(false)
            && let Ok(Some(ValueType::Object(identity))) =
                infer_value_type(unit, owner, &unit.typed_bindings)
            && package
                .find_nominal_descriptor(&identity, |_| true)
                .is_some_and(|descriptor| {
                    effective_object_fields(package, descriptor)
                        .iter()
                        .any(|candidate| {
                            candidate.field.name == node_text(&unit.source, field)
                                && candidate.field.required
                        })
                })
        {
            self.add(binding, true, true);
        }
    }

    fn call_arguments(
        &mut self,
        package: &SemanticPackage,
        unit: &SemanticUnit,
        call: &SyntaxNode,
    ) {
        let [callee, arguments] = call.children.as_slice() else {
            return;
        };
        let mut record = |argument: &SyntaxNode| {
            if let Some(root) = argument_root(argument)
                && let Some(binding) = package
                    .resolve_name_at(unit, root.span.start, node_text(&unit.source, root))
                    .and_then(|symbol| symbol.declaration_span)
            {
                self.add(binding, true, true);
            }
        };
        let projected = unit
            .projected_call_specializations
            .get(&(call.span.file, call.span.start, call.span.end))
            .map(|specialization| specialization.projected_parameters.as_slice())
            .or_else(|| {
                projected_function_for_call(
                    package,
                    unit,
                    callee,
                    crate::syntax::call_is_unsafe(call),
                )
                .map(|function| function.parameters.as_slice())
            });
        if let Some(parameters) = projected {
            mutable_arguments(
                unit,
                arguments,
                parameters,
                |parameter| &parameter.name,
                crate::rust_interop::projection::ProjectedParameter::requires_mutable_reference,
                &mut record,
            );
        }
        if let Some(contract) = callback_contract(package, unit, callee) {
            let declaration = contract.span;
            let contract = package
                .units
                .iter()
                .filter(|owner| owner.source.id() == declaration.file)
                .flat_map(|unit| &unit.functions)
                .find(|function| function.span == declaration)
                .unwrap_or(contract);
            mutable_arguments(
                unit,
                arguments,
                &contract.parameters,
                |parameter| &parameter.name,
                |parameter| {
                    matches!(
                        parameter.binding_value_type(),
                        Some(ValueType::MutableReference(_))
                    )
                },
                &mut record,
            );
        } else if let Some(
            ValueType::Function(parameters, _, _) | ValueType::AsyncFunction(parameters, _, _, _),
        ) = unit.inferred_value_type(callee)
        {
            for (index, argument) in arguments.children.iter().enumerate() {
                if parameters
                    .get(index)
                    .or_else(|| {
                        parameters
                            .last()
                            .filter(|parameter| parameter.is_variadic())
                    })
                    .is_some_and(CallableParameterType::requires_mutable_reference)
                {
                    record(argument);
                }
            }
        }
    }
}

fn mutable_arguments<P>(
    unit: &SemanticUnit,
    arguments: &SyntaxNode,
    parameters: &[P],
    name: impl Fn(&P) -> &str,
    is_mutable: impl Fn(&P) -> bool,
    record: &mut impl FnMut(&SyntaxNode),
) {
    let mut positional = 0;
    for argument in &arguments.children {
        let named = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let index = named.map_or_else(
            || {
                let index = positional;
                positional += 1;
                index
            },
            |named| {
                parameters
                    .iter()
                    .position(|parameter| name(parameter) == node_text(&unit.source, named))
                    .unwrap_or(usize::MAX)
            },
        );
        if parameters.get(index).is_some_and(&is_mutable) {
            record(argument);
        }
    }
}

fn argument_root(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind {
        SyntaxKind::Name => Some(node),
        SyntaxKind::MemberExpression
        | SyntaxKind::IndexExpression
        | SyntaxKind::GroupExpression => node.children.first().and_then(argument_root),
        _ => node.children.last().and_then(argument_root),
    }
}

fn object_mutation_root<'a>(unit: &SemanticUnit, node: &'a SyntaxNode) -> Option<&'a SyntaxNode> {
    match node.kind {
        SyntaxKind::Name => Some(node),
        SyntaxKind::MemberExpression
            if node.children.first().is_some_and(|receiver| {
                matches!(
                    unit.inferred_value_type(receiver),
                    Some(ValueType::Object(_))
                )
            }) =>
        {
            node.children
                .first()
                .and_then(|receiver| object_mutation_root(unit, receiver))
        }
        SyntaxKind::IndexExpression | SyntaxKind::GroupExpression => node
            .children
            .first()
            .and_then(|child| object_mutation_root(unit, child)),
        _ => None,
    }
}

fn explicit_field_extraction(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    target: &SyntaxNode,
) -> bool {
    let Ok(Some(ty)) = infer_value_type(unit, target, &unit.typed_bindings) else {
        return false;
    };
    if matches!(ty, ValueType::Optional(_)) {
        return true;
    }
    if !application_is_resource_owning(package, &ty) && value_type_is_cloneable(package, unit, &ty)
    {
        return false;
    }
    let Some(receiver) = target.children.first() else {
        return false;
    };
    let Ok(Some(ValueType::Object(object))) =
        infer_value_type(unit, receiver, &unit.typed_bindings)
    else {
        return false;
    };
    package
        .units
        .iter()
        .flat_map(|unit| &unit.descriptors)
        .find(|descriptor| {
            descriptor.identity.namespace == object.namespace
                && descriptor.identity.name == object.name
        })
        .is_some_and(|descriptor| {
            target.children.get(1).is_some_and(|member| {
                descriptor
                    .fields
                    .iter()
                    .any(|field| field.name == node_text(&unit.source, member) && field.required)
            })
        })
}
