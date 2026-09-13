// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: independent-control-region-reads
fn main() {
    let mut across_if: i8 = 0;
    if 1 == 1 {
        across_if = 1;
    }
    if 1 == 1 {
        println!("{}", terrane_scalar_support::scalar_text(&across_if));
    }
    let mut across_loop: i8 = 0;
    let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(
        &String::from("ab"),
    );
    loop {
        let first = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &first;
        across_loop = 1;
    }
    let mut __terrane_iterator_1 = terrane_collection_support::string_iterator(
        &String::from("cd"),
    );
    loop {
        let second = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &second;
        println!("{}", terrane_scalar_support::scalar_text(&across_loop));
    }
    let mut across_catch: i8 = 0;
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
    let mut exclusive: i8 = 0;
    if 1 == 1 {
        exclusive = 9;
        let _ = &mut exclusive;
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&exclusive));
    }
}
