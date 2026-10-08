// Generated deterministically by Terrane <version>.
// Runtime support: platform_streams.rs, platform_files.rs, platform_system.rs
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
    pub static FILES: [&str; 1] = ["core/paths.trn"];
    pub static FUNCTIONS: [&str; 6] = [
        "/core/filesystem/paths::path-components",
        "/core/filesystem/paths::normalise-path",
        "/core/filesystem/paths::path-name",
        "/core/filesystem/paths::path-parent",
        "/core/filesystem/paths::path-stem",
        "/core/filesystem/paths::path-extension",
    ];
    pub static SITES: [Site; 12] = [
        /* terrane-site-row: site 0: /core/filesystem/paths::path-components (core/paths.trn:16:16-16:28) */
        { Site { function: 0, file: 0, line: 16, column: 16, end_line: 16, end_column: 28 } },
        /* terrane-site-row: site 1: /core/filesystem/paths::normalise-path (core/paths.trn:32:16-32:33) */
        { Site { function: 1, file: 0, line: 32, column: 16, end_line: 32, end_column: 33 } },
        /* terrane-site-row: site 2: /core/filesystem/paths::normalise-path (core/paths.trn:35:34-35:49) */
        { Site { function: 1, file: 0, line: 35, column: 34, end_line: 35, end_column: 49 } },
        /* terrane-site-row: site 3: /core/filesystem/paths::normalise-path (core/paths.trn:40:29-40:50) */
        { Site { function: 1, file: 0, line: 40, column: 29, end_line: 40, end_column: 50 } },
        /* terrane-site-row: site 4: /core/filesystem/paths::normalise-path (core/paths.trn:46:21-46:42) */
        { Site { function: 1, file: 0, line: 46, column: 21, end_line: 46, end_column: 42 } },
        /* terrane-site-row: site 5: /core/filesystem/paths::normalise-path (core/paths.trn:56:33-56:44) */
        { Site { function: 1, file: 0, line: 56, column: 33, end_line: 56, end_column: 44 } },
        /* terrane-site-row: site 6: /core/filesystem/paths::path-name (core/paths.trn:69:12-69:35) */
        { Site { function: 2, file: 0, line: 69, column: 12, end_line: 69, end_column: 35 } },
        /* terrane-site-row: site 7: /core/filesystem/paths::path-parent (core/paths.trn:83:33-83:45) */
        { Site { function: 3, file: 0, line: 83, column: 33, end_line: 83, end_column: 45 } },
        /* terrane-site-row: site 8: /core/filesystem/paths::path-stem (core/paths.trn:95:31-95:40) */
        { Site { function: 4, file: 0, line: 95, column: 31, end_line: 95, end_column: 40 } },
        /* terrane-site-row: site 9: /core/filesystem/paths::path-stem (core/paths.trn:102:33-102:46) */
        { Site { function: 4, file: 0, line: 102, column: 33, end_line: 102, end_column: 46 } },
        /* terrane-site-row: site 10: /core/filesystem/paths::path-extension (core/paths.trn:111:31-111:40) */
        { Site { function: 5, file: 0, line: 111, column: 31, end_line: 111, end_column: 40 } },
        /* terrane-site-row: site 11: /core/filesystem/paths::path-extension (core/paths.trn:113:12-113:37) */
        { Site { function: 5, file: 0, line: 113, column: 12, end_line: 113, end_column: 37 } },
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
// Namespace: conformance/filesystem-durability-exports
fn exercise(capability: Filesystem, output: &FileHandle) -> bool {
    let data_terrane_f0_s286: FilesystemOperationResult;
    let all_terrane_f0_s332: FilesystemOperationResult;
    data_terrane_f0_s286 = file_sync_data(capability.clone(), output);
    all_terrane_f0_s332 = file_sync_all(capability, output);
    return data_terrane_f0_s286.failed || all_terrane_f0_s332.failed;
}
fn main() {
    let capability_terrane_f0_s429: Filesystem;
    let output_terrane_f0_s469: FileHandle;
    capability_terrane_f0_s429 = filesystem_capability();
    output_terrane_f0_s469 = open_file(
        capability_terrane_f0_s429.clone(),
        Path::terrane_construct(String::from("durability-check.tmp")),
        false,
        true,
        true,
        true,
    );
    exercise(capability_terrane_f0_s429, &output_terrane_f0_s469);
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
        self.failed = failure;
        self.message = detail;
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
        self.exists = exists;
        self.failed = failure;
        self.message = detail;
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
        self.resolved = target;
        self.failed = failure;
        self.message = detail;
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
        self.kind = kind;
        self.size = size.clone();
        self.readonly = readonly;
        self.permission_detail = permission_detail;
        self.failed = failure;
        self.message = detail;
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
            data: Vec::from([]),
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
        self.data = data;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failure;
        self.message = detail;
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
        self.handle = Some(raw);
        self.failed = failure;
        self.message = detail;
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
        self.handle = Some(raw);
        self.failed = failure;
        self.message = detail;
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
    let raw_terrane_f1_s2468: TerranePlatformOpenResult;
    let failure_terrane_f1_s2544: bool;
    let detail_terrane_f1_s2569: String;
    let acquired_terrane_f1_s2594: TerranePlatformStreamHandle;
    let _ = &capability;
    raw_terrane_f1_s2468 = terrane_platform_open_file(
        target.text,
        readable,
        writable,
        create,
        truncate,
    );
    failure_terrane_f1_s2544 = raw_terrane_f1_s2468.failed;
    detail_terrane_f1_s2569 = raw_terrane_f1_s2468.message.clone().clone();
    acquired_terrane_f1_s2594 = raw_terrane_f1_s2468.handle.clone().clone();
    return FileHandle::terrane_construct(
        acquired_terrane_f1_s2594,
        failure_terrane_f1_s2544,
        detail_terrane_f1_s2569,
    );
}
pub fn file_read(
    capability: Filesystem,
    file: &FileHandle,
    limit: terrane_int_support::Int,
) -> FileData {
    let raw_terrane_f1_s2786: TerranePlatformReadResult;
    let _ = &capability;
    raw_terrane_f1_s2786 = terrane_platform_read(
        &file.handle.as_ref().expect("required field initialized"),
        limit,
    );
    return FileData::terrane_construct(
        raw_terrane_f1_s2786.data.clone().clone(),
        raw_terrane_f1_s2786.completed.clone(),
        raw_terrane_f1_s2786.end,
        raw_terrane_f1_s2786.failed,
        raw_terrane_f1_s2786.message.clone().clone(),
    );
}
pub fn file_write(
    capability: Filesystem,
    file: &FileHandle,
    data: Vec<u8>,
    offset: terrane_int_support::Int,
) -> FileData {
    let raw_terrane_f1_s3019: TerranePlatformWriteResult;
    let _ = &capability;
    raw_terrane_f1_s3019 = terrane_platform_write(
        &file.handle.as_ref().expect("required field initialized"),
        &data,
        terrane_int_support::Int::from(offset.clone()),
    );
    return FileData::terrane_construct(
        data,
        raw_terrane_f1_s3019.completed.clone(),
        false,
        raw_terrane_f1_s3019.failed,
        raw_terrane_f1_s3019.message.clone().clone(),
    );
}
pub fn file_flush(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw_terrane_f1_s3244: TerranePlatformUnitResult;
    let _ = &capability;
    raw_terrane_f1_s3244 = terrane_platform_flush(
        &file.handle.as_ref().expect("required field initialized"),
    );
    return FilesystemOperationResult::terrane_construct(
        raw_terrane_f1_s3244.failed,
        raw_terrane_f1_s3244.message.clone().clone(),
    );
}
pub fn file_sync_data(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw_terrane_f1_s3449: TerranePlatformUnitResult;
    let _ = &capability;
    raw_terrane_f1_s3449 = terrane_platform_sync_data(
        &file.handle.as_ref().expect("required field initialized"),
    );
    return FilesystemOperationResult::terrane_construct(
        raw_terrane_f1_s3449.failed,
        raw_terrane_f1_s3449.message.clone().clone(),
    );
}
pub fn file_sync_all(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw_terrane_f1_s3657: TerranePlatformUnitResult;
    let _ = &capability;
    raw_terrane_f1_s3657 = terrane_platform_sync_all(
        &file.handle.as_ref().expect("required field initialized"),
    );
    return FilesystemOperationResult::terrane_construct(
        raw_terrane_f1_s3657.failed,
        raw_terrane_f1_s3657.message.clone().clone(),
    );
}
pub fn file_close(
    capability: Filesystem,
    file: FileHandle,
) -> FilesystemOperationResult {
    let raw_terrane_f1_s3857: TerranePlatformUnitResult;
    let _ = &capability;
    raw_terrane_f1_s3857 = terrane_platform_close(
        &file.handle.as_ref().expect("required field initialized"),
    );
    return FilesystemOperationResult::terrane_construct(
        raw_terrane_f1_s3857.failed,
        raw_terrane_f1_s3857.message.clone().clone(),
    );
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
        self.authority = Some(authority);
    }
}
pub fn filesystem_capability() -> Filesystem {
    let authority_terrane_f1_s4178: TerraneFilesystemAuthority;
    authority_terrane_f1_s4178 = terrane_acquire_filesystem_authority();
    return Filesystem::terrane_construct(authority_terrane_f1_s4178);
}
pub fn filesystem_exists(capability: Filesystem, target: Path) -> ExistenceResult {
    let record_terrane_f1_s4352: TerraneFilesystemResult;
    let _ = &capability;
    record_terrane_f1_s4352 = terrane_filesystem_exists(target.text);
    return ExistenceResult::terrane_construct(
        terrane_filesystem_result_bool(&record_terrane_f1_s4352),
        terrane_filesystem_result_failed(&record_terrane_f1_s4352),
        terrane_filesystem_result_message(&record_terrane_f1_s4352),
    );
}
pub fn filesystem_metadata(capability: Filesystem, target: Path) -> FileMetadata {
    let record_terrane_f1_s4607: TerraneFilesystemResult;
    let _ = &capability;
    record_terrane_f1_s4607 = terrane_filesystem_metadata(target.text, true);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record_terrane_f1_s4607),
        terrane_filesystem_result_int(&record_terrane_f1_s4607),
        terrane_filesystem_result_bool(&record_terrane_f1_s4607),
        terrane_filesystem_result_detail(&record_terrane_f1_s4607),
        terrane_filesystem_result_failed(&record_terrane_f1_s4607),
        terrane_filesystem_result_message(&record_terrane_f1_s4607),
    );
}
pub fn filesystem_symlink_metadata(
    capability: Filesystem,
    target: Path,
) -> FileMetadata {
    let record_terrane_f1_s4960: TerraneFilesystemResult;
    let _ = &capability;
    record_terrane_f1_s4960 = terrane_filesystem_metadata(target.text, false);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record_terrane_f1_s4960),
        terrane_filesystem_result_int(&record_terrane_f1_s4960),
        terrane_filesystem_result_bool(&record_terrane_f1_s4960),
        terrane_filesystem_result_detail(&record_terrane_f1_s4960),
        terrane_filesystem_result_failed(&record_terrane_f1_s4960),
        terrane_filesystem_result_message(&record_terrane_f1_s4960),
    );
}
pub fn filesystem_canonical(capability: Filesystem, target: Path) -> PathResult {
    let record_terrane_f1_s5305: TerraneFilesystemResult;
    let resolved_terrane_f1_s5356: Path;
    let _ = &capability;
    record_terrane_f1_s5305 = terrane_filesystem_realpath(target.text);
    resolved_terrane_f1_s5356 = Path::terrane_construct(
        terrane_filesystem_result_text(&record_terrane_f1_s5305),
    );
    return PathResult::terrane_construct(
        resolved_terrane_f1_s5356,
        terrane_filesystem_result_failed(&record_terrane_f1_s5305),
        terrane_filesystem_result_message(&record_terrane_f1_s5305),
    );
}
pub fn filesystem_realpath(capability: Filesystem, target: Path) -> PathResult {
    return filesystem_canonical(capability, target);
}
pub fn filesystem_read_link(capability: Filesystem, target: Path) -> PathResult {
    let record_terrane_f1_s5725: TerraneFilesystemResult;
    let linked_terrane_f1_s5777: Path;
    let _ = &capability;
    record_terrane_f1_s5725 = terrane_filesystem_read_link(target.text);
    linked_terrane_f1_s5777 = Path::terrane_construct(
        terrane_filesystem_result_text(&record_terrane_f1_s5725),
    );
    return PathResult::terrane_construct(
        linked_terrane_f1_s5777,
        terrane_filesystem_result_failed(&record_terrane_f1_s5725),
        terrane_filesystem_result_message(&record_terrane_f1_s5725),
    );
}
pub fn filesystem_open_beneath(
    capability: Filesystem,
    directory: Path,
    relative: Path,
    cross_filesystem: bool,
) -> DirectoryHandle {
    let raw_terrane_f1_s6069: TerranePlatformOpenResult;
    let failure_terrane_f1_s6156: bool;
    let detail_terrane_f1_s6181: String;
    let acquired_terrane_f1_s6206: TerranePlatformStreamHandle;
    let _ = &capability;
    raw_terrane_f1_s6069 = terrane_platform_open_directory_beneath(
        directory.text,
        relative.text,
        cross_filesystem,
    );
    failure_terrane_f1_s6156 = raw_terrane_f1_s6069.failed;
    detail_terrane_f1_s6181 = raw_terrane_f1_s6069.message.clone().clone();
    acquired_terrane_f1_s6206 = raw_terrane_f1_s6069.handle.clone().clone();
    return DirectoryHandle::terrane_construct(
        acquired_terrane_f1_s6206,
        failure_terrane_f1_s6156,
        detail_terrane_f1_s6181,
    );
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
    let raw_terrane_f1_s6501: TerranePlatformOpenResult;
    let failure_terrane_f1_s6605: bool;
    let detail_terrane_f1_s6630: String;
    let acquired_terrane_f1_s6655: TerranePlatformStreamHandle;
    let _ = &capability;
    raw_terrane_f1_s6501 = terrane_platform_open_file_beneath(
        &directory.handle.as_ref().expect("required field initialized"),
        relative.text,
        readable,
        writable,
        create,
        truncate,
    );
    failure_terrane_f1_s6605 = raw_terrane_f1_s6501.failed;
    detail_terrane_f1_s6630 = raw_terrane_f1_s6501.message.clone().clone();
    acquired_terrane_f1_s6655 = raw_terrane_f1_s6501.handle.clone().clone();
    return FileHandle::terrane_construct(
        acquired_terrane_f1_s6655,
        failure_terrane_f1_s6605,
        detail_terrane_f1_s6630,
    );
}
pub fn filesystem_read_bounded(
    capability: Filesystem,
    target: Path,
    limit: terrane_int_support::Int,
) -> FileData {
    let record_terrane_f1_s6852: TerraneFilesystemResult;
    let _ = &capability;
    record_terrane_f1_s6852 = terrane_filesystem_read_bounded(target.text, limit);
    return FileData::terrane_construct(
        terrane_filesystem_result_bytes(&record_terrane_f1_s6852),
        terrane_filesystem_result_int(&record_terrane_f1_s6852),
        true,
        terrane_filesystem_result_failed(&record_terrane_f1_s6852),
        terrane_filesystem_result_message(&record_terrane_f1_s6852),
    );
}
pub fn filesystem_write_atomic(
    capability: Filesystem,
    target: Path,
    data: Vec<u8>,
) -> FilesystemOperationResult {
    let record_terrane_f1_s7177: TerraneFilesystemResult;
    let _ = &capability;
    record_terrane_f1_s7177 = terrane_filesystem_write_atomic(target.text, data);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record_terrane_f1_s7177),
        terrane_filesystem_result_message(&record_terrane_f1_s7177),
    );
}
pub fn filesystem_rename(
    capability: Filesystem,
    source: Path,
    destination: Path,
) -> FilesystemOperationResult {
    let record_terrane_f1_s7457: TerraneFilesystemResult;
    let _ = &capability;
    record_terrane_f1_s7457 = terrane_filesystem_rename(source.text, destination.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record_terrane_f1_s7457),
        terrane_filesystem_result_message(&record_terrane_f1_s7457),
    );
}
pub fn filesystem_remove(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let record_terrane_f1_s7725: TerraneFilesystemResult;
    let _ = &capability;
    record_terrane_f1_s7725 = terrane_filesystem_remove(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record_terrane_f1_s7725),
        terrane_filesystem_result_message(&record_terrane_f1_s7725),
    );
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
        self.text = input;
    }
}
pub fn path_components(subject: Path) -> terrane_collection_support::List<String> {
    let parts_terrane_f2_s224: Vec<String>;
    let mut result_terrane_f2_s260: terrane_collection_support::List<String>;
    let mut index_terrane_f2_s294: terrane_int_support::Int;
    let mut part_terrane_f2_s347: String;
    parts_terrane_f2_s224 = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    result_terrane_f2_s260 = terrane_collection_support::List::<String>::new(vec![]);
    index_terrane_f2_s294 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = result_terrane_f2_s260.make_unique();
        while index_terrane_f2_s294.clone()
            < terrane_int_support::Int::from(parts_terrane_f2_s224.len() as i128)
        {
            part_terrane_f2_s347 = __terrane_raised(
                {
                    let __terrane_receiver = &parts_terrane_f2_s224;
                    let __terrane_index = __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &index_terrane_f2_s294.clone(),
                        ),
                        0 /* terrane-site: core/paths.trn:16:16-16:28 */,
                    );
                    __terrane_receiver
                        .get(__terrane_index)
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index,
                        ))
                },
                0 /* terrane-site: core/paths.trn:16:16-16:28 */,
            );
            if part_terrane_f2_s347.as_str() != "" {
                __terrane_list_append_0.push(part_terrane_f2_s347);
            }
            index_terrane_f2_s294 = index_terrane_f2_s294.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    return result_terrane_f2_s260;
}
pub fn path_is_absolute(subject: Path) -> bool {
    return subject.text.starts_with(&String::from("/"));
}
pub fn normalise_path(subject: Path) -> Path {
    let parts_terrane_f2_s603: Vec<String>;
    let absolute_terrane_f2_s639: bool;
    let mut kept_terrane_f2_s680: terrane_collection_support::List<String>;
    let mut count_terrane_f2_s712: terrane_int_support::Int;
    let mut part_index_terrane_f2_s730: terrane_int_support::Int;
    let mut part_terrane_f2_s793: String;
    let mut result_terrane_f2_s1481: String;
    let mut index_terrane_f2_s1504: terrane_int_support::Int;
    parts_terrane_f2_s603 = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    absolute_terrane_f2_s639 = path_is_absolute(subject);
    kept_terrane_f2_s680 = terrane_collection_support::List::<String>::new(vec![]);
    count_terrane_f2_s712 = terrane_int_support::Int::from(0_i128);
    part_index_terrane_f2_s730 = terrane_int_support::Int::from(0_i128);
    while part_index_terrane_f2_s730.clone()
        < terrane_int_support::Int::from(parts_terrane_f2_s603.len() as i128)
    {
        part_terrane_f2_s793 = __terrane_raised(
            {
                let __terrane_receiver = &parts_terrane_f2_s603;
                let __terrane_index = __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &part_index_terrane_f2_s730.clone(),
                    ),
                    1 /* terrane-site: core/paths.trn:32:16-32:33 */,
                );
                __terrane_receiver
                    .get(__terrane_index)
                    .cloned()
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index,
                    ))
            },
            1 /* terrane-site: core/paths.trn:32:16-32:33 */,
        );
        if part_terrane_f2_s793.as_str() != "" && part_terrane_f2_s793.as_str() != "." {
            if part_terrane_f2_s793.as_str() == ".." {
                if count_terrane_f2_s712.clone() > terrane_int_support::Int::from(0_i128)
                    && __terrane_raised(
                            kept_terrane_f2_s680
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &(count_terrane_f2_s712.clone()
                                                - terrane_int_support::Int::from(1_i128)),
                                        ),
                                        2 /* terrane-site: core/paths.trn:35:34-35:49 */,
                                    ),
                                ),
                            2 /* terrane-site: core/paths.trn:35:34-35:49 */,
                        )
                        .as_str() != ".."
                {
                    count_terrane_f2_s712 = count_terrane_f2_s712.clone()
                        - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute_terrane_f2_s639 {
                        if count_terrane_f2_s712.clone()
                            < terrane_int_support::Int::from(
                                terrane_int_support::Int::from(
                                    kept_terrane_f2_s680.length(),
                                ),
                            )
                        {
                            __terrane_raised(
                                kept_terrane_f2_s680
                                    .set(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(
                                                &count_terrane_f2_s712.clone(),
                                            ),
                                            3 /* terrane-site: core/paths.trn:40:29-40:50 */,
                                        ),
                                        part_terrane_f2_s793,
                                    ),
                                3 /* terrane-site: core/paths.trn:40:29-40:50 */,
                            );
                        } else {
                            kept_terrane_f2_s680.append(part_terrane_f2_s793);
                        }
                        count_terrane_f2_s712 = count_terrane_f2_s712.clone()
                            + terrane_int_support::Int::from(1_i128);
                    }
                }
            } else {
                if count_terrane_f2_s712.clone()
                    < terrane_int_support::Int::from(
                        terrane_int_support::Int::from(kept_terrane_f2_s680.length()),
                    )
                {
                    __terrane_raised(
                        kept_terrane_f2_s680
                            .set(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &count_terrane_f2_s712.clone(),
                                    ),
                                    4 /* terrane-site: core/paths.trn:46:21-46:42 */,
                                ),
                                part_terrane_f2_s793,
                            ),
                        4 /* terrane-site: core/paths.trn:46:21-46:42 */,
                    );
                } else {
                    kept_terrane_f2_s680.append(part_terrane_f2_s793);
                }
                count_terrane_f2_s712 = count_terrane_f2_s712.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
        }
        part_index_terrane_f2_s730 = part_index_terrane_f2_s730.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    result_terrane_f2_s1481 = String::from("");
    index_terrane_f2_s1504 = terrane_int_support::Int::from(0_i128);
    while index_terrane_f2_s1504.clone() < count_terrane_f2_s712.clone() {
        if result_terrane_f2_s1481.as_str() != "" {
            result_terrane_f2_s1481 = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result_terrane_f2_s1481),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result_terrane_f2_s1481 = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result_terrane_f2_s1481),
            terrane_scalar_support::scalar_text(&__terrane_raised(kept_terrane_f2_s680
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index_terrane_f2_s1504
            .clone()), 5 /* terrane-site: core/paths.trn:56:33-56:44 */)),
            5 /* terrane-site: core/paths.trn:56:33-56:44 */))
        );
        index_terrane_f2_s1504 = index_terrane_f2_s1504.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    if absolute_terrane_f2_s639 {
        result_terrane_f2_s1481 = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result_terrane_f2_s1481)
        );
    }
    if result_terrane_f2_s1481.as_str() == "" && absolute_terrane_f2_s639 {
        result_terrane_f2_s1481 = String::from("/");
    }
    return Path::terrane_construct(result_terrane_f2_s1481);
}
pub fn path_name(subject: Path) -> String {
    let normal_terrane_f2_s1860: Path;
    let parts_terrane_f2_s1897: terrane_collection_support::List<String>;
    normal_terrane_f2_s1860 = normalise_path(subject);
    parts_terrane_f2_s1897 = path_components(normal_terrane_f2_s1860);
    if terrane_int_support::Int::from(
        terrane_int_support::Int::from(parts_terrane_f2_s1897.length()),
    ) == terrane_int_support::Int::from(0_i128)
    {
        return String::from("");
    }
    return __terrane_raised(
        parts_terrane_f2_s1897
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(
                            terrane_int_support::Int::from(
                                parts_terrane_f2_s1897.length(),
                            ),
                        ) - terrane_int_support::Int::from(1_i128)),
                    ),
                    6 /* terrane-site: core/paths.trn:69:12-69:35 */,
                ),
            ),
        6 /* terrane-site: core/paths.trn:69:12-69:35 */,
    );
}
pub fn path_parent(subject: Path) -> Path {
    let normal_terrane_f2_s2052: Path;
    let parts_terrane_f2_s2089: terrane_collection_support::List<String>;
    let mut result_terrane_f2_s2266: String;
    let mut index_terrane_f2_s2289: terrane_int_support::Int;
    let absolute_terrane_f2_s2477: bool;
    normal_terrane_f2_s2052 = normalise_path(subject);
    parts_terrane_f2_s2089 = path_components(normal_terrane_f2_s2052.clone());
    if terrane_int_support::Int::from(
        terrane_int_support::Int::from(parts_terrane_f2_s2089.length()),
    ) == terrane_int_support::Int::from(0_i128)
    {
        return normal_terrane_f2_s2052.clone();
    }
    if terrane_int_support::Int::from(
        terrane_int_support::Int::from(parts_terrane_f2_s2089.length()),
    ) == terrane_int_support::Int::from(1_i128)
        && !path_is_absolute(normal_terrane_f2_s2052.clone())
    {
        return Path::terrane_construct(String::from("."));
    }
    result_terrane_f2_s2266 = String::from("");
    index_terrane_f2_s2289 = terrane_int_support::Int::from(0_i128);
    while index_terrane_f2_s2289.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(parts_terrane_f2_s2089.length()),
        ) - terrane_int_support::Int::from(1_i128)
    {
        if result_terrane_f2_s2266.as_str() != "" {
            result_terrane_f2_s2266 = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result_terrane_f2_s2266),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result_terrane_f2_s2266 = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result_terrane_f2_s2266),
            terrane_scalar_support::scalar_text(&__terrane_raised(parts_terrane_f2_s2089
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index_terrane_f2_s2289
            .clone()), 7 /* terrane-site: core/paths.trn:83:33-83:45 */)),
            7 /* terrane-site: core/paths.trn:83:33-83:45 */))
        );
        index_terrane_f2_s2289 = index_terrane_f2_s2289.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    absolute_terrane_f2_s2477 = path_is_absolute(normal_terrane_f2_s2052);
    if absolute_terrane_f2_s2477 {
        result_terrane_f2_s2266 = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result_terrane_f2_s2266)
        );
    }
    return Path::terrane_construct(result_terrane_f2_s2266);
}
pub fn path_stem(subject: Path) -> String {
    let current_terrane_f2_s2643: String;
    let pieces_terrane_f2_s2676: Vec<String>;
    let mut result_terrane_f2_s2826: String;
    let mut index_terrane_f2_s2849: terrane_int_support::Int;
    current_terrane_f2_s2643 = path_name(subject);
    pieces_terrane_f2_s2676 = terrane_string_support::split(
        &current_terrane_f2_s2643,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces_terrane_f2_s2676.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return current_terrane_f2_s2643.clone();
    }
    if terrane_int_support::Int::from(pieces_terrane_f2_s2676.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
                {
                    let __terrane_receiver = &pieces_terrane_f2_s2676;
                    let __terrane_index = __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        8 /* terrane-site: core/paths.trn:95:31-95:40 */,
                    );
                    __terrane_receiver
                        .get(__terrane_index)
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index,
                        ))
                },
                8 /* terrane-site: core/paths.trn:95:31-95:40 */,
            )
            .as_str() == ""
    {
        return current_terrane_f2_s2643;
    }
    result_terrane_f2_s2826 = String::from("");
    index_terrane_f2_s2849 = terrane_int_support::Int::from(0_i128);
    while index_terrane_f2_s2849.clone()
        < terrane_int_support::Int::from(pieces_terrane_f2_s2676.len() as i128)
            - terrane_int_support::Int::from(1_i128)
    {
        if index_terrane_f2_s2849.clone() > terrane_int_support::Int::from(0_i128) {
            result_terrane_f2_s2826 = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result_terrane_f2_s2826),
                terrane_scalar_support::scalar_text(&String::from("."))
            );
        }
        result_terrane_f2_s2826 = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result_terrane_f2_s2826),
            terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_receiver = &pieces_terrane_f2_s2676; let __terrane_index =
            __terrane_raised(terrane_collection_support::index_from_int(&index_terrane_f2_s2849
            .clone()), 9 /* terrane-site: core/paths.trn:102:33-102:46 */);
            __terrane_receiver.get(__terrane_index).cloned().ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index)) },
            9 /* terrane-site: core/paths.trn:102:33-102:46 */))
        );
        index_terrane_f2_s2849 = index_terrane_f2_s2849.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    return result_terrane_f2_s2826;
}
pub fn path_extension(subject: Path) -> String {
    let current_terrane_f2_s3100: String;
    let pieces_terrane_f2_s3133: Vec<String>;
    current_terrane_f2_s3100 = path_name(subject);
    pieces_terrane_f2_s3133 = terrane_string_support::split(
        &current_terrane_f2_s3100,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces_terrane_f2_s3133.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return String::from("");
    }
    if terrane_int_support::Int::from(pieces_terrane_f2_s3133.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
                {
                    let __terrane_receiver = &pieces_terrane_f2_s3133;
                    let __terrane_index = __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        10 /* terrane-site: core/paths.trn:111:31-111:40 */,
                    );
                    __terrane_receiver
                        .get(__terrane_index)
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index,
                        ))
                },
                10 /* terrane-site: core/paths.trn:111:31-111:40 */,
            )
            .as_str() == ""
    {
        return String::from("");
    }
    return __terrane_raised(
        {
            let __terrane_receiver = &pieces_terrane_f2_s3133;
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(
                        pieces_terrane_f2_s3133.len() as i128,
                    ) - terrane_int_support::Int::from(1_i128)),
                ),
                11 /* terrane-site: core/paths.trn:113:12-113:37 */,
            );
            __terrane_receiver
                .get(__terrane_index)
                .cloned()
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index,
                ))
        },
        11 /* terrane-site: core/paths.trn:113:12-113:37 */,
    );
}
pub fn join_path(base: Path, child: Path) -> Path {
    let absolute_terrane_f2_s3358: bool;
    let mut joined_terrane_f2_s3450: String;
    let combined_terrane_f2_s3608: Path;
    absolute_terrane_f2_s3358 = path_is_absolute(child.clone());
    if absolute_terrane_f2_s3358 {
        return normalise_path(child.clone());
    }
    joined_terrane_f2_s3450 = base.text.clone();
    if joined_terrane_f2_s3450.as_str() != ""
        && !joined_terrane_f2_s3450.ends_with(&String::from("/"))
    {
        joined_terrane_f2_s3450 = format!(
            "{}{}", terrane_scalar_support::scalar_text(&joined_terrane_f2_s3450),
            terrane_scalar_support::scalar_text(&String::from("/"))
        );
    }
    joined_terrane_f2_s3450 = format!(
        "{}{}", terrane_scalar_support::scalar_text(&joined_terrane_f2_s3450),
        terrane_scalar_support::scalar_text(&child.text)
    );
    combined_terrane_f2_s3608 = Path::terrane_construct(joined_terrane_f2_s3450);
    return normalise_path(combined_terrane_f2_s3608);
}
