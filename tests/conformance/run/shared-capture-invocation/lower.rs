// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
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
    pub fn add(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = self.value.clone() + amount.clone();
            self.value = __terrane_value;
        }
    }
}
fn invoke(
    callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
) -> terrane_int_support::Int {
    return callback();
}
fn main() {
    let state: std::sync::Arc<TerraneSharedCell<Counter>>;
    let write: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    let method: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    state = std::sync::Arc::new(TerraneSharedCell::new(Counter::terrane_construct()));
    write = {
        let state = state.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            {
                let __terrane_owner_0 = state.clone();
                let __terrane_value = {
                    let __terrane_snapshot = state
                        .lock()
                        .expect("shared reference lock poisoned")
                        .value
                        .clone();
                    __terrane_snapshot
                } + terrane_int_support::Int::from(1_i128);
                __terrane_owner_0
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value = __terrane_value;
            }
            return {
                let __terrane_snapshot = state
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value
                    .clone();
                __terrane_snapshot
            };
        })
    };
    method = {
        let state = state.clone();
        std::sync::Arc::new(move || -> terrane_int_support::Int {
            {
                let __terrane_owner_2 = state.clone();
                let __terrane_shared_argument_0 = terrane_int_support::Int::from(3_i128);
                let __terrane_shared_argument_1 = __terrane_shared_argument_0;
                let __terrane_call_result = __terrane_owner_2
                    .lock()
                    .expect("shared reference lock poisoned")
                    .add(__terrane_shared_argument_1);
                __terrane_call_result
            };
            return {
                let __terrane_snapshot = state
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value
                    .clone();
                __terrane_snapshot
            };
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&invoke(write.clone())));
    println!("{}", terrane_scalar_support::scalar_text(&invoke(method.clone())));
    println!("{}", terrane_scalar_support::scalar_text(&invoke(write.clone())));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = state
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot })
    );
}
