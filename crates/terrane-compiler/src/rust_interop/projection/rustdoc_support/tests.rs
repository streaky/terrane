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
fn only_plain_self_receivers() {
    let borrowed = |is_mutable| Type::BorrowedRef {
        lifetime: None,
        is_mutable,
        type_: Box::new(Type::Generic("Self".to_owned())),
    };
    let paths = HashMap::new();
    assert_eq!(
        receiver_kind(&borrowed(true), &paths).unwrap(),
        Receiver::MutableBorrow
    );
    assert_eq!(
        receiver_kind(&borrowed(false), &paths).unwrap(),
        Receiver::Borrow
    );
    assert_eq!(
        receiver_kind(&Type::Generic("Self".to_owned()), &paths).unwrap(),
        Receiver::Move
    );
    assert!(receiver_kind(&Type::Primitive("str".to_owned()), &paths).is_err());
}
#[test]
fn schema_mismatch_is_explicit() {
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
fn shared_owner_boundaries() {
    let cases = [
        ("alloc::rcs::arc::Arc<T>::new", "alloc::sync::Arc<T>::new"),
        (
            "alloc::rcs::arc::Weak<T>::upgrade",
            "alloc::sync::Weak<T>::upgrade",
        ),
        (
            "alloc::rcs::arc::Arc<T>::Assoc<U>",
            "alloc::sync::Arc<T>::Assoc<U>",
        ),
        (
            "alloc::rcs::arc::Weak<T>::Assoc<U>",
            "alloc::sync::Weak<T>::Assoc<U>",
        ),
        ("alloc::rcs::rc::Rc<T>::new", "alloc::rc::Rc<T>::new"),
        (
            "alloc::rcs::rc::Weak<T>::upgrade",
            "alloc::rc::Weak<T>::upgrade",
        ),
        (
            "alloc::rcs::rc::Rc<T>::Assoc<U>",
            "alloc::rc::Rc<T>::Assoc<U>",
        ),
        (
            "alloc::rcs::rc::Weak<T>::Assoc<U>",
            "alloc::rc::Weak<T>::Assoc<U>",
        ),
        ("std::rcs::arc::Arc<T>::new", "std::sync::Arc<T>::new"),
        (
            "std::rcs::arc::Weak<T>::upgrade",
            "std::sync::Weak<T>::upgrade",
        ),
        (
            "std::rcs::arc::Arc<T>::Assoc<U>",
            "std::sync::Arc<T>::Assoc<U>",
        ),
        (
            "std::rcs::arc::Weak<T>::Assoc<U>",
            "std::sync::Weak<T>::Assoc<U>",
        ),
        ("std::rcs::rc::Rc<T>::new", "std::rc::Rc<T>::new"),
        (
            "std::rcs::rc::Weak<T>::upgrade",
            "std::rc::Weak<T>::upgrade",
        ),
        ("std::rcs::rc::Rc<T>::Assoc<U>", "std::rc::Rc<T>::Assoc<U>"),
        (
            "std::rcs::rc::Weak<T>::Assoc<U>",
            "std::rc::Weak<T>::Assoc<U>",
        ),
        (
            "alloc::rcs::arc::ArcInner<T>",
            "alloc::rcs::arc::ArcInner<T>",
        ),
        ("alloc::rcs::rc::UniqueRc<T>", "alloc::rcs::rc::UniqueRc<T>"),
        ("mycrate::rcs::arc::Arc<T>", "mycrate::rcs::arc::Arc<T>"),
        ("other::rcs::rc::Rc<T>", "other::rcs::rc::Rc<T>"),
        ("std::rcs::arc::ArcInner<T>", "std::rcs::arc::ArcInner<T>"),
        ("std::rcs::rc::UniqueRc<T>", "std::rcs::rc::UniqueRc<T>"),
    ];

    for (input, expected) in cases {
        let mut actual = input.to_owned();
        normalize_shared_owner_path(&mut actual);
        assert_eq!(actual, expected, "{input}");
    }
}
#[test]
fn malformed_json_is_not_declined() {
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
