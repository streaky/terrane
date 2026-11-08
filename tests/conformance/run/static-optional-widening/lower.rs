// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-optional-widening
pub static TERRANE_STATIC_HOLDER_N: std::sync::LazyLock<
    TerraneSharedCell<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(None));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    let y: i64;
    y = 4;
    {
        let __terrane_static_value: Option<terrane_int_support::Int> = Some(
            terrane_int_support::Int::from(y as i128),
        );
        *TERRANE_STATIC_HOLDER_N.lock().expect("static field lock poisoned") = __terrane_static_value;
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); { let __terrane_result = match &*
        TERRANE_STATIC_HOLDER_N.lock().expect("static field lock poisoned") {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), }; __terrane_result
        } __terrane_text }
    );
}
