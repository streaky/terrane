// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: stable-list-sorting
fn main() {
    let mut adaptive: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from_decimal("900000000000000000000000000000000000000"),
            terrane_int_support::Int::from(- 2_i128),
            terrane_int_support::Int::from(7_i128),
            terrane_int_support::Int::from(7_i128)
        ],
    );
    adaptive.sort_by(|left, right| left.cmp(right));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:7:10-7:21 */)), 0 /* terrane-site: case.trn:7:10-7:21 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        1 /* terrane-site: case.trn:7:23-7:34 */)), 1 /* terrane-site: case.trn:7:23-7:34 */))
    );
    adaptive.sort_by(|left, right| right.cmp(left));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:9:10-9:21 */)), 2 /* terrane-site: case.trn:9:10-9:21 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        3 /* terrane-site: case.trn:9:23-9:34 */)), 3 /* terrane-site: case.trn:9:23-9:34 */))
    );
    let mut int8s: terrane_collection_support::List<i8> = terrane_collection_support::List::<
        i8,
    >::new(vec![2, - 1]);
    let mut int16s: terrane_collection_support::List<i16> = terrane_collection_support::List::<
        i16,
    >::new(vec![2, - 1]);
    let mut int32s: terrane_collection_support::List<i32> = terrane_collection_support::List::<
        i32,
    >::new(vec![2, - 1]);
    let mut int64s: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![2, - 1]);
    let mut int128s: terrane_collection_support::List<i128> = terrane_collection_support::List::<
        i128,
    >::new(vec![2, - 1]);
    let mut uint8s: terrane_collection_support::List<u8> = terrane_collection_support::List::<
        u8,
    >::new(vec![2, 1]);
    let mut uint16s: terrane_collection_support::List<u16> = terrane_collection_support::List::<
        u16,
    >::new(vec![2, 1]);
    let mut uint32s: terrane_collection_support::List<u32> = terrane_collection_support::List::<
        u32,
    >::new(vec![2, 1]);
    let mut uint64s: terrane_collection_support::List<u64> = terrane_collection_support::List::<
        u64,
    >::new(vec![2, 1]);
    let mut uint128s: terrane_collection_support::List<u128> = terrane_collection_support::List::<
        u128,
    >::new(vec![2, 1]);
    int8s.sort_by(|left, right| left.cmp(right));
    int16s.sort_by(|left, right| left.cmp(right));
    int32s.sort_by(|left, right| left.cmp(right));
    int64s.sort_by(|left, right| left.cmp(right));
    int128s.sort_by(|left, right| left.cmp(right));
    uint8s.sort_by(|left, right| left.cmp(right));
    uint16s.sort_by(|left, right| left.cmp(right));
    uint32s.sort_by(|left, right| left.cmp(right));
    uint64s.sort_by(|left, right| left.cmp(right));
    uint128s.sort_by(|left, right| left.cmp(right));
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(int8s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:31:10-31:18 */)), 4 /* terrane-site: case.trn:31:10-31:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int16s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:31:20-31:29 */)), 5 /* terrane-site: case.trn:31:20-31:29 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int32s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:31:31-31:40 */)), 6 /* terrane-site: case.trn:31:31-31:40 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int64s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:31:42-31:51 */)), 7 /* terrane-site: case.trn:31:42-31:51 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int128s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        8 /* terrane-site: case.trn:31:53-31:63 */)), 8 /* terrane-site: case.trn:31:53-31:63 */))
    );
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(uint8s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        9 /* terrane-site: case.trn:32:10-32:19 */)), 9 /* terrane-site: case.trn:32:10-32:19 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint16s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        10 /* terrane-site: case.trn:32:21-32:31 */)), 10 /* terrane-site: case.trn:32:21-32:31 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint32s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        11 /* terrane-site: case.trn:32:33-32:43 */)), 11 /* terrane-site: case.trn:32:33-32:43 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint64s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        12 /* terrane-site: case.trn:32:45-32:55 */)), 12 /* terrane-site: case.trn:32:45-32:55 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint128s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        13 /* terrane-site: case.trn:32:57-32:68 */)), 13 /* terrane-site: case.trn:32:57-32:68 */))
    );
    let mut words: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("é"), String::from("e"), String::from("z")]);
    words.sort_by(|left, right| left.cmp(right));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        14 /* terrane-site: case.trn:36:10-36:18 */)), 14 /* terrane-site: case.trn:36:10-36:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        15 /* terrane-site: case.trn:36:20-36:28 */)), 15 /* terrane-site: case.trn:36:20-36:28 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        16 /* terrane-site: case.trn:36:30-36:38 */)), 16 /* terrane-site: case.trn:36:30-36:38 */))
    );
    words.sort_by(|left, right| right.cmp(left));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        17 /* terrane-site: case.trn:38:10-38:18 */)), 17 /* terrane-site: case.trn:38:10-38:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        18 /* terrane-site: case.trn:38:20-38:28 */)), 18 /* terrane-site: case.trn:38:20-38:28 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        19 /* terrane-site: case.trn:38:30-38:38 */)), 19 /* terrane-site: case.trn:38:30-38:38 */))
    );
    let zero: f64 = 0.0;
    let negative_zero: f64 = -0.0_f64;
    let one: f64 = 1.0;
    let negative_one: f64 = -1.0_f64;
    let infinity: f64 = one / zero;
    let negative_infinity: f64 = negative_one / zero;
    let first_nan: f64 = zero / zero;
    let second_nan: f64 = negative_zero / zero;
    let mut floats: terrane_collection_support::List<f64> = terrane_collection_support::List::<
        f64,
    >::new(
        vec![first_nan, negative_zero, infinity, zero, negative_infinity, second_nan],
    );
    floats.sort_by(terrane_collection_support::compare_float64_ascending);
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        20 /* terrane-site: case.trn:50:10-50:19 */)), 20 /* terrane-site: case.trn:50:10-50:19 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        21 /* terrane-site: case.trn:50:30-50:39 */)), 21 /* terrane-site: case.trn:50:30-50:39 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        22 /* terrane-site: case.trn:50:55-50:64 */)), 22 /* terrane-site: case.trn:50:55-50:64 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        23 /* terrane-site: case.trn:50:80-50:89 */)), 23 /* terrane-site: case.trn:50:80-50:89 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        24 /* terrane-site: case.trn:50:105-50:114 */)), 24 /* terrane-site: case.trn:50:105-50:114 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        25 /* terrane-site: case.trn:50:125-50:134 */)), 25 /* terrane-site: case.trn:50:125-50:134 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        26 /* terrane-site: case.trn:50:149-50:158 */)), 26 /* terrane-site: case.trn:50:149-50:158 */).is_nan())
    );
    floats.sort_by(terrane_collection_support::compare_float64_descending);
    println!(
        "{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        27 /* terrane-site: case.trn:52:10-52:19 */)), 27 /* terrane-site: case.trn:52:10-52:19 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        28 /* terrane-site: case.trn:52:30-52:39 */)), 28 /* terrane-site: case.trn:52:30-52:39 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        29 /* terrane-site: case.trn:52:55-52:64 */)), 29 /* terrane-site: case.trn:52:55-52:64 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        30 /* terrane-site: case.trn:52:80-52:89 */)), 30 /* terrane-site: case.trn:52:80-52:89 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        31 /* terrane-site: case.trn:52:105-52:114 */)), 31 /* terrane-site: case.trn:52:105-52:114 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        32 /* terrane-site: case.trn:52:125-52:134 */)), 32 /* terrane-site: case.trn:52:125-52:134 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        33 /* terrane-site: case.trn:52:150-52:159 */)), 33 /* terrane-site: case.trn:52:150-52:159 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        34 /* terrane-site: case.trn:52:174-52:183 */)), 34 /* terrane-site: case.trn:52:174-52:183 */).is_nan())
    );
    let mut narrow: terrane_collection_support::List<f32> = terrane_collection_support::List::<
        f32,
    >::new(vec![2.0_f32, - 1.0_f32]);
    narrow.sort_by(terrane_collection_support::compare_float32_ascending);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        35 /* terrane-site: case.trn:56:10-56:19 */)), 35 /* terrane-site: case.trn:56:10-56:19 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        36 /* terrane-site: case.trn:56:21-56:30 */)), 36 /* terrane-site: case.trn:56:21-56:30 */))
    );
    let mut values: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let preserved: terrane_collection_support::List<terrane_int_support::Int> = values
        .clone();
    let mut returned: terrane_collection_support::List<terrane_int_support::Int> = {
        let collection = &mut values;
        collection.sort_by(|left, right| left.cmp(right));
        collection.clone()
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(preserved
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        37 /* terrane-site: case.trn:61:10-61:22 */)), 37 /* terrane-site: case.trn:61:10-61:22 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        38 /* terrane-site: case.trn:61:24-61:33 */)), 38 /* terrane-site: case.trn:61:24-61:33 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        39 /* terrane-site: case.trn:61:35-61:46 */)), 39 /* terrane-site: case.trn:61:35-61:46 */))
    );
    returned.append(terrane_int_support::Int::from(4_i128));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(returned
        .length()))
    );
    let indexed: terrane_collection_support::Map<String, terrane_int_support::Int> = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("c"),
            terrane_int_support::Int::from(3_i128)),
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("b"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    let mut keys: terrane_collection_support::List<String> = indexed.keys();
    let ascending: terrane_collection_support::List<String> = {
        let collection = &mut keys;
        collection.sort_by(|left, right| left.cmp(right));
        collection.clone()
    };
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &ascending,
    );
    loop {
        let key = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&__terrane_raised(indexed
            .get_or_error(&key), 40 /* terrane-site: case.trn:69:17-69:29 */))
        );
    }
    let descending: terrane_collection_support::List<String> = {
        let collection = &mut keys;
        collection.sort_by(|left, right| right.cmp(left));
        collection.clone()
    };
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &descending,
    );
    loop {
        let key = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&__terrane_raised(indexed
            .get_or_error(&key), 41 /* terrane-site: case.trn:72:17-72:29 */))
        );
    }
    let mut empty: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    empty.sort_by(|left, right| left.cmp(right));
    let mut single: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    single.sort_by(|left, right| right.cmp(left));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(single
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        42 /* terrane-site: case.trn:78:24-78:33 */)), 42 /* terrane-site: case.trn:78:24-78:33 */))
    );
}
