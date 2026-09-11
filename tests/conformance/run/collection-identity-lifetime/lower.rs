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
    pub static FUNCTIONS: [&str; 1] = ["/collection-identity-lifetime::release-order"];
    pub static SITES: [Site; 6] = [
        /* terrane-site-row: site 0: /collection-identity-lifetime::release-order (case.trn:18:3-18:50) */
        { Site { function: 0, file: 0, line: 18, column: 3, end_line: 18, end_column: 50 } },
        /* terrane-site-row: site 1: /collection-identity-lifetime::release-order (case.trn:21:22-21:38) */
        { Site { function: 0, file: 0, line: 21, column: 22, end_line: 21, end_column: 38 } },
        /* terrane-site-row: site 2: /collection-identity-lifetime::release-order (case.trn:25:5-25:21) */
        { Site { function: 0, file: 0, line: 25, column: 5, end_line: 25, end_column: 21 } },
        /* terrane-site-row: site 3: /collection-identity-lifetime::release-order (case.trn:34:3-34:57) */
        { Site { function: 0, file: 0, line: 34, column: 3, end_line: 34, end_column: 57 } },
        /* terrane-site-row: site 4: /collection-identity-lifetime::release-order (case.trn:35:10-35:21) */
        { Site { function: 0, file: 0, line: 35, column: 10, end_line: 35, end_column: 21 } },
        /* terrane-site-row: site 5: /collection-identity-lifetime::release-order (case.trn:35:28-35:40) */
        { Site { function: 0, file: 0, line: 35, column: 28, end_line: 35, end_column: 40 } },
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
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneFieldMetadata {
    name: &'static str,
    external_name: &'static str,
    defaulted: bool,
    optional: bool,
    secret: bool,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
    inherently_identity_bearing: bool,
    fields: &'static [TerraneFieldMetadata],
}
// Source: case.trn
// Namespace: collection-identity-lifetime
#[derive(Clone)]
pub struct Marker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub name: String,
}
impl Marker {
    pub fn terrane_construct(name: String) -> Self {
        let mut value = Self {
            name: String::from(""),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        value.construct(name);
        value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, name: String) {
        self.name = name;
    }
    pub fn destruct(&mut self) {
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("drop-")),
            terrane_scalar_support::scalar_text(&self.name)
        );
    }
}
impl Drop for Marker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
fn release_order() {
    let mut values: terrane_collection_support::List<Marker> = terrane_collection_support::List::<
        Marker,
    >::new(
        vec![
            Marker::terrane_construct(String::from("replace-old")),
            Marker::terrane_construct(String::from("remove-me")),
            Marker::terrane_construct(String::from("destroy-last"))
        ],
    );
    __terrane_raised(
        values
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    0 /* terrane-site: case.trn:18:3-18:50 */,
                ),
                Marker::terrane_construct(String::from("replacement")),
            ),
        0 /* terrane-site: case.trn:18:3-18:50 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-set")));
    if true {
        let removed: Marker = __terrane_raised(
            values
                .remove(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(1_i128),
                        ),
                        1 /* terrane-site: case.trn:21:22-21:38 */,
                    ),
                ),
            1 /* terrane-site: case.trn:21:22-21:38 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("after-remove-")),
            terrane_scalar_support::scalar_text(&removed.name)
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-block")));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                values
                .remove(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(9_i128)),
                2 /* terrane-site: case.trn:25:5-25:21 */)), 2 /* terrane-site: case.trn:25:5-25:21 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("remove-index"))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let mut cleared: terrane_collection_support::List<Marker> = terrane_collection_support::List::<
        Marker,
    >::new(
        vec![
            Marker::terrane_construct(String::from("clear-first")),
            Marker::terrane_construct(String::from("clear-second"))
        ],
    );
    cleared.clear();
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-clear")));
    let original: terrane_collection_support::List<Marker> = terrane_collection_support::List::<
        Marker,
    >::new(vec![Marker::terrane_construct(String::from("cow-original"))]);
    let mut separated: terrane_collection_support::List<Marker> = original.clone();
    __terrane_raised(
        separated
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    3 /* terrane-site: case.trn:34:3-34:57 */,
                ),
                Marker::terrane_construct(String::from("cow-replacement")),
            ),
        3 /* terrane-site: case.trn:34:3-34:57 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(original
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:35:10-35:21 */)), 4 /* terrane-site: case.trn:35:10-35:21 */).name),
        terrane_scalar_support::scalar_text(&__terrane_raised(separated
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:35:28-35:40 */)), 5 /* terrane-site: case.trn:35:28-35:40 */).name)
    );
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-cow")));
}
fn main() {
    release_order();
    let left: std::sync::Arc<std::sync::Mutex<Marker>> = std::sync::Arc::new(
        std::sync::Mutex::new(Marker::terrane_construct(String::from("left"))),
    );
    let right: std::sync::Arc<std::sync::Mutex<Marker>> = std::sync::Arc::new(
        std::sync::Mutex::new(
            {
                let __terrane_value = left
                    .lock()
                    .expect("reference lock poisoned")
                    .clone();
                __terrane_value
            }
                .terrane_separate(),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ false }),
        terrane_scalar_support::scalar_text(&{ false })
    );
    let left_reference: std::sync::Arc<std::sync::Mutex<Marker>> = left.clone();
    let same_reference: std::sync::Arc<std::sync::Mutex<Marker>> = left_reference
        .clone();
    let other_reference: std::sync::Arc<std::sync::Mutex<Marker>> = right.clone();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_reference; let __terrane_identity_right = &same_reference;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_reference; let __terrane_identity_right = &other_reference;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    let left_weak: std::sync::Weak<std::sync::Mutex<Marker>> = std::sync::Arc::downgrade(
        &left_reference,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &left_weak; let __terrane_identity_right = &left_reference;
        std::ptr::eq(std::sync::Weak::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    let references: std::sync::Arc<
        std::sync::Mutex<
            terrane_collection_support::List<std::sync::Arc<std::sync::Mutex<Marker>>>,
        >,
    > = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<
                std::sync::Arc<std::sync::Mutex<Marker>>,
            >::new(vec![left_reference.clone(), same_reference.clone()]),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&{ false }));
    let collection_reference: std::sync::Arc<
        std::sync::Mutex<
            terrane_collection_support::List<std::sync::Arc<std::sync::Mutex<Marker>>>,
        >,
    > = references.clone();
    let collection_alias: std::sync::Arc<
        std::sync::Mutex<
            terrane_collection_support::List<std::sync::Arc<std::sync::Mutex<Marker>>>,
        >,
    > = collection_reference.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_identity_left =
        &collection_reference; let __terrane_identity_right = &collection_alias;
        std::ptr::eq(std::sync::Arc::as_ptr(__terrane_identity_left),
        std::sync::Arc::as_ptr(__terrane_identity_right)) })
    );
    let marker_type: TerraneDescriptor = {
        let _ = &{
            let __terrane_value = left.lock().expect("reference lock poisoned").clone();
            __terrane_value
        };
        TerraneDescriptor {
            identity: "/collection-identity-lifetime::marker",
            name: "marker",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[
                TerraneFieldMetadata {
                    name: "name",
                    external_name: "name",
                    defaulted: true,
                    optional: false,
                    secret: false,
                },
            ],
        }
    };
    let reference_type: TerraneDescriptor = {
        let _ = &left_reference;
        TerraneDescriptor {
            identity: "shared ref marker",
            name: "shared ref marker",
            kind: "type",
            inherently_identity_bearing: true,
            fields: &[],
        }
    };
    let collection_type: TerraneDescriptor = {
        let _ = &{
            let __terrane_value = references
                .lock()
                .expect("reference lock poisoned")
                .clone();
            __terrane_value
        };
        TerraneDescriptor {
            identity: "/core/collections::list of shared ref marker",
            name: "list of shared ref marker",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        }
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&marker_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&reference_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&collection_type.identity.to_owned())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&marker_type
        .inherently_identity_bearing)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&reference_type
        .inherently_identity_bearing)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&collection_type
        .inherently_identity_bearing)
    );
}
