// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: collection-lookup-errors
fn main() {
    let values: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(values
                .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
                0 /* terrane-site: case.trn:7:12-7:21 */)), 0 /* terrane-site: case.trn:7:12-7:21 */))
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
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&String::from("index"))
                    );
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
    let checked_index: Option<terrane_int_support::Int> = terrane_collection_support::index_from_int(
            &terrane_int_support::Int::from(2_i128),
        )
        .ok()
        .and_then(|index| values.get(index).cloned());
    println!("{}", terrane_scalar_support::scalar_text(&checked_index.is_none()));
    let values_by_key: terrane_collection_support::Map<
        String,
        terrane_int_support::Int,
    > = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("present"),
            terrane_int_support::Int::from(1_i128))
        ],
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(values_by_key
                .get_or_error(&String::from("absent")), 1 /* terrane-site: case.trn:14:12-14:35 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind == TerraneErrorKind::MissingKey
                {
                    __terrane_handled_1 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("missing"))
                    );
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let checked_key: Option<terrane_int_support::Int> = values_by_key
        .get(&String::from("absent"))
        .cloned();
    let present_key: Option<terrane_int_support::Int> = values_by_key
        .get(&String::from("present"))
        .cloned();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&checked_key.is_none()),
        terrane_scalar_support::scalar_text(&present_key.is_some())
    );
}
