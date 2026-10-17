//! A narrow definite-exit proof, separate from general loop reaching analysis.
//! A direct nonempty core list/tuple executes at least once. In a straight-line
//! body with no early exit, facts established independently of loop-written
//! entry values hold at every normal iteration exit, including the final one.
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
        let [target, collection, body] = node.children.as_slice() else {
            return Ok(false);
        };
        if target.kind != SyntaxKind::ForTarget
            || body.kind != SyntaxKind::Block
            || !straight_line(body)
        {
            return Ok(false);
        }
        let mut literal = collection;
        while literal.kind == SyntaxKind::GroupExpression {
            let Some(grouped) = literal.children.first() else {
                return Ok(false);
            };
            literal = grouped;
        }
        let known_nonempty =
            collection_constructor_identity(self.unit, literal, &self.unit.typed_bindings)
                .is_some_and(|identity| {
                    matches!(
                        identity,
                        "/core/collections::list" | "/core/collections::tuple"
                    )
                })
                && literal
                    .children
                    .get(1)
                    .is_some_and(|arguments| !arguments.children.is_empty());
        if !known_nonempty {
            return Ok(false);
        }

        self.walk(collection, owners)?;
        let mut iteration = owners.clone();
        self.forget_writes(body, &mut iteration);
        for name in &target.children {
            if let Some(binding) = self.binding(name) {
                iteration.remove(&binding);
            }
        }
        // Loop-written inputs are unknown here. Consequently any owner recovered
        // below is invariant at every normal exit, not just on the first iteration.
        self.walk(body, &mut iteration)?;
        *owners = iteration;
        Ok(true)
    }
}

fn straight_line(node: &SyntaxNode) -> bool {
    if is_function_node(node) {
        return true;
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
        .all(|child| child.kind != SyntaxKind::Block && straight_line(child))
}
