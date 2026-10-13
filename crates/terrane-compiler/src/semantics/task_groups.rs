use super::prelude::*;

// Group liveness is span-set state, so it cannot use scope_flow's FlowEnv
// transfer directly. It does share the canonical throwable query and the
// language's branch/finally structure and exit validation.

type Live = BTreeSet<Span>;

struct Flow<'a> {
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    inferred: &'a BTreeMap<(u32, usize, usize), BTreeSet<String>>,
    catches: Vec<BTreeSet<String>>,
    seen: Live,
}

fn binding<'a>(unit: &'a SemanticUnit, node: &SyntaxNode) -> Option<&'a TypedBinding> {
    if node.kind != SyntaxKind::Name {
        return None;
    }
    unit.typed_bindings.iter().rev().find(|item| {
        item.name == node_text(&unit.source, node)
            && item.is_visible_at(unit.source.id(), node.span.start)
            && matches!(item.value_type, ValueType::TaskGroup(_))
    })
}

fn undrained(unit: &SemanticUnit, span: Span) -> SemanticFailure {
    failure(
        &unit.source,
        "T0076",
        "task group must be awaited and drained on every exit path",
        span,
    )
}

fn factory<'a>(unit: &SemanticUnit, mut node: &'a SyntaxNode) -> Option<&'a SyntaxNode> {
    while node.kind == SyntaxKind::GroupExpression {
        node = node.children.first()?;
    }
    (node.kind == SyntaxKind::CallExpression
        && node.children.first().is_some_and(|callee| {
            callee.kind == SyntaxKind::Name
                && resolved_compiler_identity(unit, callee) == Some("/core/async::task-group")
        }))
    .then_some(node)
}

impl<'a> Flow<'a> {
    fn implicit_exit(
        &mut self,
        node: &SyntaxNode,
        live: &mut Live,
        awaited: bool,
        finalizers: &[&'a SyntaxNode],
    ) -> Result<(), SemanticFailure> {
        let deferred_async_call = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                matches!(
                    infer_value_type(self.unit, callee, &self.unit.typed_bindings),
                    Ok(Some(ValueType::AsyncFunction(..)))
                )
            })
            && !awaited;
        let group_operation = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                callee.kind == SyntaxKind::MemberExpression
                    && callee
                        .children
                        .first()
                        .is_some_and(|receiver| binding(self.unit, receiver).is_some())
            });
        if !live.is_empty()
            && !deferred_async_call
            && !group_operation
            && matches!(
                node.kind,
                SyntaxKind::CallExpression
                    | SyntaxKind::UnaryExpression
                    | SyntaxKind::BinaryExpression
                    | SyntaxKind::IndexExpression
            )
        {
            let mut errors =
                super::contracts::escaping_throwables(self.package, self.unit, node, self.inferred);
            for escaping in &self.catches {
                errors.retain(|error| escaping.contains(error));
            }
            if !errors.is_empty() {
                self.exit(live, finalizers, node.span)?;
            }
        }
        Ok(())
    }

    fn expression(
        &mut self,
        node: &SyntaxNode,
        live: &mut Live,
        awaited: bool,
        finalizers: &[&'a SyntaxNode],
    ) -> Result<(), SemanticFailure> {
        self.implicit_exit(node, live, awaited, finalizers)?;
        if factory(self.unit, node).is_some() {
            return Err(failure(
                &self.unit.source,
                "T0076",
                "task-group must be bound to an owning local variable and drained",
                node.span,
            ));
        }
        if node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(self.unit, node).as_deref() == Some("await")
        {
            if let Some(value) = node.children.last() {
                self.expression(value, live, true, finalizers)?;
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && let Some(owner) = binding(self.unit, receiver)
        {
            if !live.contains(&owner.span) {
                return Err(failure(
                    &self.unit.source,
                    "T0076",
                    "task group is no longer available",
                    receiver.span,
                ));
            }
            match node_text(&self.unit.source, member) {
                "drain" => {
                    if !awaited {
                        return Err(failure(
                            &self.unit.source,
                            "T0076",
                            "task-group.drain must be awaited immediately",
                            node.span,
                        ));
                    }
                    live.remove(&owner.span);
                }
                "spawn" | "cancel" => {}
                _ => {
                    return Err(failure(
                        &self.unit.source,
                        "T0076",
                        "task group cannot escape its owning scope",
                        receiver.span,
                    ));
                }
            }
            for child in node.children.iter().skip(1) {
                self.expression(child, live, false, finalizers)?;
            }
            return Ok(());
        }
        if binding(self.unit, node).is_some() {
            return Err(failure(
                &self.unit.source,
                "T0076",
                "task group cannot be copied, moved, borrowed, or returned; use its operations in the owning scope",
                node.span,
            ));
        }
        // A lambda has its own flow; it cannot capture an owning group.
        for child in &node.children {
            self.expression(
                child,
                live,
                awaited && node.kind == SyntaxKind::GroupExpression,
                finalizers,
            )?;
        }
        Ok(())
    }

    fn exit(
        &mut self,
        live: &Live,
        finalizers: &[&'a SyntaxNode],
        span: Span,
    ) -> Result<(), SemanticFailure> {
        let mut remaining = live.clone();
        for (index, finally) in finalizers.iter().enumerate().rev() {
            self.block(finally, &mut remaining, &finalizers[..index], None, false)?;
        }
        if !remaining.is_empty() {
            return Err(undrained(self.unit, span));
        }
        Ok(())
    }

    fn block(
        &mut self,
        block: &'a SyntaxNode,
        live: &mut Live,
        finalizers: &[&'a SyntaxNode],
        loop_entry: Option<&Live>,
        lexical_exit: bool,
    ) -> Result<bool, SemanticFailure> {
        let incoming = live.clone();
        for statement in &block.children {
            if !self.statement(statement, live, finalizers, loop_entry)? {
                return Ok(false);
            }
        }
        if lexical_exit && live.iter().any(|span| !incoming.contains(span)) {
            return Err(undrained(self.unit, block.span));
        }
        Ok(true)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "group liveness transfer keeps all language branch and exit forms together"
    )]
    fn statement(
        &mut self,
        node: &'a SyntaxNode,
        live: &mut Live,
        finalizers: &[&'a SyntaxNode],
        loop_entry: Option<&Live>,
    ) -> Result<bool, SemanticFailure> {
        match node.kind {
            SyntaxKind::Binding | SyntaxKind::Assignment => {
                if let Some(owner) = self.unit.typed_bindings.iter().find(|item| {
                    matches!(item.value_type, ValueType::TaskGroup(_))
                        && item.span.start >= node.span.start
                        && item.span.end <= node.span.end
                }) {
                    if let Some(value) = node.children.last() {
                        if let Some(call) = factory(self.unit, value) {
                            for arguments in call.children.iter().skip(1) {
                                self.expression(arguments, live, false, finalizers)?;
                            }
                        } else {
                            self.expression(value, live, false, finalizers)?;
                        }
                    }
                    self.seen.insert(owner.span);
                    live.insert(owner.span);
                    return Ok(true);
                }
            }
            SyntaxKind::ReturnStatement | SyntaxKind::ThrowStatement => {
                for child in &node.children {
                    self.expression(child, live, false, finalizers)?;
                }
                self.exit(live, finalizers, node.span)?;
                live.clear();
                return Ok(false);
            }
            SyntaxKind::BreakStatement | SyntaxKind::ContinueStatement => {
                if loop_entry.is_some_and(|entry| entry != live) {
                    return Err(undrained(self.unit, node.span));
                }
                return Ok(false);
            }
            SyntaxKind::IfStatement => {
                if let Some(condition) = node.children.first() {
                    self.expression(condition, live, false, finalizers)?;
                }
                let entry = live.clone();
                let mut paths = Vec::new();
                let mut has_else = false;
                for branch in node.children.iter().skip(1) {
                    let body = if branch.kind == SyntaxKind::Block {
                        Some(branch)
                    } else if branch.kind == SyntaxKind::ElseClause {
                        has_else |= branch
                            .children
                            .first()
                            .is_some_and(|child| child.kind == SyntaxKind::Block);
                        for condition in branch
                            .children
                            .iter()
                            .filter(|child| child.kind != SyntaxKind::Block)
                        {
                            self.expression(condition, &mut entry.clone(), false, finalizers)?;
                        }
                        branch
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::Block)
                    } else {
                        None
                    };
                    if let Some(body) = body {
                        let mut path = entry.clone();
                        if self.block(body, &mut path, finalizers, loop_entry, true)? {
                            paths.push(path);
                        }
                    }
                }
                if !has_else {
                    paths.push(entry);
                }
                return self.merge(paths, live, node.span);
            }
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement => {
                let entry = live.clone();
                for child in &node.children {
                    if child.kind == SyntaxKind::Block {
                        let mut path = entry.clone();
                        self.block(child, &mut path, finalizers, Some(&entry), true)?;
                        if path != entry {
                            return Err(undrained(self.unit, node.span));
                        }
                    } else {
                        self.expression(child, live, false, finalizers)?;
                    }
                }
                return Ok(true);
            }
            SyntaxKind::TryStatement => {
                let finally = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::FinallyClause)
                    .and_then(|clause| clause.children.first());
                let mut pending = finalizers.to_vec();
                if let Some(finally) = finally {
                    pending.push(finally);
                }
                let escaping = super::contracts::escaping_throwables(
                    self.package,
                    self.unit,
                    node,
                    self.inferred,
                );
                let entry = live.clone();
                let mut paths = Vec::new();
                for clause in &node.children {
                    let body = if clause.kind == SyntaxKind::Block {
                        Some(clause)
                    } else if clause.kind == SyntaxKind::CatchClause {
                        clause
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::Block)
                    } else {
                        None
                    };
                    if let Some(body) = body {
                        let mut path = entry.clone();
                        if clause.kind == SyntaxKind::Block {
                            self.catches.push(escaping.clone());
                        }
                        let result = self.block(body, &mut path, &pending, loop_entry, false);
                        if clause.kind == SyntaxKind::Block {
                            self.catches.pop();
                        }
                        if result? {
                            if let Some(finally) = finally {
                                self.block(finally, &mut path, finalizers, loop_entry, false)?;
                            }
                            paths.push(path);
                        }
                    }
                }
                return self.merge(paths, live, node.span);
            }
            SyntaxKind::MatchStatement
            | SyntaxKind::SwitchStatement
            | SyntaxKind::SelectStatement => {
                let mut paths = Vec::new();
                for case in &node.children {
                    if let Some(body) = case
                        .children
                        .last()
                        .filter(|body| body.kind == SyntaxKind::Block)
                    {
                        let mut path = live.clone();
                        for header in case
                            .children
                            .iter()
                            .filter(|child| child.kind != SyntaxKind::Block)
                        {
                            self.expression(header, &mut path, false, finalizers)?;
                        }
                        if self.block(body, &mut path, finalizers, loop_entry, true)? {
                            paths.push(path);
                        }
                    } else {
                        self.expression(case, live, false, finalizers)?;
                    }
                }
                if node.kind == SyntaxKind::SwitchStatement && !node.switch_has_default() {
                    paths.push(live.clone());
                }
                return self.merge(paths, live, node.span);
            }
            SyntaxKind::FunctionDeclaration => return Ok(true),
            _ => {}
        }
        self.expression(node, live, false, finalizers)?;
        Ok(true)
    }

    fn merge(
        &self,
        paths: Vec<Live>,
        live: &mut Live,
        span: Span,
    ) -> Result<bool, SemanticFailure> {
        let mut paths = paths.into_iter();
        let Some(first) = paths.next() else {
            live.clear();
            return Ok(false);
        };
        if paths.any(|path| path != first) {
            return Err(undrained(self.unit, span));
        }
        *live = first;
        Ok(true)
    }
}

pub(super) fn validate_task_groups(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    let inferred = package
        .units
        .iter()
        .flat_map(|unit| unit.functions.iter())
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract.escaping_throwables.clone(),
            )
        })
        .collect();
    for unit in &package.units {
        let mut flow = Flow {
            package,
            unit,
            inferred: &inferred,
            catches: Vec::new(),
            seen: Live::new(),
        };
        for contract in &unit.functions {
            if contract.span.file != unit.source.id() {
                continue;
            }
            if let Some(function) =
                super::ownership::find_node_by_span(&unit.tree.root, contract.span)
                && let Some(body) = function
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
            {
                let mut live = Live::new();
                flow.block(body, &mut live, &[], None, true)?;
                if !live.is_empty() {
                    return Err(undrained(unit, body.span));
                }
            }
        }
        for owner in unit
            .typed_bindings
            .iter()
            .filter(|item| matches!(item.value_type, ValueType::TaskGroup(_)))
        {
            if !flow.seen.contains(&owner.span) {
                return Err(undrained(unit, owner.span));
            }
        }
    }
    Ok(())
}
