use super::{Parser, SyntaxKind, SyntaxNode, TokenKind};

#[derive(Clone, Copy, Default)]
pub(super) struct ArgumentLayout {
    wrapping: usize,
    ended: bool,
    column_calls: bool,
    inline_column: Option<usize>,
    grouping_depth: usize,
}

impl Parser<'_> {
    pub(super) fn argument_ended(&self) -> bool {
        self.layout_argument.is_some_and(|layout| layout.ended)
    }

    pub(super) fn inline_call_column(&self) -> Option<usize> {
        if !self.layout_argument.is_some_and(|layout| {
            layout.column_calls && self.argument_grouping == layout.grouping_depth
        }) {
            return None;
        }
        self.position
            .checked_sub(1)
            .and_then(|index| self.tokens.get(index))
            .map(|token| self.source_indentation(token.span.start))
    }

    pub(super) fn set_inline_argument_column(&mut self, column: Option<usize>) -> Option<usize> {
        self.layout_argument
            .as_mut()
            .and_then(|layout| std::mem::replace(&mut layout.inline_column, column))
    }

    pub(super) fn separator_ends_inline_call(&self) -> bool {
        self.layout_argument.is_some_and(|layout| {
            self.argument_grouping == layout.grouping_depth
                && layout
                    .inline_column
                    .is_some_and(|column| self.column_boundary_from(self.position + 1, column))
        })
    }

    fn inline_argument_boundary(&self) -> bool {
        self.layout_argument.is_some_and(|layout| {
            self.argument_grouping == layout.grouping_depth
                && layout
                    .inline_column
                    .is_some_and(|column| self.column_boundary_from(self.position, column))
        })
    }

    pub(super) fn diagnose_inline_operand_boundary(&mut self) -> bool {
        if !self.inline_argument_boundary() {
            return false;
        }
        self.error_here_with_help(
            "S1122",
            "operator continuation crosses the inline call's argument boundary",
            "The next line is not deeper than the inline call; indent the continuation or parenthesize the operand.",
        );
        true
    }

    pub(super) fn reject_trailing_argument_comma(&mut self, comma: usize) {
        // Inline column sealing can leave the newline before the dedent unconsumed.
        let pending_dedent = self.layout_argument.is_some()
            && self.tokens[comma + 1..]
                .iter()
                .find(|token| token.kind != TokenKind::Newline)
                .is_some_and(|token| token.kind == TokenKind::Dedent);
        let dedent = self.at(TokenKind::Dedent)
            || pending_dedent
            || self.inline_argument_boundary()
                && self
                    .layout_argument
                    .and_then(|layout| layout.inline_column)
                    .is_some_and(|column| {
                        self.source_indentation(self.tokens[comma].span.start) > column
                    });
        self.error_at(comma, "S1122", "expected an argument after `,`");
        if dedent && let Some(diagnostic) = self.diagnostics.last_mut() {
            diagnostic.help = Some(
                "The dedent already ends this call's arguments; remove this comma.".to_owned(),
            );
        }
    }

    fn source_indentation(&self, offset: usize) -> usize {
        let text = self.source.text();
        let start = text[..offset].rfind('\n').map_or(0, |index| index + 1);
        text.as_bytes()[start..]
            .iter()
            .take_while(|byte| matches!(byte, b' ' | b'\t'))
            .count()
    }

    fn column_boundary_from(&self, from: usize, column: usize) -> bool {
        let Some(previous) = from.checked_sub(1).and_then(|index| self.tokens.get(index)) else {
            return false;
        };
        let mut wrapping = self.layout_argument.map_or(0, |layout| layout.wrapping);
        for token in &self.tokens[from..] {
            match token.kind {
                TokenKind::Newline => {}
                TokenKind::Indent => wrapping += 1,
                TokenKind::Dedent if wrapping > 0 => wrapping -= 1,
                TokenKind::Dedent
                | TokenKind::Eof
                | TokenKind::CloseParen
                | TokenKind::CloseBracket
                | TokenKind::CloseBrace => return false,
                _ => {
                    return self.source_indentation(token.span.start) <= column
                        && self.source.text()[previous.span.end..token.span.start].contains('\n');
                }
            }
        }
        false
    }

    pub(super) fn parse_argument_name(&mut self) -> Option<SyntaxNode> {
        if !self.at(TokenKind::Identifier)
            || !(self.peek_kind(1) == Some(TokenKind::Assign)
                || self.layout_argument.is_some() && self.peek_kind(1) == Some(TokenKind::Newline))
        {
            return None;
        }
        let start = self.position;
        let layout = self.layout_argument;
        self.bump();
        self.skip_argument_wrapping();
        if self.eat(TokenKind::Assign) {
            Some(self.node(SyntaxKind::Name, start, start + 1, Vec::new()))
        } else {
            self.position = start;
            self.layout_argument = layout;
            None
        }
    }

    pub(super) fn argument_type_membership_ahead(&mut self) -> bool {
        if self.peek_text(1) == Some("a") && self.type_starts_at(2) {
            return true;
        }
        if self.layout_argument.is_none() {
            return false;
        }
        let start = self.position;
        let layout = self.layout_argument;
        self.bump();
        self.skip_argument_wrapping();
        let found = if self.eat_text("a") {
            self.skip_argument_wrapping();
            self.type_starts_at(0)
        } else {
            false
        };
        self.position = start;
        self.layout_argument = layout;
        found
    }

    /// Only argument expressions treat physical wrapping as whitespace. Calls,
    /// chains and callable bodies consume their own structural blocks first.
    pub(super) fn skip_argument_wrapping(&mut self) {
        while let Some(layout) = self.layout_argument {
            if layout.ended {
                return;
            }
            if self.inline_argument_boundary() {
                return;
            }
            if self.at(TokenKind::Newline) {
                let mut next = self.position;
                while self
                    .tokens
                    .get(next)
                    .is_some_and(|token| token.kind == TokenKind::Newline)
                {
                    next += 1;
                }
                let kind = self.tokens.get(next).map(|token| token.kind);
                if matches!(kind, Some(TokenKind::Dedent)) && layout.wrapping == 0
                    || matches!(kind, Some(TokenKind::CloseParen | TokenKind::Eof))
                    || kind == Some(TokenKind::Indent)
                        && self
                            .tokens
                            .get(next + 1)
                            .is_some_and(|token| token.kind == TokenKind::Dot)
                {
                    return;
                }
                self.position = next;
            } else if self.at(TokenKind::Indent) {
                self.layout_argument.as_mut().unwrap().wrapping += 1;
                self.bump();
            } else if self.at(TokenKind::Dedent) && layout.wrapping > 0 {
                self.layout_argument.as_mut().unwrap().wrapping -= 1;
                self.bump();
            } else {
                return;
            }
        }
    }

    fn end_argument_block(&mut self, outer: Option<ArgumentLayout>) {
        self.layout_argument = outer.map(|mut layout| {
            layout.ended = true;
            layout
        });
    }

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

    pub(super) fn call_layout_start_is_arguments(&self) -> bool {
        self.call_layout_start()
            .is_some_and(|(_, kind)| kind != TokenKind::Dot)
    }

    pub(super) fn parse_layout_arguments(
        &mut self,
        arguments: &mut Vec<SyntaxNode>,
        preceding_comma: Option<usize>,
    ) -> bool {
        let Some((first, kind)) = self.call_layout_start() else {
            return false;
        };
        if kind == TokenKind::Dot {
            return false;
        }
        self.position = first;
        if !arguments.is_empty() && preceding_comma.is_none() {
            self.error_here("S1122", "expected `,` between layout arguments");
        }
        let outer = self.layout_argument.replace(ArgumentLayout {
            column_calls: true,
            grouping_depth: self.argument_grouping,
            ..ArgumentLayout::default()
        });
        while !matches!(
            self.current().kind,
            TokenKind::Dedent | TokenKind::Eof | TokenKind::CloseParen
        ) {
            let start = self.position;
            let mut parts = Vec::new();
            if let Some(name) = self.parse_argument_name() {
                parts.push(name);
            }
            parts.push(self.parse_expression(0, true));
            arguments.push(self.node(SyntaxKind::Argument, start, self.position, parts));
            let ended = self.argument_ended();
            self.layout_argument.as_mut().unwrap().ended = false;
            self.skip_argument_wrapping();
            if self.at(TokenKind::Comma) {
                let comma = self.position;
                self.bump();
                self.skip_argument_wrapping();
                self.skip_newlines();
                if self.at_expression_end() {
                    self.reject_trailing_argument_comma(comma);
                    break;
                }
            } else {
                self.skip_newlines();
                if !matches!(
                    self.current().kind,
                    TokenKind::Dedent | TokenKind::CloseParen | TokenKind::Eof
                ) && !ended
                {
                    self.error_here("S1122", "expected `,` between layout arguments");
                    if arguments.last().is_some_and(|argument| {
                        argument.has_line_leading_text_comparison(self.source.text())
                    }) && let Some(diagnostic) = self.diagnostics.last_mut()
                    {
                        diagnostic.help = Some(
                            "In a call argument, `>` after a completed operand is a comparison. Add `,` before a new text argument.".to_owned()
                        );
                    }
                    self.recover_line();
                    self.skip_newlines();
                }
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
        self.end_argument_block(outer);
        true
    }

    pub(super) fn parse_call_layout(
        &mut self,
        start: usize,
        mut receiver: SyntaxNode,
    ) -> SyntaxNode {
        let Some((first, TokenKind::Dot)) = self.call_layout_start() else {
            return receiver;
        };
        let outer = self.layout_argument.take();
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
                    self.layout_argument = Some(ArgumentLayout::default());
                    let arguments = self.parse_argument_list();
                    self.layout_argument = None;
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
        self.end_argument_block(outer);
        receiver
    }
}
