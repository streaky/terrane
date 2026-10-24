use super::prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Root {
    Binding(Span),
    This(Span),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FieldPath {
    root: Root,
    fields: Vec<String>,
}

type Moves = BTreeSet<FieldPath>;

fn ungroup(mut node: &SyntaxNode) -> &SyntaxNode {
    while node.kind == SyntaxKind::GroupExpression {
        let Some(child) = node.children.first() else {
            break;
        };
        node = child;
    }
    node
}

fn path_at(unit: &SemanticUnit, node: &SyntaxNode) -> Option<FieldPath> {
    let node = ungroup(node);
    if node.kind == SyntaxKind::Name {
        if node_text(&unit.source, node) == "this" {
            let owner = unit
                .functions
                .iter()
                .filter(|function| {
                    function.owner.is_some()
                        && function.span.file == node.span.file
                        && function.span.start <= node.span.start
                        && node.span.end <= function.span.end
                })
                .min_by_key(|function| function.span.end - function.span.start)?;
            return Some(FieldPath {
                root: Root::This(owner.span),
                fields: Vec::new(),
            });
        }
        let span = unit
            .flow_binding_ids
            .get(&(node.span.file, node.span.start, node.span.end))
            .copied()
            .or_else(|| {
                unit.typed_bindings
                    .iter()
                    .rev()
                    .find(|binding| {
                        binding.name == node_text(&unit.source, node)
                            && binding.is_visible_at(unit.source.id(), node.span.start)
                    })
                    .map(|binding| binding.span)
            })?;
        return Some(FieldPath {
            root: Root::Binding(span),
            fields: Vec::new(),
        });
    }
    if node.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let [receiver, member] = node.children.as_slice() else {
        return None;
    };
    let mut path = path_at(unit, receiver)?;
    path.fields.push(node_text(&unit.source, member).to_owned());
    Some(path)
}

fn field_owner(
    unit: &SemanticUnit,
    access: &SyntaxNode,
) -> Result<Option<ObjectIdentity>, SemanticFailure> {
    let Some(receiver) = access.children.first() else {
        return Ok(None);
    };
    let Some(ty) = infer_value_type(unit, receiver, &unit.typed_bindings)? else {
        return Ok(None);
    };
    let mut ty = &ty;
    loop {
        match ty {
            ValueType::Reference(inner) | ValueType::SharedReference(inner) => {
                ty = inner.value_type_ref();
            }
            ValueType::Optional(inner) => ty = inner,
            ValueType::Object(identity) => return Ok(Some(identity.clone())),
            _ => return Ok(None),
        }
    }
}

fn destructive_move(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    access: &SyntaxNode,
) -> Result<bool, SemanticFailure> {
    let Some(owner) = field_owner(unit, access)? else {
        return Ok(false);
    };
    let Some(member) = access.children.last() else {
        return Ok(false);
    };
    let ty = match object_field_type(unit, &owner, node_text(&unit.source, member), false) {
        Some(ty) => Some(ty),
        None => infer_value_type(unit, access, &unit.typed_bindings)?,
    };
    let Some(ty) = ty else { return Ok(false) };
    if matches!(ty, ValueType::Reference(_) | ValueType::SharedReference(_))
        || !value_type_contains_nonclone_foreign(unit, &ty)
    {
        return Ok(false);
    }
    // Native fields move directly. Only authored optional fields leave a `none` slot.
    if package
        .projection
        .item(&owner.namespace, &owner.name)
        .is_some()
    {
        return Ok(true);
    }
    Ok(!matches!(ty, ValueType::Optional(_)))
}

fn overlaps(left: &FieldPath, right: &FieldPath) -> bool {
    left.root == right.root
        && (left.fields.starts_with(&right.fields) || right.fields.starts_with(&left.fields))
}

fn check_access(
    unit: &SemanticUnit,
    path: &FieldPath,
    moved: &Moves,
    span: Span,
) -> Result<(), SemanticFailure> {
    if moved.iter().any(|old| overlaps(old, path)) {
        return Err(failure(
            &unit.source,
            "T0101",
            "use of a field or owner after a field was moved",
            span,
        ));
    }
    Ok(())
}

fn join(branches: impl IntoIterator<Item = Moves>) -> Moves {
    branches.into_iter().flatten().collect()
}

#[derive(Default)]
struct FieldFlow {
    normal: bool,
    breaks: Moves,
    continues: Moves,
    terminated: Moves,
}

impl FieldFlow {
    fn normal() -> Self {
        Self {
            normal: true,
            ..Self::default()
        }
    }

    fn merge_exits(&mut self, other: Self) {
        self.breaks.extend(other.breaks);
        self.continues.extend(other.continues);
        self.terminated.extend(other.terminated);
    }
}

fn visit_children(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &mut Moves,
) -> Result<FieldFlow, SemanticFailure> {
    let mut flow = FieldFlow::normal();
    for (index, child) in node.children.iter().enumerate() {
        if flow.normal && super::scope_flow::is_flow_value_child(node, index, child) {
            let child_flow = visit(package, unit, child, moved)?;
            flow.normal = child_flow.normal;
            flow.merge_exits(child_flow);
        }
    }
    Ok(flow)
}

fn visit_if(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &mut Moves,
) -> Result<FieldFlow, SemanticFailure> {
    let mut flow = FieldFlow::default();
    for child in &node.children {
        if !matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
            flow.merge_exits(visit(package, unit, child, moved)?);
        }
    }
    let mut branches = Vec::new();
    let mut has_else = false;
    for child in &node.children {
        if matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
            has_else |= child.kind == SyntaxKind::ElseClause;
            let mut branch = moved.clone();
            let outcome = visit(package, unit, child, &mut branch)?;
            if outcome.normal {
                branches.push(branch);
            }
            flow.merge_exits(outcome);
        }
    }
    if !has_else {
        branches.push(moved.clone());
    }
    flow.normal = !branches.is_empty();
    *moved = join(branches);
    Ok(flow)
}

fn visit_loop(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &mut Moves,
) -> Result<FieldFlow, SemanticFailure> {
    let body = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Block);
    for child in &node.children {
        if Some(child) != body {
            visit(package, unit, child, moved)?;
        }
    }
    let mut exits = moved.clone(); // A loop may execute zero times.
    let Some(body) = body else {
        return Ok(FieldFlow::normal());
    };
    let mut after = moved.clone();
    let mut flow = visit(package, unit, body, &mut after)?;
    exits.extend(flow.breaks.iter().cloned());
    exits.extend(flow.continues.iter().cloned());
    if flow.normal {
        exits.extend(after.iter().cloned());
    }
    if flow.normal || !flow.continues.is_empty() {
        let mut next = std::mem::take(&mut flow.continues);
        if flow.normal {
            next.extend(after);
        }
        if node.kind == SyntaxKind::ForStatement {
            if let Some(targets) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::ForTarget)
            {
                for name in &targets.children {
                    if let Some(path) = path_at(unit, name) {
                        next.retain(|old| old.root != path.root);
                    }
                }
            }
        } else {
            for condition in node
                .children
                .iter()
                .filter(|child| child.kind != SyntaxKind::Block)
            {
                visit(package, unit, condition, &mut next)?;
            }
        }
        let second = visit(package, unit, body, &mut next)?;
        exits.extend(second.breaks);
        exits.extend(second.continues);
        if second.normal {
            exits.extend(next);
        }
        flow.terminated.extend(second.terminated);
    }
    *moved = exits;
    Ok(FieldFlow {
        normal: true,
        terminated: flow.terminated,
        ..FieldFlow::default()
    })
}

fn visit_cases(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &mut Moves,
) -> Result<FieldFlow, SemanticFailure> {
    let cases: Vec<_> = if node.kind == SyntaxKind::SwitchStatement {
        if let Some(selector) = node.switch_selector() {
            visit(package, unit, selector, moved)?;
        }
        node.switch_cases().collect()
    } else if node.kind == SyntaxKind::MatchStatement {
        if let Some(scrutinee) = node.children.first() {
            visit(package, unit, scrutinee, moved)?;
        }
        node.children.iter().skip(1).collect()
    } else {
        node.children.iter().collect()
    };
    for case in &cases {
        for header in case.children.iter().filter(|child| {
            child.kind != SyntaxKind::Block && child.kind != SyntaxKind::MatchCatchAll
        }) {
            visit(package, unit, header, moved)?;
        }
    }
    let mut flow = FieldFlow::default();
    let mut branches = Vec::new();
    for case in cases {
        if let Some(body) = case
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            let mut branch = moved.clone();
            let outcome = visit(package, unit, body, &mut branch)?;
            if outcome.normal {
                branches.push(branch);
            }
            flow.merge_exits(outcome);
        }
    }
    if node.kind == SyntaxKind::SwitchStatement && !node.switch_has_default() {
        branches.push(moved.clone());
    }
    flow.normal = !branches.is_empty();
    *moved = join(branches);
    Ok(flow)
}

fn visit_try(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &mut Moves,
) -> Result<FieldFlow, SemanticFailure> {
    let mut flow = FieldFlow::default();
    let mut branches = Vec::new();
    let mut tried = moved.clone();
    if let Some(body) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Block)
    {
        let outcome = visit(package, unit, body, &mut tried)?;
        if outcome.normal {
            branches.push(tried.clone());
        }
        flow.merge_exits(outcome);
    }
    for clause in node
        .children
        .iter()
        .filter(|child| child.kind == SyntaxKind::CatchClause)
    {
        if let Some(body) = clause
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            let mut branch = join([moved.clone(), tried.clone(), flow.terminated.clone()]);
            let outcome = visit(package, unit, body, &mut branch)?;
            if outcome.normal {
                branches.push(branch);
            }
            flow.merge_exits(outcome);
        }
    }
    if let Some(finally) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::FinallyClause)
        .and_then(|clause| clause.children.first())
    {
        let mut normal = Vec::new();
        for mut branch in branches {
            let outcome = visit(package, unit, finally, &mut branch)?;
            if outcome.normal {
                normal.push(branch);
            }
            flow.merge_exits(outcome);
        }
        branches = normal;
        // Finally also runs on return, throw, break and continue paths.
        for paths in [&mut flow.breaks, &mut flow.continues, &mut flow.terminated] {
            if !paths.is_empty() {
                visit(package, unit, finally, paths)?;
            }
        }
    }
    flow.normal = !branches.is_empty();
    *moved = join(branches);
    Ok(flow)
}

fn check_rust_access(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &Moves,
) -> Result<(), SemanticFailure> {
    let identifiers = crate::rust_ir::rust_free_value_identifiers(node_text(&unit.source, node));
    for path in moved {
        let reads_root = match path.root {
            Root::Binding(span) => identifiers
                .iter()
                .filter_map(|identifier| {
                    crate::lowering::rust_value_binding_at(unit, node.span.start, identifier)
                })
                .any(|(_, binding)| binding.span == span),
            Root::This(span) => {
                span.file == node.span.file
                    && span.start <= node.span.start
                    && node.span.end <= span.end
                    && (identifiers.contains("self") || identifiers.contains("this"))
            }
        };
        if reads_root {
            check_access(
                unit,
                &FieldPath {
                    root: path.root.clone(),
                    fields: Vec::new(),
                },
                moved,
                node.span,
            )?;
        }
    }
    Ok(())
}

fn visit_assignment(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &mut Moves,
) -> Result<FieldFlow, SemanticFailure> {
    let mut flow = FieldFlow::normal();
    for child in node.children.iter().skip(1) {
        let outcome = visit(package, unit, child, moved)?;
        flow.normal = outcome.normal;
        flow.merge_exits(outcome);
        if !flow.normal {
            return Ok(flow);
        }
    }
    if let Some(target) = node.children.first() {
        if let Some(path) = path_at(unit, target) {
            // A missing parent cannot be used to restore one of its children.
            if moved.iter().any(|old| {
                old.root == path.root
                    && old.fields.len() < path.fields.len()
                    && path.fields.starts_with(&old.fields)
            }) {
                check_access(unit, &path, moved, target.span)?;
            }
            moved.retain(|old| old.root != path.root || !old.fields.starts_with(&path.fields));
        } else {
            flow.merge_exits(visit(package, unit, target, moved)?);
        }
    }
    Ok(flow)
}

fn visit(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    moved: &mut Moves,
) -> Result<FieldFlow, SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        let mut nested = moved.clone();
        let outcome = visit_children(package, unit, node, &mut nested)?;
        if node.kind == SyntaxKind::AnonymousFunction {
            nested.extend(outcome.terminated);
            moved.extend(nested.into_iter().filter(|path| match path.root {
                Root::Binding(span) => {
                    !(span.file == node.span.file
                        && node.span.start <= span.start
                        && span.end <= node.span.end)
                }
                Root::This(_) => true,
            }));
        }
        return Ok(FieldFlow::normal());
    }
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("move")
        && let Some(access) = node.children.last().map(ungroup)
        && access.kind == SyntaxKind::MemberExpression
        && let Some(path) = path_at(unit, access)
    {
        check_access(unit, &path, moved, access.span)?;
        if destructive_move(package, unit, access)? {
            moved.insert(path);
        }
        return Ok(FieldFlow::normal());
    }
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
        return visit_assignment(package, unit, node, moved);
    }
    match node.kind {
        SyntaxKind::IfStatement => return visit_if(package, unit, node, moved),
        SyntaxKind::WhileStatement | SyntaxKind::ForStatement => {
            return visit_loop(package, unit, node, moved);
        }
        SyntaxKind::SwitchStatement | SyntaxKind::MatchStatement | SyntaxKind::SelectStatement => {
            return visit_cases(package, unit, node, moved);
        }
        SyntaxKind::TryStatement => return visit_try(package, unit, node, moved),
        SyntaxKind::BreakStatement => {
            return Ok(FieldFlow {
                breaks: moved.clone(),
                ..FieldFlow::default()
            });
        }
        SyntaxKind::ContinueStatement => {
            return Ok(FieldFlow {
                continues: moved.clone(),
                ..FieldFlow::default()
            });
        }
        SyntaxKind::ReturnStatement | SyntaxKind::ThrowStatement => {
            visit_children(package, unit, node, moved)?;
            return Ok(FieldFlow {
                terminated: moved.clone(),
                ..FieldFlow::default()
            });
        }
        _ => {}
    }
    if !moved.is_empty() {
        if matches!(
            node.kind,
            SyntaxKind::RustBlock | SyntaxKind::UnsafeRustBlock
        ) {
            check_rust_access(unit, node, moved)?;
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && let Some(path) = path_at(unit, receiver)
            && let Some(owner) = field_owner(unit, callee)?
            && object_field_type(unit, &owner, node_text(&unit.source, member), false).is_none()
        {
            check_access(unit, &path, moved, receiver.span)?;
        }
        if matches!(node.kind, SyntaxKind::MemberExpression | SyntaxKind::Name)
            && let Some(path) = path_at(unit, node)
        {
            check_access(unit, &path, moved, node.span)?;
            return Ok(FieldFlow::normal());
        }
    }
    visit_children(package, unit, node, moved)
}

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        visit(package, unit, &unit.tree.root, &mut Moves::new())?;
    }
    Ok(())
}
