use super::*;
use rustdoc_types::{
    Abi, Function, FunctionHeader, FunctionSignature, Target, TypeAlias, Visibility,
};

fn document(name: &str) -> RustdocCrate {
    RustdocCrate {
        root: Id(0),
        crate_version: Some("1.0.0".to_owned()),
        includes_private: false,
        index: HashMap::new(),
        paths: HashMap::from([(
            Id(0),
            ItemSummary {
                crate_id: 0,
                path: vec![name.to_owned()],
                kind: ItemKind::Module,
            },
        )]),
        external_crates: HashMap::new(),
        target: Target {
            triple: "x86_64-unknown-linux-gnu".to_owned(),
            target_features: Vec::new(),
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    }
}

fn defining_document(offset: u32) -> RustdocCrate {
    let mut document = document("owner");
    document.external_crates = HashMap::from([
        (
            1,
            ExternalCrate {
                name: "core".to_owned(),
                html_root_url: None,
                path: "/core.rlib".into(),
            },
        ),
        (
            2,
            ExternalCrate {
                name: "std".to_owned(),
                html_root_url: None,
                path: "/std.rlib".into(),
            },
        ),
    ]);
    for (id, crate_id, path, kind) in [
        (
            Id(offset + 1),
            0,
            vec!["owner", "Outcome"],
            ItemKind::TypeAlias,
        ),
        (
            Id(offset + 2),
            1,
            vec!["core", "result", "Result"],
            ItemKind::Enum,
        ),
        (
            Id(offset + 3),
            2,
            vec!["std", "io", "Error"],
            ItemKind::Struct,
        ),
    ] {
        document.paths.insert(
            id,
            ItemSummary {
                crate_id,
                path: path.into_iter().map(str::to_owned).collect(),
                kind,
            },
        );
    }
    document.index.insert(
        Id(offset + 1),
        Item {
            id: Id(offset + 1),
            crate_id: 0,
            name: Some("Outcome".to_owned()),
            span: None,
            visibility: Visibility::Public,
            docs: None,
            links: HashMap::new(),
            attrs: Vec::new(),
            deprecation: None,
            stability: None,
            const_stability: None,
            inner: ItemEnum::TypeAlias(TypeAlias {
                type_: Type::ResolvedPath(RustdocPath {
                    path: "Result".to_owned(),
                    id: Id(offset + 2),
                    args: Some(Box::new(GenericArgs::AngleBracketed {
                        args: vec![
                            GenericArg::Type(Type::Primitive("i64".to_owned())),
                            GenericArg::Type(Type::ResolvedPath(RustdocPath {
                                path: "Error".to_owned(),
                                id: Id(offset + 3),
                                args: None,
                            })),
                        ],
                        constraints: Vec::new(),
                    })),
                }),
                generics: Generics {
                    params: Vec::new(),
                    where_predicates: Vec::new(),
                },
            }),
        },
    );
    document
}

#[test]
fn copies_do_not_conflict() {
    let first = defining_document(10);
    let second = defining_document(100);
    let mut target = document("consumer");
    target.external_crates.insert(
        1,
        ExternalCrate {
            name: "owner".to_owned(),
            html_root_url: None,
            path: "/owner.rlib".into(),
        },
    );
    let path = vec!["owner".to_owned(), "Outcome".to_owned()];
    target.paths.insert(
        Id(1),
        ItemSummary {
            crate_id: 1,
            path: path.clone(),
            kind: ItemKind::TypeAlias,
        },
    );
    let metadata = serde_json::json!({"packages": [{
        "name": "owner", "version": "1.0.0", "targets": [{"name": "owner", "kind": ["lib"]}]
    }]});
    let imports = plan(
        &target,
        &[&first, &second],
        &BTreeSet::from([path]),
        &metadata,
    );
    target.index.extend(imports.items);
    target.paths.extend(imports.paths);
    target.external_crates.extend(imports.crates);
    let function = Function {
        sig: FunctionSignature {
            inputs: Vec::new(),
            is_c_variadic: false,
            output: Some(Type::ResolvedPath(RustdocPath {
                path: "Outcome".to_owned(),
                id: Id(1),
                args: None,
            })),
        },
        generics: Generics {
            params: Vec::new(),
            where_predicates: Vec::new(),
        },
        header: FunctionHeader {
            is_const: false,
            is_unsafe: false,
            is_async: false,
            abi: Abi::Rust,
        },
        has_body: true,
        default_unstable: None,
    };
    let projected = super::super::functions::project_function(
        &function,
        &target.index,
        &target.paths,
        &BTreeMap::new(),
        Some("read"),
        false,
    )
    .unwrap();
    assert_eq!(projected.result, super::super::ProjectedType::Int);
    assert_eq!(projected.error.as_deref(), Some("std::io::Error"));
}
