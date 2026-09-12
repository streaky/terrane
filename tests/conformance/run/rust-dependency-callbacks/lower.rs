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
    Custom(DescriptorId),
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
            Self::Custom(descriptor) => {
                __terrane_error_registry::DESCRIPTORS[usize::from(descriptor.0)]
            }
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
            Self::Custom(_) => "source error",
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
    fn custom_raised(
        descriptor: DescriptorId,
        message: impl Into<String>,
        origin: TerraneSite,
    ) -> Self {
        Self::raised_with_message(TerraneErrorKind::Custom(descriptor), message, origin)
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
#[allow(dead_code, reason = "a projected dependency may expose no Result members")]
const TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId(0);
#[allow(dead_code, reason = "panic catching may be disabled or not crossed")]
const TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId(1);
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
    TerraneForeignError(
        TerraneError::custom_raised(
            TERRANE_DEPENDENCY_PANIC,
            format!(
                "Rust dependency `{crate_name}` member `{member}` panicked: {detail}"
            ),
            TERRANE_NO_SITE,
        ),
    )
}
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 2] = ["dependency-error", "dependency-panic"];
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
    pub static FILES: [&str; 1] = ["src/main.trn"];
    pub static FUNCTIONS: [&str; 8] = [
        "/app::stay-pending",
        "/app::run-retained",
        "/app",
        "/app::shared",
        "/app::mutable",
        "/app::consuming",
        "/app::destruct",
        "/app::main",
    ];
    pub static SITES: [Site; 43] = [
        /* terrane-site-row: site 0: /app::stay-pending (src/main.trn:20:24-20:44) */
        { Site { function: 0, file: 0, line: 20, column: 24, end_line: 20, end_column: 44 } },
        /* terrane-site-row: site 1: /app::run-retained (src/main.trn:25:22-25:51) */
        { Site { function: 1, file: 0, line: 25, column: 22, end_line: 25, end_column: 51 } },
        /* terrane-site-row: site 2: /app (src/main.trn:60:24-60:40) */
        { Site { function: 2, file: 0, line: 60, column: 24, end_line: 60, end_column: 40 } },
        /* terrane-site-row: site 3: /app::shared (src/main.trn:72:30-72:46) */
        { Site { function: 3, file: 0, line: 72, column: 30, end_line: 72, end_column: 46 } },
        /* terrane-site-row: site 4: /app::shared (src/main.trn:74:17-74:44) */
        { Site { function: 3, file: 0, line: 74, column: 17, end_line: 74, end_column: 44 } },
        /* terrane-site-row: site 5: /app::shared (src/main.trn:76:13-76:40) */
        { Site { function: 3, file: 0, line: 76, column: 13, end_line: 76, end_column: 40 } },
        /* terrane-site-row: site 6: /app::mutable (src/main.trn:81:30-81:46) */
        { Site { function: 4, file: 0, line: 81, column: 30, end_line: 81, end_column: 46 } },
        /* terrane-site-row: site 7: /app::mutable (src/main.trn:83:17-83:44) */
        { Site { function: 4, file: 0, line: 83, column: 17, end_line: 83, end_column: 44 } },
        /* terrane-site-row: site 8: /app::mutable (src/main.trn:85:13-85:40) */
        { Site { function: 4, file: 0, line: 85, column: 13, end_line: 85, end_column: 40 } },
        /* terrane-site-row: site 9: /app::consuming (src/main.trn:90:30-90:46) */
        { Site { function: 5, file: 0, line: 90, column: 30, end_line: 90, end_column: 46 } },
        /* terrane-site-row: site 10: /app::consuming (src/main.trn:92:17-92:44) */
        { Site { function: 5, file: 0, line: 92, column: 17, end_line: 92, end_column: 44 } },
        /* terrane-site-row: site 11: /app::consuming (src/main.trn:94:13-94:40) */
        { Site { function: 5, file: 0, line: 94, column: 13, end_line: 94, end_column: 40 } },
        /* terrane-site-row: site 12: /app::destruct (src/main.trn:97:17-97:43) */
        { Site { function: 6, file: 0, line: 97, column: 17, end_line: 97, end_column: 43 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:103:13-103:38) */
        { Site { function: 7, file: 0, line: 103, column: 13, end_line: 103, end_column: 38 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:104:13-104:39) */
        { Site { function: 7, file: 0, line: 104, column: 13, end_line: 104, end_column: 39 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:105:13-105:45) */
        { Site { function: 7, file: 0, line: 105, column: 13, end_line: 105, end_column: 45 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:106:28-106:67) */
        { Site { function: 7, file: 0, line: 106, column: 28, end_line: 106, end_column: 67 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:111:28-111:68) */
        { Site { function: 7, file: 0, line: 111, column: 28, end_line: 111, end_column: 68 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:113:25-113:35) */
        { Site { function: 7, file: 0, line: 113, column: 25, end_line: 113, end_column: 35 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:114:13-114:48) */
        { Site { function: 7, file: 0, line: 114, column: 13, end_line: 114, end_column: 48 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:116:13-116:49) */
        { Site { function: 7, file: 0, line: 116, column: 13, end_line: 116, end_column: 49 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:119:25-119:63) */
        { Site { function: 7, file: 0, line: 119, column: 25, end_line: 119, end_column: 63 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:122:49-122:77) */
        { Site { function: 7, file: 0, line: 122, column: 49, end_line: 122, end_column: 77 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:124:13-124:47) */
        { Site { function: 7, file: 0, line: 124, column: 13, end_line: 124, end_column: 47 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:125:13-125:49) */
        { Site { function: 7, file: 0, line: 125, column: 13, end_line: 125, end_column: 49 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:129:13-129:42) */
        { Site { function: 7, file: 0, line: 129, column: 13, end_line: 129, end_column: 42 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:131:13-131:48) */
        { Site { function: 7, file: 0, line: 131, column: 13, end_line: 131, end_column: 48 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:133:13-133:37) */
        { Site { function: 7, file: 0, line: 133, column: 13, end_line: 133, end_column: 37 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:135:13-135:42) */
        { Site { function: 7, file: 0, line: 135, column: 13, end_line: 135, end_column: 42 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:136:24-136:50) */
        { Site { function: 7, file: 0, line: 136, column: 24, end_line: 136, end_column: 50 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:137:13-137:39) */
        { Site { function: 7, file: 0, line: 137, column: 13, end_line: 137, end_column: 39 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:137:43-137:68) */
        { Site { function: 7, file: 0, line: 137, column: 43, end_line: 137, end_column: 68 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:139:19-139:57) */
        { Site { function: 7, file: 0, line: 139, column: 19, end_line: 139, end_column: 57 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:140:13-140:35) */
        { Site { function: 7, file: 0, line: 140, column: 13, end_line: 140, end_column: 35 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:140:39-140:59) */
        { Site { function: 7, file: 0, line: 140, column: 39, end_line: 140, end_column: 59 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:142:20-142:57) */
        { Site { function: 7, file: 0, line: 142, column: 20, end_line: 142, end_column: 57 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:143:13-143:35) */
        { Site { function: 7, file: 0, line: 143, column: 13, end_line: 143, end_column: 35 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:143:39-143:60) */
        { Site { function: 7, file: 0, line: 143, column: 39, end_line: 143, end_column: 60 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:144:19-144:79) */
        { Site { function: 7, file: 0, line: 144, column: 19, end_line: 144, end_column: 79 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:145:13-145:34) */
        { Site { function: 7, file: 0, line: 145, column: 13, end_line: 145, end_column: 34 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:145:38-145:58) */
        { Site { function: 7, file: 0, line: 145, column: 38, end_line: 145, end_column: 58 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:146:13-146:55) */
        { Site { function: 7, file: 0, line: 146, column: 13, end_line: 146, end_column: 55 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:147:19-147:72) */
        { Site { function: 7, file: 0, line: 147, column: 19, end_line: 147, end_column: 72 } },
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
trait TerraneMutableCallableBody<Arguments, Output>: Send {
    fn call(&mut self, arguments: Arguments) -> Output;
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>;
}
impl<Arguments, Output, Function> TerraneMutableCallableBody<Arguments, Output>
for Function
where
    Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
{
    fn call(&mut self, arguments: Arguments) -> Output {
        self(arguments)
    }
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>> {
        std::boxed::Box::new(self.clone())
    }
}
pub struct TerraneMutableCallable<Arguments, Output> {
    body: std::sync::Mutex<
        std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>,
    >,
}
impl<Arguments, Output> TerraneMutableCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
    {
        Self {
            body: std::sync::Mutex::new(std::boxed::Box::new(function)),
        }
    }
    fn call(&self, arguments: Arguments) -> Output {
        self.body.lock().expect("mutable callable lock poisoned").call(arguments)
    }
}
impl<Arguments, Output> Clone for TerraneMutableCallable<Arguments, Output> {
    fn clone(&self) -> Self {
        let body = self.body.lock().expect("mutable callable lock poisoned").clone_box();
        Self {
            body: std::sync::Mutex::new(body),
        }
    }
}
trait TerraneConsumingCallableBody<Arguments, Output>: Send {
    fn call(self: std::boxed::Box<Self>, arguments: Arguments) -> Output;
}
impl<Arguments, Output, Function> TerraneConsumingCallableBody<Arguments, Output>
for Function
where
    Function: FnOnce(Arguments) -> Output + Send + 'static,
{
    fn call(self: std::boxed::Box<Self>, arguments: Arguments) -> Output {
        self(arguments)
    }
}
pub struct TerraneConsumingCallable<Arguments, Output> {
    body: std::boxed::Box<dyn TerraneConsumingCallableBody<Arguments, Output>>,
}
impl<Arguments, Output> TerraneConsumingCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnOnce(Arguments) -> Output + Send + 'static,
    {
        Self {
            body: std::boxed::Box::new(function),
        }
    }
    fn call(self, arguments: Arguments) -> Output {
        self.body.call(arguments)
    }
}
use std::future::Future;
#[derive(Clone)]
struct TerraneCancellation {
    state: std::sync::Arc<TerraneCancellationState>,
}
struct TerraneCancellationState {
    cancelled: std::sync::atomic::AtomicBool,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}
impl TerraneCancellation {
    fn new() -> Self {
        Self {
            state: std::sync::Arc::new(TerraneCancellationState {
                cancelled: std::sync::atomic::AtomicBool::new(false),
                wakers: std::sync::Mutex::new(Vec::new()),
            }),
        }
    }
    fn wake_waiters(&self) {
        let wakers = std::mem::take(
            &mut *self.state.wakers.lock().expect("cancellation waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    fn cancel(&self) {
        if !self.state.cancelled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.wake_waiters();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }
    async fn cancelled(&self) {
        std::future::poll_fn(|context| {
                if self.is_cancelled() {
                    return std::task::Poll::Ready(());
                }
                let mut wakers = self
                    .state
                    .wakers
                    .lock()
                    .expect("cancellation waker lock poisoned");
                if self.is_cancelled() {
                    return std::task::Poll::Ready(());
                }
                if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                    wakers.push(context.waker().clone());
                }
                std::task::Poll::Pending
            })
            .await
    }
}
struct TerraneFinalizerState {
    depth: std::sync::atomic::AtomicUsize,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}
impl TerraneFinalizerState {
    fn new() -> Self {
        Self {
            depth: std::sync::atomic::AtomicUsize::new(0),
            wakers: std::sync::Mutex::new(Vec::new()),
        }
    }
    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn register(self: &std::sync::Arc<Self>) -> TerraneFinallyGuard {
        let depth = self.depth.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;
        TerraneFinallyGuard {
            state: Some(self.clone()),
            depth,
        }
    }
    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn unregister(&self, depth: usize) {
        let current = self.depth.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        debug_assert_eq!(current, depth, "finally regions must leave innermost first");
        let wakers = std::mem::take(
            &mut *self.wakers.lock().expect("finalizer waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    async fn reaches(&self, depth: usize) {
        std::future::poll_fn(|context| {
                if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                    return std::task::Poll::Ready(());
                }
                let mut wakers = self
                    .wakers
                    .lock()
                    .expect("finalizer waker lock poisoned");
                if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                    return std::task::Poll::Ready(());
                }
                if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                    wakers.push(context.waker().clone());
                }
                std::task::Poll::Pending
            })
            .await
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneCancellationContext {
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
}
tokio::task_local! {
    static TERRANE_CANCELLATION_CONTEXT : TerraneCancellationContext;
}
static TERRANE_PROJECTED_CLEANUPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
    0,
);
static TERRANE_PROJECTED_CLEANUPS_CHANGED: tokio::sync::Notify = tokio::sync::Notify::const_new();
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
struct TerraneProjectedEntryGuard {
    cancellation: TerraneCancellation,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
    abort: Option<tokio::task::AbortHandle>,
    armed: bool,
}
impl Drop for TerraneProjectedEntryGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        TERRANE_PROJECTED_CLEANUPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.cancellation.cancel();
        let finalizers = self.finalizers.clone();
        let abort = self
            .abort
            .take()
            .expect("armed projected entry guard must own its abort handle");
        tokio::spawn(async move {
            finalizers.reaches(0).await;
            abort.abort();
            if TERRANE_PROJECTED_CLEANUPS
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1
            {
                TERRANE_PROJECTED_CLEANUPS_CHANGED.notify_waiters();
            }
        });
    }
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_projected_async_entry<F, T>(future: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline: None,
        finalizers: finalizers.clone(),
    };
    let mut future = Box::pin(TERRANE_CANCELLATION_CONTEXT.scope(context, future));
    let first_poll = std::future::poll_fn(|cx| std::task::Poll::Ready(
            future.as_mut().poll(cx),
        ))
        .await;
    if let std::task::Poll::Ready(output) = first_poll {
        return output;
    }
    let task = tokio::spawn(future);
    let mut guard = TerraneProjectedEntryGuard {
        cancellation,
        finalizers,
        abort: Some(task.abort_handle()),
        armed: true,
    };
    let output = task
        .await
        .expect("projected asynchronous dependency entry task failed");
    guard.armed = false;
    output
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_wait_projected_cleanups() {
    loop {
        let changed = TERRANE_PROJECTED_CLEANUPS_CHANGED.notified();
        if TERRANE_PROJECTED_CLEANUPS.load(std::sync::atomic::Ordering::SeqCst) == 0 {
            return;
        }
        changed.await;
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneFinallyGuard {
    state: Option<std::sync::Arc<TerraneFinalizerState>>,
    depth: usize,
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
impl TerraneFinallyGuard {
    fn finish(&mut self) {
        if let Some(state) = self.state.take() {
            state.unregister(self.depth);
        }
    }
}
impl Drop for TerraneFinallyGuard {
    fn drop(&mut self) {
        self.finish();
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
fn __terrane_finally_guard() -> TerraneFinallyGuard {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| context.finalizers.register())
        .unwrap_or(TerraneFinallyGuard {
            state: None,
            depth: 0,
        })
}
async fn __terrane_cancellation_requested(
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
) {
    if let Some(deadline) = deadline {
        tokio::select! {
            () = cancellation.cancelled() => {} () =
            tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)) => {
            cancellation.cancel(); }
        }
    } else {
        cancellation.cancelled().await;
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_cancel_operation<F: Future>(
    guard: &TerraneFinallyGuard,
    future: F,
) -> Option<F::Output> {
    let cancellation = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            guard
                .state
                .as_ref()
                .map(|state| {
                    (context.cancellation.clone(), context.deadline, state.clone())
                })
        })
        .ok()
        .flatten();
    if let Some((cancellation, deadline, finalizers)) = cancellation {
        tokio::select! {
            biased; output = future => Some(output), () = async {
            __terrane_cancellation_requested(cancellation, deadline). await; finalizers
            .reaches(guard.depth). await; } => None,
        }
    } else {
        Some(future.await)
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_finish_cancelled_finally(mut guard: TerraneFinallyGuard) -> ! {
    guard.finish();
    std::future::pending().await
}
async fn __terrane_await<F: Future>(future: F) -> F::Output {
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            context: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            if self.0 {
                std::task::Poll::Ready(())
            } else {
                self.0 = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
    YieldOnce(false).await;
    let output = future.await;
    YieldOnce(false).await;
    output
}
async fn __terrane_cancellable<F: Future>(
    future: F,
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
) -> Option<F::Output> {
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline,
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async {
                tokio::select! {
                    biased; output = future => Some(output), () = async {
                    __terrane_cancellation_requested(cancellation, deadline). await;
                    finalizers.reaches(0). await; } => None,
                }
            },
        )
        .await
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    tokio::task::LocalSet::new()
        .block_on(
            &runtime,
            async move {
                let output = future.await;
                __terrane_wait_projected_cleanups().await;
                output
            },
        )
}
async fn __terrane_dependency_await_unwind<F: Future>(
    future: F,
) -> Result<F::Output, Box<dyn std::any::Any + Send>> {
    let mut future = std::pin::pin!(future);
    std::future::poll_fn(|context| {
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| { future.as_mut().poll(context) }),
            ) {
                Ok(std::task::Poll::Ready(value)) => std::task::Poll::Ready(Ok(value)),
                Ok(std::task::Poll::Pending) => std::task::Poll::Pending,
                Err(payload) => std::task::Poll::Ready(Err(payload)),
            }
        })
        .await
}
#[derive(Clone)]
pub struct TerraneTaskScope {
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
}
impl TerraneTaskScope {
    pub fn new(deadline_ms: Option<u64>) -> Self {
        Self {
            cancellation: TerraneCancellation::new(),
            deadline: deadline_ms
                .map(|milliseconds| {
                    std::time::Instant::now()
                        + std::time::Duration::from_millis(milliseconds)
                }),
        }
    }
    pub fn child_scope(&self, deadline_ms: u64) -> Self {
        let requested = std::time::Instant::now()
            + std::time::Duration::from_millis(deadline_ms);
        Self {
            cancellation: self.cancellation.clone(),
            deadline: Some(
                self.deadline.map_or(requested, |parent| parent.min(requested)),
            ),
        }
    }
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }
    pub fn should_cancel(&self) -> bool {
        self.cancellation.is_cancelled()
            || self
                .deadline
                .is_some_and(|deadline| std::time::Instant::now() >= deadline)
    }
    fn cancellation(&self) -> TerraneCancellation {
        self.cancellation.clone()
    }
    pub async fn join<T>(
        &self,
        mut task: TerraneScopedTask<T>,
    ) -> TerraneTaskOutcome<T> {
        let result = task
            .handle
            .take()
            .expect("scoped task joined once")
            .await
            .expect("scoped task must not panic outside its Terrane boundary");
        outcome_from_result(self, result)
    }
}
#[allow(
    dead_code,
    reason = "task result ABI is emitted before per-variant usage shaping"
)]
enum TerraneTaskResult<T> {
    Completed(T),
    Failed(TerraneError),
    Cancelled,
}
pub struct TerraneScopedTask<T> {
    handle: Option<tokio::task::JoinHandle<TerraneTaskResult<T>>>,
}
impl<T: 'static> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: Future<Output = TerraneTaskResult<T>> + 'static>(work: F) -> Self {
        Self {
            handle: Some(tokio::task::spawn_local(work)),
        }
    }
}
fn outcome_from_result<T>(
    scope: &TerraneTaskScope,
    result: TerraneTaskResult<T>,
) -> TerraneTaskOutcome<T> {
    match result {
        TerraneTaskResult::Completed(value) => {
            TerraneTaskOutcome {
                completed: true,
                cancelled: scope.should_cancel(),
                value: Some(value),
                error: None,
            }
        }
        TerraneTaskResult::Failed(error) => {
            scope.cancel();
            TerraneTaskOutcome {
                completed: false,
                cancelled: false,
                value: None,
                error: Some(error),
            }
        }
        TerraneTaskResult::Cancelled => {
            TerraneTaskOutcome {
                completed: false,
                cancelled: true,
                value: None,
                error: None,
            }
        }
    }
}
pub struct TerraneTaskOutcome<T> {
    pub completed: bool,
    pub cancelled: bool,
    pub value: Option<T>,
    pub error: Option<TerraneError>,
}
// Source: src/main.trn
// Namespace: app
fn add_two(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(2_i128);
}
fn keep_string(value: String) -> String {
    return value;
}
async fn keep_string_async(value: String) -> String {
    return value;
}
async fn stay_pending(value: terrane_int_support::Int) -> terrane_int_support::Int {
    let result: terrane_int_support::Int = __terrane_traced(
        __terrane_await({
                let __terrane_future = pending_value(value.clone());
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:20:24-20:44 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:20:24-20:44 */,
    );
    return result.clone();
}
async fn run_retained() -> terrane_int_support::Int {
    let __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = async {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = async {
            return TerraneCompletion::Return(
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future =
                    invoke_retained(std::sync::Arc::new(move | argument_0 :
                    terrane_int_support::Int | -> std::pin::Pin < Box < dyn Future <
                    Output = _ > + Send >> { Box::pin(stay_pending(argument_0)) }));
                    async move { __terrane_raised_err(__terrane_future. await,
                    1 /* terrane-site: src/main.trn:25:22-25:51 */) } }). await,
                    1 /* terrane-site: src/main.trn:25:22-25:51 */
                ),
            );
        }
            .await;
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_0 = true;
                    return TerraneCompletion::Return(
                        terrane_int_support::Int::from(0_i128),
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    }
        .await;
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn render(label: String, enabled: bool) -> String {
    if enabled {
        return terrane_string_support::upper(&label);
    }
    return label;
}
#[derive(Clone)]
pub struct ProjectedMessage {}
impl ProjectedMessage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(&self, label: String) -> String {
        return terrane_string_support::upper(&label);
    }
}
impl RenderableProtocol for ProjectedMessage {
    fn clone_box(&self) -> Box<dyn RenderableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RenderableProtocol> {
        Box::new(self.clone())
    }
    fn render(&self, label_: String) -> String {
        ProjectedMessage::render(&*self, label_)
    }
    fn decorated(&self, label_: String) -> String {
        || -> Result<String, crate::TerraneForeignError> {
            let __terrane_default = <ProjectedMessage as terrane_render_witness::Renderable>::decorated(
                &*self,
                label_,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn borrowed(&self, label_: String) -> String {
        || -> Result<String, crate::TerraneForeignError> {
            let __terrane_default = <ProjectedMessage as terrane_render_witness::Renderable>::borrowed(
                &*self,
                &label_,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn parsed(&self, text: String) -> Result<terrane_int_support::Int, TerraneError> {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = match <ProjectedMessage as terrane_render_witness::Renderable>::parsed(
                &*self,
                text,
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_render_witness::Renderable` member `parsed` failed: {error}"
                                ),
                                crate::TERRANE_NO_SITE,
                            ),
                        ),
                    );
                }
            };
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .map_err(|error| error.raised(crate::TERRANE_NO_SITE))
    }
}
impl From<ProjectedMessage> for Renderable {
    fn from(value: ProjectedMessage) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_render_witness::Renderable for ProjectedMessage {
    fn render(&self, label_: String) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedMessage>::render(&*self, label_);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct ProjectedCounter {
    pub total: terrane_int_support::Int,
}
impl ProjectedCounter {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
    pub fn current(&self) -> terrane_int_support::Int {
        return self.total.clone();
    }
}
impl AdjustableProtocol for ProjectedCounter {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn AdjustableProtocol> {
        Box::new(self.clone())
    }
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int {
        ProjectedCounter::adjust(&mut *self, delta)
    }
    fn current(&self) -> terrane_int_support::Int {
        ProjectedCounter::current(&*self)
    }
}
impl From<ProjectedCounter> for Adjustable {
    fn from(value: ProjectedCounter) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_callback_witness::Adjustable for ProjectedCounter {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedCounter>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedCounter>::current(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
#[derive(Clone)]
pub struct ProjectedLocalCounter {
    pub total: terrane_int_support::Int,
}
impl ProjectedLocalCounter {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
    pub fn current(&self) -> terrane_int_support::Int {
        return self.total.clone();
    }
}
impl LocalAdjustableProtocol for ProjectedLocalCounter {
    fn clone_box(&self) -> Box<dyn LocalAdjustableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LocalAdjustableProtocol> {
        Box::new(self.clone())
    }
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int {
        ProjectedLocalCounter::adjust(&mut *self, delta)
    }
    fn current(&self) -> terrane_int_support::Int {
        ProjectedLocalCounter::current(&*self)
    }
}
impl From<ProjectedLocalCounter> for LocalAdjustable {
    fn from(value: ProjectedLocalCounter) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_callback_witness::LocalAdjustable for ProjectedLocalCounter {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedLocalCounter>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <ProjectedLocalCounter>::current(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub struct DropAwareValue {
    pub token: DropToken,
}
impl DropAwareValue {
    pub fn terrane_construct() -> Self {
        Self {
            token: __terrane_raised(
                drop_token(),
                2 /* terrane-site: src/main.trn:60:24-60:40 */,
            ),
        }
    }
    pub fn label(&self) -> String {
        return String::from("drop-aware");
    }
    pub fn destruct(&mut self) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("drop-aware-destruct"))
        );
    }
}
impl terrane_callback_witness::DropAware for DropAwareValue {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DropAwareValue>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl Drop for DropAwareValue {
    fn drop(&mut self) {
        self.destruct();
    }
}
#[derive(Clone)]
pub struct ProjectedAsyncEntry {
    __terrane_lifetime: std::sync::Arc<()>,
}
impl ProjectedAsyncEntry {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub async fn shared(&self) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_1 = __terrane_finally_guard();
        let __terrane_maybe_completion_1: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_1,
                async {
                    let __terrane_try_1: TerraneCompletion<terrane_int_support::Int> = async {
                        let mut __terrane_finally_guard_2 = __terrane_finally_guard();
                        let __terrane_maybe_completion_2: Option<
                            TerraneCompletion<terrane_int_support::Int>,
                        > = __terrane_cancel_operation(
                                &__terrane_finally_guard_2,
                                async {
                                    let __terrane_try_2: TerraneCompletion<
                                        terrane_int_support::Int,
                                    > = async {
                                        return TerraneCompletion::Return(
                                            __terrane_traced_completion!(
                                                __terrane_await({ let __terrane_future =
                                                pending_value(terrane_int_support::Int::from(0_i128)); async
                                                move { __terrane_raised_err(__terrane_future. await,
                                                3 /* terrane-site: src/main.trn:72:30-72:46 */) } }).
                                                await, 3 /* terrane-site: src/main.trn:72:30-72:46 */
                                            ),
                                        );
                                    }
                                        .await;
                                    match __terrane_try_2 {
                                        TerraneCompletion::Return(value) => {
                                            return TerraneCompletion::Return(value);
                                        }
                                        TerraneCompletion::Break => return TerraneCompletion::Break,
                                        TerraneCompletion::Continue => {
                                            return TerraneCompletion::Continue;
                                        }
                                        TerraneCompletion::Normal => {}
                                        TerraneCompletion::Error(__terrane_error_2) => {
                                            let mut __terrane_handled_2 = false;
                                            if !__terrane_handled_2 {
                                                return TerraneCompletion::Error(__terrane_error_2);
                                            }
                                        }
                                    }
                                    TerraneCompletion::Normal
                                },
                            )
                            .await;
                        let __terrane_cancelled_2 = __terrane_maybe_completion_2
                            .is_none();
                        let mut __terrane_completion_2 = __terrane_maybe_completion_2
                            .unwrap_or(TerraneCompletion::Normal);
                        let __terrane_finally_2: TerraneCompletion<
                            terrane_int_support::Int,
                        > = (|| {
                            __terrane_raised_completion!(
                                record_async_entry_cleanup(), 4 /* terrane-site: src/main.trn:74:17-74:44 */
                            );
                            TerraneCompletion::Normal
                        })();
                        match __terrane_finally_2 {
                            TerraneCompletion::Normal => {}
                            replacement => __terrane_completion_2 = replacement,
                        }
                        if __terrane_cancelled_2
                            && matches!(
                                &__terrane_completion_2, TerraneCompletion::Normal
                            )
                        {
                            __terrane_finish_cancelled_finally(__terrane_finally_guard_2)
                                .await;
                        }
                        __terrane_finally_guard_2.finish();
                        match __terrane_completion_2 {
                            TerraneCompletion::Normal => {
                                __terrane_generated_defect(
                                    "non-fallthrough try completed normally",
                                )
                            }
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Error(error) => {
                                return TerraneCompletion::Error(error);
                            }
                            TerraneCompletion::Break | TerraneCompletion::Continue => {
                                __terrane_generated_defect(
                                    "loop control escaped a non-loop try",
                                )
                            }
                        }
                    }
                        .await;
                    match __terrane_try_1 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_1) => {
                            let mut __terrane_handled_1 = false;
                            if !__terrane_handled_1 {
                                return TerraneCompletion::Error(__terrane_error_1);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_1 = __terrane_maybe_completion_1.is_none();
        let mut __terrane_completion_1 = __terrane_maybe_completion_1
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_1: TerraneCompletion<terrane_int_support::Int> = (|| {
            __terrane_raised_completion!(
                record_async_entry_cleanup(), 5 /* terrane-site: src/main.trn:76:13-76:40 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_1 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_1 = replacement,
        }
        if __terrane_cancelled_1
            && matches!(&__terrane_completion_1, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_1).await;
        }
        __terrane_finally_guard_1.finish();
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {
                __terrane_generated_defect("non-fallthrough try completed normally")
            }
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    }
    pub async fn mutable(&mut self) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_3 = __terrane_finally_guard();
        let __terrane_maybe_completion_3: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_3,
                async {
                    let __terrane_try_3: TerraneCompletion<terrane_int_support::Int> = async {
                        let mut __terrane_finally_guard_4 = __terrane_finally_guard();
                        let __terrane_maybe_completion_4: Option<
                            TerraneCompletion<terrane_int_support::Int>,
                        > = __terrane_cancel_operation(
                                &__terrane_finally_guard_4,
                                async {
                                    let __terrane_try_4: TerraneCompletion<
                                        terrane_int_support::Int,
                                    > = async {
                                        return TerraneCompletion::Return(
                                            __terrane_traced_completion!(
                                                __terrane_await({ let __terrane_future =
                                                pending_value(terrane_int_support::Int::from(0_i128)); async
                                                move { __terrane_raised_err(__terrane_future. await,
                                                6 /* terrane-site: src/main.trn:81:30-81:46 */) } }).
                                                await, 6 /* terrane-site: src/main.trn:81:30-81:46 */
                                            ),
                                        );
                                    }
                                        .await;
                                    match __terrane_try_4 {
                                        TerraneCompletion::Return(value) => {
                                            return TerraneCompletion::Return(value);
                                        }
                                        TerraneCompletion::Break => return TerraneCompletion::Break,
                                        TerraneCompletion::Continue => {
                                            return TerraneCompletion::Continue;
                                        }
                                        TerraneCompletion::Normal => {}
                                        TerraneCompletion::Error(__terrane_error_4) => {
                                            let mut __terrane_handled_4 = false;
                                            if !__terrane_handled_4 {
                                                return TerraneCompletion::Error(__terrane_error_4);
                                            }
                                        }
                                    }
                                    TerraneCompletion::Normal
                                },
                            )
                            .await;
                        let __terrane_cancelled_4 = __terrane_maybe_completion_4
                            .is_none();
                        let mut __terrane_completion_4 = __terrane_maybe_completion_4
                            .unwrap_or(TerraneCompletion::Normal);
                        let __terrane_finally_4: TerraneCompletion<
                            terrane_int_support::Int,
                        > = (|| {
                            __terrane_raised_completion!(
                                record_async_entry_cleanup(), 7 /* terrane-site: src/main.trn:83:17-83:44 */
                            );
                            TerraneCompletion::Normal
                        })();
                        match __terrane_finally_4 {
                            TerraneCompletion::Normal => {}
                            replacement => __terrane_completion_4 = replacement,
                        }
                        if __terrane_cancelled_4
                            && matches!(
                                &__terrane_completion_4, TerraneCompletion::Normal
                            )
                        {
                            __terrane_finish_cancelled_finally(__terrane_finally_guard_4)
                                .await;
                        }
                        __terrane_finally_guard_4.finish();
                        match __terrane_completion_4 {
                            TerraneCompletion::Normal => {
                                __terrane_generated_defect(
                                    "non-fallthrough try completed normally",
                                )
                            }
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Error(error) => {
                                return TerraneCompletion::Error(error);
                            }
                            TerraneCompletion::Break | TerraneCompletion::Continue => {
                                __terrane_generated_defect(
                                    "loop control escaped a non-loop try",
                                )
                            }
                        }
                    }
                        .await;
                    match __terrane_try_3 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_3) => {
                            let mut __terrane_handled_3 = false;
                            if !__terrane_handled_3 {
                                return TerraneCompletion::Error(__terrane_error_3);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_3 = __terrane_maybe_completion_3.is_none();
        let mut __terrane_completion_3 = __terrane_maybe_completion_3
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_3: TerraneCompletion<terrane_int_support::Int> = (|| {
            __terrane_raised_completion!(
                record_async_entry_cleanup(), 8 /* terrane-site: src/main.trn:85:13-85:40 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_3 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_3 = replacement,
        }
        if __terrane_cancelled_3
            && matches!(&__terrane_completion_3, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_3).await;
        }
        __terrane_finally_guard_3.finish();
        match __terrane_completion_3 {
            TerraneCompletion::Normal => {
                __terrane_generated_defect("non-fallthrough try completed normally")
            }
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    }
    pub async fn consuming(self) -> terrane_int_support::Int {
        let mut __terrane_finally_guard_5 = __terrane_finally_guard();
        let __terrane_maybe_completion_5: Option<
            TerraneCompletion<terrane_int_support::Int>,
        > = __terrane_cancel_operation(
                &__terrane_finally_guard_5,
                async {
                    let __terrane_try_5: TerraneCompletion<terrane_int_support::Int> = async {
                        let mut __terrane_finally_guard_6 = __terrane_finally_guard();
                        let __terrane_maybe_completion_6: Option<
                            TerraneCompletion<terrane_int_support::Int>,
                        > = __terrane_cancel_operation(
                                &__terrane_finally_guard_6,
                                async {
                                    let __terrane_try_6: TerraneCompletion<
                                        terrane_int_support::Int,
                                    > = async {
                                        return TerraneCompletion::Return(
                                            __terrane_traced_completion!(
                                                __terrane_await({ let __terrane_future =
                                                pending_value(terrane_int_support::Int::from(0_i128)); async
                                                move { __terrane_raised_err(__terrane_future. await,
                                                9 /* terrane-site: src/main.trn:90:30-90:46 */) } }).
                                                await, 9 /* terrane-site: src/main.trn:90:30-90:46 */
                                            ),
                                        );
                                    }
                                        .await;
                                    match __terrane_try_6 {
                                        TerraneCompletion::Return(value) => {
                                            return TerraneCompletion::Return(value);
                                        }
                                        TerraneCompletion::Break => return TerraneCompletion::Break,
                                        TerraneCompletion::Continue => {
                                            return TerraneCompletion::Continue;
                                        }
                                        TerraneCompletion::Normal => {}
                                        TerraneCompletion::Error(__terrane_error_6) => {
                                            let mut __terrane_handled_6 = false;
                                            if !__terrane_handled_6 {
                                                return TerraneCompletion::Error(__terrane_error_6);
                                            }
                                        }
                                    }
                                    TerraneCompletion::Normal
                                },
                            )
                            .await;
                        let __terrane_cancelled_6 = __terrane_maybe_completion_6
                            .is_none();
                        let mut __terrane_completion_6 = __terrane_maybe_completion_6
                            .unwrap_or(TerraneCompletion::Normal);
                        let __terrane_finally_6: TerraneCompletion<
                            terrane_int_support::Int,
                        > = (|| {
                            __terrane_raised_completion!(
                                record_async_entry_cleanup(), 10 /* terrane-site: src/main.trn:92:17-92:44 */
                            );
                            TerraneCompletion::Normal
                        })();
                        match __terrane_finally_6 {
                            TerraneCompletion::Normal => {}
                            replacement => __terrane_completion_6 = replacement,
                        }
                        if __terrane_cancelled_6
                            && matches!(
                                &__terrane_completion_6, TerraneCompletion::Normal
                            )
                        {
                            __terrane_finish_cancelled_finally(__terrane_finally_guard_6)
                                .await;
                        }
                        __terrane_finally_guard_6.finish();
                        match __terrane_completion_6 {
                            TerraneCompletion::Normal => {
                                __terrane_generated_defect(
                                    "non-fallthrough try completed normally",
                                )
                            }
                            TerraneCompletion::Return(value) => {
                                return TerraneCompletion::Return(value);
                            }
                            TerraneCompletion::Error(error) => {
                                return TerraneCompletion::Error(error);
                            }
                            TerraneCompletion::Break | TerraneCompletion::Continue => {
                                __terrane_generated_defect(
                                    "loop control escaped a non-loop try",
                                )
                            }
                        }
                    }
                        .await;
                    match __terrane_try_5 {
                        TerraneCompletion::Return(value) => {
                            return TerraneCompletion::Return(value);
                        }
                        TerraneCompletion::Break => return TerraneCompletion::Break,
                        TerraneCompletion::Continue => return TerraneCompletion::Continue,
                        TerraneCompletion::Normal => {}
                        TerraneCompletion::Error(__terrane_error_5) => {
                            let mut __terrane_handled_5 = false;
                            if !__terrane_handled_5 {
                                return TerraneCompletion::Error(__terrane_error_5);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_5 = __terrane_maybe_completion_5.is_none();
        let mut __terrane_completion_5 = __terrane_maybe_completion_5
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_5: TerraneCompletion<terrane_int_support::Int> = (|| {
            __terrane_raised_completion!(
                record_async_entry_cleanup(), 11 /* terrane-site: src/main.trn:94:13-94:40 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_5 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_5 = replacement,
        }
        if __terrane_cancelled_5
            && matches!(&__terrane_completion_5, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_5).await;
        }
        __terrane_finally_guard_5.finish();
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {
                __terrane_generated_defect("non-fallthrough try completed normally")
            }
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    }
    pub fn destruct(&mut self) {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(async_entry_cleanup_count(),
            12 /* terrane-site: src/main.trn:97:17-97:43 */))
        );
    }
}
impl AsyncEntryProtocol for ProjectedAsyncEntry {
    fn clone_box(&self) -> Box<dyn AsyncEntryProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn AsyncEntryProtocol> {
        Box::new(self.terrane_separate())
    }
    fn shared(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    > {
        Box::pin(async move { ProjectedAsyncEntry::shared(&*self).await })
    }
    fn mutable(
        &mut self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    > {
        Box::pin(async move { ProjectedAsyncEntry::mutable(&mut *self).await })
    }
    fn consuming(
        self: Box<Self>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + 'static>,
    > {
        Box::pin(async move { ProjectedAsyncEntry::consuming(*self).await })
    }
}
impl From<ProjectedAsyncEntry> for AsyncEntry {
    fn from(value: ProjectedAsyncEntry) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_callback_witness::AsyncEntry for ProjectedAsyncEntry {
    async fn shared(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let __terrane_receiver = self.clone();
                __terrane_projected_async_entry(async move {
                        <ProjectedAsyncEntry>::shared(&__terrane_receiver).await
                    })
                    .await
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn mutable(&mut self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let mut __terrane_receiver = self.clone();
                let (__terrane_output, __terrane_receiver) = __terrane_projected_async_entry(async move {
                        let __terrane_output = <ProjectedAsyncEntry>::mutable(
                                &mut __terrane_receiver,
                            )
                            .await;
                        (__terrane_output, __terrane_receiver)
                    })
                    .await;
                *self = __terrane_receiver;
                __terrane_output
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn consuming(self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = __terrane_projected_async_entry(async move {
                    <ProjectedAsyncEntry>::consuming(self).await
                })
                .await;
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl Drop for ProjectedAsyncEntry {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
#[derive(Clone)]
pub struct ProjectedCounterHolder {
    pub inner: ProjectedCounter,
}
impl ProjectedCounterHolder {
    pub fn terrane_construct() -> Self {
        Self {
            inner: ProjectedCounter::terrane_construct(),
        }
    }
}
fn main() {
    __terrane_run(async move {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(apply_shared(terrane_int_support::Int::from(40_i128),
            std::sync::Arc::new(add_two)), 13 /* terrane-site: src/main.trn:103:13-103:38 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(apply_mutable(terrane_int_support::Int::from(10_i128),
            TerraneMutableCallable::new(move | (argument_0,) :
            (terrane_int_support::Int,) | add_two(argument_0))), 14 /* terrane-site: src/main.trn:104:13-104:39 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(apply_once(String::from("HELLO"),
            TerraneConsumingCallable::new(move | (argument_0,) : (String,) |
            keep_string(argument_0))), 15 /* terrane-site: src/main.trn:105:13-105:45 */))
        );
        let changed: String = __terrane_traced(
            __terrane_await({
                    let __terrane_future = apply_async(
                        String::from("hello"),
                        std::sync::Arc::new(move |
                            argument_0: String,
                        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                            Box::pin(keep_string_async(argument_0))
                        }),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            16 /* terrane-site: src/main.trn:106:28-106:67 */,
                        )
                    }
                })
                .await,
            16 /* terrane-site: src/main.trn:106:28-106:67 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&changed));
        let offset: i64 = 10;
        let add_offset: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        > = {
            let offset = offset.clone();
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let offset = offset.clone();
                Box::pin(async move {
                    return value.clone()
                        + terrane_int_support::Int::from(offset as i128);
                })
            })
        };
        let concurrent: terrane_int_support::Int = __terrane_traced(
            __terrane_await({
                    let __terrane_future = apply_async_concurrently(
                        terrane_int_support::Int::from(20_i128),
                        add_offset.clone(),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            17 /* terrane-site: src/main.trn:111:28-111:68 */,
                        )
                    }
                })
                .await,
            17 /* terrane-site: src/main.trn:111:28-111:68 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&concurrent));
        let callback_registry: Registrar = __terrane_raised(
            registrar(),
            18 /* terrane-site: src/main.trn:113:25-113:35 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | callback_registry
            .apply(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(5_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }, match | | -> Result < _, crate
            ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(add_two)
            .clone(); move | callback_argument_0 : i64 | { match | | -> Result < _, crate
            ::TerraneForeignError > { let callback_value =
            callback(terrane_int_support::Int::from(i128::from(callback_argument_0)));
            Ok(terrane_int_support::coerce:: < i64 > (&callback_value).map_err(| error |
            crate ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error)
            => std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::Registrar::apply")) }, 19 /* terrane-site: src/main.trn:114:13-114:48 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(dispatch(String::from("dissimilar"),
            true, std::sync::Arc::new(render)), 20 /* terrane-site: src/main.trn:116:13-116:49 */))
        );
        let retained_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let retained_child: TerraneScopedTask<terrane_int_support::Int> = {
            let __terrane_scope = retained_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(run_retained()) })(),
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        let active: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = wait_until_retained_invocation_active();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            21 /* terrane-site: src/main.trn:119:25-119:63 */,
                        )
                    }
                })
                .await,
            21 /* terrane-site: src/main.trn:119:25-119:63 */,
        );
        retained_scope.cancel();
        let retained_outcome: TerraneTaskOutcome<terrane_int_support::Int> = __terrane_await(
                retained_scope.join(retained_child),
            )
            .await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&active),
            terrane_scalar_support::scalar_text(&retained_outcome.cancelled),
            terrane_scalar_support::scalar_text(&__terrane_raised(active_retained_invocations(),
            22 /* terrane-site: src/main.trn:122:49-122:77 */))
        );
        let message: ProjectedMessage = ProjectedMessage::terrane_construct();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(render_value(&message,
            String::from("interface")), 23 /* terrane-site: src/main.trn:124:13-124:47 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(render_decorated(&message,
            String::from("default")), 24 /* terrane-site: src/main.trn:125:13-125:49 */))
        );
        let interface_message: Renderable = <Renderable>::from(message.clone());
        println!(
            "{}", terrane_scalar_support::scalar_text(&interface_message
            .decorated(String::from("provided")))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&interface_message
            .borrowed(String::from("borrowed")))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(interface_message
            .parsed(String::from("7")), 25 /* terrane-site: src/main.trn:129:13-129:42 */))
        );
        let impl_message: ProjectedMessage = ProjectedMessage::terrane_construct();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(render_impl(&impl_message,
            String::from("opaque")), 26 /* terrane-site: src/main.trn:131:13-131:48 */))
        );
        let mut counter: ProjectedCounter = ProjectedCounter::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(adjust_value(&mut
            counter, terrane_int_support::Int::from(7_i128)), 27 /* terrane-site: src/main.trn:133:13-133:37 */))
        );
        let mut holder: ProjectedCounterHolder = ProjectedCounterHolder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(adjust_value(&mut
            holder.inner, terrane_int_support::Int::from(9_i128)), 28 /* terrane-site: src/main.trn:135:13-135:42 */))
        );
        let mut retained_counter: AdjustableOwner = __terrane_raised(
            retain_adjustable(counter),
            29 /* terrane-site: src/main.trn:136:24-136:50 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | retained_counter
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(5_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::adjust")) },
            30 /* terrane-site: src/main.trn:137:13-137:39 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | retained_counter
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::current")) },
            31 /* terrane-site: src/main.trn:137:43-137:68 */))
        );
        let boxed_counter: ProjectedCounter = ProjectedCounter::terrane_construct();
        let mut boxed_owner: AdjustableOwner = __terrane_raised(
            retain_boxed_adjustable(boxed_counter),
            32 /* terrane-site: src/main.trn:139:19-139:57 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | boxed_owner
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(11_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::adjust")) },
            33 /* terrane-site: src/main.trn:140:13-140:35 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | boxed_owner
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::current")) },
            34 /* terrane-site: src/main.trn:140:39-140:59 */))
        );
        let erased_boxed: Adjustable = <Adjustable>::from(
            ProjectedCounter::terrane_construct(),
        );
        let mut erased_owner: AdjustableOwner = __terrane_raised(
            retain_boxed_adjustable(erased_boxed),
            35 /* terrane-site: src/main.trn:142:20-142:57 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | erased_owner
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(4_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::adjust")) },
            36 /* terrane-site: src/main.trn:143:13-143:35 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | erased_owner
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::AdjustableOwner::current")) },
            37 /* terrane-site: src/main.trn:143:39-143:60 */))
        );
        let mut local_owner: LocalAdjustableOwner = __terrane_raised(
            retain_local_adjustable(ProjectedLocalCounter::terrane_construct()),
            38 /* terrane-site: src/main.trn:144:19-144:79 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | local_owner
            .adjust(match | | -> Result < _, crate ::TerraneForeignError > {
            Ok(terrane_int_support::coerce:: < i64 >
            (&terrane_int_support::Int::from(3_i128)).map_err(| error | crate
            ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
            ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
            std::panic::panic_any(error) }))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::LocalAdjustableOwner::adjust")) },
            39 /* terrane-site: src/main.trn:145:13-145:34 */)),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | local_owner
            .current())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "terrane_callback_witness",
            "terrane_callback_witness::LocalAdjustableOwner::current")) },
            40 /* terrane-site: src/main.trn:145:38-145:58 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(consume_drop(DropAwareValue::terrane_construct()),
            41 /* terrane-site: src/main.trn:146:13-146:55 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future =
            cancel_async_entry(ProjectedAsyncEntry::terrane_construct()); async move {
            __terrane_raised_err(__terrane_future. await, 42 /* terrane-site: src/main.trn:147:19-147:72 */) } }). await, 42 /* terrane-site: src/main.trn:147:19-147:72 */))
        );
    });
}
// Source: <terrane>/projected/deps/terrane-callback-witness.trn
// Namespace: deps/terrane-callback-witness
pub trait AdjustableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol>;
    fn separate_box(&self) -> Box<dyn AdjustableProtocol>;
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int;
    fn current(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn AdjustableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Adjustable(Box<dyn AdjustableProtocol>);
impl Clone for Adjustable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Adjustable {
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.0.adjust(delta)
    }
    pub fn current(&self) -> terrane_int_support::Int {
        self.0.current()
    }
}
impl terrane_callback_witness::Adjustable for Adjustable {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Adjustable>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Adjustable>::current(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait AsyncEntryProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn AsyncEntryProtocol>;
    fn separate_box(&self) -> Box<dyn AsyncEntryProtocol>;
    fn shared(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    >;
    fn mutable(
        &mut self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    >;
    fn consuming(
        self: Box<Self>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + 'static>,
    >;
}
impl Clone for Box<dyn AsyncEntryProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct AsyncEntry(Box<dyn AsyncEntryProtocol>);
impl Clone for AsyncEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl AsyncEntry {
    pub async fn shared(&self) -> terrane_int_support::Int {
        self.0.shared().await
    }
    pub async fn mutable(&mut self) -> terrane_int_support::Int {
        self.0.mutable().await
    }
    pub async fn consuming(self) -> terrane_int_support::Int {
        self.0.consuming().await
    }
}
impl terrane_callback_witness::AsyncEntry for AsyncEntry {
    async fn shared(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let __terrane_receiver = self.clone();
                __terrane_projected_async_entry(async move {
                        <AsyncEntry>::shared(&__terrane_receiver).await
                    })
                    .await
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn mutable(&mut self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let mut __terrane_receiver = self.clone();
                let (__terrane_output, __terrane_receiver) = __terrane_projected_async_entry(async move {
                        let __terrane_output = <AsyncEntry>::mutable(
                                &mut __terrane_receiver,
                            )
                            .await;
                        (__terrane_output, __terrane_receiver)
                    })
                    .await;
                *self = __terrane_receiver;
                __terrane_output
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn consuming(self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = __terrane_projected_async_entry(async move {
                    <AsyncEntry>::consuming(self).await
                })
                .await;
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait DropAwareProtocol {
    fn clone_box(&self) -> Box<dyn DropAwareProtocol>;
    fn separate_box(&self) -> Box<dyn DropAwareProtocol>;
    fn label(&self) -> String;
}
impl Clone for Box<dyn DropAwareProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct DropAware(Box<dyn DropAwareProtocol>);
impl Clone for DropAware {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Drop for DropAware {
    fn drop(&mut self) {}
}
impl DropAware {
    pub fn label(&self) -> String {
        self.0.label()
    }
}
impl terrane_callback_witness::DropAware for DropAware {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DropAware>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait LocalAdjustableProtocol {
    fn clone_box(&self) -> Box<dyn LocalAdjustableProtocol>;
    fn separate_box(&self) -> Box<dyn LocalAdjustableProtocol>;
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int;
    fn current(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn LocalAdjustableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct LocalAdjustable(Box<dyn LocalAdjustableProtocol>);
impl Clone for LocalAdjustable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl LocalAdjustable {
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.0.adjust(delta)
    }
    pub fn current(&self) -> terrane_int_support::Int {
        self.0.current()
    }
}
impl terrane_callback_witness::LocalAdjustable for LocalAdjustable {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <LocalAdjustable>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <LocalAdjustable>::current(&*self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub use terrane_callback_witness::AdjustableOwner;
pub use terrane_callback_witness::DropToken;
pub use terrane_callback_witness::LocalAdjustableOwner;
pub use terrane_callback_witness::Registrar;
pub fn active_retained_invocations() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::active_retained_invocations()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::active_retained_invocations",
                ),
            )
        }
    }
}
pub fn adjust_value<T: terrane_callback_witness::Adjustable>(
    value: &mut T,
    delta: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let delta = terrane_int_support::coerce::<i64>(&delta)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::adjust_value(
            value,
            delta,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::adjust_value",
                ),
            )
        }
    }
}
pub async fn apply_async(
    value: String,
    callback: std::sync::Arc<
        dyn Fn(
            String,
        ) -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync,
    >,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: String| {
            let callback = callback.clone();
            let callback_future = callback(callback_argument_0);
            Box::pin(async move {
                match async {
                    let callback_value = callback_future.await;
                    Ok::<_, crate::TerraneForeignError>(callback_value)
                }
                    .await
                {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error.0),
                }
            })
        }
    };
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::apply_async(value, callback),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_async",
                ),
            )
        }
    }
}
pub async fn apply_async_concurrently(
    value: terrane_int_support::Int,
    callback: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            let callback = callback.clone();
            let callback_future = callback(
                terrane_int_support::Int::from(i128::from(callback_argument_0)),
            );
            Box::pin(async move {
                match async {
                    let callback_value = callback_future.await;
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(
                        terrane_int_support::coerce::<i64>(&callback_value)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }
                    .await
                {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error.0),
                }
            })
        }
    };
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::apply_async_concurrently(value, callback),
        )
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_async_concurrently",
                ),
            )
        }
    }
}
pub fn apply_mutable(
    value: terrane_int_support::Int,
    callback: TerraneMutableCallable<
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback
                    .call((
                        terrane_int_support::Int::from(i128::from(callback_argument_0)),
                    ));
                Ok(
                    terrane_int_support::coerce::<i64>(&callback_value)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::apply_mutable(
            value,
            callback,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_mutable",
                ),
            )
        }
    }
}
pub fn apply_once(
    value: String,
    callback: TerraneConsumingCallable<(String,), String>,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    let callback = {
        let callback = callback;
        move |callback_argument_0: String| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((callback_argument_0,));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::apply_once(
            value,
            callback,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_once",
                ),
            )
        }
    }
}
pub fn apply_shared(
    value: terrane_int_support::Int,
    callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback(
                    terrane_int_support::Int::from(i128::from(callback_argument_0)),
                );
                Ok(
                    terrane_int_support::coerce::<i64>(&callback_value)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::apply_shared(
            value,
            callback,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_shared",
                ),
            )
        }
    }
}
pub fn async_entry_cleanup_count() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::async_entry_cleanup_count()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::async_entry_cleanup_count",
                ),
            )
        }
    }
}
pub async fn cancel_async_entry<T: 'static + terrane_callback_witness::AsyncEntry>(
    value: T,
) -> Result<bool, crate::TerraneForeignError> {
    let value = value;
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::cancel_async_entry(value),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::cancel_async_entry",
                ),
            )
        }
    }
}
pub fn consume_drop<T: 'static + terrane_callback_witness::DropAware>(
    value: T,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::consume_drop(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::consume_drop",
                ),
            )
        }
    }
}
pub fn drop_token() -> Result<DropToken, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::drop_token()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::drop_token",
                ),
            )
        }
    }
}
pub async fn invoke_retained(
    callback: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            let callback = callback.clone();
            let callback_future = callback(
                terrane_int_support::Int::from(i128::from(callback_argument_0)),
            );
            Box::pin(async move {
                match async {
                    let callback_value = callback_future.await;
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(
                        terrane_int_support::coerce::<i64>(&callback_value)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }
                    .await
                {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error.0),
                }
            })
        }
    };
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::invoke_retained(callback),
        )
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::invoke_retained",
                ),
            )
        }
    }
}
pub async fn pending_value(
    __trn_5f76616c7565: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let __trn_5f76616c7565 = terrane_int_support::coerce::<i64>(&__trn_5f76616c7565)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::pending_value(__trn_5f76616c7565),
        )
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::pending_value",
                ),
            )
        }
    }
}
pub fn record_async_entry_cleanup() -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::record_async_entry_cleanup()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::record_async_entry_cleanup",
                ),
            )
        }
    }
}
pub fn registrar() -> Result<Registrar, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::registrar()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::registrar",
                ),
            )
        }
    }
}
pub fn retain_adjustable<T: 'static + terrane_callback_witness::Adjustable>(
    value: T,
) -> Result<AdjustableOwner, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::retain_adjustable(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::retain_adjustable",
                ),
            )
        }
    }
}
pub fn retain_boxed_adjustable<
    TerraneBoxed0: terrane_callback_witness::Adjustable + core::marker::Send
        + core::marker::Sync + 'static,
>(value: TerraneBoxed0) -> Result<AdjustableOwner, crate::TerraneForeignError> {
    let value = Box::new(value);
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::retain_boxed_adjustable(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::retain_boxed_adjustable",
                ),
            )
        }
    }
}
pub fn retain_local_adjustable<T: 'static + terrane_callback_witness::LocalAdjustable>(
    value: T,
) -> Result<LocalAdjustableOwner, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::retain_local_adjustable(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::retain_local_adjustable",
                ),
            )
        }
    }
}
pub async fn wait_until_retained_invocation_active() -> Result<
    bool,
    crate::TerraneForeignError,
> {
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::wait_until_retained_invocation_active(),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::wait_until_retained_invocation_active",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/terrane-dispatch-witness.trn
// Namespace: deps/terrane-dispatch-witness
pub fn dispatch(
    label_: String,
    enabled: bool,
    callback: std::sync::Arc<dyn Fn(String, bool) -> String + Send + Sync>,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    let enabled = enabled;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: String, callback_argument_1: bool| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback(callback_argument_0, callback_argument_1);
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_dispatch_witness::dispatch(
            label_,
            enabled,
            callback,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-dispatch-witness",
                    "terrane_dispatch_witness::dispatch",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/terrane-render-witness.trn
// Namespace: deps/terrane-render-witness
pub trait RenderableProtocol: Send {
    fn clone_box(&self) -> Box<dyn RenderableProtocol>;
    fn separate_box(&self) -> Box<dyn RenderableProtocol>;
    fn render(&self, label_: String) -> String;
    fn decorated(&self, label_: String) -> String;
    fn borrowed(&self, label_: String) -> String;
    fn parsed(&self, text: String) -> Result<terrane_int_support::Int, TerraneError>;
}
impl Clone for Box<dyn RenderableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Renderable(Box<dyn RenderableProtocol>);
impl Clone for Renderable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Renderable {
    pub fn render(&self, label_: String) -> String {
        self.0.render(label_)
    }
    pub fn decorated(&self, label_: String) -> String {
        self.0.decorated(label_)
    }
    pub fn borrowed(&self, label_: String) -> String {
        self.0.borrowed(label_)
    }
    pub fn parsed(
        &self,
        text: String,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        self.0.parsed(text)
    }
}
impl terrane_render_witness::Renderable for Renderable {
    fn render(&self, label_: String) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Renderable>::render(&*self, label_);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub fn render_decorated<T: terrane_render_witness::Renderable>(
    value: &T,
    label_: String,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_render_witness::render_decorated(
            value,
            label_,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-render-witness",
                    "terrane_render_witness::render_decorated",
                ),
            )
        }
    }
}
pub fn render_impl<TerraneImpl0: terrane_render_witness::Renderable>(
    value: &TerraneImpl0,
    label_: String,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_render_witness::render_impl(
            value,
            label_,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-render-witness",
                    "terrane_render_witness::render_impl",
                ),
            )
        }
    }
}
pub fn render_value<T: terrane_render_witness::Renderable>(
    value: &T,
    label_: String,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_render_witness::render_value(
            value,
            label_,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-render-witness",
                    "terrane_render_witness::render_value",
                ),
            )
        }
    }
}
