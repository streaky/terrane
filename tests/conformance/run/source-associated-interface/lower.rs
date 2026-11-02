// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: source-associated-interface
pub trait RequestProtocol<TerraneType726573706F6E7365>: Send + Sync {
    fn clone_box(&self) -> Box<dyn RequestProtocol<TerraneType726573706F6E7365>>;
    fn separate_box(&self) -> Box<dyn RequestProtocol<TerraneType726573706F6E7365>>;
    fn dispatch(&self, input: String) -> TerraneType726573706F6E7365;
}
impl<TerraneType726573706F6E7365> Clone
for Box<dyn RequestProtocol<TerraneType726573706F6E7365>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Request<TerraneType726573706F6E7365>(
    Box<dyn RequestProtocol<TerraneType726573706F6E7365>>,
);
impl<TerraneType726573706F6E7365> Clone for Request<TerraneType726573706F6E7365> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType726573706F6E7365> Request<TerraneType726573706F6E7365> {
    pub fn dispatch(&self, input: String) -> TerraneType726573706F6E7365 {
        self.0.dispatch(input)
    }
}
#[derive(Clone)]
pub struct NumericRequest {
    pub __trn_74797065: terrane_int_support::Int,
}
impl NumericRequest {
    pub fn terrane_construct() -> Self {
        Self {
            __trn_74797065: terrane_int_support::Int::from(17_i128),
        }
    }
    pub fn dispatch(&self, input: String) -> terrane_int_support::Int {
        let _ = &input;
        return terrane_int_support::Int::from(40_i128);
    }
}
impl RequestProtocol<terrane_int_support::Int> for NumericRequest {
    fn clone_box(&self) -> Box<dyn RequestProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RequestProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, input: String) -> terrane_int_support::Int {
        NumericRequest::dispatch(&*self, input)
    }
}
impl From<NumericRequest> for Request<terrane_int_support::Int> {
    fn from(value: NumericRequest) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TextRequest {}
impl TextRequest {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(&self, input: String) -> String {
        return input;
    }
}
impl RequestProtocol<String> for TextRequest {
    fn clone_box(&self) -> Box<dyn RequestProtocol<String>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RequestProtocol<String>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, input: String) -> String {
        TextRequest::dispatch(&*self, input)
    }
}
impl From<TextRequest> for Request<String> {
    fn from(value: TextRequest) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let ordinary: NumericRequest;
    let numeric: Request<terrane_int_support::Int>;
    let count: terrane_int_support::Int;
    let textual: Request<String>;
    let message: String;
    ordinary = NumericRequest::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&ordinary.__trn_74797065));
    numeric = <Request<
        terrane_int_support::Int,
    >>::from(NumericRequest::terrane_construct());
    count = numeric.dispatch(String::from("ignored"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(count.clone() +
        terrane_int_support::Int::from(2_i128)))
    );
    textual = <Request<String>>::from(TextRequest::terrane_construct());
    message = textual.dispatch(String::from("distinct"));
    println!("{}", terrane_scalar_support::scalar_text(&message));
}
