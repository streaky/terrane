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
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 3] = [
        "/callable-class-parameters::construct",
        "/callable-class-parameters::apply",
        "/callable-class-parameters::main",
    ];
    pub static SITES: [Site; 8] = [
        {
            /* terrane-site-row: site 0: /callable-class-parameters::construct (case.trn:10:19-10:31) */
            Site {
                function: 0,
                file: 0,
                line: 10,
                column: 19,
                end_line: 10,
                end_column: 31,
            }
        },
        {
            /* terrane-site-row: site 1: /callable-class-parameters::apply (case.trn:13:12-13:42) */
            Site {
                function: 1,
                file: 0,
                line: 13,
                column: 12,
                end_line: 13,
                end_column: 42,
            }
        },
        {
            /* terrane-site-row: site 2: /callable-class-parameters::construct (case.trn:22:19-22:31) */
            Site {
                function: 0,
                file: 0,
                line: 22,
                column: 19,
                end_line: 22,
                end_column: 31,
            }
        },
        {
            /* terrane-site-row: site 3: /callable-class-parameters::apply (case.trn:25:12-25:42) */
            Site {
                function: 1,
                file: 0,
                line: 25,
                column: 12,
                end_line: 25,
                end_column: 42,
            }
        },
        {
            /* terrane-site-row: site 4: /callable-class-parameters::main (case.trn:31:26-31:59) */
            Site {
                function: 2,
                file: 0,
                line: 31,
                column: 26,
                end_line: 31,
                end_column: 59,
            }
        },
        {
            /* terrane-site-row: site 5: /callable-class-parameters::main (case.trn:32:11-32:37) */
            Site {
                function: 2,
                file: 0,
                line: 32,
                column: 11,
                end_line: 32,
                end_column: 37,
            }
        },
        {
            /* terrane-site-row: site 6: /callable-class-parameters::main (case.trn:34:22-34:53) */
            Site {
                function: 2,
                file: 0,
                line: 34,
                column: 22,
                end_line: 34,
                end_column: 53,
            }
        },
        {
            /* terrane-site-row: site 7: /callable-class-parameters::main (case.trn:35:11-35:35) */
            Site {
                function: 2,
                file: 0,
                line: 35,
                column: 11,
                end_line: 35,
                end_column: 35,
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
// Namespace: callable-class-parameters
fn increment(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(1_i128);
}
#[derive(Clone)]
pub struct DirectHolder {
    pub stored: terrane_int_support::Int,
}
impl DirectHolder {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Result<Self, TerraneError> {
        let mut value = Self {
            stored: terrane_int_support::Int::from(0_i128),
        };
        value.construct(operation)?;
        Ok(value)
    }
    pub fn construct(
        &mut self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Result<(), TerraneError> {
        self.stored = __terrane_traced_err(
            operation(terrane_int_support::Int::from(1_i128)),
            0 /* terrane-site: case.trn:10:19-10:31 */,
        )?;
        Ok(())
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        return Ok(
            __terrane_traced_err(
                operation(self.stored.clone() + value.clone()),
                1 /* terrane-site: case.trn:13:12-13:42 */,
            )?,
        );
    }
    pub fn ignore(
        &self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        let _ = &operation;
        return value.clone();
    }
}
#[derive(Clone)]
pub struct BaseHolderStorage {
    pub stored: terrane_int_support::Int,
}
impl BaseHolderStorage {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Result<Self, TerraneError> {
        let mut value = Self {
            stored: terrane_int_support::Int::from(0_i128),
        };
        value.construct(operation)?;
        Ok(value)
    }
    pub fn construct(
        &mut self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Result<(), TerraneError> {
        self.stored = __terrane_traced_err(
            operation(terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: case.trn:22:19-22:31 */,
        )?;
        Ok(())
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        return Ok(
            __terrane_traced_err(
                operation(self.stored.clone() + value.clone()),
                3 /* terrane-site: case.trn:25:12-25:42 */,
            )?,
        );
    }
}
#[derive(Clone)]
pub enum BaseHolder {
    Own(BaseHolderStorage),
    ChildHolder(ChildHolder),
}
impl BaseHolder {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Result<Self, TerraneError> {
        Ok(Self::Own(BaseHolderStorage::terrane_construct(operation)?))
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        match self {
            Self::Own(value_) => value_.apply(operation, value),
            Self::ChildHolder(value_) => value_.apply(operation, value),
        }
    }
    pub fn terrane_field_stored(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.stored,
            Self::ChildHolder(value) => &value.stored,
        }
    }
    pub fn terrane_field_stored_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.stored,
            Self::ChildHolder(value) => &mut value.stored,
        }
    }
}
#[derive(Clone)]
pub struct ChildHolder {
    pub stored: terrane_int_support::Int,
}
impl ChildHolder {
    pub fn terrane_construct(
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Result<Self, TerraneError> {
        let mut value = Self {
            stored: terrane_int_support::Int::from(0_i128),
        };
        value.construct(operation)?;
        Ok(value)
    }
    pub fn construct(
        &mut self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
    ) -> Result<(), TerraneError> {
        self.stored = __terrane_traced_err(
            operation(terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: case.trn:22:19-22:31 */,
        )?;
        Ok(())
    }
    pub fn apply(
        &self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        return Ok(
            __terrane_traced_err(
                operation(self.stored.clone() + value.clone()),
                3 /* terrane-site: case.trn:25:12-25:42 */,
            )?,
        );
    }
}
fn main() {
    let operation: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> Result<terrane_int_support::Int, TerraneError> + Send + Sync,
    > = std::sync::Arc::new(move |argument_0: terrane_int_support::Int| Ok(
        increment(argument_0),
    ));
    let direct: DirectHolder = __terrane_traced(
        DirectHolder::terrane_construct(operation.clone()),
        4 /* terrane-site: case.trn:31:26-31:59 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(direct
        .apply(operation.clone(), terrane_int_support::Int::from(4_i128)),
        5 /* terrane-site: case.trn:32:11-32:37 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&direct.ignore(operation.clone(),
        terrane_int_support::Int::from(9_i128)))
    );
    let base: BaseHolder = __terrane_traced(
        BaseHolder::terrane_construct(operation.clone()),
        6 /* terrane-site: case.trn:34:22-34:53 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(base.apply(operation
        .clone(), terrane_int_support::Int::from(5_i128)), 7 /* terrane-site: case.trn:35:11-35:35 */))
    );
}
