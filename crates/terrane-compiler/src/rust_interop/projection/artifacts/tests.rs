use std::collections::BTreeMap;
use std::fs;

use super::super::{
    Containment, Projection, ProjectionResolution, ProjectionSource, projection_content_hash,
};
use super::*;

#[test]
fn projection_cache_removes_only_legacy_projection_records() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-prune-{}", std::process::id()));
    let stable = directory.join("projection.json");
    let oracle = directory.join("oracle.json");
    let owner = directory.join("owner-rustdoc.json");
    let unrelated = directory.join("Cargo.lock");
    fs::create_dir_all(&directory).unwrap();
    fs::write(&stable, b"current").unwrap();
    fs::write(&oracle, b"oracle").unwrap();
    fs::write(&owner, b"owner envelope").unwrap();
    fs::write(&unrelated, b"lock").unwrap();
    fs::write(
        directory.join(format!("owner-rustdoc-{:064x}.json", 1)),
        b"legacy owner",
    )
    .unwrap();
    fs::write(
        directory.join(format!("owner-rustdoc-{:064x}.identity", 1)),
        b"legacy identity",
    )
    .unwrap();
    for index in 0..18 {
        fs::write(
            directory.join(format!("projection-{index:064x}.json")),
            b"legacy",
        )
        .unwrap();
    }

    remove_legacy_projection_cache(&directory).unwrap();

    assert!(stable.exists());
    assert!(oracle.exists());
    assert!(owner.exists());
    assert!(unrelated.exists());
    assert!(
        !directory
            .join(format!("owner-rustdoc-{:064x}.json", 1))
            .exists()
    );
    assert!(
        !directory
            .join(format!("owner-rustdoc-{:064x}.identity", 1))
            .exists()
    );
    assert!(
        fs::read_dir(&directory)
            .unwrap()
            .filter_map(Result::ok)
            .all(|entry| !entry
                .file_name()
                .to_string_lossy()
                .starts_with("projection-"))
    );
    fs::remove_dir_all(directory).unwrap();
}
#[test]
fn remote_artifact_requires_exact_projection_inputs() {
    let dependency = RustDependency {
        name: "witness".to_owned(),
        package: "witness-package".to_owned(),
        version: "=1.2.3".to_owned(),
        features: vec!["derive".to_owned()],
        default_features: false,
        target: Some("cfg(unix)".to_owned()),
        effects: vec!["filesystem".to_owned()],
    };
    let mut payload = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "exact".to_owned(),
        content_hash: String::new(),
        dependencies: Vec::new(),
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    payload.content_hash = projection_content_hash(&payload).unwrap();
    let artifact = ProjectionArtifact {
        format: 1,
        cache_identity: "exact".to_owned(),
        content_hash: payload.content_hash.clone(),
        target: "x86_64-unknown-linux-gnu".to_owned(),
        build_toolchain: crate::BUILD_TOOLCHAIN.to_owned(),
        rustdoc_toolchain: crate::RUSTDOC_TOOLCHAIN.to_owned(),
        rustdoc_format: rustdoc_types::FORMAT_VERSION,
        projection_schema: super::PROJECTION_SCHEMA.to_owned(),
        dependencies: vec![ArtifactDependency::from(&dependency)],
        projection: payload,
    };

    let projection = validate_projection_artifact(
        artifact.clone(),
        "exact",
        "x86_64-unknown-linux-gnu",
        std::slice::from_ref(&dependency),
        Containment::Unavailable,
    )
    .expect("exact artifact metadata must be admitted");
    assert_eq!(projection.source, ProjectionSource::Remote);
    assert_eq!(projection.containment, Containment::Unavailable);

    let reject = |candidate| {
        validate_projection_artifact(
            candidate,
            "exact",
            "x86_64-unknown-linux-gnu",
            std::slice::from_ref(&dependency),
            Containment::Unavailable,
        )
        .unwrap_err()
    };

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].version = "=9.9.9".to_owned();
    assert_eq!(
        reject(mismatched),
        "dependency metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].effects = vec!["network".to_owned()];
    assert_eq!(
        reject(mismatched),
        "dependency metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].features = vec!["different".to_owned()];
    assert_eq!(
        reject(mismatched),
        "feature metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].default_features = true;
    assert_eq!(
        reject(mismatched),
        "feature metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].target = Some("cfg(windows)".to_owned());
    assert_eq!(
        reject(mismatched),
        "dependency target mismatch for `witness`: expected `cfg(unix)`, found `cfg(windows)`"
    );

    let mut corrupt = artifact;
    corrupt.content_hash = "not-the-payload-hash".to_owned();
    let reason = validate_projection_artifact(
        corrupt,
        "exact",
        "x86_64-unknown-linux-gnu",
        &[dependency],
        Containment::Unavailable,
    )
    .unwrap_err();
    assert!(reason.contains("content hash mismatch"));
}
