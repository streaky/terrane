use super::Parser;
use crate::syntax::{SyntaxKind, SyntaxNode};
use crate::tokens::TokenKind;

impl Parser<'_> {
    pub(super) fn parse_interpolated_string(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump(); // opening quote
        let mut children = Vec::new();
        while !self.at(TokenKind::StringEnd) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::StringText) {
                children.push(self.leaf(SyntaxKind::StringText));
                continue;
            }
            if self.eat(TokenKind::InterpolationOpen) {
                let interpolation_start = self.position - 1;
                let expression = self.parse_expression(0, true);
                let mut interpolation_children = vec![expression];
                if self.at(TokenKind::RenderingSelector)
                    || self.at(TokenKind::Operator) && self.text() == "!"
                {
                    let marker = self.position;
                    self.bump();
                    if self.at(TokenKind::Colon) {
                        let colon = self.position;
                        self.bump();
                        if marker == 0
                            || self.tokens[marker - 1].span.end == self.tokens[marker].span.start
                            || self.tokens[colon].span.end == self.current().span.start
                            || self.tokens[marker].span.end != self.tokens[colon].span.start
                        {
                            self.error_at(
                                marker,
                                "S1131",
                                "rendering selector `!:` requires adjacent characters and whitespace on both sides",
                            );
                        }
                        if self.at(TokenKind::Identifier) {
                            let key = self.leaf(SyntaxKind::RenderingKey);
                            interpolation_children.push(key);
                        } else {
                            self.error_here("S1132", "expected a rendering key after `!:`");
                        }
                    } else {
                        self.error_at(marker, "S1131", "expected `:` after rendering selector `!`");
                    }
                }
                if !self.at(TokenKind::InterpolationClose) {
                    self.error_here("S1130", "malformed string interpolation");
                    while !self.at(TokenKind::InterpolationClose)
                        && !self.at(TokenKind::StringEnd)
                        && !self.at(TokenKind::Eof)
                    {
                        self.bump();
                    }
                }
                self.eat(TokenKind::InterpolationClose);
                let end = self.position;
                let span = crate::Span::new(
                    self.source.id(),
                    self.tokens[interpolation_start].span.start,
                    self.tokens[end - 1].span.end,
                );
                children.push(SyntaxNode::new(
                    SyntaxKind::Interpolation,
                    span,
                    interpolation_start..end,
                    interpolation_children,
                ));
                continue;
            }
            self.error_here("S1130", "unexpected token in interpolated string");
            self.bump();
        }
        self.expect(
            TokenKind::StringEnd,
            "S1130",
            "expected closing quote after interpolated string",
        );
        self.node(
            SyntaxKind::InterpolatedString,
            start,
            self.position,
            children,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{SyntaxKind, SyntaxNode};
    use crate::{SourceFile, lexer};

    fn parse_text(text: &str) -> (SyntaxNode, Vec<crate::Diagnostic>) {
        let source = SourceFile::new(1, "interpolation.trn".into(), text.to_owned());
        let lexical = lexer::lex_recovering(&source);
        let mut diagnostics = lexical.diagnostics;
        let parsed = super::super::parse(&source, lexical.lexed);
        diagnostics.extend(parsed.diagnostics);
        (parsed.tree.root, diagnostics)
    }

    fn find_kind(node: &SyntaxNode, kind: SyntaxKind) -> Option<&SyntaxNode> {
        if node.kind == kind {
            return Some(node);
        }
        node.children
            .iter()
            .find_map(|child| find_kind(child, kind))
    }

    #[test]
    fn keeps_code_tokens_and_text_spans() {
        let (root, diagnostics) =
            parse_text("function main;\n    return \"pre {{ {call; \"inner\"} tail\"\n");
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        let string = find_kind(&root, SyntaxKind::InterpolatedString).unwrap();
        assert!(
            string
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::StringText)
        );
        let interpolation = string
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Interpolation)
            .unwrap();
        assert_eq!(interpolation.children[0].kind, SyntaxKind::CallExpression);
        assert!(interpolation.children[0].span.start > string.span.start);
    }

    #[test]
    fn selector_requires_spaces_around_marker() {
        let (_, diagnostics) = parse_text("function main;\n    return \"{value! :key}\"\n");
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "S1131")
        );
    }

    #[test]
    fn selector_marker_characters_must_be_adjacent() {
        let (_, diagnostics) = parse_text("function main;\n    return \"{value ! : key}\"\n");
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "S1131")
        );
    }
    #[test]
    fn decodes_braces_and_unicode_escapes() {
        assert_eq!(
            lexer::string_literal_value("\"{{name}} \\u{41}\""),
            "{name} A"
        );
        let (root, diagnostics) = parse_text("function main;\n    return \"{{name}}\"\n");
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        assert!(find_kind(&root, SyntaxKind::InterpolatedString).is_none());
        assert!(find_kind(&root, SyntaxKind::Literal).is_some());
    }

    #[test]
    fn unknown_double_escape_is_diagnosed() {
        let source = SourceFile::new(
            1,
            "escape.trn".into(),
            "function main;\n    return \"\\q\"\n".to_owned(),
        );
        let lexical = lexer::lex_recovering(&source);
        assert!(
            lexical
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "L0013")
        );
    }

    #[test]
    fn third_interpolation_level_is_rejected() {
        let (_, diagnostics) = parse_text(
            "function main;\n    return \"outer { \"middle { \"inner {name}\" }\" }\"\n",
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "S1133")
        );
    }
    #[test]
    fn nested_single_quote_keeps_unknown_escape() {
        let (root, diagnostics) = parse_text("function main;\n    return \"{'\\q'}\"\n");
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        assert_eq!(lexer::string_literal_value("'\\q'"), "\\q");
        assert!(find_kind(&root, SyntaxKind::InterpolatedString).is_some());
    }
}
