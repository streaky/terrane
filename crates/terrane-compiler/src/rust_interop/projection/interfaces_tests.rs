use super::interfaces::project_external_provided_trait_methods;
use super::test_support::{dependency, projected_foreign_type_item};
use super::{DeclinedItem, ProjectedDependency, ProjectedKind, RustDependency};
use rustdoc_types::{
    Abi, Crate as RustdocCrate, Function, FunctionHeader, FunctionSignature, Generics, Id, Impl,
    Item, ItemEnum, ItemKind, ItemSummary, Path, Struct, StructKind, Target, Trait, Type,
    Visibility,
};
use std::collections::{BTreeMap, HashMap};

fn generics() -> Generics {
    Generics {
        params: Vec::new(),
        where_predicates: Vec::new(),
    }
}

fn item(id: Id, name: Option<&str>, inner: ItemEnum) -> (Id, Item) {
    (
        id,
        Item {
            id,
            crate_id: 0,
            name: name.map(str::to_owned),
            span: None,
            visibility: Visibility::Public,
            docs: None,
            links: HashMap::new(),
            attrs: Vec::new(),
            deprecation: None,
            const_stability: None,
            stability: None,
            inner,
        },
    )
}

fn document(
    index: impl IntoIterator<Item = (Id, Item)>,
    paths: impl IntoIterator<Item = (Id, ItemSummary)>,
) -> RustdocCrate {
    RustdocCrate {
        root: Id(0),
        crate_version: None,
        includes_private: false,
        index: index.into_iter().collect(),
        paths: paths.into_iter().collect(),
        external_crates: HashMap::new(),
        target: Target {
            triple: "x86_64-unknown-linux-gnu".to_owned(),
            target_features: Vec::new(),
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    }
}

fn summary(crate_id: u32, root: &str, name: &str, kind: ItemKind) -> ItemSummary {
    ItemSummary {
        crate_id,
        path: vec![root.to_owned(), name.to_owned()],
        kind,
    }
}

fn action(id: Id) -> (Id, Item) {
    item(
        id,
        Some("act"),
        ItemEnum::Function(Function {
            sig: FunctionSignature {
                inputs: Vec::new(),
                output: Some(Type::Primitive("i64".to_owned())),
                is_c_variadic: false,
            },
            generics: generics(),
            header: FunctionHeader {
                is_const: false,
                is_unsafe: false,
                is_async: false,
                abi: Abi::Rust,
            },
            has_body: true,
            default_unstable: None,
        }),
    )
}

fn owner_document(traits: &[&str]) -> RustdocCrate {
    let owner = Id(1);
    let mut paths = vec![(owner, summary(0, "owner", "Owner", ItemKind::Struct))];
    let mut entries = Vec::new();
    let mut impls = Vec::new();
    for (index, name) in (0u32..).zip(traits) {
        let trait_id = Id(2 + index * 3);
        let impl_id = Id(trait_id.0 + 1);
        let method = Id(trait_id.0 + 2);
        paths.push((trait_id, summary(1, "traits", name, ItemKind::Trait)));
        impls.push(impl_id);
        entries.push(action(method));
        entries.push(item(
            impl_id,
            None,
            ItemEnum::Impl(Impl {
                is_unsafe: false,
                generics: generics(),
                provided_trait_methods: Vec::new(),
                trait_: Some(Path {
                    path: format!("traits::{name}"),
                    id: trait_id,
                    args: None,
                }),
                for_: Type::ResolvedPath(Path {
                    path: "Owner".to_owned(),
                    id: owner,
                    args: None,
                }),
                items: vec![method],
                is_negative: false,
                is_synthetic: false,
                blanket_impl: None,
            }),
        ));
    }
    entries.push(item(
        owner,
        Some("Owner"),
        ItemEnum::Struct(Struct {
            kind: StructKind::Unit,
            generics: generics(),
            impls,
        }),
    ));
    document(entries, paths)
}

fn defining_document(traits: &[&str]) -> RustdocCrate {
    let mut entries = Vec::new();
    let mut paths = Vec::new();
    for (index, name) in (0u32..).zip(traits) {
        // Defining documents use their own IDs, not the references' IDs.
        let trait_id = Id(20 + index * 2);
        let method = Id(trait_id.0 + 1);
        paths.push((trait_id, summary(0, "traits", name, ItemKind::Trait)));
        entries.push(action(method));
        entries.push(item(
            trait_id,
            Some(name),
            ItemEnum::Trait(Trait {
                is_auto: false,
                is_unsafe: false,
                is_dyn_compatible: true,
                items: vec![method],
                generics: generics(),
                bounds: Vec::new(),
                implementations: Vec::new(),
            }),
        ));
    }
    document(entries, paths)
}

fn recover(traits: &[&str], defining: &[(&str, &[&str])]) -> ProjectedDependency {
    let mut dependencies = vec![dependency("owner", "owner", &[])];
    dependencies.extend(
        defining
            .iter()
            .map(|(name, _)| dependency(name, "traits", &[])),
    );
    let mut documents = vec![owner_document(traits)];
    documents.extend(defining.iter().map(|(_, traits)| defining_document(traits)));
    let rustdocs = dependencies
        .iter()
        .zip(documents)
        .map(|(dependency, document): (&RustDependency, _)| (dependency, document, BTreeMap::new()))
        .collect::<Vec<_>>();
    let mut projected = dependencies
        .iter()
        .map(|dependency| ProjectedDependency {
            name: dependency.name.clone(),
            package: dependency.package.clone(),
            version: dependency.version.clone(),
            items: Vec::new(),
            declined: Vec::new(),
            native_alias_identities: BTreeMap::new(),
            associated_implementations: Vec::new(),
            partial_declines: Vec::new(),
        })
        .collect::<Vec<_>>();
    projected[0].items.push(projected_foreign_type_item(
        "/deps/owner",
        "Owner",
        "owner::Owner",
        "unrelated",
    ));
    projected[0].declined.push(DeclinedItem {
        rust_path: "owner::Owner::act".to_owned(),
        reason: "not initially projectable".to_owned(),
    });
    let aliases = vec![BTreeMap::new(); rustdocs.len()];
    project_external_provided_trait_methods(&mut projected, &rustdocs, &aliases, false);
    projected.remove(0)
}

#[test]
fn keeps_both_explicit_trait_operations() {
    let projected = recover(&["First", "Second"], &[("traits", &["First", "Second"])]);
    let mut recovered = projected
        .items
        .iter()
        .filter(|item| item.name == "act")
        .map(|item| item.rust_path.as_str())
        .collect::<Vec<_>>();
    recovered.sort_unstable();
    assert_eq!(
        recovered,
        [
            "<owner::Owner as traits::First>::act",
            "<owner::Owner as traits::Second>::act",
        ]
    );
    assert!(
        !projected
            .declined
            .iter()
            .any(|item| item.rust_path == "owner::Owner::act")
    );
}

#[test]
fn skips_ambiguous_defining_trait() {
    let projected = recover(
        &["Shared"],
        &[("first", &["Shared"]), ("second", &["Shared"])],
    );
    let [owner] = projected.items.as_slice() else {
        panic!("ambiguous trait unexpectedly recovered a qualified operation");
    };
    let ProjectedKind::ForeignType { static_methods, .. } = &owner.kind else {
        panic!("expected a native owner");
    };
    assert_eq!(static_methods.as_slice(), []);
    assert_eq!(projected.declined[0].rust_path, "owner::Owner::act");
}
