// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54> {
    pub value: Option<TerraneType54>,
}
impl<TerraneType54> TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54> {
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
#[derive(Clone)]
pub struct Factory {}
impl Factory {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn both<TerraneType42>(
        &self,
        value: TerraneType42,
    ) -> TerraneNsAppBoxIdentity2F61707000426F78<TerraneType42> {
        return TerraneNsAppBoxIdentity2F61707000426F78::<
            TerraneType42,
        >::terrane_construct(value);
    }
    pub fn direct<TerraneType42>(&self, value: TerraneType42) -> TerraneType42 {
        return value;
    }
    pub fn terrane_static_static_both<TerraneType42>(
        value: TerraneType42,
    ) -> TerraneNsAppBoxIdentity2F61707000426F78<TerraneType42> {
        return TerraneNsAppBoxIdentity2F61707000426F78::<
            TerraneType42,
        >::terrane_construct(value);
    }
}
#[allow(dead_code)]
fn free_both<TerraneType42>(
    value: TerraneType42,
) -> TerraneNsAppBoxIdentity2F61707000426F78<TerraneType42> {
    return TerraneNsAppBoxIdentity2F61707000426F78::<
        TerraneType42,
    >::terrane_construct(value);
}
fn unbox<TerraneType54>(
    __trn_626f78: TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54>,
) -> TerraneType54 {
    return __trn_626f78.take();
}
fn main() {
    let factory: Factory;
    let __trn_626f78: TerraneNsAppBoxIdentity2F61707000426F78<Holder>;
    let value: terrane_query_witness::Holder;
    factory = Factory::terrane_construct();
    __trn_626f78 = factory
        .both(
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                0 /* terrane-site: src/main.trn:22:40-22:52 */,
            ),
        );
    value = unbox(__trn_626f78);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&value).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 1 /* terrane-site: src/main.trn:24:13-24:29 */))
    );
}
