use crate::syntax::{SyntaxKind, SyntaxNode};
use crate::tokens::{LexedSource, TriviaKind};
use crate::{Diagnostic, SourceFile, Span};

pub(super) fn attach(
    source: &SourceFile,
    lexed: &LexedSource,
    root: &mut SyntaxNode,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let comments = lexed
        .trivia
        .iter()
        .filter(|trivia| {
            matches!(
                trivia.kind,
                TriviaKind::DocumentationLine | TriviaKind::DocumentationBlock
            )
        })
        .collect::<Vec<_>>();
    let mut index = 0;
    while index < comments.len() {
        let first = comments[index];
        let start = first.span.start;
        let mut end = first.span.end;
        let block = first.kind == TriviaKind::DocumentationBlock;
        index += 1;
        if block {
            while !source.text()[start..end].ends_with("*/")
                && index < comments.len()
                && comments[index].kind == TriviaKind::DocumentationBlock
            {
                end = comments[index].span.end;
                index += 1;
            }
        } else {
            while index < comments.len()
                && comments[index].kind == TriviaKind::DocumentationLine
                && contiguous(&source.text()[end..comments[index].span.start])
            {
                end = comments[index].span.end;
                index += 1;
            }
        }
        let raw = &source.text()[start..end];
        let text = normalize(raw, block);
        let line_start = source.text()[..start]
            .rfind('\n')
            .map_or(0, |offset| offset + 1);
        let at_line_start = source.text()[line_start..start].trim().is_empty();
        if !at_line_start || !attach_to(root, source.text(), end, &text) {
            diagnostics.push(Diagnostic::error(
                "S1101",
                "documentation block is not attached to a declaration",
                Span::new(source.id(), start, end),
            ));
        }
    }
}

fn contiguous(gap: &str) -> bool {
    gap.chars().all(char::is_whitespace) && gap.bytes().filter(|byte| *byte == b'\n').count() <= 1
}

fn attach_to(node: &mut SyntaxNode, source: &str, end: usize, documentation: &str) -> bool {
    let eligible = matches!(
        node.kind,
        SyntaxKind::ClassDeclaration
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::Binding
            | SyntaxKind::Parameter
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration
            | SyntaxKind::EnumDeclaration
            | SyntaxKind::EnumVariant
    );
    if eligible {
        let mut applications = node
            .annotations()
            .map(|annotation| annotation.span)
            .collect::<Vec<_>>();
        applications.sort_by_key(|span| span.start);
        let anchor = applications
            .first()
            .map_or(node.span.start, |span| span.start);
        if end <= anchor && contiguous(&source[end..anchor]) {
            let mut previous = end;
            let valid = applications.iter().all(|span| {
                let attached = contiguous(&source[previous..span.start]);
                previous = span.end;
                attached
            }) && contiguous(&source[previous..node.span.start]);
            if valid && node.documentation.is_none() {
                node.documentation = Some(documentation.to_owned());
                return true;
            }
        }
    }
    node.children
        .iter_mut()
        .any(|child| attach_to(child, source, end, documentation))
}

fn normalize(raw: &str, block: bool) -> String {
    let body = if block {
        raw.get(3..raw.len().saturating_sub(2)).unwrap_or("")
    } else {
        raw
    };
    let inline_first = block
        && body
            .lines()
            .next()
            .is_some_and(|line| !line.trim().is_empty());
    let mut lines = body
        .lines()
        .map(|line| {
            if block {
                line.to_owned()
            } else {
                let line = line.trim_start().strip_prefix("///").unwrap_or(line);
                line.strip_prefix(' ').unwrap_or(line).to_owned()
            }
        })
        .collect::<Vec<_>>();
    while lines.first().is_some_and(|line| line.trim().is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }
    if block {
        let indentation = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start_matches([' ', '\t']).len())
            .min()
            .unwrap_or(0);
        for line in &mut lines {
            let prefix = indentation.min(line.len() - line.trim_start_matches([' ', '\t']).len());
            *line = line[prefix..].to_owned();
        }
        let mut decoration_lines = lines
            .iter()
            .skip(usize::from(inline_first))
            .filter(|line| !line.trim().is_empty())
            .peekable();
        let decorated =
            decoration_lines.peek().is_some() && decoration_lines.all(|line| line.starts_with('*'));
        if decorated {
            for line in lines.iter_mut().skip(usize::from(inline_first)) {
                if let Some(undecorated) = line.strip_prefix('*') {
                    *line = undecorated
                        .strip_prefix(' ')
                        .unwrap_or(undecorated)
                        .to_owned();
                }
            }
        }
    }
    while lines.first().is_some_and(|line| line.trim().is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_text(text: &str) -> super::super::ParseOutput {
        let source = SourceFile::new(1, "documentation.trn".into(), text.to_owned());
        super::super::parse(&source, crate::lexer::lex(&source).unwrap())
    }

    #[test]
    fn docs_cross_annotations_and_parameters_keep_their_own_applications() {
        let parsed = parse_text(
            "/// Command.\n@[command; name = 'report']\nfunction report int; (\n    @[option; 'limit', help = 'Maximum'] limit int = 10,\n)\n    return limit\n",
        );
        assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
        let function = &parsed.tree.root.children[0];
        assert_eq!(function.documentation(), Some("Command."));
        assert_eq!(function.annotations().count(), 1);
        let parameters = function
            .children
            .iter()
            .find(|node| node.kind == SyntaxKind::ParameterList)
            .unwrap();
        assert_eq!(parameters.children[0].annotations().count(), 1);
        assert_eq!(parameters.children[0].documentation(), None);
    }

    #[test]
    fn decorative_comment_delimiters_remain_ordinary_comments() {
        for banner in [
            "//// separator",
            "/***** banner *****/",
            "/**/",
            "/***\n * banner\n */",
        ] {
            let parsed = parse_text(&format!("{banner}\n\nfunction main;\n    return\n"));
            assert!(
                parsed.diagnostics.is_empty(),
                "{banner}: {:?}",
                parsed.diagnostics
            );
            assert_eq!(parsed.tree.root.children[0].documentation(), None);
        }
    }

    #[test]
    fn blank_lines_and_ordinary_comments_break_documentation_attachment() {
        for separator in [
            "\n",
            "# implementation\n",
            "// implementation\n",
            "/* implementation */\n",
        ] {
            let parsed = parse_text(&format!(
                "/// Not attached.\n{separator}function main;\n    return\n"
            ));
            assert!(
                parsed
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code.as_ref() == "S1101")
            );
            assert_eq!(parsed.tree.root.children[0].documentation(), None);
        }
    }

    #[test]
    fn malformed_annotation_arguments_and_statement_targets_diagnose() {
        for source in [
            "@[tag]\nfunction main;\n    return\n",
            "@[tag; name = 'x', 1]\nfunction main;\n    return\n",
            "function main;\n    @[tag;]\n    return\n",
        ] {
            let parsed = parse_text(source);
            assert!(
                parsed
                    .diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.code.as_ref() == "S1100"),
                "{source}"
            );
        }
    }

    #[test]
    fn decorated_and_plain_blocks_preserve_paragraphs_and_meaningful_indentation() {
        let decorated = "/**\n * Summary.\n *\n *     example\n */";
        let plain = "/**\n Summary.\n\n     example\n */";
        assert_eq!(normalize(decorated, true), "Summary.\n\n    example");
        assert_eq!(normalize(plain, true), "Summary.\n\n    example");
        assert_eq!(
            normalize("/// Summary.\n///\n///     example", false),
            "Summary.\n\n    example"
        );
        assert_eq!(normalize("/**/", true), "");
        assert_eq!(normalize("/** *emphasis**/", true), "*emphasis*");
        assert_eq!(
            normalize("/**\n *emphasis*\n ordinary text\n */", true),
            "*emphasis*\nordinary text"
        );
        assert_eq!(
            normalize("/** Summary.\n *\n *     example\n */", true),
            "Summary.\n\n    example"
        );
    }
}
