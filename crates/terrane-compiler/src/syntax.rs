use crate::{Span, tokens::LexedSource};

/// Complete compiler-owned inventory of Terrane language keywords.
///
/// The list is sorted so keyword checks do not require allocation.
pub const KEYWORDS: &[&str] = &[
    "a",
    "and",
    "as",
    "async",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "constant",
    "construct",
    "continue",
    "destruct",
    "else",
    "extends",
    "false",
    "finally",
    "for",
    "from",
    "function",
    "global",
    "goto",
    "if",
    "implements",
    "import",
    "in",
    "instance",
    "interface",
    "is",
    "label",
    "match",
    "move",
    "namespace",
    "not",
    "of",
    "or",
    "private",
    "protected",
    "public",
    "ref",
    "return",
    "rust",
    "select",
    "self",
    "shared",
    "static",
    "this",
    "throw",
    "throws",
    "to",
    "trait",
    "true",
    "try",
    "unsafe",
    "use",
    "uses",
    "when",
    "while",
    "yield",
];

/// Returns whether `text` is reserved by Terrane syntax.
#[must_use]
pub fn is_keyword(text: &str) -> bool {
    KEYWORDS.binary_search(&text).is_ok()
}

/// Complete set of names forbidden in ordinary declaration-name slots.
pub const RESERVED_DECLARATION_NAMES: &[&str] = &["function", "instance", "self", "this"];

/// Returns whether `text` is forbidden in an ordinary declaration-name slot.
#[must_use]
pub fn is_reserved_declaration_name(text: &str) -> bool {
    RESERVED_DECLARATION_NAMES.binary_search(&text).is_ok()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyntaxKind {
    CompilationUnit,
    NamespaceDeclaration,
    ImportDeclaration,
    ImportSelection,
    NamespacePath,
    NamespaceAnchor,
    ObjectImport,
    ImportAlias,
    Visibility,
    DeclarationQualifier,
    EffectClause,
    AnnotationApplication,
    Binding,
    FieldMetadata,
    FieldMetadataEntry,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    TraitDeclaration,
    EnumDeclaration,
    EnumVariant,
    TypeParameterList,
    TypeParameter,
    ExtendsClause,
    ImplementsClause,
    UsesClause,
    ParameterList,
    Parameter,
    VariadicMarker,
    Block,
    AnonymousFunction,
    ReturnStatement,
    ThrowStatement,
    TryStatement,
    CatchClause,
    CatchBinding,
    FinallyClause,
    IfStatement,
    ElseClause,
    WhileStatement,
    ForStatement,
    ForTarget,
    SelectStatement,
    SelectCase,
    MatchStatement,
    MatchCase,
    MatchCatchAll,
    BreakStatement,
    ContinueStatement,
    Assignment,
    BinaryExpression,
    TypeMembershipExpression,
    UnaryExpression,
    UnaryOperator,
    PostfixExpression,
    MemberExpression,
    StaticMemberExpression,
    ConstructionExpression,
    IndexExpression,
    CallExpression,
    ArgumentList,
    Argument,
    GroupExpression,
    Name,
    Literal,
    TypeExpression,
    UnionType,
    PrefixType,
    AppliedType,
    FunctionType,
    Error,
    RustBlock,
    UnsafeRustBlock,
    Unsupported,
}

impl SyntaxKind {
    /// Stable child-role label used by compiler-owned syntax projections.
    #[must_use]
    pub fn child_field(self, index: usize, child: Self) -> &'static str {
        if child == Self::Name
            && matches!(
                self,
                Self::Binding
                    | Self::FunctionDeclaration
                    | Self::ClassDeclaration
                    | Self::InterfaceDeclaration
                    | Self::TraitDeclaration
                    | Self::EnumDeclaration
                    | Self::EnumVariant
                    | Self::TypeParameter
            )
        {
            return match self {
                Self::TypeParameter => "parameter-name",
                Self::EnumVariant => "variant-name",
                _ => "name",
            };
        }
        match (self, child, index) {
            (_, Self::AnnotationApplication, _) => "annotation",
            (Self::AnnotationApplication, Self::Name, _) => "annotation-type",
            (Self::AnnotationApplication, Self::ArgumentList, _) => "arguments",
            (Self::TypeParameterList, Self::TypeParameter, _) => "parameter",
            (Self::TypeParameter, Self::TypeExpression, _) => "bound",
            (Self::EnumDeclaration, Self::Block, _) => "variants",
            (Self::EnumVariant, Self::ParameterList, _) => "payload",
            (Self::MatchStatement, _, 0) => "scrutinee",
            (Self::MatchStatement, Self::MatchCase, _) => "case",
            (
                Self::MatchCase,
                Self::StaticMemberExpression | Self::Name | Self::MatchCatchAll,
                0,
            ) => "variant",
            (Self::MatchCase, Self::ParameterList, _) => "bindings",
            (Self::Binding, Self::TypeExpression, _) => "type",
            (Self::Binding, _, _)
            | (Self::Assignment, _, 1)
            | (Self::ReturnStatement | Self::ThrowStatement, _, 0) => "value",
            (Self::FunctionDeclaration, Self::TypeExpression, _) => "return-type",
            (
                Self::FunctionDeclaration
                | Self::ClassDeclaration
                | Self::InterfaceDeclaration
                | Self::EnumDeclaration,
                Self::TypeParameterList,
                _,
            ) => "type-parameters",
            (Self::FunctionDeclaration, Self::EffectClause, _) => "effects",
            (Self::FunctionDeclaration, Self::DeclarationQualifier, _) => "qualifier",
            (Self::Assignment, _, 0) => "target",
            (Self::CallExpression, _, 0) => "callee",
            (Self::CallExpression, _, 1) => "arguments",
            (Self::MemberExpression | Self::StaticMemberExpression, _, 0) => "receiver",
            (Self::MemberExpression | Self::StaticMemberExpression, _, 1) => "member",
            (Self::IfStatement | Self::WhileStatement, _, 0) => "condition",
            (
                Self::FunctionDeclaration
                | Self::IfStatement
                | Self::WhileStatement
                | Self::MatchCase,
                Self::Block,
                _,
            ) => "body",
            (Self::CompilationUnit | Self::Block, _, _) => "item",
            _ => "child",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub span: Span,
    pub token_range: std::ops::Range<usize>,
    pub children: Vec<SyntaxNode>,
    /// Metadata syntax is separate from executable declaration children.
    pub annotation_applications: Vec<SyntaxNode>,
    pub documentation: Option<String>,
    pub(crate) is_unsafe_call: bool,
}

impl SyntaxNode {
    pub(crate) fn new(
        kind: SyntaxKind,
        span: Span,
        token_range: std::ops::Range<usize>,
        children: Vec<Self>,
    ) -> Self {
        Self {
            kind,
            span,
            token_range,
            children,
            annotation_applications: Vec::new(),
            documentation: None,
            is_unsafe_call: false,
        }
    }

    #[must_use]
    pub fn annotations(&self) -> impl Iterator<Item = &SyntaxNode> {
        self.annotation_applications.iter()
    }

    /// All syntax children, including non-executable metadata applications.
    pub fn syntax_children(&self) -> impl Iterator<Item = &SyntaxNode> + Clone {
        self.children
            .iter()
            .chain(self.annotation_applications.iter())
    }

    #[must_use]
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }
}

/// Returns whether the parser marked this node as an unsafe call or its callee.
#[must_use]
pub(crate) fn call_is_unsafe(node: &SyntaxNode) -> bool {
    node.is_unsafe_call
}

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub lexed: LexedSource,
    pub root: SyntaxNode,
}

impl SyntaxTree {
    /// Produces a deterministic structural, token, trivia, and byte-span representation for parser goldens.
    #[must_use]
    pub fn normalized(&self) -> String {
        use std::fmt::Write as _;
        let mut output = String::new();
        Self::write_node(&self.root, 0, &mut output);
        output.push_str("tokens\n");
        for token in &self.lexed.tokens {
            let _ = writeln!(
                output,
                "  {:?} {}..{} {:?}",
                token.kind, token.span.start, token.span.end, token.text
            );
        }
        output.push_str("trivia\n");
        for trivia in &self.lexed.trivia {
            let _ = writeln!(
                output,
                "  {:?} {}..{} {:?}",
                trivia.kind, trivia.span.start, trivia.span.end, trivia.text
            );
        }
        output
    }

    fn write_node(node: &SyntaxNode, depth: usize, output: &mut String) {
        use std::fmt::Write as _;
        let _ = writeln!(
            output,
            "{}{:?} {}..{}",
            "  ".repeat(depth),
            node.kind,
            node.span.start,
            node.span.end
        );
        for child in node.syntax_children() {
            Self::write_node(child, depth + 1, output);
        }
    }
}
