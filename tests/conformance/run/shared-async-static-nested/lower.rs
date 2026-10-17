// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-async-static-nested
#[derive(Clone)]
pub struct Counter<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub total: terrane_int_support::Int,
}
impl<TerraneType54> Counter<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone()
                + terrane_int_support::Int::from(1_i128);
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
    pub async fn inner(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone() + delta.clone();
            self.total = __terrane_value;
        }
        return self.increment();
    }
    pub async fn __terrane_shared_async_inner(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .total
                    .clone();
                __terrane_snapshot
            } + delta.clone();
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total = __terrane_value;
        }
        return {
            let __terrane_call_result = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .increment();
            __terrane_call_result
        };
    }
    pub async fn outer(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return __terrane_await(self.inner(delta.clone())).await;
    }
    pub async fn __terrane_shared_async_outer(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return __terrane_await({
                let __terrane_shared_call_handle = __terrane_shared_handle.clone();
                let __terrane_shared_argument_0 = delta.clone();
                let __terrane_shared_argument_1 = __terrane_shared_argument_0;
                Counter::<
                    TerraneType54,
                >::__terrane_shared_async_inner(
                    __terrane_shared_call_handle,
                    __terrane_shared_argument_1,
                )
            })
            .await;
    }
}
pub static TERRANE_STATIC_HOLDER_SLOT: std::sync::LazyLock<
    TerraneSharedCell<Counter<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    Counter::<terrane_int_support::Int>::terrane_construct(),
));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    __terrane_run(async move {
        let value: std::sync::Arc<TerraneSharedCell<Counter<terrane_int_support::Int>>>;
        let owner: std::sync::Arc<TerraneSharedCell<Counter<terrane_int_support::Int>>>;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = &* TERRANE_STATIC_HOLDER_SLOT; let
            __terrane_shared_argument_0 = terrane_int_support::Int::from(5_i128); let
            __terrane_shared_argument_1 = __terrane_shared_argument_0; Counter:: <
            terrane_int_support::Int >
            ::__terrane_shared_async_outer(__terrane_shared_call_handle,
            __terrane_shared_argument_1) }). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
            __terrane_result = TERRANE_STATIC_HOLDER_SLOT.lock()
            .expect("static field lock poisoned").total.clone(); __terrane_result };
            __terrane_snapshot })
        );
        value = std::sync::Arc::new(
            TerraneSharedCell::new(
                Counter::<terrane_int_support::Int>::terrane_construct(),
            ),
        );
        owner = value.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = owner.clone(); let __terrane_shared_argument_0
            = terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
            __terrane_shared_argument_0; Counter:: < terrane_int_support::Int >
            ::__terrane_shared_async_outer(__terrane_shared_call_handle,
            __terrane_shared_argument_1) }). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = owner
            .lock().expect("shared reference lock poisoned").total.clone();
            __terrane_snapshot })
        );
    });
}
