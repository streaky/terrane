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
        "x = outer;\n    (inner;\n        'first',\n        label = 'second'),\n    final = 3\ny = 4\n",
        "x = outer; (inner; 'first', label = 'second'), final = 3\ny = 4\n",
    );
    equivalent(
        "x = outer; 1,\n    2, 3,\n    final = 4\n",
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
        "x = outer; (inner;\n    'first',\n    label = 'second')\n",
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
        "x = (service\n    .configure; 1 +\n        2\n    .finish;)\n",
        "x = (service.configure; 1 + 2).finish;\n",
    );
    equivalent("x = (one; not\n    false)\n", "x = one; not false\n");
    equivalent("x = (one; await\n    future)\n", "x = one; await future\n");
    equivalent("x = (call;\n    -1)\n", "x = call; -1\n");
    equivalent("x = (call;\n    >text\n)\n", "x = call; >text\n");
}

#[test]
fn negative_and_text_arguments_require_commas_in_both_layout_forms() {
    equivalent("x = (sub;\n    5,\n    -2)\n", "x = sub; 5, -2\n");
    equivalent(
        "print;\n    sub;\n        5,\n        -2\n",
        "print; (sub; 5, -2)\n",
    );
    equivalent("x = (sub;\n    5\n    -2)\n", "x = sub; 5 - 2\n");
    equivalent("x = sub;\n    5\n    -2\n", "x = sub; 5 - 2\n");
    equivalent(
        "x = (combine;\n    'a' +\n        'b',\n    >b text\n)\n",
        "x = combine; 'a' + 'b', >b text\n",
    );
    equivalent(
        "x = combine;\n    'a' +\n        'b',\n    >b text\n",
        "x = combine; 'a' + 'b', >b text\n",
    );
}

#[test]
fn comma_is_required_and_trailing_commas_are_rejected_at_the_comma() {
    for text in [
        "x = call; 1,",
        "print; 'a',",
        "x = (call; 1,)",
        "x = (call; 1, 2,)",
        "x = (call; 1,\n    2,\n)\n",
        "x = call; (1,\n    2,\n)\n",
        "x = outer; (inner; 1, 2,)",
        "x = outer;\n    inner;\n        1,\n        2,\n    3\n",
        "x = outer;\n    inner;\n        1,\n",
    ] {
        let source = SourceFile::new(0, "layout.trn".into(), text.to_owned());
        let output = parse(&source, lex(&source).unwrap());
        let diagnostic = output
            .diagnostics
            .iter()
            .find(|d| d.code == "S1122")
            .unwrap();
        let span = diagnostic.primary.unwrap();
        assert_eq!(&text[span.start..span.end], ",", "{text}");
        assert_eq!(span.start, text.rfind(',').unwrap(), "{text}");
    }
}

#[test]
fn nested_layout_dedent_ends_argument_but_comma_cannot_cross_its_block() {
    equivalent(
        "x = outer;\n    inner;\n        1,\n        2\n    3\n",
        "x = outer; (inner; 1, 2), 3\n",
    );
    equivalent(
        "x = outer;\n    inner; 1, 2\n",
        "x = outer; (inner; 1, 2)\n",
    );
    equivalent(
        "x = outer;\n    service\n        .configure; 1\n        .finish;\n    2\n",
        "x = outer; ((service.configure; 1).finish;), 2\n",
    );
    let text = "x = outer;\n    inner;\n        1,\n        2,\n    3\n";
    let source = SourceFile::new(0, "layout.trn".into(), text.to_owned());
    let output = parse(&source, lex(&source).unwrap());
    let diagnostic = output
        .diagnostics
        .iter()
        .find(|d| d.code == "S1122")
        .unwrap();
    let span = diagnostic.primary.unwrap();
    assert_eq!(&text[span.start..span.end], ",");
    assert_eq!(span.start, text.rfind(',').unwrap());
}

#[test]
fn adjacent_complete_layout_arguments_need_a_comma() {
    for text in ["x = call;\n    1\n    2\n", "x = (call;\n    1\n    2)\n"] {
        let source = SourceFile::new(0, "layout.trn".into(), text.to_owned());
        let output = parse(&source, lex(&source).unwrap());
        assert!(
            output
                .diagnostics
                .iter()
                .any(|d| d.code == "S1122" && d.primary.unwrap().start == text.rfind('2').unwrap()),
            "{text}: {:?}",
            output.diagnostics
        );
    }
}

#[test]
fn callable_type_annotations_do_not_suppress_parenthesized_layout() {
    equivalent(
        "x (function to string)|none = (pick;\n        'a',\n        'b')\n",
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
        "x = outer; (inner;\n    1,\n    2), 3\n",
        "x = outer; (inner; 1, 2), 3\n",
    );
    equivalent(
        "x = (inner;\n    1,\n    2) + 3\n",
        "x = (inner; 1, 2) + 3\n",
    );
    equivalent("x = (inner; 1,\n    2)\n", "x = inner; 1, 2\n");
    equivalent(
        "x = outer;\n    (inner;\n        1,\n        2)\n    3\n",
        "x = outer; (inner; 1, 2), 3\n",
    );
    equivalent(
        "function main;\n    x = (inner;\n        1,\n        2\n)\n    y = 3\n",
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
        "while (predicate;\n    1,\n    2)\n    work;\n        3\n",
        "while predicate; 1, 2\n    work; 3\n",
    );
    for header in ["if predicate;", "for item in items;"] {
        let tree = parsed(&format!("{header}\n    work;\n"));
        let block = tree.root.children[0].children.last().unwrap();
        assert_eq!(block.kind, SyntaxKind::Block);
        assert_eq!(block.children.len(), 1);
    }
}

#[test]
fn wrapped_argument_expressions_preserve_names_and_type_tests() {
    equivalent("x = call;\n    name\n        = 2\n", "x = call; name = 2\n");
    equivalent(
        "x = call;\n    value is\n        a int\n",
        "x = call; value is a int\n",
    );
    equivalent(
        "x = call;\n    value is a\n        list of\n            int\n",
        "x = call; value is a list of int\n",
    );
    equivalent(
        "x = call;\n    instance\n        example;\n",
        "x = call; (instance example;)\n",
    );
    equivalent(
        "x = call;\n    factory\n        of int;\n",
        "x = call; (factory of int;)\n",
    );
}

#[test]
fn anonymous_parameter_headers_do_not_consume_their_body_as_argument_wrapping() {
    equivalent(
        "x = apply;\n    function string; value string\n        return value\n",
        "x = apply; (function string; value string\n    return value)\n",
    );
    equivalent(
        "x = (apply; function string; (value string)\n    return value)\n",
        "x = apply; (function string; value string\n    return value)\n",
    );
    equivalent(
        "x = (apply; function string; value string = 'ready'\n    return value)\n",
        "x = apply; (function string; value string = 'ready'\n    return value)\n",
    );
}

#[test]
fn inline_call_columns_seal_same_level_arguments_and_separators() {
    equivalent(
        "x = wrap;\n    opt; 'a',\n    'b'\n",
        "x = wrap; (opt; 'a'), 'b'\n",
    );
    equivalent(
        "x = wrap;\n    opt; 'a', 'b'\n",
        "x = wrap; (opt; 'a', 'b')\n",
    );
    equivalent(
        "x = wrap;\n    opt; 'a', 'b',\n    'c'\n",
        "x = wrap; (opt; 'a', 'b'), 'c'\n",
    );
    equivalent(
        "x = wrap;\n    opt; 'a',\n        'b'\n",
        "x = wrap; (opt; 'a', 'b')\n",
    );
    equivalent(
        "x = wrap;\n    opt; 'a',\n        'b'\n    'c'\n",
        "x = wrap; (opt; 'a', 'b'), 'c'\n",
    );
    equivalent(
        "x = wrap;\n\topt; 'a',\n\t# next outer argument\n\n\t'b'\n",
        "x = wrap; (opt; 'a'), 'b'\n",
    );
}

#[test]
fn outer_parentheses_preserve_layout_column_ownership_at_nonzero_baselines() {
    equivalent(
        "x = (wrap;\n    (opt; 'a',\n    'b'),\n    'c')\n",
        "x = wrap; (opt; 'a', 'b'), 'c'\n",
    );
    equivalent(
        "function main;\n    x = (wrap;\n        opt; 'a',\n        'b')\n",
        "function main;\n    x = wrap; (opt; 'a'), 'b'\n",
    );
    equivalent(
        "x = ((wrap;\n    opt; 'a',\n    'b'))\n",
        "x = wrap; (opt; 'a'), 'b'\n",
    );
    equivalent(
        "x = (wrap;\n    opt; 'a', 'b')\n",
        "x = wrap; (opt; 'a', 'b')\n",
    );
    equivalent(
        "x = (wrap;\n    opt;\n        'a'\n    'b')\n",
        "x = wrap; (opt; 'a'), 'b'\n",
    );
}

#[test]
fn deeper_and_grouped_binary_continuations_are_valid_but_shallow_is_not() {
    for source in [
        "x = wrap;\n    opt; 1 +\n        2,\n    3\n",
        "x = wrap;\n    opt; (1 +\n    2),\n    3\n",
        "x = wrap;\n    opt; 1 +\n        (2),\n    3\n",
    ] {
        let file = SourceFile::new(0, "layout.trn".into(), source.to_owned());
        let output = parse(&file, lex(&file).unwrap());
        assert!(
            output.diagnostics.is_empty(),
            "{source}: {:?}",
            output.diagnostics
        );
    }
    let source = "x = wrap;\n    opt; 1 +\n    2,\n    3\n";
    let file = SourceFile::new(0, "layout.trn".into(), source.to_owned());
    let output = parse(&file, lex(&file).unwrap());
    assert_eq!(
        output
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_ref())
            .collect::<Vec<_>>(),
        ["S1122"],
        "{:?}",
        output.diagnostics
    );
    let diagnostics = output
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "S1122")
        .collect::<Vec<_>>();
    assert_eq!(diagnostics.len(), 1, "{:?}", output.diagnostics);
    assert!(
        diagnostics[0]
            .help
            .as_ref()
            .is_some_and(|help| help.contains("indent") && help.contains("parenthes")),
        "{:?}",
        diagnostics[0]
    );
}

#[test]
fn deeper_inline_comma_before_dedent_is_rejected_at_comma() {
    for source in [
        "x = wrap;\n    opt; 'a',\n        'b',\n    'c'\n",
        "x = wrap;\n    opt; 'a',\n        'b',\n            'c',\n    'd'\n",
    ] {
        let file = SourceFile::new(0, "layout.trn".into(), source.to_owned());
        let output = parse(&file, lex(&file).unwrap());
        let diagnostics = output
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "S1122")
            .collect::<Vec<_>>();
        assert!(
            !diagnostics.is_empty(),
            "{source}: {:?}",
            output.diagnostics
        );
        for diagnostic in diagnostics {
            let span = diagnostic.primary.unwrap();
            assert_eq!(&source[span.start..span.end], ",", "{source}");
            assert!(
                diagnostic.help.as_ref().is_some_and(|help| {
                    help.contains("dedent") && help.contains("remove this comma")
                }),
                "{source}: {diagnostic:?}"
            );
        }
    }
}

#[test]
fn call_line_commas_explain_layout_dedents() {
    for (source, expected) in [
        (
            "x = wrap;\n    wrap;\n        opt; 'a',\n    'c'\n",
            "x = wrap; (wrap; (opt; 'a')), 'c'\n",
        ),
        (
            "x = wrap;\n    opt; 'a',\nprint; x\n",
            "x = wrap; (opt; 'a')\nprint; x\n",
        ),
        (
            "x = wrap;\n    'z',\n    opt; 'a',\nprint; x\n",
            "x = wrap; 'z', (opt; 'a')\nprint; x\n",
        ),
        (
            "x = wrap;\n    'a',\nprint; x\n",
            "x = wrap; 'a'\nprint; x\n",
        ),
    ] {
        let file = SourceFile::new(0, "layout.trn".into(), source.to_owned());
        let output = parse(&file, lex(&file).unwrap());
        let [diagnostic] = output.diagnostics.as_slice() else {
            panic!("{source}: {:?}", output.diagnostics);
        };
        assert_eq!(diagnostic.code, "S1122", "{source}");
        let span = diagnostic.primary.unwrap();
        assert_eq!(span.start, source.rfind(',').unwrap(), "{source}");
        assert_eq!(&source[span.start..span.end], ",", "{source}");
        assert!(
            diagnostic.help.as_ref().is_some_and(|help| {
                help.contains("dedent") && help.contains("remove this comma")
            }),
            "{source}: {diagnostic:?}"
        );
        let mut repaired = source.to_owned();
        repaired.replace_range(span.start..span.end, "");
        equivalent(&repaired, expected);
    }
    for source in [
        "x = (wrap; 'a', 'b',)\n",
        "function main;\n    print; 'a',\n",
    ] {
        let file = SourceFile::new(0, "layout.trn".into(), source.to_owned());
        let output = parse(&file, lex(&file).unwrap());
        let [diagnostic] = output.diagnostics.as_slice() else {
            panic!("{source}: {:?}", output.diagnostics);
        };
        assert_eq!(diagnostic.code, "S1122", "{source}");
        assert_eq!(diagnostic.help, None, "{source}");
    }
}

#[test]
fn explicit_parentheses_override_inline_call_columns() {
    equivalent(
        "x = wrap;\n    (opt; 'a',\n    'b'),\n    'c'\n",
        "x = wrap; (opt; 'a', 'b'), 'c'\n",
    );
    equivalent(
        "x = (wrap;\n    opt; 'a',\n    'b')\n",
        "x = wrap; (opt; 'a'), 'b'\n",
    );
    equivalent(
        "x = wrap;\n    opt; ('a' +\n    'b'),\n    'c'\n",
        "x = wrap; (opt; ('a' + 'b')), 'c'\n",
    );
}

#[test]
fn nested_separator_spans_belong_to_their_actual_argument_list() {
    fn collect(node: &SyntaxNode, tree: &SyntaxTree, out: &mut Vec<(String, usize)>) {
        if node.kind == SyntaxKind::CallExpression {
            let name = &tree.lexed.tokens[node.children[0].token_range.start].text;
            out.extend(
                node.children[1]
                    .argument_separators(&tree.lexed)
                    .map(|span| (name.clone(), span.start)),
            );
        }
        for child in &node.children {
            collect(child, tree, out);
        }
    }
    let source = "x = wrap;\n    opt; 'a', 'b',\n    'c'\n";
    let tree = parsed(source);
    let mut separators = Vec::new();
    collect(&tree.root, &tree, &mut separators);
    separators.sort_by_key(|(_, offset)| *offset);
    assert_eq!(
        separators,
        [
            ("opt".to_owned(), source.find(',').unwrap()),
            ("wrap".to_owned(), source.rfind(',').unwrap()),
        ]
    );
}

#[test]
fn column_sealing_still_requires_outer_separators() {
    for source in [
        "x = wrap;\n    opt; 'a'\n    'b'\n",
        "x = outer;\n    inner;\n        1,\n        2,\n    3\n",
        "x = wrap;\n    opt; 'a',\n",
    ] {
        let file = SourceFile::new(0, "layout.trn".into(), source.to_owned());
        let output = parse(&file, lex(&file).unwrap());
        assert!(
            output
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "S1122"),
            "{source}: {:?}",
            output.diagnostics
        );
        assert!(
            !output
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "S1016")
        );
    }
}

#[test]
fn text_marker_mistakes_explain_the_call_argument_boundary() {
    let source = SourceFile::new(
        0,
        "layout.trn".into(),
        "x = outer;\n    inner; 'a'\n    >second text\n".to_owned(),
    );
    let output = parse(&source, lex(&source).unwrap());
    let diagnostic = output
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "S1122")
        .unwrap();
    assert!(
        diagnostic
            .help
            .as_ref()
            .is_some_and(|help| help.contains("comparison") && help.contains("`,`"))
    );
    let ordinary = SourceFile::new(
        0,
        "layout.trn".into(),
        "x = outer;\n    1\n    2\n".to_owned(),
    );
    let output = parse(&ordinary, lex(&ordinary).unwrap());
    let diagnostic = output
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "S1122")
        .unwrap();
    assert_eq!(diagnostic.help, None);
}
