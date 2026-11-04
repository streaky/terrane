use super::{Parser, SyntaxKind, SyntaxNode, TokenKind};

impl Parser<'_> {
    // Updates are syntax sugar for sibling member assignments, not a new scope
    // or an assignment of an aggregate value. Keep all semantic consumers on
    // their ordinary assignment paths.
    pub(super) fn append_statement(children: &mut Vec<SyntaxNode>, statement: SyntaxNode) {
        if statement.kind == SyntaxKind::Block {
            children.extend(statement.children);
        } else {
            children.push(statement);
        }
    }

    pub(super) fn parse_update_block(&mut self, start: usize, target: SyntaxNode) -> SyntaxNode {
        if target.kind != SyntaxKind::Name
            || &self.source.text()[target.span.start..target.span.end] == "_"
        {
            self.error_at(
                start,
                "S1121",
                "structured updates require an existing simple binding target",
            );
        }
        let name = self.source.text()[target.span.start..target.span.end].to_owned();
        let block = self.parse_field_operations(&name);
        // Retain the authored target for lexical resolution and source tooling;
        // each field operation supplies its own implicit receiver use.
        let mut statements = vec![target];
        statements.extend(block.children);
        self.node(SyntaxKind::Block, start, self.position, statements)
    }

    pub(super) fn parse_initialization_block(
        &mut self,
        start: usize,
        construction: SyntaxNode,
    ) -> SyntaxNode {
        let temporary = self.node(SyntaxKind::Name, start, start + 1, Vec::new());
        let block = self.parse_field_operations("instance");
        let binding = SyntaxNode::new(
            SyntaxKind::Binding,
            temporary.span,
            construction.token_range.clone(),
            vec![temporary, construction],
        );
        let mut result = self.node(
            SyntaxKind::Name,
            self.position - 1,
            self.position,
            Vec::new(),
        );
        result.name_override = Some("instance".to_owned());
        self.node(
            SyntaxKind::InitializationExpression,
            start,
            self.position,
            vec![binding, block, result],
        )
    }

    fn parse_field_operations(&mut self, target: &str) -> SyntaxNode {
        let start = self.position;
        self.bump(); // opening brace
        let mut children: Vec<SyntaxNode> = Vec::new();
        let mut indentation = 0usize;
        let mut inline_layout = !self.at(TokenKind::Newline);
        let mut crossed_newline = false;
        loop {
            match self.current().kind {
                TokenKind::Newline => {
                    crossed_newline = true;
                    self.bump();
                    continue;
                }
                TokenKind::Indent => {
                    indentation += 1;
                    self.bump();
                    continue;
                }
                TokenKind::Dedent if indentation > 0 => {
                    indentation -= 1;
                    self.bump();
                    continue;
                }
                TokenKind::CloseBrace | TokenKind::Dedent | TokenKind::Eof => break,
                _ => {}
            }
            let operation_start = self.position;
            if inline_layout && crossed_newline && !children.is_empty() {
                let first_operation = children[0].token_range.start;
                self.error_at(
                    first_operation,
                    "S1121",
                    "multiline field-assignment blocks must start their first field on a new line",
                );
                inline_layout = false;
            }
            if !self.at(TokenKind::Identifier) || self.peek_kind(1) != Some(TokenKind::Assign) {
                let unset = self.peek_text(1) == Some(".") && self.peek_text(2) == Some("unset");
                if unset {
                    self.error_here(
                        "S1121",
                        "`.unset;` is not implemented; structured blocks currently accept field assignments only",
                    );
                } else {
                    self.error_here(
                        "S1121",
                        "structured blocks accept only unqualified field assignments (`field = expression`)",
                    );
                }
                self.recover_field_operation();
                continue;
            }
            let field = self.leaf(SyntaxKind::Name);
            self.bump(); // equals
            let mut receiver = SyntaxNode::new(
                SyntaxKind::Name,
                crate::Span::new(field.span.file, field.span.start, field.span.start),
                operation_start..operation_start,
                Vec::new(),
            );
            receiver.name_override = Some(target.to_owned());
            let member = SyntaxNode::new(
                SyntaxKind::MemberExpression,
                field.span,
                field.token_range.clone(),
                vec![receiver, field],
            );
            let value = self.require_expression("field assignment value");
            self.reject_literal_separator_call(&value);
            children.push(self.node(
                SyntaxKind::Assignment,
                operation_start,
                self.position,
                vec![member, value],
            ));
            if !self.at_line_end() && !self.at(TokenKind::CloseBrace) {
                self.error_here("S1121", "field assignments must be separated by newlines");
                self.recover_field_operation();
            }
        }
        if children.is_empty() {
            self.error_at(
                start,
                "S1121",
                "structured assignment blocks require at least one field assignment; omit an empty block",
            );
        }
        self.expect(
            TokenKind::CloseBrace,
            "S1121",
            "expected `}` after structured assignment block",
        );
        self.node(SyntaxKind::Block, start, self.position, children)
    }

    fn reject_literal_separator_call(&mut self, value: &SyntaxNode) {
        // `name; option = value` can be a valid named-argument call. Diagnose
        // separator-like syntax only when the callee is a literal, which cannot
        // be callable, rather than rejecting legitimate RHS calls.
        if value.kind == SyntaxKind::CallExpression
            && value
                .children
                .first()
                .is_some_and(|callee| callee.kind == SyntaxKind::Literal)
            && let Some(arguments) = value.children.get(1)
            && arguments
                .children
                .iter()
                .any(|argument| argument.children.len() == 2)
        {
            self.error_at(
                arguments.token_range.start - 1,
                "S1121",
                "field assignments must be separated by newlines; semicolon invokes a call",
            );
        }
    }

    fn recover_field_operation(&mut self) {
        let mut braces = 0usize;
        while !self.at(TokenKind::Eof) {
            match self.current().kind {
                TokenKind::OpenBrace => braces += 1,
                TokenKind::CloseBrace | TokenKind::Newline | TokenKind::Dedent if braces == 0 => {
                    break;
                }
                TokenKind::CloseBrace => braces -= 1,
                _ => {}
            }
            self.bump();
        }
    }
}
