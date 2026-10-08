// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, tasks_native_parallel.rs, time_inactive.rs, platform_streams.rs, platform_standard_streams.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi
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
    pub static FILES: [&str; 1] = ["core/streams.trn"];
    pub static FUNCTIONS: [&str; 4] = [
        "/core/streams::read",
        "/core/streams::read-exact",
        "/core/streams::read-all",
        "/core/streams::read-async",
    ];
    pub static SITES: [Site; 4] = [
        /* terrane-site-row: site 0: /core/streams::read (core/streams.trn:188:23-188:50) */
        { Site { function: 0, file: 0, line: 188, column: 23, end_line: 188, end_column: 50 } },
        /* terrane-site-row: site 1: /core/streams::read-exact (core/streams.trn:210:23-210:46) */
        { Site { function: 1, file: 0, line: 210, column: 23, end_line: 210, end_column: 46 } },
        /* terrane-site-row: site 2: /core/streams::read-all (core/streams.trn:229:23-229:46) */
        { Site { function: 2, file: 0, line: 229, column: 23, end_line: 229, end_column: 46 } },
        /* terrane-site-row: site 3: /core/streams::read-async (core/streams.trn:234:23-234:50) */
        { Site { function: 3, file: 0, line: 234, column: 23, end_line: 234, end_column: 50 } },
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
// Namespace: cancelled-stream-operation
async fn read_one() -> ReadResult {
    let input_terrane_f0_s124: ByteReader;
    let pending_terrane_f0_s143;
    let result_terrane_f0_s177: ReadResult;
    input_terrane_f0_s124 = stdin();
    pending_terrane_f0_s143 = (&input_terrane_f0_s124)
        .read_async(terrane_int_support::Int::from(2_i128));
    result_terrane_f0_s177 = __terrane_await(pending_terrane_f0_s143).await;
    input_terrane_f0_s124.close();
    return result_terrane_f0_s177;
}
fn main() {
    __terrane_run(async move {
        let scope_terrane_f0_s261: TerraneTaskScope;
        let child_terrane_f0_s285: TerraneScopedTask<ReadResult>;
        let outcome_terrane_f0_s337: TerraneTaskOutcome<ReadResult>;
        scope_terrane_f0_s261 = TerraneTaskScope::new(None);
        child_terrane_f0_s285 = {
            let __terrane_scope = scope_terrane_f0_s261.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(read_one()) })(),
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
        scope_terrane_f0_s261.cancel();
        outcome_terrane_f0_s337 = __terrane_await(
                scope_terrane_f0_s261.join(child_terrane_f0_s285),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&outcome_terrane_f0_s337
            .cancelled), terrane_scalar_support::scalar_text(&outcome_terrane_f0_s337
            .value.clone().is_none())
        );
    });
}
// Source: core/streams.trn
// Namespace: core/streams
#[derive(Clone)]
pub struct StreamOperationResult {
    pub failed: bool,
    pub message: String,
}
impl StreamOperationResult {
    pub fn terrane_construct(failed: bool, message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(failed, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String) {
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct ReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl ReadResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(data, completed, end, failed, message);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        self.data = data;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
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
        self.text = text;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct WriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
}
impl WriteResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(data, completed, failed, message);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) {
        if completed.clone() == terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            self.data = Vec::from([]);
        } else {
            self.data = data;
        }
        self.completed = completed.clone();
        self.failed = failed;
        self.message = message;
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
        self.handle = Some(handle);
    }
    pub fn read(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw_terrane_f1_s1627: TerranePlatformReadResult;
        raw_terrane_f1_s1627 = terrane_platform_read(
            &self.handle.as_ref().expect("required field initialized"),
            count,
        );
        return ReadResult::terrane_construct(
            raw_terrane_f1_s1627.data.clone().clone(),
            raw_terrane_f1_s1627.completed.clone(),
            raw_terrane_f1_s1627.end,
            raw_terrane_f1_s1627.failed,
            raw_terrane_f1_s1627.message.clone().clone(),
        );
    }
    pub fn read_exact(&self, count: terrane_int_support::Int) -> ReadResult {
        let mut data_terrane_f1_s1814: Vec<u8>;
        let mut completed_terrane_f1_s1839: terrane_int_support::Int;
        let mut end_terrane_f1_s1865: bool;
        let mut failed_terrane_f1_s1890: bool;
        let mut message_terrane_f1_s1918: String;
        let mut part_terrane_f1_s2009: TerranePlatformReadResult;
        data_terrane_f1_s1814 = Vec::from([]);
        completed_terrane_f1_s1839 = terrane_int_support::Int::from(0_i128);
        end_terrane_f1_s1865 = false;
        failed_terrane_f1_s1890 = false;
        message_terrane_f1_s1918 = String::from("");
        while completed_terrane_f1_s1839.clone() < count.clone() && !end_terrane_f1_s1865
            && !failed_terrane_f1_s1890
        {
            part_terrane_f1_s2009 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                count.clone() - completed_terrane_f1_s1839.clone(),
            );
            data_terrane_f1_s1814 = {
                let mut bytes = data_terrane_f1_s1814;
                let part_0: Vec<u8> = part_terrane_f1_s2009.data.clone();
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
            completed_terrane_f1_s1839 = completed_terrane_f1_s1839.clone()
                + part_terrane_f1_s2009.completed.clone();
            end_terrane_f1_s1865 = part_terrane_f1_s2009.end;
            failed_terrane_f1_s1890 = part_terrane_f1_s2009.failed;
            message_terrane_f1_s1918 = part_terrane_f1_s2009.message.clone().clone();
            if part_terrane_f1_s2009.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f1_s2009.end
                && !part_terrane_f1_s2009.failed
            {
                failed_terrane_f1_s1890 = true;
                message_terrane_f1_s1918 = String::from("stream read made no progress");
            }
        }
        if end_terrane_f1_s1865 && completed_terrane_f1_s1839.clone() < count.clone()
            && !failed_terrane_f1_s1890
        {
            failed_terrane_f1_s1890 = true;
            message_terrane_f1_s1918 = String::from(
                "stream ended before exact byte count",
            );
        }
        return ReadResult::terrane_construct(
            data_terrane_f1_s1814,
            completed_terrane_f1_s1839.clone(),
            end_terrane_f1_s1865,
            failed_terrane_f1_s1890,
            message_terrane_f1_s1918,
        );
    }
    pub fn read_all(&self, limit: terrane_int_support::Int) -> ReadResult {
        let mut data_terrane_f1_s2673: Vec<u8>;
        let mut completed_terrane_f1_s2698: terrane_int_support::Int;
        let mut end_terrane_f1_s2724: bool;
        let mut failed_terrane_f1_s2749: bool;
        let mut message_terrane_f1_s2777: String;
        let mut part_terrane_f1_s2868: TerranePlatformReadResult;
        data_terrane_f1_s2673 = Vec::from([]);
        completed_terrane_f1_s2698 = terrane_int_support::Int::from(0_i128);
        end_terrane_f1_s2724 = false;
        failed_terrane_f1_s2749 = false;
        message_terrane_f1_s2777 = String::from("");
        while completed_terrane_f1_s2698.clone() < limit.clone() && !end_terrane_f1_s2724
            && !failed_terrane_f1_s2749
        {
            part_terrane_f1_s2868 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                limit.clone() - completed_terrane_f1_s2698.clone(),
            );
            data_terrane_f1_s2673 = {
                let mut bytes = data_terrane_f1_s2673;
                let part_0: Vec<u8> = part_terrane_f1_s2868.data.clone();
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
            completed_terrane_f1_s2698 = completed_terrane_f1_s2698.clone()
                + part_terrane_f1_s2868.completed.clone();
            end_terrane_f1_s2724 = part_terrane_f1_s2868.end;
            failed_terrane_f1_s2749 = part_terrane_f1_s2868.failed;
            message_terrane_f1_s2777 = part_terrane_f1_s2868.message.clone().clone();
            if part_terrane_f1_s2868.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f1_s2868.end
                && !part_terrane_f1_s2868.failed
            {
                failed_terrane_f1_s2749 = true;
                message_terrane_f1_s2777 = String::from("stream read made no progress");
            }
        }
        return ReadResult::terrane_construct(
            data_terrane_f1_s2673,
            completed_terrane_f1_s2698.clone(),
            end_terrane_f1_s2724,
            failed_terrane_f1_s2749,
            message_terrane_f1_s2777,
        );
    }
    pub async fn read_async(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw_terrane_f1_s3401: TerranePlatformReadResult;
        raw_terrane_f1_s3401 = __terrane_await(
                terrane_platform_read_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    count,
                ),
            )
            .await;
        return ReadResult::terrane_construct(
            raw_terrane_f1_s3401.data.clone().clone(),
            raw_terrane_f1_s3401.completed.clone(),
            raw_terrane_f1_s3401.end,
            raw_terrane_f1_s3401.failed,
            raw_terrane_f1_s3401.message.clone().clone(),
        );
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextReader {
        return TextReader::terrane_construct(
            self.handle.as_ref().expect("required field initialized").clone(),
            codec,
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f1_s3710: TerranePlatformUnitResult;
        raw_terrane_f1_s3710 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s3710.failed,
            raw_terrane_f1_s3710.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
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
        self.handle = Some(handle);
    }
    pub fn write(&self, data: Vec<u8>) -> WriteResult {
        let offset_terrane_f1_s4175: i64;
        let raw_terrane_f1_s4198: TerranePlatformWriteResult;
        offset_terrane_f1_s4175 = 0;
        raw_terrane_f1_s4198 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &data,
            terrane_int_support::Int::from(offset_terrane_f1_s4175.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw_terrane_f1_s4198.completed.clone(),
            raw_terrane_f1_s4198.failed,
            raw_terrane_f1_s4198.message.clone().clone(),
        );
    }
    pub fn write_all(&self, data: Vec<u8>) -> WriteResult {
        let mut completed_terrane_f1_s4382: terrane_int_support::Int;
        let mut failed_terrane_f1_s4408: bool;
        let mut message_terrane_f1_s4436: String;
        let mut part_terrane_f1_s4521: TerranePlatformWriteResult;
        completed_terrane_f1_s4382 = terrane_int_support::Int::from(0_i128);
        failed_terrane_f1_s4408 = false;
        message_terrane_f1_s4436 = String::from("");
        while completed_terrane_f1_s4382.clone()
            < terrane_int_support::Int::from(data.len() as i128)
            && !failed_terrane_f1_s4408
        {
            part_terrane_f1_s4521 = terrane_platform_write(
                &self.handle.as_ref().expect("required field initialized"),
                &data,
                terrane_int_support::Int::from(completed_terrane_f1_s4382.clone()),
            );
            completed_terrane_f1_s4382 = completed_terrane_f1_s4382.clone()
                + part_terrane_f1_s4521.completed.clone();
            failed_terrane_f1_s4408 = part_terrane_f1_s4521.failed;
            message_terrane_f1_s4436 = part_terrane_f1_s4521.message.clone().clone();
            if part_terrane_f1_s4521.completed.clone()
                == terrane_int_support::Int::from(0_i128)
                && !part_terrane_f1_s4521.failed
            {
                failed_terrane_f1_s4408 = true;
                message_terrane_f1_s4436 = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(
            data,
            completed_terrane_f1_s4382.clone(),
            failed_terrane_f1_s4408,
            message_terrane_f1_s4436,
        );
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw_terrane_f1_s5023: TerranePlatformWriteResult;
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        raw_terrane_f1_s5023 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw_terrane_f1_s5023.completed.clone(),
            raw_terrane_f1_s5023.failed,
            raw_terrane_f1_s5023.message.clone().clone(),
        );
    }
    pub async fn write_async(&self, data: Vec<u8>) -> WriteResult {
        return self.write(data);
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextWriter {
        return TextWriter::terrane_construct(
            self.handle.as_ref().expect("required field initialized").clone(),
            codec,
        );
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw_terrane_f1_s5434: TerranePlatformUnitResult;
        raw_terrane_f1_s5434 = terrane_platform_flush(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s5434.failed,
            raw_terrane_f1_s5434.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw_terrane_f1_s5594: TerranePlatformUnitResult;
        raw_terrane_f1_s5594 = terrane_platform_sync_data(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s5594.failed,
            raw_terrane_f1_s5594.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw_terrane_f1_s5757: TerranePlatformUnitResult;
        raw_terrane_f1_s5757 = terrane_platform_sync_all(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s5757.failed,
            raw_terrane_f1_s5757.message.clone().clone(),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f1_s5926: TerranePlatformUnitResult;
        raw_terrane_f1_s5926 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s5926.failed,
            raw_terrane_f1_s5926.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
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
        self.handle = Some(handle);
        self.codec = codec;
    }
    pub fn read(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw_terrane_f1_s6482: TerranePlatformReadResult;
        let text_terrane_f1_s6526: String;
        raw_terrane_f1_s6482 = terrane_platform_read(
            &self.handle.as_ref().expect("required field initialized"),
            count,
        );
        text_terrane_f1_s6526 = __terrane_raised_err(
            terrane_string_support::decode(
                &raw_terrane_f1_s6482.data.clone(),
                self.codec,
            ),
            0 /* terrane-site: core/streams.trn:188:23-188:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f1_s6526,
                raw_terrane_f1_s6482.completed.clone(),
                raw_terrane_f1_s6482.end,
                raw_terrane_f1_s6482.failed,
                raw_terrane_f1_s6482.message.clone().clone(),
            ),
        );
    }
    pub fn read_exact(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data_terrane_f1_s6745: Vec<u8>;
        let mut completed_terrane_f1_s6770: terrane_int_support::Int;
        let mut end_terrane_f1_s6796: bool;
        let mut failed_terrane_f1_s6821: bool;
        let mut message_terrane_f1_s6849: String;
        let mut part_terrane_f1_s6940: TerranePlatformReadResult;
        let text_terrane_f1_s7483: String;
        data_terrane_f1_s6745 = Vec::from([]);
        completed_terrane_f1_s6770 = terrane_int_support::Int::from(0_i128);
        end_terrane_f1_s6796 = false;
        failed_terrane_f1_s6821 = false;
        message_terrane_f1_s6849 = String::from("");
        while completed_terrane_f1_s6770.clone() < count.clone() && !end_terrane_f1_s6796
            && !failed_terrane_f1_s6821
        {
            part_terrane_f1_s6940 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                count.clone() - completed_terrane_f1_s6770.clone(),
            );
            data_terrane_f1_s6745 = {
                let mut bytes = data_terrane_f1_s6745;
                let part_0: Vec<u8> = part_terrane_f1_s6940.data.clone();
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
            completed_terrane_f1_s6770 = completed_terrane_f1_s6770.clone()
                + part_terrane_f1_s6940.completed.clone();
            end_terrane_f1_s6796 = part_terrane_f1_s6940.end;
            failed_terrane_f1_s6821 = part_terrane_f1_s6940.failed;
            message_terrane_f1_s6849 = part_terrane_f1_s6940.message.clone().clone();
            if part_terrane_f1_s6940.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f1_s6940.end
                && !part_terrane_f1_s6940.failed
            {
                failed_terrane_f1_s6821 = true;
                message_terrane_f1_s6849 = String::from("stream read made no progress");
            }
        }
        if end_terrane_f1_s6796 && completed_terrane_f1_s6770.clone() < count.clone()
            && !failed_terrane_f1_s6821
        {
            failed_terrane_f1_s6821 = true;
            message_terrane_f1_s6849 = String::from(
                "stream ended before exact byte count",
            );
        }
        text_terrane_f1_s7483 = __terrane_raised_err(
            terrane_string_support::decode(&data_terrane_f1_s6745, self.codec),
            1 /* terrane-site: core/streams.trn:210:23-210:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f1_s7483,
                completed_terrane_f1_s6770.clone(),
                end_terrane_f1_s6796,
                failed_terrane_f1_s6821,
                message_terrane_f1_s6849,
            ),
        );
    }
    pub fn read_all(
        &self,
        limit: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data_terrane_f1_s7680: Vec<u8>;
        let mut completed_terrane_f1_s7705: terrane_int_support::Int;
        let mut end_terrane_f1_s7731: bool;
        let mut failed_terrane_f1_s7756: bool;
        let mut message_terrane_f1_s7784: String;
        let mut part_terrane_f1_s7875: TerranePlatformReadResult;
        let text_terrane_f1_s8279: String;
        data_terrane_f1_s7680 = Vec::from([]);
        completed_terrane_f1_s7705 = terrane_int_support::Int::from(0_i128);
        end_terrane_f1_s7731 = false;
        failed_terrane_f1_s7756 = false;
        message_terrane_f1_s7784 = String::from("");
        while completed_terrane_f1_s7705.clone() < limit.clone() && !end_terrane_f1_s7731
            && !failed_terrane_f1_s7756
        {
            part_terrane_f1_s7875 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                limit.clone() - completed_terrane_f1_s7705.clone(),
            );
            data_terrane_f1_s7680 = {
                let mut bytes = data_terrane_f1_s7680;
                let part_0: Vec<u8> = part_terrane_f1_s7875.data.clone();
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
            completed_terrane_f1_s7705 = completed_terrane_f1_s7705.clone()
                + part_terrane_f1_s7875.completed.clone();
            end_terrane_f1_s7731 = part_terrane_f1_s7875.end;
            failed_terrane_f1_s7756 = part_terrane_f1_s7875.failed;
            message_terrane_f1_s7784 = part_terrane_f1_s7875.message.clone().clone();
            if part_terrane_f1_s7875.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f1_s7875.end
                && !part_terrane_f1_s7875.failed
            {
                failed_terrane_f1_s7756 = true;
                message_terrane_f1_s7784 = String::from("stream read made no progress");
            }
        }
        text_terrane_f1_s8279 = __terrane_raised_err(
            terrane_string_support::decode(&data_terrane_f1_s7680, self.codec),
            2 /* terrane-site: core/streams.trn:229:23-229:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f1_s8279,
                completed_terrane_f1_s7705.clone(),
                end_terrane_f1_s7731,
                failed_terrane_f1_s7756,
                message_terrane_f1_s7784,
            ),
        );
    }
    pub async fn read_async(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw_terrane_f1_s8484: TerranePlatformReadResult;
        let text_terrane_f1_s8540: String;
        raw_terrane_f1_s8484 = __terrane_await(
                terrane_platform_read_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    count,
                ),
            )
            .await;
        text_terrane_f1_s8540 = __terrane_raised_err(
            terrane_string_support::decode(
                &raw_terrane_f1_s8484.data.clone(),
                self.codec,
            ),
            3 /* terrane-site: core/streams.trn:234:23-234:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f1_s8540,
                raw_terrane_f1_s8484.completed.clone(),
                raw_terrane_f1_s8484.end,
                raw_terrane_f1_s8484.failed,
                raw_terrane_f1_s8484.message.clone().clone(),
            ),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f1_s8741: TerranePlatformUnitResult;
        raw_terrane_f1_s8741 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s8741.failed,
            raw_terrane_f1_s8741.message.clone().clone(),
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
        self.handle = Some(handle);
        self.codec = codec;
    }
    pub fn write(&self, text: String) -> WriteResult {
        let data_terrane_f1_s9276: Vec<u8>;
        let offset_terrane_f1_s9321: i64;
        let raw_terrane_f1_s9344: TerranePlatformWriteResult;
        data_terrane_f1_s9276 = terrane_string_support::encode(&text, self.codec);
        offset_terrane_f1_s9321 = 0;
        raw_terrane_f1_s9344 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &data_terrane_f1_s9276,
            terrane_int_support::Int::from(offset_terrane_f1_s9321.clone()),
        );
        return WriteResult::terrane_construct(
            data_terrane_f1_s9276,
            raw_terrane_f1_s9344.completed.clone(),
            raw_terrane_f1_s9344.failed,
            raw_terrane_f1_s9344.message.clone().clone(),
        );
    }
    pub fn write_all(&self, text: String) -> WriteResult {
        let data_terrane_f1_s9528: Vec<u8>;
        let mut completed_terrane_f1_s9573: terrane_int_support::Int;
        let mut failed_terrane_f1_s9599: bool;
        let mut message_terrane_f1_s9627: String;
        let mut part_terrane_f1_s9712: TerranePlatformWriteResult;
        data_terrane_f1_s9528 = terrane_string_support::encode(&text, self.codec);
        completed_terrane_f1_s9573 = terrane_int_support::Int::from(0_i128);
        failed_terrane_f1_s9599 = false;
        message_terrane_f1_s9627 = String::from("");
        while completed_terrane_f1_s9573.clone()
            < terrane_int_support::Int::from(data_terrane_f1_s9528.len() as i128)
            && !failed_terrane_f1_s9599
        {
            part_terrane_f1_s9712 = terrane_platform_write(
                &self.handle.as_ref().expect("required field initialized"),
                &data_terrane_f1_s9528,
                terrane_int_support::Int::from(completed_terrane_f1_s9573.clone()),
            );
            completed_terrane_f1_s9573 = completed_terrane_f1_s9573.clone()
                + part_terrane_f1_s9712.completed.clone();
            failed_terrane_f1_s9599 = part_terrane_f1_s9712.failed;
            message_terrane_f1_s9627 = part_terrane_f1_s9712.message.clone().clone();
            if part_terrane_f1_s9712.completed.clone()
                == terrane_int_support::Int::from(0_i128)
                && !part_terrane_f1_s9712.failed
            {
                failed_terrane_f1_s9599 = true;
                message_terrane_f1_s9627 = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(
            data_terrane_f1_s9528,
            completed_terrane_f1_s9573.clone(),
            failed_terrane_f1_s9599,
            message_terrane_f1_s9627,
        );
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw_terrane_f1_s10214: TerranePlatformWriteResult;
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        raw_terrane_f1_s10214 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw_terrane_f1_s10214.completed.clone(),
            raw_terrane_f1_s10214.failed,
            raw_terrane_f1_s10214.message.clone().clone(),
        );
    }
    pub fn line(&self, text: String) -> WriteResult {
        return self
            .write_all(
                format!(
                    "{}{}", terrane_scalar_support::scalar_text(&text),
                    terrane_scalar_support::scalar_text(&String::from("\n"))
                ),
            );
    }
    pub async fn write_async(&self, text: String) -> WriteResult {
        return self.write(text);
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw_terrane_f1_s10619: TerranePlatformUnitResult;
        raw_terrane_f1_s10619 = terrane_platform_flush(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s10619.failed,
            raw_terrane_f1_s10619.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw_terrane_f1_s10779: TerranePlatformUnitResult;
        raw_terrane_f1_s10779 = terrane_platform_sync_data(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s10779.failed,
            raw_terrane_f1_s10779.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw_terrane_f1_s10942: TerranePlatformUnitResult;
        raw_terrane_f1_s10942 = terrane_platform_sync_all(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s10942.failed,
            raw_terrane_f1_s10942.message.clone().clone(),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f1_s11111: TerranePlatformUnitResult;
        raw_terrane_f1_s11111 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f1_s11111.failed,
            raw_terrane_f1_s11111.message.clone().clone(),
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
