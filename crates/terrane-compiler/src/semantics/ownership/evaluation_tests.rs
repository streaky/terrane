use super::collect_evaluation_steps;
use crate::{
    SourceFile,
    lexer::lex,
    parser::parse,
    semantics::EvaluationKind,
    syntax::{SyntaxKind, SyntaxNode},
};

fn find(node: &SyntaxNode, kind: SyntaxKind) -> Option<&SyntaxNode> {
    if node.kind == kind {
        Some(node)
    } else {
        node.children.iter().find_map(|child| find(child, kind))
    }
}

#[test]
fn classifies_owner_loan_barriers() {
    for (body, syntax, effect) in [
        ("first;", SyntaxKind::CallExpression, EvaluationKind::Call),
        (
            "value = instance item;",
            SyntaxKind::ConstructionExpression,
            EvaluationKind::Call,
        ),
        (
            "value = instance item; { field = 1 }",
            SyntaxKind::InitializationExpression,
            EvaluationKind::Call,
        ),
        ("value int = 1", SyntaxKind::Binding, EvaluationKind::Write),
        ("value = 2", SyntaxKind::Assignment, EvaluationKind::Write),
        (
            "return 1",
            SyntaxKind::ReturnStatement,
            EvaluationKind::Transfer,
        ),
        (
            "throw error",
            SyntaxKind::ThrowStatement,
            EvaluationKind::Transfer,
        ),
        (
            "value = move owner",
            SyntaxKind::UnaryExpression,
            EvaluationKind::Transfer,
        ),
        (
            "value = await delayed;",
            SyntaxKind::UnaryExpression,
            EvaluationKind::Suspension,
        ),
        (
            "callback = function int;\n    return 1",
            SyntaxKind::AnonymousFunction,
            EvaluationKind::Capture,
        ),
        (
            "value = rust\n    1i64",
            SyntaxKind::RustBlock,
            EvaluationKind::Native,
        ),
        (
            "value = unsafe rust\n    1i64",
            SyntaxKind::UnsafeRustBlock,
            EvaluationKind::Native,
        ),
        (
            "value = owner.field",
            SyntaxKind::MemberExpression,
            EvaluationKind::Access,
        ),
        (
            "value = owner::field",
            SyntaxKind::StaticMemberExpression,
            EvaluationKind::Access,
        ),
        (
            "value = owners[0]",
            SyntaxKind::IndexExpression,
            EvaluationKind::Access,
        ),
        (
            "value++",
            SyntaxKind::PostfixExpression,
            EvaluationKind::PostfixUpdate,
        ),
    ] {
        let source = SourceFile::new(
            0,
            "effects.trn".into(),
            format!("namespace app\nasync function main;\n  {body}\n"),
        );
        let parsed = parse(&source, lex(&source).unwrap());
        assert!(
            parsed.diagnostics.is_empty(),
            "{body}: {:?}",
            parsed.diagnostics
        );
        let node =
            find(&parsed.tree.root, syntax).unwrap_or_else(|| panic!("{body}: missing {syntax:?}"));
        let effects = collect_evaluation_steps(&source, &parsed.tree.root);
        assert!(
            effects
                .iter()
                .any(|step| step.span == node.span && step.kind == effect),
            "{body}: {effects:?}"
        );
    }
}
