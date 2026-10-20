// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(4_i128),
        }
    }
    pub async fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub async fn __terrane_shared_async_read(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .value
                .clone();
            __terrane_snapshot
        };
    }
    pub async fn bump(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
    pub async fn __terrane_shared_async_bump(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .value
                    .clone();
                __terrane_snapshot
            } + terrane_int_support::Int::from(1_i128);
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .value = __terrane_value;
        }
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .value
                .clone();
            __terrane_snapshot
        };
    }
}
#[derive(Clone)]
pub struct SnapshotOuter {
    pub inner: Option<Holder>,
}
impl SnapshotOuter {
    pub fn terrane_construct(inner: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder) {
        {
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
    }
    pub async fn run(&self) -> terrane_int_support::Int {
        let snapshot: Holder;
        snapshot = self.inner.as_ref().expect("required field initialized").clone();
        return __terrane_await((&snapshot).read()).await;
    }
    pub async fn __terrane_shared_async_run(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        let snapshot: Holder;
        snapshot = {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .inner
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        };
        return __terrane_await((&snapshot).read()).await;
    }
}
#[derive(Clone)]
pub struct SharedOuter {
    pub inner: Option<std::sync::Arc<TerraneSharedCell<Holder>>>,
}
impl SharedOuter {
    pub fn terrane_construct(inner: std::sync::Arc<TerraneSharedCell<Holder>>) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: std::sync::Arc<TerraneSharedCell<Holder>>) {
        {
            let __terrane_value = inner.clone();
            self.inner = Some(__terrane_value);
        }
    }
    pub async fn run(&self) -> terrane_int_support::Int {
        return __terrane_await({
                let __terrane_shared_call_handle = self
                    .inner
                    .as_ref()
                    .expect("required field initialized")
                    .clone();
                Holder::__terrane_shared_async_read(__terrane_shared_call_handle)
            })
            .await;
    }
    pub async fn __terrane_shared_async_run(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        return __terrane_await({
                let __terrane_shared_call_handle = {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .inner
                        .as_ref()
                        .expect("required field initialized")
                        .clone();
                    __terrane_snapshot
                }
                    .clone();
                Holder::__terrane_shared_async_read(__terrane_shared_call_handle)
            })
            .await;
    }
    pub async fn update(&mut self) -> terrane_int_support::Int {
        return __terrane_await({
                let __terrane_shared_call_handle = self
                    .inner
                    .as_ref()
                    .expect("required field initialized")
                    .clone();
                Holder::__terrane_shared_async_bump(__terrane_shared_call_handle)
            })
            .await;
    }
    pub async fn __terrane_shared_async_update(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> terrane_int_support::Int {
        return __terrane_await({
                let __terrane_shared_call_handle = {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .inner
                        .as_ref()
                        .expect("required field initialized")
                        .clone();
                    __terrane_snapshot
                }
                    .clone();
                Holder::__terrane_shared_async_bump(__terrane_shared_call_handle)
            })
            .await;
    }
}
fn main() {
    __terrane_run(async move {
        let value: std::sync::Arc<TerraneSharedCell<Holder>>;
        let copied: SnapshotOuter;
        let owner: std::sync::Arc<TerraneSharedCell<Holder>>;
        let shared: std::sync::Arc<TerraneSharedCell<SharedOuter>>;
        let outer_owner: std::sync::Arc<TerraneSharedCell<SharedOuter>>;
        value = std::sync::Arc::new(TerraneSharedCell::new(Holder::terrane_construct()));
        copied = SnapshotOuter::terrane_construct(
            {
                let __terrane_value = value
                    .lock()
                    .expect("reference lock poisoned")
                    .clone();
                __terrane_value
            }
                .clone(),
        );
        owner = value.clone();
        shared = std::sync::Arc::new(
            TerraneSharedCell::new(SharedOuter::terrane_construct(owner.clone())),
        );
        outer_owner = shared.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = outer_owner.clone();
            SharedOuter::__terrane_shared_async_update(__terrane_shared_call_handle) }).
            await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = outer_owner.clone();
            SharedOuter::__terrane_shared_async_run(__terrane_shared_call_handle) }).
            await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = value.clone();
            Holder::__terrane_shared_async_read(__terrane_shared_call_handle) }). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await((&copied).run()).
            await)
        );
    });
}
