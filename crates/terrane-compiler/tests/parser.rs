use terrane_compiler::{SourceFile, lexer::lex, parser::parse, syntax::SyntaxKind};

fn parse_source(text: &str) -> terrane_compiler::syntax::SyntaxTree {
    let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
    let lexed = lex(&source).unwrap();
    let parsed = parse(&source, lexed);
    assert!(
        parsed.diagnostics.is_empty(),
        "unexpected parser diagnostics: {:#?}",
        parsed.diagnostics
    );
    parsed.tree
}

fn rejected(text: &str, code: &str) {
    let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
    let lexed = lex(&source).unwrap();
    let diagnostics = parse(&source, lexed).diagnostics;
    assert!(
        diagnostics.iter().any(|diagnostic| diagnostic.code == code),
        "{diagnostics:#?}"
    );
}

#[test]
fn rejects_invalid_switch_clause_shapes() {
    for (body, code) in [
        ("switch 1\n    print; 'not a case'\n", "S1110"),
        ("switch 1\n", "S1111"),
        (
            "switch 1\n    case else\n        return\n    case else\n        return\n",
            "S1112",
        ),
        (
            "switch 1\n    case else\n        return\n    case 1\n        return\n",
            "S1113",
        ),
        ("switch 1\n    case 1,\n        return\n", "S1114"),
        (
            "switch 1\n    case else unexpected\n        return\n",
            "S1115",
        ),
        ("switch\n    case true, false\n        return\n", "S1116"),
    ] {
        rejected(body, code);
    }
}

#[test]
fn returns_recovered_tree_and_tokens_with_diagnostics() {
    let source = SourceFile::new(0, "case.trn".into(), "value =\nnext = 1\n".to_owned());
    let lexed = lex(&source).unwrap();
    let parsed = parse(&source, lexed);
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "S1019")
    );
    assert_eq!(parsed.tree.root.children.len(), 2);
    assert_eq!(
        parsed.tree.lexed.tokens.last().unwrap().kind,
        terrane_compiler::tokens::TokenKind::Eof
    );
}

fn contains(node: &terrane_compiler::syntax::SyntaxNode, kind: SyntaxKind) -> bool {
    node.kind == kind || node.children.iter().any(|child| contains(child, kind))
}

fn count(node: &terrane_compiler::syntax::SyntaxNode, kind: SyntaxKind) -> usize {
    usize::from(node.kind == kind)
        + node
            .children
            .iter()
            .map(|child| count(child, kind))
            .sum::<usize>()
}

#[test]
fn parses_lossless_declarations_and_legal_empty_blocks() {
    let text = "namespace example/app\npublic constant count int = 1\npublic async function empty throws throwable; value int\nfunction main;\n  count = count + 1\n";
    let tree = parse_source(text);
    assert!(contains(&tree.root, SyntaxKind::NamespaceDeclaration));
    assert!(contains(&tree.root, SyntaxKind::Binding));
    assert!(contains(&tree.root, SyntaxKind::Visibility));
    assert!(contains(&tree.root, SyntaxKind::DeclarationQualifier));
    assert_eq!(
        tree.lexed.tokens.last().unwrap().kind,
        terrane_compiler::tokens::TokenKind::Eof
    );
    assert!(tree.normalized().starts_with("CompilationUnit 0.."));
}

#[test]
fn parses_callable_invocation_modes_in_every_function_form() {
    let tree = parse_source(
        "mutable function update;\nconsuming async function finish;\ncallback mutable function from int to bool\nhandler = consuming function int;\n  return 1\nclass worker\n  static async function start;\n",
    );
    assert_eq!(
        count(&tree.root, SyntaxKind::DeclarationQualifier),
        7,
        "{}",
        tree.normalized()
    );
    assert_eq!(count(&tree.root, SyntaxKind::FunctionType), 1);
    assert_eq!(count(&tree.root, SyntaxKind::AnonymousFunction), 1);
}
#[test]
fn parses_variadic_parameter_markers() {
    let tree = parse_source(
        "function collect; prefix string, values int ...\nhandler = function int; values int ...\n  return 0\n",
    );
    assert_eq!(count(&tree.root, SyntaxKind::VariadicMarker), 2);
}

#[test]
fn parses_callable_invocation_modes_in_class_fields() {
    let tree = parse_source(
        "class operations\n  change mutable function to none = noop\n  finish consuming function to none = noop\n",
    );
    assert_eq!(
        count(&tree.root, SyntaxKind::FunctionType),
        2,
        "{}",
        tree.normalized()
    );
    assert_eq!(
        count(&tree.root, SyntaxKind::DeclarationQualifier),
        2,
        "{}",
        tree.normalized()
    );
}

#[test]
fn removed_effect_words_are_not_function_qualifiers() {
    for word in [
        "pure", "io", "blocks", "mutating", "mutates", "awaits", "foreign",
    ] {
        let source = SourceFile::new(
            0,
            "case.trn".into(),
            format!("namespace app\n{word} function probe;\n"),
        );
        let parsed = parse(&source, lex(&source).unwrap());
        assert!(
            !parsed.diagnostics.is_empty(),
            "{word} unexpectedly remained a function qualifier"
        );
    }
}

#[test]
fn expression_tree_respects_precedence_and_postfix_binding() {
    let tree = parse_source("result = -left + thing.member * values[1]\n");
    let assignment = &tree.root.children[0];
    assert_eq!(assignment.kind, SyntaxKind::Assignment);
    let expression = assignment.children.last().unwrap();
    assert_eq!(expression.kind, SyntaxKind::BinaryExpression);
    assert!(contains(expression, SyntaxKind::UnaryExpression));
    assert!(contains(expression, SyntaxKind::MemberExpression));
    assert!(contains(expression, SyntaxKind::IndexExpression));
}

#[test]
fn preserves_unary_operator_identity_in_the_syntax_tree() {
    let text = "observer = ref value\nowner = shared ref value\nmoved = move value\n";
    let tree = parse_source(text);
    let operators = tree
        .root
        .children
        .iter()
        .map(|binding| {
            let unary = binding.children.last().unwrap();
            assert_eq!(unary.kind, SyntaxKind::UnaryExpression);
            let operator = unary.children.first().unwrap();
            assert_eq!(operator.kind, SyntaxKind::UnaryOperator);
            text[operator.span.start..operator.span.end].trim()
        })
        .collect::<Vec<_>>();
    assert_eq!(operators, ["ref", "shared ref", "move"]);
}

#[test]
fn parses_both_postfix_increment_and_decrement() {
    let tree = parse_source("left++\nright--\n");
    assert_eq!(tree.root.children.len(), 2);
    for expression in &tree.root.children {
        assert_eq!(expression.kind, SyntaxKind::PostfixExpression);
    }
}

#[test]
fn bare_names_are_expression_statements() {
    let tree = parse_source("thing\nfunction main;\n  thing\n");
    assert_eq!(tree.root.children[0].kind, SyntaxKind::Name);
    assert_eq!(
        tree.root.children[1].children.last().unwrap().children[0].kind,
        SyntaxKind::Name
    );
}

#[test]
fn boolean_words_are_literals_not_names() {
    let tree = parse_source("enabled = true\ndisabled = false\n");

    for binding in &tree.root.children {
        assert_eq!(binding.children.last().unwrap().kind, SyntaxKind::Literal);
    }
}

#[test]
fn calls_distinguish_names_zero_arguments_and_grouped_nesting() {
    let tree = parse_source(
        "print; 'hello'\nthing\nresult = thing;\nvalue = call; first, (convert; second)\n",
    );
    assert_eq!(tree.root.children[0].kind, SyntaxKind::CallExpression);
    assert_eq!(tree.root.children[1].kind, SyntaxKind::Name);
    assert!(contains(&tree.root, SyntaxKind::Name));
    assert!(contains(&tree.root, SyntaxKind::CallExpression));
    assert!(contains(&tree.root, SyntaxKind::GroupExpression));
    let source = SourceFile::new(
        0,
        "case.trn".into(),
        "value = call; convert; input\n".to_owned(),
    );
    let diagnostic = parse(&source, lex(&source).unwrap())
        .diagnostics
        .into_iter()
        .find(|diagnostic| diagnostic.code == "S1016")
        .unwrap();
    assert_eq!(
        diagnostic.help.as_deref(),
        Some("parenthesize the nested call, for example `outer; (inner; value)`")
    );
    assert!(
        diagnostic
            .render(&source)
            .contains("\n  help: parenthesize")
    );
}

#[test]
fn grouped_binding_lookahead_requires_type_evidence() {
    for source in [
        "function main;\n  show(5)\n",
        "function main;\n  show (6)\n",
        "function main;\n  show ('value')\n",
    ] {
        let source = SourceFile::new(0, "case.trn".into(), source.to_owned());
        let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
        assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
        assert_eq!(diagnostics[0].code, "S1025");
    }

    for source in [
        "class record\n  value (string | none)\n",
        "trait record\n  value (string | none)\n",
        "function main;\n  value (string | none)\n  value = none\n",
        "function main;\n  value ((string | none))",
        "function main;\n  value (string | none) = none\n",
    ] {
        let tree = parse_source(source);
        assert!(contains(&tree.root, SyntaxKind::Binding));
        assert!(contains(&tree.root, SyntaxKind::UnionType));
    }

    // Identifier-only groups can be nominal types, not call arguments: Terrane
    // calls use `;`, so syntax must leave descriptor resolution to semantics.
    let tree = parse_source("class value\nfunction main;\n  show (value)\n");
    assert!(contains(&tree.root, SyntaxKind::Binding));
}

#[test]
fn grouped_prefix_statements_remain_expressions() {
    for operator in ["move", "not", "ref", "await"] {
        let tree = parse_source(&format!("{operator} (value)\n"));
        assert_eq!(tree.root.children[0].kind, SyntaxKind::UnaryExpression);
        assert!(!contains(&tree.root, SyntaxKind::Binding));
    }
}

#[test]
fn initialized_grouped_prefix_names_are_declarations() {
    for scope in ["class record", "trait record", "function main;"] {
        for name in ["ref", "not", "move"] {
            let tree = parse_source(&format!("{scope}\n  {name} ((int | none)) = none\n"));
            assert!(contains(&tree.root, SyntaxKind::Binding));
            assert!(!contains(&tree.root, SyntaxKind::Assignment));
        }
    }
}

#[test]
fn object_bodies_reject_executable_statements() {
    for scope in ["class record", "trait record", "interface record"] {
        for statement in [
            "print; 'lost'",
            "not (true)",
            "value = 1",
            "if true\n    print; 'lost'",
        ] {
            let text = format!("{scope}\n  {statement}\n");
            let source = SourceFile::new(0, "case.trn".into(), text);
            let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
            assert!(
                diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code == "S1043"),
                "{scope}: {statement}: {diagnostics:#?}"
            );
        }
    }
    // Enums already have a dedicated variant parser rather than a statement block.
    for statement in ["print; 'lost'", "not (true)", "value = 1"] {
        let source = SourceFile::new(
            0,
            "case.trn".into(),
            format!("enum record\n  {statement}\n"),
        );
        assert!(
            parse(&source, lex(&source).unwrap())
                .diagnostics
                .iter()
                .any(|diagnostic| matches!(diagnostic.code.as_ref(), "S1090" | "S1025"))
        );
    }
    parse_source("class record\n  value int = 1\n  function show;\n    print; this.value\n");
}

#[test]
fn distinguishes_construction_instance_members_and_static_members() {
    let tree = parse_source(
        "made = instance widget;\nchanged = value.reset;\nshared = widget::default;\n",
    );
    let construction = tree.root.children[0].children.last().unwrap();
    assert_eq!(construction.kind, SyntaxKind::CallExpression);
    assert_eq!(
        construction.children[0].kind,
        SyntaxKind::ConstructionExpression
    );
    assert_eq!(
        tree.root.children[1].children.last().unwrap().children[0].kind,
        SyntaxKind::MemberExpression
    );
    assert_eq!(
        tree.root.children[2].children.last().unwrap().children[0].kind,
        SyntaxKind::StaticMemberExpression
    );
}

#[test]
fn construction_requires_invocation_marker() {
    rejected("made = instance widget\n", "S1093");
}

#[test]
fn construction_postfix_reports_one_source_oriented_diagnostic() {
    let source = SourceFile::new(
        0,
        "case.trn".into(),
        "value int = instance widget.value;\n".to_owned(),
    );
    let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
    assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
    assert_eq!(diagnostics[0].code, "S1096");
    assert_eq!(
        diagnostics[0].help.as_deref(),
        Some("invoke construction first, for example `(instance class;).member`")
    );
}

#[test]
fn rejects_spaced_member_access_and_chained_comparisons() {
    rejected("value = print .concat\n", "S1013");
    rejected("value = a < b < c\n", "S1012");
}

#[test]
fn tail_strings_remain_literals_while_comparisons_and_shifts_parse_as_operators() {
    let tree = parse_source("message = >text\nsmall = a > b\nshifted = a >> b\n");
    assert!(contains(&tree.root.children[0], SyntaxKind::Literal));
    assert_eq!(
        tree.root.children[1].children.last().unwrap().kind,
        SyntaxKind::BinaryExpression
    );
    assert_eq!(
        tree.root.children[2].children.last().unwrap().kind,
        SyntaxKind::BinaryExpression
    );
}

#[test]
fn return_accepts_tail_and_block_string_literals() {
    let tree = parse_source(
        "function inline string;\n  return >ok\nfunction block string;\n  return >>\n    first\n    second\n",
    );
    assert_eq!(count(&tree.root, SyntaxKind::ReturnStatement), 2);
    assert_eq!(count(&tree.root, SyntaxKind::Literal), 2);
}

#[test]
fn parses_control_flow_and_recovers_at_layout_boundaries() {
    let tree = parse_source(
        "function main;\n  if ready\n    return value\n  else\n  while running\n    continue\n  for key, value in values\n    break\n  for i = 0; i < 3; i++\n    value = i\n",
    );
    assert!(contains(&tree.root, SyntaxKind::IfStatement));
    assert!(contains(&tree.root, SyntaxKind::ElseClause));
    assert!(contains(&tree.root, SyntaxKind::WhileStatement));
    assert!(contains(&tree.root, SyntaxKind::ForStatement));
    assert!(contains(&tree.root, SyntaxKind::ReturnStatement));
    assert!(contains(&tree.root, SyntaxKind::BreakStatement));
    assert!(contains(&tree.root, SyntaxKind::ContinueStatement));
    assert!(contains(&tree.root, SyntaxKind::ForTarget));
}

#[test]
fn parses_structured_select_cases_and_bindings() {
    let tree = parse_source(
        "async function choose;\n  select\n    case value int = await integer-task\n      print; value\n    case await shutdown-task\n      return\n",
    );
    assert_eq!(count(&tree.root, SyntaxKind::SelectStatement), 1);
    assert_eq!(count(&tree.root, SyntaxKind::SelectCase), 2);
    assert_eq!(count(&tree.root, SyntaxKind::Binding), 1);
}

#[test]
fn rejects_malformed_select_boundaries() {
    rejected(
        "async function choose;\n  select\n    case await only-task\n",
        "S1102",
    );
    rejected(
        "async function choose;\n  select\n    case value = prefix + await first\n    case await second\n",
        "S1103",
    );
    rejected(
        "async function choose;\n  select\n    case global value = await first\n      return\n    case await second\n      return\n",
        "S1105",
    );
    rejected(
        "async function choose;\n  select\n    case value = await first if value\n      return\n    case await second\n      return\n",
        "S1106",
    );
    rejected(
        "async function choose;\n  select\n    case default\n      return\n    case await second\n      return\n",
        "S1107",
    );
    rejected("result = select\n", "S1108");
    rejected("async function choose;\n  select\n    return\n", "S1101");
}

#[test]
fn three_clause_for_requires_grouping_for_calls() {
    parse_source("for i = (next;); i < limit; i++\n");
    rejected("for i = next; value; i < limit; i++\n", "S1016");
    parse_source("for item in (values; a, (b; c))\n  break\n");
    for text in [
        "for item.member in values\n  break\n",
        "for item + other in values\n  break\n",
    ] {
        let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
        let parsed = parse(&source, lex(&source).unwrap());
        assert_eq!(parsed.diagnostics.len(), 1, "{:#?}", parsed.diagnostics);
        assert_eq!(parsed.diagnostics[0].code, "S1009");
        assert!(contains(&parsed.tree.root, SyntaxKind::Block));
        assert!(contains(&parsed.tree.root, SyntaxKind::BreakStatement));
    }
}

#[test]
fn preserves_type_shapes_without_keywording_core_names() {
    let tree = parse_source(
        "value list of string\nmaybe int | none\ncallback function from int, string to bool\npublic visible-callback function from int to bool\nconstant stable-callback function to bool\nborrowed ref bytes\n",
    );
    assert!(contains(&tree.root, SyntaxKind::AppliedType));
    assert!(contains(&tree.root, SyntaxKind::UnionType));
    assert!(contains(&tree.root, SyntaxKind::FunctionType));
    assert!(contains(&tree.root, SyntaxKind::PrefixType));
    assert_eq!(tree.root.children[3].kind, SyntaxKind::Binding);
    assert!(contains(&tree.root.children[3], SyntaxKind::FunctionType));
}

#[test]
fn distinguishes_identity_from_type_membership() {
    let tree = parse_source(
        "same = value is a\nmember = value is a int\ncomputed = count + 1 is a int\nready = value is a int and enabled\ngrouped = value is a (int | none)\n",
    );
    assert_eq!(
        tree.root.children[0].children.last().unwrap().kind,
        SyntaxKind::BinaryExpression
    );
    assert_eq!(
        tree.root.children[1].children.last().unwrap().kind,
        SyntaxKind::TypeMembershipExpression
    );
    assert_eq!(
        tree.root.children[2].children.last().unwrap().kind,
        SyntaxKind::TypeMembershipExpression
    );
    assert_eq!(
        tree.root.children[3].children.last().unwrap().kind,
        SyntaxKind::BinaryExpression
    );
    assert!(contains(
        tree.root.children[4].children.last().unwrap(),
        SyntaxKind::GroupExpression
    ));
}

#[test]
fn rejects_tighter_operators_after_type_membership() {
    for text in [
        "value = thing is a int + 1\n",
        "value = thing is a int == other\n",
    ] {
        let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
        let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
        assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
        assert_eq!(diagnostics[0].code, "S1012");
    }
}

#[test]
fn deferred_spellings_receive_canonical_fixes() {
    let source = SourceFile::new(0, "case.trn".into(), "same = left === right\n".to_owned());
    let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
    assert_eq!(
        diagnostics[0].help.as_deref(),
        Some("use `==` for equality or `is` for identity")
    );
    for text in ["items list<string>\n", "items list<string>= value\n"] {
        let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
        let lexed = lex(&source).unwrap();
        let diagnostics = parse(&source, lexed).diagnostics;
        assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
        assert_eq!(diagnostics[0].code, "S1092");
        assert_eq!(
            diagnostics[0].help.as_deref(),
            Some("write `list of string`")
        );
    }
}

#[test]
fn binary_word_operators_do_not_look_like_bindings() {
    for operator in ["in", "is", "and", "or"] {
        let text = format!("left {operator} right\n");
        let source = SourceFile::new(0, "case.trn".into(), text);
        let parsed = parse(&source, lex(&source).unwrap());
        assert_ne!(parsed.tree.root.children[0].kind, SyntaxKind::Binding);
        if let Some(diagnostic) = parsed.diagnostics.first() {
            assert_eq!(diagnostic.primary.unwrap().start, 5);
        }
    }
}

#[test]
fn normalized_tree_retains_tokens_and_trivia() {
    assert_eq!(
        parse_source("value = 1 # note\n").normalized(),
        concat!(
            "CompilationUnit 0..17\n",
            "  Assignment 0..9\n",
            "    Name 0..5\n",
            "    Literal 8..9\n",
            "tokens\n",
            "  Identifier 0..5 \"value\"\n",
            "  Assign 6..7 \"=\"\n",
            "  Number 8..9 \"1\"\n",
            "  Newline 16..17 \"\\n\"\n",
            "  Eof 17..17 \"\"\n",
            "trivia\n",
            "  Whitespace 5..6 \" \"\n",
            "  Whitespace 7..8 \" \"\n",
            "  Whitespace 9..10 \" \"\n",
            "  LineComment 10..16 \"# note\"\n",
        )
    );
}

#[test]
fn normalized_import_tree_retains_anchors_and_aliases() {
    assert_eq!(
        parse_source("from /core/output import debug as trace\n").normalized(),
        concat!(
            "CompilationUnit 0..40\n",
            "  ImportDeclaration 0..39\n",
            "    NamespacePath 5..17\n",
            "      NamespaceAnchor 5..6\n",
            "      Name 6..10\n",
            "      Name 11..17\n",
            "    ObjectImport 25..39\n",
            "      Name 25..30\n",
            "      ImportAlias 31..39\n",
            "        Name 34..39\n",
            "tokens\n",
            "  Identifier 0..4 \"from\"\n",
            "  Operator 5..6 \"/\"\n",
            "  Identifier 6..10 \"core\"\n",
            "  Operator 10..11 \"/\"\n",
            "  Identifier 11..17 \"output\"\n",
            "  Identifier 18..24 \"import\"\n",
            "  Identifier 25..30 \"debug\"\n",
            "  Identifier 31..33 \"as\"\n",
            "  Identifier 34..39 \"trace\"\n",
            "  Newline 39..40 \"\\n\"\n",
            "  Eof 40..40 \"\"\n",
            "trivia\n",
            "  Whitespace 4..5 \" \"\n",
            "  Whitespace 17..18 \" \"\n",
            "  Whitespace 24..25 \" \"\n",
            "  Whitespace 30..31 \" \"\n",
            "  Whitespace 33..34 \" \"\n",
        )
    );
}

#[test]
fn parses_structural_import_forms_and_named_arguments() {
    let tree = parse_source(
        "from /core/output import print, debug as trace\nfrom ../sibling import item\nimport with sandboxed-import\nvalue = render; input, width = 80\n",
    );
    let import = &tree.root.children[0];
    assert_eq!(import.kind, SyntaxKind::ImportDeclaration);
    assert_eq!(import.children[0].kind, SyntaxKind::NamespacePath);
    assert_eq!(import.children[0].children.len(), 3);
    assert_eq!(
        import.children[0].children[0].kind,
        SyntaxKind::NamespaceAnchor
    );
    assert_eq!(
        tree.root.children[1].children[0].children[0].kind,
        SyntaxKind::NamespaceAnchor
    );
    assert_eq!(import.children[1].kind, SyntaxKind::ObjectImport);
    assert_eq!(import.children[2].kind, SyntaxKind::ObjectImport);
    assert_eq!(import.children[2].children[1].kind, SyntaxKind::ImportAlias);
    assert_eq!(tree.root.children[2].children[0].kind, SyntaxKind::Name);
    assert!(contains(&tree.root, SyntaxKind::CallExpression));
    assert_eq!(
        tree.root.children[3].children.last().unwrap().children[1]
            .children
            .len(),
        2
    );
}

#[test]
fn grouped_imports_preserve_aliases_and_lexical_boundaries() {
    for ending in ["", "\n", "\nvalue = 1\n"] {
        let tree = parse_source(&format!(
            "from /core/types import\n\tint8 as byte, int16,\n\tint32 as word{ending}"
        ));
        let import = &tree.root.children[0];
        assert_eq!(import.children.len(), 4);
        assert_eq!(import.children[1].children[1].kind, SyntaxKind::ImportAlias);
        assert_eq!(import.children[3].children[1].kind, SyntaxKind::ImportAlias);
        if ending.contains("value") {
            assert_eq!(tree.root.children[1].kind, SyntaxKind::Assignment);
        } else {
            assert_eq!(tree.root.children.len(), 1);
        }
    }
    let tree = parse_source(
        "function main;\n\tfrom /core/types import\n\t\tint8 as byte,\n\t\tint16\n\tvalue byte = 1\nprint; 2\n",
    );
    assert_eq!(
        tree.root.children[0]
            .children
            .last()
            .unwrap()
            .children
            .len(),
        2
    );
    assert_eq!(tree.root.children.len(), 2);
}

#[test]
fn rejects_invalid_grouped_import_boundaries() {
    for (text, message, count) in [
        (
            "from /core/types import\nint8\n",
            "expected an indented import list",
            1,
        ),
        (
            "from /core/types import\n",
            "expected an indented import list",
            1,
        ),
        (
            "from /core/types import int8,\n\tint16\n",
            "an inline import list cannot continue",
            1,
        ),
        (
            "from /core/types import int8\n\tint16\n",
            "an inline import list cannot continue",
            1,
        ),
        (
            "from /core/types import\n\tint8\n\tint16\n",
            "expected `,` between imported names",
            1,
        ),
        (
            "from /core/types import\n\tint8 int16\n",
            "expected `,` between imported names",
            1,
        ),
        (
            "from /core/types import\n\tint8,\n\t\tint16\n",
            "import items must use the same indentation",
            1,
        ),
        (
            "from /core/types import\n\tint8 as\n\tbyte\n",
            "expected an import alias after `as`",
            1,
        ),
        (
            "from /core/types import\n\tint8\n\t, int16\n",
            "a separating comma must end the previous import line",
            1,
        ),
        (
            "from /core/types import\n\tint8,\n\t, int16\n",
            "unexpected extra comma before imported name",
            1,
        ),
        (
            "from /core/types import\n\tint8,, int16\n",
            "expected an imported name",
            1,
        ),
        (
            "from /core/types import\n\tint8 + int16\n",
            "expected `,` between imported names",
            1,
        ),
        ("from /core/types import int8,\n", "trailing comma", 1),
        ("from /core/types import\n\tint8,\n", "trailing comma", 1),
        (
            "from /core/types import\n\tint8,\n\tint16,",
            "trailing comma",
            1,
        ),
    ] {
        let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
        let parsed = parse(&source, lex(&source).unwrap());
        assert_eq!(
            parsed.diagnostics.len(),
            count,
            "{text}\n{:#?}",
            parsed.diagnostics
        );
        assert!(
            parsed
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code == "S1026")
        );
        assert!(
            parsed
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains(message)),
            "{text}\n{:#?}",
            parsed.diagnostics
        );
    }
}

#[test]
fn malformed_import_item_entries_recover_once() {
    let missing_name = "expected an imported name";
    let missing_alias = "expected an import alias after `as`";
    for (items, names, message) in [
        ("\t+ int8", vec![], missing_name),
        ("\tint8,\n\t+ int32", vec!["int8"], missing_name),
        ("\tint8\n\t+ int32", vec!["int8"], missing_name),
        ("\t(int8)", vec![], missing_name),
        ("\t+ int8, int32", vec!["int32"], missing_name),
        (
            "\tint8,\n\t+ int16, int32",
            vec!["int8", "int32"],
            missing_name,
        ),
        ("\t, int8", vec!["int8"], missing_name),
        (
            "\tint8 as + alias, int32",
            vec!["int8", "int32"],
            missing_alias,
        ),
        ("\tint8 as , int32", vec!["int8", "int32"], missing_alias),
    ] {
        for nested in [false, true] {
            let text = format!("from /core/types import\n{items}\nvalue int32 = 1\n");
            let text = if nested {
                let mut body = String::from("function main;\n");
                for line in text.lines() {
                    body.push('\t');
                    body.push_str(line);
                    body.push('\n');
                }
                body
            } else {
                text
            };
            let source = SourceFile::new(0, "case.trn".into(), text);
            let parsed = parse(&source, lex(&source).unwrap());
            assert_eq!(parsed.diagnostics.len(), 1, "{:#?}", parsed.diagnostics);
            assert_eq!(parsed.diagnostics[0].code, "S1026");
            assert!(parsed.diagnostics[0].message.contains(message));
            let statements = if nested {
                &parsed.tree.root.children[0]
                    .children
                    .last()
                    .unwrap()
                    .children
            } else {
                &parsed.tree.root.children
            };
            assert_eq!(statements.len(), 2);
            assert_eq!(statements[1].kind, SyntaxKind::Binding);
            let actual = statements[0]
                .children
                .iter()
                .filter(|node| node.kind == SyntaxKind::ObjectImport)
                .filter_map(|node| {
                    node.children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)
                })
                .map(|name| &source.text()[name.span.start..name.span.end])
                .collect::<Vec<_>>();
            assert_eq!(actual, names);
        }
    }
}

#[test]
fn malformed_imports_recover_following_items_and_statements() {
    for (text, message, names, count) in [
        (
            "from /core/types import int8\n\tint16\nvalue int32 = 1\n",
            "an inline import list cannot continue",
            vec!["int8"],
            1,
        ),
        (
            "from /core/types import int8,\n\n// continuation\n\tint16\nvalue int32 = 1\n",
            "an inline import list cannot continue",
            vec!["int8"],
            1,
        ),
        (
            "from /core/types import\n\tint8,\n\t\tint16,\n\tint32\nvalue int32 = 1\n",
            "import items must use the same indentation",
            vec!["int8", "int32"],
            1,
        ),
        (
            "from /core/types import\n\tint8\n\t\tint16,\n\tint32\nvalue int32 = 1\n",
            "import items must use the same indentation",
            vec!["int8", "int32"],
            2,
        ),
        (
            "from /core/types import\n\tint8\n\t, int16\nvalue int32 = 1\n",
            "a separating comma must end the previous import line",
            vec!["int8", "int16"],
            1,
        ),
        (
            "from /core/types import\n\tint8,\n\t, int16\nvalue int32 = 1\n",
            "unexpected extra comma before imported name",
            vec!["int8", "int16"],
            1,
        ),
        (
            "from /core/types import\n\tint8,, int16,\n\tint32\nvalue int32 = 1\n",
            "expected an imported name",
            vec!["int8", "int16", "int32"],
            1,
        ),
        (
            "from /core/types import\n\tint8 + int16, int32\nvalue int32 = 1\n",
            "expected `,` between imported names",
            vec!["int8", "int32"],
            1,
        ),
        (
            "from /core/types import\n\tint8 + int16\n\tint32\nvalue int32 = 1\n",
            "expected `,` between imported names",
            vec!["int8", "int32"],
            1,
        ),
    ] {
        for nested in [false, true] {
            let text = if nested {
                let mut body = String::from("function main;\n");
                for line in text.lines() {
                    body.push('\t');
                    body.push_str(line);
                    body.push('\n');
                }
                body
            } else {
                text.to_owned()
            };
            let source = SourceFile::new(0, "case.trn".into(), text);
            let parsed = parse(&source, lex(&source).unwrap());
            assert_eq!(parsed.diagnostics.len(), count, "{:#?}", parsed.diagnostics);
            assert!(parsed.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "S1026" && diagnostic.message.contains(message)
            }));
            let statements = if nested {
                &parsed.tree.root.children[0]
                    .children
                    .last()
                    .unwrap()
                    .children
            } else {
                &parsed.tree.root.children
            };
            assert_eq!(statements.len(), 2);
            assert_eq!(statements[1].kind, SyntaxKind::Binding);
            let actual = statements[0]
                .children
                .iter()
                .filter(|node| node.kind == SyntaxKind::ObjectImport)
                .map(|node| {
                    let span = node.children[0].span;
                    &source.text()[span.start..span.end]
                })
                .collect::<Vec<_>>();
            assert_eq!(actual, names);
        }
    }
}

#[test]
fn two_token_import_binding_does_not_consume_structural_imports() {
    let tree = parse_source("import value = 1\nfrom /core/output import print\n");

    assert_eq!(tree.root.children[0].kind, SyntaxKind::Binding);
    assert_eq!(tree.root.children[1].kind, SyntaxKind::ImportDeclaration);
}

#[test]
fn parses_slash_namespace_paths_and_rejects_whitespace_separators() {
    let tree = parse_source(
        "namespace example/app\nfrom /core/output import print\nfrom ../shared/config import settings\n",
    );
    assert_eq!(tree.root.children.len(), 3);
    assert!(tree.root.children.iter().all(|node| {
        matches!(
            node.kind,
            SyntaxKind::NamespaceDeclaration | SyntaxKind::ImportDeclaration
        )
    }));

    rejected("namespace example app\n", "S1002");
    rejected("from /core output import print\n", "S1026");
    rejected("from .. shared import item\n", "S1026");
}

#[test]
fn rejects_malformed_declarations_and_reserved_constructs() {
    rejected("namespace\n", "S1002");
    rejected("value =\n", "S1019");
    rejected("function main; ,\n", "S1007");
    let source = SourceFile::new(
        0,
        "case.trn".into(),
        "public private value int\n".to_owned(),
    );
    let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
    assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
    assert_eq!(diagnostics[0].code, "S1029");
    rejected("constant global value int\n", "S1029");
    for text in ["constant public value int\n", "global private value int\n"] {
        let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
        let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
        assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
        assert_eq!(diagnostics[0].code, "S1029");
    }
    for text in ["global function main;\n", "constant function main;\n"] {
        let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
        let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
        assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
        assert_eq!(diagnostics[0].code, "S1029");
    }
    rejected("async async function work;\n", "S1029");
    rejected("mutable consuming function work;\n", "S1029");
    rejected("consuming mutable function work;\n", "S1029");
    rejected("static mutable function work;\n", "S1029");
    rejected("static consuming function work;\n", "S1029");
    rejected("async mutable function work;\n", "S1029");
    rejected("callback async mutable function to none = noop\n", "S1029");
    rejected("callback = async mutable function;\n", "S1029");
    rejected(
        "callback mutable mutable function to none = noop\n",
        "S1029",
    );
    rejected("value mutable int\n", "S1005");
    rejected("value consuming string\n", "S1005");

    rejected("catch problem\n", "S1090");
    rejected("finally\n", "S1090");
    rejected("case value\n", "S1090");
    rejected("from import thing\n", "S1026");
    rejected("from /core/output import print, , ]\n", "S1026");
    rejected("import with anything at all\n", "S1025");
}

#[test]
fn requires_parameter_markers_on_every_function_header() {
    for source in [
        "function main\n",
        "async function load response throws throwable\n",
        "handler = function\n",
    ] {
        rejected(source, "S1038");
    }

    rejected("function load response throws;\n", "S1039");
}

#[test]
fn parses_freely_wrapped_parameter_and_call_groups() {
    let tree = parse_source(
        "function connect response; (\n  host string,\n  port int)\n\
         function main;\n\
           print; (foo; (\n\
             first,\n\
             second, third\n\
           ))\n\
           print; (\n\
             format;\n\
               value\n\
           )\n",
    );
    assert!(contains(&tree.root, SyntaxKind::ParameterList));
    assert!(contains(&tree.root, SyntaxKind::CallExpression));

    parse_source("function compact response; (\n  host string, port int)\n");
}

#[test]
fn rejects_an_unclosed_multiline_parameter_group() {
    rejected("function connect response; (\n  host string,\n", "S1040");
}

#[test]
fn rejects_every_reserved_statement_keyword() {
    for keyword in [
        "yield", "unsafe", "rust", "label", "goto", "when", "use", "case",
    ] {
        rejected(&format!("{keyword}\n"), "S1090");
    }
}

#[test]
fn rejects_language_keywords_in_declaration_name_positions() {
    for source in ["function function;\n", "class instance\n"] {
        rejected(source, "S1095");
    }
}

#[test]
fn rejects_invalid_postfix_and_control_flow_boundaries() {
    rejected("value = thing.\n", "S1014");
    rejected("value = values[\n", "S1019");
    rejected("break value\n", "S1011");
    rejected("for item values\n", "S1009");
    rejected("if\n", "S1019");
}

#[test]
fn rejects_non_associative_identity_and_recovers_layout_errors_once() {
    rejected("same = a is b is c\n", "S1012");
    rejected("same = a is a int is b\n", "S1012");

    for (text, code) in [
        ("value = 1\n    deeper = 2\n", "S1001"),
        ("if a = b\n    value = 1\n", "S1037"),
        ("while a = b\n    value = 1\n", "S1037"),
    ] {
        let source = SourceFile::new(0, "case.trn".into(), text.to_owned());
        let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
        assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
        assert_eq!(diagnostics[0].code, code);
    }

    let source = SourceFile::new(0, "case.trn".into(), "if a = b\n    value = 1\n".to_owned());
    let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
    assert_eq!(
        diagnostics[0].help.as_deref(),
        Some("use `==` for equality")
    );

    let source = SourceFile::new(
        0,
        "case.trn".into(),
        "class item extends\nfunction main;\n".to_owned(),
    );
    let diagnostics = parse(&source, lex(&source).unwrap()).diagnostics;
    assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
    assert_eq!(diagnostics[0].code, "S1041");

    let source = SourceFile::new(
        0,
        "case.trn".into(),
        "value = 1\n    deeper = 2\nafter = 3\n".to_owned(),
    );
    let parsed = parse(&source, lex(&source).unwrap());
    assert_eq!(parsed.diagnostics.len(), 1, "{:#?}", parsed.diagnostics);
    assert_eq!(parsed.tree.root.children.len(), 2);

    let source = SourceFile::new(
        0,
        "case.trn".into(),
        "function main;\n  value = 1\n    deeper = 2\n  after = 3\n".to_owned(),
    );
    let parsed = parse(&source, lex(&source).unwrap());
    assert_eq!(parsed.diagnostics.len(), 1, "{:#?}", parsed.diagnostics);
    assert_eq!(parsed.diagnostics[0].code, "S1001");
    let block = parsed.tree.root.children[0].children.last().unwrap();
    assert_eq!(block.children.len(), 2);
}

#[test]
fn scoped_regions_preserve_nested_statement_and_call_boundaries() {
    let tree = parse_source(
        "function main;\n    with scoped-access guard = (acquire; (1, 2)) (\n        inspect; guard\n        with (local-scope;) child = (borrow; guard) (\n            inspect; child\n        )\n        inspect; guard\n    )\n    after;\n",
    );
    let body = tree.root.children[0].children.last().unwrap();
    assert_eq!(body.children.len(), 2);
    let region = &body.children[0];
    assert_eq!(region.kind, SyntaxKind::WithRegion);
    assert_eq!(region.modifier_applications.len(), 1);
    let acquisition = region.children[0].children.last().unwrap();
    let call = acquisition.children.first().unwrap();
    assert_eq!(call.kind, SyntaxKind::CallExpression);
    assert_eq!(call.children[1].children.len(), 2);
    let inner = &region.children[1].children[1];
    assert_eq!(inner.kind, SyntaxKind::WithRegion);
    assert_eq!(inner.children[1].children.len(), 1);
    assert_eq!(region.children[1].children.len(), 3);
    assert_eq!(body.children[1].kind, SyntaxKind::CallExpression);
}

#[test]
fn declaration_modifiers_keep_typed_arguments_separate_from_initializer() {
    let tree = parse_source("with first, (second; 64, enabled = true) global count int = 3\n");
    let declaration = &tree.root.children[0];
    assert_eq!(declaration.kind, SyntaxKind::Binding);
    assert_eq!(declaration.modifier_applications.len(), 2);
    let arguments = &declaration.modifier_applications[1].children[1];
    assert_eq!(arguments.children.len(), 2);
    assert_eq!(arguments.children[1].children.len(), 2);
    assert_eq!(
        declaration.children.last().unwrap().kind,
        SyntaxKind::Literal
    );
}

#[test]
fn rejects_structural_keyword_in_modifier_position() {
    rejected("with scoped-access, global item = 1\n", "S1120");
}
