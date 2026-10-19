// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: authored-generic-selection
#[derive(Clone)]
pub struct Slot<TerraneType54> {
    pub value: Option<TerraneType54>,
}
impl<TerraneType54> Slot<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self { value: None }
    }
}
fn identity<TerraneType54>(value: TerraneType54) -> TerraneType54 {
    return value;
}
fn absent<TerraneType54>() -> Option<TerraneType54> {
    return None;
}
fn apply<TerraneType54, TerraneType55>(
    value: TerraneType54,
    transform: std::sync::Arc<dyn Fn(TerraneType54) -> TerraneType55 + Send + Sync>,
) -> TerraneType55 {
    return transform(value);
}
fn render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("mapped");
}
#[derive(Clone)]
pub enum Pair {
    Values(terrane_int_support::Int, terrane_int_support::Int),
}
fn first() -> terrane_int_support::Int {
    println!("{}", String::from("first"));
    return terrane_int_support::Int::from(11_i128);
}
fn second() -> terrane_int_support::Int {
    println!("{}", String::from("second"));
    return terrane_int_support::Int::from(23_i128);
}
fn main() {
    let empty: Slot<String>;
    let missing: Option<terrane_int_support::Int>;
    let keep: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let state: Pair;
    let left: terrane_int_support::Int;
    let right: terrane_int_support::Int;
    empty = Slot::<String>::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&(&empty.value).is_none()));
    missing = absent();
    println!("{}", terrane_scalar_support::scalar_text(&(&missing).is_none()));
    keep = std::sync::Arc::new(identity);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&keep(terrane_int_support::Int::from(44_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&apply(terrane_int_support::Int::from(4_i128),
        std::sync::Arc::new(render)))
    );
    state = {
        let __terrane_enum_payload_0 = second();
        let __terrane_enum_payload_1 = first();
        Pair::Values(__terrane_enum_payload_1, __terrane_enum_payload_0)
    };
    let __terrane_match_value_813 = state.clone();
    match __terrane_match_value_813 {
        Pair::Values(__terrane_pattern_833_left, __terrane_pattern_833_right) => {
            left = __terrane_pattern_833_left;
            right = __terrane_pattern_833_right;
            println!(
                "{}", terrane_scalar_support::scalar_text(&(left.clone() + right
                .clone()))
            );
        }
    }
}
