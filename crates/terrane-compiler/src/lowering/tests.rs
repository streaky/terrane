use std::path::PathBuf;

use super::prelude::{LoweringRegistry, emit_error_support};
use crate::{SourceFile, Span};

#[test]
fn error_support_is_canonical_rust() {
    for (has_custom_throwable, has_dependency) in [(false, false), (true, false), (true, true)] {
        let mut emitted = String::new();
        let registry = LoweringRegistry::default();
        if has_dependency {
            registry.register_descriptor("/core/errors::dependency-error", "dependency-error");
            registry.register_descriptor("/core/errors::dependency-panic", "dependency-panic");
        }
        emit_error_support(
            &mut emitted,
            has_custom_throwable,
            has_dependency,
            &registry,
        );
        let canonical = crate::rust_ir::canonicalize_rust(&emitted).unwrap();
        assert_eq!(
            crate::rust_ir::canonicalize_rust(&canonical).unwrap(),
            canonical
        );
    }
}

#[test]
fn lowering_registry_reuses_identical_semantic_sites() {
    let registry = LoweringRegistry::default();
    let source = SourceFile::new(0, PathBuf::from("case.trn"), "value".to_owned());
    let span = Span::new(0, 0, 5);

    let first = registry.register_site("case.trn", "/demo::main", &source, span);
    let second = registry.register_site("case.trn", "/demo::main", &source, span);

    assert_eq!(first, second);
    assert_eq!(registry.sites.borrow().len(), 1);
}
