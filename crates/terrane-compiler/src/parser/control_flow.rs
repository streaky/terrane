use super::{Diagnostic, Parser, SyntaxKind, SyntaxNode, TokenKind};

impl Parser<'_> {
    pub(super) fn parse_switch(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump();
        let selector = if self.at_line_end() {
            None
        } else {
            Some(self.require_expression("switch selector"))
        };
        self.expect(
            TokenKind::Newline,
            "S1023",
            "expected a newline before switch cases",
        );
        self.skip_newlines();
        let mut children = selector.into_iter().collect::<Vec<_>>();
        let case_start = children.len();
        if self.eat(TokenKind::Indent) {
            self.block_depth += 1;
            self.skip_newlines();
            while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
                if self.at_text("case") {
                    children.push(self.parse_switch_case());
                    self.finish_statement();
                } else {
                    self.error_here("S1110", "a switch body may contain only `case` clauses");
                    self.recover_line();
                }
                self.skip_newlines();
            }
            self.block_depth -= 1;
            self.expect(
                TokenKind::Dedent,
                "S1024",
                "expected the end of the indented switch",
            );
        }
        if children.len() == case_start {
            self.diagnostics.push(Diagnostic::error(
                "S1111",
                "`switch` requires at least one `case` clause",
                self.tokens[start].span,
            ));
        }
        let cases = &children[case_start..];
        if case_start == 0 {
            for case in cases {
                if let Some(header) = case.children.first()
                    && header.kind == SyntaxKind::ArgumentList
                    && header.children.len() != 1
                {
                    self.diagnostics.push(Diagnostic::error(
                        "S1116",
                        "a selectorless switch case requires exactly one condition",
                        header.span,
                    ));
                }
            }
        }
        let defaults = cases
            .iter()
            .filter(|case| {
                case.children
                    .first()
                    .is_some_and(|header| header.kind == SyntaxKind::MatchCatchAll)
            })
            .collect::<Vec<_>>();
        if defaults.len() > 1 {
            self.diagnostics.push(Diagnostic::error(
                "S1112",
                "`switch` permits only one `case else`",
                defaults[1].span,
            ));
        }
        if defaults
            .first()
            .is_some_and(|default| cases.last().is_some_and(|last| last.span != default.span))
        {
            self.diagnostics.push(Diagnostic::error(
                "S1113",
                "`case else` must be the final switch clause",
                defaults[0].span,
            ));
        }
        self.node(SyntaxKind::SwitchStatement, start, self.position, children)
    }

    fn parse_switch_case(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump();
        let header = if self.eat_text("else") {
            self.node(SyntaxKind::MatchCatchAll, start, self.position, Vec::new())
        } else {
            let mut expressions = vec![self.require_expression("switch case expression")];
            while self.eat(TokenKind::Comma) {
                if self.at_line_end() {
                    self.error_here("S1114", "expected an expression after `,`");
                    break;
                }
                expressions.push(self.require_expression("switch case expression"));
            }
            self.node(SyntaxKind::ArgumentList, start, self.position, expressions)
        };
        if !self.at_line_end() {
            self.error_here("S1115", "unexpected content after switch case header");
            self.recover_line();
        }
        let block = self.parse_block();
        self.node(
            SyntaxKind::SwitchCase,
            start,
            self.position,
            vec![header, block],
        )
    }

    pub(super) fn parse_match(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump();
        let scrutinee = self.require_expression("match scrutinee");
        self.expect(
            TokenKind::Newline,
            "S1023",
            "expected a newline before match cases",
        );
        self.skip_newlines();
        let mut children = vec![scrutinee];
        if self.eat(TokenKind::Indent) {
            self.block_depth += 1;
            self.skip_newlines();
            while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
                if self.at_text("case") {
                    children.push(self.parse_match_case());
                    self.finish_statement();
                } else {
                    self.error_here("S1090", "a match body may contain only `case` clauses");
                    self.recover_line();
                }
                self.skip_newlines();
            }
            self.block_depth -= 1;
            self.expect(
                TokenKind::Dedent,
                "S1024",
                "expected the end of the indented match",
            );
        }
        self.node(SyntaxKind::MatchStatement, start, self.position, children)
    }

    fn parse_match_case(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump();
        let catch_all = self.eat_text("else");
        let selector = if catch_all {
            self.node(SyntaxKind::MatchCatchAll, start, self.position, Vec::new())
        } else if self.at_text("none") {
            self.leaf(SyntaxKind::Name)
        } else {
            let enum_name = if self.at(TokenKind::Identifier) {
                self.leaf(SyntaxKind::Name)
            } else {
                self.error_here("S1090", "expected a qualified enum variant");
                self.node(SyntaxKind::Error, self.position, self.position, Vec::new())
            };
            if !self.eat(TokenKind::DoubleColon) {
                self.error_here("S1090", "match cases require `Enum::variant`");
            }
            let variant = if self.at(TokenKind::Identifier) {
                self.leaf(SyntaxKind::Name)
            } else {
                self.error_here("S1090", "expected a variant name");
                self.node(SyntaxKind::Error, self.position, self.position, Vec::new())
            };
            self.node(
                SyntaxKind::StaticMemberExpression,
                start,
                self.position,
                vec![enum_name, variant],
            )
        };
        let mut children = vec![selector];
        if self.eat(TokenKind::Semicolon) {
            let bindings_start = self.position;
            let mut bindings = Vec::new();
            loop {
                if self.at(TokenKind::Identifier) || self.text() == "_" {
                    bindings.push(self.leaf(SyntaxKind::Name));
                } else {
                    self.error_here("S1090", "expected a payload binding or `_`");
                    break;
                }
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            children.push(self.node(
                SyntaxKind::ParameterList,
                bindings_start,
                self.position,
                bindings,
            ));
        }
        children.push(self.parse_block());
        self.node(SyntaxKind::MatchCase, start, self.position, children)
    }

    pub(super) fn parse_select(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump();
        if !self.at_line_end() {
            self.error_here("S1100", "`select` does not take an expression");
            self.recover_line();
        }
        self.expect(
            TokenKind::Newline,
            "S1023",
            "expected a newline before select cases",
        );
        self.skip_newlines();
        let mut cases = Vec::new();
        if self.eat(TokenKind::Indent) {
            self.block_depth += 1;
            self.skip_newlines();
            while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
                if self.at_text("case") {
                    cases.push(self.parse_select_case());
                    self.finish_statement();
                } else {
                    self.error_here("S1101", "a select body may contain only `case` clauses");
                    self.recover_line();
                }
                self.skip_newlines();
            }
            self.block_depth -= 1;
            self.expect(
                TokenKind::Dedent,
                "S1024",
                "expected the end of the indented select",
            );
        }
        if cases.len() < 2 {
            self.diagnostics.push(Diagnostic::error(
                "S1102",
                "`select` requires at least two `case` clauses",
                self.tokens[start].span,
            ));
        }
        self.node(SyntaxKind::SelectStatement, start, self.position, cases)
    }

    fn parse_select_case(&mut self) -> SyntaxNode {
        let start = self.position;
        self.bump();
        let unsupported_default = self.at_text("else") || self.at_text("default");
        let header = if unsupported_default {
            self.error_here("S1107", "default select cases are not supported");
            self.recover_line();
            self.node(SyntaxKind::Error, self.position, self.position, Vec::new())
        } else if self.at_line_end() {
            self.error_here(
                "S1103",
                "a select case requires `await expression` or `binding = await expression`",
            );
            self.node(SyntaxKind::Error, self.position, self.position, Vec::new())
        } else if self.at_text("await") {
            self.parse_expression(0, true)
        } else if (self.peek_kind(0) == Some(TokenKind::Identifier)
            && self.peek_kind(1) == Some(TokenKind::Assign))
            || self.looks_like_binding()
        {
            self.parse_binding()
        } else {
            self.parse_expression(0, true)
        };
        let awaited = if header.kind == SyntaxKind::Binding {
            header.children.last()
        } else {
            Some(&header)
        };
        if header.kind == SyntaxKind::Binding
            && header.children.iter().any(|child| {
                matches!(
                    child.kind,
                    SyntaxKind::Visibility | SyntaxKind::DeclarationQualifier
                )
            })
        {
            self.diagnostics.push(Diagnostic::error(
                "S1105",
                "a select case binding must be an ordinary local binding",
                header.span,
            ));
        }
        let top_level_await = awaited.is_some_and(|expression| {
            expression.kind == SyntaxKind::UnaryExpression
                && expression.children.first().is_some_and(|operator| {
                    self.source.text()[operator.span.start..operator.span.end].trim() == "await"
                })
        });
        if !top_level_await && !unsupported_default {
            self.diagnostics.push(Diagnostic::error(
                "S1103",
                "a select case header must contain exactly one top-level `await`",
                header.span,
            ));
        }
        if self.at_text("if") {
            self.error_here("S1106", "select case guards are not supported");
            self.recover_line();
        } else if !self.at_line_end() {
            self.error_here("S1104", "unexpected content after select case header");
            self.recover_line();
        }
        let block = self.parse_block();
        self.node(
            SyntaxKind::SelectCase,
            start,
            self.position,
            vec![header, block],
        )
    }
}
