// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-async-resource-field
pub struct Holder {
    pub input: Option<ByteReader>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { input: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = Some(stdin());
            self.input = __terrane_value;
        }
    }
    pub async fn clear(&mut self) -> bool {
        let previous: Option<ByteReader>;
        let present: bool;
        previous = std::mem::take(&mut self.input);
        present = (&previous).is_some();
        if (&previous).is_some() {
            previous.expect("semantic optional narrowing").close();
        }
        return present;
    }
    pub async fn __terrane_shared_async_clear(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> bool {
        let previous: Option<ByteReader>;
        let present: bool;
        previous = std::mem::take(
            &mut __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .input,
        );
        present = (&previous).is_some();
        if (&previous).is_some() {
            previous.expect("semantic optional narrowing").close();
        }
        return present;
    }
}
fn main() {
    __terrane_run(async move {
        let value: std::sync::Arc<TerraneSharedCell<Holder>>;
        let owner: std::sync::Arc<TerraneSharedCell<Holder>>;
        let alias: std::sync::Arc<TerraneSharedCell<Holder>>;
        value = std::sync::Arc::new(TerraneSharedCell::new(Holder::terrane_construct()));
        owner = value.clone();
        alias = owner.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = owner.clone();
            Holder::__terrane_shared_async_clear(__terrane_shared_call_handle) }). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = alias.clone();
            Holder::__terrane_shared_async_clear(__terrane_shared_call_handle) }). await)
        );
    });
}
