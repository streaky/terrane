// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Unrelated {}
impl Unrelated {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn poison(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
}
fn main() {
    let local: Unrelated;
    let left: Left;
    local = Unrelated::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&local.poison()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(poison(),
        0 /* terrane-site: src/main.trn:13:11-13:18 */))
    );
    left = __terrane_raised(
        terrane_static_trn_4c656674_new(),
        1 /* terrane-site: src/main.trn:14:10-14:20 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&left).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload,
        "terrane_member_graph_witness",
        "terrane_member_graph_witness::left::Left::value")) }, 2 /* terrane-site: src/main.trn:15:11-15:22 */))
    );
}
