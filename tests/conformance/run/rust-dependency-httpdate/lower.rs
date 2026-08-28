// Generated deterministically by Terrane <version>.
// Source: src/main.trn
// Namespace: app
fn main() {
    let moment: SystemTime = __trn_70617273655f687474705f64617465(
        String::from("Sun, 06 Nov 1994 08:49:37 GMT"),
    );
    let rendered: String = __trn_666d745f687474705f64617465(moment);
    println!("{}", terrane_scalar_support::scalar_text(&rendered));
}
// Source: <terrane>/projected//dependencies/httpdate.trn
// Namespace: dependencies/httpdate
pub type SystemTime = std::time::SystemTime;
pub fn __trn_666d745f687474705f64617465(d: SystemTime) -> String {
    let crossed = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| httpdate::fmt_http_date(d)),
        )
        .unwrap_or_else(|payload| std::panic::resume_unwind(payload));
    crossed
}
pub fn __trn_70617273655f687474705f64617465(s: String) -> SystemTime {
    let crossed = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| httpdate::parse_http_date(&s)),
        )
        .unwrap_or_else(|payload| std::panic::resume_unwind(payload));
    crossed.unwrap_or_else(|error| panic!("Rust dependency call failed: {error}"))
}
