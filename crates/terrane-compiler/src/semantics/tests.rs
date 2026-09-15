use super::prelude::*;
use crate::projection::{
    Containment, ProjectedDependency, ProjectedItem, ProjectedKind, Projection,
    ProjectionResolution, ProjectionSource,
};

fn ambiguous_projection() -> Projection {
    let dependency = |package: &str, rust_path: &str, send: bool| ProjectedDependency {
        name: package.to_owned(),
        package: package.to_owned(),
        version: "1.0.0".to_owned(),
        items: vec![ProjectedItem {
            namespace: "/deps/shared".to_owned(),
            name: "Generic".to_owned(),
            rust_path: rust_path.to_owned(),
            docs: None,
            kind: ProjectedKind::ForeignType {
                methods: Vec::new(),
                static_methods: Vec::new(),
                cloneable: false,
                send,
                sync: false,
            },
        }],
        declined: Vec::new(),
    };
    Projection {
        cache_identity: "ambiguous-semantics".to_owned(),
        content_hash: String::new(),
        dependencies: vec![
            dependency("one", "one::Generic<A>", true),
            dependency("two", "two::Generic<B>", false),
        ],
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::default(),
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    }
}

#[test]
fn ambiguous_projected_import_names_every_rust_identity() {
    let package = Package::implicit(
        "main.trn",
        "namespace app\nfrom /deps/shared import Generic\n".to_owned(),
    );

    let failure = analyze_with_projection(&package, ambiguous_projection()).unwrap_err();

    assert_eq!(failure.diagnostics[0].code, "S2056");
    assert_eq!(
        failure.diagnostics[0].message,
        "Rust dependency member `Generic` in `/deps/shared` is ambiguous: projected Rust types `one::Generic<A>`, `two::Generic<B>`"
    );
}

#[test]
fn ambiguous_projected_destination_names_every_rust_identity() {
    let semantic = analyze_with_projection(
        &Package::implicit("main.trn", "namespace app\n".to_owned()),
        ambiguous_projection(),
    )
    .unwrap();

    let error = destination_projected_type(
        &semantic,
        &ValueType::Object(ObjectIdentity::new("/deps/shared", "Generic")),
    )
    .unwrap_err();

    assert_eq!(
        error,
        "projected object `/deps/shared::Generic` is ambiguous: projected Rust types `one::Generic<A>`, `two::Generic<B>`"
    );
}

#[test]
fn compiler_owned_declarations_require_kebab_case() {
    let package = Package::implicit(
        "main.trn",
        "namespace app\nfunction NotKebab;\n  return\nfunction main;\n  return\n".to_owned(),
    );
    let mut semantic = analyze(&package).unwrap();
    semantic.units[0].bundled = true;

    let failure = validate_compiler_owned_names(&semantic.units).unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2018");
    assert_eq!(
        failure.diagnostics[0].message,
        "compiler-owned declaration `NotKebab` is not kebab-case"
    );
}

#[test]
fn authored_name_style_is_an_opt_in_warning() {
    let package = Package::implicit(
        "main.trn",
        "namespace app\nfunction main;\n  Answer = 42\n  print; Answer\n".to_owned(),
    );
    let semantic = analyze(&package).unwrap();

    assert!(warnings(&semantic, false).is_empty());
    let diagnostics = warnings(&semantic, true);
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "S2018"
            && diagnostic.message == "declared name `Answer` is not kebab-case"
            && diagnostic.severity == crate::Severity::Warning
    }));
}

#[test]
fn object_union_arm_identity_follows_import_aliases() {
    let package = Package::implicit(
        "main.trn",
        concat!(
            "namespace app\n",
            "from /core/errors import throwable as first, throwable as second\n",
            "function main;\n",
            "  return\n",
        )
        .to_owned(),
    );
    let semantic = analyze(&package).unwrap();
    let unit = &semantic.units[0];
    let first = unit.source.text().find("first").unwrap();
    let second = unit.source.text().find("second").unwrap();

    assert_eq!(
        union_arm_identity(&semantic, unit, "first", first),
        union_arm_identity(&semantic, unit, "second", second),
    );
}

#[test]
fn arbitrary_object_optional_types_are_semantic_values() {
    let package = Package::implicit(
            "main.trn",
            "namespace app\nclass widget\nfunction maybe widget|none;\n  return none\nfunction main;\n  value widget|none = maybe;\n  return\n".to_owned(),
        );
    let semantic = analyze(&package).unwrap();
    let maybe = semantic.units[0]
        .functions
        .iter()
        .find(|function| function.name == "maybe")
        .unwrap();

    assert!(matches!(
        &maybe.return_type,
        Some(ValueType::Optional(inner))
            if matches!(inner.as_ref(), ValueType::Object(identity) if identity.name == "widget")
    ));
}
