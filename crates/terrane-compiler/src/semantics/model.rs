use super::prelude::*;

pub const BOOTSTRAP_VERSION: &str = "2";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SymbolKind {
    Binding,
    Function,
    TypeDescriptor,
    Interface,
    Class,
    Trait,
    ErrorObject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
    pub identity: String,
    pub(crate) lowering_identity: Option<String>,
    pub name: String,
    pub namespace: String,
    pub visibility: Visibility,
    pub global: bool,
    pub constant: bool,
    pub kind: SymbolKind,
    pub declaration_span: Option<Span>,
    pub binding_span: Option<Span>,
}

impl Symbol {
    #[must_use]
    pub(crate) fn compiler_identity(&self) -> &str {
        self.lowering_identity.as_deref().unwrap_or(&self.identity)
    }

    /// Returns this symbol's canonical descriptor identity when it denotes a descriptor.
    #[must_use]
    pub fn descriptor_identity(&self) -> Option<&str> {
        (self.kind == SymbolKind::TypeDescriptor).then_some(self.identity.as_str())
    }

    #[must_use]
    pub fn available_in_function_body(&self) -> bool {
        self.kind != SymbolKind::Binding || self.constant || self.global
    }
}

#[derive(Clone, Debug, Default)]
pub struct Namespace {
    pub symbols: BTreeMap<String, Symbol>,
}

#[derive(Clone, Debug)]
pub struct SemanticPackage {
    pub identity: String,
    pub prelude: bool,
    pub reflection: crate::package::ReflectionProfile,
    pub executor: crate::package::ExecutorProfile,
    pub(crate) execution_strategy: crate::execution::ExecutionStrategy,
    pub(crate) execution_requirements: crate::execution::ExecutionRequirements,
    pub profile: crate::package::CapabilityProfile,
    pub projection: crate::projection::Projection,
    pub namespaces: BTreeMap<String, Namespace>,
    pub globals: BTreeMap<String, Symbol>,
    pub prelude_bindings: BTreeMap<String, Symbol>,
    pub descriptor_constructs: BTreeMap<String, Symbol>,
    pub units: Vec<SemanticUnit>,
    pub(super) binding_events: BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
    pub(super) referenced_functions: BTreeSet<(u32, usize, usize)>,
    pub(super) import_warnings: Vec<Diagnostic>,
    pub bootstrap_version: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementType(Box<ValueType>);

impl ElementType {
    pub(crate) fn new(value_type: ValueType) -> Self {
        Self(Box::new(value_type))
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.0.as_ref().clone()
    }

    pub(crate) fn value_type_ref(&self) -> &ValueType {
        self.0.as_ref()
    }

    pub(super) fn scalar(&self) -> Option<ScalarType> {
        match self.0.as_ref() {
            ValueType::Scalar(scalar) => Some(*scalar),
            _ => None,
        }
    }
}

impl std::fmt::Display for ElementType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceProjection {
    Field(String),
    Element,
    CallResult,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceProvenance {
    pub owner: Span,
    pub external_lender: bool,
    pub lender_parameter: Option<Span>,
    pub path: Vec<ReferenceProjection>,
    pub lifetime_end: Option<Span>,
}
pub(super) fn iterable_item_type(
    unit: &SemanticUnit,
    value_type: ValueType,
) -> Result<ValueType, (&'static str, Option<Span>)> {
    if !matches!(value_type, ValueType::Object(_) | ValueType::Reference(_))
        && !descriptor_has_member(unit, &value_type, "iterator")
    {
        return Err(("collection iteration requires an iterable value", None));
    }
    match value_type {
        ValueType::Scalar(ScalarType::String)
        | ValueType::StringList
        | ValueType::StringView(TextUnit::Scalars | TextUnit::Graphemes) => {
            Ok(ValueType::Scalar(ScalarType::String))
        }
        ValueType::TextRangeList => Ok(ValueType::TextRange),
        ValueType::Scalar(ScalarType::Bytes) | ValueType::StringView(TextUnit::Bytes) => {
            Ok(ValueType::Scalar(ScalarType::Uint8))
        }
        ValueType::Iterator(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::UnorderedSet(item)
        | ValueType::Tuple(item, _) => Ok(item.value_type()),
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value) => {
            Ok(ValueType::Entry(key, value))
        }
        ValueType::Range => Ok(ValueType::Scalar(ScalarType::Int)),
        ValueType::Reference(item) => iterable_item_type(unit, item.value_type())
            .map(|item| ValueType::Reference(ElementType::new(item))),
        ValueType::Object(identity) => {
            let iterator =
                super::member_inference::descriptor_protocol_method(unit, &identity, "iterator")
                    .ok_or((
                        "source iterable must define a non-static `iterator` method",
                        None,
                    ))?;
            if iterator.is_async
                || iterator.throws
                || iterator.mutates_receiver
                || !iterator.parameters.is_empty()
            {
                return Err((
                    "source iterable `iterator` must be synchronous, non-throwing, non-mutating, and parameterless",
                    Some(iterator.span),
                ));
            }
            match iterator.return_type.as_ref() {
                Some(ValueType::Iterator(item)) => Ok(item.value_type()),
                Some(ValueType::Object(iterator_identity)) => {
                    let next = super::member_inference::descriptor_protocol_method(
                        unit,
                        iterator_identity,
                        "next",
                    )
                    .ok_or((
                        "source iterator must define a non-static `next` method",
                        Some(iterator.span),
                    ))?;
                    if next.is_async || next.throws || !next.parameters.is_empty() {
                        return Err((
                            "source iterator `next` must be synchronous, non-throwing, and parameterless",
                            Some(next.span),
                        ));
                    }
                    match next.return_type.as_ref() {
                        Some(ValueType::IterationStep(item)) => Ok(item.value_type()),
                        _ => Err((
                            "source iterator `next` must return `iteration-step of T`",
                            Some(next.span),
                        )),
                    }
                }
                _ => Err((
                    "source iterable `iterator` must return a built-in iterator or source iterator object",
                    Some(iterator.span),
                )),
            }
        }
        _ => Err(("collection iteration requires an iterable value", None)),
    }
}

pub(super) fn iteration_target_bindings(
    unit: &SemanticUnit,
    target: &SyntaxNode,
    visible_from: usize,
    scope: Span,
    item_type: ValueType,
) -> Result<Vec<TypedBinding>, SemanticFailure> {
    let binding = |name: &SyntaxNode, value_type| TypedBinding {
        name: node_text(&unit.source, name).to_owned(),
        span: name.span,
        visible_from,
        scope: Some(scope),
        value_type,
        destination_arms: Vec::new(),
        storage_type: None,
        mutable: false,
    };
    match (target.children.as_slice(), item_type) {
        ([name], item_type) => Ok(vec![binding(name, item_type)]),
        ([key_name, value_name], ValueType::Entry(key, value)) => Ok(vec![
            binding(key_name, key.value_type()),
            binding(value_name, value.value_type()),
        ]),
        ([_, _], other) => Err(failure(
            &unit.source,
            "T0016",
            format!(
                "`key, value` iteration destructuring requires an `entry` item, found `{other}`"
            ),
            target.span,
        )),
        (names, ValueType::Entry(_, _)) => Err(failure(
            &unit.source,
            "T0016",
            format!(
                "entry iteration requires one target or exactly two destructuring targets, found {}",
                names.len()
            ),
            target.span,
        )),
        (names, other) => Err(failure(
            &unit.source,
            "T0016",
            format!(
                "iteration item of type `{other}` does not support {}-target destructuring",
                names.len()
            ),
            target.span,
        )),
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ObjectIdentity {
    pub namespace: String,
    pub name: String,
}

impl ObjectIdentity {
    pub(super) fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    pub(crate) fn qualified(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl std::fmt::Display for ObjectIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskTransferability {
    Local,
    Transferable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    Scalar(ScalarType),
    Optional(Box<ValueType>),
    OverflowResult(ScalarType),
    DivRemResult(ScalarType),
    FloatDecomposition(ScalarType),
    StringView(TextUnit),
    StringList,
    TextRange,
    TextRangeView(TextUnit),
    TextRangeList,
    Iterator(ElementType),
    IterationStep(ElementType),
    IterationEnd,
    AsyncIterationStep(ElementType),
    AsyncSinkOutcome,
    ChannelPair(ElementType),
    ChannelSender(ElementType),
    ChannelReceiver(ElementType),
    ChannelSendOutcome(ElementType),
    ChannelReceiveOutcome(ElementType),
    ChannelOverflowPolicy,
    DocumentDecodeOutcome(ElementType),
    DocumentDiagnostic,
    List(ElementType),
    Map(ElementType, ElementType),
    Set(ElementType),
    Tuple(ElementType, Option<usize>),
    Range,
    Entry(ElementType, ElementType),
    UnorderedMap(ElementType, ElementType),
    UnorderedSet(ElementType),
    Encoding,
    Function(Vec<ElementType>, ElementType),
    AsyncFunction(Vec<ElementType>, ElementType, TaskTransferability),
    Descriptor(String),
    Task(ElementType, TaskTransferability),
    ScopedTask(ElementType, TaskTransferability),
    TaskScope,
    TaskOutcome(ElementType),
    FilesystemAuthority,
    PlatformFilesystemResult,
    PlatformStreamHandle,
    PlatformOpenResult,
    PlatformReadResult,
    PlatformWriteResult,
    PlatformUnitResult,
    PlatformDataResult,
    PlatformUrlResult,
    PlatformCapability,
    PlatformResourceHandle,
    PlatformResult,
    Object(ObjectIdentity),
    Reference(ElementType),
    SharedReference(ElementType),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalDefault {
    BoolFalse,
    AdaptiveIntegerZero,
    FixedIntegerZero,
    Float32Zero,
    Float64Zero,
    EmptyString,
    EmptyBytes,
    AbsentOptional,
    EmptyList,
    EmptyMap,
    EmptySet,
    EmptyUnorderedMap,
    EmptyUnorderedSet,
}

pub(crate) fn canonical_default(value_type: &ValueType) -> Option<CanonicalDefault> {
    match value_type {
        ValueType::Scalar(scalar) => match scalar {
            ScalarType::Bool => Some(CanonicalDefault::BoolFalse),
            ScalarType::Int => Some(CanonicalDefault::AdaptiveIntegerZero),
            ScalarType::Int8
            | ScalarType::Int16
            | ScalarType::Int32
            | ScalarType::Int64
            | ScalarType::Int128
            | ScalarType::Uint8
            | ScalarType::Uint16
            | ScalarType::Uint32
            | ScalarType::Uint64
            | ScalarType::Uint128 => Some(CanonicalDefault::FixedIntegerZero),
            ScalarType::Float32 => Some(CanonicalDefault::Float32Zero),
            ScalarType::Float64 => Some(CanonicalDefault::Float64Zero),
            ScalarType::String => Some(CanonicalDefault::EmptyString),
            ScalarType::Bytes => Some(CanonicalDefault::EmptyBytes),
            ScalarType::None => None,
        },
        ValueType::Optional(_) => Some(CanonicalDefault::AbsentOptional),
        ValueType::List(_) => Some(CanonicalDefault::EmptyList),
        ValueType::Map(_, _) => Some(CanonicalDefault::EmptyMap),
        ValueType::Set(_) => Some(CanonicalDefault::EmptySet),
        ValueType::UnorderedMap(_, _) => Some(CanonicalDefault::EmptyUnorderedMap),
        ValueType::UnorderedSet(_) => Some(CanonicalDefault::EmptyUnorderedSet),
        ValueType::OverflowResult(_)
        | ValueType::DivRemResult(_)
        | ValueType::FloatDecomposition(_)
        | ValueType::StringView(_)
        | ValueType::StringList
        | ValueType::TextRange
        | ValueType::TextRangeView(_)
        | ValueType::TextRangeList
        | ValueType::Iterator(_)
        | ValueType::IterationStep(_)
        | ValueType::IterationEnd
        | ValueType::AsyncIterationStep(_)
        | ValueType::AsyncSinkOutcome
        | ValueType::ChannelPair(_)
        | ValueType::ChannelSender(_)
        | ValueType::ChannelReceiver(_)
        | ValueType::ChannelSendOutcome(_)
        | ValueType::ChannelReceiveOutcome(_)
        | ValueType::ChannelOverflowPolicy
        | ValueType::DocumentDecodeOutcome(_)
        | ValueType::DocumentDiagnostic
        | ValueType::Tuple(_, _)
        | ValueType::Range
        | ValueType::Entry(_, _)
        | ValueType::Encoding
        | ValueType::Function(_, _)
        | ValueType::AsyncFunction(_, _, _)
        | ValueType::Descriptor(_)
        | ValueType::Task(_, _)
        | ValueType::ScopedTask(_, _)
        | ValueType::TaskScope
        | ValueType::TaskOutcome(_)
        | ValueType::FilesystemAuthority
        | ValueType::PlatformFilesystemResult
        | ValueType::PlatformStreamHandle
        | ValueType::PlatformOpenResult
        | ValueType::PlatformReadResult
        | ValueType::PlatformWriteResult
        | ValueType::PlatformUnitResult
        | ValueType::PlatformDataResult
        | ValueType::PlatformUrlResult
        | ValueType::PlatformCapability
        | ValueType::PlatformResourceHandle
        | ValueType::PlatformResult
        | ValueType::Object(_)
        | ValueType::Reference(_)
        | ValueType::SharedReference(_) => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextUnit {
    Bytes,
    Scalars,
    Graphemes,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StringFamily {
    Trim,
    Contains,
    Find,
    Upper,
    Lower,
    Normalise,
    CaseFold,
    Split,
    Replace,
    Encode,
    Decode,
}

impl StringFamily {
    pub(crate) const fn source_name(self) -> &'static str {
        match self {
            Self::Trim => "trim",
            Self::Contains => "contains",
            Self::Find => "find",
            Self::Upper => "upper",
            Self::Lower => "lower",
            Self::Normalise => "normalise",
            Self::CaseFold => "case-fold",
            Self::Split => "split",
            Self::Replace => "replace",
            Self::Encode => "encode",
            Self::Decode => "decode",
        }
    }

    pub(super) const fn has_children(self) -> bool {
        matches!(
            self,
            Self::Trim | Self::Contains | Self::Find | Self::Normalise | Self::Upper | Self::Lower
        )
    }

    pub(super) fn from_source_name(name: &str) -> Option<Self> {
        match name {
            "trim" => Some(Self::Trim),
            "contains" => Some(Self::Contains),
            "find" => Some(Self::Find),
            "upper" => Some(Self::Upper),
            "lower" => Some(Self::Lower),
            "normalise" => Some(Self::Normalise),
            "case-fold" => Some(Self::CaseFold),
            "split" => Some(Self::Split),
            "replace" => Some(Self::Replace),
            "encode" => Some(Self::Encode),
            "decode" => Some(Self::Decode),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StringCallSelection {
    pub receiver: Span,
    pub family: StringFamily,
    pub child: String,
}

impl std::fmt::Display for ValueType {
    #[expect(
        clippy::too_many_lines,
        reason = "the closed semantic value-type enum has one exhaustive source-name mapping"
    )]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(ty) => ty.fmt(formatter),
            Self::Optional(inner) => write!(formatter, "{inner}|none"),
            Self::OverflowResult(ty) => write!(formatter, "overflow-result of {ty}"),
            Self::DivRemResult(ty) => write!(formatter, "div-rem-result of {ty}"),
            Self::FloatDecomposition(ty) => {
                write!(formatter, "float-decomposition of {ty}")
            }
            Self::StringView(TextUnit::Bytes) => formatter.write_str("string.bytes"),
            Self::StringView(TextUnit::Scalars) => formatter.write_str("string.scalars"),
            Self::StringView(TextUnit::Graphemes) => formatter.write_str("string.graphemes"),
            Self::StringList => formatter.write_str("list of string"),
            Self::TextRange => formatter.write_str("text-range"),
            Self::TextRangeView(TextUnit::Bytes) => formatter.write_str("text-range.bytes"),
            Self::TextRangeView(TextUnit::Scalars) => formatter.write_str("text-range.scalars"),
            Self::TextRangeView(TextUnit::Graphemes) => formatter.write_str("text-range.graphemes"),
            Self::TextRangeList => formatter.write_str("list of text-range"),
            Self::Iterator(item) => write!(formatter, "iterator of {}", item.value_type()),
            Self::IterationStep(item) => {
                write!(formatter, "iteration-step of {}", item.value_type())
            }
            Self::IterationEnd => formatter.write_str("iteration-step.end"),
            Self::AsyncIterationStep(item) => {
                write!(formatter, "async-iteration-step of {}", item.value_type())
            }
            Self::AsyncSinkOutcome => formatter.write_str("async-sink-outcome"),
            Self::ChannelPair(item) => write!(formatter, "channel-pair of {}", item.value_type()),
            Self::ChannelSender(item) => {
                write!(formatter, "channel-sender of {}", item.value_type())
            }
            Self::ChannelReceiver(item) => {
                write!(formatter, "channel-receiver of {}", item.value_type())
            }
            Self::ChannelSendOutcome(item) => {
                write!(formatter, "channel-send-outcome of {}", item.value_type())
            }
            Self::ChannelReceiveOutcome(item) => {
                write!(
                    formatter,
                    "channel-receive-outcome of {}",
                    item.value_type()
                )
            }
            Self::ChannelOverflowPolicy => formatter.write_str("channel-overflow-policy"),
            Self::List(item) => write!(formatter, "list of {}", item.value_type()),
            Self::Map(key, value) => write!(formatter, "map of {key}, {value}"),
            Self::Set(item) => write!(formatter, "set of {item}"),
            Self::Tuple(item, _) => write!(formatter, "tuple of {}", item.value_type()),
            Self::Range => formatter.write_str("range"),
            Self::Entry(key, value) => write!(formatter, "entry of {key}, {value}"),
            Self::UnorderedMap(key, value) => {
                write!(formatter, "unordered-map of {key}, {value}")
            }
            Self::UnorderedSet(item) => write!(formatter, "unordered-set of {item}"),
            Self::Encoding => formatter.write_str("encoding"),
            Self::Function(parameters, result) => {
                formatter.write_str("function")?;
                if !parameters.is_empty() {
                    formatter.write_str(" from ")?;
                    for (index, parameter) in parameters.iter().enumerate() {
                        if index != 0 {
                            formatter.write_str(", ")?;
                        }
                        parameter.fmt(formatter)?;
                    }
                }
                write!(formatter, " to {result}")
            }
            Self::AsyncFunction(parameters, result, _) => {
                formatter.write_str("async function")?;
                if !parameters.is_empty() {
                    formatter.write_str(" from ")?;
                    for (index, parameter) in parameters.iter().enumerate() {
                        if index != 0 {
                            formatter.write_str(", ")?;
                        }
                        parameter.fmt(formatter)?;
                    }
                }
                write!(formatter, " to {result}")
            }
            Self::Task(result, _) => write!(formatter, "task of {result}"),
            Self::Descriptor(_) => formatter.write_str("descriptor"),
            Self::Object(identity) => identity.fmt(formatter),
            Self::ScopedTask(result, _) => write!(formatter, "scoped task of {result}"),
            Self::TaskScope => formatter.write_str("task-scope"),
            Self::TaskOutcome(result) => write!(formatter, "task-outcome of {result}"),
            Self::DocumentDecodeOutcome(value) => {
                write!(formatter, "document-decode-outcome of {value}")
            }
            Self::DocumentDiagnostic => formatter.write_str("document-diagnostic"),
            Self::FilesystemAuthority => formatter.write_str("filesystem-authority"),
            Self::PlatformFilesystemResult => formatter.write_str("platform-filesystem-result"),
            Self::PlatformStreamHandle => formatter.write_str("platform-stream-handle"),
            Self::PlatformOpenResult => formatter.write_str("platform-open-result"),
            Self::PlatformReadResult => formatter.write_str("platform-read-result"),
            Self::PlatformWriteResult => formatter.write_str("platform-write-result"),
            Self::PlatformUnitResult => formatter.write_str("platform-unit-result"),
            Self::PlatformDataResult => formatter.write_str("platform-data-result"),
            Self::PlatformUrlResult => formatter.write_str("platform-url-result"),
            Self::PlatformCapability => formatter.write_str("platform-capability"),
            Self::PlatformResourceHandle => formatter.write_str("platform-resource-handle"),
            Self::PlatformResult => formatter.write_str("platform-result"),
            Self::Reference(item) => write!(formatter, "ref {}", item.value_type()),
            Self::SharedReference(item) => write!(formatter, "shared ref {}", item.value_type()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticFamily {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    DivRem,
    Negate,
    ShiftLeft,
    ShiftRight,
}

impl ArithmeticFamily {
    pub(crate) const fn source_name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Subtract => "subtract",
            Self::Multiply => "multiply",
            Self::Divide => "divide",
            Self::Remainder => "remainder",
            Self::DivRem => "div-rem",
            Self::Negate => "negate",
            Self::ShiftLeft => "shift-left",
            Self::ShiftRight => "shift-right",
        }
    }

    pub(super) fn from_source_name(name: &str) -> Option<Self> {
        match name {
            "add" => Some(Self::Add),
            "subtract" => Some(Self::Subtract),
            "multiply" => Some(Self::Multiply),
            "divide" => Some(Self::Divide),
            "remainder" => Some(Self::Remainder),
            "div-rem" => Some(Self::DivRem),
            "negate" => Some(Self::Negate),
            "shift-left" => Some(Self::ShiftLeft),
            "shift-right" => Some(Self::ShiftRight),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemberFamily {
    Coerce,
    Parse,
    Radix,
    Arithmetic(ArithmeticFamily),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FloatMemberOperation {
    Finite,
    Infinite,
    NotANumber,
    NegativeSign,
    Zero,
    Normal,
    Subnormal,
    SquareRoot,
    CubeRoot,
    Hypotenuse,
    Power,
    IntegerPower,
    Sine,
    Cosine,
    SineCosine,
    Tangent,
    ArcSine,
    ArcCosine,
    ArcTangent,
    ArcTangentTwo,
    NaturalLog,
    Exponential,
    BinaryExponential,
    ExponentialMinusOne,
    NaturalLogOnePlus,
    BinaryLog,
    DecimalLog,
    Logarithm,
    Absolute,
    CopySign,
    Clamp,
    FractionalPart,
    NextUp,
    NextDown,
    Decompose,
    ScaleBinary,
    Round,
    Floor,
    Ceiling,
    Truncate,
    Minimum,
    Maximum,
    MultiplyAdd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FloatMemberArgument {
    Receiver,
    Int32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FloatMemberResult {
    Receiver,
    Integer,
    Boolean,
    ReceiverPair,
    Decomposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FloatMemberContract {
    pub operation: FloatMemberOperation,
    pub parameters: Option<&'static [FloatMemberArgument]>,
    pub result: FloatMemberResult,
}

pub(crate) fn float_member_contract(name: &str) -> Option<FloatMemberContract> {
    use FloatMemberArgument::{Int32, Receiver};
    use FloatMemberOperation as Operation;
    use FloatMemberResult as Result;

    let operation = match name {
        "finite" => Operation::Finite,
        "infinite" => Operation::Infinite,
        "not-a-number" => Operation::NotANumber,
        "negative-sign" => Operation::NegativeSign,
        "zero" => Operation::Zero,
        "normal" => Operation::Normal,
        "subnormal" => Operation::Subnormal,
        "square-root" => Operation::SquareRoot,
        "cube-root" => Operation::CubeRoot,
        "hypotenuse" => Operation::Hypotenuse,
        "power" => Operation::Power,
        "integer-power" => Operation::IntegerPower,
        "sine" => Operation::Sine,
        "cosine" => Operation::Cosine,
        "sine-cosine" => Operation::SineCosine,
        "tangent" => Operation::Tangent,
        "arc-sine" => Operation::ArcSine,
        "arc-cosine" => Operation::ArcCosine,
        "arc-tangent" => Operation::ArcTangent,
        "arc-tangent-two" => Operation::ArcTangentTwo,
        "natural-log" => Operation::NaturalLog,
        "exponential" => Operation::Exponential,
        "binary-exponential" => Operation::BinaryExponential,
        "exponential-minus-one" => Operation::ExponentialMinusOne,
        "natural-log-one-plus" => Operation::NaturalLogOnePlus,
        "binary-log" => Operation::BinaryLog,
        "decimal-log" => Operation::DecimalLog,
        "logarithm" => Operation::Logarithm,
        "absolute" => Operation::Absolute,
        "copy-sign" => Operation::CopySign,
        "clamp" => Operation::Clamp,
        "fractional-part" => Operation::FractionalPart,
        "next-up" => Operation::NextUp,
        "next-down" => Operation::NextDown,
        "decompose" => Operation::Decompose,
        "scale-binary" => Operation::ScaleBinary,
        "round" => Operation::Round,
        "floor" => Operation::Floor,
        "ceiling" => Operation::Ceiling,
        "truncate" => Operation::Truncate,
        "minimum" => Operation::Minimum,
        "maximum" => Operation::Maximum,
        "multiply-add" => Operation::MultiplyAdd,
        _ => return None,
    };
    let (parameters, result): (Option<&'static [_]>, _) = match operation {
        Operation::Finite
        | Operation::Infinite
        | Operation::NotANumber
        | Operation::NegativeSign
        | Operation::Zero
        | Operation::Normal
        | Operation::Subnormal => (None, Result::Boolean),
        Operation::SineCosine => (Some(&[]), Result::ReceiverPair),
        Operation::Decompose => (Some(&[]), Result::Decomposition),
        Operation::Round | Operation::Floor | Operation::Ceiling | Operation::Truncate => {
            (Some(&[]), Result::Integer)
        }
        Operation::Hypotenuse
        | Operation::Power
        | Operation::ArcTangentTwo
        | Operation::Logarithm
        | Operation::CopySign
        | Operation::Minimum
        | Operation::Maximum => (Some(&[Receiver]), Result::Receiver),
        Operation::Clamp | Operation::MultiplyAdd => {
            (Some(&[Receiver, Receiver]), Result::Receiver)
        }
        Operation::IntegerPower | Operation::ScaleBinary => (Some(&[Int32]), Result::Receiver),
        Operation::SquareRoot
        | Operation::CubeRoot
        | Operation::Sine
        | Operation::Cosine
        | Operation::Tangent
        | Operation::ArcSine
        | Operation::ArcCosine
        | Operation::ArcTangent
        | Operation::NaturalLog
        | Operation::Exponential
        | Operation::BinaryExponential
        | Operation::ExponentialMinusOne
        | Operation::NaturalLogOnePlus
        | Operation::BinaryLog
        | Operation::DecimalLog
        | Operation::Absolute
        | Operation::FractionalPart
        | Operation::NextUp
        | Operation::NextDown => (Some(&[]), Result::Receiver),
    };
    Some(FloatMemberContract {
        operation,
        parameters,
        result,
    })
}

impl FloatMemberContract {
    pub(super) fn result_type(self, receiver: ScalarType) -> ValueType {
        match self.result {
            FloatMemberResult::Receiver => ValueType::Scalar(receiver),
            FloatMemberResult::Integer => ValueType::Scalar(ScalarType::Int),
            FloatMemberResult::Boolean => ValueType::Scalar(ScalarType::Bool),
            FloatMemberResult::ReceiverPair => {
                ValueType::Tuple(ElementType::new(ValueType::Scalar(receiver)), Some(2))
            }
            FloatMemberResult::Decomposition => ValueType::FloatDecomposition(receiver),
        }
    }

    pub(super) fn member_type(self, receiver: ScalarType) -> ValueType {
        let result = self.result_type(receiver);
        self.parameters.map_or(result.clone(), |parameters| {
            ValueType::Function(
                parameters
                    .iter()
                    .map(|parameter| {
                        ElementType::new(ValueType::Scalar(match parameter {
                            FloatMemberArgument::Receiver => receiver,
                            FloatMemberArgument::Int32 => ScalarType::Int32,
                        }))
                    })
                    .collect(),
                ElementType::new(result),
            )
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundMethod {
    pub receiver: Span,
    pub family: MemberFamily,
    pub child: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CoercionPolicy {
    Default,
    Checked,
    Wrap,
    Saturate,
}

impl CoercionPolicy {
    pub(crate) fn from_member(member: &str) -> Option<Self> {
        match member {
            "checked" => Some(Self::Checked),
            "wrap" => Some(Self::Wrap),
            "saturate" => Some(Self::Saturate),
            _ => None,
        }
    }

    pub(super) fn source_name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Checked => "checked",
            Self::Wrap => "wrap",
            Self::Saturate => "saturate",
        }
    }

    pub(super) fn invocation_name(self) -> &'static str {
        match self {
            Self::Default => ".coerce",
            Self::Checked => ".coerce.checked",
            Self::Wrap => ".coerce.wrap",
            Self::Saturate => ".coerce.saturate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedBinding {
    pub name: String,
    pub span: Span,
    pub visible_from: usize,
    pub scope: Option<Span>,
    pub value_type: ValueType,
    pub destination_arms: Vec<ScalarType>,
    pub storage_type: Option<ScalarType>,
    pub mutable: bool,
}

impl TypedBinding {
    pub(crate) fn is_visible_at(&self, file: u32, position: usize) -> bool {
        self.span.file == file
            && self.visible_from <= position
            && self.scope.is_none_or(|scope| position <= scope.end)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DescriptorAlias {
    pub(super) visible_from: usize,
    pub(super) scope: Option<Span>,
    pub(super) identity: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectKind {
    Class,
    Interface,
    Trait,
    Type,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BuiltinDescriptor {
    Value,
    StringView,
    Scalar(ScalarType),
    Category(TypeCategory),
    Encoding,
    OverflowResult,
    DivRemResult,
    FloatDecomposition,
    Iterator,
    IterationStep,
    List,
    ReadonlyList,
    Map,
    Set,
    Tuple,
    Range,
    Entry,
    UnorderedMap,
    UnorderedSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectFieldMetadata {
    pub external_name: String,
    pub defaulted: bool,
    pub optional: bool,
    pub secret: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectField {
    pub name: String,
    pub span: Span,
    pub value_type: ValueType,
    pub initializer_span: Option<Span>,
    pub is_static: bool,
    pub metadata: ObjectFieldMetadata,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptorContract {
    /// Name visible in this unit; imported contracts carry their local alias here.
    pub name: String,
    /// Stable declaration identity used for semantic equality and cross-unit lookup.
    pub identity: ObjectIdentity,
    pub span: Span,
    pub kind: ObjectKind,
    pub resource_owning: bool,
    /// Compiler-owned built-in template represented by this same canonical contract.
    pub(crate) builtin: Option<BuiltinDescriptor>,
    /// Nominal category contracts implemented by values of this descriptor.
    pub(crate) categories: Vec<TypeCategory>,
    /// Stable operation identifier selected for each canonical instance-member path.
    pub(crate) operations: BTreeMap<String, String>,
    /// Canonical instance-member paths, including family children such as `trim.start`.
    pub(crate) members: BTreeSet<String>,
    /// Canonical subset of `members` that denotes invocable instance members.
    pub(crate) methods: BTreeSet<String>,
    /// Methods whose family selections cannot yet be stored as bound values.
    pub(crate) invocation_only_methods: BTreeSet<String>,
    /// Canonical static-member names declared by this descriptor.
    pub(crate) static_members: BTreeSet<String>,
    /// Canonical subset of `static_members` that denotes invocable members.
    pub(crate) static_methods: BTreeSet<String>,
    pub base: Option<ObjectIdentity>,
    pub interfaces: Vec<ObjectIdentity>,
    pub traits: Vec<ObjectIdentity>,
    pub fields: Vec<ObjectField>,
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "callable contracts retain independent semantic properties"
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionContract {
    pub name: String,
    pub span: Span,
    pub owner: Option<String>,
    /// Canonical owner identity for methods; aliases never rewrite it.
    pub(crate) owner_identity: Option<ObjectIdentity>,
    pub captures: Vec<String>,
    pub parameters: Vec<ParameterContract>,
    pub return_type: Option<ValueType>,
    pub exported: bool,
    pub thrown_types: Vec<ValueType>,
    pub escaping_throwables: BTreeSet<String>,
    pub throws: bool,
    pub is_async: bool,
    pub task_transferability: TaskTransferability,
    pub is_static: bool,
    pub mutates_receiver: bool,
    pub consumes_receiver: bool,
    pub(crate) execution_requirements: crate::execution::ExecutionRequirements,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterContract {
    pub name: String,
    pub span: Span,
    pub value_type: Option<ValueType>,
    pub optional: bool,
    pub mutable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationKind {
    Call,
    ShortCircuitRhs,
    PostfixUpdate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvaluationStep {
    pub kind: EvaluationKind,
    pub span: Span,
    pub conditional: bool,
}

#[derive(Clone, Debug)]
pub struct SemanticFailure {
    pub source: SourceFile,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ContextualConstant {
    Integer(BigInt),
    Float32(f32),
    Float64(f64),
}

#[derive(Clone, Debug)]
pub struct SemanticUnit {
    pub source: SourceFile,
    pub source_path: String,
    pub tree: SyntaxTree,
    pub namespace: String,
    pub(super) prelude: bool,
    pub(crate) bundled: bool,
    pub scopes: Vec<LexicalScope>,
    pub typed_bindings: Vec<TypedBinding>,
    /// Proven owners and projections for non-owning reference expressions and bindings.
    pub reference_provenance: BTreeMap<(usize, usize), ReferenceProvenance>,
    /// Reference-returning callable contract to the exact lender parameter index.
    pub reference_return_lenders: BTreeMap<(u32, usize, usize), usize>,
    /// Function contracts declared by every source unit in this unit's namespace.
    pub functions: Vec<FunctionContract>,
    pub descriptors: Vec<DescriptorContract>,
    pub(crate) builtin_descriptors: std::sync::Arc<[DescriptorContract]>,
    pub(super) comparable_foreign_objects: BTreeSet<ObjectIdentity>,
    pub(super) function_aliases: BTreeMap<String, FunctionContract>,
    pub(super) function_contracts_by_span: BTreeMap<(u32, usize, usize), FunctionContract>,
    pub(super) enclosing_function_spans: BTreeMap<usize, Option<Span>>,
    pub(super) descriptor_aliases: BTreeMap<String, Vec<DescriptorAlias>>,
    pub(super) projected_removals: Vec<crate::projection::RemovedItem>,
    pub unreachable_spans: Vec<Span>,
    pub evaluation_steps: Vec<EvaluationStep>,
}

impl SemanticUnit {
    /// Returns the compiler-resolved value type for an expression when it is statically known.
    pub(crate) fn inferred_value_type(&self, node: &SyntaxNode) -> Option<ValueType> {
        infer_value_type(self, node, &self.typed_bindings)
            .ok()
            .flatten()
    }

    pub(crate) fn function_contract_at(&self, node: &SyntaxNode) -> Option<&FunctionContract> {
        self.function_contracts_by_span
            .get(&(node.span.file, node.span.start, node.span.end))
    }

    pub(super) fn descriptor_alias_identity(&self, name: &str, position: usize) -> Option<&str> {
        self.descriptor_aliases.get(name).and_then(|history| {
            history
                .iter()
                .rev()
                .find(|alias| alias.is_visible_at(self.source.id(), position))
                .map(|alias| alias.identity.as_str())
        })
    }

    pub(super) fn descriptor_alias_at(&self, name: &str, position: usize) -> Option<ScalarType> {
        self.descriptor_alias_identity(name, position)?
            .strip_prefix("/core/types::")
            .and_then(ScalarType::from_source_name)
    }
    pub(super) fn removed_projected_member(
        &self,
        identity: &ObjectIdentity,
        member: &str,
        is_static: bool,
    ) -> Option<&crate::projection::RemovedItem> {
        let separator = if is_static { "::" } else { "." };
        let name = format!("{}{separator}{member}", identity.name);
        self.projected_removals
            .iter()
            .find(|removed| removed.namespace == identity.namespace && removed.name == name)
    }
}

impl DescriptorAlias {
    pub(super) fn is_visible_at(&self, file: u32, position: usize) -> bool {
        self.visible_from <= position
            && self.scope.is_none_or(|scope| {
                scope.file == file && scope.start <= position && position <= scope.end
            })
    }
}

pub(super) fn visible_descriptor_aliases(
    aliases: &BTreeMap<String, Vec<DescriptorAlias>>,
    file: u32,
    position: usize,
) -> BTreeMap<String, ScalarType> {
    aliases
        .iter()
        .filter_map(|(name, history)| {
            let alias = history
                .iter()
                .rev()
                .find(|alias| alias.is_visible_at(file, position))?;
            let scalar = alias
                .identity
                .strip_prefix("/core/types::")
                .and_then(ScalarType::from_source_name)?;
            Some((name.clone(), scalar))
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct LexicalScope {
    pub span: Span,
    pub parent: Option<usize>,
    pub symbols: BTreeMap<String, Vec<Symbol>>,
    pub(super) import_warnings: Vec<Diagnostic>,
}

#[derive(Clone)]
pub(super) struct Import {
    pub(super) source: SourceFile,
    pub(super) bundled: bool,
    pub(super) namespace: String,
    pub(super) target: String,
    pub(super) namespace_wide: bool,
    pub(super) object: String,
    pub(super) alias: String,
    pub(super) span: Span,
}

pub(super) fn is_function_node(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    )
}

pub(super) fn object_name_containing(unit: &SemanticUnit, span: Span) -> Option<String> {
    unit.tree.root.children.iter().find_map(|object| {
        matches!(
            object.kind,
            SyntaxKind::ClassDeclaration
                | SyntaxKind::InterfaceDeclaration
                | SyntaxKind::TraitDeclaration
        )
        .then_some(object)
        .filter(|object| object.span.start <= span.start && span.end <= object.span.end)
        .and_then(|object| declaration_name(object, &unit.source))
    })
}

pub(super) fn implicit_receiver_span(node: &SyntaxNode, name: &str) -> Span {
    let offset = node.span.start + usize::from(name == "this");
    Span {
        file: node.span.file,
        start: offset,
        end: offset,
    }
}

pub(super) fn index_enclosing_function_spans(root: &SyntaxNode) -> BTreeMap<usize, Option<Span>> {
    fn visit(
        node: &SyntaxNode,
        enclosing_function: Option<Span>,
        spans: &mut BTreeMap<usize, Option<Span>>,
    ) {
        let enclosing_function = is_function_node(node)
            .then_some(node.span)
            .or(enclosing_function);
        spans.insert(node.span.start, enclosing_function);
        for child in &node.children {
            visit(child, enclosing_function, spans);
        }
    }

    let mut spans = BTreeMap::new();
    visit(root, None, &mut spans);
    spans
}
