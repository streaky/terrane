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

    /// Returns the compiler-owned scalar represented by this canonical type descriptor.
    #[must_use]
    pub fn descriptor_type(&self) -> Option<ScalarType> {
        (self.kind == SymbolKind::TypeDescriptor)
            .then(|| self.identity.strip_prefix("/core/types::"))
            .flatten()
            .and_then(ScalarType::from_source_name)
    }

    #[must_use]
    pub fn descriptor_category(&self) -> Option<TypeCategory> {
        (self.kind == SymbolKind::TypeDescriptor)
            .then(|| self.identity.strip_prefix("/core/types::"))
            .flatten()
            .and_then(TypeCategory::from_source_name)
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
    pub profile: crate::package::CapabilityProfile,
    pub projection: crate::projection::Projection,
    pub namespaces: BTreeMap<String, Namespace>,
    pub globals: BTreeMap<String, Symbol>,
    pub prelude_bindings: BTreeMap<String, Symbol>,
    pub descriptor_constructs: BTreeMap<String, Symbol>,
    pub units: Vec<SemanticUnit>,
    pub(super) binding_events: BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
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
pub(super) fn iterable_item_type(value_type: ValueType) -> Option<ValueType> {
    match value_type {
        ValueType::Scalar(ScalarType::String) | ValueType::StringList => {
            Some(ValueType::Scalar(ScalarType::String))
        }
        ValueType::Scalar(ScalarType::Bytes) => Some(ValueType::Scalar(ScalarType::Uint8)),
        ValueType::Iterator(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::UnorderedSet(item)
        | ValueType::Tuple(item, _) => Some(item.value_type()),
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value) => {
            Some(ValueType::Entry(key, value))
        }
        ValueType::Range => Some(ValueType::Scalar(ScalarType::Int)),
        _ => None,
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

    pub(super) fn qualified(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl std::fmt::Display for ObjectIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValueType {
    Scalar(ScalarType),
    Optional(Box<ValueType>),
    OverflowResult(ScalarType),
    DivRemResult(ScalarType),
    StringView(TextUnit),
    StringList,
    TextRange,
    TextRangeView(TextUnit),
    TextRangeList,
    Iterator(ElementType),
    IterationStep(ElementType),
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
    AsyncFunction(Vec<ElementType>, ElementType),
    Descriptor(String),
    Task(ElementType),
    ScopedTask(ElementType),
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(ty) => ty.fmt(formatter),
            Self::Optional(inner) => write!(formatter, "{inner}|none"),
            Self::OverflowResult(ty) => write!(formatter, "overflow-result of {ty}"),
            Self::DivRemResult(ty) => write!(formatter, "div-rem-result of {ty}"),
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
            Self::AsyncFunction(parameters, result) => {
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
            Self::Task(result) => write!(formatter, "task of {result}"),
            Self::Descriptor(_) => formatter.write_str("descriptor"),
            Self::Object(identity) => identity.fmt(formatter),
            Self::ScopedTask(result) => write!(formatter, "scoped task of {result}"),
            Self::TaskScope => formatter.write_str("task-scope"),
            Self::TaskOutcome(result) => write!(formatter, "task-outcome of {result}"),
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
    SquareRoot,
    Sine,
    Cosine,
    SineCosine,
    NaturalLog,
    Exponential,
    Absolute,
    Round,
    Floor,
    Ceiling,
    Truncate,
    Minimum,
    Maximum,
    MultiplyAdd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FloatMemberResult {
    Receiver,
    Integer,
    Boolean,
    ReceiverPair,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FloatMemberContract {
    pub operation: FloatMemberOperation,
    pub arity: Option<usize>,
    pub result: FloatMemberResult,
}

pub(crate) fn float_member_contract(name: &str) -> Option<FloatMemberContract> {
    let operation = match name {
        "finite" => FloatMemberOperation::Finite,
        "infinite" => FloatMemberOperation::Infinite,
        "not-a-number" => FloatMemberOperation::NotANumber,
        "square-root" => FloatMemberOperation::SquareRoot,
        "sine" => FloatMemberOperation::Sine,
        "cosine" => FloatMemberOperation::Cosine,
        "sine-cosine" => FloatMemberOperation::SineCosine,
        "natural-log" => FloatMemberOperation::NaturalLog,
        "exponential" => FloatMemberOperation::Exponential,
        "absolute" => FloatMemberOperation::Absolute,
        "round" => FloatMemberOperation::Round,
        "floor" => FloatMemberOperation::Floor,
        "ceiling" => FloatMemberOperation::Ceiling,
        "truncate" => FloatMemberOperation::Truncate,
        "minimum" => FloatMemberOperation::Minimum,
        "maximum" => FloatMemberOperation::Maximum,
        "multiply-add" => FloatMemberOperation::MultiplyAdd,
        _ => return None,
    };
    let (arity, result) = match operation {
        FloatMemberOperation::Finite
        | FloatMemberOperation::Infinite
        | FloatMemberOperation::NotANumber => (None, FloatMemberResult::Boolean),
        FloatMemberOperation::SineCosine => (Some(0), FloatMemberResult::ReceiverPair),
        FloatMemberOperation::Round
        | FloatMemberOperation::Floor
        | FloatMemberOperation::Ceiling
        | FloatMemberOperation::Truncate => (Some(0), FloatMemberResult::Integer),
        FloatMemberOperation::Minimum | FloatMemberOperation::Maximum => {
            (Some(1), FloatMemberResult::Receiver)
        }
        FloatMemberOperation::MultiplyAdd => (Some(2), FloatMemberResult::Receiver),
        FloatMemberOperation::SquareRoot
        | FloatMemberOperation::Sine
        | FloatMemberOperation::Cosine
        | FloatMemberOperation::NaturalLog
        | FloatMemberOperation::Exponential
        | FloatMemberOperation::Absolute => (Some(0), FloatMemberResult::Receiver),
    };
    Some(FloatMemberContract {
        operation,
        arity,
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
        }
    }

    pub(super) fn member_type(self, receiver: ScalarType) -> ValueType {
        let result = self.result_type(receiver);
        self.arity.map_or(result.clone(), |arity| {
            ValueType::Function(
                vec![ElementType::new(ValueType::Scalar(receiver)); arity],
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DescriptorAlias {
    pub(super) visible_from: usize,
    pub(super) scope: Option<Span>,
    pub(super) value_type: ScalarType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectKind {
    Class,
    Interface,
    Trait,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectField {
    pub name: String,
    pub span: Span,
    pub value_type: ValueType,
    pub is_static: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectContract {
    /// Name visible in this unit; imported contracts carry their local alias here.
    pub name: String,
    /// Stable declaration identity used for semantic equality and cross-unit lookup.
    pub identity: ObjectIdentity,
    pub span: Span,
    pub kind: ObjectKind,
    pub resource_owning: bool,
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
    pub captures: Vec<String>,
    pub parameters: Vec<ParameterContract>,
    pub return_type: Option<ValueType>,
    pub exported: bool,
    pub thrown_types: Vec<ValueType>,
    pub escaping_throwables: BTreeSet<String>,
    pub throws: bool,
    pub is_async: bool,
    pub is_static: bool,
    pub mutates_receiver: bool,
    pub consumes_receiver: bool,
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
    /// Function contracts declared by every source unit in this unit's namespace.
    pub functions: Vec<FunctionContract>,
    pub objects: Vec<ObjectContract>,
    pub(super) comparable_foreign_objects: BTreeSet<ObjectIdentity>,
    pub(super) function_aliases: BTreeMap<String, FunctionContract>,
    pub(super) function_contracts_by_span: BTreeMap<(u32, usize, usize), FunctionContract>,
    pub(super) enclosing_function_spans: BTreeMap<usize, Option<Span>>,
    pub(super) descriptor_aliases: BTreeMap<String, Vec<DescriptorAlias>>,
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

    pub(super) fn descriptor_alias_at(&self, name: &str, position: usize) -> Option<ScalarType> {
        self.descriptor_aliases.get(name).and_then(|history| {
            history
                .iter()
                .rev()
                .find(|alias| alias.is_visible_at(self.source.id(), position))
                .map(|alias| alias.value_type)
        })
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
            history
                .iter()
                .rev()
                .find(|alias| alias.is_visible_at(file, position))
                .map(|alias| (name.clone(), alias.value_type))
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
