//! Select the pipeline language only through a resolved modifier operation.
//! Ordinary fluent methods retain their normal identities and semantics.
use super::modifiers::ModifierOperation;
use super::prelude::*;

pub(super) fn normalize(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &mut package.units {
        rewrite(
            &unit.source,
            &mut unit.tree.root,
            &package.declaration_modifiers,
            false,
        )?;
    }
    Ok(())
}

fn rewrite(
    source: &SourceFile,
    node: &mut SyntaxNode,
    modifiers: &BTreeMap<Span, Vec<super::modifiers::ResolvedModifier>>,
    in_function: bool,
) -> Result<(), SemanticFailure> {
    let in_function = in_function
        || matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        );
    if let Some(applications) = modifiers.get(&node.span)
        && applications
            .iter()
            .any(|application| application.operation == ModifierOperation::PipelineSequential)
    {
        if !in_function
            || !matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            || node
                .assignment_target()
                .is_none_or(|target| target.kind != SyntaxKind::Name)
            || applications.len() != 1
        {
            return Err(failure(
                source,
                "S2121",
                "pipeline-sequential requires a local binding and cannot compose with another modifier operation",
                node.span,
            ));
        }
        let value = node
            .children
            .last_mut()
            .expect("binding has an initializer");
        let placeholder = SyntaxNode::new(
            SyntaxKind::Error,
            value.span,
            value.token_range.clone(),
            Vec::new(),
        );
        *value = pipeline(source, std::mem::replace(value, placeholder))?;
    }
    for child in &mut node.children {
        rewrite(source, child, modifiers, in_function)?;
    }
    Ok(())
}

fn pipeline(source: &SourceFile, mut value: SyntaxNode) -> Result<SyntaxNode, SemanticFailure> {
    let span = value.span;
    let tokens = value.token_range.clone();
    let mut stages = Vec::new();
    loop {
        if value.kind == SyntaxKind::GroupExpression && stages.is_empty() {
            if value.children.len() != 1 {
                break;
            }
            value = value.children.remove(0);
            continue;
        }
        if value.kind != SyntaxKind::CallExpression
            || value.children.len() != 2
            || value.children[0].kind != SyntaxKind::MemberExpression
            || value.children[0].children.len() != 2
        {
            break;
        }
        let stage_span = value.span;
        let stage_tokens = value.token_range.clone();
        let arguments = value.children.pop().unwrap();
        let mut callee = value.children.pop().unwrap();
        let mut operation = callee.children.pop().unwrap();
        operation.kind = SyntaxKind::PipelineOperation;
        value = callee.children.pop().unwrap();
        stages.push(SyntaxNode::new(
            SyntaxKind::PipelineStage,
            stage_span,
            stage_tokens,
            vec![operation, arguments],
        ));
    }
    stages.reverse();
    if stages
        .last()
        .is_none_or(|stage| node_text(source, &stage.children[0]) != "collect")
    {
        return Err(failure(
            source,
            "S2121",
            "pipeline must end with `.collect;`",
            span,
        ));
    }
    let last = stages.len() - 1;
    for (index, stage) in stages.iter_mut().enumerate() {
        let name = node_text(source, &stage.children[0]);
        let expected = match name {
            "collect" if index == last => 0,
            "filter" | "map" | "take" => 1,
            "collect" => {
                return Err(failure(
                    source,
                    "S2121",
                    "`.collect;` must be the final pipeline stage",
                    stage.children[0].span,
                ));
            }
            _ => {
                return Err(failure(
                    source,
                    "S2121",
                    "unsupported sequential pipeline stage",
                    stage.children[0].span,
                ));
            }
        };
        let arguments = &mut stage.children[1];
        if arguments.children.len() != expected {
            return Err(failure(
                source,
                "S2121",
                "filter, map and take require one positional argument; collect requires none",
                arguments.span,
            ));
        }
        for argument in &mut arguments.children {
            if argument.kind == SyntaxKind::Argument {
                if argument.children.len() != 1 {
                    return Err(failure(
                        source,
                        "S2121",
                        "pipeline stages require positional arguments",
                        argument.span,
                    ));
                }
                *argument = argument.children.remove(0);
            }
        }
    }
    let mut children = Vec::with_capacity(stages.len() + 1);
    children.push(value);
    children.extend(stages);
    Ok(SyntaxNode::new(
        SyntaxKind::PipelineExpression,
        span,
        tokens,
        children,
    ))
}
