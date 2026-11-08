// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct F<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub number: terrane_int_support::Int,
}
impl<TerraneType54> F<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            number: terrane_int_support::Int::from(3_i128),
        }
    }
}
pub struct G {
    pub value: Option<F<Holder>>,
}
impl G {
    pub fn terrane_construct(value: F<Holder>) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: F<Holder>) {
        {
            let __terrane_value: F<Holder> = value;
            self.value = Some(__terrane_value);
        }
    }
}
fn main() {
    let mut g: G;
    let f: F<Holder>;
    g = G::terrane_construct(F::<Holder>::terrane_construct());
    f = g.value.take().expect("required field initialized");
    println!("{}", terrane_scalar_support::scalar_text(&f.number));
}
