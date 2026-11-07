// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        0 /* terrane-site: src/main.trn:4:9-4:21 */,
    );
    let _ = v
        .insert(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&h).view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                1 /* terrane-site: src/main.trn:5:9-5:16 */,
            ),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!("{}", terrane_scalar_support::scalar_text(&0));
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(v.take()
            .expect("flow-proven availability").into_size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::View::into_size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 2 /* terrane-site: src/main.trn:9:17-9:29 */))
        );
        let _ = v
            .insert(
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { { let value =
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                    Result < _, crate ::TerraneForeignError > { Ok((&h).view()) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane_query_witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 3 /* terrane-site: src/main.trn:10:13-10:20 */
                ),
            );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v.as_ref()
        .expect("flow-proven available binding")).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 4 /* terrane-site: src/main.trn:11:13-11:20 */))
    );
}
