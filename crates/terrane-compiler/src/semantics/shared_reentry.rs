//! Cheap alias checks for obvious synchronous guarded-method reentry.
//! Runtime borrow checking handles cases requiring interprocedural/control-flow analysis.
use super::prelude::{
    BTreeMap, FunctionContract, InvocationMode, SemanticFailure, SemanticPackage, SemanticUnit,
    Span, SyntaxKind, SyntaxNode, ValueType, binding_initializer, failure, find_node_by_span,
    function_contract_for_call, infer_value_type, is_function_node, node_text,
    resolve_call_parameter, unary_operator_text,
};

type Owners = BTreeMap<Span, Span>;

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        for contract in &unit.functions {
            if contract.is_async || contract.span.file != unit.source.id() {
                continue;
            }
            let Some(body) = function_body(unit, contract) else {
                continue;
            };
            let pass = Pass { package, unit };
            pass.walk(
                body,
                &mut contract
                    .parameters
                    .iter()
                    .map(|p| (p.span, p.span))
                    .collect(),
            )?;
        }
    }
    Ok(())
}

fn function_body<'a>(
    unit: &'a SemanticUnit,
    contract: &FunctionContract,
) -> Option<&'a SyntaxNode> {
    find_node_by_span(&unit.tree.root, contract.span)?
        .children
        .iter()
        .find(|c| c.kind == SyntaxKind::Block)
}

struct Pass<'a> {
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
}
impl Pass<'_> {
    fn binding(&self, node: &SyntaxNode) -> Option<Span> {
        if node.kind != SyntaxKind::Name {
            return None;
        }
        self.unit
            .flow_binding_ids
            .get(&(node.span.file, node.span.start, node.span.end))
            .copied()
            .or_else(|| {
                self.package
                    .resolve_name_at(
                        self.unit,
                        node.span.start,
                        node_text(&self.unit.source, node),
                    )
                    .and_then(|s| s.declaration_span.or(s.binding_span))
            })
    }
    fn source_contract<'a>(&'a self, callee: &SyntaxNode) -> Option<&'a FunctionContract> {
        let contract = function_contract_for_call(self.package, self.unit, callee)?;
        self.package
            .units
            .iter()
            .any(|u| {
                u.source.id() == contract.span.file
                    && u.functions.iter().any(|f| f.span == contract.span)
            })
            .then_some(contract)
    }
    fn owner(&self, node: &SyntaxNode, owners: &Owners) -> Option<Span> {
        match node.kind {
            SyntaxKind::Name => owners.get(&self.binding(node)?).copied(),
            SyntaxKind::GroupExpression => self.owner(node.children.first()?, owners),
            SyntaxKind::UnaryExpression
                if matches!(
                    unary_operator_text(self.unit, node).as_deref(),
                    Some("ref" | "shared ref")
                ) =>
            {
                self.owner(node.children.last()?, owners)
            }
            _ => None,
        }
    }
    fn guarded(&self, callee: &SyntaxNode, owners: &Owners) -> Option<Span> {
        if callee.kind != SyntaxKind::MemberExpression {
            return None;
        }
        let receiver = callee.children.first()?;
        let contract = self.source_contract(callee)?;
        if contract.owner.is_none()
            || contract.is_static
            || contract.is_async
            || !matches!(
                contract.written_invocation_mode,
                InvocationMode::Shared | InvocationMode::Mutable
            )
        {
            return None;
        }
        let owner = self.owner(receiver, owners)?;
        let ty = infer_value_type(self.unit, receiver, &self.unit.typed_bindings)
            .ok()
            .flatten()?;
        (matches!(ty, ValueType::SharedReference(_))
            || self
                .package
                .reference_owner_uses_shared_storage(self.unit, owner))
        .then_some(owner)
    }
    fn call(&self, node: &SyntaxNode, owners: &Owners) -> Result<(), SemanticFailure> {
        let Some(callee) = node.children.first() else {
            return Ok(());
        };
        let Some(contract) = self.source_contract(callee) else {
            return Ok(());
        };
        if let Some(receiver) = self.guarded(callee, owners) {
            for (i, p) in contract.parameters.iter().enumerate() {
                if matches!(p.value_type, Some(ValueType::SharedReference(_)))
                    && let Some(v) = argument(self.unit, node, contract, i)
                    && self.owner(v, owners) == Some(receiver)
                {
                    return Err(failure(
                        &self.unit.source,
                        "T0143",
                        "a guarded method cannot receive an alias of its receiver's shared storage",
                        v.span,
                    ));
                }
            }
        }
        Ok(())
    }
    fn walk(&self, node: &SyntaxNode, owners: &mut Owners) -> Result<(), SemanticFailure> {
        if is_function_node(node) {
            return Ok(());
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && let Some(t) = node.children.iter().find(|c| c.kind == SyntaxKind::Name)
            && node
                .children
                .first()
                .is_some_and(|f| node.kind == SyntaxKind::Binding || f.kind == SyntaxKind::Name)
            && let Some(id) = self.binding(t)
        {
            let value = binding_initializer(node);
            if let Some(v) = value {
                self.walk(v, owners)?;
            }
            let ty = self.unit.flow_binding_types.get(&id).or_else(|| {
                self.unit
                    .typed_bindings
                    .iter()
                    .find(|b| b.span == id)
                    .map(|b| &b.value_type)
            });
            let owner = if matches!(
                ty,
                Some(ValueType::Reference(_) | ValueType::SharedReference(_))
            ) {
                value.and_then(|v| self.owner(v, owners))
            } else if matches!(ty, Some(ValueType::Object(_))) {
                Some(node.span)
            } else {
                None
            };
            owners.remove(&id);
            if let Some(x) = owner {
                owners.insert(id, x);
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression {
            self.call(node, owners)?;
        }
        if node.kind != SyntaxKind::Block
            && node.children.iter().any(|c| c.kind == SyntaxKind::Block)
        {
            self.forget_writes(node, owners);
            for c in &node.children {
                self.walk(c, &mut owners.clone())?;
            }
        } else {
            for c in &node.children {
                self.walk(c, owners)?;
                if matches!(
                    c.kind,
                    SyntaxKind::ReturnStatement
                        | SyntaxKind::ThrowStatement
                        | SyntaxKind::BreakStatement
                        | SyntaxKind::ContinueStatement
                ) {
                    break;
                }
            }
        }
        Ok(())
    }
    fn forget_writes(&self, node: &SyntaxNode, owners: &mut Owners) {
        if is_function_node(node) {
            return;
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && let Some(t) = node.children.iter().find(|c| c.kind == SyntaxKind::Name)
            && let Some(id) = self.binding(t)
        {
            owners.remove(&id);
        }
        for c in &node.children {
            self.forget_writes(c, owners);
        }
    }
}
fn argument<'a>(
    unit: &SemanticUnit,
    call: &'a SyntaxNode,
    c: &FunctionContract,
    index: usize,
) -> Option<&'a SyntaxNode> {
    let mut p = 0;
    let mut named = false;
    for v in &call.children.get(1)?.children {
        let name = v
            .children
            .first()
            .filter(|x| x.kind == SyntaxKind::Name && v.children.len() > 1);
        if let Ok(param) = resolve_call_parameter(unit, v, name, c, &mut p, &mut named)
            && param.span == c.parameters.get(index)?.span
        {
            return v.children.last().or(Some(v));
        }
    }
    None
}
