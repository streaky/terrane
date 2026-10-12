// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: list-append-bulk-mutation
fn validate_large_literal(enabled: bool) {
    let mut values: terrane_collection_support::List<i64>;
    let mut index: i64;
    if enabled {
        values = terrane_collection_support::List::<i64>::new(Vec::new());
        index = 0;
        {
            let __terrane_list_append_0 = values.make_unique();
            let __terrane_list_start_0 = index;
            let __terrane_list_end_0 = 4000000000 as i64;
            let __terrane_list_length_0 = (__terrane_list_start_0..__terrane_list_end_0)
                .size_hint()
                .0;
            let __terrane_list_capacity_limit_0 = 32usize
                / std::mem::size_of::<i64>().max(1);
            if __terrane_list_length_0 <= __terrane_list_capacity_limit_0 {
                __terrane_list_append_0.reserve(__terrane_list_length_0);
                for __terrane_list_index_0 in __terrane_list_start_0..__terrane_list_end_0 {
                    index = __terrane_list_index_0;
                    __terrane_list_append_0.push(index);
                }
                index = std::cmp::max(__terrane_list_start_0, __terrane_list_end_0);
            } else {
                __terrane_list_append_0.reserve(__terrane_list_capacity_limit_0);
                while index < 4000000000 {
                    __terrane_list_append_0.push(index);
                    index = index + 1;
                }
            }
            let _ = &index;
        }
    }
}
fn validate_return() -> terrane_int_support::Int {
    let mut values: terrane_collection_support::List<i64>;
    let mut index: i64;
    let limit: i64;
    values = terrane_collection_support::List::<i64>::new(Vec::new());
    index = 0;
    limit = 100000000000000;
    {
        let __terrane_list_append_1 = values.make_unique();
        while index < limit {
            __terrane_list_append_1.push(index);
            if index > 2 {
                return terrane_int_support::Int::from(
                    __terrane_raised(
                        terrane_int_support::fixed_addition(index, 1),
                        0 /* terrane-site: case.trn:23:14-23:23 */,
                    ) as i128,
                );
            }
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                1 /* terrane-site: case.trn:24:5-24:12 */,
            );
        }
    }
    return terrane_int_support::Int::from(0_i128);
}
fn validate_exit() {
    let mut values: terrane_collection_support::List<i64>;
    let mut index: i64;
    let limit: i64;
    values = terrane_collection_support::List::<i64>::new(Vec::new());
    index = 0;
    limit = 100000000000000;
    {
        let __terrane_list_append_2 = values.make_unique();
        while index < limit {
            __terrane_list_append_2.push(index);
            if index > 2 {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition(index,
                    1), 2 /* terrane-site: case.trn:34:14-34:23 */))
                );
                exit(make_exit_status(terrane_int_support::Int::from(0_i128)));
            }
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                3 /* terrane-site: case.trn:36:5-36:12 */,
            );
        }
    }
}
fn validate_throw() {
    let mut values: Option<terrane_collection_support::List<i64>> = None;
    let mut index: Option<i64> = None;
    let mut limit: Option<i64> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = values
                .insert(terrane_collection_support::List::<i64>::new(Vec::new()));
            let _ = index.insert(0);
            let _ = limit.insert(100000000000000);
            {
                let __terrane_list_append_3 = values
                    .as_mut()
                    .expect("flow-proven available binding")
                    .make_unique();
                while *index.as_ref().expect("flow-proven available binding")
                    < *limit.as_ref().expect("flow-proven available binding")
                {
                    __terrane_list_append_3
                        .push(*index.as_ref().expect("flow-proven available binding"));
                    if *index.as_ref().expect("flow-proven available binding") > 2 {
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_int_support::fixed_addition(*
                            index.as_ref().expect("flow-proven available binding"), 1),
                            4 /* terrane-site: case.trn:46:16-46:25 */))
                        );
                        return TerraneCompletion::Error(
                            TerraneError::raised(
                                TerraneErrorKind::ArithmeticOverflow,
                                5 /* terrane-site: case.trn:47:9-47:34 */,
                            ),
                        );
                    }
                    let _ = index
                        .insert(
                            __terrane_raised_completion!(
                                terrane_int_support::fixed_addition(* index.as_ref()
                                .expect("flow-proven available binding"), 1),
                                6 /* terrane-site: case.trn:48:7-48:14 */
                            ),
                        );
                }
            }
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
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("throw-caught"));
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
}
fn observe_prefix(value: i64) -> i64 {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    return value;
}
fn main() {
    let mut values: terrane_collection_support::List<i64>;
    let mut index: i64;
    let limit: i64;
    let original: terrane_collection_support::List<i64>;
    let mut three_clause: terrane_collection_support::List<i64>;
    let mut for_index: i64;
    let mut three_clause_break: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut break_index: terrane_int_support::Int;
    let mut update_observed: terrane_collection_support::List<terrane_int_support::Int>;
    let mut update_index: terrane_int_support::Int;
    let mut condition_observed: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut condition_index: terrane_int_support::Int;
    let mut double_index: i64;
    let mut double_update: terrane_collection_support::List<i64>;
    let mut dependent: terrane_collection_support::List<i64>;
    let mut dependent_index: i64;
    let mut value: i64;
    let mut inner_index: i64;
    let mut inner: terrane_collection_support::List<i64>;
    let mut nested: terrane_collection_support::List<i64>;
    let mut outer_index: i64;
    let mut nested_index: i64;
    let mut early_exit: terrane_collection_support::List<i64>;
    let mut early_index: i64;
    let early_limit: i64;
    let mut nested_break: terrane_collection_support::List<i64>;
    let mut break_outer: i64;
    let mut break_inner: i64;
    let source: terrane_collection_support::List<i64>;
    let mut collected: terrane_collection_support::List<i64>;
    let mut observed: terrane_collection_support::List<i64>;
    let rows: terrane_collection_support::List<terrane_collection_support::List<i64>>;
    let mut flattened: terrane_collection_support::List<i64>;
    let mut row: terrane_collection_support::List<i64>;
    let mut aliased: terrane_collection_support::List<i64>;
    let alias: terrane_collection_support::List<i64>;
    let mut self_source: terrane_collection_support::List<i64>;
    let mut negative: terrane_collection_support::List<i64>;
    let mut negative_index: i64;
    let negative_limit: i64;
    let mut mapped: terrane_collection_support::List<f64>;
    let mut mapped_index: i64;
    let mapped_limit: i64;
    let mut mapped_value: f64;
    let mut mutated_before_builder: terrane_collection_support::List<i64>;
    let mut mutated_index: i64;
    let mut reassigned_before_builder: terrane_collection_support::List<i64>;
    let replacement: terrane_collection_support::List<i64>;
    let mut reassigned_index: i64;
    let mut dynamically_reentered: terrane_collection_support::List<i64>;
    let mut outer_builder_index: i64;
    let mut observed_length: terrane_int_support::Int;
    let mut inner_builder_index: i64;
    let mut prefix_effects: terrane_collection_support::List<i64>;
    let mut prefix_index: i64;
    let mut prefix_value: i64;
    let mut fallback: terrane_collection_support::List<i64>;
    let mut fallback_index: i64;
    let captured: terrane_collection_support::List<i64>;
    let mut captured_index: i64;
    let capturing_builder: TerraneMutableCallable<(), ()>;
    let local_mutation: TerraneMutableCallable<(), i64>;
    let returned_capture: terrane_collection_support::List<i64>;
    let return_capture: TerraneMutableCallable<
        (),
        terrane_collection_support::List<i64>,
    >;
    let first_returned: terrane_collection_support::List<i64>;
    let second_returned: terrane_collection_support::List<i64>;
    let inferred_capture: terrane_collection_support::List<i64>;
    let inspect_capture: TerraneMutableCallable<(), i64>;
    validate_large_literal(false);
    values = terrane_collection_support::List::<i64>::new(Vec::new());
    index = 0;
    limit = 4;
    {
        let __terrane_list_append_4 = values.make_unique();
        let __terrane_list_start_1 = index;
        let __terrane_list_end_1 = limit;
        let __terrane_list_length_1 = (__terrane_list_start_1..__terrane_list_end_1)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_1 = 32usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_1 <= __terrane_list_capacity_limit_1 {
            __terrane_list_append_4.reserve(__terrane_list_length_1);
            for __terrane_list_index_1 in __terrane_list_start_1..__terrane_list_end_1 {
                index = __terrane_list_index_1;
                __terrane_list_append_4.push(index);
            }
            index = std::cmp::max(__terrane_list_start_1, __terrane_list_end_1);
        } else {
            __terrane_list_append_4.reserve(__terrane_list_capacity_limit_1);
            while index < limit {
                __terrane_list_append_4.push(index);
                index = __terrane_raised(
                    terrane_int_support::fixed_addition(index, 1),
                    7 /* terrane-site: case.trn:63:5-63:12 */,
                );
            }
        }
        let _ = &index;
    }
    original = values.clone();
    values.append(9);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(original
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    three_clause = terrane_collection_support::List::<i64>::new(Vec::new());
    for_index = 0;
    println!("{}", terrane_scalar_support::scalar_text(&for_index));
    for_index = 0;
    {
        let __terrane_list_append_5 = three_clause.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(for_index),
            usize::try_from(limit),
        ) {
            let __terrane_capacity_limit = 32usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_5
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        '__terrane_break_2: while for_index < limit {
            '__terrane_continue_2: {
                __terrane_list_append_5.push(for_index);
            }
            for_index = __terrane_raised(
                terrane_int_support::fixed_addition(for_index, 1),
                8 /* terrane-site: case.trn:73:41-73:52 */,
            );
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(three_clause
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(three_clause
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        9 /* terrane-site: case.trn:75:31-75:46 */)), 9 /* terrane-site: case.trn:75:31-75:46 */)), terrane_scalar_support::scalar_text(&for_index)
    );
    three_clause_break = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    break_index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_6 = three_clause_break.make_unique();
        '__terrane_break_3: while break_index.clone()
            < terrane_int_support::Int::from(10_i128)
        {
            '__terrane_continue_3: {
                __terrane_list_append_6.push(break_index.clone());
                if break_index.clone() > terrane_int_support::Int::from(2_i128) {
                    break '__terrane_break_3;
                }
            }
            break_index = break_index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(three_clause_break
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(three_clause_break
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        10 /* terrane-site: case.trn:82:37-82:58 */)), 10 /* terrane-site: case.trn:82:37-82:58 */))
    );
    update_observed = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    update_index = terrane_int_support::Int::from(0_i128);
    '__terrane_break_4: while update_index.clone()
        < terrane_int_support::Int::from(3_i128)
    {
        '__terrane_continue_4: {
            update_observed.append(update_index.clone());
        }
        update_index = update_index.clone() + terrane_int_support::Int::from(1_i128)
            + terrane_int_support::Int::from(
                terrane_int_support::Int::from(update_observed.length()),
            )
            - terrane_int_support::Int::from(
                terrane_int_support::Int::from(update_observed.length()),
            );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(update_observed
        .length()))
    );
    condition_observed = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    condition_index = terrane_int_support::Int::from(0_i128);
    '__terrane_break_5: while condition_index.clone()
        < terrane_int_support::Int::from(4_i128)
            + terrane_int_support::Int::from(
                terrane_int_support::Int::from(condition_observed.length()),
            )
            - terrane_int_support::Int::from(
                terrane_int_support::Int::from(condition_observed.length()),
            )
    {
        '__terrane_continue_5: {
            condition_observed.append(condition_index.clone());
        }
        condition_index = condition_index.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(condition_observed
        .length()))
    );
    double_index = 0;
    println!("{}", terrane_scalar_support::scalar_text(&double_index));
    double_update = terrane_collection_support::List::<i64>::new(Vec::new());
    double_index = 0;
    {
        let __terrane_list_append_7 = double_update.make_unique();
        '__terrane_break_6: while double_index < limit {
            '__terrane_continue_6: {
                __terrane_list_append_7.push(double_index);
                double_index = __terrane_raised(
                    terrane_int_support::fixed_addition(double_index, 1),
                    11 /* terrane-site: case.trn:99:5-99:19 */,
                );
            }
            double_index = __terrane_raised(
                terrane_int_support::fixed_addition(double_index, 1),
                12 /* terrane-site: case.trn:97:47-97:61 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(double_update
        .length()))
    );
    dependent = terrane_collection_support::List::<i64>::new(Vec::new());
    dependent_index = 0;
    while dependent_index < 3 {
        dependent
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(&terrane_int_support::Int::from(dependent.length())),
                    13 /* terrane-site: case.trn:105:23-105:39 */,
                ),
            );
        dependent_index = dependent_index + 1;
    }
    let __terrane_iterable_7 = dependent;
    let mut __terrane_iterator_7 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_7,
    );
    loop {
        value = match __terrane_iterator_7.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    inner_index = 0;
    while inner_index < 1 {
        inner = terrane_collection_support::List::<i64>::new(Vec::new());
        inner.append(inner_index);
        inner_index = inner_index + 1;
    }
    nested = terrane_collection_support::List::<i64>::new(Vec::new());
    outer_index = 0;
    {
        let __terrane_list_append_8 = nested.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(outer_index),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 32usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_8
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while outer_index < 3 {
            __terrane_list_append_8.push(outer_index);
            nested_index = 0;
            while nested_index < 2 {
                __terrane_list_append_8.push(nested_index);
                nested_index = nested_index + 1;
            }
            outer_index = outer_index + 1;
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested
        .length()))
    );
    early_exit = terrane_collection_support::List::<i64>::new(Vec::new());
    early_index = 0;
    early_limit = 100000000000000;
    {
        let __terrane_list_append_9 = early_exit.make_unique();
        while early_index < early_limit {
            __terrane_list_append_9.push(early_index);
            if early_index > 2 {
                break;
            }
            early_index = __terrane_raised(
                terrane_int_support::fixed_addition(early_index, 1),
                14 /* terrane-site: case.trn:135:5-135:18 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(early_exit
        .length()))
    );
    nested_break = terrane_collection_support::List::<i64>::new(Vec::new());
    break_outer = 0;
    {
        let __terrane_list_append_10 = nested_break.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(break_outer),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 32usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_10
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while break_outer < 3 {
            __terrane_list_append_10.push(break_outer);
            break_inner = 0;
            while break_inner < 3 {
                break;
            }
            break_outer = break_outer + 1;
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested_break
        .length()))
    );
    source = terrane_collection_support::List::<i64>::new(vec![1, 2, 3, 4]);
    collected = terrane_collection_support::List::<i64>::new(Vec::new());
    let __terrane_iterable_8 = source.clone();
    let mut __terrane_iterator_8 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_8,
    );
    {
        let __terrane_list_append_11 = collected.make_unique();
        loop {
            value = match __terrane_iterator_8.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &value;
            __terrane_list_append_11.push(value);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(collected
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(collected
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        15 /* terrane-site: case.trn:152:28-152:40 */)), 15 /* terrane-site: case.trn:152:28-152:40 */))
    );
    observed = terrane_collection_support::List::<i64>::new(Vec::new());
    let __terrane_iterable_9 = source;
    let mut __terrane_iterator_9 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_9,
    );
    loop {
        value = match __terrane_iterator_9.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        observed
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(&terrane_int_support::Int::from(observed.length())),
                    16 /* terrane-site: case.trn:156:22-156:37 */,
                ),
            );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(observed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        17 /* terrane-site: case.trn:157:10-157:21 */)), 17 /* terrane-site: case.trn:157:10-157:21 */))
    );
    rows = terrane_collection_support::List::<
        terrane_collection_support::List<i64>,
    >::new(
        vec![
            terrane_collection_support::List::< i64 >::new(vec![1, 2]),
            terrane_collection_support::List::< i64 >::new(vec![3])
        ],
    );
    flattened = terrane_collection_support::List::<i64>::new(Vec::new());
    let __terrane_iterable_10 = rows;
    let mut __terrane_iterator_10 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_10,
    );
    {
        let __terrane_list_append_12 = flattened.make_unique();
        loop {
            row = match __terrane_iterator_10.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let __terrane_iterable_11 = row;
            let mut __terrane_iterator_11 = terrane_collection_support::Iterable::terrane_iterator(
                &__terrane_iterable_11,
            );
            loop {
                value = match __terrane_iterator_11.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                let _ = &value;
                __terrane_list_append_12.push(value);
            }
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(flattened
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(flattened
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        18 /* terrane-site: case.trn:164:28-164:40 */)), 18 /* terrane-site: case.trn:164:28-164:40 */))
    );
    aliased = terrane_collection_support::List::<i64>::new(vec![1, 2]);
    alias = aliased.clone();
    let __terrane_iterable_12 = alias.clone();
    let mut __terrane_iterator_12 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_12,
    );
    {
        let __terrane_list_append_13 = aliased.make_unique();
        loop {
            value = match __terrane_iterator_12.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &value;
            __terrane_list_append_13.push(value);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(aliased
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(alias
        .length()))
    );
    self_source = terrane_collection_support::List::<i64>::new(vec![1, 2]);
    let __terrane_iterable_13 = self_source.clone();
    let mut __terrane_iterator_13 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_13,
    );
    loop {
        value = match __terrane_iterator_13.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        self_source.append(value);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(self_source
        .length()))
    );
    negative = terrane_collection_support::List::<i64>::new(Vec::new());
    negative_index = 0;
    negative_limit = -3;
    {
        let __terrane_list_append_14 = negative.make_unique();
        let __terrane_list_start_14 = negative_index;
        let __terrane_list_end_14 = negative_limit;
        let __terrane_list_length_14 = (__terrane_list_start_14..__terrane_list_end_14)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_14 = 32usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_14 <= __terrane_list_capacity_limit_14 {
            __terrane_list_append_14.reserve(__terrane_list_length_14);
            for __terrane_list_index_14 in __terrane_list_start_14..__terrane_list_end_14 {
                negative_index = __terrane_list_index_14;
                __terrane_list_append_14.push(negative_index);
            }
            negative_index = std::cmp::max(
                __terrane_list_start_14,
                __terrane_list_end_14,
            );
        } else {
            __terrane_list_append_14.reserve(__terrane_list_capacity_limit_14);
            while negative_index < negative_limit {
                __terrane_list_append_14.push(negative_index);
                negative_index = __terrane_raised(
                    terrane_int_support::fixed_addition(negative_index, 1),
                    19 /* terrane-site: case.trn:182:5-182:21 */,
                );
            }
        }
        let _ = &negative_index;
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(negative
        .length())), terrane_scalar_support::scalar_text(&negative_index)
    );
    mapped = terrane_collection_support::List::<f64>::new(Vec::new());
    mapped_index = 0;
    mapped_limit = 4;
    {
        let __terrane_list_append_15 = mapped.make_unique();
        let __terrane_list_start_15 = mapped_index;
        let __terrane_list_end_15 = mapped_limit;
        let __terrane_list_length_15 = (__terrane_list_start_15..__terrane_list_end_15)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_15 = 32usize
            / std::mem::size_of::<f64>().max(1);
        if __terrane_list_length_15 <= __terrane_list_capacity_limit_15 {
            __terrane_list_append_15.reserve(__terrane_list_length_15);
            for __terrane_list_index_15 in __terrane_list_start_15..__terrane_list_end_15 {
                mapped_index = __terrane_list_index_15;
                mapped_value = __terrane_raised(
                    terrane_int_support::exact_fixed_f64(mapped_index),
                    20 /* terrane-site: case.trn:189:28-189:40 */,
                );
                __terrane_list_append_15.push(mapped_value * mapped_value);
            }
            mapped_index = std::cmp::max(__terrane_list_start_15, __terrane_list_end_15);
        } else {
            __terrane_list_append_15.reserve(__terrane_list_capacity_limit_15);
            while mapped_index < mapped_limit {
                mapped_value = __terrane_raised(
                    terrane_int_support::exact_fixed_f64(mapped_index),
                    20 /* terrane-site: case.trn:189:28-189:40 */,
                );
                __terrane_list_append_15.push(mapped_value * mapped_value);
                mapped_index = __terrane_raised(
                    terrane_int_support::fixed_addition(mapped_index, 1),
                    21 /* terrane-site: case.trn:191:5-191:19 */,
                );
            }
        }
        let _ = &mapped_index;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(mapped
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(mapped
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        22 /* terrane-site: case.trn:192:25-192:34 */)), 22 /* terrane-site: case.trn:192:25-192:34 */)), terrane_scalar_support::scalar_text(&mapped_index)
    );
    mutated_before_builder = terrane_collection_support::List::<i64>::new(Vec::new());
    mutated_before_builder.append(7);
    mutated_index = 0;
    {
        let __terrane_list_append_16 = mutated_before_builder.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(mutated_index),
            usize::try_from(2 as i64),
        ) {
            let __terrane_capacity_limit = 32usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_16
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while mutated_index < 2 {
            __terrane_list_append_16.push(mutated_index);
            mutated_index = mutated_index + 1;
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(mutated_before_builder
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(mutated_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        23 /* terrane-site: case.trn:200:41-200:66 */)), 23 /* terrane-site: case.trn:200:41-200:66 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(mutated_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        24 /* terrane-site: case.trn:200:68-200:93 */)), 24 /* terrane-site: case.trn:200:68-200:93 */))
    );
    reassigned_before_builder = terrane_collection_support::List::<i64>::new(Vec::new());
    let _ = &mut reassigned_before_builder;
    replacement = terrane_collection_support::List::<i64>::new(vec![8]);
    reassigned_before_builder = replacement;
    reassigned_index = 0;
    {
        let __terrane_list_append_17 = reassigned_before_builder.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(reassigned_index),
            usize::try_from(2 as i64),
        ) {
            let __terrane_capacity_limit = 32usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_17
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while reassigned_index < 2 {
            __terrane_list_append_17.push(reassigned_index);
            reassigned_index = reassigned_index + 1;
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(reassigned_before_builder
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(reassigned_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        25 /* terrane-site: case.trn:209:44-209:72 */)), 25 /* terrane-site: case.trn:209:44-209:72 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(reassigned_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        26 /* terrane-site: case.trn:209:74-209:102 */)), 26 /* terrane-site: case.trn:209:74-209:102 */))
    );
    dynamically_reentered = terrane_collection_support::List::<i64>::new(Vec::new());
    outer_builder_index = 0;
    while outer_builder_index < 3 {
        observed_length = terrane_int_support::Int::from(
            terrane_int_support::Int::from(dynamically_reentered.length()),
        );
        let _ = &observed_length;
        inner_builder_index = 0;
        {
            let __terrane_list_append_18 = dynamically_reentered.make_unique();
            if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                usize::try_from(inner_builder_index),
                usize::try_from(2 as i64),
            ) {
                let __terrane_capacity_limit = 32usize
                    / std::mem::size_of::<i64>().max(1);
                __terrane_list_append_18
                    .reserve(
                        __terrane_end
                            .saturating_sub(__terrane_start)
                            .min(__terrane_capacity_limit),
                    );
            }
            while inner_builder_index < 2 {
                __terrane_list_append_18.push(inner_builder_index);
                inner_builder_index = inner_builder_index + 1;
            }
        }
        outer_builder_index = outer_builder_index + 1;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(dynamically_reentered
        .length()))
    );
    prefix_effects = terrane_collection_support::List::<i64>::new(Vec::new());
    prefix_index = 0;
    {
        let __terrane_list_append_19 = prefix_effects.make_unique();
        let __terrane_list_start_16 = prefix_index;
        let __terrane_list_end_16 = 3 as i64;
        let __terrane_list_length_16 = (__terrane_list_start_16..__terrane_list_end_16)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_16 = 32usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_16 <= __terrane_list_capacity_limit_16 {
            __terrane_list_append_19.reserve(__terrane_list_length_16);
            for __terrane_list_index_16 in __terrane_list_start_16..__terrane_list_end_16 {
                prefix_index = __terrane_list_index_16;
                prefix_value = observe_prefix(prefix_index);
                __terrane_list_append_19.push(prefix_value);
            }
            prefix_index = std::cmp::max(__terrane_list_start_16, __terrane_list_end_16);
        } else {
            __terrane_list_append_19.reserve(__terrane_list_capacity_limit_16);
            while prefix_index < 3 {
                prefix_value = observe_prefix(prefix_index);
                __terrane_list_append_19.push(prefix_value);
                prefix_index = prefix_index + 1;
            }
        }
        let _ = &prefix_index;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(prefix_effects
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(prefix_effects
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        27 /* terrane-site: case.trn:228:33-228:50 */)), 27 /* terrane-site: case.trn:228:33-228:50 */)), terrane_scalar_support::scalar_text(&prefix_index)
    );
    fallback = terrane_collection_support::List::<i64>::new(Vec::new());
    fallback_index = 0;
    {
        let __terrane_list_append_20 = fallback.make_unique();
        let __terrane_list_start_17 = fallback_index;
        let __terrane_list_end_17 = 5 as i64;
        let __terrane_list_length_17 = (__terrane_list_start_17..__terrane_list_end_17)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_17 = 32usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_17 <= __terrane_list_capacity_limit_17 {
            __terrane_list_append_20.reserve(__terrane_list_length_17);
            for __terrane_list_index_17 in __terrane_list_start_17..__terrane_list_end_17 {
                fallback_index = __terrane_list_index_17;
                __terrane_list_append_20.push(fallback_index);
            }
            fallback_index = std::cmp::max(
                __terrane_list_start_17,
                __terrane_list_end_17,
            );
        } else {
            __terrane_list_append_20.reserve(__terrane_list_capacity_limit_17);
            while fallback_index < 5 {
                __terrane_list_append_20.push(fallback_index);
                fallback_index = fallback_index + 1;
            }
        }
        let _ = &fallback_index;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fallback
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(fallback
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        28 /* terrane-site: case.trn:235:27-235:38 */)), 28 /* terrane-site: case.trn:235:27-235:38 */)), terrane_scalar_support::scalar_text(&fallback_index)
    );
    captured = terrane_collection_support::List::<i64>::new(Vec::new());
    captured_index = 0;
    capturing_builder = {
        let mut captured = captured.clone();
        let mut captured_index = captured_index.clone();
        TerraneMutableCallable::new(move |(): ()| -> () {
            {
                let __terrane_list_append_21 = captured.make_unique();
                if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                    usize::try_from(captured_index),
                    usize::try_from(3 as i64),
                ) {
                    let __terrane_capacity_limit = 32usize
                        / std::mem::size_of::<i64>().max(1);
                    __terrane_list_append_21
                        .reserve(
                            __terrane_end
                                .saturating_sub(__terrane_start)
                                .min(__terrane_capacity_limit),
                        );
                }
                while captured_index < 3 {
                    __terrane_list_append_21.push(captured_index);
                    captured_index = __terrane_raised(
                        terrane_int_support::fixed_addition(captured_index, 1),
                        29 /* terrane-site: case.trn:242:7-242:23 */,
                    );
                }
            }
            ()
        })
    };
    capturing_builder.call(());
    captured_index = 0;
    capturing_builder.call(());
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(captured
        .length())), terrane_scalar_support::scalar_text(&captured_index)
    );
    local_mutation = {
        TerraneMutableCallable::new(move |(): ()| -> i64 {
            let mut local_value: i64;
            local_value = 0;
            local_value = __terrane_raised(
                terrane_int_support::fixed_addition(local_value, 1),
                30 /* terrane-site: case.trn:250:5-250:18 */,
            );
            return local_value;
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&local_mutation.call(())));
    returned_capture = terrane_collection_support::List::<i64>::new(vec![7, 8]);
    return_capture = {
        let mut returned_capture = returned_capture.clone();
        TerraneMutableCallable::new(move |
            (): (),
        | -> terrane_collection_support::List<i64> {
            returned_capture.append(9);
            return returned_capture.clone();
        })
    };
    first_returned = return_capture.call(());
    second_returned = return_capture.call(());
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(first_returned
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(first_returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        31 /* terrane-site: case.trn:260:33-260:50 */)), 31 /* terrane-site: case.trn:260:33-260:50 */)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(second_returned
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(second_returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        32 /* terrane-site: case.trn:260:76-260:94 */)), 32 /* terrane-site: case.trn:260:76-260:94 */))
    );
    inferred_capture = terrane_collection_support::List::<i64>::new(vec![2, 3]);
    inspect_capture = {
        let mut inferred_capture = inferred_capture.clone();
        TerraneMutableCallable::new(move |(): ()| -> i64 {
            let mut inferred_total: i64;
            let mut inferred_value: i64;
            inferred_total = __terrane_raised(
                terrane_int_support::coerce::<
                    i64,
                >(&terrane_int_support::Int::from(inferred_capture.length())),
                33 /* terrane-site: case.trn:264:28-264:51 */,
            );
            inferred_capture.append(4);
            let __terrane_iterable_18 = inferred_capture.clone();
            let mut __terrane_iterator_18 = terrane_collection_support::Iterable::terrane_iterator(
                &__terrane_iterable_18,
            );
            loop {
                inferred_value = match __terrane_iterator_18.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                inferred_total = __terrane_raised(
                    terrane_int_support::fixed_addition(inferred_total, inferred_value),
                    34 /* terrane-site: case.trn:267:24-267:55 */,
                );
            }
            return inferred_total;
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&inspect_capture.call(())));
    println!("{}", terrane_scalar_support::scalar_text(&validate_return()));
    validate_throw();
    validate_exit();
}
