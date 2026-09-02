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
    pub static FILES: [&str; 1] = ["standard/paths.trn"];
    pub static FUNCTIONS: [&str; 6] = [
        "/standard/paths::path-components",
        "/standard/paths::normalise-path",
        "/standard/paths::path-name",
        "/standard/paths::path-parent",
        "/standard/paths::path-stem",
        "/standard/paths::path-extension",
    ];
    pub static SITES: [Site; 12] = [
        {
            /* terrane-site-row: site 0: /standard/paths::path-components (standard/paths.trn:16:16-16:28) */
            Site {
                function: 0,
                file: 0,
                line: 16,
                column: 16,
                end_line: 16,
                end_column: 28,
            }
        },
        {
            /* terrane-site-row: site 1: /standard/paths::normalise-path (standard/paths.trn:32:16-32:33) */
            Site {
                function: 1,
                file: 0,
                line: 32,
                column: 16,
                end_line: 32,
                end_column: 33,
            }
        },
        {
            /* terrane-site-row: site 2: /standard/paths::normalise-path (standard/paths.trn:35:34-35:49) */
            Site {
                function: 1,
                file: 0,
                line: 35,
                column: 34,
                end_line: 35,
                end_column: 49,
            }
        },
        {
            /* terrane-site-row: site 3: /standard/paths::normalise-path (standard/paths.trn:40:29-40:50) */
            Site {
                function: 1,
                file: 0,
                line: 40,
                column: 29,
                end_line: 40,
                end_column: 50,
            }
        },
        {
            /* terrane-site-row: site 4: /standard/paths::normalise-path (standard/paths.trn:46:21-46:42) */
            Site {
                function: 1,
                file: 0,
                line: 46,
                column: 21,
                end_line: 46,
                end_column: 42,
            }
        },
        {
            /* terrane-site-row: site 5: /standard/paths::normalise-path (standard/paths.trn:56:33-56:44) */
            Site {
                function: 1,
                file: 0,
                line: 56,
                column: 33,
                end_line: 56,
                end_column: 44,
            }
        },
        {
            /* terrane-site-row: site 6: /standard/paths::path-name (standard/paths.trn:69:12-69:35) */
            Site {
                function: 2,
                file: 0,
                line: 69,
                column: 12,
                end_line: 69,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 7: /standard/paths::path-parent (standard/paths.trn:83:33-83:45) */
            Site {
                function: 3,
                file: 0,
                line: 83,
                column: 33,
                end_line: 83,
                end_column: 45,
            }
        },
        {
            /* terrane-site-row: site 8: /standard/paths::path-stem (standard/paths.trn:95:31-95:40) */
            Site {
                function: 4,
                file: 0,
                line: 95,
                column: 31,
                end_line: 95,
                end_column: 40,
            }
        },
        {
            /* terrane-site-row: site 9: /standard/paths::path-stem (standard/paths.trn:102:33-102:46) */
            Site {
                function: 4,
                file: 0,
                line: 102,
                column: 33,
                end_line: 102,
                end_column: 46,
            }
        },
        {
            /* terrane-site-row: site 10: /standard/paths::path-extension (standard/paths.trn:111:31-111:40) */
            Site {
                function: 5,
                file: 0,
                line: 111,
                column: 31,
                end_line: 111,
                end_column: 40,
            }
        },
        {
            /* terrane-site-row: site 11: /standard/paths::path-extension (standard/paths.trn:113:12-113:37) */
            Site {
                function: 5,
                file: 0,
                line: 113,
                column: 12,
                end_line: 113,
                end_column: 37,
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
// Namespace: conformance/lexical-paths
fn main() {
    let relative: Path = Path::terrane_construct(
        String::from("alpha/./beta/../gamma/../../delta"),
    );
    let rooted: Path = Path::terrane_construct(
        String::from("/alpha/../../beta/file.tar.gz"),
    );
    let base: Path = Path::terrane_construct(String::from("work/root"));
    let child: Path = Path::terrane_construct(String::from("../next"));
    let relative_normal: Path = normalise_path(relative);
    let rooted_normal: Path = normalise_path(rooted);
    let relative_text: String = relative_normal.text.clone();
    let rooted_text: String = rooted_normal.text.clone();
    let rooted_name: String = path_name(rooted_normal.clone());
    let rooted_stem: String = path_stem(rooted_normal.clone());
    let rooted_extension: String = path_extension(rooted_normal.clone());
    let rooted_parent: Path = path_parent(rooted_normal.clone());
    let rooted_parent_text: String = rooted_parent.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&relative_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_name));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_stem));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_extension));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_parent_text));
    let resolved: Path = join_path(base, child);
    let resolved_text: String = resolved.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&resolved_text));
    let components: terrane_collection_support::List<String> = path_components(
        rooted_normal.clone(),
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(components
        .length())), terrane_scalar_support::scalar_text(&path_is_absolute(rooted_normal
        .clone())), terrane_scalar_support::scalar_text(&path_is_absolute(relative_normal
        .clone()))
    );
    let hidden: Path = Path::terrane_construct(String::from(".profile"));
    println!("{}", terrane_scalar_support::scalar_text(&path_stem(hidden.clone())));
    println!("{}", terrane_scalar_support::scalar_text(&path_extension(hidden.clone())));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&path_parent(Path::terrane_construct(String::from("child")))
        .text)
    );
}
// Source: standard/paths.trn
// Namespace: standard/paths
#[derive(Clone)]
pub struct Path {
    pub text: String,
}
impl Path {
    pub fn terrane_construct(input: String) -> Self {
        let mut value = Self { text: String::from("") };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: String) {
        self.text = input;
    }
}
pub fn path_components(subject: Path) -> terrane_collection_support::List<String> {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let mut result: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        let part: String = __terrane_raised(
            parts
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&index.clone()),
                        0 /* terrane-site: standard/paths.trn:16:16-16:28 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(&index.clone()),
                        0 /* terrane-site: standard/paths.trn:16:16-16:28 */,
                    ),
                }),
            0 /* terrane-site: standard/paths.trn:16:16-16:28 */,
        );
        if part != String::from("") {
            result.append(part);
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result.clone();
}
pub fn path_is_absolute(subject: Path) -> bool {
    return subject.text.starts_with(&String::from("/"));
}
pub fn normalise_path(subject: Path) -> Path {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let absolute: bool = path_is_absolute(subject.clone());
    let mut kept: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    let mut part_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while part_index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        let part: String = __terrane_raised(
            parts
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&part_index.clone()),
                        1 /* terrane-site: standard/paths.trn:32:16-32:33 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(&part_index.clone()),
                        1 /* terrane-site: standard/paths.trn:32:16-32:33 */,
                    ),
                }),
            1 /* terrane-site: standard/paths.trn:32:16-32:33 */,
        );
        if part != String::from("") && part != String::from(".") {
            if part == String::from("..") {
                if count.clone() > terrane_int_support::Int::from(0_i128)
                    && __terrane_raised(
                        kept
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &(count.clone() - terrane_int_support::Int::from(1_i128)),
                                    ),
                                    2 /* terrane-site: standard/paths.trn:35:34-35:49 */,
                                ),
                            ),
                        2 /* terrane-site: standard/paths.trn:35:34-35:49 */,
                    ) != String::from("..")
                {
                    count = count.clone() - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute {
                        if count.clone()
                            < terrane_int_support::Int::from(
                                terrane_int_support::Int::from(kept.length()),
                            )
                        {
                            __terrane_raised(
                                kept
                                    .set(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(&count.clone()),
                                            3 /* terrane-site: standard/paths.trn:40:29-40:50 */,
                                        ),
                                        part,
                                    ),
                                3 /* terrane-site: standard/paths.trn:40:29-40:50 */,
                            );
                        } else {
                            kept.append(part);
                        }
                        count = count.clone() + terrane_int_support::Int::from(1_i128);
                    }
                }
            } else {
                if count.clone()
                    < terrane_int_support::Int::from(
                        terrane_int_support::Int::from(kept.length()),
                    )
                {
                    __terrane_raised(
                        kept
                            .set(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&count.clone()),
                                    4 /* terrane-site: standard/paths.trn:46:21-46:42 */,
                                ),
                                part,
                            ),
                        4 /* terrane-site: standard/paths.trn:46:21-46:42 */,
                    );
                } else {
                    kept.append(part);
                }
                count = count.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
        part_index = part_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < count.clone() {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&__terrane_raised(kept
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 5 /* terrane-site: standard/paths.trn:56:33-56:44 */)),
            5 /* terrane-site: standard/paths.trn:56:33-56:44 */))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    if result == String::from("") && absolute {
        result = String::from("/");
    }
    return Path::terrane_construct(result);
}
pub fn path_name(subject: Path) -> String {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(normal);
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return String::from("");
    }
    return __terrane_raised(
        parts
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(
                            terrane_int_support::Int::from(parts.length()),
                        ) - terrane_int_support::Int::from(1_i128)),
                    ),
                    6 /* terrane-site: standard/paths.trn:69:12-69:35 */,
                ),
            ),
        6 /* terrane-site: standard/paths.trn:69:12-69:35 */,
    );
}
pub fn path_parent(subject: Path) -> Path {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(
        normal.clone(),
    );
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return normal.clone();
    }
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(1_i128) && !path_is_absolute(normal.clone())
    {
        return Path::terrane_construct(String::from("."));
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
            - terrane_int_support::Int::from(1_i128)
    {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&__terrane_raised(parts
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 7 /* terrane-site: standard/paths.trn:83:33-83:45 */)),
            7 /* terrane-site: standard/paths.trn:83:33-83:45 */))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let absolute: bool = path_is_absolute(normal.clone());
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    return Path::terrane_construct(result);
}
pub fn path_stem(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return current;
    }
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
            pieces
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        8 /* terrane-site: standard/paths.trn:95:31-95:40 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        8 /* terrane-site: standard/paths.trn:95:31-95:40 */,
                    ),
                }),
            8 /* terrane-site: standard/paths.trn:95:31-95:40 */,
        ) == String::from("")
    {
        return current;
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(pieces.len() as i128)
            - terrane_int_support::Int::from(1_i128)
    {
        if index.clone() > terrane_int_support::Int::from(0_i128) {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("."))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&__terrane_raised(pieces
            .get(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 9 /* terrane-site: standard/paths.trn:102:33-102:46 */))
            .cloned().ok_or(terrane_collection_support::IndexError { index :
            __terrane_raised(terrane_collection_support::index_from_int(&index.clone()),
            9 /* terrane-site: standard/paths.trn:102:33-102:46 */) }),
            9 /* terrane-site: standard/paths.trn:102:33-102:46 */))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result;
}
pub fn path_extension(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return String::from("");
    }
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
            pieces
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        10 /* terrane-site: standard/paths.trn:111:31-111:40 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        10 /* terrane-site: standard/paths.trn:111:31-111:40 */,
                    ),
                }),
            10 /* terrane-site: standard/paths.trn:111:31-111:40 */,
        ) == String::from("")
    {
        return String::from("");
    }
    return __terrane_raised(
        pieces
            .get(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(pieces.len() as i128)
                            - terrane_int_support::Int::from(1_i128)),
                    ),
                    11 /* terrane-site: standard/paths.trn:113:12-113:37 */,
                ),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(pieces.len() as i128)
                            - terrane_int_support::Int::from(1_i128)),
                    ),
                    11 /* terrane-site: standard/paths.trn:113:12-113:37 */,
                ),
            }),
        11 /* terrane-site: standard/paths.trn:113:12-113:37 */,
    );
}
pub fn join_path(base: Path, child: Path) -> Path {
    let absolute: bool = path_is_absolute(child.clone());
    if absolute {
        return normalise_path(child.clone());
    }
    let mut joined: String = base.text.clone();
    if joined != String::from("") && !joined.ends_with(&String::from("/")) {
        joined = format!(
            "{}{}", terrane_scalar_support::scalar_text(&joined),
            terrane_scalar_support::scalar_text(&String::from("/"))
        );
    }
    joined = format!(
        "{}{}", terrane_scalar_support::scalar_text(&joined),
        terrane_scalar_support::scalar_text(&child.text)
    );
    let combined: Path = Path::terrane_construct(joined);
    return normalise_path(combined);
}
