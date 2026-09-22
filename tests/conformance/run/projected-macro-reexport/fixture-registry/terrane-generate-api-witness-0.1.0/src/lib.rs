use proc_macro::TokenStream;

#[proc_macro_derive(GeneratedValueApi)]
pub fn generated_value_api(_input: TokenStream) -> TokenStream {
    "impl GeneratedValue { pub fn standard() -> Self { Self { value: 42 } } pub fn value(&self) -> i64 { self.value } }"
        .parse()
        .expect("fixed generated implementation is valid Rust")
}
