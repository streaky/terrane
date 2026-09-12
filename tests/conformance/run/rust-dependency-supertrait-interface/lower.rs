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
    pub static SITES: [Site; 3] = [
        /* terrane-site-row: site 0: /app::main (src/main.trn:23:13-23:44) */
        { Site { function: 0, file: 0, line: 23, column: 13, end_line: 23, end_column: 44 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:24:13-24:51) */
        { Site { function: 0, file: 0, line: 24, column: 13, end_line: 24, end_column: 51 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:25:13-25:54) */
        { Site { function: 0, file: 0, line: 25, column: 13, end_line: 25, end_column: 54 } },
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
pub struct Values {}
impl Values {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn middle(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(12_i128);
    }
    pub fn child(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(20_i128);
    }
}
impl ChildProtocol for Values {
    fn clone_box(&self) -> Box<dyn ChildProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ChildProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <Values as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn middle(&self) -> terrane_int_support::Int {
        Values::middle(&*self)
    }
    fn child(&self) -> terrane_int_support::Int {
        Values::child(&*self)
    }
}
impl From<Values> for Child {
    fn from(value: Values) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Child for Values {
    fn child(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Values>::child(&*self);
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
impl MiddleProtocol for Values {
    fn clone_box(&self) -> Box<dyn MiddleProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn MiddleProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <Values as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn middle(&self) -> terrane_int_support::Int {
        Values::middle(&*self)
    }
}
impl From<Values> for Middle {
    fn from(value: Values) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Middle for Values {
    fn middle(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Values>::middle(&*self);
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
impl BaseProtocol for Values {
    fn clone_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <Values as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl From<Values> for Base {
    fn from(value: Values) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Base for Values {}
#[derive(Clone)]
pub struct DiamondValues {}
impl DiamondValues {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn left(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn right(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
    pub fn diamond(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
}
impl DiamondProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn DiamondProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DiamondProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn left(&self) -> terrane_int_support::Int {
        DiamondValues::left(&*self)
    }
    fn right(&self) -> terrane_int_support::Int {
        DiamondValues::right(&*self)
    }
    fn diamond(&self) -> terrane_int_support::Int {
        DiamondValues::diamond(&*self)
    }
}
impl From<DiamondValues> for Diamond {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Diamond for DiamondValues {
    fn diamond(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DiamondValues>::diamond(&*self);
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
impl LeftProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn LeftProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LeftProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn left(&self) -> terrane_int_support::Int {
        DiamondValues::left(&*self)
    }
}
impl From<DiamondValues> for Left {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Left for DiamondValues {
    fn left(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DiamondValues>::left(&*self);
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
impl BaseProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn BaseProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl From<DiamondValues> for Base {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Base for DiamondValues {}
impl RightProtocol for DiamondValues {
    fn clone_box(&self) -> Box<dyn RightProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RightProtocol> {
        Box::new(self.clone())
    }
    fn base(&self) -> terrane_int_support::Int {
        || -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
            let __terrane_default = <DiamondValues as terrane_supertrait_witness::Base>::base(
                &*self,
            );
            Ok(terrane_int_support::Int::from(i128::from(__terrane_default)))
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn right(&self) -> terrane_int_support::Int {
        DiamondValues::right(&*self)
    }
}
impl From<DiamondValues> for Right {
    fn from(value: DiamondValues) -> Self {
        Self(Box::new(value))
    }
}
impl terrane_supertrait_witness::Right for DiamondValues {
    fn right(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DiamondValues>::right(&*self);
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
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(child_total(Values::terrane_construct()),
        0 /* terrane-site: src/main.trn:23:13-23:44 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(erased_child_total(Values::terrane_construct()),
        1 /* terrane-site: src/main.trn:24:13-24:51 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(diamond_total(DiamondValues::terrane_construct()),
        2 /* terrane-site: src/main.trn:25:13-25:54 */))
    );
}
// Source: <terrane>/projected/deps/terrane-supertrait-witness.trn
// Namespace: deps/terrane-supertrait-witness
pub trait BaseProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn BaseProtocol>;
    fn separate_box(&self) -> Box<dyn BaseProtocol>;
    fn base(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn BaseProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Base(Box<dyn BaseProtocol>);
impl Clone for Base {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Base {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
}
impl terrane_supertrait_witness::Base for Base {}
pub trait ChildProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn ChildProtocol>;
    fn separate_box(&self) -> Box<dyn ChildProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn middle(&self) -> terrane_int_support::Int;
    fn child(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn ChildProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Child(Box<dyn ChildProtocol>);
impl Clone for Child {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Child {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn middle(&self) -> terrane_int_support::Int {
        self.0.middle()
    }
    pub fn child(&self) -> terrane_int_support::Int {
        self.0.child()
    }
}
impl terrane_supertrait_witness::Middle for Child {
    fn middle(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Child>::middle(&*self);
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
impl terrane_supertrait_witness::Base for Child {}
impl terrane_supertrait_witness::Child for Child {
    fn child(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Child>::child(&*self);
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
pub trait DiamondProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DiamondProtocol>;
    fn separate_box(&self) -> Box<dyn DiamondProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn left(&self) -> terrane_int_support::Int;
    fn right(&self) -> terrane_int_support::Int;
    fn diamond(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn DiamondProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Diamond(Box<dyn DiamondProtocol>);
impl Clone for Diamond {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Diamond {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn left(&self) -> terrane_int_support::Int {
        self.0.left()
    }
    pub fn right(&self) -> terrane_int_support::Int {
        self.0.right()
    }
    pub fn diamond(&self) -> terrane_int_support::Int {
        self.0.diamond()
    }
}
impl terrane_supertrait_witness::Left for Diamond {
    fn left(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Diamond>::left(&*self);
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
impl terrane_supertrait_witness::Base for Diamond {}
impl terrane_supertrait_witness::Right for Diamond {
    fn right(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Diamond>::right(&*self);
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
impl terrane_supertrait_witness::Diamond for Diamond {
    fn diamond(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Diamond>::diamond(&*self);
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
pub trait LeftProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn LeftProtocol>;
    fn separate_box(&self) -> Box<dyn LeftProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn left(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn LeftProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Left(Box<dyn LeftProtocol>);
impl Clone for Left {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Left {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn left(&self) -> terrane_int_support::Int {
        self.0.left()
    }
}
impl terrane_supertrait_witness::Base for Left {}
impl terrane_supertrait_witness::Left for Left {
    fn left(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Left>::left(&*self);
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
pub trait MiddleProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn MiddleProtocol>;
    fn separate_box(&self) -> Box<dyn MiddleProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn middle(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn MiddleProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Middle(Box<dyn MiddleProtocol>);
impl Clone for Middle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Middle {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn middle(&self) -> terrane_int_support::Int {
        self.0.middle()
    }
}
impl terrane_supertrait_witness::Base for Middle {}
impl terrane_supertrait_witness::Middle for Middle {
    fn middle(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Middle>::middle(&*self);
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
pub trait RightProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn RightProtocol>;
    fn separate_box(&self) -> Box<dyn RightProtocol>;
    fn base(&self) -> terrane_int_support::Int;
    fn right(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn RightProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Right(Box<dyn RightProtocol>);
impl Clone for Right {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Right {
    pub fn base(&self) -> terrane_int_support::Int {
        self.0.base()
    }
    pub fn right(&self) -> terrane_int_support::Int {
        self.0.right()
    }
}
impl terrane_supertrait_witness::Base for Right {}
impl terrane_supertrait_witness::Right for Right {
    fn right(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Right>::right(&*self);
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
pub fn child_total<T: terrane_supertrait_witness::Child>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_supertrait_witness::child_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-supertrait-witness",
                    "terrane_supertrait_witness::child_total",
                ),
            )
        }
    }
}
pub fn diamond_total<T: terrane_supertrait_witness::Diamond>(
    value: T,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_supertrait_witness::diamond_total(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-supertrait-witness",
                    "terrane_supertrait_witness::diamond_total",
                ),
            )
        }
    }
}
pub fn erased_child_total<
    TerraneBoxed0: terrane_supertrait_witness::Child + core::marker::Send
        + core::marker::Sync + 'static,
>(value: TerraneBoxed0) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = Box::new(value);
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_supertrait_witness::erased_child_total(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-supertrait-witness",
                    "terrane_supertrait_witness::erased_child_total",
                ),
            )
        }
    }
}
