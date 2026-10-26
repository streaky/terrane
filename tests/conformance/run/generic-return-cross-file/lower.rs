// Generated deterministically by Terrane <version>.
// Source: src/main.trn
// Namespace: generic-return-cross-file
#[derive(Clone)]
pub struct Slot<TerraneType54> {
    pub value: Option<TerraneType54>,
}
impl<TerraneType54> Slot<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self { value: None }
    }
}
fn make() -> Slot<String> {
    return Slot::<String>::terrane_construct();
}
fn main() {
    let mut made: Slot<String>;
    let value: Option<String>;
    made = make();
    {
        let __terrane_value = Some(String::from("ready"));
        made.value = __terrane_value;
    }
    value = made.value;
    if (&value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
// Source: src/other.trn
// Namespace: generic-return-cross-file
#[allow(dead_code)]
fn other() -> Slot<terrane_int_support::Int> {
    return Slot::<terrane_int_support::Int>::terrane_construct();
}
