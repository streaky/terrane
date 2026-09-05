use super::prelude::*;
pub(super) fn descriptor_runtime_module() -> GeneratedModule {
    GeneratedModule {
        name: "reflection",
        items: vec![Item::generated(
            "#[allow(dead_code)]\n\
             #[derive(Clone, Copy)]\n\
             struct TerraneDescriptor { identity: &'static str, name: &'static str, kind: &'static str }\n",
        )],
    }
}

pub(super) fn package_uses_task_scope(package: &SemanticPackage) -> bool {
    fn contains(package: &SemanticPackage, unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
            && package
                .resolve_name_at(
                    unit,
                    callee.span.start,
                    &unit.source.text()[callee.span.start..callee.span.end],
                )
                .is_some_and(|symbol| symbol.identity == "/core/async::task-scope")
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| contains(package, unit, child))
    }

    package
        .units
        .iter()
        .any(|unit| contains(package, unit, &unit.tree.root))
}

pub(super) fn package_uses_structured_errors(package: &SemanticPackage) -> bool {
    fn contains(package: &SemanticPackage, unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        matches!(
            node.kind,
            SyntaxKind::ThrowStatement
                | SyntaxKind::TryStatement
                | SyntaxKind::IndexExpression
                | SyntaxKind::FunctionType
                | SyntaxKind::AnonymousFunction
        ) || string_call_selection(&unit.source, node)
            .is_some_and(|selection| selection.family == StringFamily::Decode)
            || node
                .children
                .iter()
                .any(|child| contains(package, unit, child))
            || (node.kind == SyntaxKind::CallExpression
                && node.children.first().is_some_and(|callee| {
                    let range = if callee.kind == SyntaxKind::MemberExpression {
                        callee.children.first()
                    } else {
                        Some(callee)
                    };
                    range.is_some_and(|range| {
                        package
                            .resolve_name_at(
                                unit,
                                range.span.start,
                                &unit.source.text()[range.span.start..range.span.end],
                            )
                            .is_some_and(|symbol| symbol.identity == "/core/collections::range")
                    })
                }))
            || (node.kind == SyntaxKind::CallExpression
                && node.children.first().is_some_and(|callee| {
                    callee.kind == SyntaxKind::MemberExpression
                        && callee.children.get(1).is_some_and(|member| {
                            &unit.source.text()[member.span.start..member.span.end] == "set"
                        })
                }))
    }
    package.units.iter().any(|unit| {
        unit.functions.iter().any(|contract| contract.throws)
            || contains(package, unit, &unit.tree.root)
    }) || package.projection.dependencies.iter().any(|dependency| {
        dependency.items.iter().any(|item| match &item.kind {
            crate::projection::ProjectedKind::Function(_) => true,
            crate::projection::ProjectedKind::ForeignType { methods } => !methods.is_empty(),
            crate::projection::ProjectedKind::Enum { .. } => false,
        })
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "the generated error runtime remains directly reviewable as one canonical Rust template"
)]
pub(super) fn emit_error_support(
    output: &mut String,
    has_custom_throwable: bool,
    has_dependency: bool,
    registry: &LoweringRegistry,
) {
    output.push_str(indoc! {r#"
        type TerraneSite = u32;
        const TERRANE_NO_SITE: TerraneSite = u32::MAX;
        #[allow(
            dead_code,
            reason = "custom descriptors are absent from some lowered programs"
        )]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct DescriptorId(u16);
        #[allow(
            dead_code,
            reason = "one canonical runtime enum covers every compiler-owned throwable kind"
        )]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(u16)]
        enum TerraneErrorKind {
            ArithmeticOverflow,
            DivisionByZero,
            IntegerConversionOverflow,
            NegativeShiftCount,
            CoercionError,
            DecodeError,
            IndexError,
            MissingKey,
            ResourceError,
            SourceError,
    "#});
    if has_custom_throwable {
        output.push_str("    Custom(DescriptorId),\n");
    }
    output.push_str(indoc! {r#"
        }
        impl TerraneErrorKind {
            fn display_name(self) -> &'static str {
                match self {
                    Self::ArithmeticOverflow => "arithmetic-overflow",
                    Self::DivisionByZero => "division-by-zero",
                    Self::IntegerConversionOverflow => "integer-conversion-overflow",
                    Self::NegativeShiftCount => "negative-shift-count",
                    Self::CoercionError => "coercion-error",
                    Self::DecodeError => "decode-error",
                    Self::IndexError => "index-error",
                    Self::MissingKey => "missing-key",
                    Self::ResourceError => "resource-error",
                    Self::SourceError => "error",
    "#});
    if has_custom_throwable {
        output.push_str(
            "                Self::Custom(descriptor) => __terrane_error_registry::DESCRIPTORS[usize::from(descriptor.0)],\n",
        );
    }
    output.push_str(indoc! {r#"
                }
            }
            fn default_message(self) -> &'static str {
                match self {
                    Self::ArithmeticOverflow => "fixed-width integer arithmetic overflow",
                    Self::DivisionByZero => "integer division by zero",
                    Self::IntegerConversionOverflow => "integer conversion overflow",
                    Self::NegativeShiftCount => "negative integer shift count",
                    Self::CoercionError => "coercion has no compatible result",
                    Self::DecodeError => "invalid byte sequence for selected encoding",
                    Self::IndexError => "collection index is out of range",
                    Self::MissingKey => "collection key is absent",
                    Self::ResourceError => {
                        "integer shift count cannot be represented on this target"
                    }
                    Self::SourceError => "source error",
    "#});
    if has_custom_throwable {
        output.push_str("            Self::Custom(_) => \"source error\",\n");
    }
    output.push_str(indoc! {r#"
                }
            }
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct TerraneErrorDetail {
            message: Option<String>,
            cause: Option<Box<TerraneError>>,
            frames: Vec<TerraneSite>,
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct TerraneError {
            kind: TerraneErrorKind,
            origin: TerraneSite,
            detail: Option<Box<TerraneErrorDetail>>,
        }
        #[cfg(target_pointer_width = "64")]
        const _: () = assert!(std::mem::size_of::<TerraneError>() == 16);
        #[cfg(target_pointer_width = "64")]
        const _: () = assert!(std::mem::size_of::<Result<i64, TerraneError>>() == 16);
        #[allow(
            dead_code,
            reason = "one canonical runtime implementation serves every lowered error shape"
        )]
        impl TerraneError {
            #[cold]
            #[inline(never)]
            fn raised(kind: TerraneErrorKind, origin: TerraneSite) -> Self {
                Self {
                    kind,
                    origin,
                    detail: None,
                }
            }
            #[cold]
            #[inline(never)]
            fn raised_with_message(
                kind: TerraneErrorKind,
                message: impl Into<String>,
                origin: TerraneSite,
            ) -> Self {
                Self {
                    kind,
                    origin,
                    detail: Some(Box::new(TerraneErrorDetail {
                        message: Some(message.into()),
                        cause: None,
                        frames: Vec::new(),
                    })),
                }
            }
    "#});
    if has_custom_throwable {
        output.push_str(indoc! {r"
            #[cold]
            #[inline(never)]
            fn custom_raised(
                descriptor: DescriptorId,
                message: impl Into<String>,
                origin: TerraneSite,
            ) -> Self {
                Self::raised_with_message(TerraneErrorKind::Custom(descriptor), message, origin)
            }
        "});
    }
    output.push_str(indoc! {r#"
            #[cold]
            #[inline(never)]
            fn with_cause(mut self, cause: TerraneError) -> Self {
                self.detail
                    .get_or_insert_with(|| {
                        Box::new(TerraneErrorDetail {
                            message: None,
                            cause: None,
                            frames: Vec::new(),
                        })
                    })
                    .cause = Some(Box::new(cause));
                self
            }
            #[cold]
            #[inline(never)]
            fn attributed(mut self, origin: TerraneSite) -> Self {
                debug_assert_eq!(self.origin, TERRANE_NO_SITE);
                self.origin = origin;
                self
            }
            #[cold]
            #[inline(never)]
            fn at(mut self, frame: TerraneSite) -> Self {
                self.detail
                    .get_or_insert_with(|| {
                        Box::new(TerraneErrorDetail {
                            message: None,
                            cause: None,
                            frames: Vec::new(),
                        })
                    })
                    .frames
                    .push(frame);
                self
            }
            fn message(&self) -> &str {
                self.detail
                    .as_ref()
                    .and_then(|detail| detail.message.as_deref())
                    .unwrap_or_else(|| self.kind.default_message())
            }
            #[cold]
            #[inline(never)]
            fn render(&self) -> String {
                let mut rendered = format!("{}: {}", self.kind.display_name(), self.message());
                if let Some(cause) = self
                    .detail
                    .as_ref()
                    .and_then(|detail| detail.cause.as_ref())
                {
                    rendered.push_str("\ncaused by: ");
                    rendered.push_str(&cause.render());
                }
                if self.origin != TERRANE_NO_SITE {
                    rendered.push_str("\nat ");
                    rendered.push_str(&__terrane_trace::render(self.origin));
                }
                if let Some(detail) = &self.detail {
                    for frame in &detail.frames {
                        rendered.push_str("\nat ");
                        rendered.push_str(&__terrane_trace::render(*frame));
                    }
                }
                rendered
            }
        }
        impl std::fmt::Display for TerraneError {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&self.render())
            }
        }
        #[allow(
            dead_code,
            reason = "fresh support failures are absent from some lowered programs"
        )]
        trait TerraneRaised {
            fn raised(self, origin: TerraneSite) -> TerraneError;
        }
        pub struct TerraneForeignError(TerraneError);
        impl TerraneForeignError {
            pub fn render(&self) -> String {
                self.0.render()
            }
        }
        impl TerraneRaised for TerraneForeignError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                self.0.attributed(origin)
            }
        }
        impl TerraneRaised for terrane_int_support::ArithmeticError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                use terrane_int_support::ArithmeticError;
                match self {
                    ArithmeticError::DivisionByZero => {
                        TerraneError::raised(TerraneErrorKind::DivisionByZero, origin)
                    }
                    ArithmeticError::ArithmeticOverflow => {
                        TerraneError::raised(TerraneErrorKind::ArithmeticOverflow, origin)
                    }
                    ArithmeticError::NegativeShiftCount => {
                        TerraneError::raised(TerraneErrorKind::NegativeShiftCount, origin)
                    }
                    ArithmeticError::ShiftCountTooLarge => {
                        TerraneError::raised(TerraneErrorKind::ResourceError, origin)
                    }
                    error @ (ArithmeticError::IntegerConversionOverflow
                    | ArithmeticError::IntegerConversionOverflowDetail { .. }) => {
                        TerraneError::raised_with_message(
                            TerraneErrorKind::IntegerConversionOverflow,
                            error.to_string(),
                            origin,
                        )
                    }
                    error @ (ArithmeticError::InvalidRadix | ArithmeticError::InvalidRadixText) => {
                        TerraneError::raised_with_message(
                            TerraneErrorKind::CoercionError,
                            error.to_string(),
                            origin,
                        )
                    }
                }
            }
        }
        impl TerraneRaised for terrane_string_support::DecodeError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::DecodeError,
                    self.to_string(),
                    origin,
                )
            }
        }
        impl TerraneRaised for terrane_collection_support::IndexError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::IndexError,
                    self.to_string(),
                    origin,
                )
            }
        }
        impl TerraneRaised for terrane_collection_support::MissingKey {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::MissingKey,
                    self.to_string(),
                    origin,
                )
            }
        }
        impl TerraneRaised for terrane_collection_support::RangeStepError {
            fn raised(self, origin: TerraneSite) -> TerraneError {
                TerraneError::raised_with_message(
                    TerraneErrorKind::SourceError,
                    self.to_string(),
                    origin,
                )
            }
        }
        #[allow(
            dead_code,
            reason = "terminating fresh failures are absent from some lowered programs"
        )]
        #[cold]
        #[inline(never)]
        fn __terrane_raise<E: TerraneRaised>(error: E, origin: TerraneSite) -> ! {
            __terrane_uncaught(error.raised(origin))
        }
        #[allow(
            dead_code,
            reason = "propagating failures are absent from some lowered programs"
        )]
        #[cold]
        #[inline(never)]
        fn __terrane_trace_error(error: TerraneError, frame: TerraneSite) -> TerraneError {
            error.at(frame)
        }
        #[allow(
            dead_code,
            reason = "terminating fresh failures are absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_raised<T, E: TerraneRaised>(
            result: Result<T, E>,
            origin: TerraneSite,
        ) -> T {
            result.unwrap_or_else(|error| __terrane_raise(error, origin))
        }
        #[allow(
            dead_code,
            reason = "fresh failure propagation is absent from some lowered programs"
        )]
        #[cold]
        #[inline(never)]
        fn __terrane_fresh_error<E: TerraneRaised>(
            error: E,
            origin: TerraneSite,
        ) -> TerraneError {
            error.raised(origin)
        }
        #[allow(
            dead_code,
            reason = "returning fresh failures are absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_raised_err<T, E: TerraneRaised>(
            result: Result<T, E>,
            origin: TerraneSite,
        ) -> Result<T, TerraneError> {
            result.map_err(|error| __terrane_fresh_error(error, origin))
        }
        macro_rules! __terrane_raised_completion {
            ($result:expr, $origin:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(error) => {
                        return TerraneCompletion::Error(__terrane_fresh_error(error, $origin));
                    }
                }
            };
        }
        #[allow(
            dead_code,
            reason = "terminating propagation is absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_traced<T>(
            result: Result<T, TerraneError>,
            frame: TerraneSite,
        ) -> T {
            result.unwrap_or_else(|error| __terrane_uncaught(__terrane_trace_error(error, frame)))
        }
        #[allow(
            dead_code,
            reason = "returning propagation is absent from some lowered programs"
        )]
        #[inline]
        fn __terrane_traced_err<T>(
            result: Result<T, TerraneError>,
            frame: TerraneSite,
        ) -> Result<T, TerraneError> {
            result.map_err(|error| __terrane_trace_error(error, frame))
        }
        macro_rules! __terrane_traced_completion {
            ($result:expr, $frame:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(error) => {
                        return TerraneCompletion::Error(__terrane_trace_error(error, $frame));
                    }
                }
            };
        }
        fn __terrane_uncaught(error: TerraneError) -> ! {
            eprintln!("{}", error.render());
            std::process::exit(1);
        }
        fn __terrane_generated_defect(message: &str) -> ! {
            eprintln!(
                "internal compiler defect: generated program reached an impossible completion: {message}"
            );
            std::process::exit(5);
        }
        #[allow(dead_code)]
        enum TerraneCompletion<T> {
            Normal,
            Return(T),
            Error(TerraneError),
            Break,
            Continue,
        }
    "#});
    if has_dependency {
        let error_descriptor =
            registry.register_descriptor("/core/errors::dependency-error", "dependency-error");
        let panic_descriptor =
            registry.register_descriptor("/core/errors::dependency-panic", "dependency-panic");
        writeln!(
            output,
            "#[allow(dead_code, reason = \"a projected dependency may expose no Result members\")]\nconst TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId({error_descriptor});\n#[allow(dead_code, reason = \"panic catching may be disabled or not crossed\")]\nconst TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId({panic_descriptor});"
        )
        .expect("writing to a String cannot fail");
        output.push_str(indoc! {r#"
            #[allow(
                dead_code,
                reason = "projected type methods may be imported without being crossed"
            )]
            fn __terrane_dependency_panic(
                payload: Box<dyn std::any::Any + Send>,
                crate_name: &'static str,
                member: &'static str,
            ) -> TerraneForeignError {
                let detail = payload
                    .downcast_ref::<&str>()
                    .copied()
                    .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
                    .unwrap_or("non-string panic payload");
                TerraneForeignError(TerraneError::custom_raised(
                    TERRANE_DEPENDENCY_PANIC,
                    format!("Rust dependency `{crate_name}` member `{member}` panicked: {detail}"),
                    TERRANE_NO_SITE,
                ))
            }
        "#});
    }
    emit_site_tables(output, registry);
}

pub(super) fn emit_site_tables(output: &mut String, registry: &LoweringRegistry) {
    let files = registry.files.borrow();
    let functions = registry.functions.borrow();
    let sites = registry.sites.borrow();
    let descriptors = registry.descriptors.borrow();
    output.push_str(indoc! {r#"
        mod __terrane_error_registry {
            #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    "#});
    writeln!(
        output,
        "    pub static DESCRIPTORS: [&str; {}] = [",
        descriptors.len()
    )
    .expect("writing to a String cannot fail");
    for (_, name) in descriptors.iter() {
        writeln!(output, "        {name:?},").expect("writing to a String cannot fail");
    }
    output.push_str("    ];\n}\n");
    output.push_str(indoc! {r"
        mod __terrane_trace {
            pub struct Site {
                pub function: u32,
                pub file: u32,
                pub line: u32,
                pub column: u32,
                pub end_line: u32,
                pub end_column: u32,
            }
    "});
    writeln!(output, "    pub static FILES: [&str; {}] = [", files.len())
        .expect("writing to a String cannot fail");
    for file in files.iter() {
        writeln!(output, "        {file:?},").expect("writing to a String cannot fail");
    }
    output.push_str("    ];\n");
    writeln!(
        output,
        "    pub static FUNCTIONS: [&str; {}] = [",
        functions.len()
    )
    .expect("writing to a String cannot fail");
    for function in functions.iter() {
        writeln!(output, "        {function:?},").expect("writing to a String cannot fail");
    }
    output.push_str("    ];\n");
    writeln!(output, "    pub static SITES: [Site; {}] = [", sites.len())
        .expect("writing to a String cannot fail");
    for (id, site) in sites.iter().enumerate() {
        let function = &functions[usize::try_from(site.function).expect("u32 must fit usize")];
        let file = &files[usize::try_from(site.file).expect("u32 must fit usize")];
        writeln!(
            output,
            "        {{\n            __terrane_site_comment!({:?});\n            Site {{ function: {}, file: {}, line: {}, column: {}, end_line: {}, end_column: {} }}\n        }},",
            format!("site {id}: {function} ({file}:{}:{}-{}:{})", site.line, site.column, site.end_line, site.end_column),
            site.function,
            site.file,
            site.line,
            site.column,
            site.end_line,
            site.end_column,
        )
        .expect("writing to a String cannot fail");
    }
    output.push_str(indoc! {r#"
            ];
            #[cold]
            #[inline(never)]
            pub fn render(site: u32) -> String {
                let site = &SITES[usize::try_from(site).expect("site id must fit usize")];
                format!(
                    "{} ({}:{}:{}-{}:{})",
                    FUNCTIONS[usize::try_from(site.function).expect("function id must fit usize")],
                    FILES[usize::try_from(site.file).expect("file id must fit usize")],
                    site.line,
                    site.column,
                    site.end_line,
                    site.end_column,
                )
            }
        }
    "#});
}

pub(super) fn emit_global_storage(
    package: &SemanticPackage,
    registry: &LoweringRegistry,
    output: &mut String,
) {
    for (name, symbol) in &package.globals {
        if symbol.kind != SymbolKind::Binding {
            continue;
        }
        let Some(span) = symbol.declaration_span else {
            continue;
        };
        let Some(unit) = package
            .units
            .iter()
            .find(|unit| unit.source.id() == span.file)
        else {
            continue;
        };
        let Some(node) = find_node_by_span(&unit.tree.root, span) else {
            continue;
        };
        let Some(name_node) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
        else {
            continue;
        };
        let emitter = Emitter::new(registry, package, unit);
        let value_type = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span)
            .map(|binding| binding.value_type.clone())
            .or_else(|| emitter.value_type(name_node));
        let Some(ValueType::Scalar(scalar)) = value_type else {
            continue;
        };
        let initial = package.units.iter().rev().find_map(|candidate_unit| {
            candidate_unit
                .tree
                .root
                .children
                .iter()
                .rev()
                .find_map(|candidate| {
                    let global = candidate.children.iter().any(|child| {
                        child.kind == SyntaxKind::DeclarationQualifier
                            && candidate_unit.source.text()[child.span.start..child.span.end].trim()
                                == "global"
                    });
                    let candidate_name = candidate
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)?;
                    (global
                        && &candidate_unit.source.text()
                            [candidate_name.span.start..candidate_name.span.end]
                            == name.as_str())
                    .then_some((candidate_unit, candidate, candidate_name))
                })
        });
        let initial = initial.and_then(|(initial_unit, initial_node, initial_name)| {
            let name_index = initial_node
                .children
                .iter()
                .position(|child| child.span == initial_name.span)?;
            let initializer = binding_initializer(initial_node, name_index)?;
            let mut initial_emitter = Emitter::new(registry, package, initial_unit);
            Some(initial_emitter.expression_as(initializer, ValueType::Scalar(scalar)))
        });
        let initial = initial.map_or_else(|| "None".to_owned(), |value| format!("Some({value})"));
        writeln!(
            output,
            "static {}: std::sync::LazyLock<std::sync::Mutex<Option<{}>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new({initial}));",
            global_binding_name(name),
            rust_type(scalar)
        )
        .unwrap();
    }
    if package
        .globals
        .values()
        .any(|symbol| symbol.kind == SymbolKind::Binding)
    {
        output.push_str(
            "fn __terrane_uninitialized_global(name: &str, path: &str, line: usize, column: usize) -> ! {\n    eprintln!(\"{path}:{line}:{column}: error[T0007]: `{name}` may be read before it is assigned\");\n    std::process::exit(1);\n}\n",
        );
    }
}
