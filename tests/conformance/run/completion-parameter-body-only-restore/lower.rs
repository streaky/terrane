// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: completion-parameter-body-only-restore
fn eat(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(value.length());
}
fn restored(
    value: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut value: Option<terrane_collection_support::List<terrane_int_support::Int>> = Some(
        value,
    );
    let __terrane_completion_0: TerraneCompletion<terrane_int_support::Int> = (|| {
        let __terrane_try_0: TerraneCompletion<terrane_int_support::Int> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&eat(value.as_ref()
                .expect("flow-proven available binding").clone()))
            );
            let _ = value
                .insert(
                    terrane_collection_support::List::<
                        terrane_int_support::Int,
                    >::new(vec![terrane_int_support::Int::from(2_i128)]),
                );
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
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&9));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return __terrane_raised(
        value
            .as_ref()
            .expect("flow-proven available binding")
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    0 /* terrane-site: case.trn:13:12-13:20 */,
                ),
            ),
        0 /* terrane-site: case.trn:13:12-13:20 */,
    );
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&restored(terrane_collection_support::List::
        < terrane_int_support::Int >
        ::new(vec![terrane_int_support::Int::from(1_i128)])))
    );
}
