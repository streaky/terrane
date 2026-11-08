// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-optional-async-replacement
async fn five() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(5_i128);
}
pub static TERRANE_STATIC_HOLDER_F: std::sync::LazyLock<
    TerraneSharedCell<
        Option<
            std::sync::Arc<
                dyn Fn(
                ) -> std::pin::Pin<
                        Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                    > + Send + Sync,
            >,
        >,
    >,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(None));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    __terrane_run(async move {
        let callback: Option<
            std::sync::Arc<
                dyn Fn(
                ) -> std::pin::Pin<
                        Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                    > + Send + Sync,
            >,
        >;
        {
            let __terrane_static_value: Option<
                std::sync::Arc<
                    dyn Fn(
                    ) -> std::pin::Pin<
                            Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                        > + Send + Sync,
                >,
            > = Some(
                std::sync::Arc::new(move || -> std::pin::Pin<
                    Box<dyn Future<Output = _> + Send>,
                > { Box::pin(five()) }),
            );
            *TERRANE_STATIC_HOLDER_F.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        callback = {
            let __terrane_value = TERRANE_STATIC_HOLDER_F
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        };
        if (&callback).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_await(match
                &callback { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") } ()). await)
            );
        }
        {
            let __terrane_static_value: Option<
                std::sync::Arc<
                    dyn Fn(
                    ) -> std::pin::Pin<
                            Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                        > + Send + Sync,
                >,
            > = None;
            *TERRANE_STATIC_HOLDER_F.lock().expect("static field lock poisoned") = __terrane_static_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value =
            TERRANE_STATIC_HOLDER_F.lock().expect("static field lock poisoned").clone();
            __terrane_value } .is_none())
        );
    });
}
