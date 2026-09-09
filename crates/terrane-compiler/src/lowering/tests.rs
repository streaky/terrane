use std::path::PathBuf;

use super::prelude::{LoweringRegistry, emit_error_support};
use crate::{SourceFile, Span};

#[test]
fn error_support_is_canonical_rust() {
    for (has_custom_throwable, has_dependency, uses_float_coercion_error) in [
        (false, false, false),
        (true, false, false),
        (true, true, false),
        (false, false, true),
    ] {
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
            uses_float_coercion_error,
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

#[test]
fn unsupported_reference_address_shape_returns_a_lowering_failure() {
    fn replace_reference_operand(node: &mut crate::syntax::SyntaxNode) -> bool {
        if node.kind == crate::syntax::SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last_mut()
        {
            operand.kind = crate::syntax::SyntaxKind::Literal;
            return true;
        }
        node.children.iter_mut().any(replace_reference_operand)
    }

    let package = crate::Package::implicit(
        PathBuf::from("case.trn"),
        "namespace case\nfunction main;\n  value = 1\n  observer ref int = ref value\n  print; observer\n"
            .to_owned(),
    );
    let mut semantic = crate::semantics::analyze(&package).expect("source must pass semantics");
    assert!(replace_reference_operand(&mut semantic.units[0].tree.root));

    let Err(failure) = super::lower(&semantic) else {
        panic!("unsupported address lowering must not produce a program");
    };
    assert_eq!(
        failure.message,
        "validated reference expression `value` has no native address lowering"
    );
}
