// Source availability, control-flow legality, and return-path validation.
use super::prelude::*;
#[expect(
    clippy::too_many_lines,
    reason = "definite assignment keeps statement-specific branch joins in one ordered traversal"
)]
pub(super) fn validate_assignment_block(
    unit: &SemanticUnit,
    block: &SyntaxNode,
    declared: &mut BTreeSet<String>,
    assigned: &mut BTreeSet<String>,
) -> Result<(), SemanticFailure> {
    for statement in &block.children {
        match statement.kind {
            SyntaxKind::Binding => {
                let name_node = statement
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name);
                let Some(name_node) = name_node else { continue };
                let name = node_text(&unit.source, name_node).to_owned();
                let initializer = super::ownership::binding_initializer(statement);
                if statement.children.iter().any(|child| {
                    child.kind == SyntaxKind::DeclarationQualifier
                        && node_text(&unit.source, child) == "global"
                }) {
                    if let Some(value) = initializer {
                        validate_assigned_reads(unit, value, declared, assigned)?;
                    }
                    continue;
                }
                declared.insert(name.clone());
                if let Some(value) = initializer {
                    validate_assigned_reads(unit, value, declared, assigned)?;
                    assigned.insert(name);
                }
            }
            SyntaxKind::Assignment => {
                if let Some(value) = statement.children.get(1) {
                    validate_assigned_reads(unit, value, declared, assigned)?;
                }
                if let Some(target) = statement
                    .children
                    .first()
                    .filter(|target| target.kind == SyntaxKind::Name)
                {
                    let name = node_text(&unit.source, target).to_owned();
                    declared.insert(name.clone());
                    assigned.insert(name);
                }
            }
            SyntaxKind::TryStatement => {
                let incoming = assigned.clone();
                let mut results = Vec::new();
                if let Some(block) = statement.children.first() {
                    let mut try_declared = declared.clone();
                    let mut try_assigned = incoming.clone();
                    validate_assignment_block(unit, block, &mut try_declared, &mut try_assigned)?;
                    declared.extend(try_declared);
                    results.push(try_assigned);
                }
                for clause in statement
                    .children
                    .iter()
                    .filter(|child| child.kind == SyntaxKind::CatchClause)
                {
                    let Some(block) = clause
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block)
                    else {
                        continue;
                    };
                    let mut catch_declared = declared.clone();
                    let mut catch_assigned = incoming.clone();
                    if let Some(alias) = clause
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::CatchBinding)
                    {
                        let name = node_text(&unit.source, alias).to_owned();
                        catch_declared.insert(name.clone());
                        catch_assigned.insert(name);
                    }
                    validate_assignment_block(
                        unit,
                        block,
                        &mut catch_declared,
                        &mut catch_assigned,
                    )?;
                    declared.extend(catch_declared);
                    results.push(catch_assigned);
                }
                if let Some(first) = results.first() {
                    *assigned = results
                        .iter()
                        .skip(1)
                        .fold(first.clone(), |common, branch| {
                            common.intersection(branch).cloned().collect()
                        });
                }
                if let Some(finally) = statement
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::FinallyClause)
                    .and_then(|clause| clause.children.first())
                {
                    validate_assignment_block(unit, finally, declared, assigned)?;
                }
            }
            SyntaxKind::IfStatement => {
                if let Some(condition) = statement.children.first() {
                    validate_assigned_reads(unit, condition, declared, assigned)?;
                }
                let incoming = assigned.clone();
                let mut branch_results = Vec::new();
                let mut falls_through = true;
                for branch in statement.children.iter().skip(1) {
                    if !falls_through {
                        break;
                    }
                    let condition = if branch.kind == SyntaxKind::Block {
                        statement.children.first()
                    } else if branch.kind == SyntaxKind::ElseClause {
                        branch
                            .children
                            .iter()
                            .find(|child| child.kind != SyntaxKind::Block)
                    } else {
                        branch.children.first()
                    };
                    if let Some(condition) = condition {
                        validate_assigned_reads(unit, condition, declared, &incoming)?;
                        if constant_boolean(unit, condition) == Some(false) {
                            continue;
                        }
                    }
                    let branch_block = if branch.kind == SyntaxKind::Block {
                        Some(branch)
                    } else {
                        branch
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::Block)
                    };
                    if let Some(branch_block) = branch_block {
                        let mut branch_declared = declared.clone();
                        let mut branch_assigned = incoming.clone();
                        validate_assignment_block(
                            unit,
                            branch_block,
                            &mut branch_declared,
                            &mut branch_assigned,
                        )?;
                        declared.extend(branch_declared);
                        branch_results.push(branch_assigned);
                    }
                    if condition.is_none()
                        || constant_boolean(unit, condition.unwrap()) == Some(true)
                    {
                        falls_through = false;
                    }
                }
                if falls_through {
                    branch_results.push(incoming);
                }
                if let Some(first) = branch_results.first() {
                    *assigned = branch_results
                        .iter()
                        .skip(1)
                        .fold(first.clone(), |common, branch| {
                            common.intersection(branch).cloned().collect()
                        });
                }
            }
            SyntaxKind::SelectStatement | SyntaxKind::MatchStatement => {
                let incoming = assigned.clone();
                let mut results = Vec::new();
                let mut exhaustive = statement.kind == SyntaxKind::SelectStatement;
                for case in &statement.children {
                    if !matches!(
                        case.kind,
                        SyntaxKind::SelectCase | SyntaxKind::MatchCase | SyntaxKind::ElseClause
                    ) {
                        continue;
                    }
                    let header = case.children.iter().find(|child| {
                        child.kind != SyntaxKind::Block && child.kind != SyntaxKind::ParameterList
                    });
                    let body = case
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block);
                    let Some(body) = body else { continue };
                    let mut case_declared = declared.clone();
                    let mut case_assigned = incoming.clone();
                    if let Some(header) = header {
                        if case.kind == SyntaxKind::SelectCase && header.kind == SyntaxKind::Binding
                        {
                            if let Some(value) = header.children.iter().rev().find(|child| {
                                child.kind != SyntaxKind::Name
                                    && child.kind != SyntaxKind::TypeExpression
                            }) {
                                validate_assigned_reads(unit, value, declared, &incoming)?;
                            }
                            if let Some(name) = declaration_name(header, &unit.source) {
                                case_declared.insert(name.clone());
                                case_assigned.insert(name);
                            }
                        } else {
                            validate_assigned_reads(unit, header, declared, &incoming)?;
                        }
                    }
                    if case.kind == SyntaxKind::MatchCase {
                        if case
                            .children
                            .first()
                            .is_some_and(|child| child.kind == SyntaxKind::MatchCatchAll)
                        {
                            exhaustive = true;
                        }
                        if let Some(parameters) = case
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::ParameterList)
                        {
                            for parameter in &parameters.children {
                                let name = node_text(&unit.source, parameter).to_owned();
                                if name != "_" {
                                    case_declared.insert(name.clone());
                                    case_assigned.insert(name);
                                }
                            }
                        }
                    }
                    validate_assignment_block(unit, body, &mut case_declared, &mut case_assigned)?;
                    declared.extend(case_declared);
                    results.push(case_assigned);
                }
                if !exhaustive {
                    results.push(incoming);
                }
                if let Some(first) = results.first() {
                    *assigned = results
                        .iter()
                        .skip(1)
                        .fold(first.clone(), |common, branch| {
                            common.intersection(branch).cloned().collect()
                        });
                }
            }
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement => {
                if statement.kind == SyntaxKind::ForStatement
                    && let Some(initializer) = statement.children.first()
                    && matches!(
                        initializer.kind,
                        SyntaxKind::Binding | SyntaxKind::Assignment
                    )
                    && let Some(value) = super::ownership::binding_initializer(initializer)
                {
                    validate_assigned_reads(unit, value, declared, assigned)?;
                    if let Some(name) = initializer
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)
                    {
                        let name = node_text(&unit.source, name).to_owned();
                        declared.insert(name.clone());
                        assigned.insert(name);
                    }
                }
                let incoming = assigned.clone();
                for (index, expression) in statement.children.iter().enumerate() {
                    if matches!(expression.kind, SyntaxKind::Block | SyntaxKind::ForTarget)
                        || (statement.kind == SyntaxKind::ForStatement
                            && index == 0
                            && matches!(
                                expression.kind,
                                SyntaxKind::Binding | SyntaxKind::Assignment
                            ))
                    {
                        continue;
                    }
                    validate_assigned_reads(unit, expression, declared, assigned)?;
                }
                let body = statement
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block);
                if let Some(body) = body {
                    let mut body_declared = declared.clone();
                    let mut body_assigned = incoming.clone();
                    if statement.kind == SyntaxKind::ForStatement
                        && let Some(target) = statement
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::ForTarget)
                    {
                        for name in target
                            .children
                            .iter()
                            .filter(|child| child.kind == SyntaxKind::Name)
                        {
                            let name = node_text(&unit.source, name).to_owned();
                            body_declared.insert(name.clone());
                            body_assigned.insert(name);
                        }
                    }
                    validate_assignment_block(unit, body, &mut body_declared, &mut body_assigned)?;
                    declared.extend(body_declared);
                }
                *assigned = incoming;
            }

            _ => validate_assigned_reads(unit, statement, declared, assigned)?,
        }
    }
    Ok(())
}

pub(super) fn validate_assigned_reads(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declared: &BTreeSet<String>,
    assigned: &BTreeSet<String>,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        return Ok(());
    }
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        if declared.contains(name)
            && !assigned.contains(name)
            && !lexical_scope_chain(unit, node.span.start)
                .find_map(|scope| {
                    scope.symbols.get(name)?.iter().rev().find(|symbol| {
                        symbol
                            .binding_span
                            .is_none_or(|span| span.end <= node.span.start)
                    })
                })
                .is_some_and(|symbol| symbol.kind == SymbolKind::Function || symbol.global)
        {
            let key = (node.span.file, node.span.start, node.span.end);
            if !unit.flow_availability.contains_key(&key) {
                return Err(failure(
                    &unit.source,
                    "T0007",
                    format!("`{name}` may be read before it is assigned"),
                    node.span,
                ));
            }
        }
    }
    for (index, child) in node.children.iter().enumerate() {
        if is_flow_value_child(node, index, child) {
            validate_assigned_reads(unit, child, declared, assigned)?;
        }
    }
    Ok(())
}

pub(super) fn validate_control_flow(
    package: &SemanticPackage,
) -> Result<Vec<Vec<Span>>, SemanticFailure> {
    fn function_declarations<'a>(node: &'a SyntaxNode, declarations: &mut Vec<&'a SyntaxNode>) {
        if node.kind == SyntaxKind::FunctionDeclaration {
            declarations.push(node);
            return;
        }
        for child in &node.children {
            function_declarations(child, declarations);
        }
    }
    let mut unreachable_units = Vec::with_capacity(package.units.len());
    for unit in &package.units {
        let mut unreachable = Vec::new();
        let mut declarations = Vec::new();
        function_declarations(&unit.tree.root, &mut declarations);
        for function in declarations {
            let Some(contract) = unit
                .functions
                .iter()
                .find(|contract| contract.span == function.span)
            else {
                continue;
            };
            let Some(block) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            else {
                continue;
            };
            if block.children.is_empty() {
                continue;
            }
            let bindings = call_site_bindings(unit, Some(contract));
            let falls_through =
                validate_flow_block(unit, block, contract, &bindings, 0, &mut unreachable)?;
            if contract.return_type.clone().is_some() && falls_through {
                return Err(failure(
                    &unit.source,
                    "T0015",
                    format!(
                        "function `{}` may finish without returning a value",
                        contract.name
                    ),
                    function.span,
                ));
            }
        }
        unreachable_units.push(unreachable);
    }
    Ok(unreachable_units)
}

pub(super) fn block_may_fall_through(block: &SyntaxNode) -> bool {
    let Some(statement) = block.children.last() else {
        return true;
    };
    match statement.kind {
        SyntaxKind::ReturnStatement
        | SyntaxKind::ThrowStatement
        | SyntaxKind::BreakStatement
        | SyntaxKind::ContinueStatement => false,
        SyntaxKind::IfStatement => {
            let branches = statement
                .children
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause))
                .collect::<Vec<_>>();
            let has_else = branches
                .iter()
                .any(|branch| branch.kind == SyntaxKind::ElseClause);
            !has_else || branches.iter().any(|branch| block_may_fall_through(branch))
        }
        SyntaxKind::MatchCase => statement.children.last().is_none_or(block_may_fall_through),
        SyntaxKind::Block | SyntaxKind::ElseClause | SyntaxKind::SelectCase => {
            block_may_fall_through(statement)
        }
        SyntaxKind::SelectStatement => statement.children.iter().any(block_may_fall_through),
        SyntaxKind::MatchStatement => statement
            .children
            .iter()
            .filter(|child| matches!(child.kind, SyntaxKind::MatchCase | SyntaxKind::ElseClause))
            .any(block_may_fall_through),
        _ => true,
    }
}

pub(super) fn validate_flow_block(
    unit: &SemanticUnit,
    block: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
    loop_depth: usize,
    unreachable: &mut Vec<Span>,
) -> Result<bool, SemanticFailure> {
    let mut falls_through = true;
    for statement in &block.children {
        if !falls_through {
            unreachable.push(statement.span);
            continue;
        }
        falls_through =
            validate_flow_statement(unit, statement, contract, bindings, loop_depth, unreachable)?;
    }
    Ok(falls_through)
}

#[expect(
    clippy::too_many_lines,
    reason = "flow validation keeps every statement transition in one exhaustive dispatch"
)]
pub(super) fn validate_flow_statement(
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
    loop_depth: usize,
    unreachable: &mut Vec<Span>,
) -> Result<bool, SemanticFailure> {
    match statement.kind {
        SyntaxKind::ReturnStatement => {
            validate_return(unit, statement, contract, bindings)?;
            Ok(false)
        }
        SyntaxKind::ThrowStatement => Ok(false),
        SyntaxKind::BreakStatement | SyntaxKind::ContinueStatement => {
            if loop_depth == 0 {
                let keyword = node_text(&unit.source, statement);
                return Err(failure(
                    &unit.source,
                    "T0014",
                    format!("`{keyword}` is only valid inside a loop"),
                    statement.span,
                ));
            }
            Ok(false)
        }
        SyntaxKind::IfStatement => {
            validate_if_flow(unit, statement, contract, bindings, loop_depth, unreachable)
        }
        SyntaxKind::MatchStatement => {
            let mut any_falls_through = false;
            for case in statement.children.iter().filter(|child| {
                matches!(child.kind, SyntaxKind::MatchCase | SyntaxKind::ElseClause)
            }) {
                if let Some(block) = case.children.last() {
                    any_falls_through |= validate_flow_block(
                        unit,
                        block,
                        contract,
                        bindings,
                        loop_depth,
                        unreachable,
                    )?;
                }
            }
            Ok(any_falls_through)
        }
        SyntaxKind::SelectStatement => {
            let mut any_falls_through = false;
            for case in &statement.children {
                if let Some(block) = case.children.last() {
                    any_falls_through |= validate_flow_block(
                        unit,
                        block,
                        contract,
                        bindings,
                        loop_depth,
                        unreachable,
                    )?;
                }
            }
            Ok(any_falls_through)
        }

        SyntaxKind::TryStatement => {
            let try_falls_through = if let Some(block) = statement.children.first() {
                validate_flow_block(unit, block, contract, bindings, loop_depth, unreachable)?
            } else {
                true
            };
            let mut catch_falls_through = false;
            for clause in statement
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
            {
                if let Some(block) = clause
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
                {
                    catch_falls_through |= validate_flow_block(
                        unit,
                        block,
                        contract,
                        bindings,
                        loop_depth,
                        unreachable,
                    )?;
                }
            }
            if let Some(finally) = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::FinallyClause)
                .and_then(|clause| clause.children.first())
                && !validate_flow_block(unit, finally, contract, bindings, loop_depth, unreachable)?
            {
                return Ok(false);
            }
            Ok(try_falls_through || catch_falls_through)
        }
        SyntaxKind::WhileStatement => {
            if let Some(condition) = statement.children.first() {
                validate_bool_condition(unit, condition, bindings)?;
            }
            if let Some(block) = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                validate_flow_block(unit, block, contract, bindings, loop_depth + 1, unreachable)?;
            }
            Ok(true)
        }
        SyntaxKind::ForStatement => {
            let mut loop_bindings = bindings.to_vec();
            if statement.children.len() == 4 {
                validate_bool_condition(unit, &statement.children[1], bindings)?;
            } else if let [target, collection, block] = statement.children.as_slice() {
                let collection_type =
                    infer_value_type(unit, collection, bindings)?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0016",
                            "collection iteration requires an iterable value",
                            collection.span,
                        )
                    })?;
                let item_type = iterable_item_type(unit, collection_type).map_err(
                    |(code, message, span)| {
                        failure(&unit.source, code, message, span.unwrap_or(collection.span))
                    },
                )?;
                loop_bindings.extend(iteration_target_bindings(
                    unit,
                    target,
                    collection.span.end,
                    block.span,
                    item_type,
                )?);
            }
            if let Some(block) = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                validate_flow_block(
                    unit,
                    block,
                    contract,
                    &loop_bindings,
                    loop_depth + 1,
                    unreachable,
                )?;
            }
            Ok(true)
        }
        SyntaxKind::PostfixExpression => {
            let Some(operand) = statement.children.first() else {
                return Ok(true);
            };
            if operand.kind != SyntaxKind::Name
                || !matches!(
                    infer_value_type(unit, operand, bindings)?,
                    Some(ValueType::Scalar(ty)) if ty.is_integer()
                )
            {
                return Err(failure(
                    &unit.source,
                    "T0014",
                    "postfix update requires an assignable integer binding",
                    statement.span,
                ));
            }
            Ok(true)
        }
        _ => Ok(true),
    }
}

pub(super) fn validate_bool_condition(
    unit: &SemanticUnit,
    condition: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    match infer_value_type(unit, condition, bindings)? {
        Some(value_type)
            if descriptor_operation(unit, &value_type, "truth") == Some("value.truth") =>
        {
            return Ok(());
        }
        Some(ValueType::Object(identity)) => {
            let Some(truth) = descriptor_protocol_method(unit, &identity, "truth") else {
                return Err(failure(
                    &unit.source,
                    "T0014",
                    "control-flow object must define a non-static `truth` method",
                    condition.span,
                ));
            };
            if truth.is_async
                || truth.throws
                || truth.written_invocation_mode != InvocationMode::Shared
                || !truth.parameters.is_empty()
                || truth.return_type != Some(ValueType::Scalar(ScalarType::Bool))
            {
                return Err(failure(
                    &unit.source,
                    "T0014",
                    "truth protocol requires a synchronous, non-throwing, non-mutating, parameterless method returning `bool`",
                    truth.span,
                ));
            }
            return Ok(());
        }
        _ => {}
    }
    Err(failure(
        &unit.source,
        "T0014",
        "control-flow condition must have type `bool` or satisfy the truth protocol",
        condition.span,
    ))
}

pub(super) fn validate_if_flow(
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
    loop_depth: usize,
    unreachable: &mut Vec<Span>,
) -> Result<bool, SemanticFailure> {
    let condition = statement.children.first().ok_or_else(|| {
        failure(
            &unit.source,
            "T0014",
            "an `if` statement requires a condition",
            statement.span,
        )
    })?;
    validate_bool_condition(unit, condition, bindings)?;
    let mut branch_falls_through = Vec::new();
    let mut has_else = false;
    for branch in statement.children.iter().skip(1) {
        let block = if branch.kind == SyntaxKind::Block {
            Some(branch)
        } else if branch.kind == SyntaxKind::ElseClause {
            let mut children = branch.children.iter();
            let first = children.next();
            if first.is_some_and(|child| child.kind == SyntaxKind::Block) {
                has_else = true;
                first
            } else {
                if let Some(condition) = first {
                    validate_bool_condition(unit, condition, bindings)?;
                }
                children.find(|child| child.kind == SyntaxKind::Block)
            }
        } else {
            None
        };
        if let Some(block) = block {
            branch_falls_through.push(validate_flow_block(
                unit,
                block,
                contract,
                bindings,
                loop_depth,
                unreachable,
            )?);
        }
    }
    Ok(!has_else || branch_falls_through.into_iter().any(|branch| branch))
}

fn condition_proves_absent_member(
    source: &SourceFile,
    condition: &SyntaxNode,
    target_name: &str,
) -> bool {
    let condition = super::types::ungrouped_expression(condition);
    if condition.kind != SyntaxKind::BinaryExpression {
        return false;
    }
    let [left, right] = condition.children.as_slice() else {
        return false;
    };
    let operator = source.text()[left.span.end..right.span.start].trim();
    let left = super::types::ungrouped_expression(left);
    let right = super::types::ungrouped_expression(right);
    operator == "=="
        && matches!(
            (node_text(source, left), node_text(source, right)),
            (target, "none") | ("none", target) if target == target_name
        )
}

fn writes_exact_target(source: &SourceFile, node: &SyntaxNode, target_name: &str) -> bool {
    if matches!(
        node.kind,
        SyntaxKind::Assignment | SyntaxKind::PostfixExpression
    ) && node
        .children
        .first()
        .is_some_and(|target| node_text(source, target) == target_name)
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| writes_exact_target(source, child, target_name))
}

fn enclosing_block(node: &SyntaxNode, position: usize) -> Option<&SyntaxNode> {
    if !(node.span.start <= position && position <= node.span.end) {
        return None;
    }
    node.children
        .iter()
        .find_map(|child| enclosing_block(child, position))
        .or_else(|| (node.kind == SyntaxKind::Block).then_some(node))
}

fn post_if_initialized_member_type(
    unit: &SemanticUnit,
    returned: &SyntaxNode,
    actual: &ValueType,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let ValueType::Optional(inner) = actual else {
        return Ok(None);
    };
    let returned = super::types::ungrouped_expression(returned);
    if returned.kind != SyntaxKind::StaticMemberExpression {
        return Ok(None);
    }
    let target_name = node_text(&unit.source, returned);
    let Some(block) = enclosing_block(&unit.tree.root, returned.span.start) else {
        return Ok(None);
    };
    let Some(return_index) = block.children.iter().position(|statement| {
        statement.span.file == returned.span.file
            && statement.span.start <= returned.span.start
            && returned.span.end <= statement.span.end
    }) else {
        return Ok(None);
    };
    for (candidate_index, candidate) in block.children[..return_index].iter().enumerate().rev() {
        if candidate.kind != SyntaxKind::IfStatement
            || candidate
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::ElseClause)
        {
            continue;
        }
        let Some(condition) = candidate.children.first() else {
            continue;
        };
        if !condition_proves_absent_member(&unit.source, condition, target_name)
            || block.children[candidate_index + 1..return_index]
                .iter()
                .any(|statement| writes_exact_target(&unit.source, statement, target_name))
        {
            continue;
        }
        let Some(body) = candidate
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        else {
            continue;
        };
        let mut assignments = body.children.iter().filter(|statement| {
            statement.kind == SyntaxKind::Assignment
                && statement
                    .children
                    .first()
                    .is_some_and(|target| node_text(&unit.source, target) == target_name)
        });
        let Some(assignment) = assignments.next() else {
            continue;
        };
        if assignments.next().is_some() {
            continue;
        }
        if body.children.iter().any(|statement| {
            statement.span != assignment.span
                && writes_exact_target(&unit.source, statement, target_name)
        }) {
            continue;
        }
        let Some(value) = assignment.children.get(1) else {
            continue;
        };
        let Some(assigned_type) = infer_value_type(unit, value, bindings)? else {
            continue;
        };
        if super::types::value_types_compatible(&unit.descriptors, inner, &assigned_type) {
            return Ok(Some(inner.as_ref().clone()));
        }
    }
    Ok(None)
}

pub(super) fn validate_return(
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let value = statement.children.first();
    match (contract.return_type.clone(), value) {
        (None, None) => Ok(()),
        (None, Some(value)) => Err(failure(
            &unit.source,
            "T0015",
            format!("function `{}` does not return a value", contract.name),
            value.span,
        )),
        (Some(expected), None) => Err(failure(
            &unit.source,
            "T0015",
            format!(
                "function `{}` must return `{}`",
                contract.name,
                diagnostic_value_type(&unit.descriptors, &expected)
            ),
            statement.span,
        )),
        (Some(expected), Some(value)) => {
            if contextual_collection_constructor_matches(unit, value, &expected, bindings) {
                return validate_collection_constructor_value(
                    unit,
                    value,
                    &expected,
                    &contract.name,
                    bindings,
                );
            }
            let Some(actual) = infer_value_type(unit, value, bindings)? else {
                return Err(failure(
                    &unit.source,
                    "T0015",
                    format!(
                        "function `{}` must return `{}`",
                        contract.name,
                        diagnostic_value_type(&unit.descriptors, &expected)
                    ),
                    value.span,
                ));
            };
            let actual =
                post_if_initialized_member_type(unit, value, &actual, bindings)?.unwrap_or(actual);
            validate_value_destination(
                &unit.source,
                &unit.descriptors,
                &contract.name,
                expected,
                actual,
                value,
                "T0015",
            )
        }
    }
}
