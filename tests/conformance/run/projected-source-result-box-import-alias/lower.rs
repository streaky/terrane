// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: helpers/boxes.trn
// Namespace: helpers
#[derive(Clone)]
pub struct TerraneNsHelpersBoxIdentity2F68656C7065727300426F78<TerraneType54> {
    pub value: Option<TerraneType54>,
}
impl<TerraneType54> TerraneNsHelpersBoxIdentity2F68656C7065727300426F78<TerraneType54> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
    pub fn take(mut self) -> TerraneType54 {
        return self.value.take().expect("required field initialized");
    }
}
fn unbox<TerraneType54>(
    __trn_626f78: TerraneNsHelpersBoxIdentity2F68656C7065727300426F78<TerraneType54>,
) -> TerraneType54 {
    return __trn_626f78.take();
}
// Source: src/main.trn
// Namespace: app
fn main() {
    let value: terrane_query_witness::Holder;
    value = unbox(
        TerraneNsHelpersBoxIdentity2F68656C7065727300426F78::<
            Holder,
        >::terrane_construct(
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                0 /* terrane-site: src/main.trn:7:51-7:63 */,
            ),
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&value).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 1 /* terrane-site: src/main.trn:8:13-8:29 */))
    );
}
