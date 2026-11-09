use terrane_compiler::{
    SourceFile,
    lexer::lex,
    parser::parse,
    syntax::{SyntaxKind, SyntaxNode, SyntaxTree},
};

fn parsed(text: &str) -> SyntaxTree {
    let source = SourceFile::new(0, "layout.trn".into(), text.to_owned());
    let output = parse(&source, lex(&source).expect("valid lexing"));
    assert!(output.diagnostics.is_empty(), "{:#?}", output.diagnostics);
    output.tree
}

fn shape(node: &SyntaxNode, tree: &SyntaxTree) -> String {
    if node.kind == SyntaxKind::GroupExpression {
        return shape(&node.children[0], tree);
    }
    if node.children.is_empty() {
        return format!(
            "{:?}:{}",
            node.kind,
            tree.lexed.tokens[node.token_range.clone()]
                .iter()
                .map(|token| token.text.as_str())
                .collect::<String>()
        );
    }
    format!(
        "{:?}({})",
        node.kind,
        node.children
            .iter()
            .map(|child| shape(child, tree))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn equivalent(layout: &str, explicit: &str) {
    let layout = parsed(layout);
    let explicit = parsed(explicit);
    assert_eq!(
        shape(&layout.root, &layout),
        shape(&explicit.root, &explicit)
    );
}

#[test]
fn chains_complete_calls_before_selecting_members() {
    equivalent(
        "x = message.with-text; 'ready'\n    .after; (milliseconds; 1)\n    .wait;\n",
        "x = ((message.with-text; 'ready').after; (milliseconds; 1)).wait;\n",
    );
    equivalent(
        "x = await message\n    .with-text;\n        'ready'\n    .after;\n        milliseconds; 1\n    .wait;\n",
        "x = await ((message.with-text; 'ready').after; (milliseconds; 1)).wait;\n",
    );
    equivalent(
        "x = outer; await future\n    .finish;\n",
        "x = (outer; await future).finish;\n",
    );
}

#[test]
fn argument_levels_bound_nested_calls_and_named_arguments() {
    equivalent(
        "x = outer;\n    inner;\n        'first'\n        label = 'second'\n    final = 3\ny = 4\n",
        "x = outer; (inner; 'first', label = 'second'), final = 3\ny = 4\n",
    );
    equivalent(
        "x = outer; 1\n    2, 3\n    final = 4\n",
        "x = outer; 1, 2, 3, final = 4\n",
    );
}

#[test]
fn mixed_parentheses_bound_layout_without_changing_wrapping() {
    equivalent(
        "x = (message\n    .with-text;\n        ('ready')\n    .wait;)\n",
        "x = (message.with-text; 'ready').wait;\n",
    );
    equivalent(
        "x = outer; (inner;\n    'first'\n    label = 'second')\n",
        "x = outer; (inner; 'first', label = 'second')\n",
    );
    equivalent("x = outer; (\n    1,\n    2\n)\n", "x = outer; (1, 2)\n");
    equivalent(
        "x = (combine; 'a' +\n            'b', 'c')\n",
        "x = combine; 'a' + 'b', 'c'\n",
    );
    equivalent(
        "x = (combine; 'a'\n            , 'b')\n",
        "x = combine; 'a', 'b'\n",
    );
    equivalent("x = (one; 'a' +\n        'b')\n", "x = one; 'a' + 'b'\n");
    equivalent(
        "x = (one; true\n    and true)\n",
        "x = one; true and true\n",
    );
    equivalent(
        "x = (service\n    .configure; 1 +\n        2\n    .finish;)\n",
        "x = (service.configure; 1 + 2).finish;\n",
    );
    equivalent("x = (one; not\n    false)\n", "x = one; not false\n");
    equivalent("x = (one; await\n    future)\n", "x = one; await future\n");
    equivalent("x = (call;\n    -1)\n", "x = call; -1\n");
    equivalent("x = (call;\n    >text\n)\n", "x = call; >text\n");
}
#[test]
fn callable_type_annotations_do_not_suppress_parenthesized_layout() {
    equivalent(
        "x (function to string)|none = (pick;\n        'a'\n        'b')\n",
        "x (function to string)|none = pick; 'a', 'b'\n",
    );
}

#[test]
fn comments_tabs_and_literals_preserve_argument_boundaries() {
    fn members(node: &SyntaxNode, tree: &SyntaxTree, names: &mut Vec<String>) {
        for child in &node.children {
            members(child, tree, names);
        }
        if node.kind == SyntaxKind::MemberExpression {
            names.push(
                tree.lexed.tokens[node.children[1].token_range.start]
                    .text
                    .clone(),
            );
        }
    }
    let tree = parsed(
        "x = message\n\t# chain\n\t.with-text;\n\t\t>>\n\t\t\t.ready; is text\n\t# terminal\n\t.wait;\ny = 1\n",
    );
    let mut names = Vec::new();
    members(&tree.root, &tree, &mut names);
    assert_eq!(names, ["with-text", "wait"]);
    assert_eq!(tree.root.children.len(), 2);
}

#[test]
fn layout_does_not_create_implicit_operator_continuation() {
    let source = SourceFile::new(0, "layout.trn".into(), "x = 1\n    + 2\n".to_owned());
    let output = parse(&source, lex(&source).unwrap());
    assert!(
        output
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "S1001")
    );
}

#[test]
fn layout_rejects_malformed_shapes_with_specific_diagnostics() {
    let cases = [
        ("x = service\n    .\n", &["S1014"][..]),
        ("x = service\n    .start;\n    unrelated;\n", &["S1123"][..]),
        ("x = call;\n    1,\n", &["S1122"][..]),
        ("x = call;\n    1 2\n", &["S1122"][..]),
        ("x = service\n.start;\n", &["S1017"][..]),
        ("x = service\n    . start;\n", &["S1013"][..]),
        ("x = (\n    service\n    .start;)\n", &["S1013"][..]),
    ];
    for (text, expected) in cases {
        let source = SourceFile::new(0, "layout.trn".into(), text.to_owned());
        let output = parse(&source, lex(&source).unwrap());
        let codes = output
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_ref())
            .collect::<Vec<_>>();
        assert!(
            expected.iter().all(|code| codes.contains(code)),
            "{text}: {codes:?}"
        );
        if expected == ["S1014"] {
            assert_eq!(codes, ["S1014"], "{text}: {codes:?}");
        }
    }
}

#[test]
fn parentheses_close_layout_before_outer_arguments_and_operators() {
    equivalent(
        "x = outer; (inner;\n    1\n    2), 3\n",
        "x = outer; (inner; 1, 2), 3\n",
    );
    equivalent(
        "x = (inner;\n    1\n    2) + 3\n",
        "x = (inner; 1, 2) + 3\n",
    );
    equivalent("x = (inner; 1\n    2)\n", "x = inner; 1, 2\n");
    equivalent(
        "x = outer;\n    (inner;\n        1\n        2)\n    3\n",
        "x = outer; (inner; 1, 2), 3\n",
    );
    equivalent(
        "function main;\n    x = (inner;\n        1\n        2\n)\n    y = 3\n",
        "function main;\n    x = inner; 1, 2\n    y = 3\n",
    );
    equivalent(
        "x = (message\n    /* member */ .start;\n    .finish;)\n",
        "x = (message.start;).finish;\n",
    );
    equivalent(
        "x = (\n    message\n        .start;\n        .finish;\n)\n",
        "x = (message.start;).finish;\n",
    );
}

#[test]
fn block_headers_keep_their_statement_bodies() {
    let tree = parsed("while pending;\n    work;\n    other;\nafter;\n");
    assert_eq!(tree.root.children.len(), 2);
    let loop_node = &tree.root.children[0];
    assert_eq!(loop_node.kind, SyntaxKind::WhileStatement);
    assert_eq!(loop_node.children[0].children[1].children.len(), 0);
    assert_eq!(loop_node.children[1].kind, SyntaxKind::Block);
    assert_eq!(loop_node.children[1].children.len(), 2);
    equivalent(
        "while (predicate;\n    1\n    2)\n    work;\n        3\n",
        "while predicate; 1, 2\n    work; 3\n",
    );
    for header in ["if predicate;", "for item in items;"] {
        let tree = parsed(&format!("{header}\n    work;\n"));
        let block = tree.root.children[0].children.last().unwrap();
        assert_eq!(block.kind, SyntaxKind::Block);
        assert_eq!(block.children.len(), 1);
    }
}
