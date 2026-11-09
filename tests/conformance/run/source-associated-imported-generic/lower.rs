// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TextRequest {}
impl TextRequest {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(&self, input: String) -> terrane_int_support::Int {
        let _ = &input;
        return terrane_int_support::Int::from(42_i128);
    }
}
impl RequestProtocol<String, terrane_int_support::Int> for TextRequest {
    fn clone_box(&self) -> Box<dyn RequestProtocol<String, terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<dyn RequestProtocol<String, terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, input: String) -> terrane_int_support::Int {
        TextRequest::dispatch(&*self, input)
    }
}
impl From<TextRequest> for Request<String, terrane_int_support::Int> {
    fn from(value: TextRequest) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let value: Request<String, terrane_int_support::Int>;
    value = <Request<
        String,
        terrane_int_support::Int,
    >>::from(TextRequest::terrane_construct());
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.dispatch(String::from("input")))
    );
}
// Source: contract/request.trn
// Namespace: contract
pub trait RequestProtocol<
    TerraneType496E707574,
    TerraneType726573706F6E7365,
>: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<dyn RequestProtocol<TerraneType496E707574, TerraneType726573706F6E7365>>;
    fn separate_box(
        &self,
    ) -> Box<dyn RequestProtocol<TerraneType496E707574, TerraneType726573706F6E7365>>;
    fn dispatch(&self, input: TerraneType496E707574) -> TerraneType726573706F6E7365;
}
impl<TerraneType496E707574, TerraneType726573706F6E7365> Clone
for Box<dyn RequestProtocol<TerraneType496E707574, TerraneType726573706F6E7365>> {
    fn clone(&self) -> Self {
        RequestProtocol::clone_box(&**self)
    }
}
pub struct Request<TerraneType496E707574, TerraneType726573706F6E7365>(
    Box<dyn RequestProtocol<TerraneType496E707574, TerraneType726573706F6E7365>>,
);
impl<TerraneType496E707574, TerraneType726573706F6E7365> Clone
for Request<TerraneType496E707574, TerraneType726573706F6E7365> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneType496E707574,
    TerraneType726573706F6E7365,
> Request<TerraneType496E707574, TerraneType726573706F6E7365> {
    pub fn dispatch(&self, input: TerraneType496E707574) -> TerraneType726573706F6E7365 {
        self.0.dispatch(input)
    }
}
