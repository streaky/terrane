// Generated deterministically by Terrane <version>.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    Custom(&'static str),
}
impl TerraneErrorKind {
    fn from_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
            Self::Custom(name) => name,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string().trim_start_matches(".decode-error: "),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
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
#[allow(
    dead_code,
    reason = "projected type methods may be imported without being crossed"
)]
fn __terrane_dependency_panic(
    payload: Box<dyn std::any::Any + Send>,
    crate_name: &'static str,
    member: &'static str,
) -> TerraneError {
    let detail = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("non-string panic payload");
    TerraneError::new(
        TerraneErrorKind::Custom("dependency-panic"),
        format!("Rust dependency `{crate_name}` member `{member}` panicked: {detail}"),
    )
}
// Source: src/main.trn
// Namespace: app
fn main() {
    let buffer: BytesMut = with_capacity(terrane_int_support::Int::from(8_i128))
        .unwrap_or_else(|error| __terrane_uncaught(
            error.at("/app::main (main.trn:8:23)"),
        ));
    let remaining: terrane_int_support::Int = remaining_mut(&buffer)
        .unwrap_or_else(|error| __terrane_uncaught(
            error.at("/app::main (main.trn:9:21)"),
        ));
    let candidate: Option<Number> = from_u128(terrane_int_support::Int::from(42_i128))
        .unwrap_or_else(|error| __terrane_uncaught(
            error.at("/app::main (main.trn:10:29)"),
        ));
    let data: Category = __trn_44617461()
        .unwrap_or_else(|error| __terrane_uncaught(
            error.at("/app::main (main.trn:11:21)"),
        ));
    let io: Category = __trn_496f()
        .unwrap_or_else(|error| __terrane_uncaught(
            error.at("/app::main (main.trn:12:19)"),
        ));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(remaining.clone() >
        terrane_int_support::Int::from(0_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&candidate.is_some()));
    println!("{}", terrane_scalar_support::scalar_text(&(data != io)));
}
// Source: <terrane>/projected/deps/bytes/bufmut.trn
// Namespace: deps/bytes/bufmut
pub fn remaining_mut(
    receiver: &BytesMut,
) -> Result<terrane_int_support::Int, crate::TerraneError> {
    match std::panic::catch_unwind(|| <bytes::BytesMut as bytes::BufMut>::remaining_mut(
        receiver,
    )) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "bytes",
                    "<bytes::BytesMut as bytes::BufMut>::remaining_mut",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/bytes/bytes.trn
// Namespace: deps/bytes/bytes
pub use bytes::Bytes;
// Source: <terrane>/projected/deps/bytes/bytes-mut.trn
// Namespace: deps/bytes/bytes-mut
pub use bytes::BytesMut;
pub fn with_capacity(
    capacity: terrane_int_support::Int,
) -> Result<BytesMut, crate::TerraneError> {
    let capacity = terrane_int_support::coerce::<usize>(&capacity)
        .map_err(crate::TerraneError::from)?;
    match std::panic::catch_unwind(|| bytes::BytesMut::with_capacity(capacity)) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "bytes",
                    "bytes::BytesMut::with_capacity",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/serde-json/error.trn
// Namespace: deps/serde-json/error
pub use serde_json::error::Category;
// Source: <terrane>/projected/deps/serde-json/error/category.trn
// Namespace: deps/serde-json/error/category
/// Projected enum variant constructor for `serde_json::error::Category::Data`.
pub fn __trn_44617461() -> Result<Category, crate::TerraneError> {
    match std::panic::catch_unwind(|| serde_json::error::Category::Data) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::error::Category::Data",
                ),
            )
        }
    }
}
/// Projected enum variant constructor for `serde_json::error::Category::Io`.
pub fn __trn_496f() -> Result<Category, crate::TerraneError> {
    match std::panic::catch_unwind(|| serde_json::error::Category::Io) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::error::Category::Io",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/serde-json/number.trn
// Namespace: deps/serde-json/number
pub use serde_json::Number;
pub fn from_u128(
    i: terrane_int_support::Int,
) -> Result<Option<Number>, crate::TerraneError> {
    let i = terrane_int_support::coerce::<u128>(&i).map_err(crate::TerraneError::from)?;
    match std::panic::catch_unwind(|| serde_json::Number::from_u128(i)) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "serde-json",
                    "serde_json::Number::from_u128",
                ),
            )
        }
    }
}
