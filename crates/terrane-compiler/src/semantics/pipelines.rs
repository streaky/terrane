use super::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct PipelinePlan {
    pub source_type: ValueType,
    pub stages: Vec<PipelineStage>,
    pub result_type: ValueType,
}

#[derive(Clone, Debug)]
pub(crate) enum PipelineStage {
    Filter {
        stage_node: usize,
        parameter: ValueType,
        effects: CallableEffects,
    },
    Map {
        stage_node: usize,
        parameter: ValueType,
        result: ElementType,
        effects: CallableEffects,
    },
    Take {
        stage_node: usize,
    },
}

fn diagnostic(
    unit: &SemanticUnit,
    span: Span,
    code: &'static str,
    message: &str,
) -> SemanticFailure {
    failure(&unit.source, code, message, span)
}

fn stage_parts(stage: &SyntaxNode) -> Option<(&SyntaxNode, Option<&SyntaxNode>)> {
    if stage.kind != SyntaxKind::PipelineStage {
        return None;
    }
    let [name, arguments] = stage.children.as_slice() else {
        return None;
    };
    match arguments.children.as_slice() {
        [] => Some((name, None)),
        [argument] => Some((name, Some(argument))),
        _ => None,
    }
}

fn callable(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<(ValueType, ElementType, CallableEffects)>, SemanticFailure> {
    let Some(value_type) = infer_value_type(unit, node, bindings)? else {
        return Ok(None);
    };
    match value_type {
        ValueType::Function(parameters, result, effects) if parameters.len() == 1 => {
            let parameter = &parameters[0];
            if effects.modes.written == InvocationMode::Consuming
                || effects.modes.exact == InvocationMode::Consuming
                || parameter.requires_mutable_reference()
                || parameter.is_variadic()
            {
                return Err(diagnostic(
                    unit,
                    node.span,
                    "S2122",
                    "pipeline callbacks must be repeatable and cannot require mutable-reference or variadic inputs",
                ));
            }
            Ok(Some((parameter.value_type(), result, effects)))
        }
        _ => Err(diagnostic(
            unit,
            node.span,
            "S2122",
            "pipeline filter/map stage requires a synchronous one-argument callable",
        )),
    }
}

fn list_item_type(value_type: &ValueType) -> Option<ElementType> {
    match value_type {
        ValueType::List(item) => Some(item.clone()),
        ValueType::Reference(item)
        | ValueType::MutableReference(item)
        | ValueType::SharedReference(item) => list_item_type(item.value_type_ref()),
        _ => None,
    }
}

pub(crate) fn pipeline_plan(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<PipelinePlan>, SemanticFailure> {
    let Some(source) = node.children.first() else {
        return Err(diagnostic(
            unit,
            node.span,
            "S2121",
            "pipeline requires a list source",
        ));
    };
    let Some(source_type) = infer_value_type(unit, source, bindings)? else {
        return Ok(None);
    };
    let Some(mut item_type) = list_item_type(&source_type) else {
        return Err(diagnostic(
            unit,
            source.span,
            "S2121",
            "pipeline-sequential currently requires a list source",
        ));
    };
    let mut stages = Vec::with_capacity(node.children.len().saturating_sub(2));
    for (index, stage) in node.children.iter().enumerate().skip(1) {
        let Some((operation, argument)) = stage_parts(stage) else {
            return Err(diagnostic(
                unit,
                stage.span,
                "S2121",
                "malformed pipeline stage",
            ));
        };
        let name = node_text(&unit.source, operation);
        if name == "collect" {
            if index != node.children.len() - 1 || argument.is_some() {
                return Err(diagnostic(
                    unit,
                    stage.span,
                    "S2121",
                    "`.collect;` must be the final stage and take no arguments",
                ));
            }
            continue;
        }
        let Some(argument) = argument else {
            return Err(diagnostic(
                unit,
                stage.span,
                "S2121",
                "pipeline stage requires one argument",
            ));
        };
        let Some(stage) = plan_stage(unit, name, argument, index, &mut item_type, bindings)? else {
            return Ok(None);
        };
        stages.push(stage);
    }
    if node
        .children
        .last()
        .and_then(|stage| stage.children.first())
        .is_none_or(|operation| node_text(&unit.source, operation) != "collect")
    {
        return Err(diagnostic(
            unit,
            node.span,
            "S2121",
            "pipeline must end with `.collect;`",
        ));
    }
    Ok(Some(PipelinePlan {
        source_type,
        stages,
        result_type: ValueType::List(item_type),
    }))
}

fn plan_stage(
    unit: &SemanticUnit,
    name: &str,
    argument: &SyntaxNode,
    stage_node: usize,
    item_type: &mut ElementType,
    bindings: &[TypedBinding],
) -> Result<Option<PipelineStage>, SemanticFailure> {
    if name == "take" {
        let Some(count_type) = infer_value_type(unit, argument, bindings)? else {
            return Ok(None);
        };
        if !matches!(count_type, ValueType::Scalar(scalar) if scalar.is_integer()) {
            return Err(diagnostic(
                unit,
                argument.span,
                "S2122",
                "pipeline take count must be an integer",
            ));
        }
        return Ok(Some(PipelineStage::Take { stage_node }));
    }
    if !matches!(name, "filter" | "map") {
        return Err(diagnostic(
            unit,
            argument.span,
            "S2121",
            "unsupported sequential pipeline stage",
        ));
    }
    let Some((parameter, result, effects)) = callable(unit, argument, bindings)? else {
        return Ok(None);
    };
    if &parameter != item_type.value_type_ref()
        && !matches!(&parameter, ValueType::Reference(inner) if inner.value_type_ref() == item_type.value_type_ref())
    {
        return Err(diagnostic(
            unit,
            argument.span,
            "S2122",
            "pipeline callback parameter must match the current element type or borrow it by ref",
        ));
    }
    if name == "filter" {
        if result.value_type_ref() != &ValueType::Scalar(ScalarType::Bool) {
            return Err(diagnostic(
                unit,
                argument.span,
                "S2122",
                "pipeline filter callable must return bool",
            ));
        }
        return Ok(Some(PipelineStage::Filter {
            stage_node,
            parameter,
            effects,
        }));
    }
    if matches!(
        result.value_type_ref(),
        ValueType::Reference(_)
            | ValueType::MutableReference(_)
            | ValueType::InvocationScopedNative { .. }
    ) {
        return Err(diagnostic(
            unit,
            argument.span,
            "S2122",
            "pipeline mapper cannot return a borrowed or invocation-scoped value",
        ));
    }
    *item_type = result.clone();
    Ok(Some(PipelineStage::Map {
        stage_node,
        parameter,
        result,
        effects,
    }))
}

fn require_cloneable(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value_type: &ValueType,
    span: Span,
) -> Result<(), SemanticFailure> {
    if super::objects::value_type_is_cloneable(package, unit, value_type) {
        Ok(())
    } else {
        Err(diagnostic(
            unit,
            span,
            "S2122",
            "pipeline requires cloning this element to preserve the borrowed source",
        ))
    }
}

fn validate_borrowed_callback_loans(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    stages: &[PipelineStage],
) -> Result<(), SemanticFailure> {
    let mut exclusive = BTreeSet::new();
    for stage in stages {
        let (stage_node, effects) = match stage {
            PipelineStage::Filter {
                stage_node,
                effects,
                ..
            }
            | PipelineStage::Map {
                stage_node,
                effects,
                ..
            } => (stage_node, effects),
            PipelineStage::Take { .. } => continue,
        };
        if !effects.borrowed_environment || effects.modes.written != InvocationMode::Mutable {
            continue;
        }
        let argument =
            super::types::ungrouped_expression(&node.children[*stage_node].children[1].children[0]);
        if argument.kind != SyntaxKind::Name {
            continue;
        }
        let identity = unit
            .flow_binding_ids
            .get(&(argument.span.file, argument.span.start, argument.span.end))
            .copied()
            .or_else(|| {
                package
                    .resolve_name_at(unit, argument.span.start, node_text(&unit.source, argument))
                    .and_then(|symbol| symbol.declaration_span)
            });
        if let Some(identity) = identity
            && !exclusive.insert(identity)
        {
            return Err(diagnostic(
                unit,
                argument.span,
                "S2122",
                "a borrowed mutable callable cannot be loaned to more than one pipeline stage",
            ));
        }
    }
    Ok(())
}

fn validate_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::PipelineExpression {
        let plan = pipeline_plan(unit, node, &unit.typed_bindings)?.ok_or_else(|| {
            diagnostic(
                unit,
                node.span,
                "S2122",
                "pipeline stage types could not be resolved",
            )
        })?;
        validate_borrowed_callback_loans(package, unit, node, &plan.stages)?;
        let mut item_type = list_item_type(&plan.source_type)
            .expect("pipeline_plan validated list source")
            .value_type();
        let mut borrowed = true;
        for stage in &plan.stages {
            match stage {
                PipelineStage::Take { .. } => {}
                PipelineStage::Filter {
                    stage_node,
                    parameter,
                    ..
                } => {
                    if !matches!(parameter, ValueType::Reference(_)) {
                        require_cloneable(
                            package,
                            unit,
                            &item_type,
                            node.children[*stage_node].span,
                        )?;
                    }
                }
                PipelineStage::Map {
                    stage_node,
                    parameter,
                    result,
                    ..
                } => {
                    if !matches!(parameter, ValueType::Reference(_)) && borrowed {
                        require_cloneable(
                            package,
                            unit,
                            &item_type,
                            node.children[*stage_node].span,
                        )?;
                    }
                    item_type = result.value_type();
                    borrowed = false;
                }
            }
        }
        if borrowed {
            require_cloneable(package, unit, &item_type, node.span)?;
        }
    }
    for child in &node.children {
        validate_node(package, unit, child)?;
    }
    Ok(())
}

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        validate_node(package, unit, &unit.tree.root)?;
    }
    Ok(())
}
