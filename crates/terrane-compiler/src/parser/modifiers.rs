use super::{Parser, SyntaxKind, SyntaxNode, TokenKind};

impl Parser<'_> {
    pub(super) fn parse_with(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump();
        let mut applications = Vec::new();
        loop {
            applications.push(self.parse_modifier_application());
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        let mut declaration = self.parse_unannotated_statement();
        if self.at(TokenKind::OpenParen) {
            if !matches!(
                declaration.kind,
                SyntaxKind::Binding | SyntaxKind::Assignment
            ) || declaration
                .children
                .last()
                .is_none_or(|value| value.kind != SyntaxKind::GroupExpression)
            {
                self.error_here("S1120", "a modifier region requires an explicit binding with a parenthesized acquisition");
            }
            let body = self.parse_modifier_region();
            let mut region = self.node(
                SyntaxKind::WithRegion,
                start,
                self.position,
                vec![declaration, body],
            );
            region.modifier_applications = applications;
            region
        } else {
            if !matches!(
                declaration.kind,
                SyntaxKind::Binding
                    | SyntaxKind::Assignment
                    | SyntaxKind::FunctionDeclaration
                    | SyntaxKind::ClassDeclaration
                    | SyntaxKind::InterfaceDeclaration
                    | SyntaxKind::TraitDeclaration
                    | SyntaxKind::EnumDeclaration
            ) {
                self.error_at(
                    declaration.token_range.start,
                    "S1120",
                    "a `with` clause requires a declaration or a bound region",
                );
            }
            declaration.modifier_applications = applications;
            declaration.span.start = self.tokens[start].span.start;
            declaration.token_range.start = start;
            declaration
        }
    }

    fn parse_modifier_application(&mut self) -> SyntaxNode {
        let start = self.position;
        let grouped = self.eat(TokenKind::OpenParen);
        if self.at(TokenKind::Identifier) && !crate::syntax::is_keyword(self.text()) {
            let name = self.leaf(SyntaxKind::Name);
            let arguments = if self.eat(TokenKind::Semicolon) {
                let list_start = self.position;
                let mut values = Vec::new();
                let mut named = false;
                while grouped && !self.at(TokenKind::CloseParen) && !self.at_line_end() {
                    let argument_start = self.position;
                    let mut parts = Vec::new();
                    if self.at(TokenKind::Identifier)
                        && self.peek_kind(1) == Some(TokenKind::Assign)
                    {
                        named = true;
                        parts.push(self.leaf(SyntaxKind::Name));
                        self.bump();
                    } else if named {
                        self.error_here(
                            "S1120",
                            "positional modifier arguments must precede named arguments",
                        );
                    }
                    parts.push(self.parse_expression(0, false));
                    values.push(self.node(
                        SyntaxKind::Argument,
                        argument_start,
                        self.position,
                        parts,
                    ));
                    if !self.eat(TokenKind::Comma) {
                        break;
                    }
                }
                if !grouped {
                    self.error_here("S1120", "modifier calls must be parenthesized");
                }
                self.node(SyntaxKind::ArgumentList, list_start, self.position, values)
            } else {
                self.node(
                    SyntaxKind::ArgumentList,
                    self.position,
                    self.position,
                    Vec::new(),
                )
            };
            if grouped {
                self.expect(
                    TokenKind::CloseParen,
                    "S1120",
                    "expected `)` after modifier arguments",
                );
            }
            self.node(
                SyntaxKind::ModifierApplication,
                start,
                self.position,
                vec![name, arguments],
            )
        } else {
            self.error_here(
                "S1120",
                "expected a modifier name; structural declaration keywords are not modifiers",
            );
            if !self.at_line_end() {
                self.bump();
            }
            self.node(SyntaxKind::Error, start, self.position, Vec::new())
        }
    }

    fn parse_modifier_region(&mut self) -> SyntaxNode {
        self.expect(
            TokenKind::OpenParen,
            "S1120",
            "expected `(` before a modifier region",
        );
        let body = self.parse_block();
        self.expect(
            TokenKind::CloseParen,
            "S1120",
            "expected `)` after modifier region",
        );
        body
    }
}
