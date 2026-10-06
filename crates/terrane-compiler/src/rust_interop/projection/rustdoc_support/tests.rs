use super::*;
use crate::{RUSTDOC_TOOLCHAIN, RustDependency};
use serde_json::json;
fn parse_rustdoc(
    dependency: &RustDependency,
    bytes: &[u8],
) -> Result<rustdoc_types::Crate, super::super::ProjectionError> {
    terrane_rust_analysis::parse_rustdoc(&dependency.package, bytes, RUSTDOC_TOOLCHAIN).map_err(
        |error| super::super::ProjectionError {
            message: error.message,
        },
    )
}

#[test]
fn receiver_kind_preserves_only_plain_self_receivers() {
    let borrowed = |is_mutable| Type::BorrowedRef {
        lifetime: None,
        is_mutable,
        type_: Box::new(Type::Generic("Self".to_owned())),
    };
    assert_eq!(
        receiver_kind(&borrowed(true)).unwrap(),
        Receiver::MutableBorrow
    );
    assert_eq!(receiver_kind(&borrowed(false)).unwrap(), Receiver::Borrow);
    assert_eq!(
        receiver_kind(&Type::Generic("Self".to_owned())).unwrap(),
        Receiver::Move
    );
    assert!(receiver_kind(&Type::Primitive("str".to_owned())).is_err());
}
#[test]
fn rustdoc_schema_mismatch_is_explicit() {
    let document = json!({
        "root": 0,
        "crate_version": "1.0.0",
        "includes_private": false,
        "index": {},
        "paths": {},
        "external_crates": {},
        "target": {"triple": "x86_64-unknown-linux-gnu", "target_features": []},
        "format_version": rustdoc_types::FORMAT_VERSION + 1
    });
    let dependency = RustDependency {
        name: "witness".to_owned(),
        package: "witness".to_owned(),
        version: "=1.0.0".to_owned(),
        features: Vec::new(),
        default_features: true,
        target: None,
        effects: Vec::new(),
    };

    let error = parse_rustdoc(&dependency, &serde_json::to_vec(&document).unwrap())
        .expect_err("an unsupported rustdoc schema must fail");

    assert!(error.message.contains("schema mismatch"));
    assert!(
        error
            .message
            .contains(&format!("format {}", rustdoc_types::FORMAT_VERSION + 1))
    );
    assert!(error.message.contains(&format!(
        "expected format {}",
        rustdoc_types::FORMAT_VERSION
    )));
}

#[test]
fn malformed_rustdoc_json_is_not_silently_declined() {
    let dependency = RustDependency {
        name: "witness".to_owned(),
        package: "witness".to_owned(),
        version: "=1.0.0".to_owned(),
        features: Vec::new(),
        default_features: true,
        target: None,
        effects: Vec::new(),
    };
    let malformed = format!(r#"{{"format_version":{}}}"#, rustdoc_types::FORMAT_VERSION);

    let error = parse_rustdoc(&dependency, malformed.as_bytes())
        .expect_err("malformed rustdoc JSON must fail");

    assert!(error.message.contains("schema mismatch"));
    assert!(error.message.contains(&format!(
        "expected rustdoc format {}",
        rustdoc_types::FORMAT_VERSION
    )));
}
