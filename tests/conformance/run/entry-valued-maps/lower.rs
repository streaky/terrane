// Generated deterministically by Terrane <version>.
type TerraneSite = u32;
const TERRANE_NO_SITE: TerraneSite = u32::MAX;
#[allow(dead_code, reason = "custom descriptors are absent from some lowered programs")]
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
}
impl TerraneErrorKind {
    #[allow(
        dead_code,
        reason = "support-error conversions are selected by each lowered program"
    )]
    fn from_support_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
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
const _: () = assert!(std::mem::size_of::< TerraneError > () == 16);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::size_of::< Result < i64, TerraneError >> () == 16);
#[allow(
    dead_code,
    reason = "one canonical runtime implementation serves every lowered error shape"
)]
impl TerraneError {
    #[cold]
    #[inline(never)]
    fn raised(kind: TerraneErrorKind, origin: TerraneSite) -> Self {
        Self { kind, origin, detail: None }
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
            detail: Some(
                Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    cause: None,
                    frames: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn with_cause(mut self, cause: TerraneError) -> Self {
        self
            .detail
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
            error @ (ArithmeticError::InvalidRadix
            | ArithmeticError::InvalidRadixText) => {
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
            self.to_string().trim_start_matches(".decode-error: "),
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
        match $result { Ok(value) => value, Err(error) => { return
        TerraneCompletion::Error(__terrane_fresh_error(error, $origin)); } }
    };
}
#[allow(
    dead_code,
    reason = "terminating propagation is absent from some lowered programs"
)]
#[inline]
fn __terrane_traced<T>(result: Result<T, TerraneError>, frame: TerraneSite) -> T {
    result
        .unwrap_or_else(|error| __terrane_uncaught(__terrane_trace_error(error, frame)))
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
        match $result { Ok(value) => value, Err(error) => { return
        TerraneCompletion::Error(__terrane_trace_error(error, $frame)); } }
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
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 0] = [];
}
mod __terrane_trace {
    pub struct Site {
        pub function: u32,
        pub file: u32,
        pub line: u32,
        pub column: u32,
        pub end_line: u32,
        pub end_column: u32,
    }
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/entry-valued-maps::main"];
    pub static SITES: [Site; 12] = [
        {
            /* terrane-site-row: site 0: /entry-valued-maps::main (case.trn:13:10-13:28) */
            Site {
                function: 0,
                file: 0,
                line: 13,
                column: 10,
                end_line: 13,
                end_column: 28,
            }
        },
        {
            /* terrane-site-row: site 1: /entry-valued-maps::main (case.trn:13:34-13:52) */
            Site {
                function: 0,
                file: 0,
                line: 13,
                column: 34,
                end_line: 13,
                end_column: 52,
            }
        },
        {
            /* terrane-site-row: site 2: /entry-valued-maps::main (case.trn:14:10-14:29) */
            Site {
                function: 0,
                file: 0,
                line: 14,
                column: 10,
                end_line: 14,
                end_column: 29,
            }
        },
        {
            /* terrane-site-row: site 3: /entry-valued-maps::main (case.trn:14:35-14:54) */
            Site {
                function: 0,
                file: 0,
                line: 14,
                column: 35,
                end_line: 14,
                end_column: 54,
            }
        },
        {
            /* terrane-site-row: site 4: /entry-valued-maps::main (case.trn:15:10-15:30) */
            Site {
                function: 0,
                file: 0,
                line: 15,
                column: 10,
                end_line: 15,
                end_column: 30,
            }
        },
        {
            /* terrane-site-row: site 5: /entry-valued-maps::main (case.trn:15:36-15:56) */
            Site {
                function: 0,
                file: 0,
                line: 15,
                column: 36,
                end_line: 15,
                end_column: 56,
            }
        },
        {
            /* terrane-site-row: site 6: /entry-valued-maps::main (case.trn:16:10-16:31) */
            Site {
                function: 0,
                file: 0,
                line: 16,
                column: 10,
                end_line: 16,
                end_column: 31,
            }
        },
        {
            /* terrane-site-row: site 7: /entry-valued-maps::main (case.trn:16:37-16:58) */
            Site {
                function: 0,
                file: 0,
                line: 16,
                column: 37,
                end_line: 16,
                end_column: 58,
            }
        },
        {
            /* terrane-site-row: site 8: /entry-valued-maps::main (case.trn:17:10-17:31) */
            Site {
                function: 0,
                file: 0,
                line: 17,
                column: 10,
                end_line: 17,
                end_column: 31,
            }
        },
        {
            /* terrane-site-row: site 9: /entry-valued-maps::main (case.trn:17:37-17:58) */
            Site {
                function: 0,
                file: 0,
                line: 17,
                column: 37,
                end_line: 17,
                end_column: 58,
            }
        },
        {
            /* terrane-site-row: site 10: /entry-valued-maps::main (case.trn:18:10-18:33) */
            Site {
                function: 0,
                file: 0,
                line: 18,
                column: 10,
                end_line: 18,
                end_column: 33,
            }
        },
        {
            /* terrane-site-row: site 11: /entry-valued-maps::main (case.trn:18:39-18:62) */
            Site {
                function: 0,
                file: 0,
                line: 18,
                column: 39,
                end_line: 18,
                end_column: 62,
            }
        },
    ];
    #[cold]
    #[inline(never)]
    pub fn render(site: u32) -> String {
        let site = &SITES[usize::try_from(site).expect("site id must fit usize")];
        format!(
            "{} ({}:{}:{}-{}:{})", FUNCTIONS[usize::try_from(site.function)
            .expect("function id must fit usize")], FILES[usize::try_from(site.file)
            .expect("file id must fit usize")], site.line, site.column, site.end_line,
            site.end_column,
        )
    }
}
// Source: case.trn
// Namespace: entry-valued-maps
fn main() {
    let inner: terrane_collection_support::Entry<String, i8> = terrane_collection_support::Entry::<
        String,
        i8,
    >::new(String::from("b"), 7);
    let ordered_bound: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, i8>,
    > = terrane_collection_support::Map::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![terrane_collection_support::Entry::new(String::from("a"), inner.clone())],
    );
    let ordered_inline: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, i8>,
    > = terrane_collection_support::Map::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, i8 >::new(String::from("c"), 8))
        ],
    );
    let unordered_bound: terrane_collection_support::UnorderedMap<
        String,
        terrane_collection_support::Entry<String, i8>,
    > = terrane_collection_support::UnorderedMap::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![terrane_collection_support::Entry::new(String::from("a"), inner.clone())],
    );
    let unordered_inline: terrane_collection_support::UnorderedMap<
        String,
        terrane_collection_support::Entry<String, i8>,
    > = terrane_collection_support::UnorderedMap::<
        String,
        terrane_collection_support::Entry<String, i8>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, i8 >::new(String::from("d"), 9))
        ],
    );
    let inferred_ordered: terrane_collection_support::Map<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    > = terrane_collection_support::Map::<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, terrane_int_support::Int
            >::new(String::from("e"), terrane_int_support::Int::from(6_i128)))
        ],
    );
    let inferred_unordered: terrane_collection_support::UnorderedMap<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    > = terrane_collection_support::UnorderedMap::<
        String,
        terrane_collection_support::Entry<String, terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_collection_support::Entry::< String, terrane_int_support::Int
            >::new(String::from("f"), terrane_int_support::Int::from(5_i128)))
        ],
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered_bound
        .get_or_error(&String::from("a")), 0 /* terrane-site: case.trn:13:10-13:28 */)
        .key), terrane_scalar_support::scalar_text(&__terrane_raised(ordered_bound
        .get_or_error(&String::from("a")), 1 /* terrane-site: case.trn:13:34-13:52 */)
        .value)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered_inline
        .get_or_error(&String::from("a")), 2 /* terrane-site: case.trn:14:10-14:29 */)
        .key), terrane_scalar_support::scalar_text(&__terrane_raised(ordered_inline
        .get_or_error(&String::from("a")), 3 /* terrane-site: case.trn:14:35-14:54 */)
        .value)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(unordered_bound
        .get_or_error(&String::from("a")), 4 /* terrane-site: case.trn:15:10-15:30 */)
        .key), terrane_scalar_support::scalar_text(&__terrane_raised(unordered_bound
        .get_or_error(&String::from("a")), 5 /* terrane-site: case.trn:15:36-15:56 */)
        .value)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(unordered_inline
        .get_or_error(&String::from("a")), 6 /* terrane-site: case.trn:16:10-16:31 */)
        .key), terrane_scalar_support::scalar_text(&__terrane_raised(unordered_inline
        .get_or_error(&String::from("a")), 7 /* terrane-site: case.trn:16:37-16:58 */)
        .value)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(inferred_ordered
        .get_or_error(&String::from("a")), 8 /* terrane-site: case.trn:17:10-17:31 */)
        .key), terrane_scalar_support::scalar_text(&__terrane_raised(inferred_ordered
        .get_or_error(&String::from("a")), 9 /* terrane-site: case.trn:17:37-17:58 */)
        .value)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(inferred_unordered
        .get_or_error(&String::from("a")), 10 /* terrane-site: case.trn:18:10-18:33 */).key),
        terrane_scalar_support::scalar_text(&__terrane_raised(inferred_unordered
        .get_or_error(&String::from("a")), 11 /* terrane-site: case.trn:18:39-18:62 */).value)
    );
}
