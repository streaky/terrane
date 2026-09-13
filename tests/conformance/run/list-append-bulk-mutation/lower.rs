// Generated deterministically by Terrane <version>.
// Runtime support: platform_result_type.rs, platform_process.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: case.trn
// Namespace: list-append-bulk-mutation
fn validate_large_literal(enabled: bool) {
    if enabled {
        let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        let mut index: i64 = 0;
        {
            let __terrane_list_append_0 = values.make_unique();
            if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                usize::try_from(index),
                usize::try_from(4000000000 as i64),
            ) {
                let __terrane_capacity_limit = 268435456usize
                    / std::mem::size_of::<i64>().max(1);
                __terrane_list_append_0
                    .reserve(
                        __terrane_end
                            .saturating_sub(__terrane_start)
                            .min(__terrane_capacity_limit),
                    );
            }
            while index < 4000000000 {
                __terrane_list_append_0.push(index);
                index = index + 1;
            }
        }
    }
}
fn validate_return() -> terrane_int_support::Int {
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 100000000000000;
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
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 100000000000000;
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
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
                i64,
            >::new(Vec::new());
            let mut index: i64 = 0;
            let limit: i64 = 100000000000000;
            {
                let __terrane_list_append_3 = values.make_unique();
                while index < limit {
                    __terrane_list_append_3.push(index);
                    if index > 2 {
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_int_support::fixed_addition(index,
                            1), 4 /* terrane-site: case.trn:46:16-46:25 */))
                        );
                        return TerraneCompletion::Error(
                            TerraneError::raised(
                                TerraneErrorKind::ArithmeticOverflow,
                                5 /* terrane-site: case.trn:47:9-47:34 */,
                            ),
                        );
                    }
                    index = __terrane_raised_completion!(
                        terrane_int_support::fixed_addition(index, 1),
                        6 /* terrane-site: case.trn:48:7-48:14 */
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
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("throw-caught"))
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
}
fn main() {
    validate_large_literal(false);
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 4;
    {
        let __terrane_list_append_4 = values.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(index),
            usize::try_from(limit),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_4
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while index < limit {
            __terrane_list_append_4.push(index);
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                7 /* terrane-site: case.trn:59:5-59:12 */,
            );
        }
    }
    let original: terrane_collection_support::List<i64> = values.clone();
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
    let mut three_clause: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut for_index: i64 = 0;
    println!("{}", terrane_scalar_support::scalar_text(&for_index));
    for_index = 0;
    {
        let __terrane_list_append_5 = three_clause.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(for_index),
            usize::try_from(limit),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_5
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        '__terrane_break_0: while for_index < limit {
            '__terrane_continue_0: {
                __terrane_list_append_5.push(for_index);
            }
            for_index = __terrane_raised(
                terrane_int_support::fixed_addition(for_index, 1),
                8 /* terrane-site: case.trn:69:41-69:52 */,
            );
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(three_clause
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(three_clause
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        9 /* terrane-site: case.trn:71:31-71:46 */)), 9 /* terrane-site: case.trn:71:31-71:46 */)), terrane_scalar_support::scalar_text(&for_index)
    );
    let mut three_clause_break: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut break_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    {
        let __terrane_list_append_6 = three_clause_break.make_unique();
        '__terrane_break_1: while break_index.clone()
            < terrane_int_support::Int::from(10_i128)
        {
            '__terrane_continue_1: {
                __terrane_list_append_6.push(break_index.clone());
                if break_index.clone() > terrane_int_support::Int::from(2_i128) {
                    break '__terrane_break_1;
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
        10 /* terrane-site: case.trn:78:37-78:58 */)), 10 /* terrane-site: case.trn:78:37-78:58 */))
    );
    let mut update_observed: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut update_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    '__terrane_break_2: while update_index.clone()
        < terrane_int_support::Int::from(3_i128)
    {
        '__terrane_continue_2: {
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
    let mut condition_observed: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut condition_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    '__terrane_break_3: while condition_index.clone()
        < terrane_int_support::Int::from(4_i128)
            + terrane_int_support::Int::from(
                terrane_int_support::Int::from(condition_observed.length()),
            )
            - terrane_int_support::Int::from(
                terrane_int_support::Int::from(condition_observed.length()),
            )
    {
        '__terrane_continue_3: {
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
    let mut double_index: i64 = 0;
    println!("{}", terrane_scalar_support::scalar_text(&double_index));
    let mut double_update: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    double_index = 0;
    {
        let __terrane_list_append_7 = double_update.make_unique();
        '__terrane_break_4: while double_index < limit {
            '__terrane_continue_4: {
                __terrane_list_append_7.push(double_index);
                double_index = __terrane_raised(
                    terrane_int_support::fixed_addition(double_index, 1),
                    11 /* terrane-site: case.trn:95:5-95:19 */,
                );
            }
            double_index = __terrane_raised(
                terrane_int_support::fixed_addition(double_index, 1),
                12 /* terrane-site: case.trn:93:47-93:61 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(double_update
        .length()))
    );
    let mut dependent: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut dependent_index: i64 = 0;
    while dependent_index < 3 {
        dependent
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(&terrane_int_support::Int::from(dependent.length())),
                    13 /* terrane-site: case.trn:101:23-101:39 */,
                ),
            );
        dependent_index = dependent_index + 1;
    }
    let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
        &dependent,
    );
    loop {
        let value = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let mut inner_index: i64 = 0;
    while inner_index < 1 {
        let mut inner: terrane_collection_support::List<i64> = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        inner.append(inner_index);
        inner_index = inner_index + 1;
    }
    let mut nested: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut outer_index: i64 = 0;
    {
        let __terrane_list_append_8 = nested.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(outer_index),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
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
            let mut nested_index: i64 = 0;
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
    let mut early_exit: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut early_index: i64 = 0;
    let early_limit: i64 = 100000000000000;
    {
        let __terrane_list_append_9 = early_exit.make_unique();
        while early_index < early_limit {
            __terrane_list_append_9.push(early_index);
            if early_index > 2 {
                break;
            }
            early_index = __terrane_raised(
                terrane_int_support::fixed_addition(early_index, 1),
                14 /* terrane-site: case.trn:131:5-131:18 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(early_exit
        .length()))
    );
    let mut nested_break: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut break_outer: i64 = 0;
    {
        let __terrane_list_append_10 = nested_break.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(break_outer),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
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
            let break_inner: i64 = 0;
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
    let source: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2, 3, 4]);
    let mut collected: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &source,
    );
    {
        let __terrane_list_append_11 = collected.make_unique();
        loop {
            let value = match __terrane_iterator_6.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_11.push(value);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(collected
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(collected
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        15 /* terrane-site: case.trn:148:28-148:40 */)), 15 /* terrane-site: case.trn:148:28-148:40 */))
    );
    let mut observed: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut __terrane_iterator_7 = terrane_collection_support::Iterable::terrane_iterator(
        &source,
    );
    loop {
        let value = match __terrane_iterator_7.next() {
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
                    16 /* terrane-site: case.trn:152:22-152:37 */,
                ),
            );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(observed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        17 /* terrane-site: case.trn:153:10-153:21 */)), 17 /* terrane-site: case.trn:153:10-153:21 */))
    );
    let rows: terrane_collection_support::List<terrane_collection_support::List<i64>> = terrane_collection_support::List::<
        terrane_collection_support::List<i64>,
    >::new(
        vec![
            terrane_collection_support::List::< i64 >::new(vec![1, 2]),
            terrane_collection_support::List::< i64 >::new(vec![3])
        ],
    );
    let mut flattened: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut __terrane_iterator_8 = terrane_collection_support::Iterable::terrane_iterator(
        &rows,
    );
    {
        let __terrane_list_append_12 = flattened.make_unique();
        loop {
            let row = match __terrane_iterator_8.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let mut __terrane_iterator_9 = terrane_collection_support::Iterable::terrane_iterator(
                &row,
            );
            loop {
                let value = match __terrane_iterator_9.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                __terrane_list_append_12.push(value);
            }
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(flattened
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(flattened
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        18 /* terrane-site: case.trn:160:28-160:40 */)), 18 /* terrane-site: case.trn:160:28-160:40 */))
    );
    let mut aliased: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2]);
    let alias: terrane_collection_support::List<i64> = aliased.clone();
    let mut __terrane_iterator_10 = terrane_collection_support::Iterable::terrane_iterator(
        &alias,
    );
    {
        let __terrane_list_append_13 = aliased.make_unique();
        loop {
            let value = match __terrane_iterator_10.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
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
    let mut self_source: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2]);
    let mut __terrane_iterator_11 = terrane_collection_support::Iterable::terrane_iterator(
        &self_source,
    );
    loop {
        let value = match __terrane_iterator_11.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        self_source.append(value);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(self_source
        .length()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&validate_return()));
    validate_throw();
    validate_exit();
}
// Source: core/process.trn
// Namespace: core/process
#[derive(Clone)]
pub struct NativeString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl NativeString {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        value.construct(encoded);
        value
    }
    pub fn construct(&mut self, encoded: String) {
        self.is_text = terrane_platform_value_is_text(&encoded);
        self.text = terrane_platform_value_text(&encoded);
        self.raw = terrane_platform_value_bytes(&encoded);
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: NativeString,
    pub value: NativeString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: NativeString, entry_value: NativeString) -> Self {
        let mut value = Self {
            name: NativeString::terrane_construct(String::from("text:")),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: NativeString, entry_value: NativeString) {
        self.name = name.clone();
        self.value = entry_value.clone();
    }
}
#[derive(Clone)]
pub struct ProcessHostNameResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: NativeString,
}
impl ProcessHostNameResult {
    pub fn terrane_construct(
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) -> Self {
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(did_fail, is_available, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) {
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value.clone();
    }
}
pub fn process_host_name() -> ProcessHostNameResult {
    let raw: TerranePlatformResult = terrane_platform_support::system_host_name();
    return ProcessHostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        NativeString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<NativeString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = values.make_unique();
        while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
            __terrane_list_append_0
                .push(
                    NativeString::terrane_construct(
                        __terrane_raised(
                            encoded
                                .get(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        19 /* terrane-site: core/process.trn:44:49-44:63 */,
                                    ),
                                )
                                .cloned()
                                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        19 /* terrane-site: core/process.trn:44:49-44:63 */,
                                    ),
                                )),
                            19 /* terrane-site: core/process.trn:44:49-44:63 */,
                        ),
                    ),
                );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = values.make_unique();
        while index.clone() + terrane_int_support::Int::from(1_i128)
            < terrane_int_support::Int::from(encoded.len() as i128)
        {
            let name: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                20 /* terrane-site: core/process.trn:53:40-53:54 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                20 /* terrane-site: core/process.trn:53:40-53:54 */,
                            ),
                        )),
                    20 /* terrane-site: core/process.trn:53:40-53:54 */,
                ),
            );
            let value: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                21 /* terrane-site: core/process.trn:54:41-54:59 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                21 /* terrane-site: core/process.trn:54:41-54:59 */,
                            ),
                        )),
                    21 /* terrane-site: core/process.trn:54:41-54:59 */,
                ),
            );
            __terrane_list_append_1
                .push(EnvironmentEntry::terrane_construct(name, value));
            index = index.clone() + terrane_int_support::Int::from(2_i128);
        }
    }
    return values.clone();
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(declared);
        value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared.clone();
    }
}
#[derive(Clone)]
pub struct CommandLine {
    pub flags: terrane_collection_support::List<String>,
    pub option_names: terrane_collection_support::List<String>,
    pub option_values: terrane_collection_support::List<NativeString>,
    pub positionals: terrane_collection_support::List<NativeString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                NativeString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                NativeString,
            >::new(Vec::new()),
            diagnostic_arguments: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            diagnostic_messages: terrane_collection_support::List::<
                String,
            >::new(Vec::new()),
        }
    }
}
pub fn schema_has(schema: CliSchema, sought: String) -> bool {
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &schema.entries,
    );
    loop {
        let entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry == sought {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<NativeString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_2 = diagnostic_arguments.make_unique();
        let __terrane_list_append_3 = diagnostic_messages.make_unique();
        let __terrane_list_append_4 = flags.make_unique();
        let __terrane_list_append_5 = option_names.make_unique();
        let __terrane_list_append_6 = option_values.make_unique();
        let __terrane_list_append_7 = positionals.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            let argument: NativeString = __terrane_raised(
                supplied
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            22 /* terrane-site: core/process.trn:89:20-89:35 */,
                        ),
                    ),
                22 /* terrane-site: core/process.trn:89:20-89:35 */,
            );
            if !argument.is_text {
                __terrane_list_append_2.push(index.clone());
                __terrane_list_append_3
                    .push(String::from("command-line option is not Unicode text"));
            } else {
                let flag_entry: String = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                    terrane_scalar_support::scalar_text(&argument.text)
                );
                let value_entry: String = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                    terrane_scalar_support::scalar_text(&argument.text)
                );
                if schema_has(schema.clone(), flag_entry) {
                    __terrane_list_append_4.push(argument.text.clone());
                } else if schema_has(schema.clone(), value_entry) {
                    if index.clone() + terrane_int_support::Int::from(1_i128)
                        >= terrane_int_support::Int::from(
                            terrane_int_support::Int::from(supplied.length()),
                        )
                    {
                        __terrane_list_append_2.push(index.clone());
                        __terrane_list_append_3
                            .push(String::from("option requires a value"));
                    } else {
                        __terrane_list_append_5.push(argument.text.clone());
                        __terrane_list_append_6
                            .push(
                                __terrane_raised(
                                    supplied
                                        .get_or_error(
                                            __terrane_raised(
                                                terrane_collection_support::index_from_int(
                                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                                ),
                                                23 /* terrane-site: core/process.trn:104:43-104:62 */,
                                            ),
                                        ),
                                    23 /* terrane-site: core/process.trn:104:43-104:62 */,
                                ),
                            );
                        index = index.clone() + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument.text.starts_with(&String::from("--")) {
                    __terrane_list_append_2.push(index.clone());
                    __terrane_list_append_3.push(String::from("unknown option"));
                } else {
                    __terrane_list_append_7.push(argument.clone());
                }
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut result: CommandLine = CommandLine::terrane_construct();
    result.flags = flags.clone();
    result.option_names = option_names.clone();
    result.option_values = option_values.clone();
    result.positionals = positionals.clone();
    result.diagnostic_arguments = diagnostic_arguments.clone();
    result.diagnostic_messages = diagnostic_messages.clone();
    return result.clone();
}
#[derive(Clone)]
pub struct ExitStatus {
    pub code: terrane_int_support::Int,
    pub valid: bool,
}
impl ExitStatus {
    pub fn terrane_construct() -> Self {
        Self {
            code: terrane_int_support::Int::from(0_i128),
            valid: true,
        }
    }
}
pub fn make_exit_status(requested: terrane_int_support::Int) -> ExitStatus {
    let mut result: ExitStatus = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result.code = terrane_int_support::Int::from(255_i128);
        result.valid = false;
    } else {
        result.code = requested.clone();
    }
    return result.clone();
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
