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
#[derive(Clone)]
pub struct Factory<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub __trn_626f78: Option<TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54>>,
}
impl<TerraneType54> Factory<TerraneType54> {
    pub fn terrane_construct(
        __trn_626f78: TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            __trn_626f78: None,
        };
        __terrane_constructed_value.construct(__trn_626f78);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        __trn_626f78: TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54>,
    ) {
        {
            let __terrane_value = __trn_626f78;
            self.__trn_626f78 = Some(__terrane_value);
        }
    }
    pub fn mk(mut self) -> TerraneNsAppBoxIdentity2F61707000426F78<TerraneType54> {
        return self.__trn_626f78.take().expect("required field initialized");
    }
}
fn main() {
    let factory: Factory<Holder>;
    let alias: Factory<Holder>;
    let produced: TerraneNsAppBoxIdentity2F61707000426F78<Holder>;
    let value: terrane_query_witness::Holder;
    factory = Factory::<
        Holder,
    >::terrane_construct(
        TerraneNsAppBoxIdentity2F61707000426F78::<
            Holder,
        >::terrane_construct(
            __terrane_raised(
                terrane_static_trn_486f6c646572_new(),
                0 /* terrane-site: src/main.trn:23:67-23:79 */,
            ),
        ),
    );
    alias = factory;
    produced = alias.mk();
    value = unbox(produced);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&value).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 1 /* terrane-site: src/main.trn:27:11-27:27 */))
    );
}
