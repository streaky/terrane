use super::model::RequiredInitProof;
use super::prelude::*;

#[derive(Clone, Default)]
struct FieldState {
    definite: BTreeSet<String>,
    possible: BTreeSet<String>,
}

#[derive(Default)]
struct Flow {
    normal: Option<FieldState>,
    breaks: Vec<FieldState>,
    continues: Vec<FieldState>,
    failures: Vec<FieldState>,
    returns: Vec<FieldState>,
}

struct ConstructorAnalysis<'a> {
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    required: BTreeSet<String>,
    fields: BTreeSet<String>,
}

fn merge(states: impl IntoIterator<Item = FieldState>) -> Option<FieldState> {
    let mut states = states.into_iter();
    let mut merged = states.next()?;
    for state in states {
        merged
            .definite
            .retain(|field| state.definite.contains(field));
        merged.possible.extend(state.possible);
    }
    Some(merged)
}

impl ConstructorAnalysis<'_> {
    fn complete(&self, state: &FieldState, span: Span) -> Result<(), SemanticFailure> {
        let missing = self
            .required
            .difference(&state.definite)
            .cloned()
            .collect::<Vec<_>>();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(failure(
                &self.unit.source,
                "T0220",
                format!(
                    "construction must initialize every required field on every successful path; missing {}",
                    missing.join(", "),
                ),
                span,
            ))
        }
    }

    fn direct_field<'a>(&'a self, node: &SyntaxNode) -> Option<&'a str> {
        let [receiver, member] = node.children.as_slice() else {
            return None;
        };
        (node.kind == SyntaxKind::MemberExpression
            && receiver.kind == SyntaxKind::Name
            && node_text(&self.unit.source, receiver) == "this")
            .then(|| node_text(&self.unit.source, member))
    }

    fn reads(&self, node: &SyntaxNode, state: &mut FieldState) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(self.unit, node).as_deref() == Some("move")
            && let Some(operand) = node.children.last()
        {
            let mut operand = operand;
            while operand.kind == SyntaxKind::GroupExpression {
                let Some(inner) = operand.children.first() else {
                    break;
                };
                operand = inner;
            }
            if let Some(field) = self.direct_field(operand)
                && self.required.contains(field)
            {
                self.reads(operand, state)?;
                state.definite.remove(field);
                return Ok(());
            }
        }
        if let Some(field) = self.direct_field(node)
            && self.fields.contains(field)
        {
            if self.required.contains(field) && !state.definite.contains(field) {
                return Err(failure(
                    &self.unit.source,
                    "T0221",
                    format!("required field `{field}` cannot be read before it is initialized"),
                    node.span,
                ));
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::Name
            && node_text(&self.unit.source, node) == "this"
            && !self.required.is_subset(&state.definite)
        {
            return Err(failure(
                &self.unit.source,
                "T0223",
                "a partially initialized object cannot escape or be passed to another operation",
                node.span,
            ));
        }
        for child in &node.children {
            self.reads(child, state)?;
        }
        Ok(())
    }

    fn may_throw(&self, node: &SyntaxNode) -> bool {
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && function_contract_for_call_with_safety(
                self.package,
                self.unit,
                callee,
                node.is_unsafe_call,
            )
            .is_some_and(|contract| contract.throws || !contract.escaping_throwables.is_empty())
        {
            return true;
        }
        node.children.iter().any(|child| self.may_throw(child))
    }

    fn block(&self, block: &SyntaxNode, state: FieldState) -> Result<Flow, SemanticFailure> {
        let mut output = Flow {
            normal: Some(state),
            ..Flow::default()
        };
        for statement in &block.children {
            let Some(state) = output.normal.take() else {
                break;
            };
            let mut next = self.statement(statement, state)?;
            output.normal = next.normal.take();
            output.breaks.append(&mut next.breaks);
            output.continues.append(&mut next.continues);
            output.failures.append(&mut next.failures);
            output.returns.append(&mut next.returns);
        }
        Ok(output)
    }

    fn branches(&self, node: &SyntaxNode, mut state: FieldState) -> Result<Flow, SemanticFailure> {
        let mut output = Flow::default();
        let mut normals = Vec::new();
        let mut exhaustive = node.kind != SyntaxKind::IfStatement;
        for child in &node.children {
            let block = if child.kind == SyntaxKind::Block {
                Some(child)
            } else {
                child
                    .children
                    .iter()
                    .find(|part| part.kind == SyntaxKind::Block)
            };
            if let Some(block) = block {
                if child.kind == SyntaxKind::ElseClause && child.children.len() == 1 {
                    exhaustive = true;
                }
                for condition in child
                    .children
                    .iter()
                    .filter(|part| part.kind != SyntaxKind::Block)
                {
                    if child.kind == SyntaxKind::ElseClause {
                        self.reads(condition, &mut state)?;
                    }
                }
                let mut branch = self.block(block, state.clone())?;
                normals.extend(branch.normal.take());
                output.breaks.append(&mut branch.breaks);
                output.continues.append(&mut branch.continues);
                output.failures.append(&mut branch.failures);
                output.returns.append(&mut branch.returns);
            } else {
                self.reads(child, &mut state)?;
                if self.may_throw(child) {
                    output.failures.push(state.clone());
                }
            }
        }
        if !exhaustive {
            normals.push(state);
        }
        output.normal = merge(normals);
        Ok(output)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "Statement-kind transfer functions keep successful and exceptional constructor exits in one proof"
    )]
    fn statement(&self, node: &SyntaxNode, mut state: FieldState) -> Result<Flow, SemanticFailure> {
        match node.kind {
            SyntaxKind::IfStatement | SyntaxKind::MatchStatement | SyntaxKind::SelectStatement => {
                self.branches(node, state)
            }
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement => {
                let mut output = Flow::default();
                for child in node
                    .children
                    .iter()
                    .filter(|child| child.kind != SyntaxKind::Block)
                {
                    self.reads(child, &mut state)?;
                    if self.may_throw(child) {
                        output.failures.push(state.clone());
                    }
                }
                let Some(block) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
                else {
                    return Ok(Flow {
                        normal: Some(state),
                        ..output
                    });
                };
                let mut body = self.block(block, state.clone())?;
                let back_edge = merge(
                    body.normal
                        .take()
                        .into_iter()
                        .chain(body.continues.drain(..)),
                );
                if let Some(back_edge) = back_edge {
                    if back_edge.possible != state.possible {
                        return Err(failure(
                            &self.unit.source,
                            "T0222",
                            "a loop cannot initialize a required field more than once",
                            node.span,
                        ));
                    }
                    if !state.definite.is_subset(&back_edge.definite) {
                        return Err(failure(
                            &self.unit.source,
                            "T0221",
                            "a loop cannot leave a required field uninitialized before its next iteration",
                            node.span,
                        ));
                    }
                }
                let always = node.kind == SyntaxKind::WhileStatement
                    && node
                        .children
                        .first()
                        .is_some_and(|condition| node_text(&self.unit.source, condition) == "true");
                let mut exits = body.breaks;
                if !always {
                    exits.push(state);
                }
                output.normal = merge(exits);
                output.failures.append(&mut body.failures);
                output.returns.append(&mut body.returns);
                Ok(output)
            }
            SyntaxKind::TryStatement => {
                let Some(block) = node.children.first() else {
                    return Ok(Flow {
                        normal: Some(state),
                        ..Flow::default()
                    });
                };
                let mut output = self.block(block, state)?;
                let incoming_failures = output.failures.clone();
                let mut normals = output.normal.take().into_iter().collect::<Vec<_>>();
                let mut catches_all = false;
                for clause in node
                    .children
                    .iter()
                    .filter(|child| child.kind == SyntaxKind::CatchClause)
                {
                    catches_all |= clause
                        .children
                        .iter()
                        .all(|child| child.kind == SyntaxKind::Block);
                    let Some(block) = clause
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block)
                    else {
                        continue;
                    };
                    if let Some(incoming) = merge(incoming_failures.clone()) {
                        let mut caught = self.block(block, incoming)?;
                        normals.extend(caught.normal.take());
                        output.breaks.append(&mut caught.breaks);
                        output.continues.append(&mut caught.continues);
                        output.failures.append(&mut caught.failures);
                        output.returns.append(&mut caught.returns);
                    }
                }
                if catches_all {
                    output.failures.drain(..incoming_failures.len());
                }
                output.normal = merge(normals);
                if let Some(finally) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::FinallyClause)
                    .and_then(|clause| clause.children.first())
                {
                    let departures = [
                        output.normal.take().into_iter().collect::<Vec<_>>(),
                        std::mem::take(&mut output.breaks),
                        std::mem::take(&mut output.continues),
                        std::mem::take(&mut output.failures),
                        std::mem::take(&mut output.returns),
                    ];
                    let mut completed = Flow::default();
                    let mut normal = Vec::new();
                    for (kind, states) in departures.into_iter().enumerate() {
                        for state in states {
                            let mut finished = self.block(finally, state)?;
                            if let Some(state) = finished.normal.take() {
                                match kind {
                                    0 => normal.push(state),
                                    1 => completed.breaks.push(state),
                                    2 => completed.continues.push(state),
                                    3 => completed.failures.push(state),
                                    4 => completed.returns.push(state),
                                    _ => unreachable!("five constructor control-flow exits"),
                                }
                            }
                            completed.breaks.append(&mut finished.breaks);
                            completed.continues.append(&mut finished.continues);
                            completed.failures.append(&mut finished.failures);
                            completed.returns.append(&mut finished.returns);
                        }
                    }
                    completed.normal = merge(normal);
                    output = completed;
                }
                Ok(output)
            }
            SyntaxKind::ReturnStatement => {
                self.reads(node, &mut state)?;
                Ok(Flow {
                    returns: vec![state],
                    ..Flow::default()
                })
            }
            SyntaxKind::ThrowStatement => {
                self.reads(node, &mut state)?;
                Ok(Flow {
                    failures: vec![state],
                    ..Flow::default()
                })
            }
            SyntaxKind::BreakStatement => Ok(Flow {
                breaks: vec![state],
                ..Flow::default()
            }),
            SyntaxKind::ContinueStatement => Ok(Flow {
                continues: vec![state],
                ..Flow::default()
            }),
            SyntaxKind::Assignment => {
                let field = node
                    .children
                    .first()
                    .and_then(|target| self.direct_field(target));
                let mut failures = Vec::new();
                if let Some(value) = node.children.get(1) {
                    self.reads(value, &mut state)?;
                    if self.may_throw(value) {
                        failures.push(state.clone());
                    }
                }
                if let Some(field) = field.filter(|field| self.required.contains(*field)) {
                    if state.possible.contains(field) {
                        return Err(failure(
                            &self.unit.source,
                            "T0222",
                            format!(
                                "required field `{field}` must be initialized exactly once during construction",
                            ),
                            node.span,
                        ));
                    }
                    state.definite.insert(field.to_owned());
                    state.possible.insert(field.to_owned());
                } else if let Some(target) = node.children.first() {
                    self.reads(target, &mut state)?;
                }
                Ok(Flow {
                    normal: Some(state),
                    failures,
                    ..Flow::default()
                })
            }
            _ => {
                self.reads(node, &mut state)?;
                let failures = if self.may_throw(node) {
                    vec![state.clone()]
                } else {
                    Vec::new()
                };
                Ok(Flow {
                    normal: Some(state),
                    failures,
                    ..Flow::default()
                })
            }
        }
    }
}

pub(super) fn validate(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let mut proofs = Vec::new();
        for object in unit.descriptors.iter().filter(|object| {
            object.kind == ObjectKind::Class
                && object.span.file == unit.source.id()
                && package
                    .projection
                    .item(&object.identity.namespace, &object.identity.name)
                    .is_none()
        }) {
            let fields = effective_object_fields(package, object);
            let required = fields
                .iter()
                .filter(|field| field.field.required)
                .map(|field| field.name.clone())
                .collect::<BTreeSet<_>>();
            if required.is_empty() {
                continue;
            }
            let constructor = unit
                .functions
                .iter()
                .find(|function| {
                    function.name == "construct"
                        && function
                            .owner_identity
                            .as_ref()
                            .is_some_and(|owner| owner.base() == object.identity.base())
                })
                .ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0220",
                        format!(
                            "class `{}` requires a constructor to initialize {}",
                            object.name,
                            required.iter().cloned().collect::<Vec<_>>().join(", "),
                        ),
                        object.span,
                    )
                })?;
            let declaration =
                super::ownership::find_node_by_span(&unit.tree.root, constructor.span)
                    .expect("source constructor contract has its declaration");
            let block = declaration
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
                .expect("source constructor declaration has a body");
            let analysis = ConstructorAnalysis {
                package,
                unit,
                required: required.clone(),
                fields: fields.iter().map(|field| field.name.clone()).collect(),
            };
            let flow = analysis.block(block, FieldState::default())?;
            if let Some(state) = flow.normal {
                analysis.complete(&state, block.span)?;
            }
            for state in &flow.returns {
                analysis.complete(state, block.span)?;
            }
            proofs.push(RequiredInitProof {
                constructor: constructor.span,
                initialized_fields: required,
            });
        }
        for proof in proofs {
            package.units[index].required_init_proofs.insert(
                (
                    proof.constructor.file,
                    proof.constructor.start,
                    proof.constructor.end,
                ),
                proof,
            );
        }
    }
    Ok(())
}
