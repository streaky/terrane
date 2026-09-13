// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: collections-value-semantics
fn main() {
    let original: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let mut independent: terrane_collection_support::List<terrane_int_support::Int> = original
        .clone();
    independent.append(terrane_int_support::Int::from(3_i128));
    let _ = __terrane_raised(
        independent
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    ),
                    0 /* terrane-site: case.trn:8:3-8:21 */,
                ),
                terrane_int_support::Int::from(4_i128),
            ),
        0 /* terrane-site: case.trn:8:3-8:21 */,
    );
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(original
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(independent
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(independent
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        1 /* terrane-site: case.trn:9:47-9:61 */)), 1 /* terrane-site: case.trn:9:47-9:61 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(independent
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:9:63-9:77 */)), 2 /* terrane-site: case.trn:9:63-9:77 */))
    );
    let mut ordered: terrane_collection_support::Map<String, terrane_int_support::Int> = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    ordered.set(String::from("third"), terrane_int_support::Int::from(3_i128));
    let _ = ordered.set(String::from("second"), terrane_int_support::Int::from(4_i128));
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &ordered,
    );
    loop {
        let pair = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&pair.key),
            terrane_scalar_support::scalar_text(&pair.value)
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered
        .get_or_error(&String::from("second")), 3 /* terrane-site: case.trn:15:10-15:27 */))
    );
    let mut unique: terrane_collection_support::Set<String> = terrane_collection_support::Set::<
        String,
    >::new(vec![String::from("b"), String::from("a"), String::from("b")]);
    unique.add(String::from("c"));
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &unique,
    );
    loop {
        let value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let pair: terrane_collection_support::Tuple<String> = terrane_collection_support::Tuple::<
        String,
    >::new(vec![String::from("left"), String::from("right")]);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(pair
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:21:23-21:30 */)), 4 /* terrane-site: case.trn:21:23-21:30 */))
    );
    let explicit: terrane_collection_support::Entry<String, terrane_int_support::Int> = terrane_collection_support::Entry::<
        String,
        terrane_int_support::Int,
    >::new(String::from("key"), terrane_int_support::Int::from(7_i128));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&explicit.key),
        terrane_scalar_support::scalar_text(&explicit.value)
    );
    let numbers: terrane_collection_support::Range = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i64),
        ),
        5 /* terrane-site: case.trn:24:13-24:24 */,
    );
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &numbers,
    );
    loop {
        let number = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    let inclusive: terrane_collection_support::Range = __terrane_raised(
        terrane_collection_support::Range::through(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(-1_i128),
        ),
        6 /* terrane-site: case.trn:27:15-27:38 */,
    );
    let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
        &inclusive,
    );
    loop {
        let number = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    let mut empty_count: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    let empty: terrane_collection_support::Range = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(-1_i128),
        ),
        7 /* terrane-site: case.trn:31:11-31:26 */,
    );
    let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
        &empty,
    );
    loop {
        let ignored = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &ignored;
        empty_count = empty_count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&empty_count));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                terrane_collection_support::Range::new(terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(3_i128),
                terrane_int_support::Int::from(0_i128)), 8 /* terrane-site: case.trn:36:5-36:19 */
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
                    && __terrane_error_0.kind == TerraneErrorKind::SourceError
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&String::from("zero"))
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
    let mut deterministic_map: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    > = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("second"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    let _ = deterministic_map
        .set(String::from("second"), terrane_int_support::Int::from(3_i128));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_map
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(deterministic_map
        .get_or_error(&String::from("second")), 9 /* terrane-site: case.trn:41:36-41:63 */))
    );
    let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
        &deterministic_map,
    );
    loop {
        let pair = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&pair.key));
    }
    let mut deterministic_set: terrane_collection_support::UnorderedSet<String> = terrane_collection_support::UnorderedSet::<
        String,
    >::new(vec![String::from("x"), String::from("y")]);
    deterministic_set.add(String::from("z"));
    deterministic_set.remove(&String::from("x"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&deterministic_set
        .contains(&String::from("y")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_set
        .length()))
    );
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &deterministic_set,
    );
    loop {
        let value = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let mut empty_list: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    empty_list.append(terrane_int_support::Int::from(5_i128));
    let mut empty_map: terrane_collection_support::Map<
        terrane_int_support::Int,
        String,
    > = terrane_collection_support::Map::<
        terrane_int_support::Int,
        String,
    >::new(Vec::new());
    empty_map.set(terrane_int_support::Int::from(1_i128), String::from("one"));
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    > = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(9_i128)])
        ],
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty_list
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(empty_map
        .get_or_error(&terrane_int_support::Int::from(1_i128)), 10 /* terrane-site: case.trn:56:29-56:41 */)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested
        .length()))
    );
    let arbitrary: terrane_collection_support::Map<terrane_int_support::Int, String> = terrane_collection_support::Map::<
        terrane_int_support::Int,
        String,
    >::new(
        vec![
            terrane_collection_support::Entry::< terrane_int_support::Int, String
            >::new(terrane_int_support::Int::from(2_i128), String::from("two")),
            terrane_collection_support::Entry::< terrane_int_support::Int, String
            >::new(terrane_int_support::Int::from(3_i128), String::from("three"))
        ],
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(arbitrary
        .get_or_error(&terrane_int_support::Int::from(2_i128)), 11 /* terrane-site: case.trn:58:10-58:22 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(arbitrary
        .get_or_error(&terrane_int_support::Int::from(3_i128)), 12 /* terrane-site: case.trn:58:24-58:36 */))
    );
}
