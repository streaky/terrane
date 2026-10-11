// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: authored-generic-contracts
#[derive(Clone)]
pub struct Envelope<TerraneType54> {
    pub payload: Option<TerraneType54>,
}
impl<TerraneType54> Envelope<TerraneType54> {
    pub fn terrane_construct(payload: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self { payload: None };
        __terrane_constructed_value.construct(payload);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, payload: TerraneType54) {
        self.payload = Some(payload);
    }
}
#[derive(Clone)]
pub struct Slot<TerraneType54> {
    pub value: Option<TerraneType54>,
}
impl<TerraneType54> Slot<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self { value: None }
    }
    pub fn set(&mut self, value: TerraneType54) {
        self.value = Some(value);
    }
}
fn identity<TerraneType54>(value: TerraneType54) -> TerraneType54 {
    return value;
}
fn choose<TerraneType54>(
    first: TerraneType54,
    second: TerraneType54,
    use_first: bool,
) -> TerraneType54 {
    if use_first {
        return first;
    }
    return second;
}
fn apply<TerraneType54, TerraneType55>(
    value: TerraneType54,
    transform: std::sync::Arc<dyn Fn(TerraneType54) -> TerraneType55 + Send + Sync>,
) -> TerraneType55 {
    return transform(value);
}
fn label(value: terrane_int_support::Int) -> String {
    if value.clone() == terrane_int_support::Int::from(41_i128) {
        return String::from("forty-one");
    }
    return String::from("other");
}
fn main() {
    let number_envelope: Envelope<terrane_int_support::Int>;
    let text_envelope: Envelope<String>;
    let selected: terrane_int_support::Int;
    let transformed: String;
    let keep_int: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let mut pending: Slot<String>;
    let value: Option<String>;
    number_envelope = Envelope::<
        terrane_int_support::Int,
    >::terrane_construct(terrane_int_support::Int::from(41_i128));
    text_envelope = Envelope::<String>::terrane_construct(String::from("hello"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&number_envelope.payload.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&text_envelope.payload.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&identity:: < terrane_int_support::Int
        > (terrane_int_support::Int::from(42_i128)))
    );
    selected = choose(
        terrane_int_support::Int::from(11_i128),
        terrane_int_support::Int::from(23_i128),
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&selected));
    transformed = apply(
        terrane_int_support::Int::from(41_i128),
        std::sync::Arc::new(label),
    );
    println!("{}", terrane_scalar_support::scalar_text(&transformed));
    keep_int = std::sync::Arc::new(identity::<terrane_int_support::Int>);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&keep_int(terrane_int_support::Int::from(44_i128)))
    );
    pending = Slot::<String>::terrane_construct();
    pending.set(String::from("ready"));
    value = pending.value;
    if value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
