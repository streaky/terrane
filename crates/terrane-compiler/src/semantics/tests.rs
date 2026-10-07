use super::prelude::*;
use crate::package::RustDependency;
use crate::rust_interop::projection::{
    Containment, DeclinedItem, ProjectedBoundaryCapabilities, ProjectedDependency, ProjectedItem,
    ProjectedKind, Projection, ProjectionResolution, ProjectionSource,
};

fn ambiguous_projection() -> Projection {
    let dependency = |package: &str, rust_path: &str, send: bool| ProjectedDependency {
        name: package.to_owned(),
        native_alias_identities: BTreeMap::new(),
        package: package.to_owned(),
        version: "1.0.0".to_owned(),
        items: vec![ProjectedItem {
            namespace: "/deps/shared".to_owned(),
            name: "Generic".to_owned(),
            rust_path: rust_path.to_owned(),
            docs: None,
            kind: ProjectedKind::ForeignType {
                generic_parameters: Vec::new(),
                constructor: None,
                methods: Vec::new(),
                static_methods: Vec::new(),
                constants: Vec::new(),
                cloneable: false,
                send,
                sync: false,
                boundary: ProjectedBoundaryCapabilities::default(),
                fields: Vec::new(),
                borrowed_view: false,
                native_view_type: None,
                enum_payload: None,
                displayable: false,
            },
        }],
        declined: Vec::new(),
        partial_declines: Vec::new(),
    };
    Projection {
        native_owner_aliases: BTreeMap::default(),
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
fn local_function_value_preserves_callable_result_type() {
    let package = Package::implicit(
        "main.trn",
        "namespace app\nfunction first int;\n    function helper int;\n        return 1\n    value = helper;\n    return value\n".to_owned(),
    );
    let semantic = analyze(&package).expect("the local function call has the declared int result");
    let value = semantic
        .units
        .iter()
        .filter(|unit| !unit.bundled)
        .flat_map(|unit| &unit.typed_bindings)
        .find(|binding| binding.name == "value")
        .unwrap();
    assert_eq!(value.value_type, ValueType::Scalar(ScalarType::Int));
}

#[test]
fn local_function_value_rejects_incompatible_enclosing_result() {
    let package = Package::implicit(
        "main.trn",
        "namespace app\nfunction first string;\n    function helper int;\n        return 1\n    value = helper;\n    return value\n".to_owned(),
    );
    let failure = analyze(&package).expect_err("int callable must not satisfy string result");
    assert_eq!(failure.diagnostics[0].code, "T0015");
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
fn projected_public_reexports_are_addressable_before_authored_imports() {
    let mut projection = ambiguous_projection();
    for (dependency, namespace) in projection
        .dependencies
        .iter_mut()
        .zip(["/deps/facade", "/deps/provider"])
    {
        dependency.name = namespace.trim_start_matches("/deps/").to_owned();
        dependency.package = dependency.name.clone();
        dependency.items[0].namespace = namespace.to_owned();
        dependency.items[0].rust_path = "provider::Generic".to_owned();
    }
    let mut renamed = projection.dependencies[1].items[0].clone();
    renamed.name = "PublicGeneric".to_owned();
    projection.dependencies[1].items.push(renamed);
    let mut package = Package::implicit(
        "main.trn",
        concat!(
            "namespace app\n",
            "from /deps/provider import Generic as ProviderGeneric, PublicGeneric\n",
            "from /deps/facade import Generic as FacadeGeneric\n",
        )
        .to_owned(),
    );
    for name in ["provider", "facade"] {
        package.rust_dependencies.push(RustDependency {
            name: name.to_owned(),
            package: name.to_owned(),
            version: "=1.0.0".to_owned(),
            features: Vec::new(),
            default_features: false,
            target: None,
            effects: Vec::new(),
        });
    }
    let semantic = analyze_with_projection(&package, projection).unwrap();
    let symbols = &semantic.namespaces["/app"].symbols;
    assert_eq!(
        symbols["ProviderGeneric"].identity,
        symbols["FacadeGeneric"].identity
    );
    assert_eq!(
        symbols["PublicGeneric"].identity,
        symbols["FacadeGeneric"].identity
    );
    assert_eq!(
        semantic.namespaces["/deps/provider"].symbols["PublicGeneric"].identity,
        symbols["FacadeGeneric"].identity
    );
}

fn unavailable_projection() -> Projection {
    Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "unavailable-semantics".to_owned(),
        content_hash: String::new(),
        dependencies: vec![ProjectedDependency {
            name: "shared".to_owned(),
            native_alias_identities: BTreeMap::new(),
            package: "shared".to_owned(),
            version: "1.0.0".to_owned(),
            items: Vec::new(),
            declined: vec![DeclinedItem {
                rust_path: "shared::Missing".to_owned(),
                reason: "requires a compiler capability".to_owned(),
            }],
            partial_declines: Vec::new(),
        }],
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::default(),
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    }
}

fn package_with_unavailable_import(source: &str) -> Package {
    let mut package = Package::implicit("main.trn", source.to_owned());
    package.rust_dependencies.push(RustDependency {
        name: "shared".to_owned(),
        package: "shared".to_owned(),
        version: "=1.0.0".to_owned(),
        features: Vec::new(),
        default_features: false,
        target: None,
        effects: Vec::new(),
    });
    package
}

#[test]
fn unused_unavailable_projected_import_does_not_fail_analysis() {
    let package = package_with_unavailable_import(
        "namespace app\nfrom /deps/shared import Missing\nfunction main;\n    return\n",
    );

    analyze_with_projection(&package, unavailable_projection()).unwrap();
}

#[test]
fn demanded_unavailable_projected_import_fails_at_the_demand() {
    let package = package_with_unavailable_import(
        "namespace app\nfrom /deps/shared import Missing\nfunction main;\n    print; Missing\n",
    );

    let failure = analyze_with_projection(&package, unavailable_projection()).unwrap_err();

    assert_eq!(failure.diagnostics[0].code, "S2029");
    let primary = failure.diagnostics[0].primary.unwrap();
    assert_eq!(
        &package.units[0].source.text()[primary.start..primary.end],
        "Missing"
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

    assert!(warnings(&semantic, false, false).is_empty());
    let diagnostics = warnings(&semantic, true, false);
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "S2018"
            && diagnostic.message == "declared name `Answer` is not kebab-case"
            && diagnostic.severity == crate::Severity::Warning
    }));
}

#[test]
fn unused_top_level_function_warning_is_opt_in() {
    let package = Package::implicit(
        "main.trn",
        "namespace app\nfunction helper;\n  print; 1\nfunction main;\n  print; 2\n".to_owned(),
    );
    let semantic = analyze(&package).unwrap();

    assert!(warnings(&semantic, false, false).is_empty());
    let diagnostics = warnings(&semantic, false, true);
    assert_eq!(
        diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>(),
        vec!["W4005"]
    );
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
