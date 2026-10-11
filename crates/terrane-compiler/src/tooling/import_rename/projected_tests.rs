use super::super::tests::TemporaryDirectory;
use super::super::{Availability, SnapshotOptions, SourceInput, ToolingEngine};
use std::fs;
use std::path::PathBuf;

#[test]
fn projected_dependency_alias_rename_and_removal_update_only_its_bound_uses() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/check/rust-dependency-reqwest")
        .canonicalize()
        .expect("real dependency fixture");
    let directory = TemporaryDirectory::new("projected-alias");
    let root = directory.path();
    fs::create_dir_all(root.join("src")).expect("temporary source directory");
    fs::copy(fixture.join("package.toml"), root.join("package.toml"))
        .expect("copy dependency manifest");
    fs::copy(
        fixture.join("terrane-projection.lock"),
        root.join("terrane-projection.lock"),
    )
    .expect("copy projection lock");
    let source_path = root.join("src/main.trn");
    let source = "namespace app\nfrom /deps/http import StatusCode\nfrom /deps/http import StatusCode as ReplyCode, StatusCode as OtherCode\nfunction main;\n    reply ReplyCode|none = none\n    other OtherCode|none = none\n    unrelated StatusCode|none = none\n";
    fs::write(&source_path, source).expect("write temporary consumer");
    let source_uri = format!("file://{}", source_path.display());
    let manifest_uri = format!("file://{}", root.join("package.toml").display());
    let mut engine = ToolingEngine::default();
    let snapshot = engine
        .open_snapshot(
            vec![SourceInput {
                uri: source_uri.clone(),
                text: source.to_owned(),
            }],
            Some(SourceInput {
                uri: manifest_uri,
                text: fs::read_to_string(root.join("package.toml")).expect("manifest text"),
            }),
            None,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        )
        .expect("semantic dependency snapshot");
    assert!(matches!(
        snapshot.dependency_projection,
        Availability::Known(_)
    ));

    for (new_name, expected) in [
        (
            "ResponseCode",
            "namespace app\nfrom /deps/http import StatusCode\nfrom /deps/http import StatusCode as ResponseCode, StatusCode as OtherCode\nfunction main;\n    reply ResponseCode|none = none\n    other OtherCode|none = none\n    unrelated StatusCode|none = none\n",
        ),
        (
            "StatusCode",
            "namespace app\nfrom /deps/http import StatusCode\nfrom /deps/http import StatusCode, StatusCode as OtherCode\nfunction main;\n    reply StatusCode|none = none\n    other OtherCode|none = none\n    unrelated StatusCode|none = none\n",
        ),
    ] {
        for offset in [
            source.find("ReplyCode").expect("alias token"),
            source.find("reply ReplyCode").expect("bound use") + "reply ".len(),
        ] {
            let proposal = engine
                .propose_rename(&snapshot.snapshot_id, &source_uri, offset, new_name)
                .expect("projected dependency rename");
            assert!(proposal.semantic_reanalysis);
            assert_eq!(proposal.preview_diagnostics, Vec::new());
            let mut actual = source.to_owned();
            for replacement in proposal.replacements.iter().rev() {
                assert_eq!(replacement.uri, source_uri);
                actual.replace_range(
                    replacement.span.start..replacement.span.end,
                    &replacement.text,
                );
            }
            assert_eq!(actual, expected);
        }
    }
}
