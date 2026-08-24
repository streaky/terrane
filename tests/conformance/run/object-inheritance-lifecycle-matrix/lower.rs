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
#[derive(Clone, Debug)]
struct TerraneError {
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
// Namespace: object-inheritance-lifecycle-matrix
pub trait NamedProtocol {
    fn clone_box(&self) -> Box<dyn NamedProtocol>;
    fn separate_box(&self) -> Box<dyn NamedProtocol>;
    fn report(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn NamedProtocol> { fn clone(&self) -> Self { self.clone_box() } }
#[derive(Clone)]
pub struct Named(Box<dyn NamedProtocol>);
impl Named {
    pub fn report(&self) -> terrane_int_support::Int {
        self.0.report()
    }
    fn terrane_separate(&self) -> Self { Self(self.0.separate_box()) }
}
#[derive(Clone)]
pub struct BaseStorage {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn report(&self) -> terrane_int_support::Int {
        return (self.value).clone();
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        self.value = value.clone();
    }
    pub fn destruct(&self) {
        println!("{}", terrane_scalar_support::scalar_text(&(String::from("base-destruct"))));
    }
}
#[derive(Clone)]
pub enum Base {
    Own(BaseStorage),
    Child(Child),
}
impl Base {
    pub fn terrane_construct() -> Self { Self::Own(BaseStorage::terrane_construct()) }
    pub fn terrane_separate(&self) -> Self {
        match self {
            Self::Own(value) => Self::Own(value.terrane_separate()),
            Self::Child(value) => Self::Child(value.terrane_separate()),
        }
    }
    pub fn report(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.report(),
            Self::Child(value) => value.report(),
        }
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        match self {
            Self::Own(value_) => value_.set(value),
            Self::Child(value_) => value_.set(value),
        }
    }
    pub fn terrane_field_value(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.value,
            Self::Child(value) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.value,
            Self::Child(value) => &mut value.value,
        }
    }
}
impl NamedProtocol for Base {
    fn clone_box(&self) -> Box<dyn NamedProtocol> { Box::new(self.clone()) }
    fn separate_box(&self) -> Box<dyn NamedProtocol> { Box::new(self.terrane_separate()) }
    fn report(&self) -> terrane_int_support::Int {
        Base::report(self, )
    }
}
impl From<Base> for Named { fn from(value: Base) -> Self { Self(Box::new(value)) } }
impl Drop for BaseStorage {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
#[derive(Clone)]
pub struct Child {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
    pub extra: terrane_int_support::Int,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            extra: terrane_int_support::Int::from(2_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn report(&self) -> terrane_int_support::Int {
        return (self.value).clone();
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        self.value = value.clone();
    }
    pub fn destruct(&self) {
        println!("{}", terrane_scalar_support::scalar_text(&(String::from("child-destruct"))));
    }
    fn terrane_destruct_0(&self) {
        println!("{}", terrane_scalar_support::scalar_text(&(String::from("base-destruct"))));
    }
}
impl NamedProtocol for Child {
    fn clone_box(&self) -> Box<dyn NamedProtocol> { Box::new(self.clone()) }
    fn separate_box(&self) -> Box<dyn NamedProtocol> { Box::new(self.terrane_separate()) }
    fn report(&self) -> terrane_int_support::Int {
        Child::report(self, )
    }
}
impl From<Child> for Named { fn from(value: Child) -> Self { Self(Box::new(value)) } }
impl Drop for Child {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
            self.terrane_destruct_0();
        }
    }
}
fn main() {
    let mut value: Child = Child::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&(value.report())));
    value.set(terrane_int_support::Int::from(4_i128));
    println!("{}", terrane_scalar_support::scalar_text(&(value.report())));
    let view: Named = Named::from((value).terrane_separate());
    let copied: Named = (view).terrane_separate();
    println!("{}", terrane_scalar_support::scalar_text(&(copied.report())));
}
