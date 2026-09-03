use std::collections::{BTreeMap, BTreeSet};

use num_bigint::BigInt;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxTree};
use crate::{Diagnostic, Package, ScalarType, SourceFile, Span, TypeCategory, lexer, parser};

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
    binding_events: BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
    import_warnings: Vec<Diagnostic>,
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

    fn scalar(&self) -> Option<ScalarType> {
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
fn iterable_item_type(value_type: ValueType) -> Option<ValueType> {
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

fn iteration_target_bindings(
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
    fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    fn qualified(&self) -> String {
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

    const fn has_children(self) -> bool {
        matches!(
            self,
            Self::Trim | Self::Contains | Self::Find | Self::Normalise | Self::Upper | Self::Lower
        )
    }

    fn from_source_name(name: &str) -> Option<Self> {
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

    fn from_source_name(name: &str) -> Option<Self> {
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
    fn result_type(self, receiver: ScalarType) -> ValueType {
        match self.result {
            FloatMemberResult::Receiver => ValueType::Scalar(receiver),
            FloatMemberResult::Integer => ValueType::Scalar(ScalarType::Int),
            FloatMemberResult::Boolean => ValueType::Scalar(ScalarType::Bool),
            FloatMemberResult::ReceiverPair => {
                ValueType::Tuple(ElementType::new(ValueType::Scalar(receiver)), Some(2))
            }
        }
    }

    fn member_type(self, receiver: ScalarType) -> ValueType {
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

    fn source_name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Checked => "checked",
            Self::Wrap => "wrap",
            Self::Saturate => "saturate",
        }
    }

    fn invocation_name(self) -> &'static str {
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
struct DescriptorAlias {
    visible_from: usize,
    scope: Option<Span>,
    value_type: ScalarType,
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
    prelude: bool,
    pub(crate) bundled: bool,
    pub scopes: Vec<LexicalScope>,
    pub typed_bindings: Vec<TypedBinding>,
    /// Function contracts declared by every source unit in this unit's namespace.
    pub functions: Vec<FunctionContract>,
    pub objects: Vec<ObjectContract>,
    comparable_foreign_objects: BTreeSet<ObjectIdentity>,
    function_aliases: BTreeMap<String, FunctionContract>,
    function_contracts_by_span: BTreeMap<(u32, usize, usize), FunctionContract>,
    enclosing_function_spans: BTreeMap<usize, Option<Span>>,
    descriptor_aliases: BTreeMap<String, Vec<DescriptorAlias>>,
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

    fn descriptor_alias_at(&self, name: &str, position: usize) -> Option<ScalarType> {
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
    fn is_visible_at(&self, file: u32, position: usize) -> bool {
        self.visible_from <= position
            && self.scope.is_none_or(|scope| {
                scope.file == file && scope.start <= position && position <= scope.end
            })
    }
}

fn visible_descriptor_aliases(
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
    import_warnings: Vec<Diagnostic>,
}

#[derive(Clone)]
struct Import {
    source: SourceFile,
    bundled: bool,
    namespace: String,
    target: String,
    namespace_wide: bool,
    object: String,
    alias: String,
    span: Span,
}

fn is_function_node(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    )
}

fn object_name_containing(unit: &SemanticUnit, span: Span) -> Option<String> {
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

fn implicit_receiver_span(node: &SyntaxNode, name: &str) -> Span {
    let offset = node.span.start + usize::from(name == "this");
    Span {
        file: node.span.file,
        start: offset,
        end: offset,
    }
}

fn index_enclosing_function_spans(root: &SyntaxNode) -> BTreeMap<usize, Option<Span>> {
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

fn parse_unit(
    source: &SourceFile,
    source_path: String,
    expected_namespace: Option<&str>,
    prelude: bool,
    bundled: bool,
) -> Result<SemanticUnit, SemanticFailure> {
    let lexed = lexer::lex(source).map_err(|diagnostics| SemanticFailure {
        source: source.clone(),
        diagnostics,
    })?;
    let parsed = parser::parse(source, lexed);
    if !parsed.diagnostics.is_empty() {
        return Err(SemanticFailure {
            source: source.clone(),
            diagnostics: parsed.diagnostics,
        });
    }
    let namespace =
        declared_namespace(source, &parsed.tree).map_err(|diagnostic| SemanticFailure {
            source: source.clone(),
            diagnostics: vec![diagnostic],
        })?;
    if let Some(expected) = expected_namespace
        && namespace != expected
    {
        let span = parsed
            .tree
            .root
            .children
            .iter()
            .find(|node| node.kind == SyntaxKind::NamespaceDeclaration)
            .map_or(Span::new(source.id(), 0, source.text().len()), |node| {
                node.span
            });
        let diagnostic = Diagnostic::error(
            "S2020",
            format!(
                "declared namespace `{namespace}` does not match `{expected}` required by its source directory"
            ),
            span,
        )
        .with_help(format!("declare `namespace {}`", expected.trim_start_matches('/')));
        return Err(SemanticFailure {
            source: source.clone(),
            diagnostics: vec![diagnostic],
        });
    }
    let enclosing_function_spans = index_enclosing_function_spans(&parsed.tree.root);
    Ok(SemanticUnit {
        source: source.clone(),
        source_path,
        tree: parsed.tree,
        namespace,
        prelude,
        bundled,
        scopes: Vec::new(),
        typed_bindings: Vec::new(),
        functions: Vec::new(),
        objects: Vec::new(),
        comparable_foreign_objects: BTreeSet::new(),
        function_aliases: BTreeMap::new(),
        function_contracts_by_span: BTreeMap::new(),
        descriptor_aliases: BTreeMap::new(),
        enclosing_function_spans,
        unreachable_spans: Vec::new(),
        evaluation_steps: Vec::new(),
    })
}

fn parse_units(
    package: &Package,
    projection: &crate::projection::Projection,
) -> Result<Vec<SemanticUnit>, SemanticFailure> {
    let mut units = package
        .units
        .iter()
        .map(|unit| {
            parse_unit(
                &unit.source,
                unit.relative_path_text(),
                unit.expected_namespace.as_deref(),
                package.prelude,
                false,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut loaded = units
        .iter()
        .map(|unit| unit.namespace.clone())
        .collect::<BTreeSet<_>>();
    let mut next_source_id = units
        .iter()
        .map(|unit| unit.source.id())
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    let mut dependency_imports = BTreeMap::<String, BTreeSet<String>>::new();
    for unit in &units {
        for import in imports_in_tree(unit)?
            .into_iter()
            .filter(|import| import.target.starts_with("/deps/"))
        {
            dependency_imports
                .entry(import.target)
                .or_default()
                .insert(import.object);
        }
    }
    for (namespace, text) in projection.source_for_imports(&dependency_imports) {
        if !loaded.insert(namespace.clone()) {
            continue;
        }
        let path = format!(
            "<terrane>/projected/{}.trn",
            namespace.trim_start_matches('/')
        );
        let source = SourceFile::new(next_source_id, path.clone().into(), text);
        next_source_id = next_source_id.saturating_add(1);
        units.push(parse_unit(
            &source,
            path,
            Some(&namespace),
            package.prelude,
            true,
        )?);
    }
    let mut index = 0;
    while index < units.len() {
        let targets = imports_in_tree(&units[index])?
            .into_iter()
            .map(|import| import.target)
            .collect::<BTreeSet<_>>();
        for target in targets {
            let Some(bundled) = crate::bundled::source(&target) else {
                continue;
            };
            if !loaded.insert(target) {
                continue;
            }
            let source = SourceFile::new(
                next_source_id,
                std::path::PathBuf::from(format!("<terrane>/{}", bundled.path)),
                bundled.text.to_owned(),
            );
            next_source_id = next_source_id.saturating_add(1);
            units.push(parse_unit(
                &source,
                bundled.path.to_owned(),
                Some(bundled.namespace),
                package.prelude,
                true,
            )?);
        }
        index += 1;
    }
    apply_projected_method_contracts(&mut units, projection);
    Ok(units)
}
fn apply_projected_method_contracts(
    units: &mut [SemanticUnit],
    projection: &crate::projection::Projection,
) {
    for unit in units {
        if !unit.namespace.starts_with("/deps/") {
            continue;
        }
        for contract in &mut unit.functions {
            let Some(owner) = contract.owner.as_deref() else {
                continue;
            };
            let Some(method) = projection.method(&unit.namespace, owner, &contract.name) else {
                continue;
            };
            contract.throws = true;
            contract.mutates_receiver = matches!(
                method.receiver,
                Some(crate::projection::Receiver::MutableBorrow)
            );
            contract.consumes_receiver =
                matches!(method.receiver, Some(crate::projection::Receiver::Move));
        }
    }
}

fn dependency_projection(
    package: &Package,
) -> Result<crate::projection::Projection, SemanticFailure> {
    crate::projection::resolve(&package.root, &package.rust_dependencies).map_err(|error| {
        failure(
            &package.units[0].source,
            "S2028",
            error.message,
            Span::new(package.units[0].source.id(), 0, 0),
        )
    })
}

/// Builds the complete namespace tree, then resolves declarations and imports.
///
/// Semantic phases fail at the first diagnostic in deterministic package and source
/// order. Unlike independently discoverable manifest errors, later semantic errors can
/// depend on declarations or imports that an earlier error prevented from assembling.
///
/// # Errors
/// Returns the first source-oriented lexer, parser, namespace, scope, or import failure.
#[expect(
    clippy::too_many_lines,
    reason = "semantic phase orchestration remains linear and order-sensitive"
)]
pub fn analyze(package: &Package) -> Result<SemanticPackage, SemanticFailure> {
    let projection = dependency_projection(package)?;
    let mut units = parse_units(package, &projection)?;
    for unit in &mut units {
        unit.comparable_foreign_objects = projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| {
                matches!(
                    item.kind,
                    crate::projection::ProjectedKind::Enum {
                        data_carrying: false,
                        comparable: true,
                    }
                )
            })
            .map(|item| ObjectIdentity::new(&item.namespace, &item.name))
            .collect();
    }
    validate_compiler_owned_names(&units)?;

    let mut namespaces = bootstrap_namespaces();
    for unit in &units {
        let bundled = unit.bundled;
        if !bundled
            && (unit.namespace == "/core"
                || unit.namespace.starts_with("/core/")
                || unit.namespace == "/deps"
                || unit.namespace.starts_with("/deps/")
                || crate::bundled::source(&unit.namespace).is_some())
        {
            let span = unit
                .tree
                .root
                .children
                .iter()
                .find(|node| node.kind == SyntaxKind::NamespaceDeclaration)
                .map_or(Span::new(unit.source.id(), 0, 0), |node| node.span);
            return Err(failure(
                &unit.source,
                "S2017",
                format!(
                    "cannot declare into compiler-owned namespace `{}`",
                    unit.namespace
                ),
                span,
            ));
        }
        namespaces.entry(unit.namespace.clone()).or_default();
    }

    let mut imports = Vec::new();
    let mut globals = BTreeMap::<String, Symbol>::new();
    for unit in &units {
        collect_unit(unit, &mut namespaces, &mut globals, &mut imports)?;
    }
    let discovered_imports = units
        .iter()
        .map(imports_in_tree)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    for import in discovered_imports.iter().filter(|import| !import.bundled) {
        for capability in namespace_capabilities(&import.target) {
            if !package.profile.allows(capability) {
                return Err(failure(
                    &import.source,
                    "S2032",
                    format!(
                        "profile `{}` forbids capability `{capability}` required by `{}` imported by `{}`",
                        package.profile.name, import.target, import.namespace
                    ),
                    import.span,
                ));
            }
        }
    }
    for import in &discovered_imports {
        let Some(dependency) = import
            .target
            .strip_prefix("/deps/")
            .and_then(|path| path.split('/').next())
        else {
            continue;
        };
        if !package
            .rust_dependencies
            .iter()
            .any(|declared| declared.name.replace('_', "-") == dependency)
        {
            return Err(failure(
                &import.source,
                "S2027",
                format!("Rust dependency `{dependency}` is not declared in `package.toml`"),
                import.span,
            ));
        }
        if projection.item(&import.target, &import.object).is_none() {
            if let Some(removed) = projection
                .removed
                .iter()
                .find(|removed| removed.namespace == import.target && removed.name == import.object)
            {
                return Err(failure(
                    &import.source,
                    "S2031",
                    format!(
                        "Rust dependency member `{}` in `{}` was removed when the projected dependency changed from version `{}` to `{}`",
                        import.object,
                        import.target,
                        removed.previous_version,
                        removed.current_version
                    ),
                    import.span,
                ));
            }
            let reason = projection.dependencies.iter().find_map(|dependency| {
                dependency.declined.iter().find_map(|declined| {
                    (crate::projection::namespace_for_rust_path(dependency, &declined.rust_path)
                        == import.target
                        && declined.rust_path.rsplit("::").next() == Some(import.object.as_str()))
                    .then_some(declined.reason.as_str())
                })
            });
            let message = reason.map_or_else(
                || {
                    format!(
                        "Rust dependency projection has no member `{}` in `{}`",
                        import.object, import.target
                    )
                },
                |reason| {
                    format!(
                        "Rust dependency member `{}` in `{}` is not projected: {reason}",
                        import.object, import.target
                    )
                },
            );
            return Err(failure(&import.source, "S2029", message, import.span));
        }
    }
    let prelude_bindings = if package.prelude {
        bootstrap_prelude()
    } else {
        BTreeMap::new()
    };
    let mut import_warnings =
        resolve_imports(imports, &mut namespaces, &globals, &prelude_bindings)?;
    for unit in &mut units {
        unit.scopes = collect_lexical_scopes(unit, &namespaces, &globals, &prelude_bindings)?;
        import_warnings.extend(
            unit.scopes
                .iter()
                .flat_map(|scope| scope.import_warnings.iter().cloned()),
        );
    }
    let descriptor_constructs = bootstrap_descriptor_constructs();

    let mut semantic = SemanticPackage {
        identity: package.identity.clone(),
        prelude: package.prelude,
        reflection: package.reflection,
        executor: package.executor,
        profile: package.profile.clone(),
        namespaces,
        globals,
        prelude_bindings,
        descriptor_constructs,
        units,
        projection,
        binding_events: BTreeMap::new(),
        import_warnings,
        bootstrap_version: BOOTSTRAP_VERSION,
    };
    validate_initializer_dependencies(&semantic)?;
    validate_references(&semantic)?;
    analyze_types(&mut semantic)?;
    validate_error_clauses(&semantic)?;
    validate_moves(&semantic)?;
    validate_reference_origins(&semantic)?;
    validate_referenced_replacements(&semantic)?;
    infer_throwing_effects(&mut semantic)?;
    validate_constant_reassignment(&semantic)?;
    validate_global_definite_assignment(&semantic)?;
    record_binding_mutability(&mut semantic);
    validate_calls(&semantic)?;
    validate_definite_assignment(&semantic)?;
    record_binding_events(&mut semantic);
    validate_suspension_ownership(&semantic)?;
    validate_task_consumption(&semantic)?;
    let unreachable_units = validate_control_flow(&semantic)?;
    for (unit, unreachable_spans) in semantic.units.iter_mut().zip(unreachable_units) {
        unit.unreachable_spans = unreachable_spans;
        unit.evaluation_steps = collect_evaluation_steps(&unit.source, &unit.tree.root);
    }
    Ok(semantic)
}

fn object_implements_identity(object: &ObjectContract, target: &str) -> bool {
    object
        .interfaces
        .iter()
        .any(|interface| interface.qualified() == target)
}

fn identity_implements(package: &SemanticPackage, identity: &str, target: &str) -> bool {
    package.units.iter().any(|unit| {
        unit.objects.iter().any(|object| {
            package
                .namespaces
                .values()
                .flat_map(|namespace| namespace.symbols.values())
                .any(|symbol| {
                    symbol.identity == identity && symbol.declaration_span == Some(object.span)
                })
                && object_implements_identity(object, target)
        })
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "error validation keeps throw, catch, and finally rules in one ordered traversal"
)]
fn validate_error_clauses(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    #[expect(
        clippy::too_many_lines,
        reason = "the recursive visitor validates the complete structured-error boundary"
    )]
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        in_catch: bool,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::ThrowStatement {
            if node.children.is_empty() {
                if !in_catch {
                    return Err(failure(
                        &unit.source,
                        "T0020",
                        "bare `throw` is only valid inside a catch clause",
                        node.span,
                    ));
                }
            } else {
                let thrown = &node.children[0];
                let symbol = package.resolve_name_at(
                    unit,
                    thrown.span.start,
                    node_text(&unit.source, thrown.children.first().unwrap_or(thrown)),
                );
                let standard = symbol.is_some_and(|symbol| symbol.kind == SymbolKind::ErrorObject);
                let value_type = infer_value_type(unit, thrown, &unit.typed_bindings)?;
                let object_name = match &value_type {
                    Some(ValueType::Descriptor(name)) => Some(name.as_str()),
                    Some(ValueType::Object(identity)) => Some(identity.name.as_str()),
                    _ if thrown.kind == SyntaxKind::CallExpression => thrown
                        .children
                        .first()
                        .filter(|callee| callee.kind == SyntaxKind::Name)
                        .map(|callee| node_text(&unit.source, callee)),
                    _ => None,
                };
                let user_throwable = object_name
                    .and_then(|name| {
                        package.resolve_name_at(
                            unit,
                            thrown.span.start,
                            name.rsplit_once("::").map_or(name, |(_, local)| local),
                        )
                    })
                    .is_some_and(|symbol| {
                        identity_implements(package, &symbol.identity, "/core/errors::throwable")
                    });
                if !standard && !user_throwable {
                    return Err(failure(
                        &unit.source,
                        "T0021",
                        "thrown values must implement `throwable`",
                        thrown.span,
                    ));
                }
            }
        }
        if node.kind == SyntaxKind::TryStatement {
            let mut caught = BTreeSet::new();
            let mut catches_all = false;
            for clause in node
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
            {
                if let Some(alias) = clause
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::CatchBinding)
                {
                    return Err(failure(
                        &unit.source,
                        "T0027",
                        "catch aliases are unavailable until error values expose source-level members",
                        alias.span,
                    ));
                }
                let Some(descriptor) = clause
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Name)
                else {
                    if catches_all {
                        return Err(failure(
                            &unit.source,
                            "T0022",
                            "catch-all clause is unreachable",
                            clause.span,
                        ));
                    }
                    catches_all = true;
                    continue;
                };
                let name = node_text(&unit.source, descriptor);
                let symbol = package.resolve_name_at(unit, descriptor.span.start, name);
                let valid = symbol.is_some_and(|symbol| {
                    symbol.kind == SymbolKind::ErrorObject
                        || (symbol.kind == SymbolKind::Interface
                            && symbol.identity == "/core/errors::throwable")
                        || (matches!(symbol.kind, SymbolKind::Class | SymbolKind::TypeDescriptor)
                            && identity_implements(
                                package,
                                &symbol.identity,
                                "/core/errors::throwable",
                            ))
                });
                if !valid {
                    return Err(failure(
                        &unit.source,
                        "T0021",
                        format!("`{name}` is not a throwable descriptor"),
                        descriptor.span,
                    ));
                }
                let identity = &symbol.expect("validated error symbol").identity;
                if catches_all || !caught.insert(identity.clone()) {
                    return Err(failure(
                        &unit.source,
                        "T0022",
                        format!("catch clause for `{name}` is unreachable"),
                        clause.span,
                    ));
                }
                catches_all = identity == "/core/errors::throwable";
            }
        }
        for child in &node.children {
            let child_in_catch = in_catch || node.kind == SyntaxKind::CatchClause;
            visit(package, unit, child, child_in_catch)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit(package, unit, &unit.tree.root, false)?;
    }
    Ok(())
}

fn populate_namespace_function_contracts(package: &mut SemanticPackage) {
    let namespaces = package
        .units
        .iter()
        .map(|unit| unit.namespace.clone())
        .collect::<Vec<_>>();
    let functions = package
        .units
        .iter()
        .map(|unit| unit.functions.clone())
        .collect::<Vec<_>>();
    for (unit, namespace) in package.units.iter_mut().zip(&namespaces) {
        unit.functions = namespaces
            .iter()
            .zip(&functions)
            .filter(|(candidate, _)| *candidate == namespace)
            .flat_map(|(_, functions)| functions.iter().cloned())
            .collect();
    }
}

fn populate_object_aliases(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| unit.objects.iter())
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        let mut aliases = package
            .namespaces
            .get(&unit.namespace)
            .into_iter()
            .flat_map(|namespace| &namespace.symbols)
            .chain(
                unit.scopes
                    .iter()
                    .flat_map(|scope| &scope.symbols)
                    .flat_map(|(name, symbols)| symbols.iter().map(move |symbol| (name, symbol))),
            )
            .filter_map(|(visible_name, symbol)| {
                let span = symbol.declaration_span?;
                matches!(
                    symbol.kind,
                    SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                )
                .then(|| contracts.get(&(span.file, span.start, span.end)))
                .flatten()
                .cloned()
                .map(|mut contract| {
                    contract.name.clone_from(visible_name);
                    contract
                })
            })
            .collect::<Vec<_>>();
        aliases.retain(|alias| {
            !unit
                .objects
                .iter()
                .any(|contract| contract.name == alias.name)
        });
        unit.objects.extend(aliases);
    }
}

fn populate_function_aliases(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| unit.functions.iter())
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        let mut aliases = BTreeMap::new();
        let mut contracts_by_span = BTreeMap::new();
        for namespace_name in namespace_chain(&unit.namespace) {
            let Some(namespace) = package.namespaces.get(&namespace_name) else {
                continue;
            };
            for (visible_name, symbol) in &namespace.symbols {
                let Some(span) = symbol.declaration_span else {
                    continue;
                };
                if symbol.kind != SymbolKind::Function || !visible_from(symbol, &unit.namespace) {
                    continue;
                }
                let key = (span.file, span.start, span.end);
                if let Some(contract) = contracts.get(&key) {
                    aliases
                        .entry(visible_name.clone())
                        .or_insert_with(|| contract.clone());
                    contracts_by_span
                        .entry(key)
                        .or_insert_with(|| contract.clone());
                }
            }
        }
        for symbol in unit
            .scopes
            .iter()
            .flat_map(|scope| scope.symbols.values())
            .flatten()
            .filter(|symbol| symbol.kind == SymbolKind::Function)
        {
            let Some(span) = symbol.declaration_span else {
                continue;
            };
            let key = (span.file, span.start, span.end);
            if let Some(contract) = contracts.get(&key) {
                contracts_by_span
                    .entry(key)
                    .or_insert_with(|| contract.clone());
            }
        }
        unit.function_aliases = aliases;
        unit.function_contracts_by_span = contracts_by_span;
    }
}

fn resolved_function_contract<'a>(
    unit: &'a SemanticUnit,
    name: &str,
    offset: usize,
) -> Option<&'a FunctionContract> {
    lexical_scope_chain(unit, offset)
        .find_map(|scope| {
            let symbol = scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol.kind == SymbolKind::Function
                    && symbol.binding_span.is_none_or(|span| span.end <= offset)
            })?;
            let span = symbol.declaration_span?;
            unit.function_contracts_by_span
                .get(&(span.file, span.start, span.end))
        })
        .or_else(|| unit.function_aliases.get(name))
}

fn populate_function_type_dependencies(package: &mut SemanticPackage) {
    let objects = package
        .units
        .iter()
        .flat_map(|unit| unit.objects.iter())
        .map(|object| (object.identity.clone(), object.clone()))
        .collect::<BTreeMap<_, _>>();
    let methods = package
        .units
        .iter()
        .flat_map(|unit| unit.functions.iter())
        .filter_map(|method| {
            method
                .owner
                .as_ref()
                .map(|owner| ((method.span.file, owner.clone()), method.clone()))
        })
        .fold(
            BTreeMap::<(u32, String), Vec<FunctionContract>>::new(),
            |mut methods, (key, method)| {
                methods.entry(key).or_default().push(method);
                methods
            },
        );
    for unit in &mut package.units {
        let mut queue = unit
            .function_aliases
            .values()
            .chain(unit.function_contracts_by_span.values())
            .filter_map(|contract| match &contract.return_type {
                Some(ValueType::Object(identity)) => Some(identity.clone()),
                _ => None,
            })
            .chain(
                unit.objects
                    .iter()
                    .filter(|object| {
                        object.name != object.identity.name
                            || object.identity.namespace != unit.namespace
                    })
                    .map(|object| object.identity.clone()),
            )
            .collect::<Vec<_>>();
        let mut visited = BTreeSet::new();
        while let Some(key) = queue.pop() {
            if !visited.insert(key.clone()) {
                continue;
            }
            let Some(object) = objects.get(&key) else {
                continue;
            };
            for field in &object.fields {
                if let ValueType::Object(name) = &field.value_type {
                    queue.push(name.clone());
                }
            }
            let object_methods = methods
                .get(&(object.span.file, object.name.clone()))
                .cloned()
                .unwrap_or_default();
            for method in &object_methods {
                if let Some(ValueType::Object(name)) = &method.return_type {
                    queue.push(name.clone());
                }
                for parameter in &method.parameters {
                    if let Some(ValueType::Object(name)) = &parameter.value_type {
                        queue.push(name.clone());
                    }
                }
            }
            if !unit
                .objects
                .iter()
                .any(|candidate| candidate.name == object.name)
            {
                unit.objects.push(object.clone());
            }
            for method in object_methods {
                if !unit
                    .functions
                    .iter()
                    .any(|candidate| candidate.span == method.span && candidate.name == method.name)
                {
                    unit.functions.push(method);
                }
            }
        }
    }
}

impl SemanticPackage {
    #[must_use]
    pub fn symbol(&self, namespace: &str, name: &str) -> Option<&Symbol> {
        self.namespaces.get(namespace)?.symbols.get(name)
    }

    #[must_use]
    pub fn resolve_name(&self, namespace: &str, name: &str) -> Option<&Symbol> {
        namespace_chain(namespace)
            .find_map(|path| {
                self.symbol(&path, name).filter(|symbol| {
                    visible_from(symbol, namespace)
                        && (symbol.kind != SymbolKind::Binding
                            || symbol.constant
                            || symbol.global
                            || symbol.namespace == namespace)
                })
            })
            .or_else(|| {
                self.globals
                    .get(name)
                    .filter(|symbol| visible_from(symbol, namespace))
            })
            .or_else(|| self.symbol("/core/types", name))
            .or_else(|| self.prelude_bindings.get(name))
    }

    #[must_use]
    pub fn resolve_name_at<'a>(
        &'a self,
        unit: &'a SemanticUnit,
        offset: usize,
        name: &str,
    ) -> Option<&'a Symbol> {
        let mut scopes = lexical_scope_chain(unit, offset).peekable();
        let inside_lexical_scope = scopes.peek().is_some();
        scopes
            .find_map(|scope| {
                scope
                    .symbols
                    .get(name)?
                    .iter()
                    .rev()
                    .find(|symbol| symbol.binding_span.is_none_or(|span| span.end <= offset))
            })
            .or_else(|| {
                self.resolve_name(&unit.namespace, name)
                    .filter(|symbol| !inside_lexical_scope || symbol.available_in_function_body())
            })
    }

    #[must_use]
    pub fn is_lexical_replacement(&self, unit: &SemanticUnit, span: Span, name: &str) -> bool {
        let Some(current) = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.name == name && binding.span == span)
        else {
            return false;
        };
        let current_scope = lexical_scope_index_at(unit, current.span.start);
        lexical_scope_chain(unit, span.start).any(|scope| {
            scope.symbols.get(name).is_some_and(|symbols| {
                symbols
                    .iter()
                    .any(|symbol| symbol.declaration_span == Some(span))
                    && symbols.iter().any(|symbol| {
                        symbol.declaration_span.is_some_and(|prior| {
                            prior.start < span.start
                                && lexical_scope_index_at(unit, prior.start) == current_scope
                        })
                    })
            })
        })
    }
}

fn declared_namespace(source: &SourceFile, tree: &SyntaxTree) -> Result<String, Diagnostic> {
    let declarations = tree
        .root
        .children
        .iter()
        .filter(|node| node.kind == SyntaxKind::NamespaceDeclaration)
        .collect::<Vec<_>>();
    if declarations.is_empty() {
        return Err(Diagnostic::error(
            "S2002",
            "each source unit must declare exactly one namespace",
            Span::new(source.id(), 0, source.text().len()),
        ));
    }
    if declarations.len() > 1 {
        return Err(Diagnostic::error(
            "S0005",
            "duplicate namespace declaration",
            declarations[1].span,
        ));
    }
    let components = declarations[0]
        .children
        .iter()
        .filter(|child| child.kind == SyntaxKind::Name)
        .map(|child| {
            let component = node_text(source, child);
            validate_namespace_segment(component, child.span)?;
            Ok(component)
        })
        .collect::<Result<Vec<_>, Diagnostic>>()?;
    normalize_declared_path(&components).ok_or_else(|| {
        Diagnostic::error(
            "S2003",
            "namespace declaration requires an unanchored path",
            declarations[0].span,
        )
    })
}

fn validate_namespace_segment(component: &str, span: Span) -> Result<(), Diagnostic> {
    fn valid(component: &str) -> bool {
        let mut bytes = component.bytes();
        let Some(first) = bytes.next() else {
            return false;
        };
        if !first.is_ascii_lowercase() {
            return false;
        }
        let mut previous_hyphen = false;
        for byte in bytes {
            if byte == b'-' {
                if previous_hyphen {
                    return false;
                }
                previous_hyphen = true;
            } else if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
                previous_hyphen = false;
            } else {
                return false;
            }
        }
        !previous_hyphen
    }

    if !valid(component) {
        let lowercase = component.to_ascii_lowercase();
        let mut diagnostic = Diagnostic::error(
            "S2018",
            format!(
                "invalid namespace segment `{component}`; segments must match `[a-z]([a-z0-9]|-[a-z0-9])*`"
            ),
            span,
        );
        if lowercase != component && valid(&lowercase) {
            diagnostic = diagnostic.with_help(format!("use `{lowercase}`"));
        }
        return Err(diagnostic);
    }
    if is_reserved_namespace_segment(component) {
        return Err(Diagnostic::error(
            "S2019",
            format!("namespace segment `{component}` is reserved"),
            span,
        )
        .with_help(format!(
            "choose a different name, such as `{component}-app`"
        )));
    }
    Ok(())
}

fn is_reserved_namespace_segment(component: &str) -> bool {
    matches!(component, "con" | "prn" | "aux" | "nul")
        || component
            .strip_prefix("com")
            .or_else(|| component.strip_prefix("lpt"))
            .is_some_and(|suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9'))
}

fn collect_unit(
    unit: &SemanticUnit,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &mut BTreeMap<String, Symbol>,
    imports: &mut Vec<Import>,
) -> Result<(), SemanticFailure> {
    for node in &unit.tree.root.children {
        match node.kind {
            SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration => {
                collect_declaration(unit, node, namespaces, globals)?;
            }
            SyntaxKind::ImportDeclaration => imports.extend(imports_from_syntax(unit, node)?),
            _ => {}
        }
        collect_nested_declarations(unit, node, namespaces, globals)?;
    }
    Ok(())
}
fn collect_nested_declarations(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &mut BTreeMap<String, Symbol>,
) -> Result<(), SemanticFailure> {
    for child in &node.children {
        if matches!(
            child.kind,
            SyntaxKind::Binding | SyntaxKind::Assignment | SyntaxKind::FunctionDeclaration
        ) && declaration_from_syntax(unit, child).is_some_and(|declaration| declaration.global)
        {
            collect_declaration(unit, child, namespaces, globals)?;
        }
        collect_nested_declarations(unit, child, namespaces, globals)?;
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct Declaration {
    name: String,
    visibility: Visibility,
    explicit_visibility: bool,
    global: bool,
    constant: bool,
    kind: SymbolKind,
}

fn declaration_from_syntax(unit: &SemanticUnit, node: &SyntaxNode) -> Option<Declaration> {
    let name_node = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)?;
    let name = node_text(&unit.source, name_node).to_owned();
    let visibility_node = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Visibility);
    let visibility = visibility_node
        .map(|child| node_text(&unit.source, child))
        .map_or(Visibility::Public, |visibility| match visibility {
            "private" => Visibility::Private,
            "protected" => Visibility::Protected,
            _ => Visibility::Public,
        });
    let qualifier = |expected| {
        node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == expected
        })
    };
    let kind = match node.kind {
        SyntaxKind::FunctionDeclaration => SymbolKind::Function,
        SyntaxKind::ClassDeclaration => SymbolKind::Class,
        SyntaxKind::InterfaceDeclaration => SymbolKind::Interface,
        SyntaxKind::TraitDeclaration => SymbolKind::Trait,
        _ => SymbolKind::Binding,
    };
    Some(Declaration {
        name,
        visibility,
        explicit_visibility: visibility_node.is_some(),
        global: qualifier("global"),
        constant: qualifier("constant"),
        kind,
    })
}

fn collect_declaration(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &mut BTreeMap<String, Symbol>,
) -> Result<(), SemanticFailure> {
    let declaration = declaration_from_syntax(unit, node).ok_or_else(|| {
        failure(
            &unit.source,
            "S2004",
            "declaration has no resolvable name",
            node.span,
        )
    })?;
    if node.kind == SyntaxKind::Assignment && globals.contains_key(&declaration.name) {
        return Ok(());
    }
    if declaration.kind == SymbolKind::Binding
        && !declaration.constant
        && !declaration.global
        && declaration.explicit_visibility
        && declaration.visibility == Visibility::Public
    {
        return Err(failure(
            &unit.source,
            "S2025",
            format!("namespace variable `{}` cannot be public", declaration.name),
            node.span,
        ));
    }
    let identity = if declaration.global {
        format!("global::{}", declaration.name)
    } else {
        format!("{}::{}", unit.namespace, declaration.name)
    };
    let symbol = Symbol {
        identity,
        lowering_identity: None,
        name: declaration.name.clone(),
        namespace: unit.namespace.clone(),
        visibility: declaration.visibility,
        global: declaration.global,
        constant: declaration.constant,
        kind: declaration.kind,
        declaration_span: Some(node.span),
        binding_span: Some(node.span),
    };
    if declaration.global {
        globals.insert(declaration.name, symbol);
        return Ok(());
    }
    let table = &mut namespaces
        .get_mut(&unit.namespace)
        .expect("every source-unit namespace is assembled before declarations")
        .symbols;
    if node.kind == SyntaxKind::Assignment
        && table.get(&declaration.name).is_some_and(|existing| {
            existing
                .declaration_span
                .is_some_and(|span| span.file == node.span.file)
        })
    {
        return Ok(());
    }
    if table.contains_key(&declaration.name) {
        return Err(failure(
            &unit.source,
            "S2005",
            format!("duplicate declaration `{}`", declaration.name),
            node.span,
        ));
    }
    table.insert(declaration.name, symbol);
    Ok(())
}

fn namespace_capabilities(namespace: &str) -> &'static [&'static str] {
    match namespace {
        "/core/streams" | "/core/process" => &["process"],
        "/core/filesystem" => &["filesystem"],
        "/core/random" | "/core/random/uuid" => &["entropy"],
        "/core/networking" => &["networking"],
        "/core/networking/tls" => &["networking", "tls"],
        "/core/concurrency" => &["threads"],
        _ => &[],
    }
}

fn imports_from_syntax(
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<Vec<Import>, SemanticFailure> {
    let path = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::NamespacePath)
        .ok_or_else(|| failure(&unit.source, "S2006", "malformed import", node.span))?;
    let anchored = path.children.first().is_some_and(|child| {
        child.kind == SyntaxKind::NamespaceAnchor && node_text(&unit.source, child) == "/"
    });
    let components = path
        .children
        .iter()
        .map(|child| node_text(&unit.source, child))
        .collect::<Vec<_>>();
    let target = resolve_path(&unit.namespace, anchored, &components).ok_or_else(|| {
        failure(
            &unit.source,
            "S2007",
            "namespace path escapes above root",
            path.span,
        )
    })?;
    let imports = node
        .children
        .iter()
        .filter(|child| child.kind == SyntaxKind::ObjectImport);
    let mut result = Vec::new();
    for import_node in imports {
        let imported_node = import_node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "S2008",
                    "import has no name",
                    import_node.span,
                )
            })?;
        let imported = node_text(&unit.source, imported_node);
        let alias = import_node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::ImportAlias)
            .and_then(|alias| {
                alias
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
            })
            .map_or(imported, |alias| node_text(&unit.source, alias));
        result.push(Import {
            source: unit.source.clone(),
            bundled: unit.bundled,
            namespace: unit.namespace.clone(),
            target: target.clone(),
            namespace_wide: false,
            object: imported.to_owned(),
            alias: alias.to_owned(),
            span: import_node.span,
        });
    }
    if result.is_empty() {
        if target == "/deps" || target.starts_with("/deps/") {
            return Err(failure(
                &unit.source,
                "S2033",
                "namespace-wide dependency imports are not implemented; import dependency objects explicitly",
                node.span,
            ));
        }
        result.push(Import {
            source: unit.source.clone(),
            bundled: unit.bundled,
            namespace: unit.namespace.clone(),
            target,
            namespace_wide: true,
            object: String::new(),
            alias: String::new(),
            span: node.span,
        });
    }
    Ok(result)
}
fn imports_in_tree(unit: &SemanticUnit) -> Result<Vec<Import>, SemanticFailure> {
    fn collect(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        imports: &mut Vec<Import>,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::ImportDeclaration {
            imports.extend(imports_from_syntax(unit, node)?);
        }
        for child in &node.children {
            collect(unit, child, imports)?;
        }
        Ok(())
    }

    let mut imports = Vec::new();
    collect(unit, &unit.tree.root, &mut imports)?;
    Ok(imports)
}
fn imported_object(
    import: &Import,
    namespaces: &BTreeMap<String, Namespace>,
) -> Result<Symbol, SemanticFailure> {
    let export = namespaces
        .get(&import.target)
        .and_then(|namespace| namespace.symbols.get(&import.object))
        .ok_or_else(|| {
            failure(
                &import.source,
                "S2009",
                format!("unresolved name `{}` in `{}`", import.object, import.target),
                import.span,
            )
        })?;
    if !visible_from(export, &import.namespace) {
        return Err(failure(
            &import.source,
            "S2010",
            format!("name `{}` is inaccessible", import.object),
            import.span,
        ));
    }
    if !export.available_in_function_body() {
        return Err(namespace_variable_import_failure(
            &import.source,
            &import.object,
            import.span,
        ));
    }
    Ok(export.clone())
}
fn imported_objects(
    import: &Import,
    namespaces: &BTreeMap<String, Namespace>,
) -> Result<Vec<(String, Symbol)>, SemanticFailure> {
    if !import.namespace_wide {
        return Ok(vec![(
            import.alias.clone(),
            imported_object(import, namespaces)?,
        )]);
    }
    let namespace = namespaces.get(&import.target).ok_or_else(|| {
        failure(
            &import.source,
            "S2009",
            format!("unknown namespace `{}`", import.target),
            import.span,
        )
    })?;
    if let Some(symbol) = namespace.symbols.values().find(|symbol| {
        symbol.visibility == Visibility::Public && !symbol.available_in_function_body()
    }) {
        return Err(namespace_variable_import_failure(
            &import.source,
            &symbol.name,
            import.span,
        ));
    }
    Ok(namespace
        .symbols
        .values()
        .filter(|symbol| {
            symbol.visibility == Visibility::Public && symbol.available_in_function_body()
        })
        .map(|symbol| (symbol.name.clone(), symbol.clone()))
        .collect())
}

fn import_collision_failure(import: &Import, name: &str) -> SemanticFailure {
    failure(
        &import.source,
        "S2011",
        format!("object import collides on `{name}`; use an `as` alias"),
        import.span,
    )
}

fn import_overwrite_warning(
    name: &str,
    existing: &Symbol,
    replacement: &Symbol,
    span: Span,
) -> Diagnostic {
    Diagnostic::warning(
        "W4004",
        format!(
            "namespace-wide import overwrites `{name}` from `{}` with `{}`",
            existing.identity, replacement.identity
        ),
        span,
    )
    .with_help("use selective `from ... import ... as ...` imports to retain both objects")
}

fn import_declaration_precedence_warning(
    import: &Import,
    name: &str,
    declaration: &Symbol,
    rejected: &Symbol,
) -> Diagnostic {
    Diagnostic::warning(
        "W4004",
        format!(
            "namespace-wide import leaves declared `{name}` from `{}` in place instead of `{}`",
            declaration.identity, rejected.identity
        ),
        import.span,
    )
    .with_help("use a selective `from ... import ... as ...` import to bind the imported object")
}

fn visible_fallback_symbol<'a>(
    namespace: &str,
    name: &str,
    namespaces: &'a BTreeMap<String, Namespace>,
    globals: &'a BTreeMap<String, Symbol>,
    prelude_bindings: &'a BTreeMap<String, Symbol>,
) -> Option<&'a Symbol> {
    namespace_chain(namespace)
        .skip(1)
        .find_map(|path| {
            namespaces.get(&path)?.symbols.get(name).filter(|symbol| {
                visible_from(symbol, namespace)
                    && (symbol.kind != SymbolKind::Binding
                        || symbol.constant
                        || symbol.global
                        || symbol.namespace == namespace)
            })
        })
        .or_else(|| {
            globals
                .get(name)
                .filter(|symbol| visible_from(symbol, namespace))
        })
        .or_else(|| namespaces.get("/core/types")?.symbols.get(name))
        .or_else(|| prelude_bindings.get(name))
}

fn resolve_imports(
    imports: Vec<Import>,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    prelude_bindings: &BTreeMap<String, Symbol>,
) -> Result<Vec<Diagnostic>, SemanticFailure> {
    let mut warnings = Vec::new();
    for import in imports {
        let exports = imported_objects(&import, namespaces)?;
        for (name, mut export) in exports {
            let existing = namespaces
                .get(&import.namespace)
                .and_then(|destination| destination.symbols.get(&name))
                .cloned();
            if let Some(existing) = existing {
                if existing.identity == export.identity {
                    continue;
                }
                if !import.namespace_wide {
                    return Err(import_collision_failure(&import, &name));
                }
                if existing.namespace == import.namespace {
                    warnings.push(import_declaration_precedence_warning(
                        &import, &name, &existing, &export,
                    ));
                    continue;
                }
                warnings.push(import_overwrite_warning(
                    &name,
                    &existing,
                    &export,
                    existing.binding_span.unwrap_or(import.span),
                ));
            } else if import.namespace_wide
                && let Some(existing) = visible_fallback_symbol(
                    &import.namespace,
                    &name,
                    namespaces,
                    globals,
                    prelude_bindings,
                )
                && existing.identity != export.identity
            {
                warnings.push(import_overwrite_warning(
                    &name,
                    existing,
                    &export,
                    import.span,
                ));
            }
            export.binding_span = Some(import.span);
            namespaces
                .get_mut(&import.namespace)
                .expect("every import destination is a preassembled source-unit namespace")
                .symbols
                .insert(name, export);
        }
    }
    Ok(warnings)
}

fn resolved_object_span(package: &SemanticPackage, identity: &ObjectIdentity) -> Option<Span> {
    package
        .units
        .iter()
        .flat_map(|unit| &unit.objects)
        .find(|object| object.identity == *identity)
        .map(|object| object.span)
}

fn enclosing_function_contract(unit: &SemanticUnit, offset: usize) -> Option<&FunctionContract> {
    let span = unit
        .enclosing_function_spans
        .get(&offset)
        .copied()
        .flatten()?;
    unit.functions.iter().find(|contract| contract.span == span)
}

fn is_implicit_object_receiver(unit: &SemanticUnit, offset: usize, name: &str) -> bool {
    let Some(function_span) = unit
        .enclosing_function_spans
        .get(&offset)
        .copied()
        .flatten()
    else {
        return false;
    };
    if object_name_containing(unit, function_span).is_none() {
        return false;
    }
    if name == "self" {
        return true;
    }
    if name != "this" {
        return false;
    }
    find_node_by_span(&unit.tree.root, function_span).is_some_and(|function| {
        !function.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == "static"
        })
    })
}

fn class_designator_identity(
    unit: &SemanticUnit,
    designator: &SyntaxNode,
) -> Option<ObjectIdentity> {
    if designator.kind != SyntaxKind::Name {
        return None;
    }
    let name = node_text(&unit.source, designator);
    if name != "self"
        && unit.typed_bindings.iter().rev().any(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), designator.span.start)
        })
    {
        return None;
    }
    if name == "self" {
        let owner = enclosing_function_contract(unit, designator.span.start)?
            .owner
            .as_deref()?;
        return unit
            .objects
            .iter()
            .find(|object| object.name == owner && object.kind == ObjectKind::Class)
            .map(|object| object.identity.clone());
    }
    unit.objects
        .iter()
        .find(|object| object.name == name && object.kind == ObjectKind::Class)
        .map(|object| object.identity.clone())
}

fn method_contract<'a>(
    package: &'a SemanticPackage,
    object_identity: &ObjectIdentity,
    method_name: &str,
    is_static: bool,
) -> Option<&'a FunctionContract> {
    fn contract<'a>(
        unit: &'a SemanticUnit,
        object_name: &str,
        method_name: &str,
        is_static: bool,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| {
                method.owner.as_deref() == Some(object_name)
                    && method.name == method_name
                    && method.is_static == is_static
            })
            .or_else(|| {
                unit.objects
                    .iter()
                    .find(|object| object.name == object_name)
                    .and_then(|object| object.base.as_ref())
                    .and_then(|base| unit.objects.iter().find(|object| object.identity == *base))
                    .and_then(|base| contract(unit, &base.name, method_name, is_static))
            })
    }
    let object = package
        .units
        .iter()
        .flat_map(|candidate| &candidate.objects)
        .find(|object| object.identity == *object_identity)?;
    package
        .units
        .iter()
        .find(|candidate| candidate.source.id() == object.span.file)
        .and_then(|candidate| contract(candidate, &object.name, method_name, is_static))
}

fn construction_contract<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    callee: &SyntaxNode,
) -> Option<&'a FunctionContract> {
    let class = callee
        .children
        .first()
        .filter(|_| callee.kind == SyntaxKind::ConstructionExpression)?;
    let identity = class_designator_identity(unit, class)?;
    method_contract(package, &identity, "construct", false)
}

fn function_parameters<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    callee: &SyntaxNode,
) -> Option<&'a [ParameterContract]> {
    if matches!(
        callee.kind,
        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
    ) {
        let [receiver, member] = callee.children.as_slice() else {
            return None;
        };
        let object_identity = if callee.kind == SyntaxKind::StaticMemberExpression {
            class_designator_identity(unit, receiver)?
        } else {
            let ValueType::Object(object_identity) = unit.inferred_value_type(receiver)? else {
                return None;
            };
            object_identity
        };
        return method_contract(
            package,
            &object_identity,
            node_text(&unit.source, member),
            callee.kind == SyntaxKind::StaticMemberExpression,
        )
        .map(|method| method.parameters.as_slice());
    }
    if callee.kind == SyntaxKind::ConstructionExpression {
        return construction_contract(package, unit, callee)
            .map(|function| function.parameters.as_slice());
    }
    if callee.kind != SyntaxKind::Name {
        return None;
    }
    let symbol =
        package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
    let declaration = symbol.declaration_span?;
    package
        .units
        .iter()
        .flat_map(|candidate| &candidate.functions)
        .find(|function| function.span == declaration)
        .map(|function| function.parameters.as_slice())
}
#[expect(
    clippy::too_many_lines,
    reason = "move provenance and its control-flow join remain one auditable analysis"
)]
fn validate_moves(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn binding_at(unit: &SemanticUnit, name: &str, position: usize) -> Option<usize> {
        unit.typed_bindings
            .iter()
            .enumerate()
            .rev()
            .find(|(_, binding)| {
                binding.name == name && binding.is_visible_at(unit.source.id(), position)
            })
            .map(|(index, _)| index)
    }

    fn resource_binding(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        binding: usize,
        resource_objects: &BTreeSet<(u32, usize, usize)>,
    ) -> bool {
        match &unit.typed_bindings[binding].value_type {
            ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
            ValueType::Object(name) => resolved_object_span(package, name)
                .is_some_and(|span| resource_objects.contains(&span_key(span))),
            _ => false,
        }
    }

    fn method_consumes_receiver(
        package: &SemanticPackage,
        object_identity: &ObjectIdentity,
        method_name: &str,
    ) -> bool {
        method_contract(package, object_identity, method_name, false)
            .is_some_and(|method| method.consumes_receiver)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "move traversal keeps scope transitions and diagnostics in one ordered dispatch"
    )]
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        moved: &mut BTreeSet<usize>,
        declaration_name: bool,
        resource_objects: &BTreeSet<(u32, usize, usize)>,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
            && unary_operator_text(unit, node).as_deref() == Some("move")
            && operand.kind == SyntaxKind::Name
        {
            let name = node_text(&unit.source, operand);
            if let Some(binding) = binding_at(unit, name, operand.span.start) {
                if !moved.insert(binding) {
                    return Err(failure(
                        &unit.source,
                        "T0058",
                        format!("`{name}` was already moved and is unavailable until rebound"),
                        operand.span,
                    ));
                }
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member, ..] = callee.children.as_slice()
            && receiver.kind != SyntaxKind::Name
            && let Ok(Some(ValueType::Object(object_name))) =
                infer_value_type(unit, receiver, &unit.typed_bindings)
            && method_consumes_receiver(package, &object_name, node_text(&unit.source, member))
            && resolved_object_span(package, &object_name)
                .is_some_and(|span| resource_objects.contains(&span_key(span)))
        {
            return Err(failure(
                &unit.source,
                "T0101",
                "a resource-consuming call requires a named binding; move the member into a binding first",
                receiver.span,
            ));
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member, ..] = callee.children.as_slice()
            && receiver.kind == SyntaxKind::Name
            && matches!(
                infer_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(ValueType::Object(object_name)))
                    if method_consumes_receiver(
                        package,
                        &object_name,
                        node_text(&unit.source, member),
                    )
            )
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, receiver), receiver.span.start)
            && resource_binding(package, unit, binding, resource_objects)
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && let Some(parameters) = function_parameters(package, unit, callee)
        {
            for (argument, parameter) in arguments.children.iter().zip(parameters) {
                let Some(expected) = parameter.value_type.as_ref() else {
                    continue;
                };
                let expects_resource = match expected {
                    ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
                    ValueType::Object(name) => resolved_object_span(package, name)
                        .is_some_and(|span| resource_objects.contains(&span_key(span))),
                    _ => false,
                };
                let value = argument.children.last().unwrap_or(argument);
                if expects_resource
                    && matches!(
                        value.kind,
                        SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
                    )
                    && !value.children.first().is_some_and(|receiver| {
                        receiver.kind == SyntaxKind::Name
                            && node_text(&unit.source, receiver) == "this"
                    })
                {
                    return Err(failure(
                        &unit.source,
                        "T0101",
                        "resource transfer requires a named binding",
                        value.span,
                    ));
                }
            }
            let transferred = arguments
                .children
                .iter()
                .zip(parameters)
                .filter_map(|(argument, parameter)| {
                    parameter
                        .value_type
                        .as_ref()
                        .filter(|value_type| {
                            matches!(
                                value_type,
                                ValueType::PlatformStreamHandle
                                    | ValueType::PlatformResourceHandle
                                    | ValueType::Object(_)
                            )
                        })
                        .and_then(|_| argument.children.last())
                        .filter(|value| value.kind == SyntaxKind::Name)
                        .and_then(|value| {
                            binding_at(unit, node_text(&unit.source, value), value.span.start)
                        })
                        .filter(|binding| {
                            resource_binding(package, unit, *binding, resource_objects)
                        })
                })
                .collect::<Vec<_>>();
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.extend(transferred);
            return Ok(());
        }
        if node.kind == SyntaxKind::Name && !declaration_name {
            let name = node_text(&unit.source, node);
            if let Some(binding) = binding_at(unit, name, node.span.start)
                && moved.contains(&binding)
            {
                return Err(failure(
                    &unit.source,
                    "T0058",
                    format!("`{name}` was moved and is unavailable until rebound"),
                    node.span,
                ));
            }
            return Ok(());
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
            let transferred = node
                .children
                .last()
                .filter(|initializer| initializer.kind == SyntaxKind::Name)
                .and_then(|initializer| {
                    binding_at(
                        unit,
                        node_text(&unit.source, initializer),
                        initializer.span.start,
                    )
                })
                .filter(|binding| resource_binding(package, unit, *binding, resource_objects));
            let mut skipped_name = false;
            for child in &node.children {
                if !skipped_name && child.kind == SyntaxKind::Name {
                    skipped_name = true;
                    continue;
                }
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            if let Some(binding) = transferred {
                moved.insert(binding);
            }
            if node.kind == SyntaxKind::Assignment
                && let Some(name) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
                    .map(|name| node_text(&unit.source, name))
                && let Some(binding) = binding_at(unit, name, node.span.start)
            {
                moved.remove(&binding);
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::IfStatement {
            let mut entry = moved.clone();
            for child in &node.children {
                if !matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
                    visit(package, unit, child, &mut entry, false, resource_objects)?;
                }
            }
            let mut branches = Vec::new();
            let mut has_else = false;
            for child in &node.children {
                if matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
                    has_else |= child.kind == SyntaxKind::ElseClause;
                    let mut branch = entry.clone();
                    visit(package, unit, child, &mut branch, false, resource_objects)?;
                    branches.push(branch);
                }
            }
            if !has_else {
                branches.push(entry);
            }
            moved.clear();
            moved.extend(branches.into_iter().flatten());
            return Ok(());
        }
        if matches!(
            node.kind,
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement
        ) {
            let mut entry = moved.clone();
            let body = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block);
            for child in &node.children {
                if Some(child) != body {
                    visit(package, unit, child, &mut entry, false, resource_objects)?;
                }
            }
            if let Some(body) = body {
                let mut after_iteration = entry.clone();
                visit(
                    package,
                    unit,
                    body,
                    &mut after_iteration,
                    false,
                    resource_objects,
                )?;
                // Validate the back edge: the next iteration starts with the first iteration's
                // move state, even though only the may-execute-once state leaves the loop.
                let mut next_iteration = after_iteration.clone();
                visit(
                    package,
                    unit,
                    body,
                    &mut next_iteration,
                    false,
                    resource_objects,
                )?;
                entry.extend(after_iteration);
            }
            *moved = entry;
            return Ok(());
        }
        for child in &node.children {
            visit(package, unit, child, moved, false, resource_objects)?;
        }
        Ok(())
    }

    let resource_objects = package
        .units
        .iter()
        .flat_map(|unit| unit.objects.iter())
        .filter(|object| object.resource_owning)
        .map(|object| span_key(object.span))
        .collect();
    for unit in &package.units {
        visit(
            package,
            unit,
            &unit.tree.root,
            &mut BTreeSet::new(),
            false,
            &resource_objects,
        )?;
    }
    Ok(())
}

fn validate_reference_origins(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(unit: &SemanticUnit, node: &SyntaxNode) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
            && unary_operator_text(unit, node).as_deref() == Some("ref")
        {
            let valid_origin = operand.kind == SyntaxKind::Name
                && unit.typed_bindings.iter().rev().any(|binding| {
                    binding.name == node_text(&unit.source, operand)
                        && binding.is_visible_at(unit.source.id(), operand.span.start)
                        && binding.scope.is_some()
                        && find_node_by_span(&unit.tree.root, binding.span)
                            .is_some_and(|origin| origin.kind == SyntaxKind::Binding)
                });
            if !valid_origin {
                return Err(failure(
                    &unit.source,
                    "T0064",
                    "`ref` requires a named binding with reference-backed storage",
                    operand.span,
                ));
            }
        }
        if node.kind == SyntaxKind::ReturnStatement
            && let Some(value) = node.children.first()
            && matches!(
                infer_value_type(unit, value, &unit.typed_bindings)?,
                Some(ValueType::Reference(_))
            )
        {
            return Err(failure(
                &unit.source,
                "T0068",
                "a non-owning reference cannot escape its proven source lifetime",
                value.span,
            ));
        }
        for child in &node.children {
            visit(unit, child)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit(unit, &unit.tree.root)?;
    }
    Ok(())
}

fn validate_referenced_replacements(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn collect_origins(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        observer: Option<Span>,
        origins: &mut Vec<(Span, Span)>,
    ) {
        let observer = if node.kind == SyntaxKind::Binding {
            unit.typed_bindings
                .iter()
                .find(|binding| binding.span == node.span)
                .map(|binding| binding.span)
                .or(observer)
        } else {
            observer
        };
        if node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("ref")
            && let Some(observer) = observer
            && let Some(operand) = node.children.last()
            && operand.kind == SyntaxKind::Name
            && let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == node_text(&unit.source, operand)
                    && binding.is_visible_at(unit.source.id(), operand.span.start)
            })
        {
            origins.push((binding.span, observer));
        }
        for child in &node.children {
            collect_origins(unit, child, observer, origins);
        }
    }

    fn first_use_after(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        declaration: Span,
        position: usize,
    ) -> Option<Span> {
        if node.kind == SyntaxKind::Name
            && node.span.start > position
            && package
                .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                .and_then(|symbol| symbol.declaration_span)
                == Some(declaration)
        {
            return Some(node.span);
        }
        node.children
            .iter()
            .find_map(|child| first_use_after(package, unit, child, declaration, position))
    }

    for unit in &package.units {
        let mut origins = Vec::new();
        collect_origins(unit, &unit.tree.root, None, &mut origins);
        for replacement in &unit.typed_bindings {
            let previous = unit
                .typed_bindings
                .iter()
                .filter(|binding| {
                    binding.name == replacement.name
                        && binding.scope == replacement.scope
                        && binding.visible_from < replacement.visible_from
                })
                .max_by_key(|binding| binding.visible_from);
            if let Some(previous) = previous
                && previous.value_type != replacement.value_type
                && let Some(use_span) = origins
                    .iter()
                    .filter(|(origin, _)| *origin == previous.span)
                    .find_map(|(_, observer)| {
                        first_use_after(
                            package,
                            unit,
                            &unit.tree.root,
                            *observer,
                            replacement.span.end,
                        )
                    })
            {
                return Err(failure(
                    &unit.source,
                    "T0059",
                    format!(
                        "a reference to the previous `{}` value is unavailable after replacement",
                        replacement.name
                    ),
                    use_span,
                ));
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "reference validation keeps ownership forms in one ordered syntax traversal"
)]
fn validate_references(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        _callable_position: bool,
    ) -> Result<(), SemanticFailure> {
        match node.kind {
            SyntaxKind::Name => {
                let name = node_text(&unit.source, node);
                let resolved = package.resolve_name_at(unit, node.span.start, name);
                let implicit_receiver = is_implicit_object_receiver(unit, node.span.start, name);
                if resolved.is_none()
                    && !package.descriptor_constructs.contains_key(name)
                    && !implicit_receiver
                {
                    if namespace_chain(&unit.namespace)
                        .filter_map(|path| package.namespaces.get(&path))
                        .filter_map(|namespace| namespace.symbols.get(name))
                        .chain(package.globals.get(name))
                        .any(|symbol| {
                            symbol.kind == SymbolKind::Binding
                                && !symbol.available_in_function_body()
                        })
                    {
                        return Err(namespace_variable_reference_failure(
                            &unit.source,
                            name,
                            node.span,
                        ));
                    }
                    return Err(failure(
                        &unit.source,
                        "S2013",
                        format!("unresolved name `{name}`"),
                        node.span,
                    ));
                }
            }
            SyntaxKind::NamespaceDeclaration
            | SyntaxKind::ImportDeclaration
            | SyntaxKind::ParameterList
            | SyntaxKind::Parameter
            | SyntaxKind::ForTarget
            | SyntaxKind::TypeExpression
            | SyntaxKind::UnionType
            | SyntaxKind::PrefixType
            | SyntaxKind::AppliedType
            | SyntaxKind::FunctionType => {}
            SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::AnonymousFunction
            | SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration => {
                let mut declaration_name_skipped = false;
                for child in &node.children {
                    if !declaration_name_skipped && child.kind == SyntaxKind::Name {
                        declaration_name_skipped = true;
                        continue;
                    }
                    visit(package, unit, child, false)?;
                }
            }
            SyntaxKind::CatchClause => {
                if let Some(descriptor) = node.children.first() {
                    visit(package, unit, descriptor, false)?;
                }
                if let Some(block) = node.children.last()
                    && block.kind == SyntaxKind::Block
                {
                    visit(package, unit, block, false)?;
                }
            }
            SyntaxKind::Argument => {
                for (index, child) in node.children.iter().enumerate() {
                    if index == 0 && node.children.len() > 1 && child.kind == SyntaxKind::Name {
                        continue;
                    }
                    visit(package, unit, child, false)?;
                }
            }
            SyntaxKind::MemberExpression
            | SyntaxKind::StaticMemberExpression
            | SyntaxKind::ConstructionExpression => {
                if let Some(receiver) = node.children.first() {
                    visit(package, unit, receiver, false)?;
                }
            }
            SyntaxKind::CallExpression => {
                for (index, child) in node.children.iter().enumerate() {
                    visit(package, unit, child, index == 0)?;
                }
            }
            _ => {
                for child in &node.children {
                    visit(package, unit, child, false)?;
                }
            }
        }
        Ok(())
    }

    for unit in &package.units {
        for node in &unit.tree.root.children {
            visit(package, unit, node, false)?;
        }
    }
    Ok(())
}
fn namespace_variable_reference_failure(
    source: &SourceFile,
    name: &str,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![
            Diagnostic::error(
                "S2026",
                format!("namespace variable `{name}` cannot cross a function boundary"),
                span,
            )
            .with_help(format!(
                "pass `{name}` as a parameter or return it from a function"
            )),
        ],
    }
}

fn namespace_variable_import_failure(
    source: &SourceFile,
    name: &str,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![
            Diagnostic::error(
                "S2026",
                format!("namespace variable `{name}` cannot be imported outside its namespace"),
                span,
            )
            .with_help(format!(
                "import a function that reads `{name}` and returns its value instead"
            )),
        ],
    }
}

fn binding_initializer(node: &SyntaxNode) -> Option<&SyntaxNode> {
    let name_index = node
        .children
        .iter()
        .position(|child| child.kind == SyntaxKind::Name)?;
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

#[expect(
    clippy::too_many_lines,
    reason = "the dependency graph construction and its diagnostics are one ordered validation pass"
)]
fn validate_initializer_dependencies(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    type Key = (u32, usize, usize);

    fn key(span: Span) -> Key {
        (span.file, span.start, span.end)
    }

    fn collect_reads(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        reads: &mut Vec<(Key, Span)>,
        functions: &mut BTreeSet<Key>,
    ) {
        if node.kind == SyntaxKind::Name {
            if let Some(symbol) =
                package.resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                && let Some(span) = symbol.declaration_span
            {
                if symbol.kind == SymbolKind::Binding && !symbol.global {
                    reads.push((key(span), node.span));
                } else if symbol.kind == SymbolKind::Function && functions.insert(key(span)) {
                    for owner in &package.units {
                        if let Some(function) = find_node_by_span(&owner.tree.root, span) {
                            collect_reads(package, owner, function, reads, functions);
                            break;
                        }
                    }
                }
            }
            return;
        }
        if matches!(
            node.kind,
            SyntaxKind::NamespaceDeclaration
                | SyntaxKind::ImportDeclaration
                | SyntaxKind::Parameter
                | SyntaxKind::ForTarget
                | SyntaxKind::TypeExpression
                | SyntaxKind::UnionType
                | SyntaxKind::PrefixType
                | SyntaxKind::AppliedType
                | SyntaxKind::FunctionType
        ) {
            return;
        }
        for child in &node.children {
            collect_reads(package, unit, child, reads, functions);
        }
    }
    fn unresolved_name_span(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        name: &str,
    ) -> Option<Span> {
        if node.kind == SyntaxKind::Name
            && node_text(&unit.source, node) == name
            && package
                .resolve_name_at(unit, node.span.start, name)
                .is_none()
        {
            return Some(node.span);
        }
        node.children
            .iter()
            .find_map(|child| unresolved_name_span(package, unit, child, name))
    }
    fn validate_self_references(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::Binding
            && let Some(initializer) = binding_initializer(node)
        {
            let declaration =
                declaration_from_syntax(unit, node).expect("ordinary binding has a name");
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            let direct_unresolved_self =
                unresolved_name_span(package, unit, initializer, &declaration.name);
            if let Some(span) = reads
                .iter()
                .find(|(dependency, _)| *dependency == key(node.span))
                .map(|(_, span)| *span)
                .or(direct_unresolved_self)
            {
                return Err(failure(
                    &unit.source,
                    "S2023",
                    format!(
                        "binding `{}` cannot reference itself in its initializer",
                        declaration.name
                    ),
                    span,
                ));
            }
        }
        for child in &node.children {
            validate_self_references(package, unit, child)?;
        }
        Ok(())
    }

    fn find_cycle(
        current: Key,
        edges: &BTreeMap<Key, Vec<(Key, Span)>>,
        path: &mut BTreeSet<Key>,
    ) -> Option<Span> {
        if !path.insert(current) {
            return None;
        }
        for &(dependency, span) in edges.get(&current).into_iter().flatten() {
            if path.contains(&dependency) {
                return Some(span);
            }
            if let Some(span) = find_cycle(dependency, edges, path) {
                return Some(span);
            }
        }
        path.remove(&current);
        None
    }
    for unit in &package.units {
        validate_self_references(package, unit, &unit.tree.root)?;
    }

    let mut edges = BTreeMap::<Key, Vec<(Key, Span)>>::new();
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if node.kind != SyntaxKind::Binding {
                continue;
            }
            let Some(declaration) = declaration_from_syntax(unit, node) else {
                continue;
            };
            let Some(initializer) = binding_initializer(node) else {
                continue;
            };
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            if reads
                .iter()
                .any(|(dependency, _)| *dependency == key(node.span))
            {
                let span = reads
                    .iter()
                    .find(|(dependency, _)| *dependency == key(node.span))
                    .expect("checked self-reference")
                    .1;
                return Err(failure(
                    &unit.source,
                    "S2023",
                    format!(
                        "binding `{}` cannot reference itself in its initializer",
                        declaration.name
                    ),
                    span,
                ));
            }
            if !declaration.global {
                edges.entry(key(node.span)).or_default().extend(reads);
            }
        }
    }
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if node.kind != SyntaxKind::Assignment {
                continue;
            }
            let Some(declaration) = declaration_from_syntax(unit, node) else {
                continue;
            };
            let name = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .expect("ordinary assignment has a name");
            if package
                .globals
                .get(&declaration.name)
                .is_some_and(|global| global.namespace == unit.namespace)
                && !declaration.global
            {
                return Err(SemanticFailure {
                    source: unit.source.clone(),
                    diagnostics: vec![
                        Diagnostic::error(
                            "S2021",
                            format!(
                                "plain namespace assignment cannot replace program-global binding `{}`",
                                declaration.name
                            ),
                            name.span,
                        )
                        .with_help(
                            "pass changing values through parameters and returns instead",
                        ),
                    ],
                });
            }
            let Some(target) = package.resolve_name_at(unit, name.span.start, &declaration.name)
            else {
                continue;
            };
            let Some(initializer) = binding_initializer(node) else {
                continue;
            };
            let Some(owner) = target.declaration_span else {
                continue;
            };
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            reads.retain(|(dependency, _)| *dependency != key(owner));
            edges.entry(key(owner)).or_default().extend(reads);
        }
    }
    for &start in edges.keys() {
        if let Some(span) = find_cycle(start, &edges, &mut BTreeSet::new()) {
            let source = package
                .units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .expect("dependency span belongs to a semantic unit");
            return Err(failure(
                &source.source,
                "S2024",
                "namespace binding initialization has a dependency cycle",
                span,
            ));
        }
    }
    Ok(())
}

fn find_node_by_span(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    (node.span == span).then_some(node).or_else(|| {
        node.children
            .iter()
            .find_map(|child| find_node_by_span(child, span))
    })
}

fn collect_evaluation_steps(source: &SourceFile, root: &SyntaxNode) -> Vec<EvaluationStep> {
    fn visit(
        source: &SourceFile,
        node: &SyntaxNode,
        conditional: bool,
        steps: &mut Vec<EvaluationStep>,
    ) {
        if node.kind == SyntaxKind::BinaryExpression
            && let [left, right] = node.children.as_slice()
        {
            visit(source, left, conditional, steps);
            let operator = source.text()[left.span.end..right.span.start].trim();
            let short_circuit = matches!(operator, "and" | "or");
            if short_circuit {
                steps.push(EvaluationStep {
                    kind: EvaluationKind::ShortCircuitRhs,
                    span: right.span,
                    conditional: true,
                });
            }
            visit(source, right, conditional || short_circuit, steps);
        } else {
            for child in &node.children {
                visit(source, child, conditional, steps);
            }
        }
        let kind = match node.kind {
            SyntaxKind::CallExpression => Some(EvaluationKind::Call),
            SyntaxKind::PostfixExpression => Some(EvaluationKind::PostfixUpdate),
            _ => None,
        };
        if let Some(kind) = kind {
            steps.push(EvaluationStep {
                kind,
                span: node.span,
                conditional,
            });
        }
    }

    let mut steps = Vec::new();
    visit(source, root, false, &mut steps);
    steps
}

fn validate_calls(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract,
            )
        })
        .collect();
    for unit in &package.units {
        let bindings = call_site_bindings(unit, None);
        validate_call_nodes(package, unit, &unit.tree.root, &contracts, None, &bindings)?;
    }
    Ok(())
}

fn validate_string_member_expression(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let member = (node.kind == SyntaxKind::MemberExpression)
        .then(|| node.children.get(1))
        .flatten()
        .map(|member| node_text(&unit.source, member));
    let call_member = (node.kind == SyntaxKind::CallExpression)
        .then(|| node.children.first())
        .flatten()
        .filter(|callee| callee.kind == SyntaxKind::MemberExpression)
        .and_then(|callee| callee.children.get(1))
        .map(|member| node_text(&unit.source, member));
    if member == Some("length") || matches!(call_member, Some("concat" | "join")) {
        infer_value_type(unit, node, bindings)?;
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "call validation remains one traversal so every call form shares lexical scope and contracts"
)]
fn validate_call_nodes<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
    contracts: &BTreeMap<(u32, usize, usize), &FunctionContract>,
    active_function: Option<&'a FunctionContract>,
    scoped_bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let entered_function = is_function_node(node)
        .then(|| {
            unit.functions
                .iter()
                .find(|contract| contract.span == node.span)
        })
        .flatten();
    let active_function = entered_function.or(active_function);
    let function_bindings =
        entered_function.map(|contract| call_site_bindings(unit, Some(contract)));
    let scoped_bindings = function_bindings.as_deref().unwrap_or(scoped_bindings);
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("await")
        && !active_function.is_some_and(|function| function.is_async)
    {
        return Err(failure(
            &unit.source,
            "T0028",
            "`await` is valid only inside an async callable",
            node.span,
        ));
    }

    validate_resolved_assignment(package, unit, node, contracts)?;
    validate_integer_coercion_call(unit, node, scoped_bindings)?;
    if node.kind == SyntaxKind::CallExpression
        && let Some(arguments) = node.children.get(1)
    {
        for argument in &arguments.children {
            let value = argument.children.last().unwrap_or(argument);
            infer_value_type(unit, value, scoped_bindings)?;
        }
    }
    if node.kind == SyntaxKind::CallExpression {
        let inferred = infer_value_type(unit, node, scoped_bindings)?;
        if inferred.is_none()
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
        {
            infer_member_value_type(unit, callee, scoped_bindings)?;
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::Name
        && package
            .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
            .is_some_and(|symbol| symbol.identity == "/core/output::print")
    {
        for argument in &arguments.children {
            let value = argument.children.last().unwrap_or(argument);
            validate_call_nodes(
                package,
                unit,
                value,
                contracts,
                active_function,
                scoped_bindings,
            )?;
            let value_type =
                transparent_value_type(infer_value_type(unit, value, scoped_bindings)?);
            if !matches!(
                value_type,
                Some(
                    ValueType::Scalar(
                        ScalarType::Bool
                            | ScalarType::Int
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
                            | ScalarType::String
                            | ScalarType::None
                    ) | ValueType::Descriptor(_)
                )
            ) {
                return Err(failure(
                    &unit.source,
                    "T0035",
                    format!(
                        "`print` requires a text-displayable scalar value, found {}",
                        value_type.map_or_else(|| "unknown".to_owned(), |ty| ty.to_string())
                    ),
                    value.span,
                ));
            }
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::Name
        && let Some(binding) = scoped_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, callee)
                && binding.is_visible_at(unit.source.id(), callee.span.start)
        })
        && let ValueType::Function(parameters, _) = &binding.value_type
    {
        if arguments.children.len() != parameters.len() {
            return Err(failure(
                &unit.source,
                "T0012",
                format!(
                    "callable expects {} arguments, found {}",
                    parameters.len(),
                    arguments.children.len()
                ),
                arguments.span,
            ));
        }
        for (argument, expected) in arguments.children.iter().zip(parameters) {
            if argument.children.len() > 1 {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    "calls through function values use positional arguments",
                    argument.span,
                ));
            }
            let value = argument.children.last().unwrap_or(argument);
            if let Some(actual) = infer_value_type(unit, value, scoped_bindings)? {
                validate_value_destination(
                    &unit.source,
                    &unit.objects,
                    "callable argument",
                    expected.value_type(),
                    actual,
                    value,
                    "T0012",
                )?;
            }
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
    {
        let contract = match callee.kind {
            SyntaxKind::Name => package
                .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
                .filter(|symbol| symbol.kind == SymbolKind::Function)
                .and_then(|symbol| symbol.declaration_span)
                .and_then(|declaration_span| {
                    contracts
                        .get(&(
                            declaration_span.file,
                            declaration_span.start,
                            declaration_span.end,
                        ))
                        .copied()
                }),
            SyntaxKind::MemberExpression => match callee.children.as_slice() {
                [receiver, member] => infer_value_type(unit, receiver, scoped_bindings)
                    .ok()
                    .flatten()
                    .and_then(|value_type| {
                        let ValueType::Object(identity) = value_type else {
                            return None;
                        };
                        method_contract(package, &identity, node_text(&unit.source, member), false)
                    }),
                _ => None,
            },
            SyntaxKind::StaticMemberExpression => match callee.children.as_slice() {
                [receiver, member] => {
                    class_designator_identity(unit, receiver).and_then(|identity| {
                        method_contract(package, &identity, node_text(&unit.source, member), true)
                    })
                }
                _ => None,
            },
            SyntaxKind::ConstructionExpression => construction_contract(package, unit, callee),
            _ => None,
        };
        if let Some(contract) = contract {
            validate_call_arguments(unit, arguments, contract, scoped_bindings)?;
        }
    }
    if let [target, collection, block] = node.children.as_slice()
        && node.kind == SyntaxKind::ForStatement
        && target.kind == SyntaxKind::ForTarget
    {
        validate_call_nodes(
            package,
            unit,
            collection,
            contracts,
            active_function,
            scoped_bindings,
        )?;
        let item_type = infer_value_type(unit, collection, scoped_bindings)?
            .and_then(iterable_item_type)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0016",
                    "collection iteration requires an iterable value",
                    collection.span,
                )
            })?;
        let mut loop_bindings = scoped_bindings.to_vec();
        loop_bindings.extend(iteration_target_bindings(
            unit,
            target,
            collection.span.end,
            block.span,
            item_type,
        )?);
        validate_call_nodes(
            package,
            unit,
            block,
            contracts,
            active_function,
            &loop_bindings,
        )?;
        return Ok(());
    }
    validate_string_member_expression(unit, node, scoped_bindings)?;
    validate_coercion_family_expression(unit, node)?;
    for (index, child) in node.children.iter().enumerate() {
        if node.kind == SyntaxKind::CallExpression
            && index == 0
            && let Some((source, _)) = integer_coercion_call(&unit.source, child)
        {
            validate_call_nodes(
                package,
                unit,
                source,
                contracts,
                active_function,
                scoped_bindings,
            )?;
            continue;
        }
        validate_call_nodes(
            package,
            unit,
            child,
            contracts,
            active_function,
            scoped_bindings,
        )?;
    }
    Ok(())
}

fn validate_integer_coercion_call(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::CallExpression {
        infer_integer_coercion_type(unit, node, bindings)?;
    }
    Ok(())
}

fn validate_coercion_family_expression(
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::MemberExpression && coercion_family_receiver(unit, node) {
        return Err(failure(
            &unit.source,
            "T0018",
            "`.coerce` and its policy members are not storable values before bound methods exist",
            node.span,
        ));
    }
    Ok(())
}

fn validate_resolved_assignment(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contracts: &BTreeMap<(u32, usize, usize), &FunctionContract>,
) -> Result<(), SemanticFailure> {
    if !matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
        return Ok(());
    }
    let Some(name_node) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
    else {
        return Ok(());
    };
    let Some(initializer) = node.children.iter().rev().find(|child| {
        child.span != name_node.span
            && !matches!(
                child.kind,
                SyntaxKind::Visibility
                    | SyntaxKind::DeclarationQualifier
                    | SyntaxKind::TypeExpression
            )
    }) else {
        return Ok(());
    };
    let actual = if let Some(actual) = resolved_call_type(package, unit, initializer, contracts) {
        actual
    } else if let Some(actual) =
        infer_collection_call_type(unit, initializer, &unit.typed_bindings)?
    {
        actual
    } else if initializer.kind != SyntaxKind::CallExpression {
        let Some(actual) = infer_value_type(unit, initializer, &unit.typed_bindings)? else {
            return Ok(());
        };
        actual
    } else {
        return Ok(());
    };
    let name = node_text(&unit.source, name_node);
    let Some(expected) = unit
        .typed_bindings
        .iter()
        .rev()
        .find(|binding| {
            binding.name == name
                && if node.kind == SyntaxKind::Binding {
                    binding.span == node.span
                } else {
                    binding.is_visible_at(unit.source.id(), node.span.start)
                }
        })
        .map(|binding| binding.value_type.clone())
    else {
        return Ok(());
    };
    validate_value_destination(
        &unit.source,
        &unit.objects,
        name,
        expected,
        actual,
        initializer,
        "T0002",
    )
}

fn resolved_call_type(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contracts: &BTreeMap<(u32, usize, usize), &FunctionContract>,
) -> Option<ValueType> {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| resolved_call_type(package, unit, child, contracts));
    }
    let [callee, _arguments] = node.children.as_slice() else {
        return None;
    };
    if node.kind != SyntaxKind::CallExpression || callee.kind != SyntaxKind::Name {
        return None;
    }
    let symbol =
        package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
    let declaration = symbol.declaration_span?;
    let contract = contracts.get(&(declaration.file, declaration.start, declaration.end))?;
    let result = ElementType::new(
        contract
            .return_type
            .clone()
            .unwrap_or(ValueType::Scalar(ScalarType::None)),
    );
    Some(if contract.is_async {
        ValueType::Task(result)
    } else {
        result.value_type()
    })
}

fn validate_call_arguments(
    unit: &SemanticUnit,
    arguments: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let mut bound = BTreeSet::new();
    let mut positional = 0;
    let mut named_seen = false;
    for argument in &arguments.children {
        let name = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let parameter = if let Some(name) = name {
            named_seen = true;
            let name_text = node_text(&unit.source, name);
            contract
                .parameters
                .iter()
                .find(|parameter| parameter.name == name_text)
                .ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0012",
                        format!(
                            "function `{}` has no parameter named `{name_text}`",
                            contract.name
                        ),
                        name.span,
                    )
                })?
        } else {
            if named_seen {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    "positional arguments must precede named arguments",
                    argument.span,
                ));
            }
            let parameter = contract.parameters.get(positional).ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0012",
                    format!("too many arguments for function `{}`", contract.name),
                    argument.span,
                )
            })?;
            positional += 1;
            parameter
        };
        if !bound.insert(parameter.name.as_str()) {
            return Err(failure(
                &unit.source,
                "T0012",
                format!("parameter `{}` is bound more than once", parameter.name),
                argument.span,
            ));
        }
        let value = argument.children.last().unwrap_or(argument);
        if let Some(expected) = parameter.value_type.clone() {
            if contextual_collection_constructor_matches(unit, value, &expected, bindings) {
                validate_collection_constructor_value(
                    unit,
                    value,
                    &expected,
                    &parameter.name,
                    bindings,
                )?;
            } else if let Some(actual) = infer_value_type(unit, value, bindings)? {
                validate_value_destination(
                    &unit.source,
                    &unit.objects,
                    &parameter.name,
                    expected,
                    actual,
                    value,
                    "T0012",
                )?;
            }
        }
    }
    if let Some(missing) = contract
        .parameters
        .iter()
        .find(|parameter| !parameter.optional && !bound.contains(parameter.name.as_str()))
    {
        return Err(failure(
            &unit.source,
            "T0012",
            format!("missing required argument `{}`", missing.name),
            arguments.span,
        ));
    }
    Ok(())
}

fn call_site_bindings(
    unit: &SemanticUnit,
    active_function: Option<&FunctionContract>,
) -> Vec<TypedBinding> {
    let mut bindings = unit
        .typed_bindings
        .iter()
        .filter(|binding| {
            let owner = unit
                .functions
                .iter()
                .filter(|function| {
                    function.span.file == binding.span.file
                        && function.span.start <= binding.span.start
                        && binding.span.end <= function.span.end
                })
                .min_by_key(|function| function.span.end - function.span.start);
            owner
                .is_none_or(|owner| active_function.is_some_and(|active| active.span == owner.span))
        })
        .cloned()
        .collect::<Vec<_>>();
    if let Some(function) = active_function {
        bindings.extend(function.parameters.iter().filter_map(|parameter| {
            parameter.value_type.clone().map(|value_type| TypedBinding {
                name: parameter.name.clone(),
                span: parameter.span,
                visible_from: parameter.span.start,
                scope: Some(function.span),
                value_type,
                destination_arms: Vec::new(),
                storage_type: None,
                mutable: false,
            })
        }));
    }
    bindings
}

fn descriptor_construct_alias_history(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> BTreeMap<String, Vec<DescriptorAlias>> {
    let mut aliases = package
        .descriptor_constructs
        .iter()
        .filter_map(|(name, symbol)| Some((name.clone(), symbol.descriptor_type()?)))
        .collect::<BTreeMap<_, _>>();
    if let Some(namespace) = package.namespaces.get(&unit.namespace) {
        aliases.extend(
            namespace
                .symbols
                .iter()
                .filter_map(|(name, symbol)| Some((name.clone(), symbol.descriptor_type()?))),
        );
    }
    aliases
        .into_iter()
        .map(|(name, value_type)| {
            (
                name,
                vec![DescriptorAlias {
                    visible_from: 0,
                    scope: None,
                    value_type,
                }],
            )
        })
        .collect()
}

#[expect(
    clippy::too_many_lines,
    reason = "object analysis assembles one complete declaration contract"
)]
fn analyze_object_contracts(
    unit: &SemanticUnit,
    aliases: &BTreeMap<String, Vec<DescriptorAlias>>,
    visible_objects: &BTreeMap<String, ObjectIdentity>,
) -> Result<Vec<ObjectContract>, SemanticFailure> {
    let visible = visible_descriptor_aliases(aliases, unit.source.id(), 0);
    let mut objects = Vec::new();
    for node in &unit.tree.root.children {
        let kind = match node.kind {
            SyntaxKind::ClassDeclaration => ObjectKind::Class,
            SyntaxKind::InterfaceDeclaration => ObjectKind::Interface,
            SyntaxKind::TraitDeclaration => ObjectKind::Trait,
            _ => continue,
        };
        let name = declaration_name(node, &unit.source).ok_or_else(|| {
            failure(
                &unit.source,
                "T0053",
                "object declaration requires a name",
                node.span,
            )
        })?;
        let clause_identities = |clause_kind| {
            node.children
                .iter()
                .find(|child| child.kind == clause_kind)
                .map(|clause| {
                    clause
                        .children
                        .iter()
                        .map(|name| {
                            let name = node_text(&unit.source, name);
                            visible_objects.get(name).cloned().unwrap_or_else(|| {
                                if name == "throwable" {
                                    ObjectIdentity::new("/core/errors", name)
                                } else {
                                    ObjectIdentity::new(&unit.namespace, name)
                                }
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        let base = clause_identities(SyntaxKind::ExtendsClause)
            .into_iter()
            .next();
        let interfaces = clause_identities(SyntaxKind::ImplementsClause);
        let traits = clause_identities(SyntaxKind::UsesClause);
        let mut fields = Vec::new();
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            for field in block
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::Binding)
            {
                let Some(field_name) = field
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
                else {
                    continue;
                };
                let initializer = field.children.last().filter(|child| {
                    child.span != field_name.span
                        && !matches!(
                            child.kind,
                            SyntaxKind::Visibility
                                | SyntaxKind::DeclarationQualifier
                                | SyntaxKind::TypeExpression
                        )
                });
                let value_type = if let Some(type_node) = field
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::TypeExpression)
                {
                    declared_value_type_with_visible_objects(
                        unit,
                        type_node,
                        &visible,
                        visible_objects,
                    )?
                } else if let Some(initializer) = initializer {
                    infer_value_type(unit, initializer, &[])?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0065",
                            "object field type cannot be inferred",
                            field.span,
                        )
                    })?
                } else {
                    return Err(failure(
                        &unit.source,
                        "T0066",
                        "object fields require a type or initializer",
                        field.span,
                    ));
                };
                if kind == ObjectKind::Class
                    && initializer.is_none()
                    && !matches!(
                        value_type,
                        ValueType::PlatformStreamHandle
                            | ValueType::PlatformResourceHandle
                            | ValueType::FilesystemAuthority
                    )
                {
                    return Err(failure(
                        &unit.source,
                        "T0061",
                        format!(
                            "class field `{}` requires an initializer",
                            node_text(&unit.source, field_name)
                        ),
                        field.span,
                    ));
                }
                fields.push(ObjectField {
                    name: node_text(&unit.source, field_name).to_owned(),
                    span: field.span,
                    value_type,
                    is_static: field.children.iter().any(|child| {
                        child.kind == SyntaxKind::DeclarationQualifier
                            && node_text(&unit.source, child) == "static"
                    }),
                });
            }
        }
        let resource_owning = kind == ObjectKind::Class
            && fields.iter().any(|field| {
                matches!(
                    field.value_type,
                    ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle
                )
            });
        objects.push(ObjectContract {
            identity: ObjectIdentity::new(&unit.namespace, &name),
            name,
            span: node.span,
            kind,
            resource_owning,
            base,
            interfaces,
            traits,
            fields,
        });
    }
    for object in &objects {
        let require_kind = |identity: &ObjectIdentity, expected: ObjectKind, role: &str| {
            let local = objects
                .iter()
                .find(|candidate| candidate.identity == *identity);
            let valid = (expected == ObjectKind::Interface
                && identity == &ObjectIdentity::new("/core/errors", "throwable"))
                || local.is_some_and(|candidate| candidate.kind == expected)
                || local.is_none() && visible_objects.values().any(|visible| visible == identity);
            valid.then_some(()).ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0054",
                    format!(
                        "`{}` does not resolve to a {role}",
                        diagnostic_object_identity(&objects, identity)
                    ),
                    object.span,
                )
            })
        };
        if let Some(base) = &object.base {
            require_kind(base, ObjectKind::Class, "class")?;
        }
        for interface in &object.interfaces {
            require_kind(interface, ObjectKind::Interface, "interface")?;
        }
        for used_trait in &object.traits {
            require_kind(used_trait, ObjectKind::Trait, "trait")?;
        }
    }
    Ok(objects)
}

fn value_type_owns_resource(
    value_type: &ValueType,
    resource_identities: &BTreeSet<String>,
) -> bool {
    match value_type {
        ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
        ValueType::Object(identity) => resource_identities.contains(&identity.qualified()),
        ValueType::Optional(inner) => value_type_owns_resource(inner, resource_identities),
        ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::UnorderedSet(item)
        | ValueType::Task(item)
        | ValueType::ScopedTask(item)
        | ValueType::TaskOutcome(item)
        | ValueType::Reference(item)
        | ValueType::SharedReference(item) => {
            value_type_owns_resource(&item.value_type(), resource_identities)
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_owns_resource(&key.value_type(), resource_identities)
                || value_type_owns_resource(&value.value_type(), resource_identities)
        }
        _ => false,
    }
}

fn value_type_is_resource_container(
    value_type: &ValueType,
    resource_identities: &BTreeSet<String>,
) -> bool {
    matches!(
        value_type,
        ValueType::List(_)
            | ValueType::Map(_, _)
            | ValueType::Set(_)
            | ValueType::Tuple(_, _)
            | ValueType::UnorderedMap(_, _)
            | ValueType::UnorderedSet(_)
    ) && value_type_owns_resource(value_type, resource_identities)
}

fn propagate_resource_ownership(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    loop {
        let resource_identities = package
            .units
            .iter()
            .flat_map(|unit| {
                unit.objects
                    .iter()
                    .filter(|object| object.resource_owning)
                    .filter_map(|object| {
                        package
                            .resolve_name_at(unit, object.span.start, &object.name)
                            .map(|symbol| symbol.identity.clone())
                    })
            })
            .collect::<BTreeSet<_>>();
        let mut newly_resource_owning = Vec::new();
        for (unit_index, unit) in package.units.iter().enumerate() {
            for (object_index, object) in unit.objects.iter().enumerate() {
                if object.kind != ObjectKind::Class || object.resource_owning {
                    continue;
                }
                let owns_field_resource = object
                    .fields
                    .iter()
                    .any(|field| value_type_owns_resource(&field.value_type, &resource_identities));
                let owns_base_resource = object
                    .base
                    .as_ref()
                    .is_some_and(|base| resource_identities.contains(&base.qualified()));
                if owns_field_resource || owns_base_resource {
                    newly_resource_owning.push((unit_index, object_index));
                }
            }
        }
        if newly_resource_owning.is_empty() {
            break;
        }
        for (unit_index, object_index) in newly_resource_owning {
            package.units[unit_index].objects[object_index].resource_owning = true;
        }
    }

    let resource_identities = package
        .units
        .iter()
        .flat_map(|unit| {
            unit.objects
                .iter()
                .filter(|object| object.resource_owning)
                .filter_map(|object| {
                    package
                        .resolve_name_at(unit, object.span.start, &object.name)
                        .map(|symbol| symbol.identity.clone())
                })
        })
        .collect::<BTreeSet<_>>();

    for unit in &package.units {
        for object in &unit.objects {
            if object.resource_owning
                && (object.base.is_some()
                    || !object.interfaces.is_empty()
                    || !object.traits.is_empty())
            {
                return Err(failure(
                    &unit.source,
                    "T0098",
                    "a resource-owning class cannot extend, implement, or use copyable object contracts",
                    object.span,
                ));
            }
            if let Some(field) = object.fields.iter().find(|field| {
                value_type_is_resource_container(&field.value_type, &resource_identities)
            }) {
                return Err(failure(
                    &unit.source,
                    "T0101",
                    "resource-owning values in collections are not supported yet",
                    field.span,
                ));
            }
        }
    }
    Ok(())
}
fn validate_resource_collection_types(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    let resource_identities = package
        .units
        .iter()
        .flat_map(|unit| {
            unit.objects
                .iter()
                .filter(|object| object.resource_owning)
                .filter_map(|object| {
                    package
                        .resolve_name_at(unit, object.span.start, &object.name)
                        .map(|symbol| symbol.identity.clone())
                })
        })
        .collect::<BTreeSet<_>>();
    for unit in &package.units {
        if let Some(binding) = unit.typed_bindings.iter().find(|binding| {
            value_type_is_resource_container(&binding.value_type, &resource_identities)
        }) {
            return Err(failure(
                &unit.source,
                "T0101",
                "resource-owning values in collections are not supported yet",
                binding.span,
            ));
        }
        for function in &unit.functions {
            if let Some(parameter) = function.parameters.iter().find(|parameter| {
                parameter.value_type.as_ref().is_some_and(|value_type| {
                    value_type_is_resource_container(value_type, &resource_identities)
                })
            }) {
                return Err(failure(
                    &unit.source,
                    "T0101",
                    "resource-owning values in collections are not supported yet",
                    parameter.span,
                ));
            }
            if function.return_type.as_ref().is_some_and(|value_type| {
                value_type_is_resource_container(value_type, &resource_identities)
            }) {
                return Err(failure(
                    &unit.source,
                    "T0101",
                    "resource-owning values in collections are not supported yet",
                    function.span,
                ));
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "object conformance checks inheritance, interfaces, and trait conflicts together"
)]
fn validate_object_conformance(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn same_signature(left: &FunctionContract, right: &FunctionContract) -> bool {
        left.parameters.len() == right.parameters.len()
            && left
                .parameters
                .iter()
                .zip(&right.parameters)
                .all(|(left, right)| {
                    left.value_type == right.value_type
                        && left.optional == right.optional
                        && left.mutable == right.mutable
                })
            && left.return_type == right.return_type
            && (!right.throws || left.throws)
            && left.is_async == right.is_async
            && left.consumes_receiver == right.consumes_receiver
    }

    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| method.owner.as_deref() == Some(&object.name) && method.name == name)
            .or_else(|| {
                object
                    .base
                    .as_ref()
                    .and_then(|base| {
                        unit.objects
                            .iter()
                            .find(|candidate| candidate.identity == *base)
                    })
                    .and_then(|base| effective_method(unit, base, name))
            })
    }

    for unit in &package.units {
        for object in unit
            .objects
            .iter()
            .filter(|object| object.kind == ObjectKind::Class)
        {
            let declaration_unit = package
                .units
                .iter()
                .find(|candidate| candidate.source.id() == object.span.file)
                .expect("object declaration source must belong to the semantic package");
            let object = declaration_unit
                .objects
                .iter()
                .find(|candidate| candidate.identity == object.identity)
                .expect("object identity must resolve in its declaration unit");
            for interface_identity in &object.interfaces {
                let Some(resolved_interface) =
                    package.resolve_name(&interface_identity.namespace, &interface_identity.name)
                else {
                    return Err(failure(
                        &declaration_unit.source,
                        "T0001",
                        format!(
                            "interface `{}` implemented by `{}` does not resolve",
                            diagnostic_object_identity(
                                &declaration_unit.objects,
                                interface_identity
                            ),
                            object.name
                        ),
                        object.span,
                    ));
                };
                if resolved_interface.identity == "/core/errors::throwable" {
                    let has_message = object.fields.iter().any(|field| {
                        field.name == "message"
                            && field.value_type == ValueType::Scalar(ScalarType::String)
                    });
                    if !has_message {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0062",
                            format!(
                                "class `{}` must provide a `message string` field to implement `throwable`",
                                object.name
                            ),
                            object.span,
                        ));
                    }
                    let Some(render) = effective_method(declaration_unit, object, "render") else {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0062",
                            format!(
                                "class `{}` does not implement interface member `throwable.render`",
                                object.name
                            ),
                            object.span,
                        ));
                    };
                    let required_render = FunctionContract {
                        name: "render".to_owned(),
                        span: object.span,
                        owner: Some("/core/errors::throwable".to_owned()),
                        captures: Vec::new(),
                        parameters: Vec::new(),
                        is_static: false,
                        return_type: Some(ValueType::Scalar(ScalarType::String)),
                        exported: true,
                        thrown_types: Vec::new(),
                        escaping_throwables: BTreeSet::new(),
                        throws: false,
                        is_async: false,
                        mutates_receiver: false,
                        consumes_receiver: false,
                    };
                    if !same_signature(&required_render, render) {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0067",
                            format!(
                                "class `{}` implements `throwable.render` with an incompatible signature",
                                object.name
                            ),
                            render.span,
                        ));
                    }
                    continue;
                }
                let interface_unit = package
                    .units
                    .iter()
                    .find(|candidate| {
                        candidate.namespace == resolved_interface.namespace
                            && candidate.objects.iter().any(|candidate| {
                                candidate.name == resolved_interface.name
                                    && candidate.kind == ObjectKind::Interface
                            })
                    })
                    .expect("resolved interface must have a semantic declaration");
                let interface = interface_unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.name == resolved_interface.name)
                    .expect("resolved interface must have an object contract");
                for required in interface_unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&interface.name))
                {
                    let Some(actual) = effective_method(declaration_unit, object, &required.name)
                    else {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0062",
                            format!(
                                "class `{}` does not implement interface member `{}.{}`",
                                object.name, interface.name, required.name
                            ),
                            object.span,
                        ));
                    };
                    if !same_signature(required, actual) {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0067",
                            format!(
                                "class `{}` implements `{}.{}` with an incompatible signature",
                                object.name, interface.name, required.name
                            ),
                            actual.span,
                        ));
                    }
                }
            }

            let own_methods = declaration_unit
                .functions
                .iter()
                .filter(|method| method.owner.as_deref() == Some(&object.name))
                .map(|method| method.name.as_str())
                .collect::<BTreeSet<_>>();
            let own_fields = object
                .fields
                .iter()
                .map(|field| field.name.as_str())
                .collect::<BTreeSet<_>>();
            let mut providers = BTreeMap::<&str, Vec<&str>>::new();
            for trait_name in &object.traits {
                let used_trait = declaration_unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.identity == *trait_name)
                    .expect("object-kind validation must resolve used traits");
                for method in declaration_unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&used_trait.name))
                {
                    providers
                        .entry(method.name.as_str())
                        .or_default()
                        .push(used_trait.name.as_str());
                }
                for field in &used_trait.fields {
                    providers
                        .entry(field.name.as_str())
                        .or_default()
                        .push(used_trait.name.as_str());
                }
            }
            if let Some((member, traits)) = providers.iter().find(|(member, traits)| {
                traits.len() > 1
                    && !own_methods.contains(**member)
                    && !own_fields.contains(**member)
            }) {
                return Err(failure(
                    &declaration_unit.source,
                    "T0063",
                    format!(
                        "class `{}` inherits conflicting member `{member}` from traits {}",
                        object.name,
                        traits.join(", ")
                    ),
                    object.span,
                ));
            }
        }
    }
    Ok(())
}

fn propagate_interface_receiver_mutability(package: &mut SemanticPackage) {
    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| method.owner.as_deref() == Some(&object.name) && method.name == name)
            .or_else(|| {
                object
                    .base
                    .as_ref()
                    .and_then(|base| {
                        unit.objects
                            .iter()
                            .find(|candidate| candidate.identity == *base)
                    })
                    .and_then(|base| effective_method(unit, base, name))
            })
            .or_else(|| {
                object.traits.iter().find_map(|used_trait| {
                    unit.objects
                        .iter()
                        .find(|candidate| candidate.identity == *used_trait)
                        .and_then(|used_trait| effective_method(unit, used_trait, name))
                })
            })
    }

    let mut mutating = BTreeSet::<(u32, usize, usize, String)>::new();
    for unit in &package.units {
        for class in unit
            .objects
            .iter()
            .filter(|object| object.kind == ObjectKind::Class)
        {
            for interface_name in &class.interfaces {
                let Some(interface) = unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.identity == *interface_name)
                else {
                    continue;
                };
                for required in unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&interface.name))
                {
                    if effective_method(unit, class, &required.name)
                        .is_some_and(|actual| actual.mutates_receiver)
                    {
                        mutating.insert((
                            interface.span.file,
                            interface.span.start,
                            interface.span.end,
                            required.name.clone(),
                        ));
                    }
                }
            }
        }
    }

    for unit in &mut package.units {
        for method in &mut unit.functions {
            let Some(owner) = method.owner.as_deref() else {
                continue;
            };
            let Some(interface) = unit
                .objects
                .iter()
                .find(|object| object.kind == ObjectKind::Interface && object.name == owner)
            else {
                continue;
            };
            if mutating.contains(&(
                interface.span.file,
                interface.span.start,
                interface.span.end,
                method.name.clone(),
            )) {
                method.mutates_receiver = true;
            }
        }
    }
}
#[expect(
    clippy::too_many_lines,
    reason = "receiver-consumption inference keeps its source-ownership helpers scoped to one fixed-point pass"
)]
fn infer_receiver_consumption(package: &mut SemanticPackage) {
    fn owns_resource(package: &SemanticPackage, value_type: &ValueType) -> bool {
        match value_type {
            ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
            ValueType::Object(name) => resolved_object_span(package, name)
                .and_then(|span| {
                    package
                        .units
                        .iter()
                        .flat_map(|candidate| &candidate.objects)
                        .find(|object| object.span == span)
                })
                .is_some_and(|object| object.resource_owning),
            _ => false,
        }
    }

    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object_name: &str,
        method_name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| {
                method.owner.as_deref() == Some(object_name) && method.name == method_name
            })
            .or_else(|| {
                unit.objects
                    .iter()
                    .find(|object| object.name == object_name)
                    .and_then(|object| object.base.as_ref())
                    .and_then(|base| unit.objects.iter().find(|object| object.identity == *base))
                    .and_then(|base| effective_method(unit, &base.name, method_name))
            })
    }

    fn receiver_resource_expression(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        contract: &FunctionContract,
        expression: &SyntaxNode,
    ) -> bool {
        if expression.kind == SyntaxKind::Name && node_text(&unit.source, expression) == "this" {
            return contract.owner.as_deref().is_some_and(|owner| {
                unit.objects
                    .iter()
                    .find(|object| object.name == owner)
                    .is_some_and(|object| object.resource_owning)
            });
        }
        let [receiver, member] = expression.children.as_slice() else {
            return expression
                .children
                .iter()
                .any(|child| receiver_resource_expression(package, unit, contract, child));
        };
        if expression.kind != SyntaxKind::MemberExpression
            || receiver.kind != SyntaxKind::Name
            || node_text(&unit.source, receiver) != "this"
        {
            return false;
        }
        contract.owner.as_deref().is_some_and(|owner| {
            unit.objects
                .iter()
                .find(|object| object.name == owner)
                .and_then(|object| {
                    object
                        .fields
                        .iter()
                        .find(|field| field.name == node_text(&unit.source, member))
                })
                .is_some_and(|field| owns_resource(package, &field.value_type))
        })
    }

    fn callable_parameters<'a>(
        package: &'a SemanticPackage,
        unit: &'a SemanticUnit,
        call: &SyntaxNode,
    ) -> Option<&'a [ParameterContract]> {
        let callee = call.children.first()?;
        if callee.kind != SyntaxKind::Name {
            return None;
        }
        let symbol =
            package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
        let declaration = symbol.declaration_span?;
        if symbol.kind == SymbolKind::Class {
            let object = package
                .units
                .iter()
                .flat_map(|candidate| &candidate.objects)
                .find(|object| object.span == declaration)?;
            return package
                .units
                .iter()
                .flat_map(|candidate| &candidate.functions)
                .find(|function| {
                    function.owner.as_deref() == Some(&object.name) && function.name == "construct"
                })
                .map(|function| function.parameters.as_slice());
        }
        package
            .units
            .iter()
            .flat_map(|candidate| &candidate.functions)
            .find(|function| function.span == declaration)
            .map(|function| function.parameters.as_slice())
    }

    fn node_consumes_receiver(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        contract: &FunctionContract,
        node: &SyntaxNode,
    ) -> bool {
        if node.kind == SyntaxKind::CallExpression {
            let Some(callee) = node.children.first() else {
                return false;
            };
            let arguments = node
                .children
                .get(1)
                .map_or(&[][..], |arguments| arguments.children.as_slice());
            if callee.kind == SyntaxKind::Name {
                let identity = package
                    .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
                    .map(Symbol::compiler_identity);
                if matches!(
                    identity,
                    Some("intrinsic:streams::close" | "intrinsic:streams::release")
                ) && arguments
                    .first()
                    .and_then(|argument| argument.children.last())
                    .is_some_and(|argument| {
                        receiver_resource_expression(package, unit, contract, argument)
                    })
                {
                    return true;
                }
                if let Some(parameters) = callable_parameters(package, unit, node)
                    && arguments
                        .iter()
                        .zip(parameters)
                        .any(|(argument, parameter)| {
                            argument.children.last().is_some_and(|argument| {
                                parameter
                                    .value_type
                                    .as_ref()
                                    .is_some_and(|value_type| owns_resource(package, value_type))
                                    && receiver_resource_expression(
                                        package, unit, contract, argument,
                                    )
                            })
                        })
                {
                    return true;
                }
            } else if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
                && matches!(
                    infer_value_type(unit, receiver, &unit.typed_bindings),
                    Ok(Some(ValueType::Object(object_name)))
                        if effective_method(
                            unit,
                            &object_name.name,
                            node_text(&unit.source, member)
                        )
                        .is_some_and(|method| method.consumes_receiver)
                )
                && receiver_resource_expression(package, unit, contract, receiver)
            {
                return true;
            }
        }
        node.children
            .iter()
            .any(|child| node_consumes_receiver(package, unit, contract, child))
    }

    loop {
        let mut newly_consuming = BTreeSet::new();
        for unit in &package.units {
            for contract in &unit.functions {
                if !contract.consumes_receiver
                    && contract.owner.is_some()
                    && contract.name != "destruct"
                    && find_node_by_span(&unit.tree.root, contract.span)
                        .is_some_and(|node| node_consumes_receiver(package, unit, contract, node))
                {
                    newly_consuming.insert((
                        contract.span.file,
                        contract.span.start,
                        contract.span.end,
                    ));
                }
            }
        }
        if newly_consuming.is_empty() {
            break;
        }
        for unit in &mut package.units {
            for contract in &mut unit.functions {
                if newly_consuming.contains(&(
                    contract.span.file,
                    contract.span.start,
                    contract.span.end,
                )) {
                    contract.consumes_receiver = true;
                }
            }
        }
    }

    let mut consuming_interfaces = BTreeSet::<((u32, usize, usize), String)>::new();
    for unit in &package.units {
        for class in unit
            .objects
            .iter()
            .filter(|object| object.kind == ObjectKind::Class)
        {
            for interface_name in &class.interfaces {
                let Some(interface) = unit.objects.iter().find(|object| {
                    object.kind == ObjectKind::Interface && object.identity == *interface_name
                }) else {
                    continue;
                };
                for required in unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&interface.name))
                {
                    if effective_method(unit, &class.name, &required.name)
                        .is_some_and(|actual| actual.consumes_receiver)
                    {
                        consuming_interfaces.insert((
                            (
                                interface.span.file,
                                interface.span.start,
                                interface.span.end,
                            ),
                            required.name.clone(),
                        ));
                    }
                }
            }
        }
    }
    for unit in &mut package.units {
        for method in &mut unit.functions {
            if method.owner.as_deref().is_some_and(|owner| {
                unit.objects
                    .iter()
                    .find(|object| object.kind == ObjectKind::Interface && object.name == owner)
                    .is_some_and(|interface| {
                        consuming_interfaces.contains(&(
                            (
                                interface.span.file,
                                interface.span.start,
                                interface.span.end,
                            ),
                            method.name.clone(),
                        ))
                    })
            }) {
                method.consumes_receiver = true;
            }
        }
    }
}

fn analyze_types(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for index in 0..package.units.len() {
        let objects = {
            let unit = &package.units[index];
            let alias_history = descriptor_construct_alias_history(package, unit);
            let visible_objects = package
                .namespaces
                .get(&unit.namespace)
                .into_iter()
                .flat_map(|namespace| &namespace.symbols)
                .filter(|(_, symbol)| {
                    matches!(
                        symbol.kind,
                        SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                    )
                })
                .map(|(visible_name, symbol)| {
                    (
                        visible_name.clone(),
                        ObjectIdentity::new(&symbol.namespace, &symbol.name),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            analyze_object_contracts(unit, &alias_history, &visible_objects)?
        };
        package.units[index].objects = objects;
    }
    populate_object_aliases(package);
    propagate_resource_ownership(package)?;
    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let mut alias_history = descriptor_construct_alias_history(package, unit);
        let mut functions = Vec::new();
        collect_type_declarations(
            unit,
            &unit.tree.root,
            &mut alias_history,
            &mut functions,
            None,
        )?;
        package.units[index].descriptor_aliases = alias_history;
        package.units[index].functions = functions;
    }
    populate_namespace_function_contracts(package);
    populate_function_aliases(package);
    populate_function_type_dependencies(package);
    propagate_interface_receiver_mutability(package);
    validate_descriptor_value_uses(package)?;

    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let mut visible_bindings = Vec::new();
        let mut bindings = Vec::new();
        collect_typed_bindings(
            unit,
            &unit.tree.root,
            &mut visible_bindings,
            &mut bindings,
            None,
        )?;
        package.units[index].typed_bindings = bindings;
    }
    validate_resource_collection_types(package)?;
    infer_receiver_consumption(package);
    validate_object_conformance(package)?;
    populate_closure_captures(package);
    Ok(())
}
fn populate_closure_captures(package: &mut SemanticPackage) {
    fn collect(
        unit: &SemanticUnit,
        closure: Span,
        node: &SyntaxNode,
        captures: &mut BTreeSet<String>,
        declaration_name: bool,
    ) {
        if node.kind == SyntaxKind::Name && !declaration_name {
            let name = node_text(&unit.source, node);
            if unit
                .typed_bindings
                .iter()
                .rev()
                .find(|binding| {
                    binding.name == name && binding.is_visible_at(unit.source.id(), node.span.start)
                })
                .is_some_and(|binding| {
                    !(closure.start <= binding.span.start && binding.span.end <= closure.end)
                })
            {
                captures.insert(name.to_owned());
            }
            return;
        }
        match node.kind {
            SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::AnonymousFunction => {
                let mut skipped_name = false;
                for child in &node.children {
                    if !skipped_name && child.kind == SyntaxKind::Name {
                        skipped_name = true;
                        continue;
                    }
                    collect(unit, closure, child, captures, false);
                }
            }
            SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression => {
                if let Some(receiver) = node.children.first() {
                    collect(unit, closure, receiver, captures, false);
                }
            }
            SyntaxKind::ConstructionExpression => {}
            SyntaxKind::Argument if node.children.len() > 1 => {
                for child in node.children.iter().skip(1) {
                    collect(unit, closure, child, captures, false);
                }
            }
            _ => {
                for child in &node.children {
                    collect(unit, closure, child, captures, false);
                }
            }
        }
    }
    fn closure_node(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
        if node.kind == SyntaxKind::AnonymousFunction && node.span == span {
            return Some(node);
        }
        node.children
            .iter()
            .find_map(|child| closure_node(child, span))
    }

    for unit in &mut package.units {
        let captures = unit
            .functions
            .iter()
            .filter(|contract| contract.name.starts_with("closure@"))
            .map(|contract| {
                let mut captures = BTreeSet::new();
                if let Some(node) = closure_node(&unit.tree.root, contract.span) {
                    collect(unit, contract.span, node, &mut captures, false);
                }
                (contract.span, captures.into_iter().collect::<Vec<_>>())
            })
            .collect::<Vec<_>>();
        for contract in &mut unit.functions {
            if let Some((_, captures)) = captures.iter().find(|(span, _)| *span == contract.span) {
                contract.captures.clone_from(captures);
            }
        }
    }
}

fn validate_descriptor_value_uses(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        validate_descriptor_value_node(package, unit, &unit.tree.root, false)?;
    }
    Ok(())
}

fn validate_descriptor_value_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    descriptor_context: bool,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::TypeMembershipExpression
        && let Some(descriptor) = node.children.get(1)
        && descriptor_expression_type(package, unit, descriptor).is_none()
        && descriptor_expression_category(package, unit, descriptor).is_none()
    {
        return Err(failure(
            &unit.source,
            "T0001",
            format!(
                "`{}` does not resolve to a type descriptor",
                node_text(&unit.source, descriptor).trim()
            ),
            descriptor.span,
        ));
    }
    if !descriptor_context
        && node.kind == SyntaxKind::MemberExpression
        && node.children.first().is_some_and(|receiver| {
            descriptor_expression_type(package, unit, receiver).is_some()
                || descriptor_expression_category(package, unit, receiver).is_some()
        })
        && package.reflection == crate::package::ReflectionProfile::Minimal
    {
        return Err(failure(
            &unit.source,
            "T0070",
            "the selected minimal profile does not retain reflection metadata",
            node.span,
        ));
    }

    for (index, child) in node.children.iter().enumerate() {
        let child_is_descriptor_context = descriptor_context
            || node.kind == SyntaxKind::TypeExpression
            || node.kind == SyntaxKind::ImportDeclaration
            || (node.kind == SyntaxKind::TypeMembershipExpression && index == 1)
            || (node.kind == SyntaxKind::MemberExpression && index == 1)
            || (matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) && index == 0)
            || (node.kind == SyntaxKind::BinaryExpression
                && node.children.len() == 2
                && node_text(&unit.source, node)[node.children[0].span.end - node.span.start
                    ..node.children[1].span.start - node.span.start]
                    .trim()
                    == "is")
            || (node.kind == SyntaxKind::BinaryExpression
                && node.children.len() == 2
                && matches!(
                    node_text(&unit.source, node)[node.children[0].span.end - node.span.start
                        ..node.children[1].span.start - node.span.start]
                        .trim(),
                    "==" | "!="
                )
                && node_text(&unit.source, child).trim() == "none")
            || (node.kind == SyntaxKind::CallExpression
                && index == 1
                && node.children.first().is_some_and(|callee| {
                    coercion_family_receiver(unit, callee)
                        || obsolete_integer_coercion_member(unit, callee).is_some()
                }));
        validate_descriptor_value_node(package, unit, child, child_is_descriptor_context)?;
    }
    Ok(())
}

pub(crate) fn descriptor_expression_type(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<ScalarType> {
    let name = node_text(&unit.source, node).trim();
    match node.kind {
        SyntaxKind::Name | SyntaxKind::TypeExpression => unit
            .descriptor_alias_at(name, node.span.start)
            .or_else(|| package.descriptor_constructs.get(name)?.descriptor_type())
            .or_else(|| {
                node.children
                    .first()
                    .and_then(|child| descriptor_expression_type(package, unit, child))
            }),
        _ => None,
    }
}

pub(crate) fn descriptor_expression_category(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<TypeCategory> {
    let name = node_text(&unit.source, node).trim();
    match node.kind {
        SyntaxKind::Name | SyntaxKind::TypeExpression => package
            .resolve_name_at(unit, node.span.start, name)
            .and_then(Symbol::descriptor_category)
            .or_else(|| {
                node.children
                    .first()
                    .and_then(|child| descriptor_expression_category(package, unit, child))
            }),
        _ => None,
    }
}

fn collect_type_declarations(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    aliases: &mut BTreeMap<String, Vec<DescriptorAlias>>,
    functions: &mut Vec<FunctionContract>,
    scope: Option<Span>,
) -> Result<(), SemanticFailure> {
    if let Some((name, alias)) = descriptor_alias(unit, node, aliases, scope) {
        aliases.entry(name).or_default().push(alias);
    }
    if is_function_node(node) {
        let visible = visible_descriptor_aliases(aliases, unit.source.id(), node.span.start);
        functions.push(analyze_function_contract(unit, node, &visible)?);
    }
    let child_scope = is_function_node(node).then_some(node.span).or(scope);
    for child in &node.children {
        collect_type_declarations(unit, child, aliases, functions, child_scope)?;
    }
    Ok(())
}

fn descriptor_alias(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    aliases: &BTreeMap<String, Vec<DescriptorAlias>>,
    scope: Option<Span>,
) -> Option<(String, DescriptorAlias)> {
    if !matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        || node
            .children
            .iter()
            .any(|child| child.kind == SyntaxKind::TypeExpression)
    {
        return None;
    }
    let name = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)?;
    let initializer = node.children.last()?;
    let descriptor_name = node_text(&unit.source, initializer).trim();
    if descriptor_name == "none" {
        return None;
    }
    let value_type = match initializer.kind {
        SyntaxKind::Name => {
            visible_descriptor_aliases(aliases, unit.source.id(), initializer.span.start)
                .get(descriptor_name)
                .copied()
        }
        _ => None,
    }?;
    Some((
        node_text(&unit.source, name).to_owned(),
        DescriptorAlias {
            visible_from: node.span.end,
            scope,
            value_type,
        },
    ))
}

#[expect(
    clippy::too_many_lines,
    reason = "binding collection preserves declaration and scope ordering in one traversal"
)]
fn collect_typed_bindings(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    visible_bindings: &mut Vec<TypedBinding>,
    bindings: &mut Vec<TypedBinding>,
    scope: Option<Span>,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration
    ) {
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            for method in block
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::FunctionDeclaration)
            {
                collect_typed_bindings(
                    unit,
                    method,
                    visible_bindings,
                    bindings,
                    Some(method.span),
                )?;
            }
        }
        return Ok(());
    }
    if is_function_node(node) {
        let contract = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed function declaration must have a semantic contract");
        let mut parameter_bindings = contract
            .parameters
            .iter()
            .filter_map(|parameter| {
                parameter.value_type.clone().map(|value_type| TypedBinding {
                    name: parameter.name.clone(),
                    span: parameter.span,
                    visible_from: parameter.span.start,
                    scope: Some(node.span),
                    value_type,
                    destination_arms: Vec::new(),
                    storage_type: None,
                    mutable: false,
                })
            })
            .collect::<Vec<_>>();
        if let Some(owner) = &contract.owner {
            parameter_bindings.push(TypedBinding {
                name: "self".to_owned(),
                span: implicit_receiver_span(node, "self"),
                visible_from: node.span.start,
                scope: Some(node.span),
                value_type: ValueType::Descriptor(owner.clone()),
                destination_arms: Vec::new(),
                storage_type: None,
                mutable: false,
            });
            if !contract.is_static {
                parameter_bindings.push(TypedBinding {
                    name: "this".to_owned(),
                    span: implicit_receiver_span(node, "this"),
                    visible_from: node.span.start,
                    scope: Some(node.span),
                    value_type: ValueType::Object(ObjectIdentity::new(&unit.namespace, owner)),
                    destination_arms: Vec::new(),
                    storage_type: None,
                    mutable: true,
                });
            }
        }
        let mut function_bindings = visible_bindings.clone();
        function_bindings.extend(parameter_bindings.iter().cloned());
        bindings.extend(parameter_bindings);
        for child in &node.children {
            collect_typed_bindings(
                unit,
                child,
                &mut function_bindings,
                bindings,
                Some(node.span),
            )?;
        }
        return Ok(());
    }
    if let [target, collection, block] = node.children.as_slice()
        && node.kind == SyntaxKind::ForStatement
        && target.kind == SyntaxKind::ForTarget
    {
        collect_typed_bindings(unit, collection, visible_bindings, bindings, scope)?;
        let item_type = infer_value_type(unit, collection, visible_bindings)?
            .and_then(iterable_item_type)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0016",
                    "collection iteration requires an iterable value",
                    collection.span,
                )
            })?;
        let loop_bindings =
            iteration_target_bindings(unit, target, collection.span.end, block.span, item_type)?;
        bindings.extend(loop_bindings.iter().cloned());
        let mut visible_loop_bindings = visible_bindings.clone();
        visible_loop_bindings.extend(loop_bindings);
        collect_typed_bindings(
            unit,
            block,
            &mut visible_loop_bindings,
            bindings,
            Some(block.span),
        )?;
        return Ok(());
    }
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
        let prior_len = visible_bindings.len();
        analyze_binding_node(unit, node, visible_bindings, scope)?;
        bindings.extend_from_slice(&visible_bindings[prior_len..]);
    }
    for child in &node.children {
        let child_scope = (child.kind == SyntaxKind::Block)
            .then_some(child.span)
            .or(scope);
        collect_typed_bindings(unit, child, visible_bindings, bindings, child_scope)?;
    }
    Ok(())
}

fn mutates_object_receiver(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
    if node.kind == SyntaxKind::Assignment
        && let Some(target) = node.children.first()
        && target.kind == SyntaxKind::MemberExpression
        && target.children.first().is_some_and(|receiver| {
            receiver.kind == SyntaxKind::Name && node_text(&unit.source, receiver) == "this"
        })
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| mutates_object_receiver(unit, child))
}

#[expect(
    clippy::too_many_lines,
    reason = "callable signature analysis keeps parameter and result contracts in source order"
)]
fn analyze_function_contract(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    aliases: &BTreeMap<String, ScalarType>,
) -> Result<FunctionContract, SemanticFailure> {
    let name_node = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name);
    if node.kind == SyntaxKind::FunctionDeclaration && name_node.is_none() {
        return Err(failure(
            &unit.source,
            "T0004",
            "function requires a name",
            node.span,
        ));
    }
    let return_type = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::TypeExpression)
        .map(|type_node| declared_value_type(unit, type_node, aliases))
        .transpose()?;
    let mut parameters = Vec::new();
    let mut optional_seen = false;
    if let Some(parameter_list) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::ParameterList)
    {
        for parameter in &parameter_list.children {
            let Some(parameter_name) = parameter
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            else {
                continue;
            };
            let type_node = parameter
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::TypeExpression);
            let value_type = type_node
                .map(|type_node| declared_value_type(unit, type_node, aliases))
                .transpose()?;
            let default = parameter.children.iter().find(|child| {
                child.span != parameter_name.span && child.kind != SyntaxKind::TypeExpression
            });
            let optional = default.is_some();
            if optional {
                optional_seen = true;
            } else if optional_seen {
                return Err(failure(
                    &unit.source,
                    "T0005",
                    "required parameters must precede optional parameters",
                    parameter.span,
                ));
            }
            if let (Some(expected), Some(default)) = (value_type.clone(), default) {
                let actual =
                    infer_value_type(unit, default, &unit.typed_bindings)?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0006",
                            "parameter default has no value",
                            default.span,
                        )
                    })?;
                validate_value_destination(
                    &unit.source,
                    &unit.objects,
                    node_text(&unit.source, parameter_name),
                    expected,
                    actual,
                    default,
                    "T0006",
                )?;
            }
            parameters.push(ParameterContract {
                name: node_text(&unit.source, parameter_name).to_owned(),
                span: parameter.span,
                value_type,
                optional,
                mutable: false,
            });
        }
    }
    if name_node.is_some_and(|name| node_text(&unit.source, name) == "main")
        && object_name_containing(unit, node.span).is_none()
        && !parameters.is_empty()
    {
        return Err(failure(
            &unit.source,
            "T0078",
            "program entrypoint `main` cannot declare parameters",
            node.span,
        ));
    }
    let mut thrown_types = Vec::new();
    for child in &node.children {
        if child.kind == SyntaxKind::EffectClause {
            if let Some(type_node) = child
                .children
                .iter()
                .find(|part| part.kind == SyntaxKind::TypeExpression)
            {
                thrown_types.push(declared_value_type(unit, type_node, aliases)?);
            }
        }
    }
    let is_async = node.children.iter().any(|child| {
        child.kind == SyntaxKind::DeclarationQualifier && node_text(&unit.source, child) == "async"
    });
    let is_static = node.children.iter().any(|child| {
        child.kind == SyntaxKind::DeclarationQualifier && node_text(&unit.source, child) == "static"
    });
    let throws = !thrown_types.is_empty();
    let exported = node.children.iter().any(|child| {
        child.kind == SyntaxKind::Visibility && node_text(&unit.source, child) == "public"
    });
    Ok(FunctionContract {
        name: name_node.map_or_else(
            || format!("closure@{}", node.span.start),
            |name| node_text(&unit.source, name).to_owned(),
        ),
        span: node.span,
        owner: (node.kind == SyntaxKind::FunctionDeclaration)
            .then(|| object_name_containing(unit, node.span))
            .flatten(),
        parameters,
        captures: Vec::new(),
        return_type,
        thrown_types,
        escaping_throwables: BTreeSet::new(),
        throws,
        is_async,
        is_static,
        mutates_receiver: mutates_object_receiver(unit, node),
        consumes_receiver: false,
        exported,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "the fixed-point call graph and its local traversals form one auditable effect analysis"
)]
fn infer_throwing_effects(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    type FunctionKey = (u32, usize, usize);

    fn key(span: Span) -> FunctionKey {
        (span.file, span.start, span.end)
    }

    fn direct_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> BTreeSet<String> {
        if node.kind == SyntaxKind::FunctionDeclaration {
            return BTreeSet::new();
        }
        if node.kind == SyntaxKind::ThrowStatement {
            return node
                .children
                .first()
                .and_then(|error| {
                    let descriptor = if error.kind == SyntaxKind::CallExpression {
                        error.children.first().unwrap_or(error)
                    } else {
                        error
                    };
                    let descriptor = if descriptor.kind == SyntaxKind::ConstructionExpression {
                        descriptor.children.first().unwrap_or(descriptor)
                    } else {
                        descriptor
                    };
                    package.resolve_name_at(
                        unit,
                        descriptor.span.start,
                        node_text(&unit.source, descriptor),
                    )
                })
                .map(|symbol| symbol.identity.clone())
                .into_iter()
                .collect();
        }
        if node.kind == SyntaxKind::TryStatement {
            let mut errors = node
                .children
                .first()
                .map_or_else(BTreeSet::new, |block| direct_errors(package, unit, block));
            let mut clauses_finished = false;
            for child in node.children.iter().skip(1) {
                if child.kind == SyntaxKind::CatchClause {
                    let descriptor = child
                        .children
                        .first()
                        .filter(|candidate| candidate.kind == SyntaxKind::Name);
                    if let Some(descriptor) = descriptor
                        && let Some(symbol) = package.resolve_name_at(
                            unit,
                            descriptor.span.start,
                            node_text(&unit.source, descriptor),
                        )
                    {
                        if symbol.identity == "/core/errors::throwable" {
                            errors.clear();
                        } else {
                            errors.remove(&symbol.identity);
                        }
                    } else {
                        errors.clear();
                    }
                    if let Some(block) = child.children.last() {
                        errors.extend(direct_errors(package, unit, block));
                    }
                    clauses_finished = true;
                } else if child.kind == SyntaxKind::FinallyClause {
                    if let Some(block) = child.children.last() {
                        errors.extend(direct_errors(package, unit, block));
                    }
                } else if !clauses_finished {
                    errors.extend(direct_errors(package, unit, child));
                }
            }
            return errors;
        }
        node.children
            .iter()
            .flat_map(|child| direct_errors(package, unit, child))
            .collect()
    }

    fn integer_coercion_can_fail(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        let Some(callee) = node.children.first() else {
            return false;
        };
        let Some((source_node, CoercionPolicy::Default)) =
            integer_coercion_call(&unit.source, callee)
        else {
            return false;
        };
        let Ok(Some(ValueType::Scalar(source))) =
            infer_receiver_value_type(unit, source_node, &unit.typed_bindings)
        else {
            return false;
        };
        let Some(destination_node) = node
            .children
            .get(1)
            .and_then(|arguments| arguments.children.first())
            .and_then(|argument| argument.children.last())
        else {
            return false;
        };
        let Some(destination) = unit.descriptor_alias_at(
            node_text(&unit.source, destination_node),
            destination_node.span.start,
        ) else {
            return false;
        };
        if destination == ScalarType::Int {
            return false;
        }
        let Some(destination_bounds) = scalar_integer_bounds(destination) else {
            return false;
        };
        let Some(source_bounds) = scalar_integer_bounds(source) else {
            return source == ScalarType::Int;
        };
        source_bounds.0 < destination_bounds.0 || source_bounds.1 > destination_bounds.1
    }

    fn fixed_integer_bits(ty: ScalarType) -> Option<u16> {
        match ty {
            ScalarType::Int8 | ScalarType::Uint8 => Some(8),
            ScalarType::Int16 | ScalarType::Uint16 => Some(16),
            ScalarType::Int32 | ScalarType::Uint32 => Some(32),
            ScalarType::Int64 | ScalarType::Uint64 => Some(64),
            ScalarType::Int128 | ScalarType::Uint128 => Some(128),
            _ => None,
        }
    }

    fn numeric_conversion_can_fail(source: ScalarType, destination: ScalarType) -> bool {
        if source == destination {
            return false;
        }
        if destination == ScalarType::Int {
            return matches!(source, ScalarType::Float32 | ScalarType::Float64);
        }
        if source == ScalarType::Int {
            return destination.is_integer()
                || matches!(destination, ScalarType::Float32 | ScalarType::Float64);
        }
        if source.is_integer() && destination.is_integer() {
            let Some(source_bounds) = scalar_integer_bounds(source) else {
                return false;
            };
            let Some(destination_bounds) = scalar_integer_bounds(destination) else {
                return false;
            };
            return source_bounds.0 < destination_bounds.0
                || source_bounds.1 > destination_bounds.1;
        }
        if source == ScalarType::Float32 && destination == ScalarType::Float64 {
            return false;
        }
        if source.is_integer() && matches!(destination, ScalarType::Float32 | ScalarType::Float64) {
            let exact_bits = if destination == ScalarType::Float32 {
                16
            } else {
                32
            };
            return fixed_integer_bits(source).is_some_and(|bits| bits > exact_bits);
        }
        matches!(source, ScalarType::Float32 | ScalarType::Float64)
            && (destination.is_integer() || destination == ScalarType::Float32)
    }

    fn destination_conversion_can_fail(
        unit: &SemanticUnit,
        expected: &ValueType,
        value: &SyntaxNode,
    ) -> bool {
        if value.kind == SyntaxKind::GroupExpression
            && let [grouped] = value.children.as_slice()
        {
            return destination_conversion_can_fail(unit, expected, grouped);
        }
        if let ValueType::Optional(inner) = expected {
            return destination_conversion_can_fail(unit, inner, value);
        }
        if let ValueType::Scalar(destination) = expected {
            if contextual_constant(&unit.source, value, *destination).is_some() {
                return false;
            }
            let Ok(Some(ValueType::Scalar(source))) =
                infer_value_type(unit, value, &unit.typed_bindings)
            else {
                return false;
            };
            return numeric_conversion_can_fail(source, *destination);
        }
        let [callee, arguments] = value.children.as_slice() else {
            return false;
        };
        if value.kind != SyntaxKind::CallExpression {
            return false;
        }
        let Some(identity) = collection_constructor_identity(unit, callee, &unit.typed_bindings)
        else {
            return false;
        };
        let name = identity
            .strip_prefix("/core/collections::")
            .unwrap_or(identity);
        match (name, expected) {
            (
                "list" | "tuple" | "set" | "unordered-set",
                ValueType::List(item)
                | ValueType::Tuple(item, _)
                | ValueType::Set(item)
                | ValueType::UnorderedSet(item),
            ) => arguments.children.iter().any(|argument| {
                destination_conversion_can_fail(
                    unit,
                    &item.value_type(),
                    argument.children.last().unwrap_or(argument),
                )
            }),
            ("entry", ValueType::Entry(key, entry_value)) => {
                let [key_argument, value_argument] = arguments.children.as_slice() else {
                    return false;
                };
                destination_conversion_can_fail(
                    unit,
                    &key.value_type(),
                    key_argument.children.last().unwrap_or(key_argument),
                ) || destination_conversion_can_fail(
                    unit,
                    &entry_value.value_type(),
                    value_argument.children.last().unwrap_or(value_argument),
                )
            }
            (
                "map" | "unordered-map",
                ValueType::Map(key, map_value) | ValueType::UnorderedMap(key, map_value),
            ) => arguments.children.iter().any(|argument| {
                let value_node = argument.children.last().unwrap_or(argument);
                if argument.children.len() < 2
                    && matches!(
                        infer_value_type(unit, value_node, &unit.typed_bindings),
                        Ok(Some(ValueType::Entry(_, _)))
                    )
                {
                    destination_conversion_can_fail(
                        unit,
                        &ValueType::Entry(key.clone(), map_value.clone()),
                        value_node,
                    )
                } else {
                    destination_conversion_can_fail(unit, &map_value.value_type(), value_node)
                }
            }),
            _ => false,
        }
    }

    fn call_argument_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> BTreeSet<String> {
        let mut errors = BTreeSet::new();
        let [callee, arguments] = node.children.as_slice() else {
            return errors;
        };
        if node.kind != SyntaxKind::CallExpression {
            return errors;
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && let Ok(Some(receiver_type)) =
                infer_receiver_value_type(unit, receiver, &unit.typed_bindings)
        {
            let argument_can_fail = |index: usize, expected: &ValueType| {
                arguments.children.get(index).is_some_and(|argument| {
                    destination_conversion_can_fail(
                        unit,
                        expected,
                        argument.children.last().unwrap_or(argument),
                    )
                })
            };
            let conversion_can_fail = match (receiver_type, node_text(&unit.source, member)) {
                (ValueType::List(item), "append")
                | (
                    ValueType::Set(item) | ValueType::UnorderedSet(item),
                    "add" | "contains" | "remove",
                ) => argument_can_fail(0, &item.value_type()),
                (ValueType::List(item), "set") => argument_can_fail(1, &item.value_type()),
                (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "set") => {
                    argument_can_fail(0, &key.value_type())
                        || argument_can_fail(1, &value.value_type())
                }
                _ => false,
            };
            if conversion_can_fail {
                errors.insert("/core/errors::integer-conversion-overflow".to_owned());
            }
        }
        let Some(parameters) = function_parameters(package, unit, callee) else {
            return errors;
        };
        let mut positional = 0;
        for argument in &arguments.children {
            let name = argument
                .children
                .first()
                .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1)
                .map(|name| node_text(&unit.source, name));
            let parameter = if let Some(name) = name {
                parameters.iter().find(|parameter| parameter.name == name)
            } else {
                let parameter = parameters.get(positional);
                positional += 1;
                parameter
            };
            let value = argument.children.last().unwrap_or(argument);
            if parameter
                .and_then(|parameter| parameter.value_type.as_ref())
                .is_some_and(|expected| destination_conversion_can_fail(unit, expected, value))
            {
                errors.insert("/core/errors::integer-conversion-overflow".to_owned());
            }
        }
        errors
    }

    fn local_builtin_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> BTreeSet<String> {
        let mut errors = call_argument_errors(package, unit, node);
        if node.kind == SyntaxKind::BinaryExpression
            && let [left, right] = node.children.as_slice()
            && matches!(
                unit.inferred_value_type(node),
                Some(ValueType::Scalar(ScalarType::Int))
            )
            && matches!(
                unit.source.text()[left.span.end..right.span.start].trim(),
                "/" | "%"
            )
        {
            errors.insert("/core/errors::division-by-zero".to_owned());
        }
        if node.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = node.children.as_slice()
            && matches!(
                node_text(&unit.source, member),
                "round" | "floor" | "ceiling" | "truncate"
            )
            && matches!(
                infer_receiver_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(ValueType::Scalar(
                    ScalarType::Float32 | ScalarType::Float64
                )))
            )
        {
            errors.insert("/core/errors::integer-conversion-overflow".to_owned());
        }
        let destination = if node.kind == SyntaxKind::Binding {
            node.children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .and_then(|name| {
                    unit.typed_bindings
                        .iter()
                        .find(|binding| binding.span == name.span)
                        .map(|binding| binding.value_type.clone())
                })
                .zip(node.children.iter().find(|child| {
                    !matches!(
                        child.kind,
                        SyntaxKind::Name
                            | SyntaxKind::Visibility
                            | SyntaxKind::DeclarationQualifier
                            | SyntaxKind::TypeExpression
                    )
                }))
        } else if node.kind == SyntaxKind::Assignment {
            let [target, value] = node.children.as_slice() else {
                return errors;
            };
            infer_value_type(unit, target, &unit.typed_bindings)
                .ok()
                .flatten()
                .map(|expected| (expected, value))
        } else {
            None
        };
        if destination.is_some_and(|(expected, value)| {
            destination_conversion_can_fail(unit, &expected, value)
        }) {
            errors.insert("/core/errors::integer-conversion-overflow".to_owned());
        }
        if node.kind == SyntaxKind::ReturnStatement
            && let Some(value) = node.children.first()
            && let Some(function_span) = unit
                .enclosing_function_spans
                .get(&node.span.start)
                .copied()
                .flatten()
            && let Some(expected) = unit
                .functions
                .iter()
                .find(|contract| contract.span == function_span)
                .and_then(|contract| contract.return_type.as_ref())
            && destination_conversion_can_fail(unit, expected, value)
        {
            errors.insert("/core/errors::integer-conversion-overflow".to_owned());
        }
        errors
    }
    fn escaping_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        inferred: &BTreeMap<FunctionKey, BTreeSet<String>>,
    ) -> BTreeSet<String> {
        if node.kind == SyntaxKind::FunctionDeclaration {
            return BTreeSet::new();
        }
        if node.kind == SyntaxKind::ThrowStatement {
            return direct_errors(package, unit, node);
        }
        let local_errors = local_builtin_errors(package, unit, node);
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
        {
            let member_name = node_text(&unit.source, member);
            let receiver_type = infer_receiver_value_type(unit, receiver, &unit.typed_bindings)
                .ok()
                .flatten();
            let mut errors = if integer_coercion_can_fail(unit, node) {
                BTreeSet::from(["/core/errors::integer-conversion-overflow".to_owned()])
            } else if let Some(ValueType::Object(object)) = receiver_type {
                unit.functions
                    .iter()
                    .find(|contract| {
                        contract.owner.as_deref() == Some(object.name.as_str())
                            && contract.name == member_name
                    })
                    .and_then(|contract| inferred.get(&key(contract.span)))
                    .cloned()
                    .unwrap_or_default()
            } else if member_name == "decode" {
                BTreeSet::from(["/core/errors::decode-error".to_owned()])
            } else {
                BTreeSet::new()
            };
            errors.extend(local_errors);
            errors.extend(
                node.children
                    .iter()
                    .flat_map(|child| escaping_errors(package, unit, child, inferred)),
            );
            return errors;
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
            && let Some(symbol) =
                package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
            && symbol.kind == SymbolKind::Function
            && let Some(span) = symbol.declaration_span
        {
            let mut errors = inferred.get(&key(span)).cloned().unwrap_or_default();
            errors.extend(local_errors);
            errors.extend(
                node.children
                    .iter()
                    .skip(1)
                    .flat_map(|argument| escaping_errors(package, unit, argument, inferred)),
            );
            return errors;
        }
        if node.kind == SyntaxKind::TryStatement {
            let mut errors = node.children.first().map_or_else(BTreeSet::new, |block| {
                escaping_errors(package, unit, block, inferred)
            });
            let mut clauses_finished = false;
            for child in node.children.iter().skip(1) {
                if child.kind == SyntaxKind::CatchClause {
                    let descriptor = child
                        .children
                        .first()
                        .filter(|candidate| candidate.kind == SyntaxKind::Name);
                    if let Some(descriptor) = descriptor
                        && let Some(symbol) = package.resolve_name_at(
                            unit,
                            descriptor.span.start,
                            node_text(&unit.source, descriptor),
                        )
                    {
                        if symbol.identity == "/core/errors::throwable" {
                            errors.clear();
                        } else {
                            errors.remove(&symbol.identity);
                        }
                    } else {
                        errors.clear();
                    }
                    if let Some(block) = child.children.last() {
                        errors.extend(escaping_errors(package, unit, block, inferred));
                    }
                    clauses_finished = true;
                } else if child.kind == SyntaxKind::FinallyClause {
                    if let Some(block) = child.children.last() {
                        errors.extend(escaping_errors(package, unit, block, inferred));
                    }
                } else if !clauses_finished {
                    errors.extend(escaping_errors(package, unit, child, inferred));
                }
            }
            return errors;
        }
        let mut errors = local_errors;
        errors.extend(
            node.children
                .iter()
                .flat_map(|child| escaping_errors(package, unit, child, inferred)),
        );
        errors
    }

    let mut inferred_throwables = BTreeMap::<FunctionKey, BTreeSet<String>>::new();
    let mut bodies = BTreeMap::<FunctionKey, (usize, SyntaxNode)>::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        for contract in unit
            .functions
            .iter()
            .filter(|contract| contract.span.file == unit.source.id())
        {
            let Some(function) = find_node_by_span(&unit.tree.root, contract.span) else {
                continue;
            };
            let function_key = key(function.span);
            bodies.insert(function_key, (unit_index, function.clone()));
            let function_throwables = function
                .children
                .iter()
                .flat_map(|child| direct_errors(package, unit, child))
                .collect();
            inferred_throwables.insert(function_key, function_throwables);
        }
    }

    loop {
        let mut changed = false;
        for function in bodies.keys() {
            let (unit_index, body) = &bodies[function];
            let unit = &package.units[*unit_index];
            let combined_throwables = body
                .children
                .iter()
                .flat_map(|child| escaping_errors(package, unit, child, &inferred_throwables))
                .collect::<BTreeSet<_>>();
            if combined_throwables != inferred_throwables[function] {
                inferred_throwables.insert(*function, combined_throwables);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for unit in &package.units {
        for contract in unit
            .functions
            .iter()
            .filter(|contract| contract.span.file == unit.source.id())
        {
            let Some(ValueType::Object(bound)) = contract.thrown_types.first() else {
                continue;
            };
            for identity in inferred_throwables
                .get(&key(contract.span))
                .into_iter()
                .flatten()
            {
                let actual = identity
                    .rsplit_once("::")
                    .map_or(identity.as_str(), |(_, name)| name);
                let bound_identity = Some(bound.qualified());
                let compatible = bound_identity.as_deref().is_some_and(|bound_identity| {
                    bound_identity == "/core/errors::throwable"
                        || bound_identity == identity
                        || identity_implements(package, identity, bound_identity)
                });
                if !compatible {
                    return Err(failure(
                        &unit.source,
                        "T0027",
                        format!(
                            "`{actual}` may escape `{}` but does not satisfy its `throws {bound}` contract",
                            contract.name
                        ),
                        contract.span,
                    ));
                }
            }
        }
    }

    for unit in &mut package.units {
        for contract in &mut unit.functions {
            contract.escaping_throwables = inferred_throwables
                .get(&key(contract.span))
                .cloned()
                .unwrap_or_default();
            contract.throws = !contract.escaping_throwables.is_empty();
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "binding analysis keeps destination selection and initialization validation together"
)]
fn analyze_binding_node(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &mut Vec<TypedBinding>,
    scope: Option<Span>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::Assignment
        && let [target, value] = node.children.as_slice()
        && target.kind == SyntaxKind::IndexExpression
        && let [receiver, _] = target.children.as_slice()
        && let Some(receiver_type) = infer_receiver_value_type(unit, receiver, bindings)?
    {
        let expected = match receiver_type {
            ValueType::List(item) => Some(item.value_type()),
            ValueType::Map(_, value) | ValueType::UnorderedMap(_, value) => {
                Some(value.value_type())
            }
            ValueType::Tuple(_, _) => {
                return Err(failure(
                    &unit.source,
                    "T0048",
                    "tuple items are fixed at construction and cannot be replaced",
                    target.span,
                ));
            }
            other => {
                return Err(failure(
                    &unit.source,
                    "T0048",
                    format!("indexed assignment is not supported for `{other}`"),
                    target.span,
                ));
            }
        };
        let actual = infer_value_type(unit, value, bindings)?;
        if let (Some(expected), Some(actual)) = (expected, actual)
            && expected != actual
        {
            return Err(failure(
                &unit.source,
                "T0046",
                format!("indexed assignment requires `{expected}`, found `{actual}`"),
                value.span,
            ));
        }
        return Ok(());
    }
    let Some(name_node) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
    else {
        return Ok(());
    };
    let name = node_text(&unit.source, name_node).to_owned();
    let declared = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::TypeExpression);
    let initializer = node.children.iter().rev().find(|child| {
        child.span != name_node.span
            && !matches!(
                child.kind,
                SyntaxKind::Visibility
                    | SyntaxKind::DeclarationQualifier
                    | SyntaxKind::TypeExpression
            )
    });

    if node.kind == SyntaxKind::Assignment
        && declared.is_none()
        && let Some(previous) = bindings.iter().rev().find(|binding| binding.name == name)
        && let Some(initializer) = initializer
        && let Some(actual) = infer_value_type(unit, initializer, bindings)?
    {
        if previous.destination_arms.is_empty() {
            if let ValueType::Scalar(expected) = previous.value_type.clone() {
                validate_numeric_destination(
                    &unit.source,
                    &name,
                    expected,
                    actual,
                    initializer,
                    "T0002",
                )?;
            }
        } else {
            select_union_candidates(
                &unit.source,
                initializer,
                actual,
                previous.destination_arms.clone(),
            )?;
        }
        return Ok(());
    }
    let declared_value = declared
        .map(|type_node| {
            let aliases = visible_descriptor_aliases(
                &unit.descriptor_aliases,
                unit.source.id(),
                type_node.span.start,
            );
            declared_value_type(unit, type_node, &aliases).or_else(|failure| {
                union_destination_candidates(unit, type_node)
                    .ok()
                    .and_then(|candidates| candidates.into_iter().next())
                    .map(ValueType::Scalar)
                    .ok_or(failure)
            })
        })
        .transpose()?;
    if declared_value.is_none()
        && let Some(initializer) = initializer
        && let Some(identity) = empty_collection_identity(unit, initializer, bindings)
    {
        let collection = identity
            .strip_prefix("/core/collections::")
            .unwrap_or(identity);
        if collection == "entry" {
            return Err(failure(
                &unit.source,
                "T0045",
                "`entry` requires exactly a key and value",
                initializer.span,
            ));
        }
        let message = if matches!(collection, "map" | "unordered-map") {
            format!("an empty `{collection}` requires explicit key and value types")
        } else {
            format!("an empty `{collection}` requires an explicit item type")
        };
        return Err(failure(&unit.source, "T0043", message, initializer.span));
    }
    if let Some(initializer) = initializer
        && initializer.kind == SyntaxKind::Name
        && collection_constructor_identity(unit, initializer, bindings).is_some_and(|identity| {
            identity
                .strip_prefix("/core/collections::")
                .unwrap_or(identity)
                == "entry"
        })
    {
        return Err(failure(
            &unit.source,
            "T0045",
            "`entry` requires exactly a key and value",
            initializer.span,
        ));
    }
    let inferred = initializer
        .map(|value| {
            if let Some(declared_value) = declared_value.clone()
                && collection_constructor_matches(unit, value, &declared_value, bindings)
            {
                validate_collection_constructor_items(
                    unit,
                    value,
                    &declared_value,
                    &name,
                    bindings,
                )?;
                Ok(Some(declared_value))
            } else {
                infer_value_type(unit, value, bindings)
            }
        })
        .transpose()?
        .flatten();
    let value_type =
        if let (Some(type_node), Some(declared_type)) = (declared, declared_value.clone()) {
            let value_type = if matches!(declared_type, ValueType::Optional(_)) {
                declared_type
            } else if let (Some(inferred), Some(initializer), Ok(_)) = (
                inferred.clone(),
                initializer,
                union_destination_candidates(unit, type_node),
            ) {
                ValueType::Scalar(select_union_destination(
                    unit,
                    type_node,
                    initializer,
                    inferred,
                )?)
            } else {
                declared_type
            };
            if let (Some(inferred), Some(initializer)) = (inferred.clone(), initializer) {
                validate_value_destination(
                    &unit.source,
                    &unit.objects,
                    &name,
                    value_type.clone(),
                    inferred,
                    initializer,
                    "T0002",
                )?;
            }
            value_type
        } else if let Some(inferred) = inferred.clone() {
            inferred
        } else {
            return Ok(());
        };
    let destination_arms = if matches!(value_type, ValueType::Optional(_)) {
        Vec::new()
    } else {
        declared
            .and_then(|type_node| union_destination_candidates(unit, type_node).ok())
            .filter(|arms| arms.len() > 1)
            .unwrap_or_default()
    };
    let storage_type = (value_type == ValueType::Scalar(ScalarType::Int))
        .then(|| initializer.and_then(|value| small_int_storage(unit, value, inferred.clone())))
        .flatten();

    bindings.push(TypedBinding {
        name,
        span: node.span,
        visible_from: node.span.end,
        scope,
        value_type,
        destination_arms,
        storage_type,
        mutable: false,
    });
    Ok(())
}

fn declared_value_type(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
    aliases: &BTreeMap<String, ScalarType>,
) -> Result<ValueType, SemanticFailure> {
    declared_value_type_with_visible_objects(unit, type_node, aliases, &BTreeMap::new())
}

#[expect(
    clippy::too_many_lines,
    reason = "type-shape validation keeps all supported composite forms in one ordered match"
)]
fn declared_value_type_with_visible_objects(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
    aliases: &BTreeMap<String, ScalarType>,
    visible_objects: &BTreeMap<String, ObjectIdentity>,
) -> Result<ValueType, SemanticFailure> {
    let shape = if type_node.kind == SyntaxKind::TypeExpression {
        type_node.children.first().unwrap_or(type_node)
    } else {
        type_node
    };
    if shape.kind == SyntaxKind::PrefixType
        && let Some(inner) = shape.children.first()
    {
        let inner = ElementType::new(declared_value_type_with_visible_objects(
            unit,
            inner,
            aliases,
            visible_objects,
        )?);
        return Ok(
            if node_text(&unit.source, shape)
                .split_whitespace()
                .take(2)
                .eq(["shared", "ref"])
            {
                ValueType::SharedReference(inner)
            } else {
                ValueType::Reference(inner)
            },
        );
    }
    if shape.kind == SyntaxKind::FunctionType {
        let function = shape;
        let Some((result, parameters)) = function.children.split_last() else {
            return Err(failure(
                &unit.source,
                "T0001",
                "function type requires a result type",
                type_node.span,
            ));
        };
        let parameters = parameters
            .iter()
            .map(|parameter| {
                declared_value_type_with_visible_objects(unit, parameter, aliases, visible_objects)
                    .map(ElementType::new)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let result = ElementType::new(declared_value_type_with_visible_objects(
            unit,
            result,
            aliases,
            visible_objects,
        )?);
        return Ok(if node_text(&unit.source, function).starts_with("async") {
            ValueType::AsyncFunction(parameters, result)
        } else {
            ValueType::Function(parameters, result)
        });
    }
    if let Some(union) = type_node
        .children
        .first()
        .filter(|child| child.kind == SyntaxKind::UnionType)
    {
        let arms = union
            .children
            .iter()
            .filter(|arm| node_text(&unit.source, arm).trim() != "none")
            .collect::<Vec<_>>();
        if union.children.len() == 2 && arms.len() == 1 {
            let Some(arm) = arms.first().copied() else {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    "an optional type requires one non-`none` arm",
                    union.span,
                ));
            };
            let inner =
                declared_value_type_with_visible_objects(unit, arm, aliases, visible_objects)?;
            if matches!(
                inner,
                ValueType::Scalar(ScalarType::None) | ValueType::Optional(_)
            ) {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    "an optional type cannot contain `none` or another optional type",
                    union.span,
                ));
            }
            if !matches!(inner, ValueType::Scalar(_) | ValueType::Object(_)) {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    "a general optional type requires a scalar or object value",
                    union.span,
                ));
            }
            return Ok(ValueType::Optional(Box::new(inner)));
        }
    }
    let type_name = node_text(&unit.source, type_node).trim();
    let lexical_identity = lexical_scope_chain(unit, type_node.span.start).find_map(|scope| {
        scope.symbols.get(type_name).and_then(|symbols| {
            symbols.iter().rev().find_map(|symbol| {
                matches!(
                    symbol.kind,
                    SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                )
                .then(|| ObjectIdentity::new(&symbol.namespace, &symbol.name))
            })
        })
    });
    let object_identity = lexical_identity
        .or_else(|| visible_objects.get(type_name).cloned())
        .or_else(|| {
            unit.objects
                .iter()
                .find(|object| object.name == type_name)
                .map(|object| object.identity.clone())
        });
    if let Some(identity) = object_identity {
        return Ok(ValueType::Object(identity));
    }
    for (constructor, construct) in [
        ("list of ", ValueType::List as fn(ElementType) -> ValueType),
        (
            "tuple of ",
            (|item| ValueType::Tuple(item, None)) as fn(ElementType) -> ValueType,
        ),
        (
            "iterator of ",
            ValueType::Iterator as fn(ElementType) -> ValueType,
        ),
    ] {
        if let Some(argument) = type_name.strip_prefix(constructor) {
            let argument = argument.trim();
            let lexical_identity =
                lexical_scope_chain(unit, type_node.span.start).find_map(|scope| {
                    scope.symbols.get(argument).and_then(|symbols| {
                        symbols.iter().rev().find_map(|symbol| {
                            matches!(
                                symbol.kind,
                                SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                            )
                            .then(|| ObjectIdentity::new(&symbol.namespace, &symbol.name))
                        })
                    })
                });
            let object_identity = lexical_identity
                .or_else(|| visible_objects.get(argument).cloned())
                .or_else(|| {
                    unit.objects
                        .iter()
                        .find(|object| object.name == argument)
                        .map(|object| object.identity.clone())
                });
            if let Some(identity) = object_identity {
                return Ok(construct(ElementType::new(ValueType::Object(identity))));
            }
        }
    }
    match type_name {
        "host-resource-handle" => return Ok(ValueType::PlatformStreamHandle),
        "host-filesystem-authority" => return Ok(ValueType::FilesystemAuthority),
        "host-platform-data-result" => return Ok(ValueType::PlatformDataResult),
        "host-platform-url-result" => return Ok(ValueType::PlatformUrlResult),
        "host-platform-capability" => return Ok(ValueType::PlatformCapability),
        "host-platform-resource-handle" => return Ok(ValueType::PlatformResourceHandle),
        "host-platform-result" => return Ok(ValueType::PlatformResult),
        _ => {}
    }
    if type_name == "encoding" {
        return Ok(ValueType::Encoding);
    }
    parse_declared_value_type(type_name, aliases).ok_or_else(|| {
        failure(
            &unit.source,
            "T0001",
            format!("`{type_name}` does not resolve to a type descriptor"),
            type_node.span,
        )
    })
}

fn parse_declared_value_type(
    type_name: &str,
    aliases: &BTreeMap<String, ScalarType>,
) -> Option<ValueType> {
    if matches!(
        type_name,
        "throwable"
            | "arithmetic-overflow"
            | "division-by-zero"
            | "integer-conversion-overflow"
            | "negative-shift-count"
            | "coercion-error"
            | "decode-error"
            | "index-error"
            | "missing-key"
            | "dependency-error"
            | "dependency-panic"
    ) {
        return Some(ValueType::Object(ObjectIdentity::new(
            "/core/errors",
            type_name,
        )));
    }
    let type_name = type_name.trim();
    if let Some(scalar) = aliases
        .get(type_name)
        .copied()
        .or_else(|| ScalarType::from_source_name(type_name))
    {
        return Some(ValueType::Scalar(scalar));
    }
    for (constructor, construct) in [
        (
            "overflow-result of ",
            ValueType::OverflowResult as fn(ScalarType) -> ValueType,
        ),
        (
            "div-rem-result of ",
            ValueType::DivRemResult as fn(ScalarType) -> ValueType,
        ),
    ] {
        if let Some(argument) = type_name.strip_prefix(constructor)
            && let Some(scalar) = aliases
                .get(argument.trim())
                .copied()
                .or_else(|| ScalarType::from_source_name(argument.trim()))
        {
            return Some(construct(scalar));
        }
    }
    for (constructor, construct) in [
        ("list of ", ValueType::List as fn(ElementType) -> ValueType),
        (
            "tuple of ",
            (|item| ValueType::Tuple(item, None)) as fn(ElementType) -> ValueType,
        ),
        ("set of ", ValueType::Set as fn(ElementType) -> ValueType),
        (
            "unordered-set of ",
            ValueType::UnorderedSet as fn(ElementType) -> ValueType,
        ),
        (
            "iterator of ",
            ValueType::Iterator as fn(ElementType) -> ValueType,
        ),
        (
            "iteration-step of ",
            ValueType::IterationStep as fn(ElementType) -> ValueType,
        ),
    ] {
        if let Some(argument) = type_name.strip_prefix(constructor) {
            let item = ElementType::new(parse_declared_value_type(argument, aliases)?);
            if matches!(constructor, "set of " | "unordered-set of ") && item.scalar().is_none() {
                return None;
            }
            return Some(construct(item));
        }
    }
    for (constructor, construct) in [
        (
            "map of ",
            ValueType::Map as fn(ElementType, ElementType) -> ValueType,
        ),
        (
            "unordered-map of ",
            ValueType::UnorderedMap as fn(ElementType, ElementType) -> ValueType,
        ),
        (
            "entry of ",
            ValueType::Entry as fn(ElementType, ElementType) -> ValueType,
        ),
    ] {
        if let Some(arguments) = type_name.strip_prefix(constructor)
            && let Some((key, value)) = arguments.split_once(',')
        {
            let key = ElementType::new(parse_declared_value_type(key, aliases)?);
            key.scalar()?;
            let value = ElementType::new(parse_declared_value_type(value, aliases)?);
            return Some(construct(key, value));
        }
    }
    None
}

fn union_destination_candidates(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
) -> Result<Vec<ScalarType>, SemanticFailure> {
    let Some(union) = type_node
        .children
        .first()
        .filter(|child| child.kind == SyntaxKind::UnionType)
    else {
        return Err(failure(
            &unit.source,
            "T0001",
            format!(
                "`{}` does not resolve to a scalar type descriptor",
                node_text(&unit.source, type_node).trim()
            ),
            type_node.span,
        ));
    };
    let mut candidates = Vec::new();
    for arm in &union.children {
        let name = node_text(&unit.source, arm).trim();
        let candidate = unit
            .descriptor_alias_at(name, arm.span.start)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0001",
                    format!("`{name}` does not resolve to a scalar type descriptor"),
                    arm.span,
                )
            })?;
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

fn select_union_destination(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
    value: &SyntaxNode,
    actual: ValueType,
) -> Result<ScalarType, SemanticFailure> {
    select_union_candidates(
        &unit.source,
        value,
        actual,
        union_destination_candidates(unit, type_node)?,
    )
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "union selection owns the inferred recursive value type for complete matching"
)]
fn select_union_candidates(
    source: &SourceFile,
    value: &SyntaxNode,
    actual: ValueType,
    candidates: Vec<ScalarType>,
) -> Result<ScalarType, SemanticFailure> {
    let is_constant = candidates
        .iter()
        .any(|candidate| contextual_constant(source, value, *candidate).is_some());
    if !is_constant
        && let ValueType::Scalar(actual) = actual
        && candidates.contains(&actual)
    {
        return Ok(actual);
    }
    let admitted = candidates
        .into_iter()
        .filter(|candidate| {
            if let Some(result) = contextual_constant(source, value, *candidate) {
                return result.is_ok();
            }
            matches!(actual, ValueType::Scalar(actual) if is_numeric(actual) && is_numeric(*candidate))
        })
        .collect::<Vec<_>>();
    match admitted.as_slice() {
        [candidate] => Ok(*candidate),
        [] => Err(failure(
            source,
            "T0002",
            "value is not admitted by any union destination arm",
            value.span,
        )),
        candidates => Err(failure(
            source,
            "T0002",
            format!(
                "numeric destination is ambiguous between {}",
                candidates
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            value.span,
        )),
    }
}

fn validate_numeric_destination(
    source: &SourceFile,
    name: &str,
    expected: ScalarType,
    actual: ValueType,
    value: &SyntaxNode,
    mismatch_code: &'static str,
) -> Result<(), SemanticFailure> {
    let ValueType::Scalar(actual) = actual else {
        return Err(failure(
            source,
            mismatch_code,
            destination_mismatch_message(mismatch_code, name, expected, actual),
            value.span,
        ));
    };
    if is_numeric(expected)
        && let Some(constant) = contextual_constant(source, value, expected)
    {
        constant?;
        return Ok(());
    }
    if actual == expected {
        return Ok(());
    }
    if is_numeric(actual) && is_numeric(expected) {
        return Ok(());
    }
    Err(failure(
        source,
        mismatch_code,
        destination_mismatch_message(mismatch_code, name, expected, ValueType::Scalar(actual)),
        value.span,
    ))
}

fn diagnostic_object_identity(objects: &[ObjectContract], identity: &ObjectIdentity) -> String {
    let identities = objects
        .iter()
        .filter(|object| object.identity.name == identity.name)
        .map(|object| &object.identity)
        .collect::<BTreeSet<_>>();
    if identities.len() > 1 {
        identity.qualified()
    } else {
        identity.name.clone()
    }
}

fn diagnostic_value_type(objects: &[ObjectContract], value_type: &ValueType) -> String {
    let nested = |item: &ElementType| diagnostic_value_type(objects, &item.value_type());
    match value_type {
        ValueType::Optional(inner) => {
            format!("{}|none", diagnostic_value_type(objects, inner))
        }
        ValueType::Object(identity) => diagnostic_object_identity(objects, identity),
        ValueType::Iterator(item) => format!("iterator of {}", nested(item)),
        ValueType::IterationStep(item) => format!("iteration-step of {}", nested(item)),
        ValueType::List(item) => format!("list of {}", nested(item)),
        ValueType::Map(key, value) => format!("map of {}, {}", nested(key), nested(value)),
        ValueType::Set(item) => format!("set of {}", nested(item)),
        ValueType::Tuple(item, _) => format!("tuple of {}", nested(item)),
        ValueType::Entry(key, value) => format!("entry of {}, {}", nested(key), nested(value)),
        ValueType::UnorderedMap(key, value) => {
            format!("unordered-map of {}, {}", nested(key), nested(value))
        }
        ValueType::UnorderedSet(item) => format!("unordered-set of {}", nested(item)),
        ValueType::Function(parameters, result) | ValueType::AsyncFunction(parameters, result) => {
            let prefix = if matches!(value_type, ValueType::AsyncFunction(..)) {
                "async function"
            } else {
                "function"
            };
            let parameters = parameters.iter().map(nested).collect::<Vec<_>>().join(", ");
            let from = if parameters.is_empty() {
                String::new()
            } else {
                format!(" from {parameters}")
            };
            format!("{prefix}{from} to {}", nested(result))
        }
        ValueType::Task(result) => format!("task of {}", nested(result)),
        ValueType::ScopedTask(result) => format!("scoped task of {}", nested(result)),
        ValueType::TaskOutcome(result) => format!("task-outcome of {}", nested(result)),
        ValueType::Reference(item) => format!("ref {}", nested(item)),
        ValueType::SharedReference(item) => format!("shared ref {}", nested(item)),
        _ => value_type.to_string(),
    }
}

fn validate_value_destination(
    source: &SourceFile,
    objects: &[ObjectContract],
    name: &str,
    expected: ValueType,
    actual: ValueType,
    value: &SyntaxNode,
    mismatch_code: &'static str,
) -> Result<(), SemanticFailure> {
    if let ValueType::Scalar(expected) = expected {
        return validate_numeric_destination(source, name, expected, actual, value, mismatch_code);
    }
    if let ValueType::Optional(expected_inner) = expected {
        if actual == ValueType::Scalar(ScalarType::None) {
            return Ok(());
        }
        if let ValueType::Optional(actual_inner) = actual {
            return validate_value_destination(
                source,
                objects,
                name,
                *expected_inner,
                *actual_inner,
                value,
                mismatch_code,
            );
        }
        return validate_value_destination(
            source,
            objects,
            name,
            *expected_inner,
            actual,
            value,
            mismatch_code,
        );
    }
    if value_types_compatible(objects, &expected, &actual) {
        return Ok(());
    }
    Err(failure(
        source,
        mismatch_code,
        format!(
            "`{name}` requires `{}`, found `{}`",
            diagnostic_value_type(objects, &expected),
            diagnostic_value_type(objects, &actual)
        ),
        value.span,
    ))
}

fn value_types_compatible(
    objects: &[ObjectContract],
    expected: &ValueType,
    actual: &ValueType,
) -> bool {
    match (expected, actual) {
        (ValueType::Optional(expected), ValueType::Optional(actual)) => {
            value_types_compatible(objects, expected, actual)
        }
        (ValueType::Tuple(expected_item, None), ValueType::Tuple(actual_item, _)) => {
            value_types_compatible(
                objects,
                &expected_item.value_type(),
                &actual_item.value_type(),
            )
        }
        (
            ValueType::Tuple(expected_item, Some(expected_length)),
            ValueType::Tuple(actual_item, Some(actual_length)),
        ) => {
            expected_length == actual_length
                && value_types_compatible(
                    objects,
                    &expected_item.value_type(),
                    &actual_item.value_type(),
                )
        }
        (ValueType::List(expected), ValueType::List(actual))
        | (ValueType::Set(expected), ValueType::Set(actual))
        | (ValueType::UnorderedSet(expected), ValueType::UnorderedSet(actual))
        | (ValueType::Iterator(expected), ValueType::Iterator(actual))
        | (ValueType::IterationStep(expected), ValueType::IterationStep(actual)) => {
            value_types_compatible(objects, &expected.value_type(), &actual.value_type())
        }
        (
            ValueType::Map(expected_key, expected_value),
            ValueType::Map(actual_key, actual_value),
        )
        | (
            ValueType::UnorderedMap(expected_key, expected_value),
            ValueType::UnorderedMap(actual_key, actual_value),
        )
        | (
            ValueType::Entry(expected_key, expected_value),
            ValueType::Entry(actual_key, actual_value),
        ) => {
            value_types_compatible(
                objects,
                &expected_key.value_type(),
                &actual_key.value_type(),
            ) && value_types_compatible(
                objects,
                &expected_value.value_type(),
                &actual_value.value_type(),
            )
        }
        (ValueType::Object(expected), ValueType::Object(actual)) => {
            expected == actual
                || objects
                    .iter()
                    .find(|object| object.identity == *actual)
                    .is_some_and(|object| {
                        if object.interfaces.contains(expected) {
                            return true;
                        }
                        let mut base = object.base.as_ref();
                        while let Some(identity) = base {
                            let Some(base_object) =
                                objects.iter().find(|object| object.identity == *identity)
                            else {
                                break;
                            };
                            if base_object.identity == *expected
                                || base_object.interfaces.contains(expected)
                            {
                                return true;
                            }
                            base = base_object.base.as_ref();
                        }
                        false
                    })
        }
        _ => expected == actual,
    }
}

fn erase_tuple_lengths(value_type: ValueType) -> ValueType {
    match value_type {
        ValueType::Tuple(item, _) => ValueType::Tuple(
            ElementType::new(erase_tuple_lengths(item.value_type())),
            None,
        ),
        ValueType::List(item) => {
            ValueType::List(ElementType::new(erase_tuple_lengths(item.value_type())))
        }
        ValueType::Set(item) => {
            ValueType::Set(ElementType::new(erase_tuple_lengths(item.value_type())))
        }
        ValueType::UnorderedSet(item) => {
            ValueType::UnorderedSet(ElementType::new(erase_tuple_lengths(item.value_type())))
        }
        ValueType::Map(key, value) => ValueType::Map(
            ElementType::new(erase_tuple_lengths(key.value_type())),
            ElementType::new(erase_tuple_lengths(value.value_type())),
        ),
        ValueType::UnorderedMap(key, value) => ValueType::UnorderedMap(
            ElementType::new(erase_tuple_lengths(key.value_type())),
            ElementType::new(erase_tuple_lengths(value.value_type())),
        ),
        ValueType::Entry(key, value) => ValueType::Entry(
            ElementType::new(erase_tuple_lengths(key.value_type())),
            ElementType::new(erase_tuple_lengths(value.value_type())),
        ),
        other => other,
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "diagnostic rendering owns the recursive actual type description"
)]
fn destination_mismatch_message(
    code: &str,
    name: &str,
    expected: ScalarType,
    actual: ValueType,
) -> String {
    match code {
        "T0012" => {
            format!("argument for parameter `{name}` has type `{actual}`, expected `{expected}`")
        }
        "T0015" => format!("function `{name}` must return `{expected}`"),
        _ => format!("cannot assign `{actual}` to `{name}` of type `{expected}`"),
    }
}

const fn is_numeric(ty: ScalarType) -> bool {
    ty.is_integer() || matches!(ty, ScalarType::Float32 | ScalarType::Float64)
}

fn optional_inner(value_type: ValueType) -> Option<ValueType> {
    match value_type {
        ValueType::Optional(inner) => Some(*inner),
        _ => None,
    }
}

fn ungrouped_expression(mut node: &SyntaxNode) -> &SyntaxNode {
    while node.kind == SyntaxKind::GroupExpression {
        let Some(child) = node.children.first() else {
            break;
        };
        node = child;
    }
    node
}

fn membership_names<'a>(
    source: &'a SourceFile,
    node: &'a SyntaxNode,
) -> Option<(&'a str, &'a str)> {
    let [left, right] = node.children.as_slice() else {
        return None;
    };
    let left = ungrouped_expression(left);
    let right = ungrouped_expression(right);
    Some((node_text(source, left), node_text(source, right)))
}

fn condition_proves_present(source: &SourceFile, node: &SyntaxNode, name: &str) -> bool {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .is_some_and(|child| condition_proves_present(source, child, name));
    }
    if node.kind == SyntaxKind::BinaryExpression {
        let [left, right] = node.children.as_slice() else {
            return false;
        };
        let operator = source.text()[left.span.end..right.span.start].trim();
        let left = ungrouped_expression(left);
        let right = ungrouped_expression(right);
        let names = (node_text(source, left), node_text(source, right));
        return operator == "!="
            && matches!(names, (left, "none") | ("none", left) if left == name);
    }
    if node.kind == SyntaxKind::UnaryExpression
        && node.children.iter().any(|child| {
            child.kind == SyntaxKind::UnaryOperator && node_text(source, child) == "not"
        })
        && let Some(child) = node
            .children
            .iter()
            .find(|child| child.kind != SyntaxKind::UnaryOperator)
    {
        let child = ungrouped_expression(child);
        return child.kind == SyntaxKind::TypeMembershipExpression
            && membership_names(source, child).is_some_and(|names| names == (name, "none"));
    }
    false
}

fn is_presence_test_occurrence(
    source: &SourceFile,
    current: &SyntaxNode,
    position: usize,
    name: &str,
) -> bool {
    if current.kind == SyntaxKind::BinaryExpression
        && condition_proves_present(source, current, name)
    {
        return true;
    }
    current
        .children
        .iter()
        .filter(|child| child.span.start <= position && position <= child.span.end)
        .any(|child| is_presence_test_occurrence(source, child, position, name))
}

fn assigns_name_before(
    source: &SourceFile,
    node: &SyntaxNode,
    position: usize,
    name: &str,
) -> bool {
    if node.span.start >= position {
        return false;
    }
    if node.kind == SyntaxKind::Assignment
        && node
            .children
            .first()
            .is_some_and(|target| node_text(source, target) == name)
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| assigns_name_before(source, child, position, name))
}

fn enclosed_by_present_guard(
    source: &SourceFile,
    current: &SyntaxNode,
    position: usize,
    name: &str,
) -> bool {
    if current.kind == SyntaxKind::IfStatement
        && let Some(condition) = current.children.first()
        && let Some(block) = current.children.iter().find(|child| {
            child.kind == SyntaxKind::Block
                && child.span.start <= position
                && position <= child.span.end
        })
    {
        if condition_proves_present(source, condition, name)
            && !assigns_name_before(source, block, position, name)
        {
            return true;
        }
        return enclosed_by_present_guard(source, block, position, name);
    }
    current
        .children
        .iter()
        .filter(|child| child.span.start <= position && position <= child.span.end)
        .any(|child| enclosed_by_present_guard(source, child, position, name))
}

pub(crate) fn narrowed_optional_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    value_type: ValueType,
) -> Option<ValueType> {
    let name = node_text(&unit.source, node);
    if is_presence_test_occurrence(&unit.source, &unit.tree.root, node.span.start, name) {
        return None;
    }
    let inner = optional_inner(value_type)?;
    enclosed_by_present_guard(&unit.source, &unit.tree.root, node.span.start, name).then_some(inner)
}

pub(crate) fn narrowed_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Option<ValueType> {
    let name = node_text(&unit.source, node);
    let function_span = unit
        .enclosing_function_spans
        .get(&node.span.start)
        .copied()
        .flatten();
    let binding = bindings.iter().rev().find(|binding| {
        binding.name == name
            && binding.is_visible_at(unit.source.id(), node.span.start)
            && unit
                .enclosing_function_spans
                .get(&binding.span.start)
                .copied()
                .flatten()
                == function_span
    })?;
    narrowed_optional_type(unit, node, binding.value_type.clone())
}
#[expect(
    clippy::too_many_lines,
    reason = "value inference centralizes the precedence among syntax forms and typed member families"
)]
fn infer_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    if node.kind == SyntaxKind::Literal {
        return Ok(infer_literal_type(unit, node).map(ValueType::Scalar));
    }
    if node.kind == SyntaxKind::AnonymousFunction {
        let contract = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed closure must have a semantic contract");
        let parameters = contract
            .parameters
            .iter()
            .map(|parameter| {
                parameter
                    .value_type
                    .clone()
                    .map(ElementType::new)
                    .ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0052",
                            "stored function parameters require explicit types",
                            parameter.span,
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let result = ElementType::new(
            contract
                .return_type
                .clone()
                .unwrap_or(ValueType::Scalar(ScalarType::None)),
        );
        return Ok(Some(ValueType::Function(parameters, result)));
    }
    if node.kind == SyntaxKind::GroupExpression {
        return match node.children.first() {
            Some(child) => infer_value_type(unit, child, bindings),
            None => Ok(None),
        };
    }
    if node.kind == SyntaxKind::UnaryExpression {
        return infer_unary_type(unit, node, bindings).map(Some);
    }
    if node.kind == SyntaxKind::BinaryExpression {
        return infer_binary_type(unit, node, bindings).map(Some);
    }
    if node.kind == SyntaxKind::TypeMembershipExpression {
        return Ok(Some(ValueType::Scalar(ScalarType::Bool)));
    }
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        if name == "none" {
            return Ok(Some(ValueType::Scalar(ScalarType::None)));
        }
        if let Some(scalar) = ScalarType::from_source_name(name).or_else(|| {
            visible_descriptor_aliases(&unit.descriptor_aliases, unit.source.id(), node.span.start)
                .get(name)
                .copied()
        }) {
            return Ok(Some(ValueType::Descriptor(scalar.source_name().to_owned())));
        }
        if unit.objects.iter().any(|object| object.name == name) {
            return Ok(Some(ValueType::Descriptor(name.to_owned())));
        }
        if let Some(binding) = bindings.iter().rev().find(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), node.span.start)
        }) {
            return Ok(Some(
                narrowed_value_type(unit, node, bindings).unwrap_or(binding.value_type.clone()),
            ));
        }
        if let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.owner.is_none() && contract.name == name)
        {
            let parameters = contract
                .parameters
                .iter()
                .map(|parameter| parameter.value_type.clone().map(ElementType::new))
                .collect::<Option<Vec<_>>>();
            if let Some(parameters) = parameters {
                let result = ElementType::new(
                    contract
                        .return_type
                        .clone()
                        .unwrap_or(ValueType::Scalar(ScalarType::None)),
                );
                return Ok(Some(if contract.is_async {
                    ValueType::AsyncFunction(parameters, result)
                } else {
                    ValueType::Function(parameters, result)
                }));
            }
        }
        let resolved_symbol = lexical_scope_chain(unit, node.span.start).find_map(|scope| {
            scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol
                    .declaration_span
                    .is_none_or(|span| span.end <= node.span.start)
            })
        });
        let resolved_encoding = resolved_symbol
            .map(|symbol| symbol.identity.as_str())
            .or_else(|| {
                unit.prelude.then_some(name).and_then(|name| {
                    matches!(
                        name,
                        "utf8" | "utf16-le" | "utf16-be" | "utf32-le" | "utf32-be"
                    )
                    .then_some(name)
                })
            })
            .is_some_and(|identity| {
                identity.starts_with("/core/encodings::")
                    || matches!(
                        identity,
                        "utf8" | "utf16-le" | "utf16-be" | "utf32-le" | "utf32-be"
                    )
            });
        return Ok(resolved_encoding.then_some(ValueType::Encoding));
    }
    if member_family_receiver(unit, node) {
        return Err(failure(
            &unit.source,
            "T0018",
            "member-family selections must be invoked in the same expression",
            node.span,
        ));
    }
    if node.kind == SyntaxKind::IndexExpression {
        let Some(receiver) = node.children.first() else {
            return Ok(None);
        };
        return match infer_receiver_value_type(unit, receiver, bindings)? {
            Some(ValueType::List(item) | ValueType::Tuple(item, _)) => Ok(Some(item.value_type())),
            Some(ValueType::StringList) => Ok(Some(ValueType::Scalar(ScalarType::String))),
            Some(ValueType::Map(_, value) | ValueType::UnorderedMap(_, value)) => {
                Ok(Some(value.value_type()))
            }
            Some(ValueType::Scalar(ScalarType::String)) => Err(failure(
                &unit.source,
                "T0050",
                "string indexing is not implemented yet",
                receiver.span,
            )),
            Some(other) => Err(failure(
                &unit.source,
                "T0050",
                format!("indexing is not supported for `{other}`"),
                receiver.span,
            )),
            None => Err(failure(
                &unit.source,
                "T0050",
                "indexing requires a receiver with a statically known collection type",
                receiver.span,
            )),
        };
    }
    if node.kind == SyntaxKind::MemberExpression {
        return infer_member_value_type(unit, node, bindings);
    }
    if node.kind == SyntaxKind::StaticMemberExpression {
        let [receiver, member] = node.children.as_slice() else {
            return Ok(None);
        };
        let identity = class_designator_identity(unit, receiver).ok_or_else(|| {
            failure(
                &unit.source,
                "T0104",
                "the left side of `::` must resolve to a class",
                receiver.span,
            )
        })?;
        return object_member_type(unit, &identity, node_text(&unit.source, member), true)
            .map(Some)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0105",
                    format!(
                        "class `{}` has no static member `{}`",
                        identity.name,
                        node_text(&unit.source, member)
                    ),
                    member.span,
                )
            });
    }
    if node.kind == SyntaxKind::CallExpression {
        if let [callee, arguments] = node.children.as_slice() {
            if callee.kind == SyntaxKind::ConstructionExpression {
                let class = callee.children.first().ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0103",
                        "construction requires a class",
                        callee.span,
                    )
                })?;
                let identity = class_designator_identity(unit, class).ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0103",
                        format!(
                            "`{}` does not resolve to a constructible class",
                            node_text(&unit.source, class)
                        ),
                        class.span,
                    )
                })?;
                return Ok(Some(ValueType::Object(identity)));
            }
            if callee.kind == SyntaxKind::StaticMemberExpression {
                let [receiver, member] = callee.children.as_slice() else {
                    return Ok(None);
                };
                let identity = class_designator_identity(unit, receiver).ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0104",
                        "the left side of `::` must resolve to a class",
                        receiver.span,
                    )
                })?;
                let member_type =
                    object_member_type(unit, &identity, node_text(&unit.source, member), true)
                        .ok_or_else(|| {
                            failure(
                                &unit.source,
                                "T0105",
                                format!(
                                    "class `{}` has no static member `{}`",
                                    identity.name,
                                    node_text(&unit.source, member)
                                ),
                                member.span,
                            )
                        })?;
                return match member_type {
                    ValueType::Function(_, result) => {
                        let result = result.value_type();
                        let result = object_method_contract(
                            unit,
                            &identity,
                            node_text(&unit.source, member),
                            true,
                        )
                        .filter(|method| {
                            matches!(
                                &result,
                                ValueType::Object(returned)
                                    if method.owner.as_deref() == Some(returned.name.as_str())
                            )
                        })
                        .map_or(result, |_| ValueType::Object(identity));
                        Ok(Some(result))
                    }
                    ValueType::AsyncFunction(_, result) => Ok(Some(ValueType::Task(result))),
                    _ => Err(failure(
                        &unit.source,
                        "T0039",
                        format!(
                            "`{}::{}` is a property and cannot be invoked",
                            identity.name,
                            node_text(&unit.source, member)
                        ),
                        callee.span,
                    )),
                };
            }
            if callee.kind == SyntaxKind::Name
                && resolved_compiler_identity(unit, callee)
                    .is_some_and(|identity| identity == "/core/async::task-scope")
            {
                return Ok(Some(ValueType::TaskScope));
            }
            if callee.kind == SyntaxKind::Name
                && let Some(identity) = resolved_compiler_identity(unit, callee)
            {
                let platform_result = match identity {
                    "intrinsic:streams::acquire-stdin"
                    | "intrinsic:streams::acquire-stdout"
                    | "intrinsic:streams::acquire-stderr" => Some(ValueType::PlatformStreamHandle),
                    "intrinsic:system::acquire-filesystem-authority" => {
                        Some(ValueType::FilesystemAuthority)
                    }
                    "intrinsic:streams::open-file"
                    | "intrinsic:streams::open-directory-beneath"
                    | "intrinsic:streams::open-file-beneath" => Some(ValueType::PlatformOpenResult),
                    "intrinsic:streams::read" => Some(ValueType::PlatformReadResult),
                    "intrinsic:streams::write" => Some(ValueType::PlatformWriteResult),
                    "intrinsic:streams::flush"
                    | "intrinsic:streams::sync-data"
                    | "intrinsic:streams::sync-all"
                    | "intrinsic:streams::close"
                    | "intrinsic:streams::release" => Some(ValueType::PlatformUnitResult),
                    "intrinsic:data::empty-document"
                    | "intrinsic:data::make-document-none"
                    | "intrinsic:data::make-document-bool"
                    | "intrinsic:data::make-document-string"
                    | "intrinsic:data::make-document-integer"
                    | "intrinsic:data::make-document-decimal"
                    | "intrinsic:data::make-document-list"
                    | "intrinsic:data::make-document-map"
                    | "intrinsic:data::document-list-append"
                    | "intrinsic:data::document-map-insert"
                    | "intrinsic:data::json-parse"
                    | "intrinsic:data::json-canonical"
                    | "intrinsic:data::yaml-parse"
                    | "intrinsic:data::document-item"
                    | "intrinsic:data::document-field"
                    | "intrinsic:data::validate-mapping" => Some(ValueType::PlatformDataResult),
                    "intrinsic:data::url-parse" => Some(ValueType::PlatformUrlResult),
                    "intrinsic:capabilities::secure-random"
                    | "intrinsic:capabilities::cancellation-token"
                    | "intrinsic:capabilities::pseudo-random"
                    | "intrinsic:capabilities::secret-buffer"
                    | "intrinsic:capabilities::result-capability"
                    | "intrinsic:concurrency::platform-capability"
                    | "intrinsic:concurrency::no-capability" => Some(ValueType::PlatformCapability),
                    "intrinsic:capabilities::result-resource"
                    | "intrinsic:capabilities::no-resource" => {
                        Some(ValueType::PlatformResourceHandle)
                    }
                    "intrinsic:capabilities::failed-result"
                    | "intrinsic:capabilities::random-bytes"
                    | "intrinsic:capabilities::random-bounded"
                    | "intrinsic:capabilities::random-split"
                    | "intrinsic:capabilities::digest"
                    | "intrinsic:capabilities::hmac"
                    | "intrinsic:capabilities::destroy-secret"
                    | "intrinsic:capabilities::hex-decode"
                    | "intrinsic:capabilities::base64-decode"
                    | "intrinsic:capabilities::uuid-parse"
                    | "intrinsic:capabilities::uuid-v4"
                    | "intrinsic:capabilities::uuid-v7"
                    | "intrinsic:capabilities::compress"
                    | "intrinsic:capabilities::decompress"
                    | "intrinsic:capabilities::parse-ip"
                    | "intrinsic:capabilities::parse-host-name"
                    | "intrinsic:capabilities::parse-socket"
                    | "intrinsic:capabilities::parse-socket-text"
                    | "intrinsic:capabilities::tcp-bind"
                    | "intrinsic:capabilities::tcp-connect"
                    | "intrinsic:capabilities::tcp-connect-host"
                    | "intrinsic:capabilities::tcp-accept"
                    | "intrinsic:capabilities::tcp-read"
                    | "intrinsic:capabilities::tcp-write"
                    | "intrinsic:capabilities::tcp-shutdown"
                    | "intrinsic:capabilities::tcp-configure"
                    | "intrinsic:capabilities::udp-bind"
                    | "intrinsic:capabilities::udp-configure"
                    | "intrinsic:capabilities::udp-send-to"
                    | "intrinsic:capabilities::udp-receive-from"
                    | "intrinsic:capabilities::dns-lookup"
                    | "intrinsic:capabilities::tls-client"
                    | "intrinsic:capabilities::tls-read"
                    | "intrinsic:capabilities::tls-write"
                    | "intrinsic:capabilities::tls-shutdown"
                    | "intrinsic:capabilities::cancel"
                    | "intrinsic:capabilities::close"
                    | "intrinsic:concurrency::platform-result"
                    | "intrinsic:concurrency::int-channel"
                    | "intrinsic:concurrency::int-mutex"
                    | "intrinsic:concurrency::int-read-write-lock"
                    | "intrinsic:concurrency::atomic-int64"
                    | "intrinsic:concurrency::thread-local-int"
                    | "intrinsic:concurrency::int-channel-send"
                    | "intrinsic:concurrency::int-channel-receive"
                    | "intrinsic:concurrency::int-channel-try-receive"
                    | "intrinsic:concurrency::int-mutex-load"
                    | "intrinsic:concurrency::int-mutex-store"
                    | "intrinsic:concurrency::int-mutex-add"
                    | "intrinsic:concurrency::int-read-write-lock-read"
                    | "intrinsic:concurrency::int-read-write-lock-write"
                    | "intrinsic:concurrency::atomic-int64-load"
                    | "intrinsic:concurrency::atomic-int64-store"
                    | "intrinsic:concurrency::atomic-int64-add"
                    | "intrinsic:concurrency::thread-local-int-get"
                    | "intrinsic:concurrency::thread-local-int-set"
                    | "intrinsic:adapters::platform-result"
                    | "intrinsic:adapters::system-host-name" => Some(ValueType::PlatformResult),
                    "intrinsic:system::filesystem-exists"
                    | "intrinsic:system::filesystem-metadata"
                    | "intrinsic:system::filesystem-realpath"
                    | "intrinsic:system::filesystem-read-link"
                    | "intrinsic:system::filesystem-read-bounded"
                    | "intrinsic:system::filesystem-write-atomic"
                    | "intrinsic:system::filesystem-rename"
                    | "intrinsic:system::filesystem-remove" => {
                        Some(ValueType::PlatformFilesystemResult)
                    }
                    "intrinsic:system::result-failed"
                    | "intrinsic:system::result-bool"
                    | "intrinsic:system::platform-value-is-text"
                    | "intrinsic:data::data-failed"
                    | "intrinsic:data::url-failed"
                    | "intrinsic:capabilities::constant-time-equal"
                    | "intrinsic:capabilities::result-failed"
                    | "intrinsic:capabilities::result-resource-limit"
                    | "intrinsic:capabilities::result-truncated"
                    | "intrinsic:capabilities::result-deadline-exceeded"
                    | "intrinsic:capabilities::result-bool"
                    | "intrinsic:concurrency::result-failed"
                    | "intrinsic:concurrency::result-bool"
                    | "intrinsic:adapters::result-failed"
                    | "intrinsic:adapters::result-bool" => {
                        Some(ValueType::Scalar(ScalarType::Bool))
                    }
                    "intrinsic:system::result-message"
                    | "intrinsic:system::result-text"
                    | "intrinsic:system::result-detail"
                    | "intrinsic:system::platform-value-text"
                    | "intrinsic:data::data-message"
                    | "intrinsic:data::data-path"
                    | "intrinsic:data::data-expected"
                    | "intrinsic:data::data-encoded"
                    | "intrinsic:data::document-kind"
                    | "intrinsic:data::document-text"
                    | "intrinsic:data::document-coefficient"
                    | "intrinsic:data::document-key"
                    | "intrinsic:data::url-message"
                    | "intrinsic:data::url-serialized"
                    | "intrinsic:data::url-display"
                    | "intrinsic:data::url-scheme"
                    | "intrinsic:data::url-username"
                    | "intrinsic:data::url-password"
                    | "intrinsic:data::url-host"
                    | "intrinsic:data::url-port"
                    | "intrinsic:data::url-path"
                    | "intrinsic:data::url-query-key"
                    | "intrinsic:data::url-query-value"
                    | "intrinsic:data::url-fragment"
                    | "intrinsic:data::url-origin"
                    | "intrinsic:capabilities::hex-encode"
                    | "intrinsic:capabilities::base64-encode"
                    | "intrinsic:capabilities::result-message"
                    | "intrinsic:capabilities::result-text"
                    | "intrinsic:capabilities::result-detail"
                    | "intrinsic:concurrency::result-message"
                    | "intrinsic:adapters::result-message"
                    | "intrinsic:adapters::result-text" => {
                        Some(ValueType::Scalar(ScalarType::String))
                    }
                    "intrinsic:system::result-bytes"
                    | "intrinsic:system::platform-value-bytes"
                    | "intrinsic:capabilities::result-bytes" => {
                        Some(ValueType::Scalar(ScalarType::Bytes))
                    }
                    "intrinsic:system::result-int"
                    | "intrinsic:data::document-exponent"
                    | "intrinsic:data::document-length"
                    | "intrinsic:data::url-query-length"
                    | "intrinsic:capabilities::result-int"
                    | "intrinsic:concurrency::result-int" => {
                        Some(ValueType::Scalar(ScalarType::Int))
                    }
                    "intrinsic:system::process-arguments"
                    | "intrinsic:system::environment-entries"
                    | "intrinsic:capabilities::result-entries" => Some(ValueType::StringList),
                    "intrinsic:system::process-exit" => Some(ValueType::Scalar(ScalarType::None)),
                    _ => None,
                };
                if platform_result.is_some() {
                    return Ok(platform_result);
                }
            }
            if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
                && matches!(
                    node_text(&unit.source, member),
                    "spawn" | "join" | "cancel" | "child-scope"
                )
                && infer_value_type(unit, receiver, bindings)? == Some(ValueType::TaskScope)
            {
                return match node_text(&unit.source, member) {
                    "spawn" => {
                        let Some(callable) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.spawn` requires one async callable",
                                node.span,
                            ));
                        };
                        let callable = callable.children.last().unwrap_or(callable);
                        match infer_value_type(unit, callable, bindings)? {
                            Some(ValueType::AsyncFunction(_, result)) => {
                                Ok(Some(ValueType::ScopedTask(result)))
                            }
                            _ => Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.spawn` requires an async callable value",
                                callable.span,
                            )),
                        }
                    }
                    "join" => {
                        let Some(task) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.join` requires one scoped task",
                                node.span,
                            ));
                        };
                        let task = task.children.last().unwrap_or(task);
                        match infer_value_type(unit, task, bindings)? {
                            Some(ValueType::ScopedTask(result)) => {
                                Ok(Some(ValueType::TaskOutcome(result)))
                            }
                            _ => Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.join` requires a scoped task",
                                task.span,
                            )),
                        }
                    }
                    "cancel" => Ok(Some(ValueType::Scalar(ScalarType::None))),
                    "child-scope" => {
                        let Some(argument) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.child-scope` requires one deadline",
                                node.span,
                            ));
                        };
                        let child = argument.children.last().unwrap_or(argument);
                        let parent_deadline =
                            task_scope_deadline_ms(unit, receiver, bindings, &mut BTreeSet::new());
                        let child_deadline =
                            constant_deadline_ms(unit, child, bindings, &mut BTreeSet::new());
                        if matches!(
                            (parent_deadline, child_deadline),
                            (Some(parent), Some(child)) if child > parent
                        ) {
                            return Err(failure(
                                &unit.source,
                                "T0075",
                                "a child scope cannot extend its parent deadline",
                                child.span,
                            ));
                        }
                        Ok(Some(ValueType::TaskScope))
                    }
                    _ => Ok(None),
                };
            }
        }
        if let Some(value_type) = infer_collection_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_iterator_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_string_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_float_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_arithmetic_family_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_parse_or_radix_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_integer_coercion_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && matches!(node_text(&unit.source, member), "concat" | "join")
        {
            let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
            if receiver_type == Some(ValueType::Scalar(ScalarType::String)) {
                return Ok(Some(ValueType::Scalar(ScalarType::String)));
            }
            if receiver_type == Some(ValueType::Scalar(ScalarType::Bytes))
                && node_text(&unit.source, member) == "concat"
            {
                return Ok(Some(ValueType::Scalar(ScalarType::Bytes)));
            }
            return Err(failure(
                &unit.source,
                "T0013",
                format!(
                    "`.{}` requires a `string` receiver{}; found `{}`",
                    node_text(&unit.source, member),
                    if node_text(&unit.source, member) == "concat" {
                        " or `bytes` receiver"
                    } else {
                        ""
                    },
                    receiver_type
                        .map_or_else(|| "unknown".to_owned(), |value_type| value_type.to_string())
                ),
                receiver.span,
            ));
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let Some(member_type) = infer_member_value_type(unit, callee, bindings)?
        {
            return match member_type {
                ValueType::Function(_, result) => Ok(Some(result.value_type())),
                ValueType::AsyncFunction(_, result) => Ok(Some(ValueType::Task(result))),
                _ => Err(failure(
                    &unit.source,
                    "T0039",
                    format!(
                        "`.{}` is a property and cannot be invoked",
                        node_text(
                            &unit.source,
                            callee.children.get(1).expect("member expression")
                        )
                    ),
                    callee.span,
                )),
            };
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
        {
            let name = node_text(&unit.source, callee);
            if unit
                .objects
                .iter()
                .any(|object| object.name == name && object.kind == ObjectKind::Class)
            {
                return Err(failure(
                    &unit.source,
                    "T0102",
                    format!("class `{name}` is not callable; construct it with `instance {name};`"),
                    callee.span,
                ));
            }
            if let Some(contract) = resolved_function_contract(unit, name, callee.span.start) {
                let result = ElementType::new(
                    contract
                        .return_type
                        .clone()
                        .unwrap_or(ValueType::Scalar(ScalarType::None)),
                );
                return Ok(Some(if contract.is_async {
                    ValueType::Task(result)
                } else {
                    result.value_type()
                }));
            }
            if let Some(binding) = bindings.iter().rev().find(|binding| {
                binding.name == name && binding.is_visible_at(unit.source.id(), callee.span.start)
            }) {
                return match &binding.value_type {
                    ValueType::Function(_, result) => Ok(Some(result.value_type())),
                    ValueType::AsyncFunction(_, result) => {
                        Ok(Some(ValueType::Task(result.clone())))
                    }
                    _ => Err(failure(
                        &unit.source,
                        "T0039",
                        format!("`{name}` is a value and cannot be called"),
                        callee.span,
                    )),
                };
            }
            return Ok(None);
        }
        return Ok(None);
    }
    Ok(None)
}

fn element_type(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<ElementType, SemanticFailure> {
    infer_value_type(unit, value, bindings)?
        .map(ElementType::new)
        .ok_or_else(|| {
            failure(
                &unit.source,
                "T0042",
                "collection items require a statically known value type",
                value.span,
            )
        })
}

fn homogeneous_element_type(
    unit: &SemanticUnit,
    arguments: &SyntaxNode,
    bindings: &[TypedBinding],
    collection: &str,
) -> Result<ElementType, SemanticFailure> {
    let mut item_type = None;
    for argument in &arguments.children {
        let value = argument.children.last().unwrap_or(argument);
        let inferred = ElementType::new(erase_tuple_lengths(
            element_type(unit, value, bindings)?.value_type(),
        ));
        if item_type.is_some_and(|existing| existing != inferred) {
            return Err(failure(
                &unit.source,
                "T0042",
                format!("`{collection}` items must have one statically known type"),
                value.span,
            ));
        }
        item_type = Some(inferred);
    }
    item_type.ok_or_else(|| {
        failure(
            &unit.source,
            "T0043",
            format!("an empty `{collection}` requires an explicit item type"),
            arguments.span,
        )
    })
}

fn collection_constructor_matches(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    expected: &ValueType,
    bindings: &[TypedBinding],
) -> bool {
    let Some(identity) = collection_constructor_identity(unit, node, bindings) else {
        return false;
    };
    let constructor = identity
        .strip_prefix("/core/collections::")
        .unwrap_or(identity);
    matches!(
        (constructor, expected),
        ("list", ValueType::List(_))
            | ("map", ValueType::Map(_, _))
            | ("unordered-map", ValueType::UnorderedMap(_, _))
            | ("set", ValueType::Set(_))
            | ("tuple", ValueType::Tuple(_, _))
            | ("unordered-set", ValueType::UnorderedSet(_))
            | ("entry", ValueType::Entry(_, _))
    )
}

fn contextual_collection_constructor_matches(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    expected: &ValueType,
    bindings: &[TypedBinding],
) -> bool {
    if node.kind == SyntaxKind::GroupExpression
        && let [grouped] = node.children.as_slice()
    {
        return contextual_collection_constructor_matches(unit, grouped, expected, bindings);
    }
    collection_constructor_matches(unit, node, expected, bindings)
}

fn validate_collection_constructor_items(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    expected: &ValueType,
    destination: &str,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let [_, arguments] = node.children.as_slice() else {
        return Ok(());
    };
    match expected {
        ValueType::List(item)
        | ValueType::Tuple(item, _)
        | ValueType::Set(item)
        | ValueType::UnorderedSet(item) => {
            for (index, argument) in arguments.children.iter().enumerate() {
                let value = argument.children.last().unwrap_or(argument);
                validate_collection_constructor_value(
                    unit,
                    value,
                    &item.value_type(),
                    &format!("{destination} item {}", index + 1),
                    bindings,
                )?;
            }
        }
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value) => {
            let entry_type = ValueType::Entry(key.clone(), value.clone());
            for (index, argument) in arguments.children.iter().enumerate() {
                let label = format!("{destination} entry {}", index + 1);
                if argument.children.len() >= 2 {
                    validate_value_destination(
                        &unit.source,
                        &unit.objects,
                        &format!("{label} key"),
                        key.value_type(),
                        ValueType::Scalar(ScalarType::String),
                        argument,
                        "T0002",
                    )?;
                    let entry_value = argument.children.last().unwrap_or(argument);
                    validate_collection_constructor_value(
                        unit,
                        entry_value,
                        &value.value_type(),
                        &format!("{label} value"),
                        bindings,
                    )?;
                } else {
                    let entry = argument.children.last().unwrap_or(argument);
                    validate_collection_constructor_value(
                        unit,
                        entry,
                        &entry_type,
                        &label,
                        bindings,
                    )?;
                }
            }
        }
        ValueType::Entry(key, value) => {
            let [key_argument, value_argument] = arguments.children.as_slice() else {
                return Err(failure(
                    &unit.source,
                    "T0045",
                    "`entry` requires exactly a key and value",
                    arguments.span,
                ));
            };
            let key_node = key_argument.children.last().unwrap_or(key_argument);
            let value_node = value_argument.children.last().unwrap_or(value_argument);
            validate_collection_constructor_value(
                unit,
                key_node,
                &key.value_type(),
                &format!("{destination} key"),
                bindings,
            )?;
            validate_collection_constructor_value(
                unit,
                value_node,
                &value.value_type(),
                &format!("{destination} value"),
                bindings,
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn validate_collection_constructor_value(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    expected: &ValueType,
    destination: &str,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    if value.kind == SyntaxKind::GroupExpression
        && let [grouped] = value.children.as_slice()
    {
        return validate_collection_constructor_value(
            unit,
            grouped,
            expected,
            destination,
            bindings,
        );
    }
    if value.kind == SyntaxKind::Name
        && matches!(expected, ValueType::Entry(_, _))
        && collection_constructor_identity(unit, value, bindings).is_some_and(|identity| {
            identity
                .strip_prefix("/core/collections::")
                .unwrap_or(identity)
                == "entry"
        })
    {
        return Err(failure(
            &unit.source,
            "T0045",
            "`entry` requires exactly a key and value",
            value.span,
        ));
    }
    if collection_constructor_matches(unit, value, expected, bindings) {
        return validate_collection_constructor_items(unit, value, expected, destination, bindings);
    }
    if let Some(actual) = infer_value_type(unit, value, bindings)? {
        validate_value_destination(
            &unit.source,
            &unit.objects,
            destination,
            expected.clone(),
            actual,
            value,
            "T0002",
        )?;
    }
    Ok(())
}

fn empty_collection_identity<'a>(
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    bindings: &[TypedBinding],
) -> Option<&'a str> {
    let is_empty = if node.kind == SyntaxKind::Name {
        true
    } else {
        let [_, arguments] = node.children.as_slice() else {
            return None;
        };
        node.kind == SyntaxKind::CallExpression && arguments.children.is_empty()
    };
    is_empty.then(|| collection_constructor_identity(unit, node, bindings))?
}

fn collection_constructor_identity<'a>(
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    bindings: &[TypedBinding],
) -> Option<&'a str> {
    let callee = if node.kind == SyntaxKind::Name {
        node
    } else {
        let [callee, _] = node.children.as_slice() else {
            return None;
        };
        (node.kind == SyntaxKind::CallExpression).then_some(callee)?
    };
    let identity = resolved_compiler_object_identity(unit, callee).or_else(|| {
        let name = node_text(&unit.source, callee);
        (!bindings.iter().rev().any(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), callee.span.start)
        }))
        .then_some(name)
    })?;
    matches!(
        identity
            .strip_prefix("/core/collections::")
            .unwrap_or(identity),
        "list" | "map" | "unordered-map" | "set" | "tuple" | "unordered-set" | "entry"
    )
    .then_some(identity)
}

#[expect(
    clippy::too_many_lines,
    reason = "collection construction inference centralizes one compiler-owned object family"
)]
fn infer_collection_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(None);
    };
    if callee.kind == SyntaxKind::MemberExpression
        && let [family, child] = callee.children.as_slice()
        && node_text(&unit.source, child) == "checked"
        && family.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = family.children.as_slice()
        && node_text(&unit.source, member) == "get"
        && let Some(receiver_type) = infer_receiver_value_type(unit, receiver, bindings)?
    {
        return Ok(match receiver_type {
            ValueType::List(item) | ValueType::Tuple(item, _) => {
                Some(ValueType::Optional(Box::new(item.value_type())))
            }
            ValueType::Map(_, value) | ValueType::UnorderedMap(_, value) => {
                Some(ValueType::Optional(Box::new(value.value_type())))
            }
            _ => None,
        });
    }
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && node_text(&unit.source, member) == "through"
        && (resolved_compiler_object_identity(unit, receiver) == Some("/core/collections::range")
            || (node_text(&unit.source, receiver) == "range"
                && !bindings.iter().rev().any(|binding| {
                    binding.name == "range"
                        && binding.is_visible_at(unit.source.id(), receiver.span.start)
                })))
    {
        return Ok(Some(ValueType::Range));
    }
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && matches!(
            node_text(&unit.source, member),
            "append" | "set" | "add" | "contains" | "remove" | "keys" | "values" | "entries"
        )
        && let Some(receiver_type) = infer_receiver_value_type(unit, receiver, bindings)?
    {
        let member = node_text(&unit.source, member);
        return Ok(match (receiver_type, member) {
            (ValueType::List(item), "append" | "set") => Some(ValueType::List(item)),
            (ValueType::Map(key, value), "set") => Some(ValueType::Map(key, value)),
            (ValueType::UnorderedMap(key, value), "set") => {
                Some(ValueType::UnorderedMap(key, value))
            }
            (ValueType::Set(item), "add") => Some(ValueType::Set(item)),
            (ValueType::UnorderedSet(item), "add") => Some(ValueType::UnorderedSet(item)),
            (ValueType::Tuple(_, _), "append" | "set" | "add" | "remove") => {
                return Err(failure(
                    &unit.source,
                    "T0048",
                    "tuple items and length are fixed at construction",
                    callee.span,
                ));
            }
            (ValueType::Set(_) | ValueType::UnorderedSet(_), "contains" | "remove") => {
                Some(ValueType::Scalar(ScalarType::Bool))
            }
            (ValueType::Map(key, _) | ValueType::UnorderedMap(key, _), "keys") => {
                Some(ValueType::List(key))
            }
            (ValueType::Map(_, value) | ValueType::UnorderedMap(_, value), "values") => {
                Some(ValueType::List(value))
            }
            (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "entries") => Some(
                ValueType::List(ElementType::new(ValueType::Entry(key, value))),
            ),
            _ => None,
        });
    }
    let identity = resolved_compiler_object_identity(unit, callee);
    let source_name = node_text(&unit.source, callee);
    let name = identity
        .and_then(|identity| identity.strip_prefix("/core/collections::"))
        .unwrap_or(source_name);
    let shadowed = bindings.iter().rev().any(|binding| {
        binding.name == source_name && binding.is_visible_at(unit.source.id(), callee.span.start)
    });
    if shadowed {
        return Ok(None);
    }
    if let Some(expected) = bindings
        .iter()
        .filter(|binding| {
            binding.span.start <= node.span.start && node.span.end <= binding.span.end
        })
        .min_by_key(|binding| binding.span.end - binding.span.start)
        .map(|binding| &binding.value_type)
        && collection_constructor_matches(unit, node, expected, bindings)
    {
        return Ok(Some(expected.clone()));
    }
    let result = match name {
        "list" => ValueType::List(homogeneous_element_type(unit, arguments, bindings, name)?),
        "tuple" => ValueType::Tuple(
            homogeneous_element_type(unit, arguments, bindings, name)?,
            Some(arguments.children.len()),
        ),
        "set" => {
            let Some(item) = homogeneous_element_type(unit, arguments, bindings, name)?.scalar()
            else {
                return Err(failure(
                    &unit.source,
                    "T0044",
                    "set keys must be immutable scalar values",
                    arguments.span,
                ));
            };
            ValueType::Set(ElementType::new(ValueType::Scalar(item)))
        }
        "unordered-set" => {
            let Some(item) = homogeneous_element_type(unit, arguments, bindings, name)?.scalar()
            else {
                return Err(failure(
                    &unit.source,
                    "T0044",
                    "unordered-set keys must be immutable scalar values",
                    arguments.span,
                ));
            };
            ValueType::UnorderedSet(ElementType::new(ValueType::Scalar(item)))
        }
        "entry" => {
            let [key, value] = arguments.children.as_slice() else {
                return Err(failure(
                    &unit.source,
                    "T0045",
                    "`entry` requires exactly a key and value",
                    arguments.span,
                ));
            };
            let Some(key) =
                element_type(unit, key.children.last().unwrap_or(key), bindings)?.scalar()
            else {
                return Err(failure(
                    &unit.source,
                    "T0044",
                    "entry keys must be immutable scalar values",
                    key.span,
                ));
            };
            let value = element_type(unit, value.children.last().unwrap_or(value), bindings)?;
            ValueType::Entry(ElementType::new(ValueType::Scalar(key)), value)
        }
        "map" | "unordered-map" => {
            let mut key_type = None;
            let mut value_type = None;
            for argument in &arguments.children {
                let value_node = argument.children.last().unwrap_or(argument);
                let inferred = element_type(unit, value_node, bindings)?;
                let (key, value) = match inferred.value_type() {
                    ValueType::Entry(key, value) if argument.children.len() < 2 => (key, value),
                    _ => (
                        ElementType::new(ValueType::Scalar(ScalarType::String)),
                        inferred,
                    ),
                };
                if key_type.as_ref().is_some_and(|existing| existing != &key) {
                    return Err(failure(
                        &unit.source,
                        "T0042",
                        "map keys must have one statically known type",
                        value_node.span,
                    ));
                }
                if value_type
                    .as_ref()
                    .is_some_and(|existing| existing != &value)
                {
                    return Err(failure(
                        &unit.source,
                        "T0042",
                        "map values must have one statically known type",
                        value_node.span,
                    ));
                }
                key_type = Some(key);
                value_type = Some(value);
            }
            let key = key_type.ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0043",
                    "an empty map requires explicit key and value types",
                    arguments.span,
                )
            })?;
            let value = value_type.expect("a map key and value are inferred together");
            if name == "map" {
                ValueType::Map(key, value)
            } else {
                ValueType::UnorderedMap(key, value)
            }
        }
        "range" => ValueType::Range,
        _ => return Ok(None),
    };
    Ok(Some(result))
}

fn resolved_compiler_object_identity<'a>(
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
) -> Option<&'a str> {
    let name = (node.kind == SyntaxKind::Name).then(|| node_text(&unit.source, node))?;
    lexical_scope_chain(unit, node.span.start).find_map(|scope| {
        scope.symbols.get(name)?.iter().rev().find_map(|symbol| {
            symbol
                .declaration_span
                .is_none_or(|span| span.end <= node.span.start)
                .then_some(symbol.identity.as_str())
        })
    })
}

fn infer_iterator_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(None);
    };
    let shadowed = bindings.iter().rev().any(|binding| {
        binding.name == node_text(&unit.source, callee)
            && binding.is_visible_at(unit.source.id(), callee.span.start)
    });
    if resolved_compiler_object_identity(unit, callee) != Some("/core/collections::iterator")
        && (node_text(&unit.source, callee) != "iterator" || shadowed)
    {
        return Ok(None);
    }
    let mut item_type = None;
    for argument in &arguments.children {
        let value = argument.children.last().unwrap_or(argument);
        let inferred = infer_value_type(unit, value, bindings)?.ok_or_else(|| {
            failure(
                &unit.source,
                "T0041",
                "iterator items require a statically known value type",
                value.span,
            )
        })?;
        let element = match inferred {
            ValueType::Scalar(ty) => ElementType::new(ValueType::Scalar(ty)),
            ValueType::TextRange => ElementType::new(ValueType::TextRange),
            other => {
                return Err(failure(
                    &unit.source,
                    "T0041",
                    format!("`iterator` cannot contain `{other}` values"),
                    value.span,
                ));
            }
        };
        if item_type.is_some_and(|existing| existing != element) {
            return Err(failure(
                &unit.source,
                "T0041",
                "iterator items must have one statically known type",
                value.span,
            ));
        }
        item_type = Some(element);
    }
    item_type.map(ValueType::Iterator).map(Some).ok_or_else(|| {
        failure(
            &unit.source,
            "T0041",
            "an empty iterator requires an explicit item type",
            node.span,
        )
    })
}

fn text_range_member_type(member_name: &str) -> Option<ValueType> {
    match member_name {
        "text" => Some(ValueType::Scalar(ScalarType::String)),
        "bytes" => Some(ValueType::TextRangeView(TextUnit::Bytes)),
        "scalars" => Some(ValueType::TextRangeView(TextUnit::Scalars)),
        "graphemes" => Some(ValueType::TextRangeView(TextUnit::Graphemes)),
        _ => None,
    }
}

fn object_contract<'a>(
    unit: &'a SemanticUnit,
    identity: &ObjectIdentity,
) -> Option<&'a ObjectContract> {
    unit.objects
        .iter()
        .find(|object| object.identity == *identity)
}

fn object_method_contract<'a>(
    unit: &'a SemanticUnit,
    object_identity: &ObjectIdentity,
    member: &str,
    is_static: bool,
) -> Option<&'a FunctionContract> {
    let object = object_contract(unit, object_identity)?;
    unit.functions
        .iter()
        .find(|function| {
            function.owner.as_deref() == Some(object.identity.name.as_str())
                && function.name == member
                && function.is_static == is_static
        })
        .or_else(|| {
            object
                .base
                .as_ref()
                .and_then(|base| object_method_contract(unit, base, member, is_static))
        })
}

fn object_member_type(
    unit: &SemanticUnit,
    object_identity: &ObjectIdentity,
    member: &str,
    is_static: bool,
) -> Option<ValueType> {
    let object = object_contract(unit, object_identity)?;
    if let Some(field) = object
        .fields
        .iter()
        .find(|field| field.name == member && field.is_static == is_static)
    {
        return Some(field.value_type.clone());
    }
    if let Some(method) = object_method_contract(unit, object_identity, member, is_static) {
        let parameters = method
            .parameters
            .iter()
            .map(|parameter| parameter.value_type.clone().map(ElementType::new))
            .collect::<Option<Vec<_>>>()?;
        let result = ElementType::new(
            method
                .return_type
                .clone()
                .unwrap_or(ValueType::Scalar(ScalarType::None)),
        );
        return Some(if method.is_async {
            ValueType::AsyncFunction(parameters, result)
        } else {
            ValueType::Function(parameters, result)
        });
    }
    for used_trait in &object.traits {
        if let Some(trait_object) = unit.objects.iter().find(|candidate| {
            candidate.identity == *used_trait && candidate.kind == ObjectKind::Trait
        }) && let Some(found) =
            object_member_type(unit, &trait_object.identity, member, is_static)
        {
            return Some(found);
        }
    }
    object.base.as_ref().and_then(|base| {
        unit.objects
            .iter()
            .find(|candidate| candidate.identity == *base)
            .and_then(|base| object_member_type(unit, &base.identity, member, is_static))
    })
}

fn infer_receiver_value_type(
    unit: &SemanticUnit,
    receiver: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    Ok(
        infer_value_type(unit, receiver, bindings)?.map(|value_type| match value_type {
            ValueType::Reference(item) | ValueType::SharedReference(item) => item.value_type(),
            value_type => value_type,
        }),
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "member inference keeps receiver precedence and diagnostics in one ordered dispatch"
)]
fn infer_member_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [receiver, member] = node.children.as_slice() else {
        return Ok(None);
    };
    let member_name = node_text(&unit.source, member);
    let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
    if let Some(ValueType::Descriptor(_)) = &receiver_type {
        return match member_name {
            "name" | "kind" | "identity" => Ok(Some(ValueType::Scalar(ScalarType::String))),
            _ => Err(failure(
                &unit.source,
                "T0071",
                format!("descriptor has no retained member `{member_name}`"),
                member.span,
            )),
        };
    }
    if matches!(
        receiver_type,
        Some(ValueType::Function(_, _) | ValueType::AsyncFunction(_, _))
    ) && matches!(
        member_name,
        "contracts" | "throwable-contract" | "escaping-throwables"
    ) {
        return Ok(Some(ValueType::Scalar(ScalarType::String)));
    }
    if let Some(result) = &receiver_type
        && matches!(
            result,
            ValueType::PlatformOpenResult
                | ValueType::PlatformReadResult
                | ValueType::PlatformWriteResult
                | ValueType::PlatformUnitResult
        )
    {
        let member_type = match (result, member_name) {
            (ValueType::PlatformOpenResult, "handle") => Some(ValueType::PlatformStreamHandle),
            (ValueType::PlatformReadResult, "data") => Some(ValueType::Scalar(ScalarType::Bytes)),
            (ValueType::PlatformReadResult | ValueType::PlatformWriteResult, "completed") => {
                Some(ValueType::Scalar(ScalarType::Int))
            }
            (ValueType::PlatformReadResult, "end")
            | (
                ValueType::PlatformOpenResult
                | ValueType::PlatformReadResult
                | ValueType::PlatformWriteResult
                | ValueType::PlatformUnitResult,
                "failed",
            ) => Some(ValueType::Scalar(ScalarType::Bool)),
            (
                ValueType::PlatformOpenResult
                | ValueType::PlatformReadResult
                | ValueType::PlatformWriteResult
                | ValueType::PlatformUnitResult,
                "message",
            ) => Some(ValueType::Scalar(ScalarType::String)),
            _ => None,
        };
        return member_type.map(Some).ok_or_else(|| {
            failure(
                &unit.source,
                "T0097",
                format!("`{result}` has no member `{member_name}`"),
                member.span,
            )
        });
    }
    if let Some(ValueType::TaskOutcome(result)) = &receiver_type {
        return match member_name {
            "completed" | "cancelled" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "value" => Ok(Some(ValueType::Optional(Box::new(result.value_type())))),
            "error" => Ok(Some(ValueType::Optional(Box::new(ValueType::Object(
                ObjectIdentity::new("/core/errors", "TerraneError"),
            ))))),
            _ => Err(failure(
                &unit.source,
                "T0074",
                format!("task outcome has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::Object(object_name)) = &receiver_type {
        return object_member_type(unit, object_name, member_name, false)
            .map(Some)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0055",
                    format!(
                        "`{}` has no instance member `{member_name}`",
                        unit.objects
                            .iter()
                            .find(|object| object.identity == *object_name)
                            .map_or_else(
                                || diagnostic_object_identity(&unit.objects, object_name),
                                |object| object.name.clone()
                            )
                    ),
                    member.span,
                )
            });
    }
    let collection_method = matches!(
        (&receiver_type, member_name),
        (
            Some(ValueType::List(_) | ValueType::Tuple(_, _)),
            "append" | "set" | "get"
        ) | (
            Some(ValueType::Map(_, _) | ValueType::UnorderedMap(_, _)),
            "set" | "get" | "keys" | "values" | "entries"
        ) | (
            Some(ValueType::Set(_) | ValueType::UnorderedSet(_)),
            "add" | "contains" | "remove"
        )
    );
    let string_method = matches!(&receiver_type, Some(ValueType::Scalar(ScalarType::String)))
        && (StringFamily::from_source_name(member_name).is_some()
            || matches!(member_name, "concat" | "join"));
    let bytes_method = matches!(&receiver_type, Some(ValueType::Scalar(ScalarType::Bytes)))
        && matches!(member_name, "decode" | "concat");
    if collection_method || string_method || bytes_method {
        let family = if string_method {
            "string methods"
        } else if bytes_method {
            "bytes methods"
        } else {
            "collection methods"
        };
        return Err(failure(
            &unit.source,
            "T0018",
            format!(
                "{family} are not storable values before bound methods exist; \
                 method `.{member_name}` must be invoked with `;`"
            ),
            node.span,
        ));
    }
    if matches!(
        receiver_type,
        Some(
            ValueType::List(_)
                | ValueType::Map(_, _)
                | ValueType::Set(_)
                | ValueType::Tuple(_, _)
                | ValueType::UnorderedMap(_, _)
                | ValueType::UnorderedSet(_)
        )
    ) && member_name == "length"
    {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    if let Some(ValueType::Entry(key, value)) = receiver_type {
        return Ok(match member_name {
            "key" => Some(key.value_type()),
            "value" => Some(value.value_type()),
            _ => None,
        });
    }
    if receiver_type == Some(ValueType::Scalar(ScalarType::String)) {
        let view = match member_name {
            "bytes" => Some(TextUnit::Bytes),
            "scalars" => Some(TextUnit::Scalars),
            "graphemes" => Some(TextUnit::Graphemes),
            _ => None,
        };
        if let Some(view) = view {
            return Ok(Some(ValueType::StringView(view)));
        }
    }
    if receiver_type == Some(ValueType::TextRange) {
        return Ok(text_range_member_type(member_name));
    }
    if matches!(receiver_type, Some(ValueType::TextRangeView(_)))
        && matches!(member_name, "start" | "end")
    {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    if matches!(
        receiver_type,
        Some(
            ValueType::StringView(_)
                | ValueType::StringList
                | ValueType::TextRangeList
                | ValueType::Scalar(ScalarType::Bytes)
        )
    ) && member_name == "length"
    {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    match (receiver_type.clone(), member_name) {
        (Some(ValueType::OverflowResult(ty)), "value")
        | (Some(ValueType::DivRemResult(ty)), "quotient" | "remainder") => {
            return Ok(Some(ValueType::Scalar(ty)));
        }
        (Some(ValueType::OverflowResult(_)), "overflowed") => {
            return Ok(Some(ValueType::Scalar(ScalarType::Bool)));
        }
        (Some(ValueType::OverflowResult(_) | ValueType::DivRemResult(_)), _) => {
            return Err(failure(
                &unit.source,
                "T0031",
                format!("result object has no member `.{member_name}`"),
                member.span,
            ));
        }
        _ => {}
    }
    if let Some(contract) = float_member_contract(member_name) {
        if let Some(ValueType::Scalar(receiver @ (ScalarType::Float32 | ScalarType::Float64))) =
            receiver_type.clone()
        {
            return Ok(Some(contract.member_type(receiver)));
        }
        return Err(failure(
            &unit.source,
            "T0013",
            format!("`.{member_name}` requires a floating receiver"),
            receiver.span,
        ));
    }
    if member_name == "type" {
        return Ok(None);
    }
    if member_name != "length" {
        return match receiver_type {
            Some(receiver_type) => Err(failure(
                &unit.source,
                "T0031",
                format!("`{receiver_type}` has no member `.{member_name}`"),
                member.span,
            )),
            None => Ok(None),
        };
    }
    if matches!(
        receiver_type,
        Some(ValueType::Scalar(ScalarType::String | ScalarType::Bytes))
    ) {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    let message = receiver_type.map_or_else(
        || {
            "`.length` requires a receiver with a statically known sequence type; \
             add a collection type annotation"
                .to_owned()
        },
        |value_type| {
            format!("`.length` requires `string`, `bytes`, or a collection, found `{value_type}`")
        },
    );
    Err(failure(&unit.source, "T0013", message, receiver.span))
}

fn infer_float_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(callee) = node.children.first() else {
        return Ok(None);
    };
    let Some([receiver, member]) = (callee.kind == SyntaxKind::MemberExpression)
        .then_some(callee.children.as_slice())
        .and_then(|children| <&[SyntaxNode; 2]>::try_from(children).ok())
    else {
        return Ok(None);
    };
    let member_name = node_text(&unit.source, member);
    let Some(contract) = float_member_contract(member_name) else {
        return Ok(None);
    };
    let Some(expected) = contract.arity else {
        return Ok(None);
    };
    let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
    let Some(ValueType::Scalar(receiver @ (ScalarType::Float32 | ScalarType::Float64))) =
        receiver_type
    else {
        return Err(failure(
            &unit.source,
            "T0013",
            format!("`.{member_name}` requires a floating receiver"),
            receiver.span,
        ));
    };
    let arguments = node.children.get(1);
    let arguments = arguments.map_or(&[][..], |arguments| arguments.children.as_slice());
    if arguments.len() != expected {
        return Err(failure(
            &unit.source,
            "T0023",
            format!(
                "`.{member_name}` requires exactly {expected} argument{}",
                if expected == 1 { "" } else { "s" }
            ),
            node.span,
        ));
    }
    for argument in arguments {
        let value = argument.children.last().unwrap_or(argument);
        if let Some(actual) = infer_value_type(unit, value, bindings)? {
            validate_value_destination(
                &unit.source,
                &unit.objects,
                "floating operation argument",
                ValueType::Scalar(receiver),
                actual,
                value,
                "T0013",
            )?;
        }
    }
    Ok(Some(contract.result_type(receiver)))
}

pub(crate) fn string_call_selection(
    source: &SourceFile,
    node: &SyntaxNode,
) -> Option<StringCallSelection> {
    let callee = node.children.first()?;
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    if callee.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let (receiver_span, family, child) = if receiver.kind == SyntaxKind::MemberExpression
        && let [nested_receiver, nested_family] = receiver.children.as_slice()
        && let Some(candidate) = StringFamily::from_source_name(node_text(source, nested_family))
        && candidate.has_children()
    {
        (
            nested_receiver.span,
            candidate,
            node_text(source, member).to_owned(),
        )
    } else {
        (
            receiver.span,
            StringFamily::from_source_name(node_text(source, member))?,
            "default".to_owned(),
        )
    };
    Some(StringCallSelection {
        receiver: receiver_span,
        family,
        child,
    })
}

#[allow(clippy::too_many_lines)]
fn infer_string_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(selection) = string_call_selection(&unit.source, node) else {
        return Ok(None);
    };
    let subject = find_node_by_span(&unit.tree.root, selection.receiver)
        .expect("selected string receiver belongs to this syntax tree");
    let family = selection.family.source_name();
    let child = selection.child.as_str();
    let subject_type = transparent_value_type(infer_value_type(unit, subject, bindings)?);
    if matches!(subject_type, Some(ValueType::Object(_))) {
        return Ok(None);
    }
    let receiver_valid = match family {
        "decode" => subject_type == Some(ValueType::Scalar(ScalarType::Bytes)),
        _ => subject_type == Some(ValueType::Scalar(ScalarType::String)),
    };
    if !receiver_valid {
        return Err(failure(
            &unit.source,
            "T0032",
            format!("`.{family}` is not available on this receiver"),
            subject.span,
        ));
    }
    let arguments = node
        .children
        .get(1)
        .map_or(&[][..], |arguments| arguments.children.as_slice());
    let (minimum, maximum) = match (family, child) {
        ("trim", "default") | ("upper" | "lower" | "normalise" | "case-fold", _) => (0, 0),
        ("trim", "start" | "end") => (0, 1),
        ("replace", _) => (2, 2),
        _ => (1, 1),
    };
    if arguments.len() < minimum || arguments.len() > maximum {
        return Err(failure(
            &unit.source,
            "T0023",
            format!("`.{family}` received the wrong number of arguments"),
            node.span,
        ));
    }
    for argument in arguments {
        let argument = argument.children.last().unwrap_or(argument);
        let expected = if matches!(family, "encode" | "decode") {
            ValueType::Encoding
        } else {
            ValueType::Scalar(ScalarType::String)
        };
        if infer_value_type(unit, argument, bindings)? != Some(expected) {
            return Err(failure(
                &unit.source,
                "T0033",
                format!("`.{family}` received an incompatible argument"),
                argument.span,
            ));
        }
    }
    let result = match (family, child) {
        ("contains", "default" | "start" | "end") => ValueType::Scalar(ScalarType::Bool),
        ("find", "default") => ValueType::Optional(Box::new(ValueType::TextRange)),
        ("find", "all") => ValueType::TextRangeList,
        ("find", "count") => ValueType::Scalar(ScalarType::Int),
        ("split", "default") => ValueType::StringList,
        ("encode", "default") => ValueType::Scalar(ScalarType::Bytes),
        ("decode" | "case-fold" | "replace", "default")
        | ("trim", "default" | "start" | "end")
        | ("upper", "default" | "first" | "words")
        | ("lower", "default" | "first")
        | ("normalise", "nfc" | "nfd" | "nfkc" | "nfkd") => ValueType::Scalar(ScalarType::String),
        _ => {
            return Err(failure(
                &unit.source,
                "T0034",
                format!("`.{family}.{child}` is not available"),
                node.span,
            ));
        }
    };
    Ok(Some(result))
}
fn infer_unary_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<ValueType, SemanticFailure> {
    let Some(operand_node) = node.children.last() else {
        return Err(operator_failure(
            unit,
            node,
            "unary operator requires an operand",
        ));
    };
    let operator = unary_operator_text(unit, node).unwrap_or_default();
    if operator == "await" {
        return match infer_value_type(unit, operand_node, bindings)? {
            Some(ValueType::Task(result)) => Ok(result.value_type()),
            _ => Err(operator_failure(
                unit,
                node,
                "`await` requires a task value",
            )),
        };
    }
    if matches!(operator.as_str(), "ref" | "shared ref" | "move") {
        let Some(operand) = infer_value_type(unit, operand_node, bindings)? else {
            return Err(operator_failure(
                unit,
                node,
                format!("operator `{operator}` requires a value operand"),
            ));
        };
        return Ok(match operator.as_str() {
            "ref" => match operand {
                ValueType::Reference(item) | ValueType::SharedReference(item) => {
                    ValueType::Reference(item)
                }
                value_type => ValueType::Reference(ElementType::new(value_type)),
            },
            "shared ref" => match operand {
                ValueType::Reference(item) | ValueType::SharedReference(item) => {
                    ValueType::SharedReference(item)
                }
                value_type => ValueType::SharedReference(ElementType::new(value_type)),
            },
            "move" => operand,
            _ => unreachable!(),
        });
    }
    let Some(ValueType::Scalar(operand)) = infer_receiver_value_type(unit, operand_node, bindings)?
    else {
        return Err(operator_failure(
            unit,
            node,
            "unary operator requires a scalar operand",
        ));
    };

    let valid = match operator.as_str() {
        "-" => operand.is_integer() || matches!(operand, ScalarType::Float32 | ScalarType::Float64),
        "~" => operand.is_integer(),
        "not" => operand == ScalarType::Bool,
        _ => false,
    };
    if !valid {
        return Err(operator_failure(
            unit,
            node,
            format!("operator `{operator}` is not defined for `{operand}`"),
        ));
    }
    Ok(ValueType::Scalar(if operator == "not" {
        ScalarType::Bool
    } else {
        operand
    }))
}
#[expect(
    clippy::too_many_lines,
    reason = "family receiver, callback, argument, and result contracts remain auditable together"
)]
fn infer_parse_or_radix_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(callee) = node.children.first() else {
        return Ok(None);
    };
    let Some(method) = bound_method(&unit.source, callee) else {
        return Ok(None);
    };
    if matches!(
        method.family,
        MemberFamily::Coerce | MemberFamily::Arithmetic(_)
    ) {
        return Ok(None);
    }
    let arguments = node.children.get(1);
    let arguments = arguments.map_or(&[][..], |arguments| arguments.children.as_slice());
    if arguments.len() != 1 {
        return Err(failure(
            &unit.source,
            "T0023",
            format!(
                "`.{}` requires exactly one argument",
                match method.family {
                    MemberFamily::Parse => "parse",
                    MemberFamily::Radix => "radix",
                    MemberFamily::Coerce | MemberFamily::Arithmetic(_) => unreachable!(),
                }
            ),
            node.span,
        ));
    }
    let receiver = find_node_by_span(&unit.tree.root, method.receiver)
        .expect("bound method receiver belongs to this syntax tree");
    let argument = arguments[0].children.last().unwrap_or(&arguments[0]);
    if method.family == MemberFamily::Radix {
        let argument_type = infer_value_type(unit, argument, bindings)?;
        if !matches!(argument_type, Some(ValueType::Scalar(scalar)) if scalar.is_integer()) {
            return Err(failure(
                &unit.source,
                "T0024",
                "`.radix` requires an integer radix argument",
                argument.span,
            ));
        }
        let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
        return match receiver_type {
            Some(ValueType::Scalar(ScalarType::String)) => {
                Ok(Some(ValueType::Scalar(ScalarType::Int)))
            }
            Some(ValueType::Scalar(scalar)) if scalar.is_integer() => {
                Ok(Some(ValueType::Scalar(ScalarType::String)))
            }
            _ => Err(failure(
                &unit.source,
                "T0024",
                "`.radix` requires a string or numeric receiver",
                receiver.span,
            )),
        };
    }
    let receiver_type = infer_value_type(unit, receiver, bindings)?;
    if receiver_type != Some(ValueType::Scalar(ScalarType::String)) {
        return Err(failure(
            &unit.source,
            "T0024",
            "`.parse` requires a string receiver",
            receiver.span,
        ));
    }
    let callback = arguments[0].children.last().unwrap_or(&arguments[0]);
    if callback.kind != SyntaxKind::Name {
        return Err(failure(
            &unit.source,
            "T0025",
            "`.parse` requires a statically resolvable function name",
            callback.span,
        ));
    }
    let callback_name = node_text(&unit.source, callback);
    let Some(contract) = resolved_function_contract(unit, callback_name, callback.span.start)
    else {
        return Err(failure(
            &unit.source,
            "T0025",
            format!("`{callback_name}` does not resolve to a parse callback"),
            callback.span,
        ));
    };
    if contract.parameters.len() != 1
        || contract.parameters[0].value_type != Some(ValueType::Scalar(ScalarType::String))
        || !matches!(contract.return_type, Some(ValueType::Scalar(_)))
    {
        return Err(failure(
            &unit.source,
            "T0026",
            format!(
                "parse callback `{callback_name}` must take one `string` value and declare a scalar return"
            ),
            callback.span,
        ));
    }
    let Some(ValueType::Scalar(result)) = contract.return_type.clone() else {
        unreachable!("checked above")
    };
    Ok(Some(if method.child == "checked" {
        ValueType::Optional(Box::new(ValueType::Scalar(result)))
    } else {
        ValueType::Scalar(result)
    }))
}

#[allow(clippy::too_many_lines)]
fn infer_arithmetic_family_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(callee) = node.children.first() else {
        return Ok(None);
    };
    let Some(method) = bound_method(&unit.source, callee) else {
        return Ok(None);
    };
    let MemberFamily::Arithmetic(family) = method.family else {
        return Ok(None);
    };
    let receiver = find_node_by_span(&unit.tree.root, method.receiver)
        .expect("bound arithmetic receiver belongs to this syntax tree");
    let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
    if matches!(receiver_type, Some(ValueType::Object(_))) {
        return Ok(None);
    }
    let Some(ValueType::Scalar(receiver_type)) = receiver_type else {
        return Err(failure(
            &unit.source,
            "T0036",
            format!("`.{}` requires an integer receiver", family.source_name()),
            receiver.span,
        ));
    };
    if !receiver_type.is_integer() {
        return Err(failure(
            &unit.source,
            "T0036",
            format!("`.{}` requires an integer receiver", family.source_name()),
            receiver.span,
        ));
    }
    if family == ArithmeticFamily::Negate
        && !matches!(
            receiver_type,
            ScalarType::Int
                | ScalarType::Int8
                | ScalarType::Int16
                | ScalarType::Int32
                | ScalarType::Int64
                | ScalarType::Int128
        )
    {
        return Err(failure(
            &unit.source,
            "T0037",
            "`.negate` is not available on unsigned integers",
            receiver.span,
        ));
    }
    let arguments = node.children.get(1);
    let arguments = arguments.map_or(&[][..], |arguments| arguments.children.as_slice());
    let expected = usize::from(family != ArithmeticFamily::Negate);
    if arguments.len() != expected {
        return Err(failure(
            &unit.source,
            "T0023",
            format!(
                "`.{}` requires exactly {expected} argument{}",
                family.source_name(),
                if expected == 1 { "" } else { "s" }
            ),
            node.span,
        ));
    }
    if let Some(argument) = arguments.first() {
        let argument = argument.children.last().unwrap_or(argument);
        let argument_type = infer_value_type(unit, argument, bindings)?;
        let valid = if matches!(
            family,
            ArithmeticFamily::ShiftLeft | ArithmeticFamily::ShiftRight
        ) {
            matches!(argument_type, Some(ValueType::Scalar(ty)) if ty.is_integer())
        } else {
            argument_type == Some(ValueType::Scalar(receiver_type))
                || contextual_constant(&unit.source, argument, receiver_type).is_some()
        };
        if !valid {
            return Err(failure(
                &unit.source,
                "T0028",
                format!(
                    "`.{}` argument is incompatible with `{receiver_type}`",
                    family.source_name()
                ),
                argument.span,
            ));
        }
    }
    let fixed = receiver_type != ScalarType::Int;
    let child_allowed = match method.child {
        "default" => true,
        "checked" => {
            fixed
                || matches!(
                    family,
                    ArithmeticFamily::Divide
                        | ArithmeticFamily::Remainder
                        | ArithmeticFamily::DivRem
                )
        }
        "wrap" => fixed && family != ArithmeticFamily::DivRem,
        "saturate" | "overflowing" => {
            fixed
                && !matches!(
                    family,
                    ArithmeticFamily::DivRem
                        | ArithmeticFamily::ShiftLeft
                        | ArithmeticFamily::ShiftRight
                )
        }
        _ => false,
    };
    if !child_allowed {
        return Err(failure(
            &unit.source,
            "T0029",
            format!(
                "`.{}.{}` is not available on `{receiver_type}`",
                family.source_name(),
                method.child
            ),
            callee.span,
        ));
    }
    let result = if method.child == "overflowing" {
        ValueType::OverflowResult(receiver_type)
    } else if family == ArithmeticFamily::DivRem {
        if method.child == "checked" {
            return Err(failure(
                &unit.source,
                "T0030",
                "`div-rem.checked` optional result values are not yet representable",
                callee.span,
            ));
        }
        ValueType::DivRemResult(receiver_type)
    } else if method.child == "checked" {
        ValueType::Optional(Box::new(ValueType::Scalar(receiver_type)))
    } else {
        ValueType::Scalar(receiver_type)
    };
    Ok(Some(result))
}

fn infer_integer_coercion_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(callee) = node.children.first() else {
        return Ok(None);
    };
    let Some((source_node, policy)) = integer_coercion_call(&unit.source, callee) else {
        if let Some(member) = obsolete_integer_coercion_member(unit, callee) {
            return Err(failure(
                &unit.source,
                "T0017",
                format!(
                    "`{member}` is not valid syntax; use `.coerce.{}`",
                    match member {
                        "checked-coerce" => "checked",
                        "wrapping-coerce" => "wrap",
                        "saturating-coerce" => "saturate",
                        _ => unreachable!("obsolete coercion members are matched above"),
                    }
                ),
                callee.span,
            ));
        }
        if let Some(chain) = invalid_coercion_policy(unit, callee) {
            return Err(failure(
                &unit.source,
                "T0010",
                format!("`{chain}` is not an available coercion policy"),
                callee.span,
            ));
        }
        return Ok(None);
    };
    let Some(ValueType::Scalar(source_type)) =
        infer_receiver_value_type(unit, source_node, bindings)?
    else {
        return Err(failure(
            &unit.source,
            "T0009",
            "`.coerce` requires an integer source",
            source_node.span,
        ));
    };
    if !source_type.is_integer() {
        return Err(failure(
            &unit.source,
            "T0009",
            format!(
                "`{}` requires an integer source, found `{source_type}`",
                policy.invocation_name()
            ),
            source_node.span,
        ));
    }
    let destination_node = node
        .children
        .get(1)
        .and_then(|arguments| arguments.children.first())
        .and_then(|argument| argument.children.last())
        .ok_or_else(|| {
            failure(
                &unit.source,
                "T0008",
                format!(
                    "`{}` from `{source_type}` requires one integer destination",
                    policy.invocation_name()
                ),
                node.span,
            )
        })?;
    let destination_name = node_text(&unit.source, destination_node);
    let destination = unit
        .descriptor_alias_at(destination_name, destination_node.span.start)
        .ok_or_else(|| {
            failure(
                &unit.source,
                "T0008",
                format!(
                    "`{destination_name}` is not a supported destination for `{}` from `{source_type}`",
                    policy.invocation_name()
                ),
                destination_node.span,
            )
        })?;
    if !destination.is_integer() {
        return Err(failure(
            &unit.source,
            "T0008",
            format!(
                "`{destination}` is not a supported destination for `{}` from `{source_type}`",
                policy.invocation_name()
            ),
            destination_node.span,
        ));
    }
    let result = integer_coercion_result_type(source_type, destination, policy)
        .map_err(|message| failure(&unit.source, "T0010", message, destination_node.span))?;
    Ok(Some(result))
}

fn integer_coercion_result_type(
    source: ScalarType,
    destination: ScalarType,
    policy: CoercionPolicy,
) -> Result<ValueType, String> {
    match (source, destination, policy) {
        (
            _,
            ScalarType::Int,
            CoercionPolicy::Checked | CoercionPolicy::Wrap | CoercionPolicy::Saturate,
        ) => Err(format!(
            "`.coerce.{}` from `{source}` requires a fixed-width integer destination",
            policy.source_name()
        )),
        (_, _, CoercionPolicy::Checked) => Ok(ValueType::Optional(Box::new(ValueType::Scalar(
            destination,
        )))),
        (_, _, CoercionPolicy::Default | CoercionPolicy::Wrap | CoercionPolicy::Saturate) => {
            Ok(ValueType::Scalar(destination))
        }
    }
}

pub(crate) fn bound_method(source: &SourceFile, callee: &SyntaxNode) -> Option<BoundMethod> {
    if callee.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    let member_name = node_text(source, member);
    let direct = match member_name {
        "coerce" => Some((MemberFamily::Coerce, "default")),
        "parse" => Some((MemberFamily::Parse, "default")),
        "radix" => Some((MemberFamily::Radix, "default")),
        name => ArithmeticFamily::from_source_name(name)
            .map(|family| (MemberFamily::Arithmetic(family), "default")),
    };
    if let Some((family, child)) = direct {
        return Some(BoundMethod {
            receiver: receiver.span,
            family,
            child,
        });
    }
    if receiver.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let [source_node, family_node] = receiver.children.as_slice() else {
        return None;
    };
    let selection = match (node_text(source, family_node), member_name) {
        ("coerce", "checked") => (MemberFamily::Coerce, "checked"),
        ("coerce", "wrap") => (MemberFamily::Coerce, "wrap"),
        ("coerce", "saturate") => (MemberFamily::Coerce, "saturate"),
        ("parse", "checked") => (MemberFamily::Parse, "checked"),
        (family, child @ ("checked" | "wrap" | "saturate" | "overflowing")) => {
            let child = match child {
                "checked" => "checked",
                "wrap" => "wrap",
                "saturate" => "saturate",
                "overflowing" => "overflowing",
                _ => unreachable!(),
            };
            (
                MemberFamily::Arithmetic(ArithmeticFamily::from_source_name(family)?),
                child,
            )
        }
        _ => return None,
    };
    Some(BoundMethod {
        receiver: source_node.span,
        family: selection.0,
        child: selection.1,
    })
}

/// Resolves the canonical `.coerce` callable family and its selected policy child.
///
/// The returned policy is shared semantic metadata for analysis and lowering; the
/// Rust helper names used after family erasure are not independent source members.
pub(crate) fn integer_coercion_call<'a>(
    source: &SourceFile,
    callee: &'a SyntaxNode,
) -> Option<(&'a SyntaxNode, CoercionPolicy)> {
    let method = bound_method(source, callee)?;
    if method.family != MemberFamily::Coerce {
        return None;
    }
    let policy = match method.child {
        "default" => CoercionPolicy::Default,
        child => CoercionPolicy::from_member(child)?,
    };
    let receiver = callee.children.first()?;
    let source_node = if method.child == "default" {
        receiver
    } else {
        receiver.children.first()?
    };
    Some((source_node, policy))
}

fn invalid_coercion_policy(unit: &SemanticUnit, callee: &SyntaxNode) -> Option<String> {
    (coercion_family_receiver(unit, callee)
        && integer_coercion_call(&unit.source, callee).is_none())
    .then(|| {
        let callee_text = node_text(&unit.source, callee);
        let family_start = callee_text.find(".coerce").unwrap_or(0);
        callee_text[family_start..].to_owned()
    })
}

fn coercion_family_receiver(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
    let [receiver, member] = node.children.as_slice() else {
        return false;
    };
    node.kind == SyntaxKind::MemberExpression
        && (node_text(&unit.source, member) == "coerce" || coercion_family_receiver(unit, receiver))
}

fn member_family_receiver(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
    let [receiver, member] = node.children.as_slice() else {
        return false;
    };
    if node_text(&unit.source, member) == "remainder"
        && matches!(
            infer_value_type(unit, receiver, &unit.typed_bindings),
            Ok(Some(ValueType::DivRemResult(_)))
        )
    {
        return false;
    }
    node.kind == SyntaxKind::MemberExpression
        && matches!(
            node_text(&unit.source, member),
            "coerce"
                | "parse"
                | "radix"
                | "add"
                | "subtract"
                | "multiply"
                | "divide"
                | "remainder"
                | "div-rem"
                | "negate"
                | "shift-left"
                | "shift-right"
        )
}

fn obsolete_integer_coercion_member<'a>(
    unit: &'a SemanticUnit,
    callee: &'a SyntaxNode,
) -> Option<&'a str> {
    let [_, member] = callee.children.as_slice() else {
        return None;
    };
    (callee.kind == SyntaxKind::MemberExpression)
        .then(|| node_text(&unit.source, member))
        .filter(|member| {
            matches!(
                *member,
                "checked-coerce" | "wrapping-coerce" | "saturating-coerce"
            )
        })
}

#[expect(
    clippy::too_many_lines,
    reason = "binary inference keeps operator precedence, optional equality, and numeric promotion auditable"
)]
fn infer_binary_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<ValueType, SemanticFailure> {
    let [left_node, right_node] = node.children.as_slice() else {
        return Err(operator_failure(
            unit,
            node,
            "binary operator requires two operands",
        ));
    };
    let left = infer_receiver_value_type(unit, left_node, bindings)?;
    let right = infer_receiver_value_type(unit, right_node, bindings)?;
    let operator = unit.source.text()[left_node.span.end..right_node.span.start].trim();
    if operator == "is"
        && (node_text(&unit.source, left_node).trim() == "none"
            || node_text(&unit.source, right_node).trim() == "none")
    {
        return Err(failure(
            &unit.source,
            "T0038",
            "`is none` is invalid; type membership is written `is a none`",
            node.span,
        ));
    }
    if operator == "is" {
        return Ok(ValueType::Scalar(ScalarType::Bool));
    }
    if matches!(operator, "==" | "!=")
        && ((matches!(left, Some(ValueType::Optional(_)))
            && node_text(&unit.source, right_node).trim() == "none")
            || (matches!(right, Some(ValueType::Optional(_)))
                && node_text(&unit.source, left_node).trim() == "none"))
    {
        return Ok(ValueType::Scalar(ScalarType::Bool));
    }
    if matches!(operator, "==" | "!=")
        && let (Some(ValueType::Object(left)), Some(ValueType::Object(right))) = (&left, &right)
        && left == right
        && unit.comparable_foreign_objects.contains(left)
    {
        return Ok(ValueType::Scalar(ScalarType::Bool));
    }
    let comparison = matches!(operator, "==" | "!=" | "<" | "<=" | ">" | ">=");
    let contextual_numeric = matches!(
        operator,
        "+" | "-" | "*" | "/" | "%" | "&" | "^" | "|" | "==" | "!=" | "<" | "<=" | ">" | ">="
    );
    if contextual_numeric
        && let Some(ValueType::Scalar(left_type)) = left
        && is_numeric(left_type)
        && contextual_constant(&unit.source, right_node, left_type)
            .transpose()?
            .is_some()
    {
        return Ok(ValueType::Scalar(if comparison {
            ScalarType::Bool
        } else {
            left_type
        }));
    }
    if contextual_numeric
        && let Some(ValueType::Scalar(right_type)) = right
        && is_numeric(right_type)
        && contextual_constant(&unit.source, left_node, right_type)
            .transpose()?
            .is_some()
    {
        return Ok(ValueType::Scalar(if comparison {
            ScalarType::Bool
        } else {
            right_type
        }));
    }
    let (Some(ValueType::Scalar(left)), Some(ValueType::Scalar(right))) = (left, right) else {
        return Err(operator_failure(
            unit,
            node,
            "operator requires scalar operands",
        ));
    };
    let same = left == right;
    if contextual_numeric && left != right && left.is_integer() && right.is_integer() {
        if contextual_constant(&unit.source, right_node, right).is_some() {
            contextual_constant(&unit.source, right_node, left).expect(
                "integer constant expression remains contextual across integer destinations",
            )?;
        }
        if contextual_constant(&unit.source, left_node, left).is_some() {
            contextual_constant(&unit.source, left_node, right).expect(
                "integer constant expression remains contextual across integer destinations",
            )?;
        }
    }
    if contextual_numeric && left != right && left.is_integer() && right.is_integer() {
        return Ok(ValueType::Scalar(if comparison {
            ScalarType::Bool
        } else {
            promoted_integer_type(left, right)
        }));
    }
    let numeric =
        |ty: ScalarType| ty.is_integer() || matches!(ty, ScalarType::Float32 | ScalarType::Float64);
    let result = match operator {
        "+" | "-" | "*" | "/" | "%" if same && numeric(left) => left,
        "<<" | ">>" if left.is_integer() && right.is_integer() => left,
        "&" | "^" | "|" if same && left.is_integer() => left,
        "and" | "or" if left == ScalarType::Bool && right == ScalarType::Bool => ScalarType::Bool,
        "==" | "!=" if same => ScalarType::Bool,
        "<" | "<=" | ">" | ">=" if same && (numeric(left) || left == ScalarType::String) => {
            ScalarType::Bool
        }
        _ => {
            return Err(operator_failure(
                unit,
                node,
                format!("operator `{operator}` is not defined for `{left}` and `{right}`"),
            ));
        }
    };
    Ok(ValueType::Scalar(result))
}

fn operator_failure(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    message: impl Into<String>,
) -> SemanticFailure {
    failure(&unit.source, "T0011", message, node.span)
}

fn infer_literal_type(unit: &SemanticUnit, node: &SyntaxNode) -> Option<ScalarType> {
    infer_literal_type_from_source(&unit.source, node)
}

fn infer_literal_type_from_source(source: &SourceFile, node: &SyntaxNode) -> Option<ScalarType> {
    if node.kind == SyntaxKind::UnaryExpression {
        return node
            .children
            .last()
            .and_then(|child| infer_literal_type_from_source(source, child));
    }
    if node.kind != SyntaxKind::Literal {
        return None;
    }

    let text = node_text(source, node);
    match text {
        "true" | "false" => Some(ScalarType::Bool),
        value if value.starts_with("b'") => Some(ScalarType::Bytes),
        value if value.starts_with(['\'', '"', '>']) => Some(ScalarType::String),
        value if value.contains('.') => Some(ScalarType::Float64),
        _ => Some(ScalarType::Int),
    }
}

pub(crate) fn contextual_constant(
    source: &SourceFile,
    node: &SyntaxNode,
    destination: ScalarType,
) -> Option<Result<ContextualConstant, SemanticFailure>> {
    if !is_numeric(destination) {
        return None;
    }
    contextual_constant_value(source, node, destination).map(|result| {
        result.and_then(|value| {
            match &value {
                ContextualConstant::Integer(integer) => {
                    check_integer_range(source, destination, integer, node.span)?;
                }
                ContextualConstant::Float32(value) if !value.is_finite() => {
                    return Err(invalid_floating_constant(source, destination, node.span));
                }
                ContextualConstant::Float64(value) if !value.is_finite() => {
                    return Err(invalid_floating_constant(source, destination, node.span));
                }
                ContextualConstant::Float32(_) | ContextualConstant::Float64(_) => {}
            }
            Ok(value)
        })
    })
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "storage selection owns the optional inferred recursive type"
)]
fn small_int_storage(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    inferred: Option<ValueType>,
) -> Option<ScalarType> {
    if let Some(Ok(ContextualConstant::Integer(integer))) =
        contextual_constant(&unit.source, value, ScalarType::Int)
        && integer.to_i64().is_some()
    {
        return Some(ScalarType::Int64);
    }
    matches!(
        inferred,
        Some(ValueType::Scalar(
            ScalarType::Int8
                | ScalarType::Int16
                | ScalarType::Int32
                | ScalarType::Int64
                | ScalarType::Uint8
                | ScalarType::Uint16
                | ScalarType::Uint32
        ))
    )
    .then_some(ScalarType::Int64)
}

fn contextual_constant_value(
    source: &SourceFile,
    node: &SyntaxNode,
    destination: ScalarType,
) -> Option<Result<ContextualConstant, SemanticFailure>> {
    let result = match node.kind {
        SyntaxKind::GroupExpression => {
            return node
                .children
                .first()
                .and_then(|child| contextual_constant_value(source, child, destination));
        }
        SyntaxKind::UnaryExpression => {
            let operand = node.children.last()?;
            let value = contextual_constant_value(source, operand, destination)?;
            value.map(|value| match value {
                ContextualConstant::Integer(value) => ContextualConstant::Integer(-value),
                ContextualConstant::Float32(value) => ContextualConstant::Float32(-value),
                ContextualConstant::Float64(value) => ContextualConstant::Float64(-value),
            })
        }
        SyntaxKind::BinaryExpression => {
            let [left, right] = node.children.as_slice() else {
                return None;
            };
            let operator = source.text()[left.span.end..right.span.start].trim();
            let valid = if destination.is_integer() {
                matches!(
                    operator,
                    "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>"
                )
            } else {
                matches!(operator, "+" | "-" | "*" | "/" | "%")
            };
            if !valid {
                return None;
            }
            let left = contextual_constant_value(source, left, destination)?;
            let right = contextual_constant_value(source, right, destination)?;
            match (left, right) {
                (Ok(ContextualConstant::Integer(left)), Ok(ContextualConstant::Integer(right))) => {
                    fold_integer_constant(source, node.span, operator, left, right)
                }
                (Ok(ContextualConstant::Float32(left)), Ok(ContextualConstant::Float32(right))) => {
                    Ok(ContextualConstant::Float32(fold_float32_constant(
                        operator, left, right,
                    )))
                }
                (Ok(ContextualConstant::Float64(left)), Ok(ContextualConstant::Float64(right))) => {
                    Ok(ContextualConstant::Float64(fold_float64_constant(
                        operator, left, right,
                    )))
                }
                (Err(error), _) | (_, Err(error)) => Err(error),
                _ => return None,
            }
        }
        SyntaxKind::Literal
            if infer_literal_type_from_source(source, node).is_some_and(is_numeric) =>
        {
            contextual_literal(source, node, destination)
        }
        _ => return None,
    };
    Some(result)
}

fn contextual_literal(
    source: &SourceFile,
    node: &SyntaxNode,
    destination: ScalarType,
) -> Result<ContextualConstant, SemanticFailure> {
    let text = node_text(source, node).replace('_', "");
    let decimal = text.contains('.');
    if destination.is_integer() {
        let value = if decimal {
            let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
            if !fraction.chars().all(|digit| digit == '0') {
                return Err(failure(
                    source,
                    "T0003",
                    format!("constant `{text}` is not an exact `{destination}` value"),
                    node.span,
                ));
            }
            BigInt::parse_bytes(whole.as_bytes(), 10).expect("validated decimal integer constant")
        } else {
            parse_integer_source_text(source, node).expect("validated integer constant")
        };
        Ok(ContextualConstant::Integer(value))
    } else if decimal {
        if destination == ScalarType::Float32 {
            let value = text
                .parse::<f32>()
                .map_err(|_| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float32(value))
        } else {
            let value = text
                .parse::<f64>()
                .map_err(|_| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float64(value))
        }
    } else {
        let integer =
            parse_integer_source_text(source, node).expect("validated whole-number constant");
        if destination == ScalarType::Float32 {
            let value = integer
                .to_f32()
                .filter(|value| BigInt::from_f32(*value).as_ref() == Some(&integer))
                .ok_or_else(|| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float32(value))
        } else {
            let value = integer
                .to_f64()
                .filter(|value| BigInt::from_f64(*value).as_ref() == Some(&integer))
                .ok_or_else(|| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float64(value))
        }
    }
}

fn invalid_floating_constant(
    source: &SourceFile,
    destination: ScalarType,
    span: Span,
) -> SemanticFailure {
    failure(
        source,
        "T0003",
        format!("constant is not a finite exact `{destination}` value"),
        span,
    )
}

fn fold_integer_constant(
    source: &SourceFile,
    span: Span,
    operator: &str,
    left: BigInt,
    right: BigInt,
) -> Result<ContextualConstant, SemanticFailure> {
    let value = match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" if right != BigInt::from(0_u8) => {
            let quotient = &left / &right;
            let remainder = &left % &right;
            if remainder < BigInt::from(0_u8) {
                if right < BigInt::from(0_u8) {
                    quotient + 1
                } else {
                    quotient - 1
                }
            } else {
                quotient
            }
        }
        "%" if right != BigInt::from(0_u8) => {
            let remainder = &left % &right;
            if remainder < BigInt::from(0_u8) {
                if right < BigInt::from(0_u8) {
                    remainder - right
                } else {
                    remainder + right
                }
            } else {
                remainder
            }
        }
        "&" => left & right,
        "|" => left | right,
        "^" => left ^ right,
        "<<" | ">>" => {
            let Some(count) = right.to_usize() else {
                return Err(failure(
                    source,
                    "T0011",
                    "constant shift count cannot be represented on this target",
                    span,
                ));
            };
            if operator == "<<" {
                left << count
            } else {
                left >> count
            }
        }
        _ => {
            return Err(failure(
                source,
                "T0011",
                "invalid constant arithmetic",
                span,
            ));
        }
    };
    Ok(ContextualConstant::Integer(value))
}

fn fold_float32_constant(operator: &str, left: f32, right: f32) -> f32 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "%" => left % right,
        _ => unreachable!("validated constant floating operator"),
    }
}

fn fold_float64_constant(operator: &str, left: f64, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "%" => left % right,
        _ => unreachable!("validated constant floating operator"),
    }
}

fn parse_integer_source_text(source: &SourceFile, node: &SyntaxNode) -> Option<BigInt> {
    let compact = source.text()[node.span.start..node.span.end]
        .chars()
        .filter(|character| !character.is_whitespace() && *character != '_')
        .collect::<String>();
    let (negative, digits) = compact
        .strip_prefix('-')
        .map_or((false, compact.as_str()), |digits| (true, digits));
    let digits = digits.strip_prefix('+').unwrap_or(digits);
    let (radix, digits) = if let Some(digits) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        (16, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0o")
        .or_else(|| digits.strip_prefix("0O"))
    {
        (8, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        (2, digits)
    } else {
        (10, digits)
    };
    let value = BigInt::parse_bytes(digits.as_bytes(), radix)?;
    Some(if negative { -value } else { value })
}

fn check_integer_range(
    source: &SourceFile,
    destination: ScalarType,
    value: &BigInt,
    span: Span,
) -> Result<(), SemanticFailure> {
    let bounds = match destination {
        ScalarType::Int8 => integer_bounds(8, true),
        ScalarType::Int16 => integer_bounds(16, true),
        ScalarType::Int32 => integer_bounds(32, true),
        ScalarType::Int64 => integer_bounds(64, true),
        ScalarType::Int128 => integer_bounds(128, true),
        ScalarType::Uint8 => integer_bounds(8, false),
        ScalarType::Uint16 => integer_bounds(16, false),
        ScalarType::Uint32 => integer_bounds(32, false),
        ScalarType::Uint64 => integer_bounds(64, false),
        ScalarType::Uint128 => integer_bounds(128, false),
        _ => return Ok(()),
    };
    if value < &bounds.0 || value > &bounds.1 {
        return Err(failure(
            source,
            "T0003",
            format!("constant `{value}` is outside the range of `{destination}`"),
            span,
        ));
    }
    Ok(())
}

fn integer_bounds(bits: usize, signed: bool) -> (BigInt, BigInt) {
    if signed {
        let magnitude = BigInt::from(1_u8) << (bits - 1);
        (-&magnitude, magnitude - 1)
    } else {
        (BigInt::from(0_u8), (BigInt::from(1_u8) << bits) - 1)
    }
}

pub(crate) fn promoted_integer_type(left: ScalarType, right: ScalarType) -> ScalarType {
    if left == ScalarType::Int || right == ScalarType::Int {
        return ScalarType::Int;
    }
    let left_bounds = scalar_integer_bounds(left).expect("integer operand has bounds");
    let right_bounds = scalar_integer_bounds(right).expect("integer operand has bounds");
    [
        ScalarType::Int8,
        ScalarType::Uint8,
        ScalarType::Int16,
        ScalarType::Uint16,
        ScalarType::Int32,
        ScalarType::Uint32,
        ScalarType::Int64,
        ScalarType::Uint64,
        ScalarType::Int128,
        ScalarType::Uint128,
    ]
    .into_iter()
    .find(|candidate| {
        let bounds = scalar_integer_bounds(*candidate).expect("fixed integer has bounds");
        bounds.0 <= left_bounds.0
            && bounds.0 <= right_bounds.0
            && bounds.1 >= left_bounds.1
            && bounds.1 >= right_bounds.1
    })
    .unwrap_or(ScalarType::Int)
}

fn scalar_integer_bounds(ty: ScalarType) -> Option<(BigInt, BigInt)> {
    match ty {
        ScalarType::Int8 => Some(integer_bounds(8, true)),
        ScalarType::Int16 => Some(integer_bounds(16, true)),
        ScalarType::Int32 => Some(integer_bounds(32, true)),
        ScalarType::Int64 => Some(integer_bounds(64, true)),
        ScalarType::Int128 => Some(integer_bounds(128, true)),
        ScalarType::Uint8 => Some(integer_bounds(8, false)),
        ScalarType::Uint16 => Some(integer_bounds(16, false)),
        ScalarType::Uint32 => Some(integer_bounds(32, false)),
        ScalarType::Uint64 => Some(integer_bounds(64, false)),
        ScalarType::Uint128 => Some(integer_bounds(128, false)),
        _ => None,
    }
}

struct LexicalScopeContext<'a> {
    namespaces: &'a BTreeMap<String, Namespace>,
    globals: &'a BTreeMap<String, Symbol>,
    prelude_bindings: &'a BTreeMap<String, Symbol>,
}

fn collect_lexical_scopes(
    unit: &SemanticUnit,
    namespaces: &BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    prelude_bindings: &BTreeMap<String, Symbol>,
) -> Result<Vec<LexicalScope>, SemanticFailure> {
    let mut scopes = Vec::new();
    let context = &LexicalScopeContext {
        namespaces,
        globals,
        prelude_bindings,
    };
    for node in &unit.tree.root.children {
        match node.kind {
            SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration => {
                if let Some(block) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
                {
                    for method in block
                        .children
                        .iter()
                        .filter(|child| child.kind == SyntaxKind::FunctionDeclaration)
                    {
                        add_lexical_scope(unit, context, &mut scopes, method, None, true)?;
                    }
                }
            }
            _ if is_function_node(node) => {
                add_lexical_scope(unit, context, &mut scopes, node, None, true)?;
            }
            SyntaxKind::Block => {
                add_lexical_scope(unit, context, &mut scopes, node, None, false)?;
            }
            _ => {}
        }
    }
    Ok(scopes)
}

fn add_lexical_scope(
    unit: &SemanticUnit,
    context: &LexicalScopeContext<'_>,
    scopes: &mut Vec<LexicalScope>,
    node: &SyntaxNode,
    parent: Option<usize>,
    function_body: bool,
) -> Result<usize, SemanticFailure> {
    let namespaces = context.namespaces;
    let index = scopes.len();
    scopes.push(LexicalScope {
        span: node.span,
        parent,
        symbols: BTreeMap::new(),
        import_warnings: Vec::new(),
    });
    if parent.is_none() && function_body {
        let mut namespace_paths = namespace_chain(&unit.namespace).collect::<Vec<_>>();
        namespace_paths.reverse();
        for path in namespace_paths {
            let Some(namespace) = namespaces.get(&path) else {
                continue;
            };
            for (name, symbol) in &namespace.symbols {
                if visible_from(symbol, &unit.namespace) && symbol.available_in_function_body() {
                    scopes[index]
                        .symbols
                        .entry(name.clone())
                        .or_default()
                        .push(symbol.clone());
                }
            }
        }
    }
    if node.kind == SyntaxKind::Block {
        populate_scope(unit, context, scopes, index, node)?;
        return Ok(index);
    }
    if is_function_node(node) && object_name_containing(unit, node.span).is_some() {
        insert_local(
            unit,
            scopes,
            index,
            "self".to_owned(),
            implicit_receiver_span(node, "self"),
        )?;
        let is_static = node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == "static"
        });
        if !is_static {
            insert_local(
                unit,
                scopes,
                index,
                "this".to_owned(),
                implicit_receiver_span(node, "this"),
            )?;
        }
    }
    if is_function_node(node)
        && let Some(parameters) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::ParameterList)
    {
        for parameter in &parameters.children {
            if let Some(name) = declaration_name(parameter, &unit.source) {
                insert_local(unit, scopes, index, name, parameter.span)?;
            }
        }
    }
    for child in &node.children {
        match child.kind {
            SyntaxKind::ParameterList => {}
            SyntaxKind::Block if is_function_node(node) => {
                populate_scope(unit, context, scopes, index, child)?;
            }
            SyntaxKind::Block => {
                add_lexical_scope(unit, context, scopes, child, Some(index), false)?;
            }
            _ if function_body => {
                populate_node(unit, context, scopes, index, child)?;
            }
            _ => {}
        }
    }
    Ok(index)
}

fn populate_scope(
    unit: &SemanticUnit,
    context: &LexicalScopeContext<'_>,
    scopes: &mut Vec<LexicalScope>,
    index: usize,
    block: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    for node in &block.children {
        populate_node(unit, context, scopes, index, node)?;
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "lexical scope construction handles each syntax-owned scope in one traversal"
)]
fn populate_node(
    unit: &SemanticUnit,
    context: &LexicalScopeContext<'_>,
    scopes: &mut Vec<LexicalScope>,
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let namespaces = context.namespaces;
    let globals = context.globals;
    let prelude_bindings = context.prelude_bindings;
    match node.kind {
        SyntaxKind::Binding => {
            populate_binding(unit, scopes, index, node)?;
            for child in &node.children {
                if child.kind == SyntaxKind::AnonymousFunction {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }
        SyntaxKind::Assignment => {
            populate_assignment(unit, namespaces, globals, scopes, index, node)?;
            for child in &node.children {
                if child.kind == SyntaxKind::AnonymousFunction {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }

        SyntaxKind::ImportDeclaration => {
            populate_imports(
                unit,
                namespaces,
                globals,
                prelude_bindings,
                scopes,
                index,
                node,
            )?;
        }
        SyntaxKind::FunctionDeclaration => {
            if let Some(name) = declaration_name(node, &unit.source) {
                insert_local(unit, scopes, index, name, node.span)?;
            }
            add_lexical_scope(unit, context, scopes, node, Some(index), true)?;
        }
        SyntaxKind::AnonymousFunction => {
            add_lexical_scope(unit, context, scopes, node, Some(index), true)?;
        }
        SyntaxKind::Block => {
            add_lexical_scope(unit, context, scopes, node, Some(index), false)?;
        }
        SyntaxKind::ForStatement => {
            let loop_index = scopes.len();
            scopes.push(LexicalScope {
                span: node.span,
                parent: Some(index),
                symbols: BTreeMap::new(),
                import_warnings: Vec::new(),
            });
            if let Some(first) = node.children.first() {
                if first.kind == SyntaxKind::ForTarget {
                    for name in &first.children {
                        insert_local(
                            unit,
                            scopes,
                            loop_index,
                            node_text(&unit.source, name).to_owned(),
                            name.span,
                        )?;
                    }
                } else {
                    populate_node(unit, context, scopes, loop_index, first)?;
                }
            }
            if let Some(block) = node.children.last()
                && block.kind == SyntaxKind::Block
            {
                add_lexical_scope(unit, context, scopes, block, Some(loop_index), false)?;
            }
        }
        SyntaxKind::CatchClause => {
            let catch_index = scopes.len();
            scopes.push(LexicalScope {
                span: node.span,
                parent: Some(index),
                symbols: BTreeMap::new(),
                import_warnings: Vec::new(),
            });
            if let Some(block) = node.children.last()
                && block.kind == SyntaxKind::Block
            {
                add_lexical_scope(unit, context, scopes, block, Some(catch_index), false)?;
            }
        }
        SyntaxKind::ElseClause => {
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    add_lexical_scope(unit, context, scopes, child, Some(index), false)?;
                }
            }
        }
        _ => {
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    add_lexical_scope(unit, context, scopes, child, Some(index), false)?;
                } else if matches!(
                    child.kind,
                    SyntaxKind::AnonymousFunction
                        | SyntaxKind::ElseClause
                        | SyntaxKind::CatchClause
                        | SyntaxKind::FinallyClause
                ) {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }
    }
    Ok(())
}

fn populate_binding(
    unit: &SemanticUnit,
    scopes: &mut [LexicalScope],
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let Some(declaration) = declaration_from_syntax(unit, node) else {
        return Ok(());
    };
    if declaration.global {
        return Ok(());
    }
    let typed_replacement = node
        .children
        .iter()
        .any(|child| child.kind == SyntaxKind::TypeExpression)
        && scopes[index].symbols.contains_key(&declaration.name);
    if typed_replacement {
        insert_local_replacement(unit, scopes, index, declaration.name, node.span);
        Ok(())
    } else {
        insert_local(unit, scopes, index, declaration.name, node.span)
    }
}

fn populate_assignment(
    unit: &SemanticUnit,
    namespaces: &BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    scopes: &mut [LexicalScope],
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let Some(declaration) = declaration_from_syntax(unit, node) else {
        return Ok(());
    };
    let typed_declaration = node
        .children
        .iter()
        .any(|child| child.kind == SyntaxKind::TypeExpression);
    if declaration.global {
        return Ok(());
    }
    if typed_declaration {
        insert_local_replacement(unit, scopes, index, declaration.name, node.span);
        return Ok(());
    }
    if local_binding_exists(scopes, index, &declaration.name) {
        return Ok(());
    }
    let namespace_binding = globals
        .get(&declaration.name)
        .filter(|symbol| symbol.kind == SymbolKind::Binding)
        .or_else(|| {
            namespace_chain(&unit.namespace).find_map(|path| {
                namespaces
                    .get(&path)
                    .and_then(|scope| scope.symbols.get(&declaration.name))
                    .filter(|symbol| symbol.kind == SymbolKind::Binding)
            })
        });
    if let Some(symbol) = namespace_binding {
        let name = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .expect("ordinary assignment has a name");
        if symbol
            .declaration_span
            .is_some_and(|span| declaration_is_constant_in_unit(unit, span))
        {
            return Err(failure(
                &unit.source,
                "S2022",
                format!(
                    "constant binding `{}` cannot be reassigned",
                    declaration.name
                ),
                name.span,
            ));
        }
        return Err(SemanticFailure {
            source: unit.source.clone(),
            diagnostics: vec![
                Diagnostic::error(
                    "S2021",
                    format!(
                        "plain assignment cannot replace namespace binding `{}`",
                        declaration.name
                    ),
                    name.span,
                )
                .with_help(format!(
                    "pass `{}` as a parameter and return changes, or declare it `constant` if it never varies",
                    declaration.name
                )),
            ],
        });
    }
    insert_local(unit, scopes, index, declaration.name, node.span)
}
fn validate_definite_assignment(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        for function in unit
            .tree
            .root
            .children
            .iter()
            .filter(|node| node.kind == SyntaxKind::FunctionDeclaration)
        {
            let mut declared = BTreeSet::new();
            let mut assigned = BTreeSet::new();
            if let Some(parameters) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::ParameterList)
            {
                for parameter in &parameters.children {
                    if let Some(name) = parameter
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)
                    {
                        assigned.insert(node_text(&unit.source, name).to_owned());
                    }
                }
            }
            if let Some(block) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                validate_assignment_block(unit, block, &mut declared, &mut assigned)?;
            }
        }
    }
    Ok(())
}

fn validate_assignment_block(
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
                let Some(name_node) = name_node else {
                    continue;
                };
                let name = node_text(&unit.source, name_node).to_owned();
                let initializer = statement.children.iter().rev().find(|child| {
                    child.span != name_node.span && child.kind != SyntaxKind::TypeExpression
                });
                if let Some(initializer) = initializer {
                    validate_assigned_reads(unit, initializer, declared, assigned)?;
                    assigned.insert(name.clone());
                }
                if statement
                    .children
                    .iter()
                    .any(|child| child.kind == SyntaxKind::TypeExpression)
                {
                    declared.insert(name);
                }
            }
            SyntaxKind::Assignment => {
                if let Some(value) = statement.children.get(1) {
                    validate_assigned_reads(unit, value, declared, assigned)?;
                }
                if let Some(target) = statement.children.first()
                    && target.kind == SyntaxKind::Name
                {
                    assigned.insert(node_text(&unit.source, target).to_owned());
                }
            }
            SyntaxKind::IfStatement => {
                if let Some(condition) = statement.children.first() {
                    validate_assigned_reads(unit, condition, declared, assigned)?;
                }
                let incoming = assigned.clone();
                let mut branch_results = Vec::new();
                for branch in statement.children.iter().skip(1) {
                    let branch_block = if branch.kind == SyntaxKind::Block {
                        Some(branch)
                    } else {
                        branch
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::Block)
                    };
                    if let Some(branch_block) = branch_block {
                        let mut branch_assigned = incoming.clone();
                        validate_assignment_block(
                            unit,
                            branch_block,
                            declared,
                            &mut branch_assigned,
                        )?;
                        branch_results.push(branch_assigned);
                    }
                }
                let has_else = statement
                    .children
                    .iter()
                    .any(|child| child.kind == SyntaxKind::ElseClause);
                if !has_else {
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
            _ => validate_assigned_reads(unit, statement, declared, assigned)?,
        }
    }
    Ok(())
}

fn validate_assigned_reads(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declared: &BTreeSet<String>,
    assigned: &BTreeSet<String>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        if declared.contains(name) && !assigned.contains(name) {
            return Err(failure(
                &unit.source,
                "T0007",
                format!("`{name}` may be read before it is assigned"),
                node.span,
            ));
        }
    }
    for child in &node.children {
        validate_assigned_reads(unit, child, declared, assigned)?;
    }
    Ok(())
}

fn validate_control_flow(package: &SemanticPackage) -> Result<Vec<Vec<Span>>, SemanticFailure> {
    let mut unreachable_units = Vec::with_capacity(package.units.len());
    for unit in &package.units {
        let mut unreachable = Vec::new();
        for function in unit
            .tree
            .root
            .children
            .iter()
            .filter(|node| node.kind == SyntaxKind::FunctionDeclaration)
        {
            let Some(name_node) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            else {
                continue;
            };
            let Some(contract) = unit
                .functions
                .iter()
                .find(|contract| contract.name == node_text(&unit.source, name_node))
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

fn validate_flow_block(
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
fn validate_flow_statement(
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
                let collection_type = infer_value_type(unit, collection, bindings)?;
                let Some(item_type) = collection_type.and_then(iterable_item_type) else {
                    return Err(failure(
                        &unit.source,
                        "T0016",
                        "collection iteration requires an iterable value",
                        collection.span,
                    ));
                };
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

fn validate_bool_condition(
    unit: &SemanticUnit,
    condition: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    if matches!(
        infer_value_type(unit, condition, bindings)?,
        Some(ValueType::Scalar(ScalarType::Bool))
    ) {
        return Ok(());
    }
    Err(failure(
        &unit.source,
        "T0014",
        "control-flow condition must have type `bool`",
        condition.span,
    ))
}

fn validate_if_flow(
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

fn validate_return(
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
                diagnostic_value_type(&unit.objects, &expected)
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
                        diagnostic_value_type(&unit.objects, &expected)
                    ),
                    value.span,
                ));
            };
            validate_value_destination(
                &unit.source,
                &unit.objects,
                &contract.name,
                expected,
                actual,
                value,
                "T0015",
            )
        }
    }
}

fn visible_symbol_for_lexical_import<'a>(
    unit: &SemanticUnit,
    namespaces: &'a BTreeMap<String, Namespace>,
    globals: &'a BTreeMap<String, Symbol>,
    prelude_bindings: &'a BTreeMap<String, Symbol>,
    scopes: &'a [LexicalScope],
    mut index: usize,
    name: &str,
) -> Option<&'a Symbol> {
    loop {
        if let Some(symbol) = scopes[index]
            .symbols
            .get(name)
            .and_then(|symbols| symbols.last())
        {
            return Some(symbol);
        }
        let Some(parent) = scopes[index].parent else {
            break;
        };
        index = parent;
    }
    visible_fallback_symbol(&unit.namespace, name, namespaces, globals, prelude_bindings)
}

fn populate_imports(
    unit: &SemanticUnit,
    namespaces: &BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    prelude_bindings: &BTreeMap<String, Symbol>,
    scopes: &mut [LexicalScope],
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    for import in imports_from_syntax(unit, node)? {
        for (name, mut export) in imported_objects(&import, namespaces)? {
            let existing = if import.namespace_wide {
                visible_symbol_for_lexical_import(
                    unit,
                    namespaces,
                    globals,
                    prelude_bindings,
                    scopes,
                    index,
                    &name,
                )
                .cloned()
            } else {
                scopes[index]
                    .symbols
                    .get(&name)
                    .and_then(|symbols| symbols.last())
                    .cloned()
            };
            if let Some(existing) = existing {
                if existing.identity == export.identity {
                    continue;
                }
                if !import.namespace_wide {
                    return Err(import_collision_failure(&import, &name));
                }
                scopes[index].import_warnings.push(import_overwrite_warning(
                    &name,
                    &existing,
                    &export,
                    import.span,
                ));
            }
            export.binding_span = Some(import.span);
            scopes[index].symbols.insert(name, vec![export]);
        }
    }
    Ok(())
}

fn local_binding_exists(scopes: &[LexicalScope], mut index: usize, name: &str) -> bool {
    loop {
        let scope = &scopes[index];
        if scope.symbols.contains_key(name) {
            return true;
        }
        let Some(parent) = scope.parent else {
            return false;
        };
        index = parent;
    }
}

fn insert_local(
    unit: &SemanticUnit,
    scopes: &mut [LexicalScope],
    index: usize,
    name: String,
    span: Span,
) -> Result<(), SemanticFailure> {
    let scope = &mut scopes[index];
    if scope.symbols.contains_key(&name) {
        return Err(failure(
            &unit.source,
            "S2012",
            format!("duplicate binding `{name}` in the same lexical scope"),
            span,
        ));
    }
    insert_local_replacement(unit, scopes, index, name, span);
    Ok(())
}

fn insert_local_replacement(
    unit: &SemanticUnit,
    scopes: &mut [LexicalScope],
    index: usize,
    name: String,
    span: Span,
) {
    scopes[index]
        .symbols
        .entry(name.clone())
        .or_default()
        .push(Symbol {
            identity: format!("{}::scope{index}::{name}@{}", unit.namespace, span.start),
            lowering_identity: None,
            name,
            namespace: unit.namespace.clone(),
            visibility: Visibility::Private,
            global: false,
            constant: false,
            kind: SymbolKind::Binding,
            declaration_span: Some(span),
            binding_span: Some(span),
        });
}

fn lexical_scope_index_at(unit: &SemanticUnit, offset: usize) -> Option<usize> {
    unit.scopes
        .iter()
        .enumerate()
        .filter(|(_, scope)| scope.span.start <= offset && offset < scope.span.end)
        .min_by_key(|(_, scope)| scope.span.end - scope.span.start)
        .map(|(index, _)| index)
}

fn lexical_scope_chain(unit: &SemanticUnit, offset: usize) -> impl Iterator<Item = &LexicalScope> {
    let mut current = lexical_scope_index_at(unit, offset);
    std::iter::from_fn(move || {
        let index = current?;
        let scope = &unit.scopes[index];
        current = scope.parent;
        Some(scope)
    })
}

fn namespace_chain(namespace: &str) -> impl Iterator<Item = String> {
    let mut current = namespace.trim_end_matches('/').to_owned();
    std::iter::from_fn(move || {
        if current.is_empty() {
            return None;
        }
        let result = current.clone();
        if current == "/" {
            current.clear();
        } else if let Some(separator) = current.rfind('/') {
            current.truncate(separator.max(1));
        } else {
            current.clear();
        }
        Some(result)
    })
}

fn visible_from(symbol: &Symbol, namespace: &str) -> bool {
    match symbol.visibility {
        Visibility::Public => true,
        Visibility::Private => symbol.namespace == namespace,
        Visibility::Protected => {
            symbol.namespace == namespace
                || namespace
                    .strip_prefix(&symbol.namespace)
                    .is_some_and(|suffix| suffix.starts_with('/'))
        }
    }
}
fn resolved_compiler_identity<'a>(unit: &'a SemanticUnit, node: &SyntaxNode) -> Option<&'a str> {
    let name = node_text(&unit.source, node);
    lexical_scope_chain(unit, node.span.start)
        .find_map(|scope| {
            scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol
                    .declaration_span
                    .is_none_or(|span| span.end <= node.span.start)
            })
        })
        .map(Symbol::compiler_identity)
        .or_else(|| (unit.prelude && name == "task-scope").then_some("/core/async::task-scope"))
}

fn constant_deadline_ms(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
    visited: &mut BTreeSet<(u32, usize, usize)>,
) -> Option<u64> {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| constant_deadline_ms(unit, child, bindings, visited));
    }
    if node.kind == SyntaxKind::Name {
        let binding = bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, node)
                && binding.is_visible_at(unit.source.id(), node.span.start)
        })?;
        if !visited.insert((binding.span.file, binding.span.start, binding.span.end)) {
            return None;
        }
        return find_binding_initializer(&unit.tree.root, binding.span)
            .and_then(|value| constant_deadline_ms(unit, value, bindings, visited));
    }
    match contextual_constant(&unit.source, node, ScalarType::Int)? {
        Ok(ContextualConstant::Integer(value)) => value.to_u64(),
        Ok(ContextualConstant::Float32(_) | ContextualConstant::Float64(_)) | Err(_) => None,
    }
}

fn find_binding_initializer(node: &SyntaxNode, name_span: Span) -> Option<&SyntaxNode> {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) && node.span == name_span {
        return node.children.last();
    }
    node.children
        .iter()
        .find_map(|child| find_binding_initializer(child, name_span))
}

fn task_scope_deadline_ms(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
    visited: &mut BTreeSet<(u32, usize, usize)>,
) -> Option<u64> {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| task_scope_deadline_ms(unit, child, bindings, visited));
    }
    if node.kind == SyntaxKind::Name {
        let binding = bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, node)
                && binding.is_visible_at(unit.source.id(), node.span.start)
        })?;
        if !visited.insert((binding.span.file, binding.span.start, binding.span.end)) {
            return None;
        }
        return find_binding_initializer(&unit.tree.root, binding.span)
            .and_then(|value| task_scope_deadline_ms(unit, value, bindings, visited));
    }
    if node.kind != SyntaxKind::CallExpression {
        return None;
    }
    let [callee, arguments] = node.children.as_slice() else {
        return None;
    };
    if callee.kind == SyntaxKind::Name
        && resolved_compiler_identity(unit, callee)
            .is_some_and(|identity| identity == "/core/async::task-scope")
    {
        return arguments
            .children
            .first()
            .and_then(|argument| argument.children.last().or(Some(argument)))
            .and_then(|value| constant_deadline_ms(unit, value, bindings, visited));
    }
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    if callee.kind != SyntaxKind::MemberExpression
        || node_text(&unit.source, member) != "child-scope"
    {
        return None;
    }
    arguments
        .children
        .first()
        .and_then(|argument| argument.children.last().or(Some(argument)))
        .and_then(|value| constant_deadline_ms(unit, value, bindings, visited))
        .or_else(|| task_scope_deadline_ms(unit, receiver, bindings, visited))
}

fn bootstrap_prelude() -> BTreeMap<String, Symbol> {
    const PRELUDE: [(&str, &str, &str); 7] = [
        ("print", "/core/output::print", "/core/output"),
        ("task-scope", "/core/async::task-scope", "/core/async"),
        ("utf8", "/core/encodings::utf8", "/core/encodings"),
        ("utf16-le", "/core/encodings::utf16-le", "/core/encodings"),
        ("utf16-be", "/core/encodings::utf16-be", "/core/encodings"),
        ("utf32-le", "/core/encodings::utf32-le", "/core/encodings"),
        ("utf32-be", "/core/encodings::utf32-be", "/core/encodings"),
    ];
    PRELUDE
        .into_iter()
        .map(|(name, identity, namespace)| {
            (
                name.to_owned(),
                Symbol {
                    identity: identity.to_owned(),
                    lowering_identity: None,
                    name: name.to_owned(),
                    namespace: namespace.to_owned(),
                    visibility: Visibility::Public,
                    global: false,
                    constant: !matches!(name, "print" | "task-scope"),
                    kind: if matches!(name, "print" | "task-scope") {
                        SymbolKind::Function
                    } else {
                        SymbolKind::Binding
                    },
                    declaration_span: None,

                    binding_span: None,
                },
            )
        })
        .collect()
}

fn bootstrap_descriptor_constructs() -> BTreeMap<String, Symbol> {
    ScalarType::SOURCE_NAMES
        .into_iter()
        .map(|(source_name, ty)| {
            let name = source_name.to_owned();
            (
                name.clone(),
                Symbol {
                    identity: format!("/core/types::{}", ty.source_name()),
                    lowering_identity: None,
                    name,
                    namespace: "/core/types".to_owned(),
                    visibility: Visibility::Public,
                    global: false,
                    constant: false,
                    kind: SymbolKind::TypeDescriptor,
                    declaration_span: None,

                    binding_span: None,
                },
            )
        })
        .collect()
}

#[expect(
    clippy::too_many_lines,
    reason = "compiler-owned namespace registration is a single auditable inventory"
)]
fn bootstrap_namespaces() -> BTreeMap<String, Namespace> {
    let mut namespaces = BTreeMap::new();
    namespaces.insert(
        "/core/output".to_owned(),
        namespace_with_objects("/core/output", ["print"], SymbolKind::Function),
    );
    namespaces.insert(
        "/core/async".to_owned(),
        namespace_with_objects("/core/async", ["task-scope"], SymbolKind::Function),
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/codecs",
        "capabilities",
        [
            "hex-encode",
            "hex-decode",
            "base64-encode",
            "base64-decode",
            "result-failed",
            "result-message",
            "result-bytes",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/compression",
        "capabilities",
        [
            "compress",
            "decompress",
            "result-failed",
            "result-resource-limit",
            "result-message",
            "result-bytes",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/concurrency",
        "concurrency",
        [
            "platform-capability",
            "platform-result",
            "no-capability",
            "int-channel",
            "int-channel-send",
            "int-channel-receive",
            "int-channel-try-receive",
            "int-mutex",
            "int-mutex-load",
            "int-mutex-store",
            "int-mutex-add",
            "int-read-write-lock",
            "int-read-write-lock-read",
            "int-read-write-lock-write",
            "atomic-int64",
            "atomic-int64-load",
            "atomic-int64-store",
            "atomic-int64-add",
            "thread-local-int",
            "thread-local-int-get",
            "thread-local-int-set",
            "result-failed",
            "result-message",
            "result-int",
            "result-bool",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/concurrency",
        "capabilities",
        [
            "cancellation-token",
            "cancel",
            "result-deadline-exceeded",
            "result-capability",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/documents",
        "data",
        [
            "platform-data-result",
            "make-document-none",
            "make-document-bool",
            "make-document-string",
            "make-document-integer",
            "make-document-decimal",
            "make-document-list",
            "document-list-append",
            "make-document-map",
            "document-map-insert",
            "empty-document",
            "data-failed",
            "data-message",
            "data-path",
            "data-expected",
            "data-encoded",
            "document-kind",
            "document-text",
            "document-coefficient",
            "document-exponent",
            "document-length",
            "document-item",
            "document-key",
            "document-field",
            "validate-mapping",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/filesystem",
        "system",
        [
            "filesystem-exists",
            "filesystem-metadata",
            "filesystem-realpath",
            "filesystem-read-link",
            "filesystem-read-bounded",
            "filesystem-write-atomic",
            "filesystem-rename",
            "filesystem-remove",
            "result-failed",
            "result-message",
            "result-text",
            "result-detail",
            "result-bytes",
            "result-int",
            "result-bool",
            "filesystem-authority",
            "acquire-filesystem-authority",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/filesystem",
        "streams",
        [
            "resource-handle",
            "open-file",
            "open-directory-beneath",
            "open-file-beneath",
            "read",
            "write",
            "flush",
            "sync-data",
            "sync-all",
            "close",
            "release",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/documents/json",
        "data",
        ["json-parse", "json-canonical"],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/networking",
        "capabilities",
        [
            "platform-resource-handle",
            "platform-capability",
            "platform-result",
            "no-resource",
            "failed-result",
            "parse-ip",
            "parse-host-name",
            "parse-socket",
            "parse-socket-text",
            "tcp-bind",
            "tcp-connect",
            "tcp-connect-host",
            "tcp-accept",
            "tcp-read",
            "tcp-write",
            "tcp-shutdown",
            "tcp-configure",
            "udp-bind",
            "udp-send-to",
            "udp-receive-from",
            "udp-configure",
            "cancellation-token",
            "cancel",
            "dns-lookup",
            "close",
            "result-failed",
            "result-truncated",
            "result-deadline-exceeded",
            "result-message",
            "result-text",
            "result-detail",
            "result-bytes",
            "result-int",
            "result-bool",
            "result-entries",
            "result-resource",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/process",
        "system",
        [
            "process-arguments",
            "environment-entries",
            "process-exit",
            "platform-value-is-text",
            "platform-value-text",
            "platform-value-bytes",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/process",
        "adapters",
        [
            "platform-result",
            "system-host-name",
            "result-failed",
            "result-message",
            "result-text",
            "result-bool",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/random",
        "capabilities",
        [
            "platform-capability",
            "secure-random",
            "pseudo-random",
            "random-bytes",
            "random-bounded",
            "random-split",
            "secret-buffer",
            "destroy-secret",
            "digest",
            "hmac",
            "constant-time-equal",
            "result-failed",
            "result-message",
            "result-bytes",
            "result-int",
            "result-capability",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/streams",
        "streams",
        [
            "resource-handle",
            "acquire-stdin",
            "acquire-stdout",
            "acquire-stderr",
            "read",
            "write",
            "flush",
            "sync-data",
            "sync-all",
            "close",
            "release",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/networking/tls",
        "capabilities",
        [
            "platform-resource-handle",
            "no-resource",
            "tls-client",
            "tls-read",
            "tls-write",
            "tls-shutdown",
            "close",
            "result-failed",
            "result-deadline-exceeded",
            "result-message",
            "result-text",
            "result-bytes",
            "result-int",
            "result-bool",
            "result-resource",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/urls",
        "data",
        [
            "platform-url-result",
            "url-parse",
            "url-failed",
            "url-message",
            "url-serialized",
            "url-display",
            "url-scheme",
            "url-username",
            "url-password",
            "url-host",
            "url-port",
            "url-path",
            "url-fragment",
            "url-origin",
            "url-query-length",
            "url-query-key",
            "url-query-value",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/random/uuid",
        "capabilities",
        [
            "platform-capability",
            "uuid-parse",
            "uuid-v4",
            "uuid-v7",
            "result-failed",
            "result-message",
            "result-text",
            "result-bytes",
        ],
    );
    add_private_host_bindings(
        &mut namespaces,
        "/core/documents/yaml",
        "data",
        ["yaml-parse", "json-canonical"],
    );
    let mut types = vec![
        "int".to_owned(),
        "float".to_owned(),
        "bool".to_owned(),
        "string".to_owned(),
        "bytes".to_owned(),
        "encoding".to_owned(),
        "none".to_owned(),
        "float32".to_owned(),
        "float64".to_owned(),
        "overflow-result".to_owned(),
        "div-rem-result".to_owned(),
    ];
    types.extend(
        TypeCategory::ABSTRACT_SOURCE_NAMES
            .into_iter()
            .map(|(name, _)| name.to_owned()),
    );
    for prefix in ["int", "uint"] {
        for width in [8, 16, 32, 64, 128] {
            types.push(format!("{prefix}{width}"));
        }
    }
    namespaces.insert(
        "/core/types".to_owned(),
        namespace_with_objects(
            "/core/types",
            types.iter().map(std::string::String::as_str),
            SymbolKind::TypeDescriptor,
        ),
    );
    namespaces.insert(
        "/core/encodings".to_owned(),
        namespace_with_objects(
            "/core/encodings",
            ["utf8", "utf16-le", "utf16-be", "utf32-le", "utf32-be"],
            SymbolKind::TypeDescriptor,
        ),
    );
    let mut errors = namespace_with_objects(
        "/core/errors",
        [
            "arithmetic-overflow",
            "division-by-zero",
            "integer-conversion-overflow",
            "negative-shift-count",
            "coercion-error",
            "decode-error",
            "index-error",
            "missing-key",
            "dependency-error",
            "dependency-panic",
        ],
        SymbolKind::ErrorObject,
    );
    errors.symbols.insert(
        "throwable".to_owned(),
        compiler_owned_object("/core/errors", "throwable", SymbolKind::Interface),
    );
    namespaces.insert("/core/errors".to_owned(), errors);
    namespaces.insert(
        "/core/collections".to_owned(),
        namespace_with_objects(
            "/core/collections",
            [
                "iterator",
                "list",
                "map",
                "set",
                "tuple",
                "range",
                "entry",
                "unordered-map",
                "unordered-set",
            ],
            SymbolKind::TypeDescriptor,
        ),
    );
    namespaces
}

fn validate_constant_reassignment(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit_declarations(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && node.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(&unit.source, child) == "constant"
            })
            && let Some(target) = first_write_to(package, unit, node.span, &unit.tree.root)
        {
            let name = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .map_or("constant", |child| node_text(&unit.source, child));
            return Err(failure(
                &unit.source,
                "S2022",
                format!("constant binding `{name}` cannot be reassigned"),
                target.span,
            ));
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && node.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(&unit.source, child) == "global"
            })
            && let Some(target) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            && let Some(symbol) =
                package.resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            && symbol
                .declaration_span
                .is_some_and(|span| declaration_is_constant(package, span))
        {
            return Err(failure(
                &unit.source,
                "S2022",
                format!(
                    "constant binding `{}` cannot be reassigned",
                    node_text(&unit.source, target)
                ),
                target.span,
            ));
        }
        for child in &node.children {
            visit_declarations(package, unit, child)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit_declarations(package, unit, &unit.tree.root)?;
    }
    Ok(())
}

fn declaration_is_constant_in_unit(unit: &SemanticUnit, span: Span) -> bool {
    fn find(node: &SyntaxNode, span: Span, source: &SourceFile) -> Option<bool> {
        if node.span == span {
            return Some(node.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(source, child) == "constant"
            }));
        }
        node.children
            .iter()
            .find_map(|child| find(child, span, source))
    }

    span.file == unit.source.id() && find(&unit.tree.root, span, &unit.source).unwrap_or(false)
}

fn declaration_is_constant(package: &SemanticPackage, span: Span) -> bool {
    package
        .units
        .iter()
        .find(|unit| unit.source.id() == span.file)
        .is_some_and(|unit| declaration_is_constant_in_unit(unit, span))
}

#[expect(
    clippy::too_many_lines,
    reason = "the global assignment transfer rules remain visible as one analysis"
)]
fn validate_global_definite_assignment(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn has_qualifier(unit: &SemanticUnit, node: &SyntaxNode, qualifier: &str) -> bool {
        node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == qualifier
        })
    }

    fn has_initializer(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        unit.source.text()[node.span.start..node.span.end].contains('=')
    }

    fn global_name<'a>(unit: &'a SemanticUnit, node: &'a SyntaxNode) -> Option<&'a str> {
        node.children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .map(|child| node_text(&unit.source, child))
    }

    fn collect_writes(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        writes: &mut BTreeSet<String>,
    ) {
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && has_qualifier(unit, node, "global")
            && has_initializer(unit, node)
            && let Some(name) = global_name(unit, node)
        {
            writes.insert(name.to_owned());
        } else if node.kind == SyntaxKind::PostfixExpression
            && let Some(target) = node.children.first()
            && package
                .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
                .is_some_and(|symbol| symbol.global)
        {
            writes.insert(node_text(&unit.source, target).to_owned());
        }
        for child in &node.children {
            collect_writes(package, unit, child, writes);
        }
    }

    fn validate_node(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        relevant: &BTreeSet<String>,
        assigned: &mut BTreeSet<String>,
    ) -> Result<(), SemanticFailure> {
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && has_qualifier(unit, node, "global")
        {
            let name_node = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name);
            for child in &node.children {
                if Some(child.span) != name_node.map(|name| name.span) {
                    validate_node(package, unit, child, relevant, assigned)?;
                }
            }
            if has_initializer(unit, node)
                && let Some(name) = name_node.map(|name| node_text(&unit.source, name))
            {
                assigned.insert(name.to_owned());
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::PostfixExpression
            && let Some(target) = node.children.first()
            && let Some(symbol) =
                package.resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            && symbol.global
        {
            let name = node_text(&unit.source, target);
            if relevant.contains(name) && !assigned.contains(name) {
                return Err(failure(
                    &unit.source,
                    "T0007",
                    format!("`{name}` may be read before it is assigned"),
                    target.span,
                ));
            }
            assigned.insert(name.to_owned());
            return Ok(());
        }
        if node.kind == SyntaxKind::Name
            && let Some(symbol) =
                package.resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
            && symbol.global
        {
            let name = node_text(&unit.source, node);
            if relevant.contains(name) && !assigned.contains(name) {
                return Err(failure(
                    &unit.source,
                    "T0007",
                    format!("`{name}` may be read before it is assigned"),
                    node.span,
                ));
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::IfStatement {
            if let Some(condition) = node.children.first() {
                validate_node(package, unit, condition, relevant, assigned)?;
            }
            let incoming = assigned.clone();
            let mut branch_results = Vec::new();
            for branch in node.children.iter().skip(1) {
                let branch_block = if branch.kind == SyntaxKind::Block {
                    Some(branch)
                } else {
                    branch
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block)
                };
                if let Some(branch_block) = branch_block {
                    let mut branch_assigned = incoming.clone();
                    validate_node(package, unit, branch_block, relevant, &mut branch_assigned)?;
                    branch_results.push(branch_assigned);
                }
            }
            if !node
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::ElseClause)
            {
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
            return Ok(());
        }
        if node.kind == SyntaxKind::WhileStatement {
            let before = assigned.clone();
            for child in &node.children {
                let mut branch = before.clone();
                validate_node(package, unit, child, relevant, &mut branch)?;
            }
            return Ok(());
        }
        for child in &node.children {
            validate_node(package, unit, child, relevant, assigned)?;
        }
        Ok(())
    }

    let mut uninitialized = package
        .globals
        .values()
        .filter(|symbol| symbol.kind == SymbolKind::Binding)
        .map(|symbol| symbol.name.clone())
        .collect::<BTreeSet<_>>();
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
                && has_qualifier(unit, node, "global")
                && has_initializer(unit, node)
                && let Some(name) = global_name(unit, node)
            {
                uninitialized.remove(name);
            }
        }
    }
    if uninitialized.is_empty() {
        return Ok(());
    }

    let mut writes = BTreeSet::new();
    for unit in &package.units {
        collect_writes(package, unit, &unit.tree.root, &mut writes);
    }
    for unit in &package.units {
        for function in unit
            .tree
            .root
            .children
            .iter()
            .filter(|node| node.kind == SyntaxKind::FunctionDeclaration)
        {
            let mut function_writes = BTreeSet::new();
            collect_writes(package, unit, function, &mut function_writes);
            let relevant = uninitialized
                .iter()
                .filter(|name| function_writes.contains(*name) || !writes.contains(*name))
                .cloned()
                .collect();
            validate_node(package, unit, function, &relevant, &mut BTreeSet::new())?;
        }
    }
    Ok(())
}

fn first_write_to<'a>(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    node: &'a SyntaxNode,
) -> Option<&'a SyntaxNode> {
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::Name
        && let Some(symbol) =
            package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
        && let Some(crate::projection::ProjectedKind::Function(function)) = package
            .projection
            .item(&symbol.namespace, &symbol.name)
            .map(|item| &item.kind)
    {
        let mut positional = 0;
        for argument in &arguments.children {
            let named = argument
                .children
                .first()
                .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
            let index = named.map_or_else(
                || {
                    let index = positional;
                    positional += 1;
                    index
                },
                |name| {
                    function
                        .parameters
                        .iter()
                        .position(|parameter| parameter.name == node_text(&unit.source, name))
                        .unwrap_or(usize::MAX)
                },
            );
            let value = argument.children.last().unwrap_or(argument);
            if function
                .parameters
                .get(index)
                .is_some_and(|parameter| parameter.mutable_borrow)
                && value.kind == SyntaxKind::Name
                && package
                    .resolve_name_at(unit, value.span.start, node_text(&unit.source, value))
                    .is_some_and(|symbol| symbol.declaration_span == Some(declaration_span))
            {
                return Some(value);
            }
        }
    }
    if matches!(
        node.kind,
        SyntaxKind::Assignment | SyntaxKind::PostfixExpression
    ) && node.span != declaration_span
        && let Some(target) = node.children.first()
        && target.kind == SyntaxKind::Name
        && package
            .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            .is_some_and(|symbol| symbol.declaration_span == Some(declaration_span))
    {
        return Some(target);
    }
    node.children
        .iter()
        .find_map(|child| first_write_to(package, unit, declaration_span, child))
}

fn record_binding_mutability(package: &mut SemanticPackage) {
    let mutable_bindings = package
        .units
        .iter()
        .map(|unit| {
            unit.typed_bindings
                .iter()
                .map(|binding| {
                    let initially_assigned =
                        unit.source.text()[binding.span.start..binding.span.end].contains('=');
                    binding_span_is_mutated(package, unit, binding.span, initially_assigned)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mutable_parameters = package
        .units
        .iter()
        .map(|unit| {
            unit.functions
                .iter()
                .map(|function| {
                    function
                        .parameters
                        .iter()
                        .map(|parameter| {
                            binding_span_is_mutated(package, unit, parameter.span, true)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for ((unit, binding_mutability), parameter_mutability) in package
        .units
        .iter_mut()
        .zip(mutable_bindings)
        .zip(mutable_parameters)
    {
        for (binding, mutable) in unit.typed_bindings.iter_mut().zip(binding_mutability) {
            binding.mutable = mutable;
        }
        for (function, mutability) in unit.functions.iter_mut().zip(parameter_mutability) {
            for (parameter, mutable) in function.parameters.iter_mut().zip(mutability) {
                parameter.mutable = mutable;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ControlRegion {
    statement: Span,
    arm: Option<usize>,
}

#[derive(Clone, Debug)]
enum BindingEvent {
    Read {
        span: Span,
        loops: Vec<Span>,
        regions: Vec<ControlRegion>,
    },
    Write {
        span: Span,
        loops: Vec<Span>,
        regions: Vec<ControlRegion>,
    },
}

fn span_key(span: Span) -> (u32, usize, usize) {
    (span.file, span.start, span.end)
}

fn binding_event_child_repeats(node: &SyntaxNode, index: usize) -> bool {
    if node.kind == SyntaxKind::ForStatement
        && node
            .children
            .get(index)
            .is_some_and(|child| child.kind == SyntaxKind::ForTarget)
    {
        return true;
    }
    match node.kind {
        SyntaxKind::WhileStatement => true,
        SyntaxKind::ForStatement if node.children.len() == 3 => index == 2,
        SyntaxKind::ForStatement if node.children.len() == 4 => index != 0,
        _ => false,
    }
}

fn binding_event_child_region(
    node: &SyntaxNode,
    child: &SyntaxNode,
    index: usize,
) -> Option<ControlRegion> {
    if node.kind == SyntaxKind::ForStatement && child.kind == SyntaxKind::ForTarget {
        return Some(ControlRegion {
            statement: node.span,
            arm: None,
        });
    }
    if node.kind == SyntaxKind::IfStatement
        && matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause)
    {
        return Some(ControlRegion {
            statement: node.span,
            arm: Some(index),
        });
    }
    if child.kind != SyntaxKind::Block {
        return None;
    }
    let statement = match node.kind {
        SyntaxKind::WhileStatement | SyntaxKind::ForStatement => node.span,
        SyntaxKind::TryStatement | SyntaxKind::CatchClause | SyntaxKind::FinallyClause => {
            child.span
        }
        _ => return None,
    };
    Some(ControlRegion {
        statement,
        arm: None,
    })
}

fn node_may_declare_typed_binding(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::Parameter
            | SyntaxKind::ForTarget
    )
}

fn declared_bindings_at_node<'a>(
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
) -> impl Iterator<Item = &'a TypedBinding> {
    unit.typed_bindings.iter().filter(move |binding| {
        if node.kind == SyntaxKind::ForTarget {
            node.children.iter().any(|name| binding.span == name.span)
        } else {
            binding.span == node.span
        }
    })
}

fn initial_store_span(node: &SyntaxNode, binding: &TypedBinding) -> Span {
    if node.kind == SyntaxKind::ForTarget {
        binding.span
    } else {
        node.span
    }
}

fn record_declared_binding_writes(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declares_binding: bool,
    events: &mut BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
    loops: &[Span],
    regions: &[ControlRegion],
) -> bool {
    if !declares_binding {
        return false;
    }
    let initial_store = node.kind == SyntaxKind::ForTarget
        || node.kind == SyntaxKind::Parameter
        || unit.source.text()[node.span.start..node.span.end].contains('=');
    if !initial_store {
        return false;
    }
    let mut recorded = false;
    for binding in declared_bindings_at_node(unit, node) {
        recorded = true;
        events
            .entry(span_key(binding.span))
            .or_default()
            .push(BindingEvent::Write {
                span: initial_store_span(node, binding),
                loops: loops.to_vec(),
                regions: regions.to_vec(),
            });
    }
    recorded
}

fn collect_binding_events(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    events: &mut BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
    declaration_name: bool,
    loops: &mut Vec<Span>,
    regions: &mut Vec<ControlRegion>,
) {
    if node.kind == SyntaxKind::Name {
        let function_span = unit
            .enclosing_function_spans
            .get(&node.span.start)
            .copied()
            .flatten();
        let typed_declaration = unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, node)
                && binding.is_visible_at(unit.source.id(), node.span.start)
                && unit
                    .enclosing_function_spans
                    .get(&binding.span.start)
                    .copied()
                    .flatten()
                    == function_span
        });
        let declaration_span = typed_declaration.map(|binding| binding.span).or_else(|| {
            package
                .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                .and_then(|symbol| symbol.declaration_span)
        });
        if !declaration_name && let Some(declaration_span) = declaration_span {
            events
                .entry(span_key(declaration_span))
                .or_default()
                .push(BindingEvent::Read {
                    span: node.span,
                    loops: loops.clone(),
                    regions: regions.clone(),
                });
        }
        return;
    }

    let declares_binding = node_may_declare_typed_binding(node)
        && declared_bindings_at_node(unit, node).next().is_some();
    let assignment_target = if matches!(
        node.kind,
        SyntaxKind::Assignment | SyntaxKind::PostfixExpression
    ) && !declares_binding
    {
        node.children
            .first()
            .filter(|target| target.kind == SyntaxKind::Name)
    } else {
        None
    };

    for (index, child) in node.children.iter().enumerate() {
        let declares_child = child.kind == SyntaxKind::Name
            && if node.kind == SyntaxKind::ForTarget {
                true
            } else {
                (declares_binding || node.kind == SyntaxKind::Parameter)
                    && !node.children[..index]
                        .iter()
                        .any(|prior| prior.kind == SyntaxKind::Name)
            };
        let plain_assignment_target =
            assignment_target.is_some() && node.kind == SyntaxKind::Assignment && index == 0;
        if !plain_assignment_target {
            let repeats = binding_event_child_repeats(node, index);
            let region = binding_event_child_region(node, child, index);
            if repeats {
                loops.push(node.span);
            }
            if let Some(region) = region {
                regions.push(region);
            }
            collect_binding_events(package, unit, child, events, declares_child, loops, regions);
            if region.is_some() {
                regions.pop();
            }
            if repeats {
                loops.pop();
            }
        }
    }
    if !record_declared_binding_writes(unit, node, declares_binding, events, loops, regions)
        && let Some(target) = assignment_target
        && let Some(declaration_span) = package
            .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            .and_then(|symbol| symbol.declaration_span)
    {
        events
            .entry(span_key(declaration_span))
            .or_default()
            .push(BindingEvent::Write {
                span: node.span,
                loops: loops.clone(),
                regions: regions.clone(),
            });
    }
}

fn record_binding_events(package: &mut SemanticPackage) {
    let mut events = BTreeMap::new();
    for unit in &package.units {
        collect_binding_events(
            package,
            unit,
            &unit.tree.root,
            &mut events,
            false,
            &mut Vec::new(),
            &mut Vec::new(),
        );
    }
    package.binding_events = events;
}

fn regions_conflict(left: &[ControlRegion], right: &[ControlRegion]) -> bool {
    left.iter().any(|left| {
        right.iter().any(|right| {
            left.statement == right.statement
                && left.arm.is_some()
                && right.arm.is_some()
                && left.arm != right.arm
        })
    })
}

fn later_store_replaces(earlier: &[ControlRegion], later: &[ControlRegion]) -> bool {
    later.iter().all(|region| earlier.contains(region))
}

fn collect_suspension_points(unit: &SemanticUnit, node: &SyntaxNode, spans: &mut Vec<Span>) {
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("await")
    {
        spans.push(node.span);
    }
    for child in &node.children {
        collect_suspension_points(unit, child, spans);
    }
}

fn moves_binding_between(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    owner_span: Span,
    after: usize,
    before: usize,
) -> bool {
    if node.span.start >= after
        && node.span.end <= before
        && node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("move")
        && node.children.last().is_some_and(|operand| {
            operand.kind == SyntaxKind::Name
                && package
                    .resolve_name_at(unit, operand.span.start, node_text(&unit.source, operand))
                    .and_then(|symbol| symbol.declaration_span)
                    == Some(owner_span)
        })
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| moves_binding_between(package, unit, child, owner_span, after, before))
}

fn reference_has_stable_local_owner(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    contract: &FunctionContract,
    reference: &TypedBinding,
) -> bool {
    let Some(declaration) = find_node_by_span(&unit.tree.root, reference.span) else {
        return false;
    };
    let Some(initializer) = declaration.children.last() else {
        return false;
    };
    if initializer.kind != SyntaxKind::UnaryExpression
        || unary_operator_text(unit, initializer).as_deref() != Some("ref")
    {
        return false;
    }
    let Some(source) = initializer
        .children
        .last()
        .filter(|source| source.kind == SyntaxKind::Name)
    else {
        return false;
    };
    let Some(owner_span) = package
        .resolve_name_at(unit, source.span.start, node_text(&unit.source, source))
        .and_then(|symbol| symbol.declaration_span)
    else {
        return false;
    };
    if !unit.typed_bindings.iter().any(|owner| {
        owner.span == owner_span
            && owner.span.start >= contract.span.start
            && owner.span.end <= contract.span.end
    }) {
        return false;
    }
    !moves_binding_between(
        package,
        unit,
        &unit.tree.root,
        owner_span,
        reference.visible_from,
        contract.span.end,
    ) && package
        .binding_events
        .get(&span_key(owner_span))
        .is_none_or(|events| {
            !events.iter().any(|event| {
                matches!(
                    event,
                    BindingEvent::Write { span, .. } if span.start >= reference.visible_from
                )
            })
        })
}

fn validate_suspension_ownership(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        let mut awaits = Vec::new();
        collect_suspension_points(unit, &unit.tree.root, &mut awaits);
        for contract in unit.functions.iter().filter(|contract| contract.is_async) {
            for binding in unit.typed_bindings.iter().filter(|binding| {
                matches!(binding.value_type, ValueType::Reference(_))
                    && binding.span.start >= contract.span.start
                    && binding.span.end <= contract.span.end
            }) {
                let Some(events) = package.binding_events.get(&span_key(binding.span)) else {
                    continue;
                };
                if let Some(suspension) = awaits.iter().find(|suspension| {
                    suspension.start >= binding.visible_from
                        && suspension.end <= contract.span.end
                        && events.iter().any(|event| {
                            matches!(
                                event,
                                BindingEvent::Read { span, .. } if span.start > suspension.end
                            )
                        })
                        && !reference_has_stable_local_owner(package, unit, contract, binding)
                }) {
                    return Err(failure(
                        &unit.source,
                        "T0073",
                        format!(
                            "non-owning reference `{}` remains live across `await`; end its use before suspension or transfer owned state",
                            binding.name
                        ),
                        *suspension,
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_task_consumption(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn consumed(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        binding: &TypedBinding,
        join_argument: bool,
    ) -> bool {
        let await_operand = node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("await");
        let joined = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                callee.kind == SyntaxKind::MemberExpression
                    && callee
                        .children
                        .get(1)
                        .is_some_and(|member| node_text(&unit.source, member) == "join")
            });
        if join_argument
            && node.kind == SyntaxKind::Name
            && node_text(&unit.source, node) == binding.name
            && unit
                .typed_bindings
                .iter()
                .rev()
                .find(|candidate| {
                    candidate.name == binding.name
                        && candidate.is_visible_at(unit.source.id(), node.span.start)
                })
                .is_some_and(|candidate| candidate.span == binding.span)
        {
            return true;
        }
        node.children.iter().enumerate().any(|(index, child)| {
            consumed(
                unit,
                child,
                binding,
                join_argument || await_operand || (joined && index == 1),
            )
        })
    }

    for unit in &package.units {
        for binding in unit.typed_bindings.iter().filter(|binding| {
            matches!(
                binding.value_type,
                ValueType::Task(_) | ValueType::ScopedTask(_)
            )
        }) {
            if !consumed(unit, &unit.tree.root, binding, false) {
                return Err(failure(
                    &unit.source,
                    "T0076",
                    format!(
                        "task `{}` must be awaited or joined before its scope ends",
                        binding.name
                    ),
                    binding.span,
                ));
            }
        }
    }
    Ok(())
}


pub(crate) fn descriptor_binding_is_materialized(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
) -> bool {
    fn read_materializes(node: &SyntaxNode, read_span: Span, is_designator: bool) -> Option<bool> {
        if node.kind == SyntaxKind::Name && node.span == read_span {
            return Some(!is_designator);
        }
        node.children.iter().enumerate().find_map(|(index, child)| {
            let child_is_designator = index == 0
                && matches!(
                    node.kind,
                    SyntaxKind::ConstructionExpression | SyntaxKind::StaticMemberExpression
                );
            read_materializes(child, read_span, child_is_designator)
        })
    }

    package
        .binding_events
        .get(&span_key(declaration_span))
        .is_some_and(|events| {
            events.iter().any(|event| {
                let BindingEvent::Read { span, .. } = event else {
                    return false;
                };
                read_materializes(&unit.tree.root, *span, false).unwrap_or(false)
            })
        })
}

pub(crate) fn binding_store_value_is_read(
    package: &SemanticPackage,
    declaration_span: Span,
    store_span: Span,
) -> bool {
    let Some(events) = package.binding_events.get(&span_key(declaration_span)) else {
        return false;
    };
    let Some((store, store_loops, store_regions)) =
        events.iter().enumerate().find_map(|(index, event)| {
            let BindingEvent::Write {
                span,
                loops,
                regions,
            } = event
            else {
                return None;
            };
            (*span == store_span).then_some((index, loops, regions))
        })
    else {
        return false;
    };
    let mut intervening_stores: Vec<&[ControlRegion]> = Vec::new();
    for event in &events[store + 1..] {
        match event {
            BindingEvent::Read { regions, .. }
                if !regions_conflict(store_regions, regions)
                    && !intervening_stores
                        .iter()
                        .any(|intervening| later_store_replaces(regions, intervening)) =>
            {
                return true;
            }
            BindingEvent::Write { regions, .. } => {
                if later_store_replaces(store_regions, regions) {
                    return false;
                }
                intervening_stores.push(regions.as_slice());
            }
            BindingEvent::Read { .. } => {}
        }
    }
    !store_loops.is_empty()
        && events.iter().any(|event| {
            let BindingEvent::Read {
                loops: read_loops, ..
            } = event
            else {
                return false;
            };
            store_loops
                .iter()
                .any(|store_loop| read_loops.contains(store_loop))
        })
}

fn collect_loop_target_spans(node: &SyntaxNode, loop_targets: &mut BTreeSet<(u32, usize, usize)>) {
    if node.kind == SyntaxKind::ForTarget {
        loop_targets.extend(node.children.iter().map(|name| span_key(name.span)));
    }
    for child in &node.children {
        collect_loop_target_spans(child, loop_targets);
    }
}

fn invalid_name_style_declarations(unit: &SemanticUnit) -> Vec<(&str, Span)> {
    let mut declarations = unit
        .typed_bindings
        .iter()
        .map(|binding| (binding.name.as_str(), binding.span))
        .chain(
            unit.functions
                .iter()
                .map(|function| (function.name.as_str(), function.span)),
        )
        .chain(
            unit.objects
                .iter()
                .map(|object| (object.name.as_str(), object.span)),
        )
        .collect::<Vec<_>>();
    declarations.sort_by_key(|(_, span)| (span.start, span.end));
    declarations.dedup_by_key(|(_, span)| (span.start, span.end));
    declarations.retain(|(text, _)| {
        !text.bytes().enumerate().all(|(index, byte)| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || (index > 0 && byte == b'-')
        })
    });
    declarations
}

fn validate_compiler_owned_names(units: &[SemanticUnit]) -> Result<(), SemanticFailure> {
    for unit in units
        .iter()
        .filter(|unit| unit.bundled && !unit.namespace.starts_with("/deps/"))
    {
        if let Some((text, span)) = invalid_name_style_declarations(unit).into_iter().next() {
            return Err(failure(
                &unit.source,
                "S2018",
                format!("compiler-owned declaration `{text}` is not kebab-case"),
                span,
            ));
        }
    }
    Ok(())
}

fn collect_name_style_warnings(unit: &SemanticUnit, warnings: &mut Vec<Diagnostic>) {
    for (text, span) in invalid_name_style_declarations(unit) {
        warnings.push(
            Diagnostic::warning(
                "S2018",
                format!("declared name `{text}` is not kebab-case"),
                span,
            )
            .with_help(
                "use kebab-case for Terrane-owned declarations; projected dependency names remain verbatim",
            ),
        );
    }
}

fn union_arm_identity(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    text: &str,
    position: usize,
) -> String {
    if let Some(scalar) = unit.descriptor_alias_at(text, position) {
        return format!("scalar:{}", scalar.source_name());
    }
    package
        .resolve_name_at(unit, position, text)
        .filter(|symbol| {
            matches!(
                symbol.kind,
                SymbolKind::Class
                    | SymbolKind::Interface
                    | SymbolKind::Trait
                    | SymbolKind::ErrorObject
            )
        })
        .map_or_else(
            || format!("unresolved:{text}"),
            |symbol| format!("object:{}", symbol.identity),
        )
}

fn collect_duplicate_union_arm_warnings(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    warnings: &mut Vec<Diagnostic>,
) {
    fn collect(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        warnings: &mut Vec<Diagnostic>,
    ) {
        if node.kind == SyntaxKind::UnionType {
            let mut seen = BTreeSet::new();
            for arm in &node.children {
                let text = node_text(&unit.source, arm).trim();
                let identity = union_arm_identity(package, unit, text, arm.span.start);
                if !seen.insert(identity) {
                    warnings.push(
                        Diagnostic::warning(
                            "W4003",
                            format!("union arm `{text}` duplicates an earlier arm"),
                            arm.span,
                        )
                        .with_help(
                            "remove the repeated arm; union arms are normalized by semantic identity",
                        ),
                    );
                }
            }
        }
        for child in &node.children {
            collect(package, unit, child, warnings);
        }
    }

    collect(package, unit, &unit.tree.root, warnings);
}

pub(crate) fn warnings(package: &SemanticPackage, lint_name_style: bool) -> Vec<Diagnostic> {
    let mut warnings = Vec::new();
    warnings.extend(package.import_warnings.iter().cloned());
    for unit in &package.units {
        if lint_name_style && !unit.bundled && !unit.namespace.starts_with("/deps/") {
            collect_name_style_warnings(unit, &mut warnings);
        }
        collect_duplicate_union_arm_warnings(package, unit, &mut warnings);
        let mut loop_targets = BTreeSet::new();
        collect_loop_target_spans(&unit.tree.root, &mut loop_targets);
        for binding in &unit.typed_bindings {
            if package
                .globals
                .values()
                .any(|symbol| symbol.declaration_span == Some(binding.span))
            {
                continue;
            }
            let parameter = unit.functions.iter().any(|contract| {
                contract
                    .parameters
                    .iter()
                    .any(|parameter| parameter.span == binding.span)
            });
            let loop_target = loop_targets.contains(&span_key(binding.span));
            let Some(events) = package.binding_events.get(&span_key(binding.span)) else {
                continue;
            };
            for (index, event) in events.iter().enumerate() {
                let BindingEvent::Write {
                    span: store_span, ..
                } = event
                else {
                    continue;
                };
                if binding_store_value_is_read(package, binding.span, *store_span) {
                    continue;
                }
                let later_store = events[index + 1..]
                    .iter()
                    .any(|event| matches!(event, BindingEvent::Write { .. }));
                let initial_store = *store_span == binding.span;
                let (code, message) = if initial_store && !later_store {
                    if parameter || loop_target {
                        continue;
                    }
                    ("W4001", format!("binding `{}` is never read", binding.name))
                } else if initial_store {
                    (
                        "W4002",
                        format!("initial value assigned to `{}` is never read", binding.name),
                    )
                } else {
                    (
                        "W4002",
                        format!("value assigned to `{}` is never read", binding.name),
                    )
                };
                warnings.push(Diagnostic::warning(code, message, *store_span));
            }
        }
    }
    warnings.sort_by_key(|diagnostic| {
        diagnostic
            .primary
            .map_or((u32::MAX, usize::MAX), |span| (span.file, span.start))
    });
    warnings
}

fn object_method_mutates(
    package: &SemanticPackage,
    object_identity: &ObjectIdentity,
    method_name: &str,
) -> bool {
    fn contract_mutates(unit: &SemanticUnit, object_name: &str, method_name: &str) -> bool {
        if let Some(method) = unit.functions.iter().find(|method| {
            method.owner.as_deref() == Some(object_name) && method.name == method_name
        }) {
            return method.mutates_receiver;
        }
        unit.objects
            .iter()
            .find(|object| object.name == object_name)
            .and_then(|object| object.base.as_ref())
            .and_then(|base| unit.objects.iter().find(|object| object.identity == *base))
            .is_some_and(|base| contract_mutates(unit, &base.name, method_name))
    }

    if package.units.iter().any(|candidate| {
        candidate
            .objects
            .iter()
            .find(|object| object.identity == *object_identity)
            .is_some_and(|object| contract_mutates(candidate, &object.name, method_name))
    }) {
        return true;
    }

    package
        .projection
        .method(
            &object_identity.namespace,
            &object_identity.name,
            method_name,
        )
        .is_some_and(|method| {
            matches!(
                method.receiver,
                Some(crate::projection::Receiver::MutableBorrow)
            )
        })
}

pub(crate) fn binding_span_is_mutated(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    initially_assigned: bool,
) -> bool {
    fn writes(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        declaration_span: Span,
        iterator_binding: bool,
        node: &SyntaxNode,
    ) -> usize {
        let resolves_to_binding = |target: &SyntaxNode| {
            target.kind == SyntaxKind::Name
                && !package.is_lexical_replacement(unit, node.span, node_text(&unit.source, target))
                && package
                    .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
                    .is_some_and(|symbol| symbol.declaration_span == Some(declaration_span))
        };
        let direct_write = matches!(
            node.kind,
            SyntaxKind::Assignment | SyntaxKind::PostfixExpression
        ) && node.span != declaration_span
            && node.children.first().is_some_and(|target| {
                resolves_to_binding(target)
                    || (matches!(
                        target.kind,
                        SyntaxKind::IndexExpression | SyntaxKind::MemberExpression
                    ) && target.children.first().is_some_and(resolves_to_binding))
            });
        let mutator_call = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                let [receiver, member] = callee.children.as_slice() else {
                    return false;
                };
                callee.kind == SyntaxKind::MemberExpression
                    && (matches!(
                        node_text(&unit.source, member),
                        "append" | "set" | "add" | "remove"
                    ) || matches!(
                        infer_value_type(unit, receiver, &unit.typed_bindings),
                        Ok(Some(ValueType::Object(object)))
                            if object_method_mutates(
                                package,
                                &object,
                                node_text(&unit.source, member)
                            )
                    ))
                    && resolves_to_binding(receiver)
            });
        let iterator_advance = iterator_binding
            && node.kind == SyntaxKind::ForStatement
            && node.children.get(1).is_some_and(resolves_to_binding);
        let writes_here = usize::from(direct_write || mutator_call || iterator_advance);
        writes_here
            + node
                .children
                .iter()
                .map(|child| writes(package, unit, declaration_span, iterator_binding, child))
                .sum::<usize>()
    }

    let iterator_binding = unit.typed_bindings.iter().any(|binding| {
        binding.span == declaration_span && matches!(binding.value_type, ValueType::Iterator(_))
    });
    writes(
        package,
        unit,
        declaration_span,
        iterator_binding,
        &unit.tree.root,
    ) > usize::from(!initially_assigned)
}

fn add_private_host_bindings<'a>(
    namespaces: &mut BTreeMap<String, Namespace>,
    path: &str,
    group: &str,
    bindings: impl IntoIterator<Item = &'a str>,
) {
    let namespace = namespaces.entry(path.to_owned()).or_default();
    for intrinsic in bindings {
        let local_name = format!("host-{intrinsic}");
        let previous = namespace.symbols.insert(
            local_name.clone(),
            Symbol {
                identity: format!("{path}::{local_name}"),
                lowering_identity: Some(format!("intrinsic:{group}::{intrinsic}")),
                name: local_name.clone(),
                namespace: path.to_owned(),
                visibility: Visibility::Private,
                global: false,
                constant: false,
                kind: SymbolKind::Function,
                declaration_span: None,

                binding_span: None,
            },
        );
        assert!(
            previous.is_none(),
            "duplicate private host binding `{path}::{local_name}`"
        );
    }
}

fn namespace_with_objects<'a>(
    path: &str,
    names: impl IntoIterator<Item = &'a str>,
    kind: SymbolKind,
) -> Namespace {
    let symbols = names
        .into_iter()
        .map(|name| (name.to_owned(), compiler_owned_object(path, name, kind)))
        .collect();
    Namespace { symbols }
}

fn compiler_owned_object(path: &str, name: &str, kind: SymbolKind) -> Symbol {
    Symbol {
        identity: format!("{path}::{name}"),
        lowering_identity: None,
        name: name.to_owned(),
        namespace: path.to_owned(),
        visibility: Visibility::Public,
        global: false,
        constant: false,
        kind,
        declaration_span: None,

        binding_span: None,
    }
}

fn normalize_declared_path(components: &[&str]) -> Option<String> {
    if components.is_empty()
        || components
            .iter()
            .any(|component| matches!(*component, "/" | "..") || component.is_empty())
    {
        return None;
    }
    Some(format!("/{}", components.join("/")))
}

fn resolve_path(current: &str, anchored: bool, path: &[&str]) -> Option<String> {
    let mut components = if anchored {
        Vec::new()
    } else {
        current
            .trim_start_matches('/')
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
    };
    for component in path {
        if *component == "/" {
            continue;
        }
        if *component == ".." {
            components.pop()?;
        } else {
            components.push(component);
        }
    }
    Some(format!("/{}", components.join("/")))
}

fn declaration_name(node: &SyntaxNode, source: &SourceFile) -> Option<String> {
    node.children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
        .map(|child| node_text(source, child).to_owned())
}

fn unary_operator_text(unit: &SemanticUnit, node: &SyntaxNode) -> Option<String> {
    node.children
        .iter()
        .find(|child| child.kind == SyntaxKind::UnaryOperator)
        .map(|operator| {
            node_text(&unit.source, operator)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
}

fn transparent_value_type(value_type: Option<ValueType>) -> Option<ValueType> {
    value_type.map(|value_type| match value_type {
        ValueType::Reference(item) | ValueType::SharedReference(item) => item.value_type(),
        value_type => value_type,
    })
}

fn node_text<'a>(source: &'a SourceFile, node: &SyntaxNode) -> &'a str {
    &source.text()[node.span.start..node.span.end]
}

fn failure(
    source: &SourceFile,
    code: &'static str,
    message: impl Into<String>,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![Diagnostic::error(code, message, span)],
    }
}

#[cfg(test)]
mod name_style_tests {
    use super::*;

    #[test]
    fn compiler_owned_declarations_require_kebab_case() {
        let package = Package::implicit(
            "main.trn",
            "namespace app\nfunction NotKebab;\n  return\nfunction main;\n  return\n".to_owned(),
        );
        let mut semantic = analyze(&package).unwrap();
        semantic.units[0].bundled = true;

        let failure = validate_compiler_owned_names(&semantic.units).unwrap_err();
        assert_eq!(failure.diagnostics[0].code, "S2018");
        assert_eq!(
            failure.diagnostics[0].message,
            "compiler-owned declaration `NotKebab` is not kebab-case"
        );
    }

    #[test]
    fn authored_name_style_is_an_opt_in_warning() {
        let package = Package::implicit(
            "main.trn",
            "namespace app\nfunction main;\n  Answer = 42\n  print; Answer\n".to_owned(),
        );
        let semantic = analyze(&package).unwrap();

        assert!(warnings(&semantic, false).is_empty());
        let diagnostics = warnings(&semantic, true);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "S2018"
                && diagnostic.message == "declared name `Answer` is not kebab-case"
                && diagnostic.severity == crate::Severity::Warning
        }));
    }

    #[test]
    fn object_union_arm_identity_follows_import_aliases() {
        let package = Package::implicit(
            "main.trn",
            concat!(
                "namespace app\n",
                "from /core/errors import throwable as first, throwable as second\n",
                "function main;\n",
                "  return\n",
            )
            .to_owned(),
        );
        let semantic = analyze(&package).unwrap();
        let unit = &semantic.units[0];
        let first = unit.source.text().find("first").unwrap();
        let second = unit.source.text().find("second").unwrap();

        assert_eq!(
            union_arm_identity(&semantic, unit, "first", first),
            union_arm_identity(&semantic, unit, "second", second),
        );
    }

    #[test]
    fn arbitrary_object_optional_types_are_semantic_values() {
        let package = Package::implicit(
            "main.trn",
            "namespace app\nclass widget\nfunction maybe widget|none;\n  return none\nfunction main;\n  value widget|none = maybe;\n  return\n".to_owned(),
        );
        let semantic = analyze(&package).unwrap();
        let maybe = semantic.units[0]
            .functions
            .iter()
            .find(|function| function.name == "maybe")
            .unwrap();

        assert!(matches!(
            &maybe.return_type,
            Some(ValueType::Optional(inner))
                if matches!(inner.as_ref(), ValueType::Object(identity) if identity.name == "widget")
        ));
    }
}
