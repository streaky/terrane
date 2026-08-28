// Generated deterministically by Terrane <version>.
// Source: src/main.trn
// Namespace: app
fn main() {
    let response: Response = get(String::from("http://127.0.0.1:38125/"));
    let body: String = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| response.text()),
        )
        .unwrap_or_else(|payload| std::panic::resume_unwind(payload))
        .unwrap_or_else(|error| panic!("Rust dependency call failed: {error}"));
    println!("{}", terrane_scalar_support::scalar_text(&body));
}
// Source: <terrane>/projected//dependencies/reqwest/blocking.trn
// Namespace: dependencies/reqwest/blocking
pub type Response = reqwest::blocking::Response;
pub fn get(url: String) -> Response {
    let crossed = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| reqwest::blocking::get(url)),
        )
        .unwrap_or_else(|payload| std::panic::resume_unwind(payload));
    crossed.unwrap_or_else(|error| panic!("Rust dependency call failed: {error}"))
}
