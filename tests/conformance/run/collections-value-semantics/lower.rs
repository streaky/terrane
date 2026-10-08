// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
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
    cause: Option<std::boxed::Box<TerraneError>>,
    frames: Vec<TerraneSite>,
    structured: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    origin: TerraneSite,
    detail: Option<std::boxed::Box<TerraneErrorDetail>>,
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .cause = Some(std::boxed::Box::new(cause));
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
    fn descriptor_name(&self) -> &str {
        self.kind.display_name()
    }
    fn source_frames(&self) -> Vec<String> {
        let mut frames = Vec::new();
        if self.origin != TERRANE_NO_SITE {
            frames.push(__terrane_trace::render(self.origin));
        }
        if let Some(detail) = &self.detail {
            frames
                .extend(
                    detail.frames.iter().map(|frame| __terrane_trace::render(*frame)),
                );
        }
        frames
    }
    fn with_structured_details(mut self, structured: Vec<String>) -> Self {
        self
            .detail
            .get_or_insert_with(|| {
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .structured = structured;
        self
    }
    fn structured_details(&self) -> &[String] {
        self.detail.as_deref().map_or(&[], |detail| detail.structured.as_slice())
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
    pub static FUNCTIONS: [&str; 1] = ["/collections-value-semantics::main"];
    pub static SITES: [Site; 13] = [
        /* terrane-site-row: site 0: /collections-value-semantics::main (case.trn:8:3-8:21) */
        { Site { function: 0, file: 0, line: 8, column: 3, end_line: 8, end_column: 21 } },
        /* terrane-site-row: site 1: /collections-value-semantics::main (case.trn:9:47-9:61) */
        { Site { function: 0, file: 0, line: 9, column: 47, end_line: 9, end_column: 61 } },
        /* terrane-site-row: site 2: /collections-value-semantics::main (case.trn:9:63-9:77) */
        { Site { function: 0, file: 0, line: 9, column: 63, end_line: 9, end_column: 77 } },
        /* terrane-site-row: site 3: /collections-value-semantics::main (case.trn:15:10-15:27) */
        { Site { function: 0, file: 0, line: 15, column: 10, end_line: 15, end_column: 27 } },
        /* terrane-site-row: site 4: /collections-value-semantics::main (case.trn:21:23-21:30) */
        { Site { function: 0, file: 0, line: 21, column: 23, end_line: 21, end_column: 30 } },
        /* terrane-site-row: site 5: /collections-value-semantics::main (case.trn:24:13-24:24) */
        { Site { function: 0, file: 0, line: 24, column: 13, end_line: 24, end_column: 24 } },
        /* terrane-site-row: site 6: /collections-value-semantics::main (case.trn:27:15-27:38) */
        { Site { function: 0, file: 0, line: 27, column: 15, end_line: 27, end_column: 38 } },
        /* terrane-site-row: site 7: /collections-value-semantics::main (case.trn:31:11-31:26) */
        { Site { function: 0, file: 0, line: 31, column: 11, end_line: 31, end_column: 26 } },
        /* terrane-site-row: site 8: /collections-value-semantics::main (case.trn:36:5-36:19) */
        { Site { function: 0, file: 0, line: 36, column: 5, end_line: 36, end_column: 19 } },
        /* terrane-site-row: site 9: /collections-value-semantics::main (case.trn:41:36-41:63) */
        { Site { function: 0, file: 0, line: 41, column: 36, end_line: 41, end_column: 63 } },
        /* terrane-site-row: site 10: /collections-value-semantics::main (case.trn:56:29-56:41) */
        { Site { function: 0, file: 0, line: 56, column: 29, end_line: 56, end_column: 41 } },
        /* terrane-site-row: site 11: /collections-value-semantics::main (case.trn:58:10-58:22) */
        { Site { function: 0, file: 0, line: 58, column: 10, end_line: 58, end_column: 22 } },
        /* terrane-site-row: site 12: /collections-value-semantics::main (case.trn:58:24-58:36) */
        { Site { function: 0, file: 0, line: 58, column: 24, end_line: 58, end_column: 36 } },
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
fn __terrane_uninitialized_binding(
    name: &str,
    path: &str,
    line: usize,
    column: usize,
) -> ! {
    eprintln!(
        "{path}:{line}:{column}: error[T0007]: `{name}` may be read before it is assigned"
    );
    std::process::exit(1);
}
// Source: case.trn
// Namespace: collections-value-semantics
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S446 {
    Arm0(terrane_collection_support::Entry<String, terrane_int_support::Int>),
    Arm1(terrane_collection_support::Tuple<String>),
}
fn main() {
    let original_terrane_f0_s186: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut independent_terrane_f0_s210: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut ordered_terrane_f0_s357: terrane_collection_support::Map<
        String,
        terrane_int_support::Int,
    >;
    let mut pair_terrane_f0_s446: Option<TerraneUnionF0S446> = None;
    let mut unique_terrane_f0_s523: terrane_collection_support::Set<String>;
    let mut value_terrane_f0_s575: String;
    let pair_terrane_f0_s610: terrane_collection_support::Tuple<String>;
    let explicit_terrane_f0_s688: terrane_collection_support::Entry<
        String,
        terrane_int_support::Int,
    >;
    let numbers_terrane_f0_s755: terrane_collection_support::Range;
    let mut number_terrane_f0_s783: terrane_int_support::Int;
    let inclusive_terrane_f0_s821: terrane_collection_support::Range;
    let mut empty_count_terrane_f0_s903: terrane_int_support::Int;
    let empty_terrane_f0_s921: terrane_collection_support::Range;
    let mut ignored_terrane_f0_s951: terrane_int_support::Int;
    let mut deterministic_map_terrane_f0_s1070: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let mut deterministic_set_terrane_f0_s1274: terrane_collection_support::UnorderedSet<
        String,
    >;
    let mut empty_list_terrane_f0_s1508: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut empty_map_terrane_f0_s1563: terrane_collection_support::Map<
        terrane_int_support::Int,
        String,
    >;
    let nested_terrane_f0_s1626: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let arbitrary_terrane_f0_s1732: terrane_collection_support::Map<
        terrane_int_support::Int,
        String,
    >;
    original_terrane_f0_s186 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    independent_terrane_f0_s210 = original_terrane_f0_s186.clone();
    independent_terrane_f0_s210.append(terrane_int_support::Int::from(3_i128));
    let _ = __terrane_raised(
        independent_terrane_f0_s210
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    ),
                    0 /* terrane-site: case.trn:8:3-8:21 */,
                ),
                terrane_int_support::Int::from(4_i128),
            ),
        0 /* terrane-site: case.trn:8:3-8:21 */,
    );
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(original_terrane_f0_s186
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(independent_terrane_f0_s210
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(independent_terrane_f0_s210
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        1 /* terrane-site: case.trn:9:47-9:61 */)), 1 /* terrane-site: case.trn:9:47-9:61 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(independent_terrane_f0_s210
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:9:63-9:77 */)), 2 /* terrane-site: case.trn:9:63-9:77 */))
    );
    ordered_terrane_f0_s357 = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    ordered_terrane_f0_s357
        .set(String::from("third"), terrane_int_support::Int::from(3_i128));
    let _ = ordered_terrane_f0_s357
        .set(String::from("second"), terrane_int_support::Int::from(4_i128));
    let __terrane_iterable_0 = ordered_terrane_f0_s357.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let _ = pair_terrane_f0_s446
            .insert(
                TerraneUnionF0S446::Arm0(
                    match __terrane_iterator_0.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    },
                ),
            );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&match pair_terrane_f0_s446
            .as_ref().expect("flow-proven available binding") {
            TerraneUnionF0S446::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .key),
            terrane_scalar_support::scalar_text(&match pair_terrane_f0_s446.as_ref()
            .expect("flow-proven available binding") { TerraneUnionF0S446::Arm0(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .value)
        );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(ordered_terrane_f0_s357
        .get_or_error(&String::from("second")), 3 /* terrane-site: case.trn:15:10-15:27 */))
    );
    unique_terrane_f0_s523 = terrane_collection_support::Set::<
        String,
    >::new(vec![String::from("b"), String::from("a"), String::from("b")]);
    unique_terrane_f0_s523.add(String::from("c"));
    let __terrane_iterable_1 = unique_terrane_f0_s523;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        value_terrane_f0_s575 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s575));
    }
    let __trn_5f5f7265706c6163656d656e74_terrane_f0_s610 = terrane_collection_support::Tuple::<
        String,
    >::new(vec![String::from("left"), String::from("right")]);
    drop(pair_terrane_f0_s446.take());
    pair_terrane_f0_s610 = __trn_5f5f7265706c6163656d656e74_terrane_f0_s610;
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(pair_terrane_f0_s610
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(pair_terrane_f0_s610
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:21:23-21:30 */)), 4 /* terrane-site: case.trn:21:23-21:30 */))
    );
    explicit_terrane_f0_s688 = terrane_collection_support::Entry::<
        String,
        terrane_int_support::Int,
    >::new(String::from("key"), terrane_int_support::Int::from(7_i128));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&explicit_terrane_f0_s688.key),
        terrane_scalar_support::scalar_text(&explicit_terrane_f0_s688.value)
    );
    numbers_terrane_f0_s755 = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i64),
        ),
        5 /* terrane-site: case.trn:24:13-24:24 */,
    );
    let __terrane_iterable_2 = numbers_terrane_f0_s755;
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        number_terrane_f0_s783 = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number_terrane_f0_s783));
    }
    inclusive_terrane_f0_s821 = __terrane_raised(
        terrane_collection_support::Range::through(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(-1_i128),
        ),
        6 /* terrane-site: case.trn:27:15-27:38 */,
    );
    let __terrane_iterable_3 = inclusive_terrane_f0_s821;
    let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_3,
    );
    loop {
        number_terrane_f0_s783 = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &number_terrane_f0_s783;
        println!("{}", terrane_scalar_support::scalar_text(&number_terrane_f0_s783));
    }
    empty_count_terrane_f0_s903 = terrane_int_support::Int::from(0_i128);
    empty_terrane_f0_s921 = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(-1_i128),
        ),
        7 /* terrane-site: case.trn:31:11-31:26 */,
    );
    let __terrane_iterable_4 = empty_terrane_f0_s921;
    let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_4,
    );
    loop {
        ignored_terrane_f0_s951 = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &ignored_terrane_f0_s951;
        empty_count_terrane_f0_s903 = empty_count_terrane_f0_s903.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&empty_count_terrane_f0_s903));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                terrane_collection_support::Range::new(terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(3_i128),
                terrane_int_support::Int::from(0_i128)), 8 /* terrane-site: case.trn:36:5-36:19 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&String::from("zero"))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    deterministic_map_terrane_f0_s1070 = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    let _ = deterministic_map_terrane_f0_s1070
        .set(String::from("second"), terrane_int_support::Int::from(3_i128));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_map_terrane_f0_s1070
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(deterministic_map_terrane_f0_s1070
        .get_or_error(&String::from("second")), 9 /* terrane-site: case.trn:41:36-41:63 */))
    );
    let __terrane_iterable_5 = deterministic_map_terrane_f0_s1070;
    let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_5,
    );
    loop {
        let _ = pair_terrane_f0_s446
            .insert(
                TerraneUnionF0S446::Arm0(
                    match __terrane_iterator_5.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    },
                ),
            );
        let _ = &pair_terrane_f0_s446;
        println!(
            "{}", terrane_scalar_support::scalar_text(&match pair_terrane_f0_s446
            .as_ref().expect("flow-proven available binding") {
            TerraneUnionF0S446::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .key)
        );
    }
    deterministic_set_terrane_f0_s1274 = terrane_collection_support::UnorderedSet::<
        String,
    >::new(vec![String::from("x"), String::from("y")]);
    deterministic_set_terrane_f0_s1274.add(String::from("z"));
    deterministic_set_terrane_f0_s1274.remove(&String::from("x"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&deterministic_set_terrane_f0_s1274
        .contains(&String::from("y")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_set_terrane_f0_s1274
        .length()))
    );
    let __terrane_iterable_6 = deterministic_set_terrane_f0_s1274;
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_6,
    );
    loop {
        value_terrane_f0_s575 = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value_terrane_f0_s575;
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s575));
    }
    empty_list_terrane_f0_s1508 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    empty_list_terrane_f0_s1508.append(terrane_int_support::Int::from(5_i128));
    empty_map_terrane_f0_s1563 = terrane_collection_support::Map::<
        terrane_int_support::Int,
        String,
    >::new(Vec::new());
    empty_map_terrane_f0_s1563
        .set(terrane_int_support::Int::from(1_i128), String::from("one"));
    nested_terrane_f0_s1626 = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(9_i128)])
        ],
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty_list_terrane_f0_s1508
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(empty_map_terrane_f0_s1563
        .get_or_error(&terrane_int_support::Int::from(1_i128)), 10 /* terrane-site: case.trn:56:29-56:41 */)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested_terrane_f0_s1626
        .length()))
    );
    arbitrary_terrane_f0_s1732 = terrane_collection_support::Map::<
        terrane_int_support::Int,
        String,
    >::new(
        vec![
            terrane_collection_support::Entry::< terrane_int_support::Int, String
            >::new(terrane_int_support::Int::from(2_i128), String::from("two")),
            terrane_collection_support::Entry::< terrane_int_support::Int, String
            >::new(terrane_int_support::Int::from(3_i128), String::from("three"))
        ],
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(arbitrary_terrane_f0_s1732
        .get_or_error(&terrane_int_support::Int::from(2_i128)), 11 /* terrane-site: case.trn:58:10-58:22 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(arbitrary_terrane_f0_s1732
        .get_or_error(&terrane_int_support::Int::from(3_i128)), 12 /* terrane-site: case.trn:58:24-58:36 */))
    );
}
