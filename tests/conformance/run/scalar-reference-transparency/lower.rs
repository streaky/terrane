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
// Source: case.trn
// Namespace: scalar-reference-transparency
fn main() {
    let text: std::sync::Arc<std::sync::Mutex<String>> = std::sync::Arc::new(
        std::sync::Mutex::new(String::from("abc")),
    );
    let seen: std::sync::Weak<std::sync::Mutex<String>> = std::sync::Arc::downgrade(
        &text,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(& (terrane_string_support::length(& ({
        let __terrane_owner = seen.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value })) as i128))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(& (({ let __terrane_owner = seen
        .upgrade().expect("reference expired"); let __terrane_value = __terrane_owner
        .lock().expect("reference lock poisoned").clone(); __terrane_value })))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(& (format!("{}{}",
        terrane_scalar_support::scalar_text(& (({ let __terrane_owner = seen.upgrade()
        .expect("reference expired"); let __terrane_value = __terrane_owner.lock()
        .expect("reference lock poisoned").clone(); __terrane_value }))),
        terrane_scalar_support::scalar_text(& (String::from("!"))))))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&
        (vec![terrane_scalar_support::scalar_text(& (String::from("x"))),
        terrane_scalar_support::scalar_text(& (String::from("y")))] .join(& (({ let
        __terrane_owner = seen.upgrade().expect("reference expired"); let __terrane_value
        = __terrane_owner.lock().expect("reference lock poisoned").clone();
        __terrane_value })))))
    );
    let encoded: std::sync::Arc<std::sync::Mutex<Vec<u8>>> = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_string_support::encode(
                &(({
                    let __terrane_value = text
                        .lock()
                        .expect("reference lock poisoned")
                        .clone();
                    __terrane_value
                })),
                terrane_string_support::Encoding::Utf8,
            ),
        ),
    );
    let decoded: std::sync::Weak<std::sync::Mutex<Vec<u8>>> = std::sync::Arc::downgrade(
        &encoded,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(& ((terrane_string_support::decode(&
        (({ let __terrane_owner = decoded.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value })), terrane_string_support::Encoding::Utf8))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/scalar-reference-transparency::main (case.trn:14:13)")))))
    );
    let number: std::sync::Arc<std::sync::Mutex<i8>> = std::sync::Arc::new(
        std::sync::Mutex::new(7),
    );
    let observed: std::sync::Weak<std::sync::Mutex<i8>> = std::sync::Arc::downgrade(
        &number,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(& (({ let __terrane_owner = observed
        .upgrade().expect("reference expired"); let __terrane_value = __terrane_owner
        .lock().expect("reference lock poisoned").clone(); __terrane_value })))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(& (terrane_int_support::Int::from((({
        let __terrane_owner = observed.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value })) as i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&
        ((terrane_int_support::fixed_addition({ let __terrane_owner = observed.upgrade()
        .expect("reference expired"); let __terrane_value = __terrane_owner.lock()
        .expect("reference lock poisoned").clone(); __terrane_value }, 2))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/scalar-reference-transparency::main (case.trn:19:13)")))))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&
        ((terrane_int_support::fixed_addition({ let __terrane_owner = observed.upgrade()
        .expect("reference expired"); let __terrane_value = __terrane_owner.lock()
        .expect("reference lock poisoned").clone(); __terrane_value }, 3))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/scalar-reference-transparency::main (case.trn:20:12)")))))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(& (- ({ let __terrane_owner = observed
        .upgrade().expect("reference expired"); let __terrane_value = __terrane_owner
        .lock().expect("reference lock poisoned").clone(); __terrane_value })))
    );
    let owner: std::sync::Arc<std::sync::Mutex<i8>> = (number).clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(& (({ let __terrane_value = owner
        .lock().expect("shared reference lock poisoned").clone(); __terrane_value })))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&
        ((terrane_int_support::fixed_multiplication({ let __terrane_value = owner.lock()
        .expect("shared reference lock poisoned").clone(); __terrane_value }, 2))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/scalar-reference-transparency::main (case.trn:24:12)")))))
    );
}
