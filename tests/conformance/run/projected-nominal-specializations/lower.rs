// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let point: terrane_point_witness::Point<f32>;
    let snapped: terrane_point_witness::Point<u32>;
    point = {
        let __terrane_conversion_input_0 = 3.4_f32;
        let __terrane_conversion_input_1 = 4.6_f32;
        __terrane_raised(
            terrane_static_trn_506f696e74_new(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            0 /* terrane-site: src/main.trn:6:13-6:33 */,
        )
    };
    snapped = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(point.snap()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_point_witness",
                                "terrane_point_witness::Point::snap",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        1 /* terrane-site: src/main.trn:7:15-7:26 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&snapped).total()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_point_witness", "terrane_point_witness::Point::total")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 2 /* terrane-site: src/main.trn:8:13-8:27 */))
    );
}
