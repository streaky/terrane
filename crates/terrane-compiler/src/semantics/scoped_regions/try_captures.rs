//! Source diagnostics for native values moved into lowering-owned try closures.
use super::super::capabilities::rust_value_is_copy;
use super::super::prelude::*;
use super::liveness::Flow;

fn named_value(mut node: &SyntaxNode) -> &SyntaxNode {
    while node.kind == SyntaxKind::GroupExpression && node.children.len() == 1 {
        node = &node.children[0];
    }
    node
}

fn rebinds(unit: &SemanticUnit, node: &SyntaxNode, binding: Span) -> bool {
    (node.kind == SyntaxKind::Assignment
        && node
            .children
            .first()
            .and_then(|target| super::binding_identity_at(unit, target))
            == Some(binding))
        || node
            .children
            .iter()
            .any(|child| rebinds(unit, child, binding))
}

fn validate_moved_value(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    value: &SyntaxNode,
    flow: &Flow,
    explicit: bool,
) -> Result<(), SemanticFailure> {
    let value = named_value(value);
    if let Some(ty) = unit.inferred_value_type(value)
        && !rust_value_is_copy(&ty)
        && !package.native_value_is_copyable(&ty)
        && !matches!(
            &ty,
            ValueType::Reference(_)
                | ValueType::MutableReference(_)
                | ValueType::SharedReference(_)
        )
        // Ordinary callable arguments clone their Arc-backed handle; they do
        // not move the outer binding as an explicit `move` would.
        && (explicit || !matches!(&ty, ValueType::Function(..) | ValueType::AsyncFunction(..)))
        && (explicit
            || !matches!(&ty, ValueType::Object(identity)
                if package.projection.item(&identity.namespace, &identity.name).is_some()
                    && super::super::objects::value_type_is_cloneable(package, unit, &ty)
                    && !application_is_resource_owning(package, &ty)))
        && let Some(binding) = super::binding_identity_at(unit, value)
        && binding.start < statement.span.start
        && rebinds(unit, statement, binding)
        && flow.completion_reads_without_rewrite(statement, value.span, binding)
    {
        return Err(failure(
            &unit.source,
            "T0058",
            "a try closure cannot restore an outer non-Copy value after consuming it; acquire and consume the value inside the try or move consumption outside the try",
            value.span,
        ));
    }
    Ok(())
}

fn visit_consumption(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    consume: &mut impl FnMut(&SyntaxNode, bool) -> Result<(), SemanticFailure>,
) -> Result<(), SemanticFailure> {
    let receiver = if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && let Some((receiver, ty, member)) =
            super::super::diagnostics::typed_member_call(unit, callee)
        && super::super::diagnostics::member_invocation_mode_at(
            package, unit, receiver, &ty, &member,
        ) == InvocationMode::Consuming
    {
        Some(receiver)
    } else if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("move")
    {
        node.children.last()
    } else {
        None
    };
    if let Some(receiver) = receiver {
        consume(receiver, node.kind == SyntaxKind::UnaryExpression)?;
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && let Some(contract) = function_contract_for_call(package, unit, callee)
    {
        let projected = super::super::bindings::projected_function_for_call(
            package,
            unit,
            callee,
            crate::syntax::call_is_unsafe(node),
        );
        for (index, parameter) in contract.parameters.iter().enumerate() {
            let borrowed = projected
                .and_then(|function| function.parameters.get(index))
                .is_some_and(|parameter| parameter.borrowed)
                || parameter.element_value_type().is_some_and(|ty| {
                    matches!(
                        ty,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    )
                });
            if !borrowed
                && let Some(value) = argument_for_parameter(unit, arguments, contract, index)
            {
                consume(value, false)?;
            }
        }
    }
    for child in &node.children {
        if !matches!(
            child.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            visit_consumption(package, unit, child, consume)?;
        }
    }
    Ok(())
}

fn contains_try(node: &SyntaxNode) -> bool {
    node.kind == SyntaxKind::TryStatement
        || node.children.iter().any(|child| {
            !matches!(
                child.kind,
                SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
            ) && contains_try(child)
        })
}

fn validate_in_function(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    flow: &Flow,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        return validate(package, unit, node);
    }
    if node.kind == SyntaxKind::TryStatement {
        visit_consumption(package, unit, node, &mut |value, explicit| {
            validate_moved_value(package, unit, node, value, flow, explicit)
        })?;
    }
    for child in &node.children {
        validate_in_function(package, unit, child, flow)?;
    }
    Ok(())
}

pub(super) fn validate(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) && contains_try(node)
    {
        let flow = Flow::new(package, unit, node);
        for child in &node.children {
            validate_in_function(package, unit, child, &flow)?;
        }
        return Ok(());
    }
    for child in &node.children {
        validate(package, unit, child)?;
    }
    Ok(())
}

fn collect(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    writes_only: bool,
    bindings: &mut BTreeSet<Span>,
) {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        return;
    }
    let target = if writes_only {
        (node.kind == SyntaxKind::Assignment)
            .then(|| node.children.first())
            .flatten()
    } else {
        (node.kind == SyntaxKind::Name).then_some(node)
    };
    if let Some(binding) = target.and_then(|target| super::binding_identity_at(unit, target)) {
        bindings.insert(binding);
    }
    for child in &node.children {
        collect(unit, child, writes_only, bindings);
    }
}
pub(super) fn completion_capture_slots(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> BTreeSet<Span> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        flow: Option<&Flow>,
        slots: &mut BTreeSet<Span>,
    ) {
        if matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            let flow = contains_try(node).then(|| Flow::new(package, unit, node));
            for child in &node.children {
                visit(package, unit, child, flow.as_ref(), slots);
            }
            return;
        }
        if node.kind == SyntaxKind::TryStatement
            && let Some(flow) = flow
        {
            let mut writes = BTreeSet::new();
            let mut body_uses = BTreeSet::new();
            let mut uses = BTreeSet::new();
            collect(unit, node, true, &mut writes);
            for clause in &node.children {
                visit_consumption(package, unit, clause, &mut |value, explicit| {
                    let value = named_value(value);
                    if !explicit
                        && unit.inferred_value_type(value).is_some_and(|ty| {
                            matches!(ty, ValueType::Function(..) | ValueType::AsyncFunction(..))
                        })
                    {
                        return Ok(());
                    }
                    if let Some(binding) = super::binding_identity_at(unit, value) {
                        body_uses.insert(binding);
                    }
                    Ok(())
                })
                .expect("collecting completion transfers is infallible");
            }
            if let Some(finally) = node
                .children
                .iter()
                .find(|clause| clause.kind == SyntaxKind::FinallyClause)
            {
                collect(unit, finally, false, &mut uses);
            }
            for binding in unit.typed_bindings.iter().filter(|binding| {
                binding.span.end <= node.span.start
                    && writes.contains(&binding.span)
                    && body_uses.contains(&binding.span)
                    && (uses.contains(&binding.span)
                        || flow.reads_after_without_rewrite(node, binding.span))
            }) {
                if !rust_value_is_copy(&binding.value_type)
                    && !package.native_value_is_copyable(&binding.value_type)
                    && !matches!(
                        &binding.value_type,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    )
                {
                    slots.insert(binding.span);
                }
            }
        }
        for child in &node.children {
            visit(package, unit, child, flow, slots);
        }
    }
    let mut slots = BTreeSet::new();
    visit(package, unit, &unit.tree.root, None, &mut slots);
    slots
}
