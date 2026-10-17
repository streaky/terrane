//! Best-effort alias proofs for non-reentrant synchronous source-method guards.
//! This pass tracks copy-time identities, not just declaration provenance. Unknown
//! expressions and writes inside compound control flow discard proofs; they never
//! become guessed aliases. Summaries describe direct receiver/argument parameter
//! pairs only, and are applied once rather than propagated through call chains.
use super::prelude::{
    BTreeMap, BTreeSet, FunctionContract, InvocationMode, SemanticFailure, SemanticPackage,
    SemanticUnit, Span, SyntaxKind, SyntaxNode, ValueType, binding_initializer, failure,
    find_node_by_span, function_contract_for_call, infer_value_type, is_function_node, node_text,
    resolve_call_parameter, unary_operator_text,
};

type Owners = BTreeMap<Span, Span>;
type Pairs = BTreeSet<(usize, usize)>;
type Summaries = BTreeMap<Span, Pairs>;

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    let mut summaries = Summaries::new();
    for unit in &package.units {
        for contract in &unit.functions {
            if contract.is_async || contract.span.file != unit.source.id() {
                continue;
            }
            let Some(body) = function_body(unit, contract) else {
                continue;
            };
            let mut pass = Pass {
                package,
                unit,
                contract,
                summaries: None,
                pairs: Pairs::new(),
            };
            pass.walk(body, &mut parameter_owners(contract))?;
            if !pass.pairs.is_empty() {
                summaries.insert(contract.span, pass.pairs);
            }
        }
    }
    for unit in &package.units {
        for contract in &unit.functions {
            if contract.is_async || contract.span.file != unit.source.id() {
                continue;
            }
            let Some(body) = function_body(unit, contract) else {
                continue;
            };
            let mut pass = Pass {
                package,
                unit,
                contract,
                summaries: Some(&summaries),
                pairs: Pairs::new(),
            };
            pass.walk(body, &mut parameter_owners(contract))?;
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
        .find(|child| child.kind == SyntaxKind::Block)
}

fn parameter_owners(contract: &FunctionContract) -> Owners {
    contract
        .parameters
        .iter()
        .map(|parameter| (parameter.span, parameter.span))
        .collect()
}

struct Pass<'a> {
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    contract: &'a FunctionContract,
    summaries: Option<&'a Summaries>,
    pairs: Pairs,
}

impl<'a> Pass<'a> {
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
                    .and_then(|symbol| symbol.declaration_span.or(symbol.binding_span))
            })
    }

    fn source_contract(&self, callee: &SyntaxNode) -> Option<&'a FunctionContract> {
        let contract = function_contract_for_call(self.package, self.unit, callee)?;
        self.package
            .units
            .iter()
            .any(|unit| {
                unit.source.id() == contract.span.file
                    && unit
                        .functions
                        .iter()
                        .any(|source| source.span == contract.span)
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
            SyntaxKind::CallExpression => {
                let callee = self.source_contract(node.children.first()?)?;
                if callee.is_async
                    || !matches!(callee.return_type, Some(ValueType::SharedReference(_)))
                {
                    return None;
                }
                let lender = self.unit.reference_return_lenders.get(&(
                    callee.span.file,
                    callee.span.start,
                    callee.span.end,
                ))?;
                self.owner(argument(self.unit, node, callee, *lender)?, owners)
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
        let receiver_type = infer_value_type(self.unit, receiver, &self.unit.typed_bindings)
            .ok()
            .flatten()?;
        (matches!(receiver_type, ValueType::SharedReference(_))
            || self
                .package
                .reference_owner_uses_shared_storage(self.unit, owner))
        .then_some(owner)
    }

    fn pair(&mut self, left: Span, right: Span, at: Span) -> Result<(), SemanticFailure> {
        if left == right {
            return Err(failure(
                &self.unit.source,
                "T0143",
                "a guarded method cannot receive an alias of its receiver's shared storage (directly or through a source helper)",
                at,
            ));
        }
        let parameter = |owner| {
            self.contract.parameters.iter().position(|parameter| {
                parameter.span == owner
                    && matches!(parameter.value_type, Some(ValueType::SharedReference(_)))
            })
        };
        if let (Some(left), Some(right)) = (parameter(left), parameter(right)) {
            self.pairs.insert((left, right));
        }
        Ok(())
    }

    fn call(&mut self, node: &SyntaxNode, owners: &Owners) -> Result<(), SemanticFailure> {
        let Some(callee) = node.children.first() else {
            return Ok(());
        };
        let Some(contract) = self.source_contract(callee) else {
            return Ok(());
        };
        if let Some(receiver) = self.guarded(callee, owners) {
            for (index, parameter) in contract.parameters.iter().enumerate() {
                if !matches!(parameter.value_type, Some(ValueType::SharedReference(_))) {
                    continue;
                }
                if let Some(value) = argument(self.unit, node, contract, index)
                    && let Some(owner) = self.owner(value, owners)
                {
                    self.pair(receiver, owner, value.span)?;
                }
            }
        }
        if let Some(pairs) = self
            .summaries
            .and_then(|summaries| summaries.get(&contract.span))
        {
            for &(left, right) in pairs {
                if let (Some(left), Some(right)) = (
                    argument(self.unit, node, contract, left),
                    argument(self.unit, node, contract, right),
                ) && let (Some(left), Some(right)) =
                    (self.owner(left, owners), self.owner(right, owners))
                {
                    self.pair(left, right, node.span)?;
                }
            }
        }
        Ok(())
    }

    fn walk(&mut self, node: &SyntaxNode, owners: &mut Owners) -> Result<(), SemanticFailure> {
        if is_function_node(node) {
            return Ok(());
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && let Some(target) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            && node.children.first().is_some_and(|first| {
                node.kind == SyntaxKind::Binding || first.kind == SyntaxKind::Name
            })
            && let Some(identity) = self.binding(target)
        {
            let value = binding_initializer(node);
            if let Some(value) = value {
                self.walk(value, owners)?;
            }
            let value_type = self.unit.flow_binding_types.get(&identity).or_else(|| {
                self.unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == identity)
                    .map(|binding| &binding.value_type)
            });
            let owner = if matches!(
                value_type,
                Some(ValueType::Reference(_) | ValueType::SharedReference(_))
            ) {
                value.and_then(|value| self.owner(value, owners))
            } else if matches!(value_type, Some(ValueType::Object(_))) {
                Some(node.span)
            } else {
                None
            };
            owners.remove(&identity);
            if let Some(owner) = owner {
                owners.insert(identity, owner);
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression {
            self.call(node, owners)?;
        }
        if node.kind != SyntaxKind::Block
            && node
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::Block)
        {
            // Without a full control-flow proof, do not retain an alias modified
            // by a branch, loop, exception/completion path, or scoped operation.
            self.forget_writes(node, owners);
            for child in &node.children {
                self.walk(child, &mut owners.clone())?;
            }
        } else {
            for child in &node.children {
                self.walk(child, owners)?;
                if matches!(
                    child.kind,
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
            && let Some(target) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            && node.children.first().is_some_and(|first| {
                node.kind == SyntaxKind::Binding || first.kind == SyntaxKind::Name
            })
            && let Some(identity) = self.binding(target)
        {
            owners.remove(&identity);
        }
        for child in &node.children {
            self.forget_writes(child, owners);
        }
    }
}

fn argument<'a>(
    unit: &SemanticUnit,
    call: &'a SyntaxNode,
    contract: &FunctionContract,
    index: usize,
) -> Option<&'a SyntaxNode> {
    let mut positional = 0;
    let mut named = false;
    for value in &call.children.get(1)?.children {
        let name = value
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && value.children.len() > 1);
        if let Ok(parameter) =
            resolve_call_parameter(unit, value, name, contract, &mut positional, &mut named)
            && parameter.span == contract.parameters.get(index)?.span
        {
            return value.children.last().or(Some(value));
        }
    }
    None
}
