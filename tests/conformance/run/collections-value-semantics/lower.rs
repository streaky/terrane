// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: collections-value-semantics
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S446 {
    Arm0(terrane_collection_support::Entry<String, terrane_int_support::Int>),
    Arm1(terrane_collection_support::Tuple<String>),
}
fn main() {
    let original: terrane_collection_support::List<terrane_int_support::Int>;
    let mut independent: terrane_collection_support::List<terrane_int_support::Int>;
    let mut ordered: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let mut pair: Option<TerraneUnionF0S446> = None;
    let mut unique: terrane_collection_support::Set<String>;
    let mut value: String;
    let pair_2: terrane_collection_support::Tuple<String>;
    let explicit: terrane_collection_support::Entry<String, terrane_int_support::Int>;
    let numbers: terrane_collection_support::Range;
    let mut number: terrane_int_support::Int;
    let inclusive: terrane_collection_support::Range;
    let mut empty_count: terrane_int_support::Int;
    let empty: terrane_collection_support::Range;
    let mut ignored: terrane_int_support::Int;
    let mut deterministic_map: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let mut deterministic_set: terrane_collection_support::UnorderedSet<String>;
    let mut empty_list: terrane_collection_support::List<terrane_int_support::Int>;
    let mut empty_map: terrane_collection_support::Map<terrane_int_support::Int, String>;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let arbitrary: terrane_collection_support::Map<terrane_int_support::Int, String>;
    original = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    independent = original.clone();
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(3_i128);
        independent.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(1_i128),
            ),
            0 /* terrane-site: case.trn:8:3-8:21 */,
        );
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        let _ = __terrane_raised(
            independent.set(__terrane_index, __terrane_value),
            0 /* terrane-site: case.trn:8:3-8:21 */,
        );
    }
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
    ordered = terrane_collection_support::Map::<
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
    {
        let __terrane_collection_argument_0 = String::from("third");
        let __terrane_collection_argument_1 = terrane_int_support::Int::from(3_i128);
        ordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    {
        let __terrane_key = String::from("second");
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        ordered.set(__terrane_key, __terrane_value);
    }
    let __terrane_iterable_0 = ordered.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let _ = pair
            .insert(
                TerraneUnionF0S446::Arm0(
                    match __terrane_iterator_0.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    },
                ),
            );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&match pair.as_ref()
            .expect("flow-proven available binding") { TerraneUnionF0S446::Arm0(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .key.clone()),
            terrane_scalar_support::scalar_text(&match pair.as_ref()
            .expect("flow-proven available binding") { TerraneUnionF0S446::Arm0(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .value.clone())
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(ordered
        .get_or_error(&String::from("second")), 3 /* terrane-site: case.trn:15:10-15:27 */))
    );
    unique = terrane_collection_support::Set::<
        String,
    >::new(vec![String::from("b"), String::from("a"), String::from("b")]);
    {
        let __terrane_collection_argument_0 = String::from("c");
        unique.add(__terrane_collection_argument_0);
    };
    let __terrane_iterable_1 = unique;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let __terrane_replacement = terrane_collection_support::Tuple::<
        String,
    >::new(vec![String::from("left"), String::from("right")]);
    drop(pair.take());
    pair_2 = __terrane_replacement;
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(pair_2
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(pair_2
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:21:23-21:30 */)), 4 /* terrane-site: case.trn:21:23-21:30 */))
    );
    explicit = terrane_collection_support::Entry::<
        String,
        terrane_int_support::Int,
    >::new(String::from("key"), terrane_int_support::Int::from(7_i128));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&explicit.key.clone()),
        terrane_scalar_support::scalar_text(&explicit.value.clone())
    );
    numbers = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i64),
        ),
        5 /* terrane-site: case.trn:24:13-24:24 */,
    );
    let __terrane_iterable_2 = numbers;
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        number = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    inclusive = __terrane_raised(
        terrane_collection_support::Range::through(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(-1_i128),
        ),
        6 /* terrane-site: case.trn:27:15-27:38 */,
    );
    let __terrane_iterable_3 = inclusive;
    let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_3,
    );
    loop {
        number = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &number;
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    empty_count = terrane_int_support::Int::from(0_i128);
    empty = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(-1_i128),
        ),
        7 /* terrane-site: case.trn:31:11-31:26 */,
    );
    let __terrane_iterable_4 = empty;
    let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_4,
    );
    loop {
        ignored = match __terrane_iterator_4.next() {
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
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("zero"));
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
    deterministic_map = terrane_collection_support::UnorderedMap::<
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
    {
        let __terrane_key = String::from("second");
        let __terrane_value = terrane_int_support::Int::from(3_i128);
        deterministic_map.set(__terrane_key, __terrane_value);
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_map
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(deterministic_map
        .get_or_error(&String::from("second")), 9 /* terrane-site: case.trn:41:36-41:63 */))
    );
    let __terrane_iterable_5 = deterministic_map;
    let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_5,
    );
    loop {
        let _ = pair
            .insert(
                TerraneUnionF0S446::Arm0(
                    match __terrane_iterator_5.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    },
                ),
            );
        let _ = &pair;
        println!(
            "{}", terrane_scalar_support::scalar_text(&match pair.as_ref()
            .expect("flow-proven available binding") { TerraneUnionF0S446::Arm0(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .key.clone())
        );
    }
    deterministic_set = terrane_collection_support::UnorderedSet::<
        String,
    >::new(vec![String::from("x"), String::from("y")]);
    {
        let __terrane_collection_argument_0 = String::from("z");
        deterministic_set.add(__terrane_collection_argument_0);
    };
    {
        let __terrane_result = {
            let __terrane_value = String::from("x");
            deterministic_set.remove(&__terrane_value)
        };
        __terrane_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&deterministic_set
        .contains(&String::from("y")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(deterministic_set
        .length()))
    );
    let __terrane_iterable_6 = deterministic_set;
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_6,
    );
    loop {
        value = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    empty_list = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(5_i128);
        empty_list.append(__terrane_collection_argument_0);
    };
    empty_map = terrane_collection_support::Map::<
        terrane_int_support::Int,
        String,
    >::new(Vec::new());
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(1_i128);
        let __terrane_collection_argument_1 = String::from("one");
        empty_map.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    nested = terrane_collection_support::List::<
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
    arbitrary = terrane_collection_support::Map::<
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
