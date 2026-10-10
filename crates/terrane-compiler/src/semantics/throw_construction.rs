use super::{SemanticPackage, SemanticUnit, SymbolKind};
use crate::syntax::{SyntaxKind, SyntaxNode};

/// Resolve class designators before inference, then reuse ordinary construction throughout
/// argument validation, ownership/effect analysis, and lowering. Existing values and factory
/// calls are left untouched, as are compiler-owned errors with their lazy runtime messages.
pub(super) fn normalize(package: &mut SemanticPackage) {
    fn collect(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        sites: &mut Vec<usize>,
    ) {
        if node.kind == SyntaxKind::ThrowStatement
            && let Some(value) = node.children.first()
        {
            let designator = if value.kind == SyntaxKind::CallExpression {
                value.children.first().unwrap_or(value)
            } else {
                value
            };
            let name = if designator.kind == SyntaxKind::AppliedType {
                designator.children.first().unwrap_or(designator)
            } else {
                designator
            };
            if name.kind == SyntaxKind::Name
                && package
                    .resolve_name_at(
                        unit,
                        name.span.start,
                        &unit.source.text()[name.span.start..name.span.end],
                    )
                    .is_some_and(|symbol| symbol.kind == SymbolKind::Class)
            {
                sites.push(node.span.start);
            }
        }
        for child in &node.children {
            collect(package, unit, child, sites);
        }
    }

    fn rewrite(node: &mut SyntaxNode, sites: &mut std::iter::Peekable<std::vec::IntoIter<usize>>) {
        if node.kind == SyntaxKind::ThrowStatement && sites.peek() == Some(&node.span.start) {
            sites.next();
            let value = &mut node.children[0];
            if value.kind != SyntaxKind::CallExpression {
                let arguments = SyntaxNode::new(
                    SyntaxKind::ArgumentList,
                    crate::Span::new(value.span.file, value.span.end, value.span.end),
                    value.token_range.end..value.token_range.end,
                    Vec::new(),
                );
                let designator = std::mem::replace(
                    value,
                    SyntaxNode::new(
                        SyntaxKind::CallExpression,
                        value.span,
                        value.token_range.clone(),
                        Vec::new(),
                    ),
                );
                value.children = vec![designator, arguments];
            }
            let callee = &mut value.children[0];
            let designator = std::mem::replace(
                callee,
                SyntaxNode::new(
                    SyntaxKind::ConstructionExpression,
                    callee.span,
                    callee.token_range.clone(),
                    Vec::new(),
                ),
            );
            callee.children.push(designator);
        }
        for child in &mut node.children {
            rewrite(child, sites);
        }
    }

    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let mut sites = Vec::new();
        collect(package, unit, &unit.tree.root, &mut sites);
        if !sites.is_empty() {
            rewrite(
                &mut package.units[index].tree.root,
                &mut sites.into_iter().peekable(),
            );
        }
    }
}
