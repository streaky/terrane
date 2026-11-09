use super::{ParsedDocument, ProtocolError, Snapshot, ToolingEngine};
use crate::syntax::{SyntaxKind, SyntaxNode};
use crate::{Span, semantics::FunctionContract};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallArgumentContext {
    pub callee_name: String,
    pub callee_span: Span,
    pub arguments_span: Span,
    pub active_argument: usize,
    pub argument_name: Option<String>,
    pub positional_index: usize,
    pub construction: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterInlayHint {
    pub offset: usize,
    pub label: String,
}

fn enclosing_call<'a>(
    node: &'a SyntaxNode,
    offset: usize,
    lexed: &crate::tokens::LexedSource,
) -> Option<&'a SyntaxNode> {
    if node.kind == SyntaxKind::CallExpression
        && argument_list(node).is_some_and(|list| {
            list.argument_separators(lexed)
                .any(|separator| separator.start <= offset && offset <= separator.end)
        })
    {
        return Some(node);
    }
    if let Some(found) = node
        .children
        .iter()
        .find_map(|child| enclosing_call(child, offset, lexed))
    {
        return Some(found);
    }
    if node.kind != SyntaxKind::CallExpression || node.span.start > offset {
        return None;
    }
    let mut end = node.span.end;
    if let Some(arguments) = argument_list(node)
        && let Some(separator) = arguments
            .argument_separators(lexed)
            .filter(|separator| separator.end >= end)
            .last()
    {
        end = lexed
            .tokens
            .iter()
            .find(|token| token.span.start >= separator.end)
            .map_or(separator.end, |token| token.span.start);
    }
    (offset <= end).then_some(node)
}

fn argument_list(call: &SyntaxNode) -> Option<&SyntaxNode> {
    call.children
        .iter()
        .find(|child| child.kind == SyntaxKind::ArgumentList)
}

fn callee_name(document: &ParsedDocument, call: &SyntaxNode) -> Option<(String, Span)> {
    let mut named = call.children.first()?;
    if named.kind == SyntaxKind::ConstructionExpression {
        named = named.children.first()?;
    }
    while matches!(
        named.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
    ) {
        named = named.children.first()?;
    }
    let leaf = named
        .children
        .last()
        .filter(|child| child.kind == SyntaxKind::Name)
        .unwrap_or(named);
    let name = document
        .source
        .text()
        .get(leaf.span.start..leaf.span.end)?
        .to_owned();
    Some((name, leaf.span))
}

fn argument_name(document: &ParsedDocument, argument: &SyntaxNode) -> Option<String> {
    argument
        .children
        .first()
        .filter(|name| argument.children.len() > 1 && name.kind == SyntaxKind::Name)
        .and_then(|name| document.source.text().get(name.span.start..name.span.end))
        .map(str::to_owned)
}

fn active_argument(list: &SyntaxNode, offset: usize, lexed: &crate::tokens::LexedSource) -> usize {
    let separators_before = list
        .argument_separators(lexed)
        .take_while(|separator| separator.end <= offset)
        .count();
    let started = list
        .children
        .iter()
        .take_while(|argument| argument.span.start <= offset)
        .count();
    separators_before
        .max(started.saturating_sub(1))
        .min(list.children.len())
}

fn shadowed(node: &SyntaxNode, offset: usize, name: &str, source: &str) -> bool {
    let binding_name = if node.kind == SyntaxKind::Assignment {
        node.children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name)
    } else if matches!(
        node.kind,
        SyntaxKind::Binding
            | SyntaxKind::Parameter
            | SyntaxKind::CatchBinding
            | SyntaxKind::ForTarget
    ) {
        node.children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
    } else {
        None
    };
    if node.span.start < offset
        && binding_name
            .is_some_and(|child| source.get(child.span.start..child.span.end) == Some(name))
    {
        return true;
    }
    node.children.iter().any(|child| {
        (child.span.start <= offset && offset <= child.span.end
            || matches!(
                child.kind,
                SyntaxKind::Binding
                    | SyntaxKind::Assignment
                    | SyntaxKind::ForTarget
                    | SyntaxKind::Parameter
                    | SyntaxKind::ParameterList
                    | SyntaxKind::CatchBinding
            ))
            && shadowed(child, offset, name, source)
    })
}
fn collect<'a>(
    node: &'a SyntaxNode,
    document: &'a ParsedDocument,
    requested: &str,
    construction: bool,
    owner: Option<&'a str>,
    inside_callable: bool,
    selected_match: &mut Option<(&'a SyntaxNode, Option<&'a str>)>,
) -> bool {
    let class_name = if node.kind == SyntaxKind::ClassDeclaration {
        node.children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .and_then(|name| document.source.text().get(name.span.start..name.span.end))
    } else {
        owner
    };
    if node.kind == SyntaxKind::FunctionDeclaration && !inside_callable {
        let name = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .and_then(|name| document.source.text().get(name.span.start..name.span.end));
        let selected = if construction {
            class_name == Some(requested) && name == Some("construct")
        } else {
            class_name.is_none() && name == Some(requested)
        };
        if selected && selected_match.replace((node, class_name)).is_some() {
            return false;
        }
    }
    let in_callable = inside_callable || node.kind == SyntaxKind::FunctionDeclaration;
    node.children.iter().all(|child| {
        collect(
            child,
            document,
            requested,
            construction,
            class_name,
            in_callable,
            selected_match,
        )
    })
}
fn source_declaration<'a>(
    document: &'a ParsedDocument,
    context: &CallArgumentContext,
) -> Option<(&'a SyntaxNode, Option<&'a str>)> {
    let call = find_call(&document.tree.root, context.arguments_span)?;
    let mut callee = call.children.first()?;
    if callee.kind == SyntaxKind::ConstructionExpression {
        callee = callee.children.first()?;
    }
    while matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
    ) {
        callee = callee.children.first()?;
    }
    // A leaf method name is not authority to select an unrelated top-level function.
    if callee.kind != SyntaxKind::Name {
        return None;
    }
    if shadowed(
        &document.tree.root,
        context.callee_span.start,
        &context.callee_name,
        document.source.text(),
    ) {
        return None;
    }
    let mut selected_match = None;
    if !collect(
        &document.tree.root,
        document,
        &context.callee_name,
        context.construction,
        None,
        false,
        &mut selected_match,
    ) {
        return None;
    }
    selected_match
}

pub(super) fn source_signature(
    document: &ParsedDocument,
    context: &CallArgumentContext,
) -> Option<super::CallableSignature> {
    let (function, owner) = source_declaration(document, context)?;
    let parameter_list = function
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::ParameterList)?;
    let parameters = parameter_list
        .children
        .iter()
        .map(|parameter| {
            let name = parameter
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)?;
            let name = document.source.text().get(name.span.start..name.span.end)?;
            let ty = parameter
                .children
                .iter()
                .find(|child| {
                    matches!(
                        child.kind,
                        SyntaxKind::TypeExpression | SyntaxKind::FunctionType
                    )
                })
                .and_then(|ty| document.source.text().get(ty.span.start..ty.span.end))?;
            let optional = parameter.children.iter().any(|child| {
                !matches!(
                    child.kind,
                    SyntaxKind::Name
                        | SyntaxKind::TypeExpression
                        | SyntaxKind::FunctionType
                        | SyntaxKind::VariadicMarker
                )
            });
            let variadic = parameter
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::VariadicMarker);
            Some(format!(
                "{name}{}: {ty}{}",
                if optional { "?" } else { "" },
                if variadic { "..." } else { "" }
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    let result = function
        .children
        .iter()
        .take_while(|child| child.kind != SyntaxKind::ParameterList)
        .find(|child| {
            matches!(
                child.kind,
                SyntaxKind::TypeExpression | SyntaxKind::FunctionType
            )
        })
        .and_then(|ty| document.source.text().get(ty.span.start..ty.span.end))
        .unwrap_or("none");
    let active_parameter =
        (!parameters.is_empty()).then(|| {
            context
                .argument_name
                .as_ref()
                .and_then(|name| {
                    parameter_list.children.iter().position(|parameter| {
                        parameter.children.first().and_then(|node| {
                            document.source.text().get(node.span.start..node.span.end)
                        }) == Some(name)
                    })
                })
                .unwrap_or_else(|| context.positional_index.min(parameters.len() - 1))
        });
    Some(super::CallableSignature {
        label: format!(
            "{}({}) -> {result}",
            owner.as_ref().map_or_else(
                || context.callee_name.clone(),
                |owner| format!(
                    "{owner}::{}",
                    if context.construction {
                        "construct"
                    } else {
                        context.callee_name.as_str()
                    }
                ),
            ),
            parameters.join(", ")
        ),
        parameters,
        active_parameter,
    })
}
impl ToolingEngine {
    /// Returns parameter labels at argument roots in the requested byte range.
    ///
    /// # Errors
    /// Returns a protocol error for stale snapshots, unknown sources, or invalid ranges.
    pub fn parameter_inlay_hints(
        &self,
        snapshot_id: &str,
        uri: &str,
        start: usize,
        end: usize,
    ) -> Result<Vec<ParameterInlayHint>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let text = document.source.text();
        if start > end
            || end > text.len()
            || !text.is_char_boundary(start)
            || !text.is_char_boundary(end)
        {
            return Err(ProtocolError::new(
                "invalid-position",
                "range must use valid UTF-8 byte boundaries within the source",
            ));
        }
        Ok(collect_inlay_hints(snapshot, document, start, end))
    }

    /// Returns the syntactic argument owner, independently of semantic analysis.
    ///
    /// # Errors
    /// Returns a protocol error for stale snapshots, unknown sources, or invalid positions.
    pub fn call_argument_context(
        &self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
    ) -> Result<Option<CallArgumentContext>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        if offset > document.source.text().len() || !document.source.text().is_char_boundary(offset)
        {
            return Err(ProtocolError::new(
                "invalid-position",
                "position must be a UTF-8 byte boundary within the source",
            ));
        }
        let Some(call) = enclosing_call(&document.tree.root, offset, &document.tree.lexed) else {
            return Ok(None);
        };
        let Some(list) = argument_list(call) else {
            return Ok(None);
        };
        let Some((name, callee_span)) = callee_name(document, call) else {
            return Ok(None);
        };
        let active = active_argument(list, offset, &document.tree.lexed);
        let argument = list.children.get(active);
        let positional_index = list.children[..active]
            .iter()
            .filter(|item| argument_name(document, item).is_none())
            .count();
        Ok(Some(CallArgumentContext {
            callee_name: name,
            callee_span,
            arguments_span: list.span,
            active_argument: active,
            argument_name: argument.and_then(|item| argument_name(document, item)),
            positional_index,
            construction: call
                .children
                .first()
                .is_some_and(|callee| callee.kind == SyntaxKind::ConstructionExpression),
        }))
    }

    /// Returns a canonical selected callable contract for the context's owning call.
    pub(super) fn selected_contract_for_context(
        snapshot: &Snapshot,
        document: &ParsedDocument,
        context: &CallArgumentContext,
    ) -> Option<FunctionContract> {
        let semantic = snapshot.semantic.as_ref()?;
        let unit = semantic
            .units
            .iter()
            .find(|unit| unit.source.id() == document.source.id())?;
        let call = find_call(&unit.tree.root, context.arguments_span)?;
        crate::semantics::selected_callable_contract(semantic, unit, call, call.is_unsafe_call)
    }
}

fn find_call(node: &SyntaxNode, arguments_span: Span) -> Option<&SyntaxNode> {
    if matches!(
        node.kind,
        SyntaxKind::CallExpression | SyntaxKind::ConstructionExpression
    ) && argument_list(node).is_some_and(|list| list.span == arguments_span)
    {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_call(child, arguments_span))
}

pub(super) fn parameter_index(contract: &FunctionContract, context: &CallArgumentContext) -> usize {
    if let Some(name) = &context.argument_name
        && let Some(index) = contract
            .parameters
            .iter()
            .position(|parameter| &parameter.name == name)
    {
        return index;
    }
    context
        .positional_index
        .min(contract.parameters.len().saturating_sub(1))
}

pub(super) fn collect_inlay_hints(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    start: usize,
    end: usize,
) -> Vec<ParameterInlayHint> {
    fn visit(
        snapshot: &Snapshot,
        document: &ParsedDocument,
        node: &SyntaxNode,
        start: usize,
        end: usize,
        hints: &mut Vec<ParameterInlayHint>,
    ) {
        if matches!(
            node.kind,
            SyntaxKind::CallExpression | SyntaxKind::ConstructionExpression
        ) && let (Some(list), Some((name, _))) =
            (argument_list(node), callee_name(document, node))
            && let Some(semantic) = &snapshot.semantic
            && let Some(unit) = semantic
                .units
                .iter()
                .find(|unit| unit.source.id() == document.source.id())
            && let Some(contract) = crate::semantics::selected_callable_contract(
                semantic,
                unit,
                node,
                node.is_unsafe_call,
            )
        {
            let mut positional = 0;
            for argument in &list.children {
                let named = argument_name(document, argument);
                let parameter = named
                    .as_ref()
                    .and_then(|n| contract.parameters.iter().position(|p| &p.name == n))
                    .unwrap_or_else(|| positional.min(contract.parameters.len().saturating_sub(1)));
                if named.is_none() {
                    positional += 1;
                }
                if let Some(parameter) = contract.parameters.get(parameter) {
                    let offset = argument.span.start;
                    if start <= offset && offset <= end {
                        hints.push(ParameterInlayHint {
                            offset,
                            label: format!("{name}.{}:", parameter.name),
                        });
                    }
                }
            }
        }
        for child in &node.children {
            visit(snapshot, document, child, start, end, hints);
        }
    }
    let mut hints = Vec::new();
    visit(
        snapshot,
        document,
        &document.tree.root,
        start,
        end,
        &mut hints,
    );
    hints.sort_by_key(|hint| hint.offset);
    hints
}

fn same_nodes(
    left: &[SyntaxNode],
    right: &[SyntaxNode],
    left_source: &str,
    right_source: &str,
) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| same_node(left, right, left_source, right_source))
}
fn same_node(left: &SyntaxNode, right: &SyntaxNode, left_source: &str, right_source: &str) -> bool {
    left.kind == right.kind
        && left.is_unsafe_call == right.is_unsafe_call
        && left.name_override == right.name_override
        && left.documentation == right.documentation
        && (!left.children.is_empty()
            || left_source.get(left.span.start..left.span.end)
                == right_source.get(right.span.start..right.span.end))
        && same_nodes(&left.children, &right.children, left_source, right_source)
        && same_nodes(
            &left.annotation_applications,
            &right.annotation_applications,
            left_source,
            right_source,
        )
        && same_nodes(
            &left.modifier_applications,
            &right.modifier_applications,
            left_source,
            right_source,
        )
}
fn non_layout(kind: crate::tokens::TokenKind) -> bool {
    !matches!(
        kind,
        crate::tokens::TokenKind::Newline
            | crate::tokens::TokenKind::Indent
            | crate::tokens::TokenKind::Dedent
            | crate::tokens::TokenKind::Eof
    )
}
fn same_tokens(
    left: &[crate::tokens::Token],
    right: &[crate::tokens::Token],
    left_source: &str,
    right_source: &str,
) -> bool {
    let mut left = left.iter().filter(|token| non_layout(token.kind));
    let mut right = right.iter().filter(|token| non_layout(token.kind));
    loop {
        match (left.next(), right.next()) {
            (Some(left), Some(right)) => {
                if left.kind != right.kind
                    || left_source.get(left.span.start..left.span.end)
                        != right_source.get(right.span.start..right.span.end)
                {
                    return false;
                }
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}
pub(super) fn preserves_call_structure(document: &ParsedDocument, text: &str) -> bool {
    let source = crate::SourceFile::new(
        document.source.id(),
        document.source.path().to_path_buf(),
        text.to_owned(),
    );
    let lexed = crate::lexer::lex_recovering(&source);
    let parsed = crate::parser::parse(&source, lexed.lexed);
    let original_diagnostics = document.diagnostics.iter();
    let formatted_diagnostics = lexed.diagnostics.iter().chain(&parsed.diagnostics);
    original_diagnostics
        .zip(formatted_diagnostics)
        .all(|(left, right)| {
            left.severity == right.severity
                && left.code == right.code
                && left.message == right.message
                && left.help == right.help
        })
        && document.diagnostics.len() == lexed.diagnostics.len() + parsed.diagnostics.len()
        && same_tokens(
            &document.tree.lexed.tokens,
            &parsed.tree.lexed.tokens,
            document.source.text(),
            text,
        )
        && same_node(
            &document.tree.root,
            &parsed.tree.root,
            document.source.text(),
            text,
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tooling::{SnapshotOptions, SourceInput};

    #[test]
    fn call_context_without_semantics() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/call-context.trn";
        let text =
            "namespace call-context\n\nfunction main;\n    outer;\n        inner; 'a', 'b'\n";
        let snapshot = engine
            .open_snapshot(
                vec![SourceInput {
                    uri: uri.to_owned(),
                    text: text.to_owned(),
                }],
                None,
                None,
                SnapshotOptions::default(),
            )
            .expect("open syntax-only snapshot");
        let offset = text.find("'b'").expect("second inner argument");
        let context = engine
            .call_argument_context(&snapshot.snapshot_id, uri, offset)
            .expect("query call context")
            .expect("inner call context");
        assert_eq!(context.callee_name, "inner");
        assert_eq!(context.active_argument, 1);
        assert_eq!(context.positional_index, 1);
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one semantic fixture exercises argument ownership across signatures, hints, constructors and variadics"
    )]
    fn semantic_call_owners_bind_signatures_and_hints() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/call-tooling.trn";
        let text = concat!(
            "namespace call-tooling\n\n",
            "function opt string; first string, second string = 'default'\n",
            "    return first\n\n",
            "function wrap string; first string, second string = 'outer'\n",
            "    return first\n\n",
            "function combine string; first string, second string = 'default'\n",
            "    return first\n\n",
            "function gather string; first string, rest string...\n",
            "    return first\n\n",
            "class widget\n",
            "    first string\n",
            "    second string\n",
            "    function construct; first string, second string = 'default'\n",
            "        this.first = first\n",
            "        this.second = second\n\n",
            "function main;\n",
            "    s1 = wrap;\n",
            "        opt; 'a',\n",
            "        'b'\n",
            "    s4 = wrap;\n",
            "        opt; 'c', 'd'\n",
            "    mixed = wrap;\n",
            "        opt; 'e',\n",
            "            'f'\n",
            "    explicit = wrap; (opt; 'g',\n",
            "        'h')\n",
            "    reordered = wrap; second = 'i', first = 'j'\n",
            "    defaulted = opt; 'k'\n",
            "    spread = gather; 'l', 'm', 'n'\n",
            "    created = instance widget; 'o', 'p'\n",
        );
        let snapshot = engine
            .open_snapshot(
                vec![SourceInput {
                    uri: uri.to_owned(),
                    text: text.to_owned(),
                }],
                None,
                None,
                SnapshotOptions {
                    semantic: true,
                    ..SnapshotOptions::default()
                },
            )
            .expect("open semantic call-tooling source");

        let context_after_comma = |needle: &str| {
            let comma = text.find(needle).expect("locate comma");
            engine
                .call_argument_context(
                    &snapshot.snapshot_id,
                    uri,
                    comma + needle.find(',').unwrap() + 1,
                )
                .expect("query call context")
                .expect("comma belongs to a call")
        };
        let s1 = context_after_comma("'a',");
        assert_eq!(s1.callee_name, "wrap");
        assert_eq!(s1.active_argument, 1);
        let s4 = context_after_comma("'c',");
        assert_eq!(s4.callee_name, "opt");
        assert_eq!(s4.active_argument, 1);
        let mixed = context_after_comma("'e',");
        assert_eq!(mixed.callee_name, "opt");
        assert_eq!(mixed.active_argument, 1);
        let explicit = context_after_comma("'g',");
        assert_eq!(explicit.callee_name, "opt");
        assert_eq!(explicit.active_argument, 1);
        let constructor = context_after_comma("'o',");
        assert!(constructor.construction);
        assert_eq!(constructor.callee_name, "widget");
        assert_eq!(constructor.active_argument, 1);
        let constructor_signature = engine
            .signature_help(
                &snapshot.snapshot_id,
                uri,
                text.find("'o',").unwrap() + "'o',".len(),
            )
            .unwrap()
            .unwrap();
        assert!(
            constructor_signature
                .label
                .starts_with("widget::construct(")
        );
        assert_eq!(constructor_signature.active_parameter, Some(1));

        let variadic = context_after_comma("'m',");
        assert_eq!(variadic.callee_name, "gather");
        assert_eq!(variadic.active_argument, 2);
        let variadic_signature = engine
            .signature_help(&snapshot.snapshot_id, uri, text.find("'n'").unwrap() + 1)
            .unwrap()
            .unwrap();
        assert_eq!(variadic_signature.active_parameter, Some(1));

        for (needle, expected_name, expected_active) in [
            ("'a',", "wrap", 1),
            ("'c',", "opt", 1),
            ("'e',", "opt", 1),
            ("'g',", "opt", 1),
        ] {
            let offset = text.find(needle).unwrap() + needle.len();
            let signature = engine
                .signature_help(&snapshot.snapshot_id, uri, offset)
                .unwrap()
                .unwrap();
            assert!(signature.label.starts_with(&format!("{expected_name}(")));
            assert_eq!(signature.active_parameter, Some(expected_active));
            assert!(
                signature
                    .parameters
                    .iter()
                    .any(|parameter| parameter.starts_with("second"))
            );
        }

        let named_offset = text.find("first = 'j'").unwrap() + "first = ".len();
        let named_context = engine
            .call_argument_context(&snapshot.snapshot_id, uri, named_offset)
            .unwrap()
            .unwrap();
        assert_eq!(named_context.argument_name.as_deref(), Some("first"));
        let named_signature = engine
            .signature_help(&snapshot.snapshot_id, uri, named_offset)
            .unwrap()
            .unwrap();
        assert_eq!(named_signature.active_parameter, Some(0));
        assert!(named_signature.label.starts_with("wrap("));

        let hints = engine
            .parameter_inlay_hints(&snapshot.snapshot_id, uri, 0, text.len())
            .unwrap();
        assert!(hints.iter().any(
            |hint| hint.label == "wrap.first:" && hint.offset == text.find("opt; 'a'").unwrap()
        ));
        assert!(
            hints
                .iter()
                .any(|hint| hint.label == "opt.first:" && hint.offset == text.find("'a'").unwrap())
        );
        assert!(hints.iter().any(|hint| hint.label == "wrap.second:"
            && hint.offset == text.find("second = 'i'").unwrap()));
        assert!(
            hints
                .iter()
                .any(|hint| hint.label == "opt.first:" && hint.offset == text.find("'k'").unwrap())
        );
        assert!(
            hints
                .iter()
                .any(|hint| hint.label == "widget.first:"
                    && hint.offset == text.find("'o'").unwrap())
        );
        assert!(
            hints.iter().any(
                |hint| hint.label == "gather.rest:" && hint.offset == text.find("'n'").unwrap()
            )
        );
        let syntax_only = engine
            .open_snapshot(
                vec![SourceInput {
                    uri: uri.to_owned(),
                    text: text.to_owned(),
                }],
                None,
                None,
                SnapshotOptions::default(),
            )
            .expect("open source-only constructor");
        let constructor_fallback = engine
            .signature_help(&syntax_only.snapshot_id, uri, text.find("'p'").unwrap() + 1)
            .unwrap()
            .unwrap();
        assert!(constructor_fallback.label.starts_with("widget::construct("));
        assert_eq!(
            constructor_fallback.parameters,
            vec!["first: string".to_owned(), "second?: string".to_owned()]
        );
        assert_eq!(constructor_fallback.active_parameter, Some(1));
    }

    #[test]
    fn incomplete_eof_comma_keeps_source_signature() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/incomplete-call.trn";
        let text = concat!(
            "namespace incomplete-call\n\n",
            "function combine string; first string, second string = 'default'\n",
            "    return first\n\n",
            "function main;\n",
            "    result = combine; 'a',   ",
        );
        let snapshot = engine
            .open_snapshot(
                vec![SourceInput {
                    uri: uri.to_owned(),
                    text: text.to_owned(),
                }],
                None,
                None,
                SnapshotOptions::default(),
            )
            .expect("open incomplete call source");
        let context = engine
            .call_argument_context(&snapshot.snapshot_id, uri, text.len())
            .expect("query EOF call context")
            .expect("trailing comma remains owned");
        assert_eq!(context.callee_name, "combine");
        assert_eq!(context.active_argument, 1);
        assert_eq!(context.argument_name, None);
        let signature = engine
            .signature_help(&snapshot.snapshot_id, uri, text.len())
            .expect("query EOF signature")
            .expect("parsed source declaration supplies signature");
        assert!(signature.label.starts_with("combine("));
        assert_eq!(signature.active_parameter, Some(1));
    }

    #[test]
    fn source_fallback_respects_scope_and_receiver() {
        for (body, available) in [
            ("function main;\n    result = item.combine; 'a',", false),
            (
                "function main;\n    combine = 1\n    result = combine; 'a',",
                false,
            ),
            (
                "function other;\n    combine = 1\nfunction main;\n    result = combine; 'a',",
                true,
            ),
        ] {
            let text = format!(
                "namespace fallback-scope\nfunction combine string; first string, second string = ''\n    return first\n{body}"
            );
            let mut engine = ToolingEngine::default();
            let uri = "file:///workspace/fallback-scope.trn";
            let snapshot = engine
                .open_snapshot(
                    vec![SourceInput {
                        uri: uri.to_owned(),
                        text: text.clone(),
                    }],
                    None,
                    None,
                    SnapshotOptions::default(),
                )
                .unwrap();
            let context = engine
                .call_argument_context(&snapshot.snapshot_id, uri, text.len())
                .unwrap()
                .unwrap();
            assert_eq!(context.callee_name, "combine");
            let signature = engine
                .signature_help(&snapshot.snapshot_id, uri, text.len())
                .unwrap();
            assert_eq!(signature.is_some(), available, "{body}: {signature:?}");
        }
    }
}
