// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<TerraneType54> TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
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
    __trn_626f78: TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54>,
) -> TerraneType54 {
    return __trn_626f78.take();
}
pub struct Maker {
    pub __trn_626f78: Option<TerraneNsAppBoxIdentity2F61707000426F78<Holder>>,
}
impl Maker {
    pub fn terrane_construct(
        __trn_626f78: TerraneNsAppBoxIdentity2F61707000426F78<Holder>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { __trn_626f78: None };
        __terrane_constructed_value.construct(__trn_626f78);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        __trn_626f78: TerraneNsAppBoxIdentity2F61707000426F78<Holder>,
    ) {
        {
            let __terrane_value = __trn_626f78;
            self.__trn_626f78 = Some(__terrane_value);
        }
    }
    pub fn extract(mut self) {
        let value: terrane_query_witness::Holder;
        value = unbox(self.__trn_626f78.take().expect("required field initialized"));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&value).label_len()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 0 /* terrane-site: src/main.trn:21:13-21:29 */))
        );
    }
}
fn main() {
    let maker: Maker;
    maker = Maker::terrane_construct(
        TerraneNsAppBoxIdentity2F61707000426F78::<
            Holder,
        >::terrane_construct(
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                1 /* terrane-site: src/main.trn:23:53-23:65 */,
            ),
        ),
    );
    maker.extract();
}
