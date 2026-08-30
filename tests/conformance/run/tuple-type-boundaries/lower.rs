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
    fn from_source_name(name: &str) -> Self {
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
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
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
const _: () = assert!(std::mem::size_of::< TerraneError > () == 16);
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
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message());
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
        TerraneError::raised(
            TerraneErrorKind::from_source_name(self.source_name()),
            origin,
        )
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
    reason = "returning fresh failures are absent from some lowered programs"
)]
#[inline]
fn __terrane_raised_err<T, E: TerraneRaised>(
    result: Result<T, E>,
    origin: TerraneSite,
) -> Result<T, TerraneError> {
    result.map_err(|error| error.raised(origin))
}
macro_rules! __terrane_raised_completion {
    ($result:expr, $origin:expr) => {
        match $result { Ok(value) => value, Err(error) => { return
        TerraneCompletion::Error(error.raised($origin)); } }
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
mod __terrane_trace {
    #[allow(
        dead_code,
        reason = "range ends are retained for diagnostics and future provenance consumers"
    )]
    pub struct Site {
        pub function: u32,
        pub file: u32,
        pub line: u32,
        pub column: u32,
        pub end_line: u32,
        pub end_column: u32,
    }
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/tuple-type-boundaries::main"];
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 0] = [];
    pub static SITES: [Site; 12] = [
        Site {
            function: 
                0 /* terrane-site: site 0: /tuple-type-boundaries::main (case.trn:13:27-13:38) */,
            file: 0,
            line: 13,
            column: 27,
            end_line: 13,
            end_column: 38,
        },
        Site {
            function: 
                0 /* terrane-site: site 1: /tuple-type-boundaries::main (case.trn:13:27-13:38) */,
            file: 0,
            line: 13,
            column: 27,
            end_line: 13,
            end_column: 38,
        },
        Site {
            function: 
                0 /* terrane-site: site 2: /tuple-type-boundaries::main (case.trn:13:40-13:51) */,
            file: 0,
            line: 13,
            column: 40,
            end_line: 13,
            end_column: 51,
        },
        Site {
            function: 
                0 /* terrane-site: site 3: /tuple-type-boundaries::main (case.trn:13:40-13:51) */,
            file: 0,
            line: 13,
            column: 40,
            end_line: 13,
            end_column: 51,
        },
        Site {
            function: 
                0 /* terrane-site: site 4: /tuple-type-boundaries::main (case.trn:18:25-18:34) */,
            file: 0,
            line: 18,
            column: 25,
            end_line: 18,
            end_column: 34,
        },
        Site {
            function: 
                0 /* terrane-site: site 5: /tuple-type-boundaries::main (case.trn:18:25-18:34) */,
            file: 0,
            line: 18,
            column: 25,
            end_line: 18,
            end_column: 34,
        },
        Site {
            function: 
                0 /* terrane-site: site 6: /tuple-type-boundaries::main (case.trn:18:25-18:37) */,
            file: 0,
            line: 18,
            column: 25,
            end_line: 18,
            end_column: 37,
        },
        Site {
            function: 
                0 /* terrane-site: site 7: /tuple-type-boundaries::main (case.trn:18:25-18:37) */,
            file: 0,
            line: 18,
            column: 25,
            end_line: 18,
            end_column: 37,
        },
        Site {
            function: 
                0 /* terrane-site: site 8: /tuple-type-boundaries::main (case.trn:18:39-18:48) */,
            file: 0,
            line: 18,
            column: 39,
            end_line: 18,
            end_column: 48,
        },
        Site {
            function: 
                0 /* terrane-site: site 9: /tuple-type-boundaries::main (case.trn:18:39-18:48) */,
            file: 0,
            line: 18,
            column: 39,
            end_line: 18,
            end_column: 48,
        },
        Site {
            function: 
                0 /* terrane-site: site 10: /tuple-type-boundaries::main (case.trn:18:39-18:51) */,
            file: 0,
            line: 18,
            column: 39,
            end_line: 18,
            end_column: 51,
        },
        Site {
            function: 
                0 /* terrane-site: site 11: /tuple-type-boundaries::main (case.trn:18:39-18:51) */,
            file: 0,
            line: 18,
            column: 39,
            end_line: 18,
            end_column: 51,
        },
    ];
    #[cold]
    #[inline(never)]
    pub fn render(site: u32) -> String {
        let site = &SITES[usize::try_from(site).expect("site id must fit usize")];
        format!(
            "{} ({}:{}:{})", FUNCTIONS[usize::try_from(site.function)
            .expect("function id must fit usize")], FILES[usize::try_from(site.file)
            .expect("file id must fit usize")], site.line, site.column,
        )
    }
}
// Source: case.trn
// Namespace: tuple-type-boundaries
fn echo(
    values: terrane_collection_support::Tuple<terrane_int_support::Int>,
) -> terrane_collection_support::Tuple<terrane_int_support::Int> {
    return values.clone();
}
fn nested(
    values: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >,
) -> terrane_collection_support::List<
    terrane_collection_support::Tuple<terrane_int_support::Int>,
> {
    return values.clone();
}
fn main() {
    let pair: terrane_collection_support::Tuple<terrane_int_support::Int> = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let returned: terrane_collection_support::Tuple<terrane_int_support::Int> = echo(
        pair,
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(returned
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:13:27-13:38 */)), 1 /* terrane-site: case.trn:13:27-13:38 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:13:40-13:51 */)), 3 /* terrane-site: case.trn:13:40-13:51 */))
    );
    let empty: terrane_collection_support::Tuple<terrane_int_support::Int> = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(Vec::new());
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty
        .length()))
    );
    let groups: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    > = terrane_collection_support::List::<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Tuple::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(4_i128)]),
            terrane_collection_support::Tuple::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(5_i128)])
        ],
    );
    let echoed: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    > = nested(groups);
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(echoed
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(echoed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:18:25-18:34 */)), 5 /* terrane-site: case.trn:18:25-18:34 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        6 /* terrane-site: case.trn:18:25-18:37 */)), 7 /* terrane-site: case.trn:18:25-18:37 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(__terrane_raised(echoed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        8 /* terrane-site: case.trn:18:39-18:48 */)), 9 /* terrane-site: case.trn:18:39-18:48 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        10 /* terrane-site: case.trn:18:39-18:51 */)), 11 /* terrane-site: case.trn:18:39-18:51 */))
    );
}
