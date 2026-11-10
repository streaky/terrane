use std::cell::RefCell;
use std::collections::HashMap;

use super::{SemanticFailure, SemanticUnit, SyntaxKind, SyntaxNode, TypedBinding, ValueType};

// A root inference call immutably borrows this unit and binding slice until the
// scope drops. Only that exact live context is cached: nested temporary scopes
// may reuse allocation addresses after they drop, but cannot reuse these still
// borrowed root objects. No pointer is dereferenced or retained across calls.
// Node-address keys additionally require every queried node to remain alive for
// the entire root call. Inference must not synthesize and drop temporary nodes
// inside that scope: allocator address reuse could otherwise produce a stale hit.
type Context = (usize, usize, usize);

struct InferenceMemo {
    context: Context,
    values: HashMap<usize, Option<ValueType>>,
}

thread_local! {
    static MEMO: RefCell<Option<InferenceMemo>> = const { RefCell::new(None) };
    #[cfg(test)]
    static WORK_LIMIT: RefCell<Option<usize>> = const { RefCell::new(None) };
}

// Break returns the cached answer; Continue performs inference. Keep the value
// inline: boxing cache hits would allocate on this hot path.
pub(super) type CachedInference = std::ops::ControlFlow<Option<ValueType>>;

pub(super) struct InferenceScope {
    owner: bool,
    context: Context,
    node: Option<usize>,
}

impl InferenceScope {
    pub(super) fn enter(unit: &SemanticUnit, node: &SyntaxNode, bindings: &[TypedBinding]) -> Self {
        let context = (
            std::ptr::from_ref(unit) as usize,
            bindings.as_ptr() as usize,
            bindings.len(),
        );
        let node = matches!(
            node.kind,
            SyntaxKind::CallExpression | SyntaxKind::MemberExpression
        )
        .then_some(std::ptr::from_ref(node) as usize);
        let owner = node.is_some()
            && MEMO.with_borrow_mut(|memo| {
                if memo.is_some() {
                    false
                } else {
                    *memo = Some(InferenceMemo {
                        context,
                        values: HashMap::new(),
                    });
                    true
                }
            });
        Self {
            owner,
            context,
            node,
        }
    }

    pub(super) fn lookup(&self) -> CachedInference {
        let Some(node) = self.node else {
            return CachedInference::Continue(());
        };
        MEMO.with_borrow(|memo| {
            match memo
                .as_ref()
                .filter(|memo| memo.context == self.context)
                .and_then(|memo| memo.values.get(&node))
            {
                Some(value) => CachedInference::Break(value.clone()),
                None => CachedInference::Continue(()),
            }
        })
    }

    pub(super) fn record(&self, value: Option<&ValueType>) {
        let Some(node) = self.node else { return };
        MEMO.with_borrow_mut(|memo| {
            if let Some(memo) = memo.as_mut().filter(|memo| memo.context == self.context) {
                memo.values.insert(node, value.cloned());
            }
        });
    }
}

impl Drop for InferenceScope {
    fn drop(&mut self) {
        if self.owner {
            MEMO.with_borrow_mut(|memo| *memo = None);
        }
    }
}

#[cfg(test)]
pub(super) fn count_computation() {
    WORK_LIMIT.with_borrow_mut(|limit| {
        if let Some(remaining) = limit {
            assert!(*remaining > 0, "expression inference work budget exceeded");
            *remaining -= 1;
        }
    });
}

// Follow only the receiver dependency that destination-directed call detection
// already evaluates before the remaining call probes. Cached selected calls are
// barriers: their receiver must not be eagerly validated merely to warm a memo.
pub(super) fn infer_receiver_type(
    unit: &SemanticUnit,
    receiver: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    fn dependency<'a>(unit: &SemanticUnit, node: &'a SyntaxNode) -> Option<&'a SyntaxNode> {
        if node.kind == SyntaxKind::GroupExpression {
            return node.children.first();
        }
        let key = (node.span.file, node.span.start, node.span.end);
        if node.kind != SyntaxKind::CallExpression
            || unit.projected_call_specializations.contains_key(&key)
            || unit.selected_expression_types.contains_key(&key)
        {
            return None;
        }
        let callee = node.children.first()?;
        (callee.kind == SyntaxKind::MemberExpression)
            .then(|| callee.children.first())
            .flatten()
    }

    if dependency(unit, receiver).is_none() {
        return super::infer_value_type(unit, receiver, bindings);
    }
    let mut memo_root = receiver;
    while memo_root.kind == SyntaxKind::GroupExpression {
        let Some(inner) = memo_root.children.first() else {
            break;
        };
        memo_root = inner;
    }
    let _scope = InferenceScope::enter(unit, memo_root, bindings);
    let mut pending = Vec::new();
    let mut current = receiver;
    while matches!(
        InferenceScope::enter(unit, current, bindings).lookup(),
        CachedInference::Continue(())
    ) {
        let Some(next) = dependency(unit, current) else {
            break;
        };
        pending.push(current);
        current = next;
    }
    let mut result = super::infer_value_type(unit, current, bindings)?;
    for node in pending.into_iter().rev() {
        result = super::infer_value_type(unit, node, bindings)?;
    }
    Ok(result)
}

#[cfg(test)]
pub(super) fn with_work_limit<T>(limit: usize, operation: impl FnOnce() -> T) -> T {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            WORK_LIMIT.with_borrow_mut(|limit| *limit = None);
        }
    }
    WORK_LIMIT.with_borrow_mut(|current| {
        assert!(current.is_none());
        *current = Some(limit);
    });
    let _reset = Reset;
    operation()
}

#[cfg(test)]
mod tests {
    use super::{InferenceScope, with_work_limit};
    use crate::syntax::{SyntaxKind, SyntaxNode};
    use crate::{
        ArtifactKind, BuildToolchain, CapabilityProfile, ExecutorProfile, Package, PackagePurpose,
        ReflectionProfile, ScalarType, SourceFile, SourceRole, SourceUnit, ValueType, analyze,
    };
    use std::collections::{BTreeMap, BTreeSet};

    fn source_package(text: String) -> Package {
        Package {
            identity: "inference-work".to_owned(),
            root: ".".into(),
            prelude: true,
            reflection: ReflectionProfile::Ordinary,
            executor: ExecutorProfile::Threaded,
            artifact: ArtifactKind::Executable,
            profile: CapabilityProfile::unrestricted(),
            purpose: PackagePurpose::Production,
            testing: crate::testing::TestConfiguration::conventional(
                CapabilityProfile::unrestricted(),
            ),
            build_toolchain: BuildToolchain::Pinned,
            consumer_configs: Vec::new(),
            consumer_tools: BTreeMap::new(),
            dependency_roots: BTreeMap::new(),
            units: vec![SourceUnit {
                relative_path: "main.trn".into(),
                source: SourceFile::new(0, "main.trn".into(), text),
                role: SourceRole::Production,
                expected_namespace: None,
                prelude: true,
            }],
            rust_dependencies: Vec::new(),
            authored_rust_modules: Vec::new(),
            terrane_dependencies: Vec::new(),
            dependency_manifests: Vec::new(),
            library_source_ids: BTreeSet::new(),
        }
    }

    #[test]
    fn long_chains_reuse_receivers() {
        let stages = "        .trim;\n".repeat(30);
        let source = format!(
            "namespace chain-work\nfunction probe string; value string\n    result = value\n{stages}    return result\n"
        );
        // A work count, not a wall-clock timeout: exponential receiver probes
        // fail promptly even on slow CI, while all semantic passes still run.
        let analyzed = with_work_limit(100_000, || analyze(&source_package(source))).unwrap();
        assert_eq!(
            analyzed.units[0]
                .typed_bindings
                .iter()
                .find(|binding| binding.name == "result")
                .unwrap()
                .value_type,
            ValueType::Scalar(ScalarType::String)
        );
    }

    #[test]
    fn binding_contexts_do_not_leak() {
        fn member_call(node: &SyntaxNode) -> Option<&SyntaxNode> {
            if node.kind == SyntaxKind::CallExpression
                && node
                    .children
                    .first()
                    .is_some_and(|callee| callee.kind == SyntaxKind::MemberExpression)
            {
                Some(node)
            } else {
                node.children.iter().find_map(member_call)
            }
        }
        let mut analyzed = analyze(&source_package(
            "namespace scoped-work\nfunction probe string; value string\n    return value.trim;\n"
                .to_owned(),
        ))
        .unwrap();
        let unit = &mut analyzed.units[0];
        // Before reaching-flow selection, the caller's lexical binding slice
        // is authoritative; final flow facts would intentionally override it.
        unit.flow_types.clear();
        unit.selected_expression_types.clear();
        let call = member_call(&unit.tree.root).unwrap();
        let mut changed = unit.typed_bindings.clone();
        changed
            .iter_mut()
            .find(|binding| binding.name == "value")
            .unwrap()
            .value_type = ValueType::Scalar(ScalarType::Int);
        {
            // Keep the original context's memo live while inferring the exact
            // same syntax with a temporary, differently typed binding scope.
            let _root = InferenceScope::enter(unit, call, &unit.typed_bindings);
            assert_eq!(
                super::super::infer_value_type(unit, call, &unit.typed_bindings).unwrap(),
                Some(ValueType::Scalar(ScalarType::String))
            );
            assert_eq!(
                super::super::infer_value_type(unit, call, &changed)
                    .unwrap_err()
                    .diagnostics[0]
                    .code,
                "T0032"
            );
            assert_eq!(
                super::super::infer_value_type(unit, call, &unit.typed_bindings).unwrap(),
                Some(ValueType::Scalar(ScalarType::String))
            );
        }
        // No successful result survives root completion either.
        assert_eq!(
            super::super::infer_value_type(unit, call, &changed)
                .unwrap_err()
                .diagnostics[0]
                .code,
            "T0032"
        );
    }
}
