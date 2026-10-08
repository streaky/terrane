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
    pub static FUNCTIONS: [&str; 2] = ["/app::render", "/app::main"];
    pub static SITES: [Site; 9] = [
        /* terrane-site-row: site 0: /app::render (src/main.trn:11:134-11:150) */
        { Site { function: 0, file: 0, line: 11, column: 134, end_line: 11, end_column: 150 } },
        /* terrane-site-row: site 1: /app::render (src/main.trn:11:118-11:151) */
        { Site { function: 0, file: 0, line: 11, column: 118, end_line: 11, end_column: 151 } },
        /* terrane-site-row: site 2: /app::render (src/main.trn:11:156-11:178) */
        { Site { function: 0, file: 0, line: 11, column: 156, end_line: 11, end_column: 178 } },
        /* terrane-site-row: site 3: /app::render (src/main.trn:11:31-11:179) */
        { Site { function: 0, file: 0, line: 11, column: 31, end_line: 11, end_column: 179 } },
        /* terrane-site-row: site 4: /app::render (src/main.trn:11:12-11:180) */
        { Site { function: 0, file: 0, line: 11, column: 12, end_line: 11, end_column: 180 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:14:23-14:56) */
        { Site { function: 1, file: 0, line: 14, column: 23, end_line: 14, end_column: 56 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:15:23-15:57) */
        { Site { function: 1, file: 0, line: 15, column: 23, end_line: 15, end_column: 57 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:16:13-16:53) */
        { Site { function: 1, file: 0, line: 16, column: 13, end_line: 16, end_column: 53 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:17:13-17:35) */
        { Site { function: 1, file: 0, line: 17, column: 13, end_line: 17, end_column: 35 } },
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
// Source: src/main.trn
// Namespace: app
fn render<'view>(state: &'view State) -> iced::Element<'view, ()> {
    let mut children: terrane_collection_support::List<_>;
    let mut child: &State;
    children = terrane_collection_support::List::<_>::new(vec![]);
    let __terrane_iterable_0 = &state.children;
    let mut __terrane_iterator_0 = __terrane_iterable_0.iter();
    loop {
        child = match __terrane_iterator_0.next() {
            Some(item) => item,
            None => break,
        };
        children.push_unique(render(child));
    }
    return {
        let value: iced::Element<'_, ()> = __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| iced::widget::container::<
                        _,
                        _,
                        _,
                    >(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                __terrane_raised(
                                    match std::panic::catch_unwind(
                                        std::panic::AssertUnwindSafe(|| {
                                            iced::widget::column!(
                                                iced::widget::text!("macro {}", 7_i64),
                                                iced::widget::row!(iced::widget::text!("nested {}", 11_i64),
                                                __terrane_raised(match
                                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                                iced::widget::text:: < _, _ > (match | | -> Result < _,
                                                crate ::TerraneForeignError > { Ok(__terrane_raised(match
                                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                                (&state).get_label())) { Ok(value) => Ok(value.to_owned()),
                                                Err(payload) => Err(crate
                                                ::__terrane_dependency_panic(payload, "witness",
                                                "witness::State::get_label")) }, 0 /* terrane-site: src/main.trn:11:134-11:150 */)) } () { Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error) }))) { Ok(value)
                                                => Ok(value), Err(payload) => Err(crate
                                                ::__terrane_dependency_panic(payload, "iced",
                                                "iced::widget::text::<_, _>")) }, 1 /* terrane-site: src/main.trn:11:118-11:151 */)), __terrane_raised(match
                                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                                                iced::widget::row:: < _, _, _ > (children.into_unique_vec()
                                                .into_iter().map(Into::into).collect:: < Vec < _ > > ()))) {
                                                Ok(value) => Ok(value), Err(payload) => Err(crate
                                                ::__terrane_dependency_panic(payload, "iced",
                                                "iced::widget::row::<_, _, _>")) }, 2 /* terrane-site: src/main.trn:11:156-11:178 */)
                                            )
                                        }),
                                    ) {
                                        Ok(value) => Ok(value),
                                        Err(payload) => {
                                            Err(
                                                crate::__terrane_dependency_panic(
                                                    payload,
                                                    "iced",
                                                    "iced::widget::column",
                                                ),
                                            )
                                        }
                                    },
                                    3 /* terrane-site: src/main.trn:11:31-11:179 */,
                                ),
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "iced",
                                "iced::widget::container::<_, _, _>",
                            ),
                        )
                    }
                },
                4 /* terrane-site: src/main.trn:11:12-11:180 */,
            )
            .into();
        value
    };
}
fn main() {
    let mut children: terrane_collection_support::List<State>;
    let state: State;
    children = terrane_collection_support::List::<State>::new(vec![]);
    children
        .append(
            __terrane_raised(
                terrane_static_trn_5374617465_new(String::from("first borrowed leaf")),
                5 /* terrane-site: src/main.trn:14:23-14:56 */,
            ),
        );
    children
        .append(
            __terrane_raised(
                terrane_static_trn_5374617465_new(String::from("second borrowed leaf")),
                6 /* terrane-site: src/main.trn:15:23-15:57 */,
            ),
        );
    state = __terrane_raised(
        terrane_static_trn_5374617465_branch(String::from("borrowed root"), children),
        7 /* terrane-site: src/main.trn:16:13-16:53 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(inspect(&state,
        render), 8 /* terrane-site: src/main.trn:17:13-17:35 */))
    );
}
// Source: <terrane>/projected/deps/iced/widget.trn
// Namespace: deps/iced/widget
// Source: <terrane>/projected/deps/iced/widget/macros.trn
// Namespace: deps/iced/widget/macros
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub use witness::State;
pub fn terrane_static_trn_5374617465_branch(
    label_: String,
    children: terrane_collection_support::List<State>,
) -> Result<State, crate::TerraneForeignError> {
    let label_ = label_;
    let children = children.into_vec();
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::State::branch(label_, children)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::State"))
        }
    }
}
pub fn terrane_static_trn_5374617465_new(
    label_: String,
) -> Result<State, crate::TerraneForeignError> {
    let label_ = label_;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::State::new(label_)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::State"))
        }
    }
}
pub fn inspect<F: for<'a> std::ops::Fn(&'a witness::State) -> iced::Element<'a, ()>>(
    state: &State,
    render: F,
) -> Result<String, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::inspect(state, render)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(payload, "witness", "witness::inspect"),
            )
        }
    }
}
