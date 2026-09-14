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
    Binding,
    FieldMetadata,
    FieldMetadataEntry,
    FunctionDeclaration,
    AnonymousFunction,
    ClassDeclaration,
    InterfaceDeclaration,
    TraitDeclaration,
    ExtendsClause,
    ImplementsClause,
    UsesClause,
    ParameterList,
    Parameter,
    Block,
    IfStatement,
    ElseClause,
    WhileStatement,
    ForStatement,
    SelectStatement,
    SelectCase,
    ForTarget,
    ReturnStatement,
    ThrowStatement,
    TryStatement,
    CatchClause,
    CatchBinding,
    FinallyClause,
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
            )
        {
            return "name";
        }
        match (self, child, index) {
            (Self::Binding, Self::TypeExpression, _) => "type",
            (Self::Binding, _, _)
            | (Self::Assignment, _, 1)
            | (Self::ReturnStatement | Self::ThrowStatement, _, 0) => "value",
            (Self::FunctionDeclaration, Self::TypeExpression, _) => "return-type",
            (Self::FunctionDeclaration, Self::EffectClause, _) => "effects",
            (Self::FunctionDeclaration, Self::DeclarationQualifier, _) => "qualifier",
            (Self::Assignment, _, 0) => "target",
            (Self::CallExpression, _, 0) => "callee",
            (Self::CallExpression, _, 1) => "arguments",
            (Self::MemberExpression | Self::StaticMemberExpression, _, 0) => "receiver",
            (Self::MemberExpression | Self::StaticMemberExpression, _, 1) => "member",
            (Self::IfStatement | Self::WhileStatement, _, 0) => "condition",
            (
                Self::FunctionDeclaration | Self::IfStatement | Self::WhileStatement,
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
        }
    }
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
        for child in &node.children {
            Self::write_node(child, depth + 1, output);
        }
    }
}
