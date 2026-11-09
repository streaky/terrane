// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: completion-finally-conditional-post-read
fn eat(
    k: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return __terrane_raised(
        k
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    0 /* terrane-site: case.trn:5:12-5:16 */,
                ),
            ),
        0 /* terrane-site: case.trn:5:12-5:16 */,
    );
}
fn main() {
    let c: bool;
    let mut o: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    c = true;
    let _ = o
        .insert(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&eat(o.as_ref()
                .expect("flow-proven available binding").clone()))
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
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        if c {
            let _ = o
                .insert(
                    terrane_collection_support::List::<
                        terrane_int_support::Int,
                    >::new(vec![terrane_int_support::Int::from(2_i128)]),
                );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(o
            .as_ref().expect("flow-proven available binding")
            .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            1 /* terrane-site: case.trn:15:16-15:20 */)), 1 /* terrane-site: case.trn:15:16-15:20 */))
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
}
