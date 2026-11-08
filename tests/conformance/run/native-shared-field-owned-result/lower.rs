// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub struct Wrapped {
    pub inner: Option<Holder>,
}
impl Wrapped {
    pub fn terrane_construct(inner: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
    }
}
fn go(w: std::sync::Arc<TerraneSharedCell<Wrapped>>) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_1 = w.clone();
        let __terrane_call_result = __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_owner_1.lock()
        .expect("shared reference lock poisoned").inner.as_mut()
        .expect("required field initialized").label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 0 /* terrane-site: src/main.trn:12:13-12:31 */);
        __terrane_call_result })
    );
}
fn main() {
    let w: std::sync::Arc<TerraneSharedCell<Wrapped>>;
    w = std::sync::Arc::new(
        TerraneSharedCell::new(
            Wrapped::terrane_construct(
                __terrane_raised(
                    terrane_static_trn_486f6c646572_new(),
                    1 /* terrane-site: src/main.trn:15:28-15:40 */,
                ),
            ),
        ),
    );
    go(w.clone());
}
