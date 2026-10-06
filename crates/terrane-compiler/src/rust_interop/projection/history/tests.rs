use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use super::*;
use crate::rust_interop::projection::*;

#[expect(
    clippy::too_many_lines,
    reason = "the history regression records a complete two-generation cache transition"
)]
#[test]
fn projection_history_retains_removed_members_across_checks() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-history-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let dependency = |version: &str, items: Vec<ProjectedItem>| ProjectedDependency {
        name: "fixture".to_owned(),
        native_alias_identities: BTreeMap::new(),
        package: "fixture".to_owned(),
        version: version.to_owned(),
        items,
        declined: Vec::new(),
        partial_declines: Vec::new(),
    };
    let item = ProjectedItem {
        namespace: "/deps/fixture".to_owned(),
        name: "removed".to_owned(),
        rust_path: "fixture::removed".to_owned(),
        docs: None,
        kind: ProjectedKind::ForeignType {
            constructor: None,
            boundary: ProjectedBoundaryCapabilities::default(),
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            generic_parameters: Vec::new(),
            displayable: false,
            methods: vec![ProjectedFunction {
                native_owner: None,
                native_path: None,
                name: "read".to_owned(),
                generic_parameters: Vec::new(),
                operation_owner_generics: Vec::new(),
                rust_generic_arguments: Vec::new(),
                parameters: Vec::new(),
                result: ProjectedType::None,
                destination_result: None,
                error: None,
                is_async: false,
                is_unsafe: false,
                into_future: false,
                execution_requirements: None,
                enum_operation: None,
                error_optional_depth: 0,
                chain_role: None,
                receiver: Some(Receiver::Borrow),
            }],
            static_methods: vec![ProjectedFunction {
                native_owner: None,
                native_path: None,
                name: "create".to_owned(),
                generic_parameters: Vec::new(),
                operation_owner_generics: Vec::new(),
                rust_generic_arguments: Vec::new(),
                parameters: Vec::new(),
                result: ProjectedType::None,
                destination_result: None,
                error: None,
                is_async: false,
                is_unsafe: false,
                into_future: false,
                execution_requirements: None,
                enum_operation: None,
                error_optional_depth: 0,
                chain_role: None,
                receiver: None,
            }],
            constants: Vec::new(),
            cloneable: false,
            send: false,
            sync: false,
        },
    };
    let mut old = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "old".to_owned(),
        content_hash: String::new(),
        dependencies: vec![dependency("1.0.0", vec![item])],
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    old.content_hash = projection_content_hash(&old).unwrap();
    apply_projection_history(&directory, &mut old).unwrap();
    let lock_path = directory.join("terrane-projection.lock");
    let mut reviewed: serde_json::Value =
        serde_json::from_slice(&fs::read(&lock_path).unwrap()).unwrap();
    let object = reviewed.as_object_mut().unwrap();
    for field in [
        "cache_identity",
        "projection_schema",
        "content_hash",
        "resolution",
    ] {
        object.remove(field);
    }
    fs::write(&lock_path, serde_json::to_vec_pretty(&reviewed).unwrap()).unwrap();
    let mut current = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "current".to_owned(),
        content_hash: String::new(),
        dependencies: vec![dependency("2.0.0", Vec::new())],
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    current.content_hash = projection_content_hash(&current).unwrap();
    apply_projection_history(&directory, &mut current).unwrap();
    assert_eq!(
        current
            .removed
            .iter()
            .map(|item| item.name.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["removed", "removed.read", "removed::create"])
    );
    current.removed.clear();
    apply_projection_history(&directory, &mut current).unwrap();
    assert_eq!(current.removed.len(), 3);
    let persisted: serde_json::Value =
        serde_json::from_slice(&fs::read(&lock_path).unwrap()).unwrap();
    assert_eq!(persisted["dependencies"][0]["version"], "2.0.0");
    assert_eq!(persisted["removed"].as_array().unwrap().len(), 3);
    for field in [
        "cache_identity",
        "projection_schema",
        "content_hash",
        "resolution",
    ] {
        assert!(persisted.get(field).is_none(), "unexpected `{field}`");
    }
    assert_eq!(persisted["source"], "Local");
    assert_eq!(persisted["rustdoc_format"], rustdoc_types::FORMAT_VERSION);
    let reviewed_bytes = serde_json::to_vec(&persisted).unwrap();
    fs::write(&lock_path, &reviewed_bytes).unwrap();
    apply_projection_history(&directory, &mut current).unwrap();
    assert_eq!(fs::read(&lock_path).unwrap(), reviewed_bytes);
    current.dependencies[0].version = "2.0.1".to_owned();
    current.content_hash = projection_content_hash(&current).unwrap();
    apply_projection_history(&directory, &mut current).unwrap();
    let updated: serde_json::Value =
        serde_json::from_slice(&fs::read(&lock_path).unwrap()).unwrap();
    assert_eq!(updated["dependencies"][0]["version"], "2.0.1");
    assert_eq!(updated["removed"].as_array().unwrap().len(), 3);
    assert!(updated.get("cache_identity").is_none());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn projection_history_keeps_content_origin_across_cache_hits() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-origin-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let generated = ProjectionResolution {
        outcome: ResolutionOutcome::LocalRustdoc,
        events: Vec::new(),
    };
    let mut projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "stable-identity".to_owned(),
        content_hash: String::new(),
        dependencies: Vec::new(),
        bound_dependencies: vec![ProjectedBoundDependency {
            name: "serde_core".to_owned(),
            package: "serde_core".to_owned(),
            version: "1.0.229".to_owned(),
        }],
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: generated.clone(),
        removed: Vec::new(),
    };
    projection.content_hash = projection_content_hash(&projection).unwrap();
    apply_projection_history(&directory, &mut projection).unwrap();
    projection.resolution = ProjectionResolution {
        outcome: ResolutionOutcome::ExactCache,
        events: Vec::new(),
    };
    apply_projection_history(&directory, &mut projection).unwrap();
    let history: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(history.resolution, Some(generated));
    assert_eq!(history.format, 4);
    assert_eq!(history.bound_dependencies, projection.bound_dependencies);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn legacy_format_three_history_migrates_without_losing_removed_members() {
    let legacy = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/projection-history/format-3.lock"
    ));
    let inventory: serde_json::Value = serde_json::from_str(legacy).unwrap();
    let expected = inventory["dependencies"][0]["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|member| {
            (
                member[0].as_str().unwrap().to_owned(),
                member[1].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    let directory = std::env::temp_dir().join(format!(
        "terrane-projection-format-three-{}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    fs::write(directory.join("terrane-projection.lock"), legacy).unwrap();
    let mut projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "format-three-migration".to_owned(),
        content_hash: String::new(),
        dependencies: vec![ProjectedDependency {
            name: "bytes".to_owned(),
            native_alias_identities: BTreeMap::new(),
            package: "bytes".to_owned(),
            version: "1.10.2".to_owned(),
            items: Vec::new(),
            declined: Vec::new(),
            partial_declines: Vec::new(),
        }],
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution {
            outcome: ResolutionOutcome::LocalRustdoc,
            events: Vec::new(),
        },
        removed: Vec::new(),
    };
    projection.content_hash = projection_content_hash(&projection).unwrap();
    apply_projection_history(&directory, &mut projection).unwrap();
    assert_eq!(
        projection
            .removed
            .iter()
            .map(|member| (member.namespace.clone(), member.name.clone(),))
            .collect::<BTreeSet<_>>(),
        expected
    );
    let migrated: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(migrated.format, 4);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn projection_history_migrates_provenance_and_detects_replay_drift() {
    let directory = std::env::temp_dir().join(format!(
        "terrane-projection-provenance-{}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    fs::write(
        directory.join("terrane-projection.lock"),
        b"{\"format\":1,\"dependencies\":[],\"removed\":[]}\n",
    )
    .unwrap();
    let mut projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "stable-identity".to_owned(),
        content_hash: String::new(),
        dependencies: Vec::new(),
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    projection.content_hash = projection_content_hash(&projection).unwrap();
    apply_projection_history(&directory, &mut projection).unwrap();

    let migrated: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(migrated.format, 4);
    assert_eq!(migrated.source, Some(ProjectionSource::Local));
    assert_eq!(migrated.rustdoc_format, Some(rustdoc_types::FORMAT_VERSION));
    assert_eq!(
        migrated.content_hash.as_deref(),
        Some(projection.content_hash.as_str())
    );

    projection.source = ProjectionSource::Remote;
    apply_projection_history(&directory, &mut projection).unwrap();
    let changed_source: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(changed_source.source, Some(ProjectionSource::Remote));

    projection.probe_wall_time_ms = 1;
    projection.content_hash = projection_content_hash(&projection).unwrap();
    let error = apply_projection_history(&directory, &mut projection).unwrap_err();
    assert!(error.message.contains("projection replay mismatch"));
    assert!(error.message.contains("previously produced"));
    fs::remove_dir_all(directory).unwrap();
}
