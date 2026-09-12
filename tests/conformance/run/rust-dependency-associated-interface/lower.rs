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
    pub static FUNCTIONS: [&str; 1] = ["/app::main"];
    pub static SITES: [Site; 6] = [
        /* terrane-site-row: site 0: /app::main (src/main.trn:37:13-37:45) */
        { Site { function: 0, file: 0, line: 37, column: 13, end_line: 37, end_column: 45 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:38:13-38:52) */
        { Site { function: 0, file: 0, line: 38, column: 13, end_line: 38, end_column: 52 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:39:13-39:48) */
        { Site { function: 0, file: 0, line: 39, column: 13, end_line: 39, end_column: 48 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:40:13-40:43) */
        { Site { function: 0, file: 0, line: 40, column: 13, end_line: 40, end_column: 43 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:41:13-41:49) */
        { Site { function: 0, file: 0, line: 41, column: 13, end_line: 41, end_column: 49 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:43:13-43:34) */
        { Site { function: 0, file: 0, line: 43, column: 13, end_line: 43, end_column: 34 } },
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
#[derive(Clone)]
pub struct Ints {
    pub value: terrane_int_support::Int,
}
impl Ints {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(40_i128),
        }
    }
    pub fn first(&self) -> Option<terrane_int_support::Int> {
        return Some(self.value.clone());
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<terrane_int_support::Int>
for Ints {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        Ints::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        Ints::length(&*self)
    }
}
impl From<Ints>
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int> {
    fn from(value: Ints) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for Ints {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Ints>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Ints>::length(&*self);
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
pub struct NestedValues {}
impl NestedValues {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn first(&self) -> Option<Nested> {
        return None;
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<Nested> for NestedValues {
    fn clone_box(
        &self,
    ) -> Box<dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<Nested>> {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<Nested>> {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<Nested> {
        NestedValues::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NestedValues::length(&*self)
    }
}
impl From<NestedValues> for TerraneNs4Deps26TerraneAssociatedWitnessSequence<Nested> {
    fn from(value: NestedValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for NestedValues {
    type Item = terrane_associated_witness::Nested;
    fn first(&self) -> Option<terrane_associated_witness::Nested> {
        let __terrane_boundary: Result<
            Option<terrane_associated_witness::Nested>,
            crate::TerraneForeignError,
        > = (|| {
            let __terrane_value = <NestedValues>::first(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NestedValues>::length(&*self);
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
pub struct NamedInts {
    pub value: terrane_int_support::Int,
}
impl NamedInts {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(30_i128),
        }
    }
    pub fn first(&self) -> Option<terrane_int_support::Int> {
        return Some(self.value.clone());
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(4_i128);
    }
    pub fn label(&self) -> String {
        return String::from("name");
    }
}
impl TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
    terrane_int_support::Int,
> for NamedInts {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        NamedInts::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NamedInts::length(&*self)
    }
    fn label(&self) -> String {
        NamedInts::label(&*self)
    }
}
impl From<NamedInts>
for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<terrane_int_support::Int> {
    fn from(value: NamedInts) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::NamedSequence for NamedInts {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NamedInts>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<terrane_int_support::Int>
for NamedInts {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
            terrane_int_support::Int,
        >,
    > {
        Box::new(self.clone())
    }
    fn first(&self) -> Option<terrane_int_support::Int> {
        NamedInts::first(&*self)
    }
    fn length(&self) -> terrane_int_support::Int {
        NamedInts::length(&*self)
    }
}
impl From<NamedInts>
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int> {
    fn from(value: NamedInts) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_associated_witness::Sequence for NamedInts {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NamedInts>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <NamedInts>::length(&*self);
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
fn interface_length(
    value: TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return value.length();
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(sequence_total(Ints::terrane_construct()),
        0 /* terrane-site: src/main.trn:37:13-37:45 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(nested_total(NestedValues::terrane_construct()),
        1 /* terrane-site: src/main.trn:38:13-38:52 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(named_total(NamedInts::terrane_construct()),
        2 /* terrane-site: src/main.trn:39:13-39:48 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(Ints::terrane_construct()),
        3 /* terrane-site: src/main.trn:40:13-40:43 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(NamedInts::terrane_construct()),
        4 /* terrane-site: src/main.trn:41:13-41:49 */))
    );
    let applied: TerraneNs4Deps26TerraneAssociatedWitnessSequence<
        terrane_int_support::Int,
    > = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
        terrane_int_support::Int,
    >>::from(Ints::terrane_construct());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_total(applied),
        5 /* terrane-site: src/main.trn:43:13-43:34 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&interface_length(<
        TerraneNs4Deps26TerraneAssociatedWitnessSequence < terrane_int_support::Int > >
        ::from(Ints::terrane_construct())))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&interface_length(<
        TerraneNs4Deps26TerraneAssociatedWitnessSequence < terrane_int_support::Int > >
        ::from(NamedInts::terrane_construct())))
    );
}
// Source: <terrane>/projected/deps/terrane-associated-witness.trn
// Namespace: deps/terrane-associated-witness
pub trait TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            TerraneAssociated,
        >,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            TerraneAssociated,
        >,
    >;
    fn first(&self) -> Option<TerraneAssociated>;
    fn length(&self) -> terrane_int_support::Int;
    fn label(&self) -> String;
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone
for Box<
    dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<TerraneAssociated>,
> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>(
    Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessNamedSequenceProtocol<
            TerraneAssociated,
        >,
    >,
);
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<TerraneAssociated> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<TerraneAssociated> {
    pub fn first(&self) -> Option<TerraneAssociated> {
        self.0.first()
    }
    pub fn length(&self) -> terrane_int_support::Int {
        self.0.length()
    }
    pub fn label(&self) -> String {
        self.0.label()
    }
}
impl terrane_associated_witness::Sequence
for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<terrane_int_support::Int> {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
                terrane_int_support::Int,
            >>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
                terrane_int_support::Int,
            >>::length(&*self);
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
impl terrane_associated_witness::NamedSequence
for TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<terrane_int_support::Int> {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessNamedSequence<
                terrane_int_support::Int,
            >>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>,
    >;
    fn first(&self) -> Option<TerraneAssociated>;
    fn length(&self) -> terrane_int_support::Int;
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone
for Box<
    dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>,
> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNs4Deps26TerraneAssociatedWitnessSequence<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
>(
    Box<dyn TerraneNs4Deps26TerraneAssociatedWitnessSequenceProtocol<TerraneAssociated>>,
);
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> Clone for TerraneNs4Deps26TerraneAssociatedWitnessSequence<TerraneAssociated> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneAssociated: 'static + core::clone::Clone + core::marker::Send
        + core::marker::Sync + 'static,
> TerraneNs4Deps26TerraneAssociatedWitnessSequence<TerraneAssociated> {
    pub fn first(&self) -> Option<TerraneAssociated> {
        self.0.first()
    }
    pub fn length(&self) -> terrane_int_support::Int {
        self.0.length()
    }
}
impl terrane_associated_witness::Sequence
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<Nested> {
    type Item = terrane_associated_witness::Nested;
    fn first(&self) -> Option<terrane_associated_witness::Nested> {
        let __terrane_boundary: Result<
            Option<terrane_associated_witness::Nested>,
            crate::TerraneForeignError,
        > = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                Nested,
            >>::first(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                Nested,
            >>::length(&*self);
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
impl terrane_associated_witness::Sequence
for TerraneNs4Deps26TerraneAssociatedWitnessSequence<terrane_int_support::Int> {
    type Item = i64;
    fn first(&self) -> Option<i64> {
        let __terrane_boundary: Result<Option<i64>, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                terrane_int_support::Int,
            >>::first(&*self);
            Ok(
                __terrane_value
                    .map(|value| -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<i64>(&value)
                                .map_err(|error| crate::TerraneForeignError(
                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                ))?,
                        )
                    })
                    .transpose()?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn length(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <TerraneNs4Deps26TerraneAssociatedWitnessSequence<
                terrane_int_support::Int,
            >>::length(&*self);
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
pub use terrane_associated_witness::Nested;
pub fn erased_total<
    TerraneBoxed0: terrane_associated_witness::Sequence<Item = i64> + 'static,
>(value: TerraneBoxed0) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = Box::new(value);
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::erased_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::erased_total",
                ),
            )
        }
    }
}
pub fn named_total<T: terrane_associated_witness::NamedSequence<Item = i64>>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::named_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::named_total",
                ),
            )
        }
    }
}
pub fn nested_total<
    T: terrane_associated_witness::Sequence<Item = terrane_associated_witness::Nested>,
>(value: T) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::nested_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::nested_total",
                ),
            )
        }
    }
}
pub fn sequence_total<T: terrane_associated_witness::Sequence<Item = i64>>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_associated_witness::sequence_total(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-associated-witness",
                    "terrane_associated_witness::sequence_total",
                ),
            )
        }
    }
}
