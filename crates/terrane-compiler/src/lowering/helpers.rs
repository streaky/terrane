use super::prelude::*;

pub(super) fn find_node(
    node: &SyntaxNode,
    kind: SyntaxKind,
    span: crate::Span,
) -> Option<&SyntaxNode> {
    if node.kind == kind && node.span == span {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_node(child, kind, span))
}

pub(super) fn literal_or_text(source: &SourceFile, node: &SyntaxNode) -> String {
    let text = &source.text()[node.span.start..node.span.end];
    if node.kind == SyntaxKind::Literal {
        literal(text)
    } else {
        text.trim().to_owned()
    }
}

pub(super) fn literal(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed == "true" || trimmed == "false" {
        return trimmed.to_owned();
    }
    if trimmed.starts_with("b'") && trimmed.ends_with('\'') {
        let value = crate::lexer::unescape_bytes(&trimmed[2..trimmed.len() - 1])
            .expect("lexer rejects malformed byte escapes before lowering");
        return format!("Vec::from({value:?})");
    }
    let compact = trimmed.replace('_', "");
    if let Some(value) = integer_literal(&compact) {
        return value.to_string();
    }
    if compact.parse::<f64>().is_ok() {
        return compact;
    }
    let value = if let Some(value) = trimmed.strip_prefix('>') {
        if let Some(block) = value.strip_prefix('>') {
            block_string(block)
        } else {
            value.to_owned()
        }
    } else if trimmed.len() >= 2
        && ((trimmed.starts_with('\'') && trimmed.ends_with('\''))
            || (trimmed.starts_with('"') && trimmed.ends_with('"')))
    {
        unescape(&trimmed[1..trimmed.len() - 1])
    } else {
        trimmed.to_owned()
    };
    format!("String::from({value:?})")
}

pub(super) fn adaptive_literal(text: &str) -> String {
    let compact = text.trim().replace('_', "");
    let value = integer_literal(&compact)
        .expect("semantic analysis accepted a non-integer adaptive literal");
    let decimal = value.to_string();
    if decimal.parse::<i128>().is_ok() {
        format!("terrane_int_support::Int::from({decimal}_i128)")
    } else {
        format!("terrane_int_support::Int::from_decimal({decimal:?})")
    }
}

pub(super) fn lower_contextual_constant(
    constant: ContextualConstant,
    destination: ScalarType,
) -> String {
    match constant {
        ContextualConstant::Integer(value) if destination == ScalarType::Int => {
            adaptive_literal(&value.to_string())
        }
        ContextualConstant::Integer(value) => value.to_string(),
        ContextualConstant::Float32(value) => float32_literal(value),
        ContextualConstant::Float64(value) => float64_literal(value),
    }
}

pub(super) fn float32_literal(value: f32) -> String {
    if value.is_nan() {
        "f32::NAN".to_owned()
    } else if value == f32::INFINITY {
        "f32::INFINITY".to_owned()
    } else if value == f32::NEG_INFINITY {
        "f32::NEG_INFINITY".to_owned()
    } else {
        format!("{value:?}_f32")
    }
}

pub(super) fn float64_literal(value: f64) -> String {
    if value.is_nan() {
        "f64::NAN".to_owned()
    } else if value == f64::INFINITY {
        "f64::INFINITY".to_owned()
    } else if value == f64::NEG_INFINITY {
        "f64::NEG_INFINITY".to_owned()
    } else {
        format!("{value:?}_f64")
    }
}

pub(super) fn integer_literal(text: &str) -> Option<BigInt> {
    let (radix, digits) =
        if let Some(digits) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
            (16, digits)
        } else if let Some(digits) = text.strip_prefix("0o").or_else(|| text.strip_prefix("0O")) {
            (8, digits)
        } else if let Some(digits) = text.strip_prefix("0b").or_else(|| text.strip_prefix("0B")) {
            (2, digits)
        } else {
            (10, text)
        };
    BigInt::parse_bytes(digits.as_bytes(), radix)
}

pub(super) fn block_string(text: &str) -> String {
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default();
    if !first.trim().is_empty() {
        return first.to_owned();
    }
    let collected = lines.collect::<Vec<_>>();
    let indent = collected
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    collected
        .iter()
        .map(|line| line.get(indent..).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn unescape(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character == '\\' {
            match chars.next() {
                Some('n') => output.push('\n'),
                Some('r') => output.push('\r'),
                Some('t') => output.push('\t'),
                Some('\\') | None => output.push('\\'),
                Some('\'') => output.push('\''),
                Some('"') => output.push('"'),
                Some(other) => output.push(other),
            }
        } else {
            output.push(character);
        }
    }
    output
}

pub(super) fn effective_object_fields<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a ObjectField> {
    fn collect<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        fields: &mut Vec<&'a ObjectField>,
    ) {
        if let Some(base) = object.base.as_ref().and_then(|identity| {
            unit.objects
                .iter()
                .find(|object| object.identity == *identity)
        }) {
            collect(unit, base, fields);
        }
        for reused in &object.traits {
            if let Some(reused) = unit
                .objects
                .iter()
                .find(|candidate| candidate.identity == *reused)
            {
                collect(unit, reused, fields);
            }
        }
        for field in &object.fields {
            if let Some(index) = fields.iter().position(|existing| {
                existing.name == field.name && existing.is_static == field.is_static
            }) {
                fields[index] = field;
            } else {
                fields.push(field);
            }
        }
    }
    let mut fields = Vec::new();
    collect(unit, object, &mut fields);
    fields
}

pub(super) fn object_descendants<'a>(
    unit: &'a SemanticUnit,
    object: &ObjectContract,
) -> Vec<&'a ObjectContract> {
    unit.objects
        .iter()
        .filter(|candidate| {
            let mut base = candidate.base.as_ref();
            while let Some(identity) = base {
                if identity == &object.identity {
                    return true;
                }
                base = unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.identity == *identity)
                    .and_then(|candidate| candidate.base.as_ref());
            }
            false
        })
        .collect()
}

pub(super) fn effective_object_interfaces<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a ObjectIdentity> {
    let mut interfaces = object
        .base
        .as_ref()
        .and_then(|identity| {
            unit.objects
                .iter()
                .find(|candidate| candidate.identity == *identity)
        })
        .map_or_else(Vec::new, |base| effective_object_interfaces(unit, base));
    for interface in &object.interfaces {
        if !interfaces.contains(&interface) {
            interfaces.push(interface);
        }
    }
    interfaces
}

pub(super) fn object_destructor_chain<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a FunctionContract> {
    let mut destructors = object
        .base
        .as_ref()
        .and_then(|identity| {
            unit.objects
                .iter()
                .find(|candidate| candidate.identity == *identity)
        })
        .map_or_else(Vec::new, |base| object_destructor_chain(unit, base));
    if let Some(destructor) = unit.functions.iter().find(|method| {
        method.owner.as_deref() == Some(object.name.as_str()) && method.name == "destruct"
    }) {
        destructors.push(destructor);
    }
    destructors
}

pub(super) fn effective_object_methods<'a>(
    unit: &'a SemanticUnit,
    object: &'a ObjectContract,
) -> Vec<&'a FunctionContract> {
    fn collect<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        methods: &mut Vec<&'a FunctionContract>,
    ) {
        if let Some(base) = object.base.as_ref().and_then(|identity| {
            unit.objects
                .iter()
                .find(|object| object.identity == *identity)
        }) {
            collect(unit, base, methods);
        }
        for reused in &object.traits {
            if let Some(reused) = unit
                .objects
                .iter()
                .find(|candidate| candidate.identity == *reused)
            {
                collect(unit, reused, methods);
            }
        }
        for method in unit
            .functions
            .iter()
            .filter(|method| method.owner.as_deref() == Some(object.identity.name.as_str()))
        {
            if let Some(index) = methods.iter().position(|existing| {
                existing.name == method.name && existing.is_static == method.is_static
            }) {
                methods[index] = method;
            } else {
                methods.push(method);
            }
        }
    }
    let mut methods = Vec::new();
    collect(unit, object, &mut methods);
    methods
}

pub(super) fn union_type_name(binding: &TypedBinding) -> String {
    format!("TerraneUnionF{}S{}", binding.span.file, binding.span.start)
}
pub(super) fn find_node_by_span(node: &SyntaxNode, span: crate::Span) -> Option<&SyntaxNode> {
    (node.span == span).then_some(node).or_else(|| {
        node.children
            .iter()
            .find_map(|child| find_node_by_span(child, span))
    })
}

pub(super) fn binding_initializer(node: &SyntaxNode, name_index: usize) -> Option<&SyntaxNode> {
    node.children
        .iter()
        .enumerate()
        .rev()
        .find(|(index, child)| {
            *index != name_index
                && !matches!(
                    child.kind,
                    SyntaxKind::TypeExpression
                        | SyntaxKind::Visibility
                        | SyntaxKind::DeclarationQualifier
                )
        })
        .map(|(_, child)| child)
}

pub(super) fn rust_type(ty: ScalarType) -> &'static str {
    ty.lowering_type()
}

pub(super) const fn rust_value_is_copy(ty: &ValueType) -> bool {
    matches!(
        ty,
        ValueType::Scalar(
            ScalarType::Bool
                | ScalarType::Int8
                | ScalarType::Int16
                | ScalarType::Int32
                | ScalarType::Int64
                | ScalarType::Int128
                | ScalarType::Uint8
                | ScalarType::Uint16
                | ScalarType::Uint32
                | ScalarType::Uint64
                | ScalarType::Uint128
                | ScalarType::Float32
                | ScalarType::Float64
                | ScalarType::None
        )
    )
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "element lowering owns the recursively described value type"
)]
pub(super) fn rust_element_type(package: &SemanticPackage, ty: ElementType) -> String {
    rust_value_type(package, ty.value_type())
}

#[expect(
    clippy::too_many_lines,
    reason = "the closed semantic value-type enum has one exhaustive Rust representation mapping"
)]
pub(super) fn rust_value_type(package: &SemanticPackage, ty: ValueType) -> String {
    match ty {
        ValueType::Scalar(scalar) => rust_type(scalar).to_owned(),
        ValueType::Optional(inner) => {
            format!("Option<{}>", rust_value_type(package, *inner))
        }
        ValueType::OverflowResult(scalar) => {
            format!("terrane_int_support::OverflowResult<{}>", rust_type(scalar))
        }
        ValueType::DivRemResult(scalar) => {
            format!("terrane_int_support::DivRemResult<{}>", rust_type(scalar))
        }
        ValueType::StringView(crate::semantics::TextUnit::Bytes) => "Vec<u8>".to_owned(),
        ValueType::StringView(_) | ValueType::TextRangeView(_) => "String".to_owned(),
        ValueType::StringList => "Vec<String>".to_owned(),
        ValueType::Encoding => "terrane_string_support::Encoding".to_owned(),
        ValueType::TextRange => "terrane_string_support::TextRange".to_owned(),
        ValueType::Iterator(item) => {
            format!(
                "terrane_collection_support::Iterator<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::IterationStep(item) => {
            format!(
                "terrane_collection_support::IterationStep<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::List(item) => {
            format!(
                "terrane_collection_support::List<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::Map(key, value) => format!(
            "terrane_collection_support::Map<{}, {}>",
            rust_element_type(package, key),
            rust_element_type(package, value)
        ),
        ValueType::Set(item) => {
            format!(
                "terrane_collection_support::Set<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::Tuple(item, _) => {
            format!(
                "terrane_collection_support::Tuple<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::Range => "terrane_collection_support::Range".to_owned(),
        ValueType::Entry(key, value) => format!(
            "terrane_collection_support::Entry<{}, {}>",
            rust_element_type(package, key),
            rust_element_type(package, value)
        ),
        ValueType::UnorderedMap(key, value) => format!(
            "terrane_collection_support::UnorderedMap<{}, {}>",
            rust_element_type(package, key),
            rust_element_type(package, value)
        ),
        ValueType::UnorderedSet(item) => {
            format!(
                "terrane_collection_support::UnorderedSet<{}>",
                rust_element_type(package, item)
            )
        }
        ValueType::TextRangeList => "Vec<terrane_string_support::TextRange>".to_owned(),
        ValueType::Function(parameters, result) => format!(
            "std::sync::Arc<dyn Fn({}) -> Result<{}, TerraneError> + Send + Sync>",
            parameters
                .into_iter()
                .map(|parameter| rust_element_type(package, parameter))
                .collect::<Vec<_>>()
                .join(", "),
            rust_element_type(package, result)
        ),
        ValueType::AsyncFunction(parameters, result) => format!(
            "std::sync::Arc<dyn Fn({}) -> std::pin::Pin<Box<dyn Future<Output = {}>>> + Send + Sync>",
            parameters
                .into_iter()
                .map(|parameter| rust_element_type(package, parameter))
                .collect::<Vec<_>>()
                .join(", "),
            rust_element_type(package, result)
        ),
        ValueType::Task(result) => {
            format!(
                "std::pin::Pin<Box<dyn Future<Output = {}>>>",
                rust_element_type(package, result)
            )
        }
        ValueType::ScopedTask(result) => {
            format!("TerraneScopedTask<{}>", rust_element_type(package, result))
        }
        ValueType::TaskScope => "TerraneTaskScope".to_owned(),
        ValueType::TaskOutcome(result) => {
            format!("TerraneTaskOutcome<{}>", rust_element_type(package, result))
        }
        ValueType::PlatformStreamHandle => "TerranePlatformStreamHandle".to_owned(),
        ValueType::FilesystemAuthority => "TerraneFilesystemAuthority".to_owned(),
        ValueType::PlatformFilesystemResult => "TerraneFilesystemResult".to_owned(),
        ValueType::PlatformOpenResult => "TerranePlatformOpenResult".to_owned(),
        ValueType::PlatformReadResult => "TerranePlatformReadResult".to_owned(),
        ValueType::PlatformWriteResult => "TerranePlatformWriteResult".to_owned(),
        ValueType::PlatformUnitResult => "TerranePlatformUnitResult".to_owned(),
        ValueType::PlatformDataResult => "terrane_document_support::DataResult".to_owned(),
        ValueType::PlatformUrlResult => "terrane_document_support::UrlResult".to_owned(),
        ValueType::Descriptor(_) => "TerraneDescriptor".to_owned(),
        ValueType::PlatformCapability | ValueType::PlatformResourceHandle => {
            "TerranePlatformCapability".to_owned()
        }
        ValueType::PlatformResult => "TerranePlatformResult".to_owned(),
        ValueType::Object(identity)
            if identity.namespace == "/core/errors" && identity.name == "throwable" =>
        {
            "TerraneError".to_owned()
        }
        ValueType::Object(identity) => rust_object_type_name(package, &identity),
        ValueType::SharedReference(item) => format!(
            "std::sync::Arc<std::sync::Mutex<{}>>",
            rust_element_type(package, item)
        ),
        ValueType::Reference(item) => format!(
            "std::sync::Weak<std::sync::Mutex<{}>>",
            rust_element_type(package, item)
        ),
    }
}

pub(super) fn integer_range_contains(destination: ScalarType, source: ScalarType) -> bool {
    let Some((destination_signed, destination_bits)) = fixed_integer_shape(destination) else {
        return false;
    };
    let Some((source_signed, source_bits)) = fixed_integer_shape(source) else {
        return false;
    };
    match (destination_signed, source_signed) {
        (true, true) | (false, false) => destination_bits >= source_bits,
        (true, false) => destination_bits > source_bits,
        (false, true) => false,
    }
}

pub(super) fn exact_integer_float_widening(source: ScalarType, destination: ScalarType) -> bool {
    let Some((_, bits)) = fixed_integer_shape(source) else {
        return false;
    };
    match destination {
        ScalarType::Float32 => bits <= 16,
        ScalarType::Float64 => bits <= 32,
        _ => false,
    }
}

pub(super) const fn fixed_integer_shape(ty: ScalarType) -> Option<(bool, u16)> {
    match ty {
        ScalarType::Int8 => Some((true, 8)),
        ScalarType::Int16 => Some((true, 16)),
        ScalarType::Int32 => Some((true, 32)),
        ScalarType::Int64 => Some((true, 64)),
        ScalarType::Int128 => Some((true, 128)),
        ScalarType::Uint8 => Some((false, 8)),
        ScalarType::Uint16 => Some((false, 16)),
        ScalarType::Uint32 => Some((false, 32)),
        ScalarType::Uint64 => Some((false, 64)),
        ScalarType::Uint128 => Some((false, 128)),
        _ => None,
    }
}

pub(super) fn block_may_fall_through(block: &SyntaxNode) -> bool {
    block.children.last().is_none_or(statement_may_fall_through)
}

pub(super) fn statement_may_fall_through(statement: &SyntaxNode) -> bool {
    match statement.kind {
        SyntaxKind::ReturnStatement
        | SyntaxKind::ThrowStatement
        | SyntaxKind::BreakStatement
        | SyntaxKind::ContinueStatement => false,
        SyntaxKind::IfStatement => {
            let mut branches = statement.children.iter().skip(1);
            let Some(first) = branches.next() else {
                return true;
            };
            let first_falls_through = first
                .children
                .last()
                .filter(|child| child.kind == SyntaxKind::Block)
                .map_or_else(|| block_may_fall_through(first), block_may_fall_through);
            let mut has_else = false;
            let mut any_falls_through = first_falls_through;
            for branch in branches {
                has_else |= branch.kind == SyntaxKind::ElseClause;
                any_falls_through |= branch
                    .children
                    .last()
                    .filter(|child| child.kind == SyntaxKind::Block)
                    .is_none_or(block_may_fall_through);
            }
            !has_else || any_falls_through
        }
        SyntaxKind::TryStatement => {
            let try_falls_through = statement
                .children
                .first()
                .is_none_or(block_may_fall_through);
            let catch_falls_through = statement
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
                .filter_map(|clause| {
                    clause
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block)
                })
                .any(block_may_fall_through);
            let finally_returns = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::FinallyClause)
                .and_then(|clause| clause.children.first())
                .is_some_and(|block| !block_may_fall_through(block));
            !finally_returns && (try_falls_through || catch_falls_through)
        }
        _ => true,
    }
}

pub(super) fn rust_builtin_error_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "arithmetic-overflow" => Some("ArithmeticOverflow"),
        "division-by-zero" => Some("DivisionByZero"),
        "integer-conversion-overflow" => Some("IntegerConversionOverflow"),
        "negative-shift-count" => Some("NegativeShiftCount"),
        "coercion-error" => Some("CoercionError"),
        "decode-error" => Some("DecodeError"),
        "index-error" => Some("IndexError"),
        "missing-key" => Some("MissingKey"),
        "resource-error" => Some("ResourceError"),
        "error" | "throwable" => Some("SourceError"),
        _ => None,
    }
}

pub(super) fn function_namespace_suffix(
    package: &SemanticPackage,
    contract: &FunctionContract,
) -> String {
    package
        .units
        .iter()
        .find(|unit| unit.source.id() == contract.span.file)
        .expect("function contract source must belong to a semantic unit")
        .namespace
        .trim_start_matches('/')
        .split('/')
        .map(rust_name)
        .collect::<Vec<_>>()
        .join("_")
}

pub(super) fn function_name(package: &SemanticPackage, contract: &FunctionContract) -> String {
    let duplicates = package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .filter(|candidate| {
            candidate.owner.is_none()
                && candidate.name == contract.name
                && candidate.span != contract.span
        })
        .collect::<Vec<_>>();
    if !duplicates.is_empty() && contract.owner.is_none() {
        let namespace = function_namespace_suffix(package, contract);
        let mut name = format!("{}_terrane_{namespace}", rust_name(&contract.name));
        let first_normalized_namespace = duplicates
            .iter()
            .filter(|candidate| function_namespace_suffix(package, candidate) == namespace)
            .map(|candidate| candidate.span.file)
            .chain(std::iter::once(contract.span.file))
            .min()
            .expect("the declaring function supplies a normalized namespace");
        if contract.span.file != first_normalized_namespace {
            write!(name, "_f{}", contract.span.file).unwrap();
        }
        name
    } else if contract.name == "main" {
        "main".to_owned()
    } else {
        rust_name(&contract.name)
    }
}

pub(super) fn namespace_binding_name(file: u32, name: &str) -> String {
    format!("__TERRANE_F{file}_{}", rust_name(name).to_uppercase())
}

pub(super) fn global_binding_name(name: &str) -> String {
    format!("__TERRANE_GLOBAL_{}", rust_name(name).to_uppercase())
}

pub(super) fn rust_object_name(name: &str) -> String {
    let mut uppercase = true;
    name.chars()
        .filter_map(|character| {
            if character == '-' {
                uppercase = true;
                None
            } else if uppercase {
                uppercase = false;
                Some(character.to_ascii_uppercase())
            } else {
                Some(character)
            }
        })
        .collect()
}

/// Qualifies colliding names with source-byte-length-prefixed CamelCase namespace segments.
///
/// Counting the source segment keeps the encoding injective when case conversion erases spelling
/// differences; the following CamelCase letter also makes adjacent decimal lengths unambiguous.
pub(super) fn rust_object_type_name(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
) -> String {
    let collides = package
        .units
        .iter()
        .flat_map(|unit| &unit.objects)
        .filter(|object| object.identity.name == identity.name)
        .map(|object| &object.identity)
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        > 1;
    if !collides {
        return rust_object_name(&identity.name);
    }
    let mut namespace = String::new();
    for segment in identity.namespace.trim_start_matches('/').split('/') {
        write!(namespace, "{}{}", segment.len(), rust_object_name(segment))
            .expect("writing to a string cannot fail");
    }
    format!("TerraneNs{namespace}{}", rust_object_name(&identity.name))
}

pub(super) fn rust_static_field_name(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
    field: &str,
) -> String {
    format!(
        "TERRANE_STATIC_{}_{}",
        rust_object_type_name(package, identity).to_ascii_uppercase(),
        rust_name(field).to_ascii_uppercase()
    )
}

pub(super) fn rust_name(name: &str) -> String {
    let readable_identifier = name.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_');
    let keyword = matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    );
    if readable_identifier && !keyword {
        return name.replace('-', "_");
    }
    let mut output = String::from("__trn_");
    for byte in name.bytes() {
        write!(output, "{byte:02x}").expect("writing to a String cannot fail");
    }
    output
}

pub(super) fn display_path(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("<memory>")
        .to_owned()
}
