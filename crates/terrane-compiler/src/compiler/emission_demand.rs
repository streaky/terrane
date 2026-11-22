//! Conservative reachability over emitted Rust items, not over semantic bodies.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use proc_macro2::{TokenStream, TokenTree};

use crate::rust_ir::{Item, Program};
use crate::syntax::{SyntaxKind, SyntaxNode};

use super::{CompilerOptions, cache};

pub(super) fn select(
    package: &crate::Package,
    semantic: &crate::SemanticPackage,
    options: CompilerOptions,
    program: &mut Program,
) {
    if package.artifact != crate::ArtifactKind::Executable
        || options.debug_build != super::DebugBuild::Disabled
        || options.retain_rust_definitions
    {
        return;
    }
    // Authored Rust and projected macro expansions can name symbols invisible
    // to the generated identifier graph. Keep the ordinary path in these cases.
    if !package.authored_rust_modules.is_empty()
        || semantic
            .units
            .iter()
            .any(|unit| contains_inline_rust(&unit.tree.root))
        || semantic
            .projection
            .dependencies
            .iter()
            .flat_map(|dep| &dep.items)
            .any(|item| {
                matches!(
                    item.kind,
                    crate::rust_interop::projection::ProjectedKind::Macro(_)
                )
            })
    {
        crate::compilation_progress::note(
            "emission demand skipped: opaque Rust",
            &package.identity,
        );
        return;
    }
    // A raw item may contain edges we cannot discover. Check before splitting
    // runtime fragments so declining pruning leaves the program untouched.
    if std::iter::once(program.globals.as_slice())
        .chain(program.runtime.iter().map(|module| module.items.as_slice()))
        .chain(program.modules.iter().map(|module| module.items.as_slice()))
        .flatten()
        .any(Item::is_opaque)
    {
        return;
    }
    let stage =
        crate::compilation_progress::start("reconciling emission demand", &package.identity);
    let cache_dir = crate::cache_scope::enabled()
        .then(|| std::env::var_os("TERRANE_DEMAND_CACHE"))
        .flatten()
        .filter(|path| !path.is_empty())
        .map(PathBuf::from);
    for module in &mut program.runtime {
        module.items = std::mem::take(&mut module.items)
            .into_iter()
            .flat_map(|mut item| item.split_runtime_functions().unwrap_or_else(|| vec![item]))
            .collect();
    }
    let groups = std::iter::once(program.globals.as_slice())
        .chain(program.runtime.iter().map(|module| module.items.as_slice()))
        .chain(program.modules.iter().map(|module| module.items.as_slice()))
        .collect::<Vec<_>>();
    let mut hits = 0;
    let mut references = Vec::new();
    for items in &groups {
        let (summary, hit) = summarize_group(items, cache_dir.as_deref());
        hits += usize::from(hit);
        references.extend(summary);
    }
    let candidates = groups
        .into_iter()
        .flatten()
        .map(Item::discardable_function)
        .collect::<Vec<_>>();
    let retained = reachable(&candidates, &references);
    let removed = retained.iter().filter(|keep| !**keep).count();
    let mut keep = retained.into_iter();
    for items in std::iter::once(&mut program.globals)
        .chain(program.runtime.iter_mut().map(|module| &mut module.items))
        .chain(program.modules.iter_mut().map(|module| &mut module.items))
    {
        items.retain(|_| keep.next().expect("one decision per emitted item"));
    }
    program.runtime.retain(|module| !module.items.is_empty());
    crate::compilation_progress::note(
        "reconciled emission demand",
        format_args!("{removed} omitted functions; {hits} reused summaries"),
    );
    stage.finish();
}

fn reachable(candidates: &[Option<String>], references: &[Vec<String>]) -> Vec<bool> {
    let mut definitions = BTreeMap::<&str, Vec<usize>>::new();
    let mut pending = Vec::new();
    for (index, name) in candidates.iter().enumerate() {
        if let Some(name) = name {
            definitions
                .entry(name.trim_start_matches("r#"))
                .or_default()
                .push(index);
        } else {
            pending.push(index);
        }
    }
    let mut retained = vec![false; candidates.len()];
    while let Some(index) = pending.pop() {
        if std::mem::replace(&mut retained[index], true) {
            continue;
        }
        for name in &references[index] {
            if let Some(targets) = definitions.remove(name.trim_start_matches("r#")) {
                pending.extend(targets);
            }
        }
    }
    retained
}

fn contains_inline_rust(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::RustBlock | SyntaxKind::UnsafeRustBlock
    ) || node.children.iter().any(contains_inline_rust)
}

// Names, not resolved indexes: consumers can compose the same generated source
// with different peers. Exact rendered content is the only input to this query.
type ItemReferences = Vec<Vec<String>>;

fn summarize_group(items: &[Item], cache_dir: Option<&Path>) -> (ItemReferences, bool) {
    // proc_macro2 tokens cannot move to workers. Walk them directly rather
    // than formatting and re-lexing every item to make the work transferable.
    let collect = |stream: TokenStream| {
        let mut identifiers = BTreeSet::new();
        collect_identifiers(stream, &mut identifiers);
        identifiers.into_iter().collect()
    };
    let streams = || {
        items.iter().map(|item| {
            item.demand_tokens()
                .expect("opaque items excluded before demand selection")
        })
    };
    let Some(dir) = cache_dir.filter(|_| !items.is_empty()) else {
        return (streams().map(collect).collect(), false);
    };
    let mut hash = crate::cache_io::HashWriter::new();
    hash.part(b"direct-token-identifiers-v1");
    let streams = streams().collect::<Vec<_>>();
    for stream in &streams {
        hash.part(stream.to_string().as_bytes());
    }
    let key = hash.finish_hex();
    let path = dir.join(&key[..2]).join(format!("{key}.json"));
    if let Some(summary) = cache::load_file::<ItemReferences>(&path, &key)
        && summary.len() == items.len()
    {
        return (summary, true);
    }
    let summary: ItemReferences = streams.into_iter().map(collect).collect();
    cache::store_file(&path, &key, &summary);
    (summary, false)
}

fn collect_identifiers(tokens: TokenStream, names: &mut BTreeSet<String>) {
    for token in tokens {
        match token {
            TokenTree::Group(group) => collect_identifiers(group.stream(), names),
            TokenTree::Ident(ident) => {
                names.insert(ident.to_string());
            }
            TokenTree::Punct(_) | TokenTree::Literal(_) => {}
        }
    }
}
