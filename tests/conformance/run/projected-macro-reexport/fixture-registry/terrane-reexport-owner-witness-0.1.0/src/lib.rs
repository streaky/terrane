#[doc(hidden)]
mod generated {
    use terrane_generate_api_witness::GeneratedValueApi;

    #[derive(GeneratedValueApi)]
    pub struct GeneratedValue {
        value: i64,
    }
}

pub use generated::GeneratedValue;
