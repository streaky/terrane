// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: byte-index-and-slice
fn main() {
    let data: Vec<u8>;
    let middle: Vec<u8>;
    let stepped: Vec<u8>;
    let empty: Vec<u8>;
    let __trn_66696e616c: Vec<u8>;
    let mut failure: Option<TerraneError> = None;
    data = {
        let __terrane_receiver_value = &String::from("A👍");
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        terrane_string_support::encode(
            &__terrane_receiver_value,
            __terrane_string_argument_0,
        )
    };
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&data,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:8:10-8:17 */)), 0 /* terrane-site: case.trn:8:10-8:17 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&data,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:8:19-8:26 */)), 1 /* terrane-site: case.trn:8:19-8:26 */))
    );
    middle = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(1_i128),
                    terrane_int_support::Int::from(5_i128),
                    terrane_int_support::Int::from(1_i64),
                ),
                2 /* terrane-site: case.trn:9:23-9:34 */,
            ),
        ),
        3 /* terrane-site: case.trn:9:18-9:35 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised(terrane_string_support::decode(&middle,
        __terrane_string_argument_0), 4 /* terrane-site: case.trn:10:11-10:30 */) })
    );
    stepped = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(0_i128),
                    terrane_int_support::Int::from(5_i128),
                    terrane_int_support::Int::from(2_i128),
                ),
                5 /* terrane-site: case.trn:11:24-11:38 */,
            ),
        ),
        6 /* terrane-site: case.trn:11:19-11:39 */,
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&stepped,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:12:10-12:20 */)), 7 /* terrane-site: case.trn:12:10-12:20 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&stepped,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        8 /* terrane-site: case.trn:12:22-12:32 */)), 8 /* terrane-site: case.trn:12:22-12:32 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&stepped,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        9 /* terrane-site: case.trn:12:34-12:44 */)), 9 /* terrane-site: case.trn:12:34-12:44 */))
    );
    empty = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(data.len() as i128),
                    terrane_int_support::Int::from(data.len() as i128),
                    terrane_int_support::Int::from(1_i64),
                ),
                10 /* terrane-site: case.trn:13:22-13:53 */,
            ),
        ),
        11 /* terrane-site: case.trn:13:17-13:54 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(empty.len() as i128)));
    __trn_66696e616c = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::through(
                    terrane_int_support::Int::from(4_i128),
                    terrane_int_support::Int::from(4_i128),
                    terrane_int_support::Int::from(1_i64),
                ),
                12 /* terrane-site: case.trn:15:22-15:41 */,
            ),
        ),
        13 /* terrane-site: case.trn:15:17-15:42 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&__trn_66696e616c,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        14 /* terrane-site: case.trn:16:10-16:18 */)), 14 /* terrane-site: case.trn:16:10-16:18 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_collection_support::byte_at(&data,
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(data
                .len() as i128)), 15 /* terrane-site: case.trn:18:12-18:29 */)),
                15 /* terrane-site: case.trn:18:12-18:29 */))
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
                    println!("{}", String::from("index"));
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
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&(__terrane_raised_completion!(terrane_collection_support::byte_slice(&data,
                &__terrane_raised_completion!(terrane_collection_support::Range::new(terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(6_i128),
                terrane_int_support::Int::from(1_i64)), 16 /* terrane-site: case.trn:22:17-22:28 */)), 17 /* terrane-site: case.trn:22:12-22:29 */) .len() as i128))
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
                    && __terrane_error_1.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("slice"));
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
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_collection_support::byte_at(&data,
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(-
                1_i128)), 18 /* terrane-site: case.trn:26:12-26:21 */)),
                18 /* terrane-site: case.trn:26:12-26:21 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_2 = true;
                    let _ = failure.insert(__terrane_error_2.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
