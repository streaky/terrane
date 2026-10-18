// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi, terrane-platform-support
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
    ReplaceError,
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
            Self::ReplaceError => "replace-error",
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
            Self::ReplaceError => {
                "replacement search and replacement lists have different lengths"
            }
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
    rendered_message: Option<String>,
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
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn raised_with_rendered_message(
        kind: TerraneErrorKind,
        message: impl Into<String>,
        rendered_message: Option<String>,
        origin: TerraneSite,
    ) -> Self {
        Self {
            kind,
            origin,
            detail: Some(
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    rendered_message,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
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
                    rendered_message: None,
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
    fn rendered_message(&self) -> &str {
        self.detail
            .as_ref()
            .and_then(|detail| detail.rendered_message.as_deref())
            .unwrap_or_else(|| self.message())
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
                    rendered_message: None,
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
        let mut rendered = format!(
            "{}: {}", self.kind.display_name(), self.rendered_message()
        );
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
impl TerraneRaised for terrane_string_support::ReplaceError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::ReplaceError,
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
    pub static DESCRIPTORS: [&str; 1] = ["/core/time::invalid-duration"];
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
    pub static FILES: [&str; 3] = [
        "core/streams.trn",
        "core/paths.trn",
        "core/time.trn",
    ];
    pub static FUNCTIONS: [&str; 19] = [
        "/core/streams::read",
        "/core/streams::read-exact",
        "/core/streams::read-all",
        "/core/streams::read-async",
        "/core/filesystem/paths::path-components",
        "/core/filesystem/paths::normalise-path",
        "/core/filesystem/paths::path-name",
        "/core/filesystem/paths::path-parent",
        "/core/filesystem/paths::path-stem",
        "/core/filesystem/paths::path-extension",
        "/core/time::multiply",
        "/core/time::seconds",
        "/core/time::milliseconds",
        "/core/time::microseconds",
        "/core/time::nanoseconds",
        "/core/time::duration-until",
        "/core/time::at",
        "/core/time::sleep-until",
        "/core/time::interval",
    ];
    pub static SITES: [Site; 25] = [
        /* terrane-site-row: site 0: /core/streams::read (core/streams.trn:381:23-381:50) */
        { Site { function: 0, file: 0, line: 381, column: 23, end_line: 381, end_column: 50 } },
        /* terrane-site-row: site 1: /core/streams::read-exact (core/streams.trn:403:23-403:46) */
        { Site { function: 1, file: 0, line: 403, column: 23, end_line: 403, end_column: 46 } },
        /* terrane-site-row: site 2: /core/streams::read-all (core/streams.trn:422:23-422:46) */
        { Site { function: 2, file: 0, line: 422, column: 23, end_line: 422, end_column: 46 } },
        /* terrane-site-row: site 3: /core/streams::read-async (core/streams.trn:427:23-427:50) */
        { Site { function: 3, file: 0, line: 427, column: 23, end_line: 427, end_column: 50 } },
        /* terrane-site-row: site 4: /core/filesystem/paths::path-components (core/paths.trn:16:16-16:28) */
        { Site { function: 4, file: 1, line: 16, column: 16, end_line: 16, end_column: 28 } },
        /* terrane-site-row: site 5: /core/filesystem/paths::normalise-path (core/paths.trn:32:16-32:33) */
        { Site { function: 5, file: 1, line: 32, column: 16, end_line: 32, end_column: 33 } },
        /* terrane-site-row: site 6: /core/filesystem/paths::normalise-path (core/paths.trn:35:34-35:49) */
        { Site { function: 5, file: 1, line: 35, column: 34, end_line: 35, end_column: 49 } },
        /* terrane-site-row: site 7: /core/filesystem/paths::normalise-path (core/paths.trn:40:29-40:50) */
        { Site { function: 5, file: 1, line: 40, column: 29, end_line: 40, end_column: 50 } },
        /* terrane-site-row: site 8: /core/filesystem/paths::normalise-path (core/paths.trn:46:21-46:42) */
        { Site { function: 5, file: 1, line: 46, column: 21, end_line: 46, end_column: 42 } },
        /* terrane-site-row: site 9: /core/filesystem/paths::normalise-path (core/paths.trn:56:33-56:44) */
        { Site { function: 5, file: 1, line: 56, column: 33, end_line: 56, end_column: 44 } },
        /* terrane-site-row: site 10: /core/filesystem/paths::path-name (core/paths.trn:69:12-69:35) */
        { Site { function: 6, file: 1, line: 69, column: 12, end_line: 69, end_column: 35 } },
        /* terrane-site-row: site 11: /core/filesystem/paths::path-parent (core/paths.trn:83:33-83:45) */
        { Site { function: 7, file: 1, line: 83, column: 33, end_line: 83, end_column: 45 } },
        /* terrane-site-row: site 12: /core/filesystem/paths::path-stem (core/paths.trn:95:31-95:40) */
        { Site { function: 8, file: 1, line: 95, column: 31, end_line: 95, end_column: 40 } },
        /* terrane-site-row: site 13: /core/filesystem/paths::path-stem (core/paths.trn:102:33-102:46) */
        { Site { function: 8, file: 1, line: 102, column: 33, end_line: 102, end_column: 46 } },
        /* terrane-site-row: site 14: /core/filesystem/paths::path-extension (core/paths.trn:111:31-111:40) */
        { Site { function: 9, file: 1, line: 111, column: 31, end_line: 111, end_column: 40 } },
        /* terrane-site-row: site 15: /core/filesystem/paths::path-extension (core/paths.trn:113:12-113:37) */
        { Site { function: 9, file: 1, line: 113, column: 12, end_line: 113, end_column: 37 } },
        /* terrane-site-row: site 16: /core/time::multiply (core/time.trn:62:13-62:45) */
        { Site { function: 10, file: 2, line: 62, column: 13, end_line: 62, end_column: 45 } },
        /* terrane-site-row: site 17: /core/time::seconds (core/time.trn:38:13-38:45) */
        { Site { function: 11, file: 2, line: 38, column: 13, end_line: 38, end_column: 45 } },
        /* terrane-site-row: site 18: /core/time::milliseconds (core/time.trn:43:13-43:45) */
        { Site { function: 12, file: 2, line: 43, column: 13, end_line: 43, end_column: 45 } },
        /* terrane-site-row: site 19: /core/time::microseconds (core/time.trn:48:13-48:45) */
        { Site { function: 13, file: 2, line: 48, column: 13, end_line: 48, end_column: 45 } },
        /* terrane-site-row: site 20: /core/time::nanoseconds (core/time.trn:53:13-53:45) */
        { Site { function: 14, file: 2, line: 53, column: 13, end_line: 53, end_column: 45 } },
        /* terrane-site-row: site 21: /core/time::duration-until (core/time.trn:76:13-76:45) */
        { Site { function: 15, file: 2, line: 76, column: 13, end_line: 76, end_column: 45 } },
        /* terrane-site-row: site 22: /core/time::at (core/time.trn:105:13-105:45) */
        { Site { function: 16, file: 2, line: 105, column: 13, end_line: 105, end_column: 45 } },
        /* terrane-site-row: site 23: /core/time::sleep-until (core/time.trn:161:13-161:45) */
        { Site { function: 17, file: 2, line: 161, column: 13, end_line: 161, end_column: 45 } },
        /* terrane-site-row: site 24: /core/time::interval (core/time.trn:172:13-172:45) */
        { Site { function: 18, file: 2, line: 172, column: 13, end_line: 172, end_column: 45 } },
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
static TERRANE_SHARED_NEXT_THREAD_TOKEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(
    1,
);
std::thread_local! {
    static TERRANE_SHARED_THREAD_TOKEN : u64 = TERRANE_SHARED_NEXT_THREAD_TOKEN
    .try_update(std::sync::atomic::Ordering::Relaxed,
    std::sync::atomic::Ordering::Relaxed, | next | next.checked_add(1),)
    .expect("shared storage thread token space exhausted");
}
#[derive(Debug)]
pub struct TerraneSharedCell<T> {
    owner: std::sync::atomic::AtomicU64,
    value: std::sync::Mutex<T>,
}
impl<T> TerraneSharedCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            owner: std::sync::atomic::AtomicU64::new(0),
            value: std::sync::Mutex::new(value),
        }
    }
    /// Acquires exclusive shared-storage access, waiting for other threads.
    ///
    /// # Panics
    /// Panics on same-thread reentry, before attempting to wait on itself.
    ///
    /// # Errors
    /// Returns a poisoned guard if a previous holder unwound while borrowing.
    pub fn lock(&self) -> std::sync::LockResult<TerraneSharedGuard<'_, T>> {
        let token = TERRANE_SHARED_THREAD_TOKEN.with(|token| *token);
        assert!(
            self.owner.load(std::sync::atomic::Ordering::Acquire) != token,
            "shared reference borrow conflict: storage is already borrowed by this thread"
        );
        let (guard, poisoned) = match self.value.lock() {
            Ok(guard) => (guard, false),
            Err(error) => (error.into_inner(), true),
        };
        self.owner.store(token, std::sync::atomic::Ordering::Release);
        let wrapped = TerraneSharedGuard {
            owner: &self.owner,
            guard,
        };
        if poisoned { Err(std::sync::PoisonError::new(wrapped)) } else { Ok(wrapped) }
    }
}
#[derive(Debug)]
pub struct TerraneSharedGuard<'a, T> {
    owner: &'a std::sync::atomic::AtomicU64,
    guard: std::sync::MutexGuard<'a, T>,
}
impl<T> std::ops::Deref for TerraneSharedGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.guard
    }
}
impl<T> std::ops::DerefMut for TerraneSharedGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}
impl<T> Drop for TerraneSharedGuard<'_, T> {
    fn drop(&mut self) {
        self.owner.store(0, std::sync::atomic::Ordering::Release);
    }
}
pub struct TerraneStaticObject<T> {
    current: TerraneSharedCell<std::sync::Arc<TerraneSharedCell<T>>>,
}
impl<T> TerraneStaticObject<T> {
    pub fn new(value: T) -> Self {
        Self {
            current: TerraneSharedCell::new(
                std::sync::Arc::new(TerraneSharedCell::new(value)),
            ),
        }
    }
    /// Captures the current object without retaining the replaceable slot guard.
    ///
    /// # Panics
    /// Panics if the slot is poisoned or is recursively borrowed by this thread.
    pub fn handle(&self) -> std::sync::Arc<TerraneSharedCell<T>> {
        self.current.lock().expect("static object slot lock poisoned").clone()
    }
    /// Installs a new object, releasing the slot before dropping its predecessor.
    ///
    /// # Panics
    /// Panics if the slot is poisoned, recursively borrowed, or its old object panics on drop.
    pub fn replace(&self, value: T) {
        let replacement = std::sync::Arc::new(TerraneSharedCell::new(value));
        let previous = {
            let mut current = self
                .current
                .lock()
                .expect("static object slot lock poisoned");
            std::mem::replace(&mut *current, replacement)
        };
        drop(previous);
    }
}
pub trait TerraneSharedHandle<T> {
    /// Acquires a bounded exclusive borrow of the receiver's live storage.
    ///
    /// # Errors
    /// Returns the original cell's poison state with a guard to the selected value.
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>>;
}
impl<T> TerraneSharedHandle<T> for std::sync::Arc<TerraneSharedCell<T>> {
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>> {
        (**self).lock()
    }
}
pub struct TerraneSharedProjection<H, U, T> {
    handle: H,
    read: fn(&U) -> &T,
    write: fn(&mut U) -> &mut T,
}
impl<H, U, T> TerraneSharedProjection<H, U, T> {
    pub fn new(handle: H, read: fn(&U) -> &T, write: fn(&mut U) -> &mut T) -> Self {
        Self { handle, read, write }
    }
}
impl<H: Clone, U, T> Clone for TerraneSharedProjection<H, U, T> {
    fn clone(&self) -> Self {
        Self::new(self.handle.clone(), self.read, self.write)
    }
}
struct TerraneSharedProjectedGuard<G, U, T> {
    guard: G,
    read: fn(&U) -> &T,
    write: fn(&mut U) -> &mut T,
}
impl<G: std::ops::Deref<Target = U>, U, T> std::ops::Deref
for TerraneSharedProjectedGuard<G, U, T> {
    type Target = T;
    fn deref(&self) -> &T {
        (self.read)(&self.guard)
    }
}
impl<G: std::ops::DerefMut<Target = U>, U, T> std::ops::DerefMut
for TerraneSharedProjectedGuard<G, U, T> {
    fn deref_mut(&mut self) -> &mut T {
        (self.write)(&mut self.guard)
    }
}
impl<H: TerraneSharedHandle<U>, U, T> TerraneSharedHandle<T>
for TerraneSharedProjection<H, U, T> {
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>> {
        let (guard, poisoned) = match self.handle.lock() {
            Ok(guard) => (guard, false),
            Err(error) => (error.into_inner(), true),
        };
        let guard = TerraneSharedProjectedGuard {
            guard,
            read: self.read,
            write: self.write,
        };
        if poisoned { Err(std::sync::PoisonError::new(guard)) } else { Ok(guard) }
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
            .await;
    }
}
trait TerraneTaskCleanup: Send + Sync {
    fn cancel(&self);
    fn cleanup(
        self: std::sync::Arc<Self>,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}
struct TerraneFinalizerState {
    depth: std::sync::atomic::AtomicUsize,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
    task_groups: parking_lot::Mutex<Vec<std::sync::Weak<dyn TerraneTaskCleanup>>>,
    pending_groups: parking_lot::Mutex<Vec<std::sync::Arc<dyn TerraneTaskCleanup>>>,
}
impl TerraneFinalizerState {
    fn new() -> Self {
        Self {
            depth: std::sync::atomic::AtomicUsize::new(0),
            wakers: std::sync::Mutex::new(Vec::new()),
            task_groups: parking_lot::Mutex::new(Vec::new()),
            pending_groups: parking_lot::Mutex::new(Vec::new()),
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
        reason = "group registration is shared by native async packages without groups"
    )]
    fn register_task_group<T: TerraneTaskCleanup + Send + Sync + 'static>(
        &self,
        group: std::sync::Weak<T>,
    ) {
        let group: std::sync::Weak<dyn TerraneTaskCleanup> = group;
        let mut groups = self.task_groups.lock();
        groups.retain(|group| group.strong_count() != 0);
        groups.push(group);
    }
    #[allow(
        dead_code,
        reason = "group cleanup is shared by native async packages without groups"
    )]
    fn retain_task_group(&self, group: std::sync::Arc<dyn TerraneTaskCleanup>) {
        self.task_groups
            .lock()
            .retain(|weak| {
                !std::ptr::addr_eq(weak.as_ptr(), std::sync::Arc::as_ptr(&group))
            });
        self.pending_groups.lock().push(group);
    }
    async fn drain_task_groups(&self) {
        let mut groups = std::mem::take(&mut *self.pending_groups.lock());
        {
            let mut registry = self.task_groups.lock();
            groups.extend(registry.iter().filter_map(std::sync::Weak::upgrade));
            registry.retain(|group| group.strong_count() != 0);
        }
        for group in &groups {
            group.cancel();
        }
        for group in groups {
            group.cleanup().await;
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
    #[allow(
        dead_code,
        reason = "projected cleanup tracking is shared by packages without projected entries"
    )]
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
            .await;
    }
    fn depth(&self) -> usize {
        self.depth.load(std::sync::atomic::Ordering::Acquire)
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneCancellationContext {
    cancellation: TerraneCancellation,
    deadline: Option<terrane_int_support::Int>,
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
fn __terrane_await_destructor<F>(future: F)
where
    F: Future<Output = ()> + Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Terrane destructor runtime must initialize")
                    .block_on(future);
            })
            .join()
            .expect("Terrane awaited destructor must not panic");
    });
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
    reason = "selection support is shared by native async packages without select statements"
)]
#[derive(Clone)]
struct TerraneSelectControl {
    requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
    waker: std::sync::Arc<std::sync::Mutex<Option<std::task::Waker>>>,
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
impl TerraneSelectControl {
    fn request_cancel(&self) {
        self.requested.store(true, std::sync::atomic::Ordering::Release);
        if let Some(waker) = self
            .waker
            .lock()
            .expect("select cancellation waker lock poisoned")
            .take()
        {
            waker.wake();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.requested.load(std::sync::atomic::Ordering::Acquire)
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
fn __terrane_select_control() -> TerraneSelectControl {
    TerraneSelectControl {
        requested: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: std::sync::Arc::new(std::sync::Mutex::new(None)),
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
async fn __terrane_select_operation<F: Future>(
    control: TerraneSelectControl,
    future: F,
) -> Option<F::Output> {
    let deadline = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| context.deadline.clone())
        .ok()
        .flatten();
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline,
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async move {
                let mut future = std::pin::pin!(future);
                let cleanup = finalizers.clone();
                let output = std::future::poll_fn(move |cx| {
                        if control.is_cancelled() {
                            cancellation.cancel();
                            let mut stored_waker = control
                                .waker
                                .lock()
                                .expect("select cancellation waker lock poisoned");
                            if stored_waker
                                .as_ref()
                                .is_none_or(|waker| !waker.will_wake(cx.waker()))
                            {
                                *stored_waker = Some(cx.waker().clone());
                            }
                            drop(stored_waker);
                            return match Future::poll(future.as_mut(), cx) {
                                std::task::Poll::Ready(output) => {
                                    std::task::Poll::Ready(Some(output))
                                }
                                std::task::Poll::Pending if finalizers.depth() == 0 => {
                                    std::task::Poll::Ready(None)
                                }
                                std::task::Poll::Pending => std::task::Poll::Pending,
                            };
                        }
                        Future::poll(future.as_mut(), cx).map(Some)
                    })
                    .await;
                if output.is_none() {
                    cleanup.drain_task_groups().await;
                }
                output
            },
        )
        .await
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
    deadline: Option<terrane_int_support::Int>,
) {
    if let Some(deadline) = deadline {
        tokio::select! {
            () = cancellation.cancelled() => {} () = terrane_time_sleep_until(deadline)
            => { cancellation.cancel(); }
        }
    } else {
        cancellation.cancelled().await;
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
fn __terrane_cancellation_is_requested() -> bool {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            if context.deadline.as_ref().is_some_and(terrane_time_deadline_expired) {
                context.cancellation.cancel();
            }
            context.cancellation.is_cancelled()
        })
        .unwrap_or(false)
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
                    (
                        context.cancellation.clone(),
                        context.deadline.clone(),
                        state.clone(),
                    )
                })
        })
        .ok()
        .flatten();
    if let Some((cancellation, deadline, finalizers)) = cancellation {
        let mut future = std::pin::pin!(future);
        tokio::select! {
            biased; output = future.as_mut() => Some(output), () =
            __terrane_cancellation_requested(cancellation, deadline) => { let output =
            std::future::poll_fn(| cx | { match Future::poll(future.as_mut(), cx) {
            std::task::Poll::Ready(output) => std::task::Poll::Ready(Some(output)),
            std::task::Poll::Pending if finalizers.depth() == guard.depth => {
            std::task::Poll::Ready(None) } std::task::Poll::Pending =>
            std::task::Poll::Pending, } }). await; if output.is_none() { finalizers
            .drain_task_groups(). await; } output },
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
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
async fn __terrane_finish_cancelled_select(mut guard: TerraneFinallyGuard) -> ! {
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
    deadline: Option<terrane_int_support::Int>,
) -> Option<F::Output> {
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline: deadline.clone(),
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async {
                let mut future = std::pin::pin!(future);
                let output = tokio::select! {
                    biased; output = future.as_mut() => Some(output), () =
                    __terrane_cancellation_requested(cancellation, deadline.clone()) => {
                    finalizers.drain_task_groups(). await; std::future::poll_fn(| cx | {
                    match Future::poll(future.as_mut(), cx) {
                    std::task::Poll::Ready(output) =>
                    std::task::Poll::Ready(Some(output)), std::task::Poll::Pending if
                    finalizers.depth() == 0 => { std::task::Poll::Ready(None) }
                    std::task::Poll::Pending => std::task::Poll::Pending, } }). await },
                };
                finalizers.drain_task_groups().await;
                output
            },
        )
        .await
}
#[allow(
    dead_code,
    reason = "async entry support is emitted for libraries without an entrypoint"
)]
async fn __terrane_async_entry<F: Future>(future: F) -> F::Output {
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: TerraneCancellation::new(),
        deadline: None,
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async {
                let output = future.await;
                finalizers.drain_task_groups().await;
                output
            },
        )
        .await
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_domain() -> terrane_int_support::Int {
    terrane_int_support::Int::from(1_i128)
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_monotonic() -> terrane_int_support::Int {
    terrane_int_support::Int::from_u128(terrane_platform_support::monotonic_nanos())
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_deadline_expired(target: &terrane_int_support::Int) -> bool {
    terrane_time_monotonic() >= target.clone()
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
async fn terrane_time_sleep_until(target: terrane_int_support::Int) {
    const MAX_WAKE_CHUNK_NANOS: u64 = u64::MAX;
    loop {
        let now = terrane_time_monotonic();
        if now >= target {
            return;
        }
        let remaining = target.clone() - now;
        let chunk = terrane_int_support::coerce::<u64>(&remaining)
            .unwrap_or(MAX_WAKE_CHUNK_NANOS);
        terrane_platform_support::sleep(std::time::Duration::from_nanos(chunk)).await;
    }
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_sleep_after(
    elapsed: terrane_int_support::Int,
) -> impl std::future::Future<Output = ()> {
    let target = terrane_time_monotonic() + elapsed;
    async move {
        terrane_time_sleep_until(target).await;
    }
}
#[derive(Clone)]
pub struct TerranePlatformStreamHandle(std::sync::Arc<i64>);
impl Default for TerranePlatformStreamHandle {
    fn default() -> Self {
        Self(std::sync::Arc::new(0))
    }
}
impl TerranePlatformStreamHandle {
    fn new(handle: terrane_stream_abi::StreamHandle) -> Self {
        Self(std::sync::Arc::new(handle.id()))
    }
    fn abi_handle(&self) -> terrane_stream_abi::StreamHandle {
        terrane_stream_abi::StreamHandle::from_id(*self.0)
    }
}
#[derive(Clone)]
pub struct TerranePlatformReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}
#[derive(Clone)]
pub struct TerranePlatformWriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}
#[derive(Clone)]
pub struct TerranePlatformUnitResult {
    pub failed: bool,
    pub message: String,
}
pub fn terrane_platform_read(
    handle: &TerranePlatformStreamHandle,
    limit: terrane_int_support::Int,
) -> TerranePlatformReadResult {
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformReadResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            end: false,
            failed: true,
            message: "stream read count is outside the supported size range".to_owned(),
            deadline_exceeded: false,
        };
    };
    match terrane_stream_abi::read(handle.abi_handle(), limit) {
        Ok(outcome) => {
            TerranePlatformReadResult {
                completed: terrane_int_support::Int::from(outcome.data.len() as i128),
                data: outcome.data,
                end: outcome.end,
                failed: false,
                message: String::new(),
                deadline_exceeded: false,
            }
        }
        Err(error) => {
            TerranePlatformReadResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(0_i64),
                end: false,
                failed: true,
                message: error.to_string(),
                deadline_exceeded: false,
            }
        }
    }
}
pub fn terrane_platform_write(
    handle: &TerranePlatformStreamHandle,
    data: &[u8],
    offset: terrane_int_support::Int,
) -> TerranePlatformWriteResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformWriteResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: "stream write offset is outside the buffer".to_owned(),
            deadline_exceeded: false,
        };
    };
    match terrane_stream_abi::write(handle.abi_handle(), &data[offset..]) {
        Ok(completed) => {
            TerranePlatformWriteResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(completed as i128),
                failed: false,
                message: String::new(),
                deadline_exceeded: false,
            }
        }
        Err(error) => {
            TerranePlatformWriteResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(0_i64),
                failed: true,
                message: error.to_string(),
                deadline_exceeded: false,
            }
        }
    }
}
pub fn terrane_platform_flush(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::flush(handle.abi_handle()))
}
pub fn terrane_platform_sync_data(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_data(handle.abi_handle()))
}
pub fn terrane_platform_sync_all(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_all(handle.abi_handle()))
}
pub fn terrane_platform_close(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::close(handle.abi_handle()))
}
pub fn terrane_platform_release(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    if std::sync::Arc::strong_count(&handle.0) == 1 {
        terrane_platform_unit(terrane_stream_abi::release(handle.abi_handle()))
    } else {
        TerranePlatformUnitResult {
            failed: false,
            message: String::new(),
        }
    }
}
pub fn terrane_platform_unit(result: std::io::Result<()>) -> TerranePlatformUnitResult {
    match result {
        Ok(()) => {
            TerranePlatformUnitResult {
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformUnitResult {
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
pub async fn terrane_platform_read_async(
    handle: &TerranePlatformStreamHandle,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformReadResult {
    if let Some(error) = terrane_platform_stream_options(&deadline, cancellation) {
        return TerranePlatformReadResult {
            data: Vec::new(),
            completed: 0_i64.into(),
            end: false,
            failed: error.failed,
            message: error.message,
            deadline_exceeded: error.deadline_exceeded,
        };
    }
    let handle = handle.clone();
    let _wait = __terrane_finally_guard();
    tokio::task::spawn_blocking(move || terrane_platform_read(&handle, limit))
        .await
        .expect("delegated stream read must not panic")
}
pub fn terrane_platform_buffer_create(
    capacity: &terrane_int_support::Int,
) -> TerranePlatformResult {
    let capacity = match terrane_platform_i128(capacity, "read buffer capacity") {
        Ok(value) => value,
        Err(error) => return error,
    };
    terrane_platform_support::buffer_create(capacity)
}
pub fn terrane_platform_buffer_clear(
    buffer: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::buffer_clear(buffer)
}
pub fn terrane_platform_buffer_capacity(
    buffer: &TerranePlatformCapability,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        terrane_platform_support::platform_buffer_capacity(buffer).number,
    )
}
pub fn terrane_platform_buffer_completed(
    buffer: &TerranePlatformCapability,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        terrane_platform_support::platform_buffer_completed(buffer).number,
    )
}
pub fn terrane_platform_buffer_filled_bytes(
    buffer: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::buffer_filled_bytes(buffer)
}
pub fn terrane_platform_buffer_filled_text(
    buffer: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::buffer_filled_text(buffer)
}
pub async fn terrane_platform_tcp_read_into_async(
    stream: &TerranePlatformCapability,
    buffer: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    let limit = match terrane_platform_i128(&limit, "TCP read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    terrane_platform_support::tcp_read_into_async(
            stream,
            buffer,
            limit,
            terrane_platform_timeout_nanos(deadline),
            cancellation,
        )
        .await
}
pub async fn terrane_platform_tls_read_into_async(
    stream: &TerranePlatformCapability,
    buffer: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    let limit = match terrane_platform_i128(&limit, "TLS read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    terrane_platform_support::tls_read_into_async(
            stream,
            buffer,
            limit,
            terrane_platform_timeout_nanos(deadline),
            cancellation,
        )
        .await
}
pub async fn terrane_platform_stream_read_into_async(
    handle: &TerranePlatformStreamHandle,
    buffer: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformResult {
    if let Some(error) = terrane_platform_stream_options(&deadline, cancellation) {
        return error;
    }
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformResult::error(
            "stream read count is outside the supported size range",
        );
    };
    let handle = handle.clone();
    let buffer = buffer.clone();
    let _wait = __terrane_finally_guard();
    tokio::task::spawn_blocking(move || {
            let mut result = terrane_platform_support::buffer_read_with(
                &buffer,
                |storage| {
                    if limit > storage.len() {
                        return Err(
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "stream read count exceeds read buffer capacity",
                            ),
                        );
                    }
                    if limit == 0 {
                        return Ok(0);
                    }
                    terrane_stream_abi::read_into(
                        handle.abi_handle(),
                        &mut storage[..limit],
                    )
                },
            );
            if limit == 0 && !result.failed {
                result.flag = false;
            }
            result
        })
        .await
        .expect("delegated stream buffer read must not panic")
}
pub async fn terrane_platform_write_async(
    handle: &TerranePlatformStreamHandle,
    data: Vec<u8>,
    offset: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> TerranePlatformWriteResult {
    if let Some(error) = terrane_platform_stream_options(&deadline, cancellation) {
        return TerranePlatformWriteResult {
            data,
            completed: 0_i64.into(),
            failed: error.failed,
            message: error.message,
            deadline_exceeded: error.deadline_exceeded,
        };
    }
    let handle = handle.clone();
    let _wait = __terrane_finally_guard();
    tokio::task::spawn_blocking(move || {
            let mut result = terrane_platform_write(&handle, &data, offset);
            result.data = data;
            result
        })
        .await
        .expect("delegated stream write must not panic")
}
pub fn terrane_platform_buffer_close(
    buffer: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::close(buffer)
}
fn terrane_platform_stream_options(
    deadline: &terrane_int_support::Int,
    cancellation: Option<&TerranePlatformCapability>,
) -> Option<TerranePlatformResult> {
    if let Some(token) = cancellation {
        let result = terrane_platform_support::check_cancellation(Some(token));
        if result.failed {
            return Some(result);
        }
    }
    match terrane_platform_i128(deadline, "stream deadline") {
        Ok(-1) => None,
        Ok(0) => {
            Some(TerranePlatformResult {
                failed: true,
                deadline_exceeded: true,
                message: "stream deadline exceeded".to_owned(),
                ..TerranePlatformResult::default()
            })
        }
        Ok(value) if value < -1 => {
            Some(
                TerranePlatformResult::error(
                    "stream deadline is outside the supported range",
                ),
            )
        }
        Ok(_) => None,
        Err(error) => Some(error),
    }
}
pub fn terrane_platform_acquire_stdin() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdin())
}
pub fn terrane_platform_acquire_stdout() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdout())
}
pub fn terrane_platform_acquire_stderr() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stderr())
}
#[derive(Clone)]
pub struct TerranePlatformOpenResult {
    pub handle: TerranePlatformStreamHandle,
    pub failed: bool,
    pub message: String,
}
pub fn terrane_file_open_options(
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> Result<terrane_stream_abi::FileOpenOptions, String> {
    let access = match (readable, writable) {
        (true, false) => terrane_stream_abi::FileAccess::Read,
        (false, true) => terrane_stream_abi::FileAccess::Write,
        (true, true) => terrane_stream_abi::FileAccess::ReadWrite,
        (false, false) => {
            return Err("a file must be opened for reading, writing, or both".to_owned());
        }
    };
    let creation = match (create, truncate) {
        (false, false) => terrane_stream_abi::FileCreation::Existing,
        (true, false) => terrane_stream_abi::FileCreation::Create,
        (false, true) => terrane_stream_abi::FileCreation::Truncate,
        (true, true) => terrane_stream_abi::FileCreation::CreateOrTruncate,
    };
    Ok(terrane_stream_abi::FileOpenOptions {
        access,
        creation,
    })
}
pub fn terrane_platform_open_result(
    result: std::io::Result<terrane_stream_abi::StreamHandle>,
) -> TerranePlatformOpenResult {
    match result {
        Ok(handle) => {
            TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::new(handle),
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::default(),
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
pub fn terrane_platform_open_file(
    path: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> TerranePlatformOpenResult {
    let request = match terrane_file_open_options(readable, writable, create, truncate) {
        Ok(request) => request,
        Err(message) => {
            return TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::default(),
                failed: true,
                message,
            };
        }
    };
    terrane_platform_open_result(terrane_stream_abi::open_file(&path, request))
}
pub fn terrane_platform_open_directory_beneath(
    base: String,
    child: String,
    cross_filesystem: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(
        terrane_stream_abi::open_directory_beneath(&base, &child, cross_filesystem),
    )
}
pub fn terrane_platform_open_file_beneath(
    directory: &TerranePlatformStreamHandle,
    child: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> TerranePlatformOpenResult {
    let request = match terrane_file_open_options(readable, writable, create, truncate) {
        Ok(request) => request,
        Err(message) => {
            return TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::default(),
                failed: true,
                message,
            };
        }
    };
    terrane_platform_open_result(
        terrane_stream_abi::open_file_beneath(directory.abi_handle(), &child, request),
    )
}
#[derive(Clone, Default)]
pub struct TerraneFilesystemAuthority {
    _private: (),
}
pub fn terrane_acquire_filesystem_authority() -> TerraneFilesystemAuthority {
    TerraneFilesystemAuthority {
        _private: (),
    }
}
#[derive(Clone, Default)]
pub struct TerraneFilesystemResult {
    pub failed: bool,
    pub message: String,
    pub text: String,
    pub detail: String,
    pub data: Vec<u8>,
    pub number: i128,
    pub flag: bool,
}
pub fn terrane_io_error(error: std::io::Error) -> TerraneFilesystemResult {
    TerraneFilesystemResult {
        failed: true,
        message: error.to_string(),
        ..TerraneFilesystemResult::default()
    }
}
pub fn terrane_filesystem_result_failed(result: &TerraneFilesystemResult) -> bool {
    result.failed
}
pub fn terrane_filesystem_result_message(result: &TerraneFilesystemResult) -> String {
    result.message.clone()
}
pub fn terrane_filesystem_result_text(result: &TerraneFilesystemResult) -> String {
    result.text.clone()
}
pub fn terrane_filesystem_result_detail(result: &TerraneFilesystemResult) -> String {
    result.detail.clone()
}
pub fn terrane_filesystem_result_bytes(result: &TerraneFilesystemResult) -> Vec<u8> {
    result.data.clone()
}
pub fn terrane_filesystem_result_int(
    result: &TerraneFilesystemResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
pub fn terrane_filesystem_result_bool(result: &TerraneFilesystemResult) -> bool {
    result.flag
}
#[cfg(unix)]
pub fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt as _;
    format!("unix-mode:{:04o}", metadata.permissions().mode() &0o7777)
}
#[cfg(not(unix))]
pub fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    format!("readonly:{}", metadata.permissions().readonly())
}
pub fn terrane_metadata(
    path: &std::path::Path,
    follow: bool,
) -> TerraneFilesystemResult {
    let metadata = if follow {
        std::fs::metadata(path)
    } else {
        std::fs::symlink_metadata(path)
    };
    match metadata {
        Ok(metadata) => {
            let file_type = metadata.file_type();
            let kind = if file_type.is_file() {
                "regular-file"
            } else if file_type.is_dir() {
                "directory"
            } else if file_type.is_symlink() {
                "symlink"
            } else {
                "other"
            };
            TerraneFilesystemResult {
                text: kind.to_owned(),
                detail: terrane_permission_detail(&metadata),
                number: i128::from(metadata.len()),
                flag: metadata.permissions().readonly(),
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => {
            TerraneFilesystemResult {
                failed: true,
                message: error.to_string(),
                text: "other".to_owned(),
                detail: "unavailable".to_owned(),
                ..TerraneFilesystemResult::default()
            }
        }
    }
}
pub fn terrane_atomic_replace(
    path: &std::path::Path,
    data: &[u8],
) -> std::io::Result<()> {
    use std::io::Write as _;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let name = path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("file"));
    let existing_permissions = std::fs::symlink_metadata(path)
        .ok()
        .filter(|metadata| !metadata.file_type().is_symlink())
        .map(|metadata| metadata.permissions());
    let mut attempt = 0_u32;
    loop {
        let mut temporary_name = std::ffi::OsString::from(".");
        temporary_name.push(name);
        temporary_name.push(format!(".terrane-{}-{attempt}", std::process::id()));
        let temporary = parent.join(temporary_name);
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary) {
            Ok(mut file) => {
                let outcome = (|| {
                    if let Some(permissions) = existing_permissions.clone() {
                        file.set_permissions(permissions)?;
                    }
                    file.write_all(data)?;
                    file.sync_all()?;
                    std::fs::rename(&temporary, path)?;
                    #[cfg(unix)] std::fs::File::open(parent)?.sync_all()?;
                    Ok(())
                })();
                if outcome.is_err() {
                    let _ = std::fs::remove_file(&temporary);
                }
                return outcome;
            }
            Err(
                error,
            ) if error.kind() == std::io::ErrorKind::AlreadyExists && attempt < 32 => {
                attempt += 1;
            }
            Err(error) => return Err(error),
        }
    }
}
pub fn terrane_filesystem_exists(path: String) -> TerraneFilesystemResult {
    match std::path::Path::new(&path).try_exists() {
        Ok(exists) => {
            TerraneFilesystemResult {
                flag: exists,
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_filesystem_metadata(
    path: String,
    follow: bool,
) -> TerraneFilesystemResult {
    terrane_metadata(std::path::Path::new(&path), follow)
}
pub fn terrane_filesystem_realpath(path: String) -> TerraneFilesystemResult {
    match std::fs::canonicalize(path).and_then(terrane_path_text) {
        Ok(value) => {
            TerraneFilesystemResult {
                text: value,
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_filesystem_read_link(path: String) -> TerraneFilesystemResult {
    match std::fs::read_link(path).and_then(terrane_path_text) {
        Ok(value) => {
            TerraneFilesystemResult {
                text: value,
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_filesystem_read_bounded(
    path: String,
    limit: impl Into<terrane_int_support::Int>,
) -> TerraneFilesystemResult {
    use std::io::Read as _;
    let Some(limit) = limit.into().as_usize() else {
        return terrane_io_error(
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid read limit"),
        );
    };
    let mut value = Vec::with_capacity(limit.min(8192));
    let outcome = std::fs::File::open(path)
        .and_then(|file| {
            file.take(limit.saturating_add(1) as u64).read_to_end(&mut value)
        });
    match outcome {
        Ok(_) if value.len() <= limit => {
            TerraneFilesystemResult {
                number: value.len() as i128,
                data: value,
                ..TerraneFilesystemResult::default()
            }
        }
        Ok(_) => {
            terrane_io_error(
                std::io::Error::new(
                    std::io::ErrorKind::FileTooLarge,
                    "file exceeds declared read limit",
                ),
            )
        }
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_filesystem_write_atomic(
    path: String,
    data: Vec<u8>,
) -> TerraneFilesystemResult {
    match terrane_atomic_replace(std::path::Path::new(&path), &data) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_filesystem_remove(path: String) -> TerraneFilesystemResult {
    match std::fs::remove_file(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_filesystem_rename(
    source: String,
    destination: String,
) -> TerraneFilesystemResult {
    match std::fs::rename(source, destination) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_path_text(path: std::path::PathBuf) -> std::io::Result<String> {
    path.into_os_string()
        .into_string()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "filesystem path is not valid Unicode",
            )
        })
}
pub type TerranePlatformCapability = terrane_platform_support::Capability;
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
pub fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| {
            TerranePlatformResult::error(
                format!("{label} is outside the signed 128-bit platform range"),
            )
        })
}
pub fn terrane_platform_timeout_nanos(value: terrane_int_support::Int) -> i128 {
    if value < terrane_int_support::Int::from(0_i128) {
        return -1;
    }
    i128::from(terrane_int_support::coerce::<u64>(&value).unwrap_or(u64::MAX))
}
#[allow(dead_code)]
fn terrane_platform_cancellation_token() -> TerranePlatformCapability {
    terrane_platform_support::cancellation_token()
}
#[allow(dead_code)]
fn terrane_platform_no_resource() -> TerranePlatformCapability {
    TerranePlatformCapability::default()
}
#[allow(dead_code)]
fn terrane_platform_failed_result() -> TerranePlatformResult {
    TerranePlatformResult::error("uninitialized platform value")
}
#[allow(dead_code)]
fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::cancel(token)
}
#[allow(dead_code)]
fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
#[allow(dead_code)]
fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool {
    result.resource_limit
}
#[allow(dead_code)]
fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool {
    result.truncated
}
#[allow(dead_code)]
fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
#[allow(dead_code)]
fn terrane_platform_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_text(result: &TerranePlatformResult) -> String {
    result.text.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String {
    result.detail.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
#[allow(dead_code)]
fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool {
    result.flag
}
#[allow(dead_code)]
fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> {
    result.entries.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_capability(
    result: &TerranePlatformResult,
) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
fn terrane_platform_time_wall() -> TerranePlatformResult {
    terrane_platform_support::wall_time()
}
fn terrane_platform_time_wall_seconds(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
fn terrane_platform_time_wall_nanoseconds(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.secondary_number)
}
fn terrane_platform_time_domain() -> terrane_int_support::Int {
    terrane_time_domain()
}
fn terrane_platform_time_monotonic() -> terrane_int_support::Int {
    terrane_time_monotonic()
}
async fn terrane_platform_time_sleep_until(target: terrane_int_support::Int) {
    terrane_time_sleep_until(target).await;
}
trait TerraneTimeDivisor {
    fn into_time_int(self) -> terrane_int_support::Int;
}
impl TerraneTimeDivisor for i32 {
    fn into_time_int(self) -> terrane_int_support::Int {
        terrane_int_support::Int::from(i64::from(self))
    }
}
impl TerraneTimeDivisor for terrane_int_support::Int {
    fn into_time_int(self) -> terrane_int_support::Int {
        self
    }
}
fn terrane_platform_time_div(
    value: &terrane_int_support::Int,
    divisor: impl TerraneTimeDivisor,
) -> terrane_int_support::Int {
    value
        .euclidean_div(&divisor.into_time_int())
        .expect("bundled time arithmetic divisors must be nonzero")
}
fn terrane_platform_time_mod(
    value: &terrane_int_support::Int,
    divisor: impl TerraneTimeDivisor,
) -> terrane_int_support::Int {
    value
        .modulo(&divisor.into_time_int())
        .expect("bundled time arithmetic divisors must be nonzero")
}
// Source: core/filesystem.trn
// Namespace: core/filesystem
#[derive(Clone)]
pub struct FilesystemOperationResult {
    pub failed: bool,
    pub message: String,
}
impl FilesystemOperationResult {
    pub fn terrane_construct(failure: bool, detail: String) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(failure, detail);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failure: bool, detail: String) {
        {
            let __terrane_value = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct ExistenceResult {
    pub exists: bool,
    pub failed: bool,
    pub message: String,
}
impl ExistenceResult {
    pub fn terrane_construct(exists: bool, failure: bool, detail: String) -> Self {
        let mut __terrane_constructed_value = Self {
            exists: false,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(exists, failure, detail);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, exists: bool, failure: bool, detail: String) {
        {
            let __terrane_value = exists;
            self.exists = __terrane_value;
        }
        {
            let __terrane_value = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct PathResult {
    pub resolved: Path,
    pub failed: bool,
    pub message: String,
}
impl PathResult {
    pub fn terrane_construct(target: Path, failure: bool, detail: String) -> Self {
        let mut __terrane_constructed_value = Self {
            resolved: Path::terrane_construct(String::from("")),
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(target, failure, detail);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, target: Path, failure: bool, detail: String) {
        {
            let __terrane_value = target;
            self.resolved = __terrane_value;
        }
        {
            let __terrane_value = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct FileMetadata {
    pub kind: String,
    pub size: terrane_int_support::Int,
    pub readonly: bool,
    pub permission_detail: String,
    pub failed: bool,
    pub message: String,
}
impl FileMetadata {
    pub fn terrane_construct(
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            kind: String::from("other"),
            size: terrane_int_support::Int::from(0_i128),
            readonly: false,
            permission_detail: String::from(""),
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value
            .construct(kind, size, readonly, permission_detail, failure, detail);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
    ) {
        {
            let __terrane_value = kind;
            self.kind = __terrane_value;
        }
        {
            let __terrane_value = size.clone();
            self.size = __terrane_value;
        }
        {
            let __terrane_value = readonly;
            self.readonly = __terrane_value;
        }
        {
            let __terrane_value = permission_detail;
            self.permission_detail = __terrane_value;
        }
        {
            let __terrane_value = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct FileData {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl FileData {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::<u8>::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(data, completed, end, failure, detail);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
    ) {
        {
            let __terrane_value = data;
            self.data = __terrane_value;
        }
        {
            let __terrane_value = completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value = end;
            self.end = __terrane_value;
        }
        {
            let __terrane_value = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
    }
}
pub struct FileHandle {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub failed: bool,
    pub message: String,
}
impl FileHandle {
    pub fn terrane_construct(
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(raw, failure, detail);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) {
        {
            let __terrane_value = raw;
            self.handle = Some(__terrane_value);
        }
        {
            let __terrane_value = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for FileHandle {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct DirectoryHandle {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub failed: bool,
    pub message: String,
}
impl DirectoryHandle {
    pub fn terrane_construct(
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(raw, failure, detail);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) {
        {
            let __terrane_value = raw;
            self.handle = Some(__terrane_value);
        }
        {
            let __terrane_value = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for DirectoryHandle {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub fn open_file(
    capability: Filesystem,
    target: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> FileHandle {
    let raw: TerranePlatformOpenResult;
    let failure: bool;
    let detail: String;
    let acquired: TerranePlatformStreamHandle;
    let _ = &capability;
    raw = terrane_platform_open_file(target.text, readable, writable, create, truncate);
    failure = raw.failed;
    detail = raw.message.clone();
    acquired = raw.handle.clone();
    return FileHandle::terrane_construct(acquired, failure, detail);
}
pub fn file_read(
    capability: Filesystem,
    file: &FileHandle,
    limit: terrane_int_support::Int,
) -> FileData {
    let raw: TerranePlatformReadResult;
    let _ = &capability;
    raw = terrane_platform_read(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
        limit,
    );
    return FileData::terrane_construct(
        raw.data.clone(),
        raw.completed.clone(),
        raw.end,
        raw.failed,
        raw.message.clone(),
    );
}
pub fn file_write(
    capability: Filesystem,
    file: &FileHandle,
    data: Vec<u8>,
    offset: terrane_int_support::Int,
) -> FileData {
    let raw: TerranePlatformWriteResult;
    let _ = &capability;
    raw = terrane_platform_write(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
        &data,
        terrane_int_support::Int::from(offset.clone()),
    );
    return FileData::terrane_construct(
        data,
        raw.completed.clone(),
        false,
        raw.failed,
        raw.message.clone(),
    );
}
pub fn file_flush(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_flush(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
    );
    return FilesystemOperationResult::terrane_construct(raw.failed, raw.message.clone());
}
pub fn file_sync_data(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_sync_data(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
    );
    return FilesystemOperationResult::terrane_construct(raw.failed, raw.message.clone());
}
pub fn file_sync_all(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_sync_all(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
    );
    return FilesystemOperationResult::terrane_construct(raw.failed, raw.message.clone());
}
pub fn file_close(
    capability: Filesystem,
    file: FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_close(
        &file.handle.as_ref().expect("required field initialized"),
    );
    return FilesystemOperationResult::terrane_construct(raw.failed, raw.message.clone());
}
#[derive(Clone)]
pub struct Filesystem {
    pub authority: Option<TerraneFilesystemAuthority>,
}
impl Filesystem {
    pub fn terrane_construct(authority: TerraneFilesystemAuthority) -> Self {
        let mut __terrane_constructed_value = Self { authority: None };
        __terrane_constructed_value.construct(authority);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, authority: TerraneFilesystemAuthority) {
        {
            let __terrane_value = authority;
            self.authority = Some(__terrane_value);
        }
    }
}
pub fn filesystem_capability() -> Filesystem {
    let authority: TerraneFilesystemAuthority;
    authority = terrane_acquire_filesystem_authority();
    return Filesystem::terrane_construct(authority);
}
pub fn filesystem_exists(capability: Filesystem, target: Path) -> ExistenceResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_exists(target.text);
    return ExistenceResult::terrane_construct(
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_metadata(capability: Filesystem, target: Path) -> FileMetadata {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_metadata(target.text, true);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_symlink_metadata(
    capability: Filesystem,
    target: Path,
) -> FileMetadata {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_metadata(target.text, false);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_canonical(capability: Filesystem, target: Path) -> PathResult {
    let record: TerraneFilesystemResult;
    let resolved: Path;
    let _ = &capability;
    record = terrane_filesystem_realpath(target.text);
    resolved = Path::terrane_construct(terrane_filesystem_result_text(&record));
    return PathResult::terrane_construct(
        resolved,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_realpath(capability: Filesystem, target: Path) -> PathResult {
    return filesystem_canonical(capability, target);
}
pub fn filesystem_read_link(capability: Filesystem, target: Path) -> PathResult {
    let record: TerraneFilesystemResult;
    let linked: Path;
    let _ = &capability;
    record = terrane_filesystem_read_link(target.text);
    linked = Path::terrane_construct(terrane_filesystem_result_text(&record));
    return PathResult::terrane_construct(
        linked,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_open_beneath(
    capability: Filesystem,
    directory: Path,
    relative: Path,
    cross_filesystem: bool,
) -> DirectoryHandle {
    let raw: TerranePlatformOpenResult;
    let failure: bool;
    let detail: String;
    let acquired: TerranePlatformStreamHandle;
    let _ = &capability;
    raw = terrane_platform_open_directory_beneath(
        directory.text,
        relative.text,
        cross_filesystem,
    );
    failure = raw.failed;
    detail = raw.message.clone();
    acquired = raw.handle.clone();
    return DirectoryHandle::terrane_construct(acquired, failure, detail);
}
pub fn open_file_beneath(
    capability: Filesystem,
    directory: &DirectoryHandle,
    relative: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> FileHandle {
    let raw: TerranePlatformOpenResult;
    let failure: bool;
    let detail: String;
    let acquired: TerranePlatformStreamHandle;
    let _ = &capability;
    raw = terrane_platform_open_file_beneath(
        &{
            let __terrane_snapshot = directory
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
        relative.text,
        readable,
        writable,
        create,
        truncate,
    );
    failure = raw.failed;
    detail = raw.message.clone();
    acquired = raw.handle.clone();
    return FileHandle::terrane_construct(acquired, failure, detail);
}
pub fn filesystem_read_bounded(
    capability: Filesystem,
    target: Path,
    limit: terrane_int_support::Int,
) -> FileData {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_read_bounded(target.text, limit);
    return FileData::terrane_construct(
        terrane_filesystem_result_bytes(&record),
        terrane_filesystem_result_int(&record),
        true,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_write_atomic(
    capability: Filesystem,
    target: Path,
    data: Vec<u8>,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_write_atomic(target.text, data);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_rename(
    capability: Filesystem,
    source: Path,
    destination: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_rename(source.text, destination.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_remove(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_remove(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
// Source: core/streams.trn
// Namespace: core/streams
#[derive(Clone)]
pub struct StreamOperationResult {
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}
impl StreamOperationResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        deadline_exceeded: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            deadline_exceeded: false,
        };
        __terrane_constructed_value.construct(failed, message, deadline_exceeded);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, deadline_exceeded: bool) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = deadline_exceeded;
            self.deadline_exceeded = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct BufferBytesResult {
    pub failed: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl BufferBytesResult {
    pub fn terrane_construct(failed: bool, message: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: Vec::<u8>::from([]),
        };
        __terrane_constructed_value.construct(failed, message, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, data: Vec<u8>) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = data;
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct BufferTextResult {
    pub failed: bool,
    pub message: String,
    pub value: String,
}
impl BufferTextResult {
    pub fn terrane_construct(failed: bool, message: String, text: String) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: String::from(""),
        };
        __terrane_constructed_value.construct(failed, message, text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, text: String) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = text;
            self.value = __terrane_value;
        }
    }
}
pub struct ReadBufferResult {
    pub failed: bool,
    pub message: String,
    pub value: OwnedReadBuffer,
}
impl ReadBufferResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        buffer: OwnedReadBuffer,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: OwnedReadBuffer::terrane_construct(terrane_platform_no_resource()),
        };
        __terrane_constructed_value.construct(failed, message, buffer);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, buffer: OwnedReadBuffer) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = buffer;
            self.value = __terrane_value;
        }
    }
}
pub struct OwnedReadBuffer {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformCapability>,
}
impl OwnedReadBuffer {
    pub fn terrane_construct(handle: TerranePlatformCapability) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
        };
        __terrane_constructed_value.construct(handle);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, handle: TerranePlatformCapability) {
        {
            let __terrane_value = handle;
            self.handle = Some(__terrane_value);
        }
    }
    pub fn capacity(&self) -> terrane_int_support::Int {
        return terrane_platform_buffer_capacity(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
    pub fn completed(&self) -> terrane_int_support::Int {
        return terrane_platform_buffer_completed(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
    pub fn clear(&mut self) -> StreamOperationResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_buffer_clear(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            false,
        );
    }
    pub fn filled_bytes(&self) -> BufferBytesResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_buffer_filled_bytes(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return BufferBytesResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn filled_text(&self) -> BufferTextResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_buffer_filled_text(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return BufferTextResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_text(&raw),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_buffer_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for OwnedReadBuffer {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct BufferedReadResult {
    pub buffer: Option<OwnedReadBuffer>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}
impl BufferedReadResult {
    pub fn terrane_construct(
        buffer: OwnedReadBuffer,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
        deadline_exceeded: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            buffer: None,
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
            deadline_exceeded: false,
        };
        __terrane_constructed_value
            .construct(buffer, completed, end, failed, message, deadline_exceeded);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        buffer: OwnedReadBuffer,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
        deadline_exceeded: bool,
    ) {
        {
            let __terrane_value = buffer;
            self.buffer = Some(__terrane_value);
        }
        {
            let __terrane_value = completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value = end;
            self.end = __terrane_value;
        }
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = deadline_exceeded;
            self.deadline_exceeded = __terrane_value;
        }
    }
}
pub fn make_read_buffer(capacity: terrane_int_support::Int) -> ReadBufferResult {
    let raw: TerranePlatformResult;
    let buffer: OwnedReadBuffer;
    raw = terrane_platform_buffer_create(&capacity);
    buffer = OwnedReadBuffer::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return ReadBufferResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        buffer,
    );
}
#[derive(Clone)]
pub struct ReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}
impl ReadResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
        deadline_exceeded: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::<u8>::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
            deadline_exceeded: false,
        };
        __terrane_constructed_value
            .construct(data, completed, end, failed, message, deadline_exceeded);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
        deadline_exceeded: bool,
    ) {
        {
            let __terrane_value = data;
            self.data = __terrane_value;
        }
        {
            let __terrane_value = completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value = end;
            self.end = __terrane_value;
        }
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = deadline_exceeded;
            self.deadline_exceeded = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct TextReadResult {
    pub text: String,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl TextReadResult {
    pub fn terrane_construct(
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            text: String::from(""),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(text, completed, end, failed, message);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
        {
            let __terrane_value = completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value = end;
            self.end = __terrane_value;
        }
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct WriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
    pub deadline_exceeded: bool,
}
impl WriteResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
        deadline_exceeded: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::<u8>::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            failed: false,
            message: String::from(""),
            deadline_exceeded: false,
        };
        __terrane_constructed_value
            .construct(data, completed, failed, message, deadline_exceeded);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
        deadline_exceeded: bool,
    ) {
        if completed.clone() == terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            {
                let __terrane_value = Vec::<u8>::from([]);
                self.data = __terrane_value;
            }
        } else {
            {
                let __terrane_value = data;
                self.data = __terrane_value;
            }
        }
        {
            let __terrane_value = completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = deadline_exceeded;
            self.deadline_exceeded = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct ExactReadProgress {
    pub target: terrane_int_support::Int,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl ExactReadProgress {
    pub fn terrane_construct(count: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            target: terrane_int_support::Int::from(0_i128),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(count);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, count: terrane_int_support::Int) {
        {
            let __terrane_value = count.clone();
            self.target = __terrane_value;
        }
    }
    pub fn pending(&self) -> bool {
        return self.completed.clone() < self.target.clone() && !self.end && !self.failed
            && !self.deadline_exceeded;
    }
    pub fn remaining(&self) -> terrane_int_support::Int {
        return self.target.clone() - self.completed.clone();
    }
    pub fn advance(
        &mut self,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
    ) {
        {
            let __terrane_value = self.completed.clone() + completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value = end;
            self.end = __terrane_value;
        }
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = deadline_exceeded;
            self.deadline_exceeded = __terrane_value;
        }
        {
            let __terrane_value = message.clone();
            self.message = __terrane_value;
        }
        if completed.clone() == terrane_int_support::Int::from(0_i128) && !end && !failed
            && !deadline_exceeded
        {
            {
                let __terrane_value = true;
                self.failed = __terrane_value;
            }
            {
                let __terrane_value = String::from("stream read made no progress");
                self.message = __terrane_value;
            }
        }
        return ();
    }
    pub fn result(&self, data: Vec<u8>) -> ReadResult {
        let mut failed: bool;
        let mut message: String;
        failed = self.failed;
        message = self.message.clone();
        if self.end && self.completed.clone() < self.target.clone() && !failed
            && !self.deadline_exceeded
        {
            failed = true;
            message = String::from("stream ended before exact byte count");
        }
        return ReadResult::terrane_construct(
            data,
            self.completed.clone(),
            self.end,
            failed,
            message,
            self.deadline_exceeded,
        );
    }
}
#[derive(Clone)]
pub struct WriteProgress {
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl WriteProgress {
    pub fn terrane_construct() -> Self {
        Self {
            completed: terrane_int_support::Int::from(0_i128),
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        }
    }
    pub fn pending(&self, count: terrane_int_support::Int) -> bool {
        return self.completed.clone() < count.clone() && !self.failed
            && !self.deadline_exceeded;
    }
    pub fn advance(
        &mut self,
        completed: terrane_int_support::Int,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
    ) {
        {
            let __terrane_value = self.completed.clone() + completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = deadline_exceeded;
            self.deadline_exceeded = __terrane_value;
        }
        {
            let __terrane_value = message.clone();
            self.message = __terrane_value;
        }
        if completed.clone() == terrane_int_support::Int::from(0_i128) && !failed
            && !deadline_exceeded
        {
            {
                let __terrane_value = true;
                self.failed = __terrane_value;
            }
            {
                let __terrane_value = String::from("stream write made no progress");
                self.message = __terrane_value;
            }
        }
        return ();
    }
    pub fn result(&self, data: Vec<u8>) -> WriteResult {
        return WriteResult::terrane_construct(
            data,
            self.completed.clone(),
            self.failed,
            self.message.clone(),
            self.deadline_exceeded,
        );
    }
}
pub trait AsyncByteReaderProtocol: Send + Sync {
    fn read_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ReadResult> + Send + '_>>;
    fn read_exact_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ReadResult> + Send + '_>>;
    fn read_into_async(
        &self,
        buffer: OwnedReadBuffer,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = BufferedReadResult> + Send + '_>,
    >;
    fn close(self: Box<Self>) -> StreamOperationResult;
}
pub struct AsyncByteReader(Box<dyn AsyncByteReaderProtocol>);
impl AsyncByteReader {
    pub async fn read_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> ReadResult {
        self.0.read_async(count, options).await
    }
    pub async fn read_exact_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> ReadResult {
        self.0.read_exact_async(count, options).await
    }
    pub async fn read_into_async(
        &self,
        buffer: OwnedReadBuffer,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> BufferedReadResult {
        self.0.read_into_async(buffer, count, options).await
    }
    pub fn close(self) -> StreamOperationResult {
        self.0.close()
    }
}
pub trait AsyncByteWriterProtocol: Send + Sync {
    fn write_async(
        &self,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = WriteResult> + Send + '_>>;
    fn write_all_async(
        &self,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = WriteResult> + Send + '_>>;
    fn close(self: Box<Self>) -> StreamOperationResult;
}
pub struct AsyncByteWriter(Box<dyn AsyncByteWriterProtocol>);
impl AsyncByteWriter {
    pub async fn write_async(
        &self,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> WriteResult {
        self.0.write_async(data, options).await
    }
    pub async fn write_all_async(
        &self,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> WriteResult {
        self.0.write_all_async(data, options).await
    }
    pub fn close(self) -> StreamOperationResult {
        self.0.close()
    }
}
pub struct ByteReader {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
}
impl ByteReader {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
        };
        __terrane_constructed_value.construct(handle);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        {
            let __terrane_value = handle;
            self.handle = Some(__terrane_value);
        }
    }
    pub fn read(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw: TerranePlatformReadResult;
        raw = terrane_platform_read(
            &self.handle.as_ref().expect("required field initialized"),
            count,
        );
        return ReadResult::terrane_construct(
            raw.data.clone(),
            raw.completed.clone(),
            raw.end,
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn read_exact(&self, count: terrane_int_support::Int) -> ReadResult {
        let mut data: Vec<u8>;
        let mut completed: terrane_int_support::Int;
        let mut end: bool;
        let mut failed: bool;
        let mut message: String;
        let mut part: TerranePlatformReadResult;
        data = Vec::<u8>::from([]);
        completed = terrane_int_support::Int::from(0_i128);
        end = false;
        failed = false;
        message = String::from("");
        while completed.clone() < count.clone() && !end && !failed {
            part = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                count.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                let part_0: Vec<u8> = part.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        if end && completed.clone() < count.clone() && !failed {
            failed = true;
            message = String::from("stream ended before exact byte count");
        }
        return ReadResult::terrane_construct(
            data,
            completed.clone(),
            end,
            failed,
            message,
            false,
        );
    }
    pub fn read_all(&self, count: terrane_int_support::Int) -> ReadResult {
        let mut data: Vec<u8>;
        let mut completed: terrane_int_support::Int;
        let mut end: bool;
        let mut failed: bool;
        let mut message: String;
        let mut part: TerranePlatformReadResult;
        data = Vec::<u8>::from([]);
        completed = terrane_int_support::Int::from(0_i128);
        end = false;
        failed = false;
        message = String::from("");
        while completed.clone() < count.clone() && !end && !failed {
            part = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                count.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                let part_0: Vec<u8> = part.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        return ReadResult::terrane_construct(
            data,
            completed.clone(),
            end,
            failed,
            message,
            false,
        );
    }
    pub async fn read_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> ReadResult {
        let operation_timeout: terrane_int_support::Int;
        let operation_token: Option<TerranePlatformCapability>;
        let raw: TerranePlatformReadResult;
        operation_timeout = operation_deadline(options.clone());
        operation_token = operation_cancellation(options);
        raw = __terrane_await(
                terrane_platform_read_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    count,
                    operation_timeout,
                    operation_token.as_ref(),
                ),
            )
            .await;
        return ReadResult::terrane_construct(
            raw.data.clone(),
            raw.completed.clone(),
            raw.end,
            raw.failed,
            raw.message.clone(),
            raw.deadline_exceeded,
        );
    }
    pub async fn __terrane_shared_async_read_async(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> ReadResult {
        let operation_timeout: terrane_int_support::Int;
        let operation_token: Option<TerranePlatformCapability>;
        let raw: TerranePlatformReadResult;
        operation_timeout = operation_deadline(options.clone());
        operation_token = operation_cancellation(options);
        raw = __terrane_await(
                terrane_platform_read_async(
                    &{
                        let __terrane_snapshot = __terrane_shared_handle
                            .lock()
                            .expect("shared async receiver lock poisoned")
                            .handle
                            .as_ref()
                            .expect("required field initialized")
                            .clone();
                        __terrane_snapshot
                    },
                    count,
                    operation_timeout,
                    operation_token.as_ref(),
                ),
            )
            .await;
        return ReadResult::terrane_construct(
            raw.data.clone(),
            raw.completed.clone(),
            raw.end,
            raw.failed,
            raw.message.clone(),
            raw.deadline_exceeded,
        );
    }
    pub async fn read_exact_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> ReadResult {
        let mut progress: ExactReadProgress;
        let mut data: Vec<u8>;
        let mut part: ReadResult;
        if count.clone() < terrane_int_support::Int::from(0_i128) {
            return __terrane_await(self.read_async(count.clone(), options.clone()))
                .await;
        }
        progress = ExactReadProgress::terrane_construct(count.clone());
        data = Vec::<u8>::from([]);
        while progress.pending() {
            part = __terrane_await(
                    self.read_async(progress.remaining(), options.clone()),
                )
                .await;
            data = {
                let mut bytes = data;
                let part_0: Vec<u8> = part.data;
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            progress
                .advance(
                    part.completed.clone(),
                    part.end,
                    part.failed,
                    part.deadline_exceeded,
                    part.message.clone(),
                );
        }
        return progress.result(data);
    }
    pub async fn __terrane_shared_async_read_exact_async(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> ReadResult {
        let mut progress: ExactReadProgress;
        let mut data: Vec<u8>;
        let mut part: ReadResult;
        if count.clone() < terrane_int_support::Int::from(0_i128) {
            return __terrane_await({
                    let __terrane_shared_call_handle = __terrane_shared_handle.clone();
                    let __terrane_shared_argument_0 = count.clone();
                    let __terrane_shared_argument_1 = options.clone();
                    let __terrane_shared_argument_2 = __terrane_shared_argument_0;
                    let __terrane_shared_argument_3 = __terrane_shared_argument_1;
                    ByteReader::__terrane_shared_async_read_async(
                        __terrane_shared_call_handle,
                        __terrane_shared_argument_2,
                        __terrane_shared_argument_3,
                    )
                })
                .await;
        }
        progress = ExactReadProgress::terrane_construct(count.clone());
        data = Vec::<u8>::from([]);
        while progress.pending() {
            part = __terrane_await({
                    let __terrane_shared_call_handle = __terrane_shared_handle.clone();
                    let __terrane_shared_argument_0 = progress.remaining();
                    let __terrane_shared_argument_1 = options.clone();
                    let __terrane_shared_argument_2 = __terrane_shared_argument_0;
                    let __terrane_shared_argument_3 = __terrane_shared_argument_1;
                    ByteReader::__terrane_shared_async_read_async(
                        __terrane_shared_call_handle,
                        __terrane_shared_argument_2,
                        __terrane_shared_argument_3,
                    )
                })
                .await;
            data = {
                let mut bytes = data;
                let part_0: Vec<u8> = part.data;
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            progress
                .advance(
                    part.completed.clone(),
                    part.end,
                    part.failed,
                    part.deadline_exceeded,
                    part.message.clone(),
                );
        }
        return progress.result(data);
    }
    pub async fn read_into_async(
        &self,
        buffer: OwnedReadBuffer,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> BufferedReadResult {
        let operation_timeout: terrane_int_support::Int;
        let operation_token: Option<TerranePlatformCapability>;
        let raw: TerranePlatformResult;
        let completed: terrane_int_support::Int;
        let end: bool;
        let failed: bool;
        let message: String;
        let deadline_exceeded: bool;
        operation_timeout = operation_deadline(options.clone());
        operation_token = operation_cancellation(options);
        raw = __terrane_await(
                terrane_platform_stream_read_into_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    &buffer.handle.as_ref().expect("required field initialized"),
                    count.clone(),
                    operation_timeout.clone(),
                    operation_token.as_ref(),
                ),
            )
            .await;
        completed = terrane_platform_result_int(&raw);
        end = terrane_platform_result_bool(&raw);
        failed = terrane_platform_result_failed(&raw);
        message = terrane_platform_result_message(&raw);
        deadline_exceeded = terrane_platform_result_deadline_exceeded(&raw);
        return BufferedReadResult::terrane_construct(
            buffer,
            completed.clone(),
            end,
            failed,
            message,
            deadline_exceeded,
        );
    }
    pub async fn __terrane_shared_async_read_into_async(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        buffer: OwnedReadBuffer,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> BufferedReadResult {
        let operation_timeout: terrane_int_support::Int;
        let operation_token: Option<TerranePlatformCapability>;
        let raw: TerranePlatformResult;
        let completed: terrane_int_support::Int;
        let end: bool;
        let failed: bool;
        let message: String;
        let deadline_exceeded: bool;
        operation_timeout = operation_deadline(options.clone());
        operation_token = operation_cancellation(options);
        raw = __terrane_await(
                terrane_platform_stream_read_into_async(
                    &{
                        let __terrane_snapshot = __terrane_shared_handle
                            .lock()
                            .expect("shared async receiver lock poisoned")
                            .handle
                            .as_ref()
                            .expect("required field initialized")
                            .clone();
                        __terrane_snapshot
                    },
                    &buffer.handle.as_ref().expect("required field initialized"),
                    count.clone(),
                    operation_timeout.clone(),
                    operation_token.as_ref(),
                ),
            )
            .await;
        completed = terrane_platform_result_int(&raw);
        end = terrane_platform_result_bool(&raw);
        failed = terrane_platform_result_failed(&raw);
        message = terrane_platform_result_message(&raw);
        deadline_exceeded = terrane_platform_result_deadline_exceeded(&raw);
        return BufferedReadResult::terrane_construct(
            buffer,
            completed.clone(),
            end,
            failed,
            message,
            deadline_exceeded,
        );
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextReader {
        return TextReader::terrane_construct(
            self.handle.as_ref().expect("required field initialized").clone(),
            codec,
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl AsyncByteReaderProtocol for ByteReader {
    fn read_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ReadResult> + Send + '_>> {
        Box::pin(async move { ByteReader::read_async(&*self, count, options).await })
    }
    fn read_exact_async(
        &self,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ReadResult> + Send + '_>> {
        Box::pin(async move {
            ByteReader::read_exact_async(&*self, count, options).await
        })
    }
    fn read_into_async(
        &self,
        buffer: OwnedReadBuffer,
        count: terrane_int_support::Int,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = BufferedReadResult> + Send + '_>,
    > {
        Box::pin(async move {
            ByteReader::read_into_async(&*self, buffer, count, options).await
        })
    }
    fn close(self: Box<Self>) -> StreamOperationResult {
        ByteReader::close(*self)
    }
}
impl From<ByteReader> for AsyncByteReader {
    fn from(value: ByteReader) -> Self {
        Self(Box::new(value))
    }
}
impl Drop for ByteReader {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct ByteWriter {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
}
impl ByteWriter {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
        };
        __terrane_constructed_value.construct(handle);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        {
            let __terrane_value = handle;
            self.handle = Some(__terrane_value);
        }
    }
    pub fn write(&self, data: Vec<u8>) -> WriteResult {
        let offset: i64;
        let raw: TerranePlatformWriteResult;
        offset = 0;
        raw = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &data,
            terrane_int_support::Int::from(offset.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw.completed.clone(),
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn write_all(&self, data: Vec<u8>) -> WriteResult {
        let mut completed: terrane_int_support::Int;
        let mut failed: bool;
        let mut message: String;
        let mut part: TerranePlatformWriteResult;
        completed = terrane_int_support::Int::from(0_i128);
        failed = false;
        message = String::from("");
        while completed.clone() < terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            part = terrane_platform_write(
                &self.handle.as_ref().expect("required field initialized"),
                &data,
                terrane_int_support::Int::from(completed.clone()),
            );
            completed = completed.clone() + part.completed.clone();
            failed = part.failed;
            message = part.message.clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.failed
            {
                failed = true;
                message = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(
            data,
            completed.clone(),
            failed,
            message,
            false,
        );
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw: TerranePlatformWriteResult;
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        raw = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw.completed.clone(),
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub async fn write_async(
        &self,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> WriteResult {
        let operation_timeout: terrane_int_support::Int;
        let operation_token: Option<TerranePlatformCapability>;
        let raw: TerranePlatformWriteResult;
        operation_timeout = operation_deadline(options.clone());
        operation_token = operation_cancellation(options);
        raw = __terrane_await(
                terrane_platform_write_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    data,
                    terrane_int_support::Int::from(0_i128),
                    operation_timeout,
                    operation_token.as_ref(),
                ),
            )
            .await;
        return WriteResult::terrane_construct(
            raw.data.clone(),
            raw.completed.clone(),
            raw.failed,
            raw.message.clone(),
            raw.deadline_exceeded,
        );
    }
    pub async fn __terrane_shared_async_write_async(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> WriteResult {
        let operation_timeout: terrane_int_support::Int;
        let operation_token: Option<TerranePlatformCapability>;
        let raw: TerranePlatformWriteResult;
        operation_timeout = operation_deadline(options.clone());
        operation_token = operation_cancellation(options);
        raw = __terrane_await(
                terrane_platform_write_async(
                    &{
                        let __terrane_snapshot = __terrane_shared_handle
                            .lock()
                            .expect("shared async receiver lock poisoned")
                            .handle
                            .as_ref()
                            .expect("required field initialized")
                            .clone();
                        __terrane_snapshot
                    },
                    data,
                    terrane_int_support::Int::from(0_i128),
                    operation_timeout,
                    operation_token.as_ref(),
                ),
            )
            .await;
        return WriteResult::terrane_construct(
            raw.data.clone(),
            raw.completed.clone(),
            raw.failed,
            raw.message.clone(),
            raw.deadline_exceeded,
        );
    }
    pub async fn write_all_async(
        &self,
        mut data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> WriteResult {
        let mut progress: WriteProgress;
        let mut operation_timeout: terrane_int_support::Int;
        let mut operation_token: Option<TerranePlatformCapability>;
        let mut part: TerranePlatformWriteResult;
        progress = WriteProgress::terrane_construct();
        while progress.pending(terrane_int_support::Int::from(data.len() as i128)) {
            operation_timeout = operation_deadline(options.clone());
            operation_token = operation_cancellation(options.clone());
            part = __terrane_await(
                    terrane_platform_write_async(
                        &self.handle.as_ref().expect("required field initialized"),
                        data,
                        progress.completed.clone(),
                        operation_timeout,
                        operation_token.as_ref(),
                    ),
                )
                .await;
            progress
                .advance(
                    part.completed.clone(),
                    part.failed,
                    part.deadline_exceeded,
                    part.message.clone(),
                );
            data = part.data.clone();
        }
        return progress.result(data);
    }
    pub async fn __terrane_shared_async_write_all_async(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        mut data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> WriteResult {
        let mut progress: WriteProgress;
        let mut operation_timeout: terrane_int_support::Int;
        let mut operation_token: Option<TerranePlatformCapability>;
        let mut part: TerranePlatformWriteResult;
        progress = WriteProgress::terrane_construct();
        while progress.pending(terrane_int_support::Int::from(data.len() as i128)) {
            operation_timeout = operation_deadline(options.clone());
            operation_token = operation_cancellation(options.clone());
            part = __terrane_await(
                    terrane_platform_write_async(
                        &{
                            let __terrane_snapshot = __terrane_shared_handle
                                .lock()
                                .expect("shared async receiver lock poisoned")
                                .handle
                                .as_ref()
                                .expect("required field initialized")
                                .clone();
                            __terrane_snapshot
                        },
                        data,
                        progress.completed.clone(),
                        operation_timeout,
                        operation_token.as_ref(),
                    ),
                )
                .await;
            progress
                .advance(
                    part.completed.clone(),
                    part.failed,
                    part.deadline_exceeded,
                    part.message.clone(),
                );
            data = part.data.clone();
        }
        return progress.result(data);
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextWriter {
        return TextWriter::terrane_construct(
            self.handle.as_ref().expect("required field initialized").clone(),
            codec,
        );
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_flush(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_sync_data(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_sync_all(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl AsyncByteWriterProtocol for ByteWriter {
    fn write_async(
        &self,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = WriteResult> + Send + '_>> {
        Box::pin(async move { ByteWriter::write_async(&*self, data, options).await })
    }
    fn write_all_async(
        &self,
        data: Vec<u8>,
        options: Option<IoOperationOptions>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = WriteResult> + Send + '_>> {
        Box::pin(async move { ByteWriter::write_all_async(&*self, data, options).await })
    }
    fn close(self: Box<Self>) -> StreamOperationResult {
        ByteWriter::close(*self)
    }
}
impl From<ByteWriter> for AsyncByteWriter {
    fn from(value: ByteWriter) -> Self {
        Self(Box::new(value))
    }
}
impl Drop for ByteWriter {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct TextReader {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub codec: terrane_string_support::Encoding,
}
impl TextReader {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            codec: terrane_string_support::Encoding::Utf8,
        };
        __terrane_constructed_value.construct(handle, codec);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        {
            let __terrane_value = handle;
            self.handle = Some(__terrane_value);
        }
        {
            let __terrane_value = codec;
            self.codec = __terrane_value;
        }
    }
    pub fn read(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw: TerranePlatformReadResult;
        let text: String;
        raw = terrane_platform_read(
            &self.handle.as_ref().expect("required field initialized"),
            count,
        );
        text = __terrane_raised_err(
            terrane_string_support::decode(&raw.data.clone(), self.codec),
            0 /* terrane-site: core/streams.trn:381:23-381:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                raw.completed.clone(),
                raw.end,
                raw.failed,
                raw.message.clone(),
            ),
        );
    }
    pub fn read_exact(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data: Vec<u8>;
        let mut completed: terrane_int_support::Int;
        let mut end: bool;
        let mut failed: bool;
        let mut message: String;
        let mut part: TerranePlatformReadResult;
        let text: String;
        data = Vec::<u8>::from([]);
        completed = terrane_int_support::Int::from(0_i128);
        end = false;
        failed = false;
        message = String::from("");
        while completed.clone() < count.clone() && !end && !failed {
            part = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                count.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                let part_0: Vec<u8> = part.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        if end && completed.clone() < count.clone() && !failed {
            failed = true;
            message = String::from("stream ended before exact byte count");
        }
        text = __terrane_raised_err(
            terrane_string_support::decode(&data, self.codec),
            1 /* terrane-site: core/streams.trn:403:23-403:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                completed.clone(),
                end,
                failed,
                message,
            ),
        );
    }
    pub fn read_all(
        &self,
        limit: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data: Vec<u8>;
        let mut completed: terrane_int_support::Int;
        let mut end: bool;
        let mut failed: bool;
        let mut message: String;
        let mut part: TerranePlatformReadResult;
        let text: String;
        data = Vec::<u8>::from([]);
        completed = terrane_int_support::Int::from(0_i128);
        end = false;
        failed = false;
        message = String::from("");
        while completed.clone() < limit.clone() && !end && !failed {
            part = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                limit.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                let part_0: Vec<u8> = part.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        text = __terrane_raised_err(
            terrane_string_support::decode(&data, self.codec),
            2 /* terrane-site: core/streams.trn:422:23-422:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                completed.clone(),
                end,
                failed,
                message,
            ),
        );
    }
    pub async fn read_async(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw: TerranePlatformReadResult;
        let text: String;
        raw = __terrane_await(
                terrane_platform_read_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    count,
                    -terrane_int_support::Int::from(1_i128),
                    None.as_ref(),
                ),
            )
            .await;
        text = __terrane_raised_err(
            terrane_string_support::decode(&raw.data.clone(), self.codec),
            3 /* terrane-site: core/streams.trn:427:23-427:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                raw.completed.clone(),
                raw.end,
                raw.failed,
                raw.message.clone(),
            ),
        );
    }
    pub async fn __terrane_shared_async_read_async(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw: TerranePlatformReadResult;
        let text: String;
        raw = __terrane_await(
                terrane_platform_read_async(
                    &{
                        let __terrane_snapshot = __terrane_shared_handle
                            .lock()
                            .expect("shared async receiver lock poisoned")
                            .handle
                            .as_ref()
                            .expect("required field initialized")
                            .clone();
                        __terrane_snapshot
                    },
                    count,
                    -terrane_int_support::Int::from(1_i128),
                    None.as_ref(),
                ),
            )
            .await;
        text = __terrane_raised_err(
            terrane_string_support::decode(
                &raw.data.clone(),
                {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .codec
                        .clone();
                    __terrane_snapshot
                },
            ),
            3 /* terrane-site: core/streams.trn:427:23-427:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                raw.completed.clone(),
                raw.end,
                raw.failed,
                raw.message.clone(),
            ),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for TextReader {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct TextWriter {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub codec: terrane_string_support::Encoding,
}
impl TextWriter {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            codec: terrane_string_support::Encoding::Utf8,
        };
        __terrane_constructed_value.construct(handle, codec);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        {
            let __terrane_value = handle;
            self.handle = Some(__terrane_value);
        }
        {
            let __terrane_value = codec;
            self.codec = __terrane_value;
        }
    }
    pub fn write(&self, text: String) -> WriteResult {
        let data: Vec<u8>;
        let offset: i64;
        let raw: TerranePlatformWriteResult;
        data = terrane_string_support::encode(&text, self.codec);
        offset = 0;
        raw = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &data,
            terrane_int_support::Int::from(offset.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw.completed.clone(),
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn write_all(&self, text: String) -> WriteResult {
        let data: Vec<u8>;
        let mut completed: terrane_int_support::Int;
        let mut failed: bool;
        let mut message: String;
        let mut part: TerranePlatformWriteResult;
        data = terrane_string_support::encode(&text, self.codec);
        completed = terrane_int_support::Int::from(0_i128);
        failed = false;
        message = String::from("");
        while completed.clone() < terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            part = terrane_platform_write(
                &self.handle.as_ref().expect("required field initialized"),
                &data,
                terrane_int_support::Int::from(completed.clone()),
            );
            completed = completed.clone() + part.completed.clone();
            failed = part.failed;
            message = part.message.clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.failed
            {
                failed = true;
                message = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(
            data,
            completed.clone(),
            failed,
            message,
            false,
        );
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw: TerranePlatformWriteResult;
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        raw = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw.completed.clone(),
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn line(&self, text: String) -> WriteResult {
        return self
            .write_all({
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &text,
                    &mut __terrane_text,
                );
                __terrane_text.push_str("\n");
                __terrane_text
            });
    }
    pub async fn write_async(&self, text: String) -> WriteResult {
        return self.write(text);
    }
    pub async fn __terrane_shared_async_write_async(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        text: String,
    ) -> WriteResult {
        return {
            let __terrane_shared_argument_0 = text;
            let __terrane_shared_argument_1 = __terrane_shared_argument_0;
            let __terrane_call_result = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .write(__terrane_shared_argument_1);
            __terrane_call_result
        };
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_flush(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_sync_data(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_sync_all(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw: TerranePlatformUnitResult;
        raw = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw.failed,
            raw.message.clone(),
            false,
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for TextWriter {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub fn stdin() -> ByteReader {
    return ByteReader::terrane_construct(terrane_platform_acquire_stdin());
}
pub fn stdout() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stdout());
}
pub fn stderr() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stderr());
}
// Source: core/paths.trn
// Namespace: core/filesystem/paths
#[derive(Clone)]
pub struct Path {
    pub text: String,
}
impl Path {
    pub fn terrane_construct(input: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(input);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, input: String) {
        {
            let __terrane_value = input;
            self.text = __terrane_value;
        }
    }
}
pub fn path_components(subject: Path) -> terrane_collection_support::List<String> {
    let parts: Vec<String>;
    let mut result: terrane_collection_support::List<String>;
    let mut index: terrane_int_support::Int;
    let mut part: String;
    parts = terrane_string_support::split(&subject.text, &String::from("/"));
    result = terrane_collection_support::List::<String>::new(vec![]);
    index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = result.make_unique();
        while index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
            part = __terrane_raised(
                {
                    let __terrane_receiver = &parts;
                    let __terrane_index = __terrane_raised(
                        terrane_collection_support::index_from_int(&index.clone()),
                        4 /* terrane-site: core/paths.trn:16:16-16:28 */,
                    );
                    __terrane_receiver
                        .get(__terrane_index)
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index,
                        ))
                },
                4 /* terrane-site: core/paths.trn:16:16-16:28 */,
            );
            if part.as_str() != "" {
                {
                    let __terrane_collection_argument_0 = part;
                    __terrane_list_append_0.push(__terrane_collection_argument_0);
                };
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return result;
}
pub fn path_is_absolute(subject: Path) -> bool {
    return subject.text.starts_with(&String::from("/"));
}
pub fn normalise_path(subject: Path) -> Path {
    let parts: Vec<String>;
    let absolute: bool;
    let mut kept: terrane_collection_support::List<String>;
    let mut count: terrane_int_support::Int;
    let mut part_index: terrane_int_support::Int;
    let mut part: String;
    let mut result: String;
    let mut index: terrane_int_support::Int;
    parts = terrane_string_support::split(&subject.text, &String::from("/"));
    absolute = path_is_absolute(subject);
    kept = terrane_collection_support::List::<String>::new(vec![]);
    count = terrane_int_support::Int::from(0_i128);
    part_index = terrane_int_support::Int::from(0_i128);
    while part_index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        part = __terrane_raised(
            {
                let __terrane_receiver = &parts;
                let __terrane_index = __terrane_raised(
                    terrane_collection_support::index_from_int(&part_index.clone()),
                    5 /* terrane-site: core/paths.trn:32:16-32:33 */,
                );
                __terrane_receiver
                    .get(__terrane_index)
                    .cloned()
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index,
                    ))
            },
            5 /* terrane-site: core/paths.trn:32:16-32:33 */,
        );
        if part.as_str() != "" && part.as_str() != "." {
            if part.as_str() == ".." {
                if count.clone() > terrane_int_support::Int::from(0_i128)
                    && __terrane_raised(
                            kept
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &(count.clone() - terrane_int_support::Int::from(1_i128)),
                                        ),
                                        6 /* terrane-site: core/paths.trn:35:34-35:49 */,
                                    ),
                                ),
                            6 /* terrane-site: core/paths.trn:35:34-35:49 */,
                        )
                        .as_str() != ".."
                {
                    count = count.clone() - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute {
                        if count.clone()
                            < terrane_int_support::Int::from(
                                terrane_int_support::Int::from(kept.length()),
                            )
                        {
                            {
                                let __terrane_collection_argument_0 = __terrane_raised(
                                    terrane_collection_support::index_from_int(&count.clone()),
                                    7 /* terrane-site: core/paths.trn:40:29-40:50 */,
                                );
                                let __terrane_collection_argument_1 = part;
                                __terrane_raised(
                                    kept
                                        .set(
                                            __terrane_collection_argument_0,
                                            __terrane_collection_argument_1,
                                        ),
                                    7 /* terrane-site: core/paths.trn:40:29-40:50 */,
                                );
                            };
                        } else {
                            {
                                let __terrane_collection_argument_0 = part;
                                kept.append(__terrane_collection_argument_0);
                            };
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
                    {
                        let __terrane_collection_argument_0 = __terrane_raised(
                            terrane_collection_support::index_from_int(&count.clone()),
                            8 /* terrane-site: core/paths.trn:46:21-46:42 */,
                        );
                        let __terrane_collection_argument_1 = part;
                        __terrane_raised(
                            kept
                                .set(
                                    __terrane_collection_argument_0,
                                    __terrane_collection_argument_1,
                                ),
                            8 /* terrane-site: core/paths.trn:46:21-46:42 */,
                        );
                    };
                } else {
                    {
                        let __terrane_collection_argument_0 = part;
                        kept.append(__terrane_collection_argument_0);
                    };
                }
                count = count.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
        part_index = part_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    result = String::from("");
    index = terrane_int_support::Int::from(0_i128);
    while index.clone() < count.clone() {
        if result.as_str() != "" {
            result = {
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &result,
                    &mut __terrane_text,
                );
                __terrane_text.push_str("/");
                __terrane_text
            };
        }
        result = {
            let mut __terrane_text = String::with_capacity(0);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &__terrane_raised(
                    kept
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                9 /* terrane-site: core/paths.trn:56:33-56:44 */,
                            ),
                        ),
                    9 /* terrane-site: core/paths.trn:56:33-56:44 */,
                ),
                &mut __terrane_text,
            );
            __terrane_text
        };
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    if absolute {
        result = {
            let mut __terrane_text = String::with_capacity(1);
            __terrane_text.push_str("/");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
    if result.as_str() == "" && absolute {
        result = String::from("/");
    }
    return Path::terrane_construct(result);
}
pub fn path_name(subject: Path) -> String {
    let normal: Path;
    let parts: terrane_collection_support::List<String>;
    normal = normalise_path(subject);
    parts = path_components(normal);
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
                    10 /* terrane-site: core/paths.trn:69:12-69:35 */,
                ),
            ),
        10 /* terrane-site: core/paths.trn:69:12-69:35 */,
    );
}
pub fn path_parent(subject: Path) -> Path {
    let normal: Path;
    let parts: terrane_collection_support::List<String>;
    let mut result: String;
    let mut index: terrane_int_support::Int;
    let absolute: bool;
    normal = normalise_path(subject);
    parts = path_components(normal.clone());
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
    result = String::from("");
    index = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
            - terrane_int_support::Int::from(1_i128)
    {
        if result.as_str() != "" {
            result = {
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &result,
                    &mut __terrane_text,
                );
                __terrane_text.push_str("/");
                __terrane_text
            };
        }
        result = {
            let mut __terrane_text = String::with_capacity(0);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &__terrane_raised(
                    parts
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                11 /* terrane-site: core/paths.trn:83:33-83:45 */,
                            ),
                        ),
                    11 /* terrane-site: core/paths.trn:83:33-83:45 */,
                ),
                &mut __terrane_text,
            );
            __terrane_text
        };
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    absolute = path_is_absolute(normal);
    if absolute {
        result = {
            let mut __terrane_text = String::with_capacity(1);
            __terrane_text.push_str("/");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
    return Path::terrane_construct(result);
}
pub fn path_stem(subject: Path) -> String {
    let current: String;
    let pieces: Vec<String>;
    let mut result: String;
    let mut index: terrane_int_support::Int;
    current = path_name(subject);
    pieces = terrane_string_support::split(&current, &String::from("."));
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return current.clone();
    }
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
                {
                    let __terrane_receiver = &pieces;
                    let __terrane_index = __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        12 /* terrane-site: core/paths.trn:95:31-95:40 */,
                    );
                    __terrane_receiver
                        .get(__terrane_index)
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index,
                        ))
                },
                12 /* terrane-site: core/paths.trn:95:31-95:40 */,
            )
            .as_str() == ""
    {
        return current;
    }
    result = String::from("");
    index = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(pieces.len() as i128)
            - terrane_int_support::Int::from(1_i128)
    {
        if index.clone() > terrane_int_support::Int::from(0_i128) {
            result = {
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &result,
                    &mut __terrane_text,
                );
                __terrane_text.push_str(".");
                __terrane_text
            };
        }
        result = {
            let mut __terrane_text = String::with_capacity(0);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &__terrane_raised(
                    {
                        let __terrane_receiver = &pieces;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            13 /* terrane-site: core/paths.trn:102:33-102:46 */,
                        );
                        __terrane_receiver
                            .get(__terrane_index)
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_index,
                            ))
                    },
                    13 /* terrane-site: core/paths.trn:102:33-102:46 */,
                ),
                &mut __terrane_text,
            );
            __terrane_text
        };
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result;
}
pub fn path_extension(subject: Path) -> String {
    let current: String;
    let pieces: Vec<String>;
    current = path_name(subject);
    pieces = terrane_string_support::split(&current, &String::from("."));
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return String::from("");
    }
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
                {
                    let __terrane_receiver = &pieces;
                    let __terrane_index = __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        14 /* terrane-site: core/paths.trn:111:31-111:40 */,
                    );
                    __terrane_receiver
                        .get(__terrane_index)
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index,
                        ))
                },
                14 /* terrane-site: core/paths.trn:111:31-111:40 */,
            )
            .as_str() == ""
    {
        return String::from("");
    }
    return __terrane_raised(
        {
            let __terrane_receiver = &pieces;
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(pieces.len() as i128)
                        - terrane_int_support::Int::from(1_i128)),
                ),
                15 /* terrane-site: core/paths.trn:113:12-113:37 */,
            );
            __terrane_receiver
                .get(__terrane_index)
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index,
                ))
        },
        15 /* terrane-site: core/paths.trn:113:12-113:37 */,
    );
}
pub fn join_path(base: Path, child: Path) -> Path {
    let absolute: bool;
    let mut joined: String;
    let combined: Path;
    absolute = path_is_absolute(child.clone());
    if absolute {
        return normalise_path(child.clone());
    }
    joined = base.text.clone();
    if joined.as_str() != "" && !joined.ends_with(&String::from("/")) {
        joined = {
            let mut __terrane_text = String::with_capacity(1);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &joined,
                &mut __terrane_text,
            );
            __terrane_text.push_str("/");
            __terrane_text
        };
    }
    joined = {
        let mut __terrane_text = String::with_capacity(0);
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &joined,
            &mut __terrane_text,
        );
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &child.text,
            &mut __terrane_text,
        );
        __terrane_text
    };
    combined = Path::terrane_construct(joined);
    return normalise_path(combined);
}
// Source: core/io.trn
// Namespace: core/io
#[derive(Clone)]
pub struct CancellationToken {
    pub handle: TerranePlatformCapability,
}
impl CancellationToken {
    pub fn terrane_construct() -> Self {
        Self {
            handle: terrane_platform_cancellation_token(),
        }
    }
}
#[derive(Clone)]
pub struct IoOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl IoOperationResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(failed, deadline_exceeded, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, deadline_exceeded: bool, message: String) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = deadline_exceeded;
            self.deadline_exceeded = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
    }
}
pub fn cancel_operation(cancellation: CancellationToken) -> IoOperationResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_cancel(&cancellation.handle);
    return IoOperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct IoOperationOptions {
    pub deadline: Option<Deadline>,
    pub cancellation: Option<CancellationToken>,
}
impl IoOperationOptions {
    pub fn terrane_construct(
        requested: Option<Deadline>,
        cancellation: Option<CancellationToken>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            deadline: None,
            cancellation: None,
        };
        __terrane_constructed_value.construct(requested, cancellation);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        requested: Option<Deadline>,
        cancellation: Option<CancellationToken>,
    ) {
        {
            let __terrane_value = requested;
            self.deadline = __terrane_value;
        }
        {
            let __terrane_value = cancellation;
            self.cancellation = __terrane_value;
        }
    }
}
pub fn operation_cancellation(
    options: Option<IoOperationOptions>,
) -> Option<TerranePlatformCapability> {
    let selected: Option<CancellationToken>;
    if options.is_some() {
        selected = match &options {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .cancellation
            .clone();
        if selected.is_some() {
            return Some(
                match &selected {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .handle
                    .clone(),
            );
        }
    }
    return None;
}
pub fn operation_deadline(
    options: Option<IoOperationOptions>,
) -> terrane_int_support::Int {
    let selected: Option<Deadline>;
    let remaining: Option<Duration>;
    if options.is_some() {
        selected = match &options {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .deadline
            .clone();
        if selected.is_some() {
            remaining = match &selected {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .remaining();
            if remaining.is_some() {
                return match &remaining {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .total_nanoseconds
                    .clone();
            }
            return terrane_int_support::Int::from(0_i128);
        }
    }
    return terrane_int_support::Int::from(-1_i128);
}
// Source: core/time.trn
// Namespace: core/time
#[derive(Clone)]
pub struct InvalidDuration {
    pub message: String,
}
impl InvalidDuration {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("duration must be exact and non-negative"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DurationSubtraction {
    pub total_nanoseconds: terrane_int_support::Int,
}
impl DurationSubtraction {
    pub fn terrane_construct(total: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(total);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, total: terrane_int_support::Int) {
        {
            let __terrane_value = total.clone();
            self.total_nanoseconds = __terrane_value;
        }
    }
    pub fn checked(&self, other: Duration) -> Option<Duration> {
        let difference: terrane_int_support::Int;
        if self.total_nanoseconds.clone() < other.total_nanoseconds.clone() {
            return None;
        }
        difference = self.total_nanoseconds.clone() - other.total_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&difference, 1000000000.clone()),
                terrane_platform_time_mod(&difference, 1000000000.clone()),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration {
    pub seconds: terrane_int_support::Int,
    pub nanoseconds: terrane_int_support::Int,
    pub total_nanoseconds: terrane_int_support::Int,
    pub subtract: DurationSubtraction,
}
impl Duration {
    pub fn terrane_construct(
        whole_seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
            subtract: DurationSubtraction::terrane_construct(
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(whole_seconds, fractional_nanoseconds);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        whole_seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = whole_seconds.clone();
            self.seconds = __terrane_value;
        }
        {
            let __terrane_value = fractional_nanoseconds.clone();
            self.nanoseconds = __terrane_value;
        }
        {
            let __terrane_value = whole_seconds.clone()
                * terrane_int_support::Int::from(1000000000_i128)
                + fractional_nanoseconds.clone();
            self.total_nanoseconds = __terrane_value;
        }
        {
            let __terrane_value = DurationSubtraction::terrane_construct(
                self.total_nanoseconds.clone(),
            );
            self.subtract = __terrane_value;
        }
    }
    pub fn add(&self, other: Duration) -> Duration {
        let fractional: terrane_int_support::Int;
        fractional = self.nanoseconds.clone() + other.nanoseconds.clone();
        return Duration::terrane_construct(
            self.seconds.clone() + other.seconds.clone()
                + terrane_platform_time_div(&fractional, 1000000000.clone()),
            terrane_platform_time_mod(&fractional, 1000000000.clone()),
        );
    }
    pub fn multiply(
        &self,
        multiplier: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        let total: terrane_int_support::Int;
        if multiplier.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    16 /* terrane-site: core/time.trn:62:13-62:45 */,
                )
            });
        }
        total = self.total_nanoseconds.clone() * multiplier.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&total, 1000000000.clone()),
                terrane_platform_time_mod(&total, 1000000000.clone()),
            ),
        );
    }
    pub fn terrane_static_seconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    17 /* terrane-site: core/time.trn:38:13-38:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                value.clone(),
                terrane_int_support::Int::from(0_i128),
            ),
        );
    }
    pub fn terrane_static_milliseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    18 /* terrane-site: core/time.trn:43:13-43:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000.clone()),
                terrane_platform_time_mod(&value, 1000.clone())
                    * terrane_int_support::Int::from(1000000_i128),
            ),
        );
    }
    pub fn terrane_static_microseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    19 /* terrane-site: core/time.trn:48:13-48:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000.clone()),
                terrane_platform_time_mod(&value, 1000000.clone())
                    * terrane_int_support::Int::from(1000_i128),
            ),
        );
    }
    pub fn terrane_static_nanoseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    20 /* terrane-site: core/time.trn:53:13-53:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000000.clone()),
                terrane_platform_time_mod(&value, 1000000000.clone()),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MonotonicInstant {
    pub domain: terrane_int_support::Int,
    pub elapsed_nanoseconds: terrane_int_support::Int,
}
impl MonotonicInstant {
    pub fn terrane_construct(
        runtime_domain: terrane_int_support::Int,
        elapsed: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            domain: terrane_int_support::Int::from(0_i128),
            elapsed_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(runtime_domain, elapsed);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        runtime_domain: terrane_int_support::Int,
        elapsed: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = runtime_domain.clone();
            self.domain = __terrane_value;
        }
        {
            let __terrane_value = elapsed.clone();
            self.elapsed_nanoseconds = __terrane_value;
        }
    }
    pub fn duration_until(
        &self,
        later: &MonotonicInstant,
    ) -> Result<Duration, TerraneError> {
        let elapsed: terrane_int_support::Int;
        if self.domain.clone()
            != {
                let __terrane_snapshot = later.domain.clone();
                __terrane_snapshot
            }
            || {
                let __terrane_snapshot = later.elapsed_nanoseconds.clone();
                __terrane_snapshot
            } < self.elapsed_nanoseconds.clone()
        {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    21 /* terrane-site: core/time.trn:76:13-76:45 */,
                )
            });
        }
        elapsed = {
            let __terrane_snapshot = later.elapsed_nanoseconds.clone();
            __terrane_snapshot
        } - self.elapsed_nanoseconds.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed, 1000000000.clone()),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Instant {
    pub unix_seconds: terrane_int_support::Int,
    pub nanoseconds: terrane_int_support::Int,
}
impl Instant {
    pub fn terrane_construct(
        seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            unix_seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(seconds, fractional_nanoseconds);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = seconds.clone();
            self.unix_seconds = __terrane_value;
        }
        {
            let __terrane_value = fractional_nanoseconds.clone();
            self.nanoseconds = __terrane_value;
        }
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Deadline {
    pub expires_at: MonotonicInstant,
}
impl Deadline {
    pub fn terrane_construct(target: MonotonicInstant) -> Self {
        let mut __terrane_constructed_value = Self {
            expires_at: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(target);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, target: MonotonicInstant) {
        {
            let __terrane_value = target;
            self.expires_at = __terrane_value;
        }
    }
    pub fn remaining(&self) -> Option<Duration> {
        let now: MonotonicInstant;
        let elapsed: terrane_int_support::Int;
        now = Clock::terrane_static_monotonic();
        if self.expires_at.elapsed_nanoseconds.clone() <= now.elapsed_nanoseconds.clone()
        {
            return None;
        }
        elapsed = self.expires_at.elapsed_nanoseconds.clone()
            - now.elapsed_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed, 1000000000.clone()),
            ),
        );
    }
    pub fn expired(&self) -> bool {
        return self.remaining().is_none();
    }
    pub fn terrane_static_at(
        target: MonotonicInstant,
    ) -> Result<Deadline, TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    22 /* terrane-site: core/time.trn:105:13-105:45 */,
                )
            });
        }
        return Ok(Deadline::terrane_construct(target));
    }
}
pub fn discard_none(value: ()) {
    let _ = &value;
    return ();
}
#[derive(Clone)]
pub struct Tick {
    pub scheduled: MonotonicInstant,
    pub observed: MonotonicInstant,
    pub count: terrane_int_support::Int,
}
impl Tick {
    pub fn terrane_construct(
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            scheduled: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            observed: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            count: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(scheduled_at, observed_at, expirations);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = scheduled_at;
            self.scheduled = __terrane_value;
        }
        {
            let __terrane_value = observed_at;
            self.observed = __terrane_value;
        }
        {
            let __terrane_value = expirations.clone();
            self.count = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct Ticker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub anchor: MonotonicInstant,
    pub period: Duration,
    pub next_index: terrane_int_support::Int,
}
impl Ticker {
    pub fn terrane_construct(interval: Duration) -> Self {
        let mut __terrane_constructed_value = Self {
            anchor: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            period: Duration::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            next_index: terrane_int_support::Int::from(1_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(interval);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, interval: Duration) {
        {
            let __terrane_value = Clock::terrane_static_monotonic();
            self.anchor = __terrane_value;
        }
        {
            let __terrane_value = interval;
            self.period = __terrane_value;
        }
    }
    pub async fn next(&mut self) -> Tick {
        let period_total: terrane_int_support::Int;
        let scheduled_nanoseconds: terrane_int_support::Int;
        let observed: MonotonicInstant;
        let elapsed: terrane_int_support::Int;
        let mut observed_index: terrane_int_support::Int;
        let count: terrane_int_support::Int;
        let delivered: MonotonicInstant;
        period_total = self.period.total_nanoseconds.clone();
        scheduled_nanoseconds = self.anchor.elapsed_nanoseconds.clone()
            + period_total.clone() * self.next_index.clone();
        discard_none(
            __terrane_await(terrane_platform_time_sleep_until(scheduled_nanoseconds))
                .await,
        );
        observed = Clock::terrane_static_monotonic();
        elapsed = observed.elapsed_nanoseconds.clone()
            - self.anchor.elapsed_nanoseconds.clone();
        observed_index = terrane_platform_time_div(&elapsed, period_total.clone());
        if observed_index.clone() < self.next_index.clone() {
            observed_index = self.next_index.clone();
        }
        count = observed_index.clone() - self.next_index.clone()
            + terrane_int_support::Int::from(1_i128);
        delivered = MonotonicInstant::terrane_construct(
            self.anchor.domain.clone(),
            self.anchor.elapsed_nanoseconds.clone()
                + period_total.clone() * observed_index.clone(),
        );
        {
            let __terrane_value = observed_index.clone()
                + terrane_int_support::Int::from(1_i128);
            self.next_index = __terrane_value;
        }
        return Tick::terrane_construct(delivered, observed, count.clone());
    }
    pub async fn __terrane_shared_async_next(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> Tick {
        let period_total: terrane_int_support::Int;
        let scheduled_nanoseconds: terrane_int_support::Int;
        let observed: MonotonicInstant;
        let elapsed: terrane_int_support::Int;
        let mut observed_index: terrane_int_support::Int;
        let count: terrane_int_support::Int;
        let delivered: MonotonicInstant;
        period_total = {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .period
                .clone();
            __terrane_snapshot
        }
            .total_nanoseconds
            .clone();
        scheduled_nanoseconds = {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .anchor
                .clone();
            __terrane_snapshot
        }
            .elapsed_nanoseconds
            .clone()
            + period_total.clone()
                * {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .next_index
                        .clone();
                    __terrane_snapshot
                };
        discard_none(
            __terrane_await(terrane_platform_time_sleep_until(scheduled_nanoseconds))
                .await,
        );
        observed = Clock::terrane_static_monotonic();
        elapsed = observed.elapsed_nanoseconds.clone()
            - {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .anchor
                    .clone();
                __terrane_snapshot
            }
                .elapsed_nanoseconds
                .clone();
        observed_index = terrane_platform_time_div(&elapsed, period_total.clone());
        if observed_index.clone()
            < {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .next_index
                    .clone();
                __terrane_snapshot
            }
        {
            observed_index = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .next_index
                    .clone();
                __terrane_snapshot
            };
        }
        count = observed_index.clone()
            - {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .next_index
                    .clone();
                __terrane_snapshot
            } + terrane_int_support::Int::from(1_i128);
        delivered = MonotonicInstant::terrane_construct(
            {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .anchor
                    .clone();
                __terrane_snapshot
            }
                .domain
                .clone(),
            {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .anchor
                    .clone();
                __terrane_snapshot
            }
                .elapsed_nanoseconds
                .clone() + period_total.clone() * observed_index.clone(),
        );
        {
            let __terrane_value = observed_index.clone()
                + terrane_int_support::Int::from(1_i128);
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .next_index = __terrane_value;
        }
        return Tick::terrane_construct(delivered, observed, count.clone());
    }
    pub fn destruct(&mut self) {
        discard_none(());
    }
}
impl Drop for Ticker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct Clock {}
impl Clock {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_wall() -> Instant {
        let raw: TerranePlatformResult;
        raw = terrane_platform_time_wall();
        return Instant::terrane_construct(
            terrane_platform_time_wall_seconds(&raw),
            terrane_platform_time_wall_nanoseconds(&raw),
        );
    }
    pub fn terrane_static_monotonic() -> MonotonicInstant {
        return MonotonicInstant::terrane_construct(
            terrane_platform_time_domain(),
            terrane_platform_time_monotonic(),
        );
    }
    pub async fn terrane_static_sleep(elapsed: Duration) {
        let target: terrane_int_support::Int;
        target = terrane_platform_time_monotonic() + elapsed.total_nanoseconds.clone();
        return __terrane_await(terrane_platform_time_sleep_until(target)).await;
    }
    pub async fn terrane_static_sleep_until(
        target: MonotonicInstant,
    ) -> Result<(), TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    23 /* terrane-site: core/time.trn:161:13-161:45 */,
                )
            });
        }
        return Ok(
            __terrane_await(
                    terrane_platform_time_sleep_until(target.elapsed_nanoseconds),
                )
                .await,
        );
    }
    pub fn terrane_static_deadline(elapsed: Duration) -> Deadline {
        let now: MonotonicInstant;
        let target: MonotonicInstant;
        now = Clock::terrane_static_monotonic();
        target = MonotonicInstant::terrane_construct(
            now.domain.clone(),
            now.elapsed_nanoseconds.clone() + elapsed.total_nanoseconds.clone(),
        );
        return Deadline::terrane_construct(target);
    }
    pub fn terrane_static_interval(period: Duration) -> Result<Ticker, TerraneError> {
        if period.total_nanoseconds.clone() == terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    24 /* terrane-site: core/time.trn:172:13-172:45 */,
                )
            });
        }
        return Ok(Ticker::terrane_construct(period));
    }
}
