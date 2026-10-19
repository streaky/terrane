// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-parameter-owner
#[derive(Clone)]
pub struct State {
    pub total: terrane_int_support::Int,
    pub drops: terrane_int_support::Int,
}
impl State {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
            drops: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn record(&mut self, value: terrane_int_support::Int) {
        {
            let __terrane_value = self.total.clone() + value.clone();
            self.total = __terrane_value;
        }
        {
            let __terrane_value = self.drops.clone()
                + terrane_int_support::Int::from(1_i128);
            self.drops = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct Tracked {
    __terrane_constructed: bool,
    __terrane_lifetime: std::sync::Arc<()>,
    pub observer: Option<std::sync::Arc<TerraneSharedCell<State>>>,
    pub value: terrane_int_support::Int,
}
impl Tracked {
    pub fn terrane_construct(
        observer: std::sync::Arc<TerraneSharedCell<State>>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            observer: None,
            value: terrane_int_support::Int::from(0_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(observer);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, observer: std::sync::Arc<TerraneSharedCell<State>>) {
        {
            let __terrane_value = observer.clone();
            self.observer = Some(__terrane_value);
        }
    }
    pub fn destruct(&mut self) {
        {
            let __terrane_owner = self
                .observer
                .as_ref()
                .expect("required field initialized")
                .clone();
            let __terrane_shared_argument_0 = self.value.clone();
            let __terrane_shared_argument_1 = __terrane_shared_argument_0;
            let __terrane_call_result = __terrane_owner
                .lock()
                .expect("shared reference lock poisoned")
                .record(__terrane_shared_argument_1);
            __terrane_call_result
        };
    }
}
impl Drop for Tracked {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn share(owner: Tracked) {
    let owner = std::sync::Arc::new(TerraneSharedCell::new(owner));
    let alias: std::sync::Arc<TerraneSharedCell<Tracked>>;
    alias = owner.clone();
    {
        let __terrane_owner = alias.clone();
        let __terrane_value = terrane_int_support::Int::from(3_i128);
        __terrane_owner.lock().expect("shared reference lock poisoned").value = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = owner
        .lock().expect("reference lock poisoned").value.clone(); __terrane_snapshot })
    );
}
async fn share_async(owner: Tracked) {
    let owner = std::sync::Arc::new(TerraneSharedCell::new(owner));
    let alias: std::sync::Arc<TerraneSharedCell<Tracked>>;
    alias = owner.clone();
    {
        let __terrane_owner = alias.clone();
        let __terrane_value = terrane_int_support::Int::from(5_i128);
        __terrane_owner.lock().expect("shared reference lock poisoned").value = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = owner
        .lock().expect("reference lock poisoned").value.clone(); __terrane_snapshot })
    );
}
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
fn replace(owner: Counter) {
    let owner = std::sync::Arc::new(TerraneSharedCell::new(owner));
    let alias: std::sync::Arc<TerraneSharedCell<Counter>>;
    alias = owner.clone();
    *owner.lock().expect("reference lock poisoned") = Counter::terrane_construct();
    {
        let __terrane_owner = owner.clone();
        let __terrane_value = terrane_int_support::Int::from(99_i128);
        __terrane_owner.lock().expect("reference lock poisoned").value = __terrane_value;
    }
    {
        let __terrane_owner = alias.clone();
        let __terrane_value = terrane_int_support::Int::from(11_i128);
        __terrane_owner.lock().expect("shared reference lock poisoned").value = __terrane_value;
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = owner
        .lock().expect("reference lock poisoned").value.clone(); __terrane_snapshot }),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = alias.lock()
        .expect("shared reference lock poisoned").value.clone(); __terrane_snapshot })
    );
}
fn main() {
    __terrane_run(async move {
        let observer: std::sync::Arc<TerraneSharedCell<State>>;
        let callback: std::sync::Arc<dyn Fn(Tracked) -> () + Send + Sync>;
        observer = std::sync::Arc::new(
            TerraneSharedCell::new(State::terrane_construct()),
        );
        share(Tracked::terrane_construct(observer.clone()));
        __terrane_await(share_async(Tracked::terrane_construct(observer.clone()))).await;
        callback = {
            std::sync::Arc::new(move |owner: Tracked| -> () {
                let owner = std::sync::Arc::new(TerraneSharedCell::new(owner));
                let alias: std::sync::Arc<TerraneSharedCell<Tracked>>;
                alias = owner.clone();
                {
                    let __terrane_owner = alias.clone();
                    let __terrane_value = terrane_int_support::Int::from(7_i128);
                    __terrane_owner
                        .lock()
                        .expect("shared reference lock poisoned")
                        .value = __terrane_value;
                }
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
                    owner.lock().expect("reference lock poisoned").value.clone();
                    __terrane_snapshot })
                );
                ()
            })
        };
        callback(Tracked::terrane_construct(observer.clone()));
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
            observer.lock().expect("shared reference lock poisoned").total.clone();
            __terrane_snapshot }), terrane_scalar_support::scalar_text(&{ let
            __terrane_snapshot = observer.lock().expect("shared reference lock poisoned")
            .drops.clone(); __terrane_snapshot })
        );
        replace(Counter::terrane_construct());
    });
}
