// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn view<'view>(state: &'view String) -> witness::ScopedView<'view> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(witness::borrowed_view(&*state)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::borrowed_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        0 /* terrane-site: src/main.trn:6:12-6:32 */,
    );
}
#[allow(unsafe_code)]
fn main() {
    let state: String;
    state = String::from("unsafe");
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(20_i128); let __terrane_conversion_input_1 =
        terrane_int_support::Int::from(22_i128); __terrane_raised(unsafe {
        unchecked_add(__terrane_conversion_input_0, __terrane_conversion_input_1) },
        1 /* terrane-site: src/main.trn:10:13-10:41 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(witness::render_unsafe(&state, view)) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::render_unsafe")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (), 2 /* terrane-site: src/main.trn:11:13-11:39 */))
    );
}
