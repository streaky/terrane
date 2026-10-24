// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: independent-control-region-reads
fn main() {
    let mut across_if: i8;
    let mut across_loop: i8;
    let mut first: String;
    let mut second: String;
    let mut across_catch: i8;
    let mut exclusive: i8;
    across_if = 0;
    if 1 == 1 {
        across_if = 1;
    }
    if 1 == 1 {
        println!("{}", terrane_scalar_support::scalar_text(&across_if));
    }
    across_loop = 0;
    let __terrane_iterable_0 = String::from("ab");
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &__terrane_iterable_0,
    );
    loop {
        first = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &first;
        across_loop = 1;
    }
    let __terrane_iterable_1 = String::from("cd");
    let mut __terrane_iterator_1 = terrane_collection_support::string_iterator(
        &__terrane_iterable_1,
    );
    loop {
        second = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &second;
        println!("{}", terrane_scalar_support::scalar_text(&across_loop));
    }
    across_catch = 0;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            across_catch = 7;
            return TerraneCompletion::Error(
                TerraneError::raised(
                    TerraneErrorKind::ArithmeticOverflow,
                    0 /* terrane-site: case.trn:17:5-17:30 */,
                ),
            );
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&across_catch));
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
    exclusive = 0;
    if 1 == 1 {
        exclusive = 9;
        let _ = &exclusive;
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&exclusive));
    }
}
