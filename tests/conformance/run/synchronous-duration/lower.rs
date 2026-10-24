// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: synchronous-duration
fn main() {
    let d: Duration;
    d = __terrane_traced(
        Duration::terrane_static_seconds(terrane_int_support::Int::from(2_i128)),
        0 /* terrane-site: case.trn:5:9-5:29 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&d.total_nanoseconds));
}
