// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-consuming-field-support
pub struct Holder {
    pub callback: Option<TerraneConsumingCallable<(), terrane_int_support::Int>>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { callback: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value: Option<
                TerraneConsumingCallable<(), terrane_int_support::Int>,
            > = None;
            self.callback = __terrane_value;
        }
    }
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&0));
}
