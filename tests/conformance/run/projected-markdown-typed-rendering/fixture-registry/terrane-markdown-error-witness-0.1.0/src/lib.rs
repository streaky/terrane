#[must_use]
pub fn mdx_options() -> markdown::Options {
    markdown::Options {
        parse: markdown::ParseOptions::mdx(),
        ..markdown::Options::default()
    }
}
