//! Definite-entry, invariant-exit proofs, separate from general loop dataflow.
//! Supported literal iteration and constant-entry counter loops may nest. Forget
//! loop-written inputs before deriving facts valid at every normal iteration exit.
use super::{Owners, Pass};
use crate::semantics::prelude::{
    SemanticFailure, SyntaxKind, SyntaxNode, collection_constructor_identity, is_function_node,
};

impl Pass<'_> {
    pub(super) fn walk_for_loop(
        &mut self,
        node: &SyntaxNode,
        owners: &mut Owners,
    ) -> Result<bool, SemanticFailure> {
        if node.kind != SyntaxKind::ForStatement {
            return Ok(false);
        }
        let Some(body) = node
            .children
            .last()
            .filter(|body| body.kind == SyntaxKind::Block)
        else {
            return Ok(false);
        };
        if !self.definitely_entered(node) || !self.supported_body(body) {
            return Ok(false);
        }

        for header in &node.children[..node.children.len() - 1] {
            if header.kind != SyntaxKind::ForTarget && header.kind != SyntaxKind::PostfixExpression
            {
                self.walk(header, owners)?;
            }
        }
        let mut iteration = owners.clone();
        self.forget_writes(node, &mut iteration);
        if let Some(target) = node
            .children
            .first()
            .filter(|target| target.kind == SyntaxKind::ForTarget)
        {
            for name in &target.children {
                if let Some(binding) = self.binding(name) {
                    iteration.remove(&binding);
                }
            }
        }
        // Loop-written inputs are unknown here. Consequently any owner recovered
        // below is invariant at every normal exit, not just on the first iteration.
        self.walk(body, &mut iteration)?;
        // Counter updates run after the body. The supported header only permits
        // a postfix counter update; do not accidentally generalize this ordering.
        for update in node
            .children
            .iter()
            .filter(|child| child.kind == SyntaxKind::PostfixExpression)
        {
            self.walk(update, &mut iteration)?;
        }
        *owners = iteration;
        Ok(true)
    }

    fn definitely_entered(&self, node: &SyntaxNode) -> bool {
        if node.kind != SyntaxKind::ForStatement {
            return false;
        }
        if super::counter::definitely_entered(self.unit, node) {
            return node
                .children
                .get(2)
                .is_some_and(|update| update.kind == SyntaxKind::PostfixExpression);
        }
        let [target, collection, body] = node.children.as_slice() else {
            return false;
        };
        if target.kind != SyntaxKind::ForTarget || body.kind != SyntaxKind::Block {
            return false;
        }
        let mut literal = collection;
        while literal.kind == SyntaxKind::GroupExpression {
            let Some(grouped) = literal.children.first() else {
                return false;
            };
            literal = grouped;
        }
        collection_constructor_identity(self.unit, literal, &self.unit.typed_bindings).is_some_and(
            |identity| {
                matches!(
                    identity,
                    "/core/collections::list" | "/core/collections::tuple"
                )
            },
        ) && literal
            .children
            .get(1)
            .is_some_and(|arguments| !arguments.children.is_empty())
    }

    fn supported_body(&self, node: &SyntaxNode) -> bool {
        if is_function_node(node) {
            return true;
        }
        if node.kind == SyntaxKind::ForStatement {
            return self.definitely_entered(node)
                && node
                    .children
                    .last()
                    .is_some_and(|body| self.supported_body(body));
        }
        if matches!(
            node.kind,
            SyntaxKind::ReturnStatement
                | SyntaxKind::ThrowStatement
                | SyntaxKind::BreakStatement
                | SyntaxKind::ContinueStatement
        ) {
            return false;
        }
        node.children
            .iter()
            .all(|child| child.kind != SyntaxKind::Block && self.supported_body(child))
    }
}
