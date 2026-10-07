use super::prelude::*;

const MARKER: &str = "/core/annotations::annotation";
const TARGET: &str = "/core/annotations::annotation-target";

#[derive(Default)]
struct EvaluationStack {
    active: BTreeSet<(u32, usize, usize)>,
    reads: BTreeMap<(u32, usize, usize), Span>,
}

fn source_unit(semantic: &SemanticPackage, file: u32) -> Option<&SemanticUnit> {
    semantic.units.iter().find(|unit| unit.source.id() == file)
}

fn metadata_span(unit: &SemanticUnit, span: Span) -> MetadataSpan {
    MetadataSpan {
        file: span.file,
        path: unit.source_path.clone(),
        start: span.start,
        end: span.end,
    }
}

fn find_node(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    if node.span == span {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_node(child, span))
}

fn value_node(node: &SyntaxNode) -> Option<&SyntaxNode> {
    node.children
        .iter()
        .skip_while(|child| child.kind != SyntaxKind::Name)
        .skip(1)
        .find(|child| {
            !matches!(
                child.kind,
                SyntaxKind::TypeExpression
                    | SyntaxKind::Visibility
                    | SyntaxKind::DeclarationQualifier
                    | SyntaxKind::FieldMetadata
                    | SyntaxKind::VariadicMarker
            )
        })
}

fn designator(mut node: &SyntaxNode) -> &SyntaxNode {
    while matches!(
        node.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression
    ) {
        let Some(child) = node.children.first() else {
            break;
        };
        node = child;
    }
    node
}

fn error(unit: &SemanticUnit, span: Span, message: impl Into<String>) -> SemanticFailure {
    failure(&unit.source, "S2110", message, span)
}

fn evaluate(
    semantic: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    stack: &mut EvaluationStack,
) -> Result<CompileTimeValue, SemanticFailure> {
    let text = node_text(&unit.source, node);
    match node.kind {
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression => {
            return evaluate(semantic, unit, &node.children[0], stack);
        }
        SyntaxKind::Literal => {
            if text == "none" {
                return Ok(CompileTimeValue::None);
            }
            if text == "true" || text == "false" {
                return Ok(CompileTimeValue::Boolean(text == "true"));
            }
            if let Some(tail) = text.strip_prefix('>') {
                return Ok(CompileTimeValue::String(
                    tail.strip_prefix('>')
                        .map_or_else(|| tail.to_owned(), lexer::block_string),
                ));
            }
            if let Some(body) = text
                .strip_prefix("b'")
                .and_then(|body| body.strip_suffix('\''))
            {
                return lexer::unescape_bytes(body)
                    .map(CompileTimeValue::Bytes)
                    .map_err(|_| error(unit, node.span, "invalid immutable byte literal"));
            }
            if text.starts_with(['\'', '"']) {
                return Ok(CompileTimeValue::String(lexer::unescape_string(
                    &text[1..text.len() - 1],
                )));
            }
            if let Some(integer) = parse_integer_source_text(&unit.source, node) {
                return Ok(CompileTimeValue::Integer(integer.to_string()));
            }
            let value = text
                .replace('_', "")
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite())
                .ok_or_else(|| {
                    error(
                        unit,
                        node.span,
                        "metadata requires a finite numeric literal",
                    )
                })?;
            return Ok(CompileTimeValue::Float(value.to_string()));
        }
        SyntaxKind::UnaryExpression => {
            if let Some(integer) = parse_integer_source_text(&unit.source, node) {
                return Ok(CompileTimeValue::Integer(integer.to_string()));
            }
            if text.trim_start().starts_with(['-', '+']) {
                let compact = text
                    .chars()
                    .filter(|character| !character.is_whitespace() && *character != '_')
                    .collect::<String>();
                if let Ok(value) = compact.parse::<f64>()
                    && value.is_finite()
                {
                    return Ok(CompileTimeValue::Float(value.to_string()));
                }
            }
        }
        SyntaxKind::Name => {
            let symbol = semantic
                .resolve_name_at(unit, node.span.start, text)
                .ok_or_else(|| {
                    error(unit, node.span, format!("unknown metadata value `{text}`"))
                })?;
            if symbol.descriptor_identity() == Some("/core/types::none") {
                return Ok(CompileTimeValue::None);
            }
            if matches!(
                symbol.kind,
                SymbolKind::TypeDescriptor
                    | SymbolKind::Class
                    | SymbolKind::Enum
                    | SymbolKind::Interface
                    | SymbolKind::Trait
                    | SymbolKind::Function
            ) {
                return Ok(CompileTimeValue::Descriptor(
                    symbol
                        .descriptor_identity()
                        .unwrap_or(&symbol.identity)
                        .to_owned(),
                ));
            }
            let span = symbol
                .declaration_span
                .ok_or_else(|| error(unit, node.span, "constant has no compile-time definition"))?;
            if !symbol.constant && !declaration_is_constant(semantic, span) {
                return Err(error(
                    unit,
                    node.span,
                    "metadata cannot read mutable or non-constant state",
                ));
            }
            let key = (span.file, span.start, span.end);
            if !stack.active.insert(key) {
                return Err(error(
                    unit,
                    node.span,
                    "cyclic compile-time metadata constant",
                ));
            }
            stack.reads.insert(key, node.span);
            let result = (|| {
                let defining_unit = source_unit(semantic, span.file)
                    .ok_or_else(|| error(unit, node.span, "constant source is unavailable"))?;
                let declaration = find_node(&defining_unit.tree.root, span)
                    .ok_or_else(|| error(unit, node.span, "constant declaration is unavailable"))?;
                let value = value_node(declaration).ok_or_else(|| {
                    error(unit, node.span, "constant has no immutable initializer")
                })?;
                evaluate(semantic, defining_unit, value, stack)
            })();
            stack.active.remove(&key);
            return result;
        }
        SyntaxKind::CallExpression => {
            if let [constructor, arguments] = node.children.as_slice()
                && constructor.kind == SyntaxKind::ConstructionExpression
                && arguments.children.is_empty()
                && let Some(member) = constructor
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::StaticMemberExpression)
                && let [owner, variant] = member.children.as_slice()
                && semantic
                    .resolve_name_at(
                        unit,
                        owner.span.start,
                        node_text(&unit.source, designator(owner)),
                    )
                    .is_some_and(|symbol| symbol.identity == TARGET)
            {
                let target = match node_text(&unit.source, variant) {
                    "class" => AnnotationTarget::Class,
                    "callable" => AnnotationTarget::Callable,
                    "parameter" => AnnotationTarget::Parameter,
                    "field" => AnnotationTarget::Field,
                    _ => return Err(error(unit, variant.span, "unknown annotation target")),
                };
                return Ok(CompileTimeValue::Kind(target));
            }
            if let [callee, arguments] = node.children.as_slice()
                && designator(callee).kind == SyntaxKind::Name
                && let Some(symbol) = semantic.resolve_name_at(
                    unit,
                    callee.span.start,
                    node_text(&unit.source, designator(callee)),
                )
                && ((symbol.kind == SymbolKind::TypeDescriptor
                    && matches!(
                        symbol.identity.as_str(),
                        "list" | "tuple" | "set" | "map" | "entry"
                    ))
                    || symbol.namespace == "/core/collections"
                        && matches!(symbol.kind, SymbolKind::TypeDescriptor | SymbolKind::Class))
            {
                let constructor = symbol.identity.rsplit("::").next().unwrap_or("");
                if matches!(constructor, "list" | "tuple" | "set" | "map" | "entry") {
                    let mut values = Vec::with_capacity(arguments.children.len());
                    for argument in &arguments.children {
                        if argument.children.len() != 1 {
                            return Err(error(
                                unit,
                                argument.span,
                                "immutable aggregates use positional values",
                            ));
                        }
                        values.push(evaluate(semantic, unit, &argument.children[0], stack)?);
                    }
                    return match constructor {
                        "list" => Ok(CompileTimeValue::List(values)),
                        "tuple" => Ok(CompileTimeValue::Tuple(values)),
                        "entry" if values.len() == 2 => Ok(CompileTimeValue::Tuple(values)),
                        "set" => {
                            let mut unique = Vec::with_capacity(values.len());
                            for value in values {
                                if !unique.contains(&value) {
                                    unique.push(value);
                                }
                            }
                            Ok(CompileTimeValue::Set(unique))
                        }
                        "map" => {
                            let mut pairs: Vec<(CompileTimeValue, CompileTimeValue)> =
                                Vec::with_capacity(values.len());
                            for value in values {
                                let CompileTimeValue::Tuple(mut pair) = value else {
                                    return Err(error(
                                        unit,
                                        node.span,
                                        "map metadata requires immutable entries",
                                    ));
                                };
                                if pair.len() != 2 {
                                    return Err(error(
                                        unit,
                                        node.span,
                                        "map metadata entries require key and value",
                                    ));
                                }
                                let value = pair.pop().unwrap();
                                let key = pair.pop().unwrap();
                                if let Some(existing) =
                                    pairs.iter_mut().find(|(existing, _)| existing == &key)
                                {
                                    existing.1 = value;
                                } else {
                                    pairs.push((key, value));
                                }
                            }
                            Ok(CompileTimeValue::Map(pairs))
                        }
                        _ => Err(error(unit, node.span, "invalid immutable aggregate shape")),
                    };
                }
            }
        }
        _ => {}
    }
    Err(error(
        unit,
        node.span,
        "metadata admits literals, immutable constants, descriptors and canonical aggregates; calls and effects are not executed",
    ))
}

fn check_type(
    unit: &SemanticUnit,
    span: Span,
    value: &mut CompileTimeValue,
    expected: &ValueType,
) -> Result<(), SemanticFailure> {
    let valid = match (expected, &mut *value) {
        (ValueType::Optional(_), CompileTimeValue::None) => true,
        (ValueType::Optional(inner), value) => {
            check_type(unit, span, value, inner)?;
            true
        }
        (ValueType::Scalar(ScalarType::None), CompileTimeValue::None)
        | (ValueType::Scalar(ScalarType::Bool), CompileTimeValue::Boolean(_))
        | (ValueType::Scalar(ScalarType::String), CompileTimeValue::String(_))
        | (ValueType::Scalar(ScalarType::Bytes), CompileTimeValue::Bytes(_)) => true,
        (ValueType::Scalar(scalar), CompileTimeValue::Integer(text))
            if scalar.source_name().starts_with("int")
                || scalar.source_name().starts_with("uint") =>
        {
            let integer =
                BigInt::parse_bytes(text.as_bytes(), 10).expect("canonical integer metadata");
            check_integer_range(&unit.source, *scalar, &integer, span)?;
            true
        }
        (
            ValueType::Scalar(scalar @ (ScalarType::Float32 | ScalarType::Float64)),
            value @ (CompileTimeValue::Integer(_) | CompileTimeValue::Float(_)),
        ) => {
            let text = match value {
                CompileTimeValue::Integer(text) | CompileTimeValue::Float(text) => text,
                _ => unreachable!(),
            };
            let number = text
                .parse::<f64>()
                .ok()
                .filter(|number| number.is_finite())
                .ok_or_else(|| {
                    error(
                        unit,
                        span,
                        "metadata number exceeds its floating destination",
                    )
                })?;
            let number = if *scalar == ScalarType::Float32 {
                f64::from(number as f32)
            } else {
                number
            };
            if !number.is_finite() {
                return Err(error(
                    unit,
                    span,
                    "metadata number exceeds its floating destination",
                ));
            }
            *value = CompileTimeValue::Float(number.to_string());
            true
        }
        (ValueType::List(element), CompileTimeValue::List(values))
        | (ValueType::Set(element), CompileTimeValue::Set(values)) => {
            for value in values {
                check_type(unit, span, value, element.value_type_ref())?;
            }
            true
        }
        (ValueType::Tuple(element, length), CompileTimeValue::Tuple(values)) => {
            if length.is_some_and(|length| length != values.len()) {
                return Err(error(unit, span, "metadata tuple has the wrong length"));
            }
            for value in values {
                check_type(unit, span, value, element.value_type_ref())?;
            }
            true
        }
        (ValueType::Map(key_type, value_type), CompileTimeValue::Map(entries)) => {
            for (key, value) in entries {
                check_type(unit, span, key, key_type.value_type_ref())?;
                check_type(unit, span, value, value_type.value_type_ref())?;
            }
            true
        }
        (ValueType::Descriptor(expected), CompileTimeValue::Descriptor(identity)) => {
            expected == identity || expected == "descriptor" || expected.is_empty()
        }
        (ValueType::Object(identity), value @ CompileTimeValue::None)
            if identity.qualified() == "/core/annotations::descriptor" =>
        {
            *value = CompileTimeValue::Descriptor("/core/types::none".into());
            true
        }
        (ValueType::Object(identity), CompileTimeValue::Descriptor(_)) => {
            identity.qualified() == "/core/annotations::descriptor"
        }
        (ValueType::Object(identity), CompileTimeValue::Kind(_)) => identity.qualified() == TARGET,
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(error(
            unit,
            span,
            format!("annotation value does not match declared type `{expected}`"),
        ))
    }
}

fn descriptor<'a>(semantic: &'a SemanticPackage, identity: &str) -> Option<&'a DescriptorContract> {
    semantic
        .units
        .iter()
        .flat_map(|unit| &unit.descriptors)
        .find(|descriptor| descriptor.identity.qualified() == identity)
}

fn construct_payload(
    semantic: &SemanticPackage,
    unit: &SemanticUnit,
    application: &SyntaxNode,
    identity: &str,
    stack: &mut EvaluationStack,
) -> Result<CompileTimeValue, SemanticFailure> {
    let schema = descriptor(semantic, identity)
        .filter(|descriptor| {
            descriptor.kind == ObjectKind::Class && descriptor.generic_parameters.is_empty()
        })
        .ok_or_else(|| {
            error(
                unit,
                application.span,
                "annotation schema must be a concrete class",
            )
        })?;
    let mut fields = effective_object_fields(semantic, schema);
    fields.retain(|field| !field.is_static);
    for field in &fields {
        let defining_unit = source_unit(semantic, field.span.file).ok_or_else(|| {
            error(
                unit,
                application.span,
                "annotation field source is unavailable",
            )
        })?;
        if find_node(&defining_unit.tree.root, field.span)
            .is_some_and(|node| visibility(defining_unit, node) != "public")
        {
            return Err(error(
                unit,
                application.span,
                "annotation schema requires public data fields",
            ));
        }
    }
    let arguments = &application.children[1].children;
    let mut supplied = BTreeMap::new();
    for (position, argument) in arguments.iter().enumerate() {
        let (key, value_node) = match argument.children.as_slice() {
            [value] => (
                fields
                    .get(position)
                    .map(|field| field.name.as_str())
                    .ok_or_else(|| {
                        error(
                            unit,
                            argument.span,
                            "too many positional annotation arguments",
                        )
                    })?,
                value,
            ),
            [name, value] => (node_text(&unit.source, name), value),
            _ => return Err(error(unit, argument.span, "malformed annotation argument")),
        };
        if !fields.iter().any(|field| field.name == key) {
            return Err(error(
                unit,
                argument.span,
                format!("unknown annotation argument `{key}`"),
            ));
        }
        if supplied.insert(key, value_node).is_some() {
            return Err(error(
                unit,
                argument.span,
                format!("duplicate annotation argument `{key}`"),
            ));
        }
    }
    let mut payload = Vec::with_capacity(fields.len());
    for field in &fields {
        let (defining_unit, expression) = if let Some(value) = supplied.get(field.name.as_str()) {
            (unit, *value)
        } else {
            let span = field.initializer_span.ok_or_else(|| {
                error(
                    unit,
                    application.span,
                    format!("missing required annotation argument `{}`", field.name),
                )
            })?;
            let defining_unit = source_unit(semantic, span.file).ok_or_else(|| {
                error(
                    unit,
                    application.span,
                    "annotation default source is unavailable",
                )
            })?;
            let expression = find_node(&defining_unit.tree.root, span).ok_or_else(|| {
                error(unit, application.span, "annotation default is unavailable")
            })?;
            (defining_unit, expression)
        };
        let mut value = evaluate(semantic, defining_unit, expression, stack)?;
        check_type(unit, application.span, &mut value, &field.value_type)?;
        payload.push((CompileTimeValue::String(field.name.clone()), value));
    }
    Ok(CompileTimeValue::Map(payload))
}

fn definition(
    semantic: &SemanticPackage,
    unit: &SemanticUnit,
    application: &SyntaxNode,
    identity: &str,
    stack: &mut EvaluationStack,
) -> Result<(BTreeSet<AnnotationTarget>, bool), SemanticFailure> {
    if identity == MARKER {
        return Ok((BTreeSet::from([AnnotationTarget::Class]), false));
    }
    let schema = descriptor(semantic, identity)
        .ok_or_else(|| error(unit, application.span, "annotation is not a declared class"))?;
    let defining_unit = source_unit(semantic, schema.span.file).ok_or_else(|| {
        error(
            unit,
            application.span,
            "annotation definition source is unavailable",
        )
    })?;
    let declaration = find_node(&defining_unit.tree.root, schema.span).ok_or_else(|| {
        error(
            unit,
            application.span,
            "annotation definition is unavailable",
        )
    })?;
    for marker in declaration.annotations() {
        if semantic
            .resolve_name_at(
                defining_unit,
                marker.children[0].span.start,
                node_text(&defining_unit.source, &marker.children[0]),
            )
            .is_some_and(|symbol| symbol.identity == MARKER)
        {
            let CompileTimeValue::Map(payload) =
                construct_payload(semantic, defining_unit, marker, MARKER, stack)?
            else {
                unreachable!()
            };
            let mut targets = BTreeSet::new();
            let mut repeatable = false;
            for (key, value) in payload {
                match (key, value) {
                    (CompileTimeValue::String(key), CompileTimeValue::List(values))
                        if key == "targets" =>
                    {
                        for value in values {
                            let CompileTimeValue::Kind(target) = value else {
                                return Err(error(
                                    defining_unit,
                                    marker.span,
                                    "annotation targets must be typed annotation-target values",
                                ));
                            };
                            targets.insert(target);
                        }
                    }
                    (CompileTimeValue::String(key), CompileTimeValue::Boolean(value))
                        if key == "repeatable" =>
                    {
                        repeatable = value
                    }
                    _ => {
                        return Err(error(
                            defining_unit,
                            marker.span,
                            "invalid annotation definition marker",
                        ));
                    }
                }
            }
            if targets.is_empty() {
                return Err(error(
                    defining_unit,
                    marker.span,
                    "annotation requires at least one attachment target",
                ));
            }
            return Ok((targets, repeatable));
        }
    }
    Err(error(
        unit,
        application.span,
        "annotation class requires the /core/annotations definition marker",
    ))
}

fn resolve_annotations(
    semantic: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    kind: DeclarationKind,
    stack: &mut EvaluationStack,
) -> Result<Vec<ResolvedAnnotation>, SemanticFailure> {
    let target = match kind {
        DeclarationKind::Class => AnnotationTarget::Class,
        DeclarationKind::Callable => AnnotationTarget::Callable,
        DeclarationKind::Parameter => AnnotationTarget::Parameter,
        DeclarationKind::Field => AnnotationTarget::Field,
    };
    let mut result: Vec<ResolvedAnnotation> =
        Vec::with_capacity(node.annotation_applications.len());
    for application in node.annotations() {
        let name = &application.children[0];
        let symbol = semantic
            .resolve_name_at(unit, name.span.start, node_text(&unit.source, name))
            .ok_or_else(|| error(unit, name.span, "unknown annotation type"))?;
        let (targets, repeatable) =
            definition(semantic, unit, application, &symbol.identity, stack)?;
        if !targets.contains(&target) {
            return Err(error(
                unit,
                application.span,
                format!(
                    "annotation `{}` does not admit this declaration target",
                    symbol.identity
                ),
            ));
        }
        if !repeatable
            && result
                .iter()
                .any(|annotation| annotation.identity == symbol.identity)
        {
            return Err(error(
                unit,
                application.span,
                "annotation is not repeatable",
            ));
        }
        let payload = construct_payload(semantic, unit, application, &symbol.identity, stack)?;
        if symbol.identity == MARKER
            && !matches!(&payload, CompileTimeValue::Map(entries) if entries.iter().any(|(key,value)| matches!((key,value), (CompileTimeValue::String(key),CompileTimeValue::List(values)) if key == "targets" && !values.is_empty())))
        {
            return Err(error(
                unit,
                application.span,
                "annotation requires at least one attachment target",
            ));
        }
        result.push(ResolvedAnnotation {
            identity: symbol.identity.clone(),
            span: metadata_span(unit, application.span),
            payload,
        });
    }
    Ok(result)
}

fn visibility(unit: &SemanticUnit, node: &SyntaxNode) -> &'static str {
    if let Some(visibility) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Visibility)
    {
        match node_text(&unit.source, visibility) {
            "private" => return "private",
            "protected" => return "protected",
            _ => {}
        }
    }
    "public"
}

fn type_snapshot(value: &ValueType) -> serde_json::Value {
    let kind = match value {
        ValueType::Scalar(_) => "scalar",
        ValueType::Optional(inner) if matches!(inner.as_ref(), ValueType::Scalar(_)) => {
            "optional-scalar"
        }
        _ => "other",
    };
    serde_json::json!({"type": value.to_string(), "type_kind": kind})
}

fn default_value(semantic: &SemanticPackage, span: Option<Span>) -> Option<CompileTimeValue> {
    let span = span?;
    let unit = source_unit(semantic, span.file)?;
    evaluate(
        semantic,
        unit,
        find_node(&unit.tree.root, span)?,
        &mut EvaluationStack::default(),
    )
    .ok()
}

fn json_default(value: &CompileTimeValue) -> Option<String> {
    Some(match value {
        CompileTimeValue::None => "null".to_owned(),
        CompileTimeValue::Boolean(value) => value.to_string(),
        CompileTimeValue::Integer(value) | CompileTimeValue::Float(value) => value.clone(),
        CompileTimeValue::String(value) => serde_json::to_string(value).expect("string JSON"),
        _ => return None,
    })
}

fn implicit_default(value_type: &ValueType) -> Option<CompileTimeValue> {
    Some(match canonical_default(value_type)? {
        CanonicalDefault::BoolFalse => CompileTimeValue::Boolean(false),
        CanonicalDefault::AdaptiveIntegerZero | CanonicalDefault::FixedIntegerZero => {
            CompileTimeValue::Integer("0".into())
        }
        CanonicalDefault::Float32Zero | CanonicalDefault::Float64Zero => {
            CompileTimeValue::Float("0".into())
        }
        CanonicalDefault::EmptyString => CompileTimeValue::String(String::new()),
        CanonicalDefault::EmptyBytes => CompileTimeValue::Bytes(Vec::new()),
        CanonicalDefault::AbsentOptional => CompileTimeValue::None,
        CanonicalDefault::EmptyList => CompileTimeValue::List(Vec::new()),
        CanonicalDefault::EmptyMap | CanonicalDefault::EmptyUnorderedMap => {
            CompileTimeValue::Map(Vec::new())
        }
        CanonicalDefault::EmptySet | CanonicalDefault::EmptyUnorderedSet => {
            CompileTimeValue::Set(Vec::new())
        }
    })
}

fn declaration_record(
    semantic: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    kind: DeclarationKind,
    owner: Option<&str>,
    stack: &mut EvaluationStack,
) -> Result<DeclarationMetadata, SemanticFailure> {
    let name = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
        .expect("named declaration");
    let identity = owner.map_or_else(
        || format!("{}::{}", unit.namespace, node_text(&unit.source, name)),
        |owner| format!("{owner}::{}", node_text(&unit.source, name)),
    );
    let contract = unit
        .functions
        .iter()
        .find(|contract| contract.span == node.span);
    let descriptor = unit
        .descriptors
        .iter()
        .find(|descriptor| descriptor.span == node.span);
    let signature = contract.map(|contract| {
        serde_json::json!({
            "parameters": contract.parameters.iter().map(|parameter| {
                let parameter_node = find_node(&unit.tree.root, parameter.span);
                let default = parameter_node.and_then(value_node).and_then(|node| default_value(semantic, Some(node.span)));
                let mut snapshot = parameter.value_type.as_ref().map(type_snapshot).unwrap_or(serde_json::json!({"type": null, "type_kind": "other"}));
                let object = snapshot.as_object_mut().unwrap();
                object.insert("name".into(), serde_json::json!(parameter.name));
                object.insert("optional".into(), serde_json::json!(parameter.optional));
                object.insert("variadic".into(), serde_json::json!(parameter.variadic));
                object.insert("mutable".into(), serde_json::json!(parameter.mutable));
                object.insert("default".into(), serde_json::json!(default));
                object.insert("documentation".into(), serde_json::json!(parameter_node.and_then(SyntaxNode::documentation)));
                object.insert("origin".into(), serde_json::json!(metadata_span(unit, parameter.span)));
                snapshot
            }).collect::<Vec<_>>(),
            "result": contract.return_type.as_ref().map(ToString::to_string),
            "result_kind": contract.return_type.as_ref().map(type_snapshot),
            "async": contract.is_async, "throws": contract.throws, "unsafe": contract.is_unsafe,
            "receiver": contract.owner, "static": contract.is_static,
            "invocation_mode": format!("{:?}", contract.exact_invocation_mode),
        })
    });
    let fields = descriptor.map_or_else(Vec::new, |descriptor| {
        effective_object_fields(semantic, descriptor)
            .into_iter()
            .map(|field| {
                let defining_unit = field.unit;
                let field_node = find_node(&defining_unit.tree.root, field.span);
                let default = if field.initializer_span.is_some() {
                    default_value(semantic, field.initializer_span)
                } else {
                    implicit_default(&field.value_type)
                };
                let mut snapshot = type_snapshot(&field.value_type);
                let object = snapshot.as_object_mut().unwrap();
                object.insert("name".into(), serde_json::json!(field.name));
                object.insert("required".into(), serde_json::json!(field.required));
                object.insert("static".into(), serde_json::json!(field.is_static));
                object.insert(
                    "defaulted".into(),
                    serde_json::json!(field.metadata.defaulted),
                );
                object.insert(
                    "optional".into(),
                    serde_json::json!(field.metadata.optional),
                );
                object.insert("default".into(), serde_json::json!(default));
                object.insert(
                    "default_value".into(),
                    serde_json::json!(default.as_ref().and_then(json_default)),
                );
                object.insert(
                    "external_name".into(),
                    serde_json::json!(field.metadata.external_name),
                );
                object.insert("secret".into(), serde_json::json!(field.metadata.secret));
                object.insert(
                    "visibility".into(),
                    serde_json::json!(
                        field_node.map_or("public", |node| visibility(defining_unit, node))
                    ),
                );
                object.insert(
                    "origin".into(),
                    serde_json::json!(metadata_span(defining_unit, field.span)),
                );
                snapshot
            })
            .collect()
    });
    let contracts = if let Some(contract) = contract {
        serde_json::json!({"throws":contract.throws, "throwable_types":contract.thrown_types.iter().map(ToString::to_string).collect::<Vec<_>>(), "escaping_throwables":contract.escaping_throwables, "invocation_mode":format!("{:?}",contract.written_invocation_mode), "exact_invocation_mode":format!("{:?}",contract.exact_invocation_mode), "async":contract.is_async, "unsafe":contract.is_unsafe, "receiver":contract.owner, "static":contract.is_static})
    } else {
        let constructor = descriptor.and_then(|descriptor| {
            unit.functions.iter().find(|function| {
                function.name == "construct"
                    && function.owner_identity.as_ref() == Some(&descriptor.identity)
            })
        });
        serde_json::json!({"object_kind":descriptor.map(|descriptor| format!("{:?}",descriptor.kind)), "base":descriptor.and_then(|descriptor| descriptor.base.as_ref().map(ObjectIdentity::qualified)), "constructor": constructor.map(|constructor| serde_json::json!({"parameters":constructor.parameters.len(), "throws":constructor.throws, "unsafe":constructor.is_unsafe, "async":constructor.is_async}))})
    };
    Ok(DeclarationMetadata {
        identity,
        span: metadata_span(unit, node.span),
        origin: metadata_span(unit, name.span),
        kind,
        visibility: visibility(unit, node).into(),
        documentation: node.documentation().map(str::to_owned),
        annotations: resolve_annotations(semantic, unit, node, kind, stack)?,
        signature,
        fields,
        contracts,
    })
}

fn collect(
    semantic: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    owner: Option<&str>,
    records: &mut Vec<DeclarationMetadata>,
    stack: &mut EvaluationStack,
) -> Result<(), SemanticFailure> {
    let kind = match node.kind {
        SyntaxKind::ClassDeclaration => Some(DeclarationKind::Class),
        SyntaxKind::FunctionDeclaration => Some(DeclarationKind::Callable),
        SyntaxKind::Parameter => Some(DeclarationKind::Parameter),
        SyntaxKind::Binding if owner.is_some() => Some(DeclarationKind::Field),
        _ => None,
    };
    if let Some(kind) = kind {
        let record = declaration_record(semantic, unit, node, kind, owner, stack)?;
        let identity = record.identity.clone();
        records.push(record);
        if kind == DeclarationKind::Class {
            for child in &node.children {
                collect(semantic, unit, child, Some(&identity), records, stack)?;
            }
        } else if kind == DeclarationKind::Callable {
            for child in &node.children {
                if child.kind == SyntaxKind::ParameterList {
                    collect(semantic, unit, child, Some(&identity), records, stack)?;
                } else if child.kind == SyntaxKind::Block {
                    collect(semantic, unit, child, None, records, stack)?;
                }
            }
        }
        return Ok(());
    }
    for child in &node.children {
        collect(semantic, unit, child, owner, records, stack)?;
    }
    Ok(())
}

pub(super) fn populate_declaration_metadata(
    semantic: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    let mut results = Vec::with_capacity(semantic.units.len());
    let mut stack = EvaluationStack::default();
    for unit in &semantic.units {
        let mut records = Vec::new();
        collect(
            semantic,
            unit,
            &unit.tree.root,
            None,
            &mut records,
            &mut stack,
        )?;
        records.sort_by_key(|record| (record.span.file, record.span.start, record.span.end));
        results.push(records);
    }
    for (unit, records) in semantic.units.iter_mut().zip(results) {
        unit.declaration_metadata = records;
    }
    semantic.metadata_constant_reads = stack.reads;
    Ok(())
}
