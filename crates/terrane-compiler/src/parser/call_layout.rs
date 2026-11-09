use super::{Parser, SyntaxKind, SyntaxNode, TokenKind};

impl Parser<'_> {
    /// Find a structural child block without consuming the statement terminator.
    fn call_layout_start(&self) -> Option<(usize, TokenKind)> {
        let mut cursor = self.position;
        if self.tokens.get(cursor)?.kind != TokenKind::Newline {
            return None;
        }
        while self.tokens.get(cursor)?.kind == TokenKind::Newline {
            cursor += 1;
        }
        if self.tokens.get(cursor)?.kind != TokenKind::Indent {
            return None;
        }
        cursor += 1;
        while self.tokens.get(cursor)?.kind == TokenKind::Newline {
            cursor += 1;
        }
        Some((cursor, self.tokens.get(cursor)?.kind))
    }

    pub(super) fn parse_layout_arguments(&mut self, arguments: &mut Vec<SyntaxNode>) {
        let Some((first, kind)) = self.call_layout_start() else {
            return;
        };
        if kind == TokenKind::Dot {
            return;
        }
        self.position = first;
        while !matches!(
            self.current().kind,
            TokenKind::Dedent | TokenKind::Eof | TokenKind::CloseParen
        ) {
            let start = self.position;
            let mut parts = Vec::new();
            if self.at(TokenKind::Identifier) && self.peek_kind(1) == Some(TokenKind::Assign) {
                parts.push(self.leaf(SyntaxKind::Name));
                self.bump();
            }
            parts.push(self.parse_expression(0, true));
            arguments.push(self.node(SyntaxKind::Argument, start, self.position, parts));
            if self.eat(TokenKind::Comma) {
                self.skip_newlines();
                if self.at(TokenKind::Dedent) || self.at(TokenKind::CloseParen) {
                    self.error_here("S1122", "expected an argument after `,`");
                }
            } else if self.at(TokenKind::Newline) {
                self.skip_newlines();
            } else if self.position > start
                && self.tokens[self.position - 1].kind == TokenKind::Dedent
            {
                // A nested call consumed its argument/chain block's terminator.
            } else if !self.at(TokenKind::Dedent) {
                self.error_here(
                    "S1122",
                    "expected a newline or `,` between layout arguments",
                );
                self.recover_line();
                self.skip_newlines();
            }
            if self.position == start {
                self.bump();
            }
        }
        self.expect(
            TokenKind::Dedent,
            "S1122",
            "expected the end of the argument indentation block",
        );
    }

    pub(super) fn parse_call_layout(
        &mut self,
        start: usize,
        mut receiver: SyntaxNode,
    ) -> SyntaxNode {
        let Some((first, TokenKind::Dot)) = self.call_layout_start() else {
            return receiver;
        };
        self.position = first;
        loop {
            if self.at(TokenKind::Dot)
                && self.peek_kind(1) == Some(TokenKind::Identifier)
                && !matches!(
                    self.current().attachment,
                    super::Attachment::Right | super::Attachment::Both
                )
            {
                self.error_here("S1013", "a chain member name must immediately follow `.`");
            }
            self.expect(
                TokenKind::Dot,
                "S1123",
                "a chain continuation must begin with `.`",
            );
            if self.at(TokenKind::Identifier) {
                let name = self.leaf(SyntaxKind::Name);
                receiver = self.node(
                    SyntaxKind::MemberExpression,
                    start,
                    self.position,
                    vec![receiver, name],
                );
                receiver = self.parse_postfix_operations(start, receiver);
                if self.eat(TokenKind::Semicolon) {
                    let arguments = self.parse_argument_list();
                    receiver = self.node(
                        SyntaxKind::CallExpression,
                        start,
                        self.position,
                        vec![receiver, arguments],
                    );
                }
                receiver = self.parse_call_layout(start, receiver);
            } else {
                self.error_here("S1014", "expected a member name after `.`");
                self.recover_line();
            }
            self.skip_newlines();
            if self.at(TokenKind::Dedent)
                || self.at(TokenKind::Eof)
                || self.at(TokenKind::CloseParen)
            {
                break;
            }
            if !self.at(TokenKind::Dot) {
                self.error_here("S1123", "a sibling chain continuation must begin with `.`");
                self.recover_line();
                self.skip_newlines();
                if !self.at(TokenKind::Dot) {
                    break;
                }
            }
        }
        self.expect(
            TokenKind::Dedent,
            "S1123",
            "expected the end of the chain indentation block",
        );
        receiver
    }
}
