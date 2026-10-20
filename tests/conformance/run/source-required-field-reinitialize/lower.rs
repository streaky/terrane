// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub struct Holder {
    pub value: Option<Builder>,
}
impl Holder {
    pub fn terrane_construct(value: Builder) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: Builder) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
fn main() {
    __terrane_run(async move {
        let mut h: Holder;
        let first: Builder;
        let mut replacement: Builder;
        h = Holder::terrane_construct(
            __terrane_raised(
                terrane_static_trn_4275696c646572_new(),
                0 /* terrane-site: src/main.trn:8:27-8:40 */,
            ),
        );
        first = h.value.take().expect("required field initialized");
        replacement = __terrane_raised(
            terrane_static_trn_4275696c646572_new(),
            1 /* terrane-site: src/main.trn:10:19-10:32 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = replacement
                        .set(
                            match || -> Result<_, crate::TerraneForeignError> {
                                Ok(
                                    terrane_int_support::coerce::<
                                        u8,
                                    >(&terrane_int_support::Int::from(3_i128))
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                )
                            }() {
                                Ok(value) => value,
                                Err(error) => std::panic::panic_any(error),
                            },
                        );
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_borrowed_future_witness",
                            "terrane_borrowed_future_witness::Builder::set",
                        ),
                    )
                }
            },
            2 /* terrane-site: src/main.trn:11:5-11:23 */,
        );
        {
            let __terrane_value = replacement;
            h.value = Some(__terrane_value);
        }
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({ let
            __terrane_future = { let __terrane_call = (&first).serve(); Box::pin(async
            move { match crate
            ::__terrane_dependency_await_unwind(std::future::IntoFuture::into_future(__terrane_call))
            . await { Ok(Ok(value)) => Ok(terrane_int_support::Int::from_u128(value as
            u128)), Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
            ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
            format!("Rust dependency `terrane_borrowed_future_witness` member `terrane_borrowed_future_witness::Builder::serve` failed: {error}"),
            crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "terrane_borrowed_future_witness",
            "terrane_borrowed_future_witness::Builder::serve")) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 3 /* terrane-site: src/main.trn:13:19-13:31 */) } }). await, 3 /* terrane-site: src/main.trn:13:19-13:31 */)),
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({ let
            __terrane_future = { let __terrane_call = (&h.value.as_ref()
            .expect("required field initialized")).serve(); Box::pin(async move { match
            crate
            ::__terrane_dependency_await_unwind(std::future::IntoFuture::into_future(__terrane_call))
            . await { Ok(Ok(value)) => Ok(terrane_int_support::Int::from_u128(value as
            u128)), Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
            ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
            format!("Rust dependency `terrane_borrowed_future_witness` member `terrane_borrowed_future_witness::Builder::serve` failed: {error}"),
            crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "terrane_borrowed_future_witness",
            "terrane_borrowed_future_witness::Builder::serve")) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 4 /* terrane-site: src/main.trn:13:41-13:55 */) } }). await, 4 /* terrane-site: src/main.trn:13:41-13:55 */))
        );
    });
}
