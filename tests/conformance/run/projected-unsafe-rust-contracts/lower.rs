// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn view<'view>(state: &'view String) -> witness::ScopedView<'view> {
    return __terrane_raised(
        borrowed_view(state),
        0 /* terrane-site: src/main.trn:6:12-6:32 */,
    );
}
#[allow(unsafe_code)]
fn main() {
    let state: String;
    state = String::from("unsafe");
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(unsafe {
        unchecked_add(terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(22_i128)) }, 1 /* terrane-site: src/main.trn:10:13-10:41 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_unsafe(&state, view))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "dependency",
        "dependency::ForeignType::render_unsafe")) }, 2 /* terrane-site: src/main.trn:11:13-11:39 */))
    );
}
