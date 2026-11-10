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
#[allow(dead_code, reason = "a projected dependency may expose no Result members")]
const TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId(0);
#[allow(dead_code, reason = "panic catching may be disabled or not crossed")]
const TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId(1);
#[allow(
    dead_code,
    reason = "projected type methods may be imported without being crossed"
)]
fn __terrane_dependency_panic(
    payload: std::boxed::Box<dyn std::any::Any + Send>,
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
    pub static DESCRIPTORS: [&str; 2] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
    ];
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
    pub static FUNCTIONS: [&str; 6] = [
        "/app::mutate-default-alias",
        "/app::main",
        "/app::consume_borrowed",
        "/app::consume_borrowed_string",
        "/app::consume_aliased_ref",
        "/app::consume_holder_field",
    ];
    pub static SITES: [Site; 27] = [
        /* terrane-site-row: site 0: /app::mutate-default-alias (src/main.trn:17:5-17:33) */
        { Site { function: 0, file: 0, line: 17, column: 5, end_line: 17, end_column: 33 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:21:23-21:36) */
        { Site { function: 1, file: 0, line: 21, column: 23, end_line: 21, end_column: 36 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:25:13-25:34) */
        { Site { function: 1, file: 0, line: 25, column: 13, end_line: 25, end_column: 34 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:26:13-26:38) */
        { Site { function: 1, file: 0, line: 26, column: 13, end_line: 26, end_column: 38 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:27:13-27:34) */
        { Site { function: 1, file: 0, line: 27, column: 13, end_line: 27, end_column: 34 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:29:39-29:50) */
        { Site { function: 1, file: 0, line: 29, column: 39, end_line: 29, end_column: 50 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:31:13-31:35) */
        { Site { function: 1, file: 0, line: 31, column: 13, end_line: 31, end_column: 35 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:34:32-34:44) */
        { Site { function: 1, file: 0, line: 34, column: 32, end_line: 34, end_column: 44 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:35:13-35:33) */
        { Site { function: 1, file: 0, line: 35, column: 13, end_line: 35, end_column: 33 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:36:13-36:32) */
        { Site { function: 1, file: 0, line: 36, column: 13, end_line: 36, end_column: 32 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:37:32-37:52) */
        { Site { function: 1, file: 0, line: 37, column: 32, end_line: 37, end_column: 52 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:38:13-38:37) */
        { Site { function: 1, file: 0, line: 38, column: 13, end_line: 38, end_column: 37 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:43:36-43:68) */
        { Site { function: 1, file: 0, line: 43, column: 36, end_line: 43, end_column: 68 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:44:32-44:64) */
        { Site { function: 1, file: 0, line: 44, column: 32, end_line: 44, end_column: 64 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:45:13-45:39) */
        { Site { function: 1, file: 0, line: 45, column: 13, end_line: 45, end_column: 39 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:46:13-46:37) */
        { Site { function: 1, file: 0, line: 46, column: 13, end_line: 46, end_column: 37 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:47:37-47:48) */
        { Site { function: 1, file: 0, line: 47, column: 37, end_line: 47, end_column: 48 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:49:39-49:51) */
        { Site { function: 1, file: 0, line: 49, column: 39, end_line: 49, end_column: 51 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:52:38-52:50) */
        { Site { function: 1, file: 0, line: 52, column: 38, end_line: 52, end_column: 50 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:54:15-54:26) */
        { Site { function: 1, file: 0, line: 54, column: 15, end_line: 54, end_column: 26 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:56:13-56:32) */
        { Site { function: 1, file: 0, line: 56, column: 13, end_line: 56, end_column: 32 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:58:25-58:36) */
        { Site { function: 1, file: 0, line: 58, column: 25, end_line: 58, end_column: 36 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:61:13-61:42) */
        { Site { function: 1, file: 0, line: 61, column: 13, end_line: 61, end_column: 42 } },
        /* terrane-site-row: site 23: /app::consume_borrowed (src/main.trn:64:12-64:29) */
        { Site { function: 2, file: 0, line: 64, column: 12, end_line: 64, end_column: 29 } },
        /* terrane-site-row: site 24: /app::consume_borrowed_string (src/main.trn:66:12-66:29) */
        { Site { function: 3, file: 0, line: 66, column: 12, end_line: 66, end_column: 29 } },
        /* terrane-site-row: site 25: /app::consume_aliased_ref (src/main.trn:69:12-69:29) */
        { Site { function: 4, file: 0, line: 69, column: 12, end_line: 69, end_column: 29 } },
        /* terrane-site-row: site 26: /app::consume_holder_field (src/main.trn:75:12-75:35) */
        { Site { function: 5, file: 0, line: 75, column: 12, end_line: 75, end_column: 35 } },
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
// Source: <terrane>/projected/deps/owner.trn
// Namespace: deps/owner
pub fn terrane_static_trn_5075626c696356616c7565_new(
    value: terrane_int_support::Int,
) -> Result<PublicValue, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<u8>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| owner::PublicValue::new(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(payload, "owner", "owner::PublicValue"),
            )
        }
    }
}
pub fn make_owner() -> Result<owner::Holder<u8>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| owner::make_owner()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "owner", "owner::make_owner"))
        }
    }
}
pub fn owner_byte(
    value: owner::Holder<u8>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| owner::owner_byte(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "owner", "owner::owner_byte"))
        }
    }
}
// Source: <terrane>/projected/deps/api.trn
// Namespace: deps/api
pub use api::Alias;
pub use owner::Holder;
pub use api::OtherHolder;
pub use api::PublicHolder;
pub use owner::PublicValue;
pub use api::StringAlias;
pub use api::MutableAlias as TerraneNsDepsApiMutableAliasIdentity2F646570732F617069004D757461626C65416C696173;
pub fn make_default() -> Result<owner::Holder<u8>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| api::make_default()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "api", "api::make_default"))
        }
    }
}
pub fn make_string() -> Result<owner::Holder<String>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| api::make_string())) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "api", "api::make_string"))
        }
    }
}
pub fn make_string_default() -> Result<
    owner::Holder<String>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| api::make_string_default()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "api",
                    "api::make_string_default",
                ),
            )
        }
    }
}
pub fn take_byte(
    value: owner::Holder<u8>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| api::take_byte(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "api", "api::take_byte"))
        }
    }
}
pub fn take_text(
    value: owner::Holder<String>,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| api::take_text(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "api", "api::take_text"))
        }
    }
}
