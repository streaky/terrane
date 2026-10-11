// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: authored-generic-class-phantom-parameter
#[derive(Clone)]
pub struct Holder<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub count: terrane_int_support::Int,
}
impl<TerraneType54> Holder<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub struct Pair<TerraneType54, TerraneType55> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54, TerraneType55)>,
    pub count: terrane_int_support::Int,
}
impl<TerraneType54, TerraneType55> Pair<TerraneType54, TerraneType55> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
fn main() {
    let mut text_holder: Holder<String>;
    let number_holder: Holder<terrane_int_support::Int>;
    let value: Pair<String, terrane_int_support::Int>;
    text_holder = Holder::<String>::terrane_construct();
    number_holder = Holder::<terrane_int_support::Int>::terrane_construct();
    text_holder.count = terrane_int_support::Int::from(7_i128);
    println!("{}", terrane_scalar_support::scalar_text(&text_holder.count));
    println!("{}", terrane_scalar_support::scalar_text(&number_holder.count));
    value = Pair::<String, terrane_int_support::Int>::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.count));
}
