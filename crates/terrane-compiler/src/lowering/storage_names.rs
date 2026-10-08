use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Span,
    semantics::{SemanticUnit, TypedBinding, ValueType},
};

/// Identifier rendering only: canonical binding identities and carrier types remain
/// owned by reaching-flow analysis. The unit caches this plan until that analysis
/// rebuilds its bindings; every backend and native-source consumer shares it.
#[derive(Clone, Debug)]
pub(crate) struct StorageNames {
    bindings: BTreeMap<Span, String>,
    parameters: BTreeMap<Span, String>,
}

impl StorageNames {
    pub(crate) fn for_unit(unit: &SemanticUnit) -> &Self {
        unit.rust_storage_names.get_or_init(|| Self::new(unit))
    }

    fn new(unit: &SemanticUnit) -> Self {
        let canonical = |span: Span| {
            unit.flow_binding_ids
                .get(&(span.file, span.start, span.end))
                .copied()
                .unwrap_or(span)
        };
        let parameters = unit
            .functions
            .iter()
            .flat_map(|function| &function.parameters)
            .map(|parameter| parameter.span)
            .collect::<BTreeSet<_>>();
        let mut scopes = BTreeMap::<Option<Span>, BTreeMap<Span, &TypedBinding>>::new();
        let mut authored_stems = BTreeMap::<Option<Span>, BTreeSet<String>>::new();
        for binding in &unit.typed_bindings {
            authored_stems
                .entry(binding.scope)
                .or_default()
                .insert(super::helpers::rust_name(&binding.name));
            if canonical(binding.span) == binding.span || parameters.contains(&binding.span) {
                scopes
                    .entry(binding.scope)
                    .or_default()
                    .insert(binding.span, binding);
            }
        }
        let mut bindings = BTreeMap::new();
        let mut inputs = BTreeMap::new();
        let mut allocated_scopes = BTreeMap::<Span, BTreeSet<String>>::new();
        for (scope, storage) in scopes {
            // Reserve this function's authored stems and nested-function stems
            // before allocating ordinals. Unrelated functions cannot rename its
            // locals, and outer versions cannot steal nested authored `x_2`s.
            let mut reserved = BTreeSet::new();
            for (owner, stems) in &authored_stems {
                let relevant = *owner == scope
                    || matches!((scope, *owner), (Some(outer), Some(inner))
                        if outer.file == inner.file
                            && outer.start <= inner.start && inner.end <= outer.end);
                if relevant {
                    reserved.extend(stems.iter().cloned());
                }
            }
            let mut used = BTreeSet::new();
            if let Some(scope) = scope {
                // Hoisted locals must not shadow callable items that remain
                // visible before the source declaration, including projections.
                if let Some(lexical) = unit.scopes.iter().find(|lexical| lexical.span == scope) {
                    used.extend(
                        lexical
                            .symbols
                            .values()
                            .flatten()
                            .filter(|symbol| symbol.kind == crate::semantics::SymbolKind::Function)
                            .map(|symbol| super::helpers::rust_name(&symbol.name)),
                    );
                }
                for (outer, names) in &allocated_scopes {
                    if outer.file == scope.file
                        && outer.start <= scope.start
                        && scope.end <= outer.end
                    {
                        used.extend(names.iter().cloned());
                    }
                }
            }
            let mut ordinals = BTreeMap::<String, usize>::new();
            for (span, binding) in storage {
                let stem = super::helpers::rust_name(&binding.name);
                let name = allocate(&stem, &reserved, &mut used, &mut ordinals);
                if parameters.contains(&span) {
                    inputs.insert(span, name.clone());
                    if canonical(span) != span {
                        continue;
                    }
                    if matches!(
                        unit.flow_binding_types.get(&span),
                        Some(ValueType::Union(_))
                    ) {
                        bindings.insert(span, allocate(&stem, &reserved, &mut used, &mut ordinals));
                        continue;
                    }
                }
                bindings.insert(span, name);
            }
            if let Some(scope) = scope {
                allocated_scopes.insert(scope, used);
            }
        }
        // Repeated declarations and per-use aliases refer to the same physical
        // name, without allocating another version or interpreting source types.
        for binding in &unit.typed_bindings {
            let identity = canonical(binding.span);
            if identity != binding.span {
                let name = bindings
                    .get(&identity)
                    .expect("canonical binding has a storage name")
                    .clone();
                bindings.insert(binding.span, name);
            }
        }
        Self {
            bindings,
            parameters: inputs,
        }
    }

    pub(crate) fn binding(&self, span: Span) -> &str {
        self.bindings
            .get(&span)
            .expect("typed binding has a storage name")
    }

    pub(crate) fn at(&self, span: Span) -> Option<&str> {
        self.bindings.get(&span).map(String::as_str)
    }

    pub(crate) fn parameter(&self, span: Span) -> Option<&str> {
        self.parameters.get(&span).map(String::as_str)
    }
}

fn allocate(
    stem: &str,
    reserved: &BTreeSet<String>,
    used: &mut BTreeSet<String>,
    ordinals: &mut BTreeMap<String, usize>,
) -> String {
    let ordinal = ordinals.entry(stem.to_owned()).or_insert(1);
    if *ordinal == 1 && used.insert(stem.to_owned()) {
        return stem.to_owned();
    }
    loop {
        *ordinal += 1;
        let candidate = format!("{stem}_{ordinal}");
        if !reserved.contains(&candidate) && used.insert(candidate.clone()) {
            return candidate;
        }
    }
}
