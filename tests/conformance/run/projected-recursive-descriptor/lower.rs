// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct TransferBox {
    pub value: Option<String>,
}
impl TransferBox {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("captured")),
        }
    }
}
fn configuration() -> Result<Descriptor, TerraneError> {
    let nested: terrane_collection_support::Tuple<Option<u32>>;
    let window: terrane_collection_support::Tuple<u32>;
    nested = terrane_collection_support::Tuple::<
        Option<u32>,
    >::new(vec![Some(2), Some(3)]);
    window = terrane_collection_support::Tuple::<u32>::new(vec![800, 600]);
    return Ok({
        let __terrane_conversion_input_5 = Some(window);
        let __terrane_conversion_input_6 = Some(nested);
        let __terrane_conversion_input_0 = __terrane_raised_err(
            path_value(),
            0 /* terrane-site: src/main.trn:17:80-17:91 */,
        )?;
        let __terrane_conversion_input_1 = __terrane_raised_err(
            timeout_value(),
            1 /* terrane-site: src/main.trn:17:105-17:119 */,
        )?;
        let __terrane_conversion_input_7 = Some(
            terrane_collection_support::UnorderedMap::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::new(String::from("mode"),
                    String::from("fast")),
                    terrane_collection_support::Entry::new(String::from("speed"),
                    String::from("quick"))
                ],
            ),
        );
        let __terrane_conversion_input_2 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("alpha")]);
        let __terrane_conversion_input_3 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("one"), String::from("two")]);
        let __terrane_conversion_input_8 = Some(String::from("ready"));
        let __terrane_conversion_input_4 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("left"), String::from("right")]);
        __terrane_raised_err(
            Descriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
                __terrane_conversion_input_2,
                __terrane_conversion_input_3,
                __terrane_conversion_input_4,
                __terrane_conversion_input_5,
                __terrane_conversion_input_6,
                __terrane_conversion_input_7,
                __terrane_conversion_input_8,
            ),
            2 /* terrane-site: src/main.trn:17:12-17:279 */,
        )?
    });
}
fn configuration_for(
    window: terrane_collection_support::Tuple<u32>,
) -> Result<Descriptor, TerraneError> {
    return Ok({
        let __terrane_conversion_input_5 = Some(window);
        let __terrane_conversion_input_6 = None;
        let __terrane_conversion_input_0 = __terrane_raised_err(
            path_value(),
            3 /* terrane-site: src/main.trn:20:78-20:89 */,
        )?;
        let __terrane_conversion_input_1 = __terrane_raised_err(
            timeout_value(),
            4 /* terrane-site: src/main.trn:20:103-20:117 */,
        )?;
        let __terrane_conversion_input_7 = None;
        let __terrane_conversion_input_2 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("alpha")]);
        let __terrane_conversion_input_3 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("kept")]);
        let __terrane_conversion_input_8 = None;
        let __terrane_conversion_input_4 = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("left")]);
        __terrane_raised_err(
            Descriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
                __terrane_conversion_input_2,
                __terrane_conversion_input_3,
                __terrane_conversion_input_4,
                __terrane_conversion_input_5,
                __terrane_conversion_input_6,
                __terrane_conversion_input_7,
                __terrane_conversion_input_8,
            ),
            5 /* terrane-site: src/main.trn:20:12-20:220 */,
        )?
    });
}
fn main() {
    let config: Descriptor;
    let config2: Descriptor;
    let config3: Descriptor;
    let unique: UniqueDescriptor;
    let result: String;
    let failing: UniqueDescriptor;
    let mut __trn_626f78: TransferBox;
    let extracted: Option<String>;
    let pair: terrane_collection_support::Tuple<String>;
    let rows: terrane_collection_support::List<
        terrane_collection_support::Tuple<String>,
    >;
    let first: Observed;
    let second: Observed;
    let mixed: MixedDescriptor;
    let window: terrane_collection_support::Tuple<u32>;
    let reused: Descriptor;
    let repeated: Descriptor;
    let retained_name: String;
    let numbers: terrane_collection_support::Tuple<terrane_int_support::Int>;
    let too_short: terrane_collection_support::Tuple<u32>;
    let too_long: terrane_collection_support::Tuple<u32>;
    let tickets: terrane_collection_support::Tuple<Ticket>;
    config = __terrane_traced(
        configuration(),
        6 /* terrane-site: src/main.trn:23:25-23:39 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(inspect(&config),
        7 /* terrane-site: src/main.trn:24:13-24:28 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(inspect_shared(&config),
        8 /* terrane-site: src/main.trn:25:13-25:35 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_descriptor_owner_1536
        = &config; let __terrane_descriptor_view_1536 =
        terrane_recursive_descriptor_witness::Descriptor { path :
        __terrane_descriptor_owner_1536.path.clone(), timeout :
        __terrane_descriptor_owner_1536.timeout, args : __terrane_descriptor_owner_1536
        .args.iter().map(| item | std::ffi::OsStr::new(&* item)).collect(), names :
        __terrane_descriptor_owner_1536.names.iter().map(| item | (* item).as_str())
        .collect(), strings : __terrane_descriptor_owner_1536.strings.as_slice(),
        window_size : __terrane_descriptor_owner_1536.window_size, nested :
        __terrane_descriptor_owner_1536.nested, labels : __terrane_descriptor_owner_1536
        .labels.clone(), note : __terrane_descriptor_owner_1536.note.as_ref().map(| value
        | (* value).as_str()) }; __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_descriptor_view_1536.inspect_method()) }))
        { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_recursive_descriptor_witness",
        "terrane_recursive_descriptor_witness::Descriptor::inspect_method")) }; Ok:: < _,
        crate ::TerraneForeignError > (value) } } (), 9 /* terrane-site: src/main.trn:26:13-26:35 */) })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_466163746f7279_inspect_static(&config),
        10 /* terrane-site: src/main.trn:27:13-27:44 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                fail_early(&config), 11 /* terrane-site: src/main.trn:29:9-29:27 */
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
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("early-failure"));
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
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        config; __terrane_raised(consume(__terrane_conversion_input_0),
        12 /* terrane-site: src/main.trn:32:13-32:33 */) })
    );
    config2 = __terrane_traced(
        configuration(),
        13 /* terrane-site: src/main.trn:33:26-33:40 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_descriptor_owner_1784
        = config2; let __terrane_descriptor_view_1784 =
        terrane_recursive_descriptor_witness::Descriptor { path :
        __terrane_descriptor_owner_1784.path, timeout : __terrane_descriptor_owner_1784
        .timeout, args : __terrane_descriptor_owner_1784.args.iter().map(| item |
        std::ffi::OsStr::new(&* item)).collect(), names : __terrane_descriptor_owner_1784
        .names.iter().map(| item | (* item).as_str()).collect(), strings :
        __terrane_descriptor_owner_1784.strings.as_slice(), window_size :
        __terrane_descriptor_owner_1784.window_size, nested :
        __terrane_descriptor_owner_1784.nested, labels : __terrane_descriptor_owner_1784
        .labels, note : __terrane_descriptor_owner_1784.note.as_ref().map(| value | (*
        value).as_str()) }; __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_descriptor_view_1784.consume_method()) }))
        { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_recursive_descriptor_witness",
        "terrane_recursive_descriptor_witness::Descriptor::consume_method")) }; Ok:: < _,
        crate ::TerraneForeignError > (value) } } (), 14 /* terrane-site: src/main.trn:34:13-34:36 */) })
    );
    config3 = __terrane_traced(
        configuration(),
        15 /* terrane-site: src/main.trn:35:26-35:40 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        config3;
        __terrane_raised(terrane_static_trn_466163746f7279_consume_static(__terrane_conversion_input_0),
        16 /* terrane-site: src/main.trn:36:13-36:50 */) })
    );
    unique = {
        let __terrane_conversion_input_0 = __terrane_raised(
            ticket(),
            17 /* terrane-site: src/main.trn:37:51-37:58 */,
        );
        let __terrane_conversion_input_1 = String::from("unique");
        __terrane_raised(
            UniqueDescriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            18 /* terrane-site: src/main.trn:37:14-37:76 */,
        )
    };
    result = {
        let __terrane_conversion_input_0 = unique;
        __terrane_raised(
            take_unique(__terrane_conversion_input_0),
            19 /* terrane-site: src/main.trn:38:14-38:38 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&result));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(ticket_drops(),
        20 /* terrane-site: src/main.trn:40:13-40:26 */))
    );
    failing = {
        let __terrane_conversion_input_0 = __terrane_raised(
            ticket(),
            21 /* terrane-site: src/main.trn:41:52-41:59 */,
        );
        let __terrane_conversion_input_1 = String::from("failure");
        __terrane_raised(
            UniqueDescriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
            ),
            22 /* terrane-site: src/main.trn:41:15-41:78 */,
        )
    };
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            {
                let __terrane_conversion_input_0 = failing;
                __terrane_raised_completion!(
                    fail_unique(__terrane_conversion_input_0), 23 /* terrane-site: src/main.trn:43:9-43:34 */
                )
            };
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
                    && __terrane_error_1.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("unique-failure"));
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
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(ticket_drops(),
        24 /* terrane-site: src/main.trn:46:13-46:26 */))
    );
    __trn_626f78 = TransferBox::terrane_construct();
    extracted = std::mem::take(&mut __trn_626f78.value);
    println!(
        "{}", { let mut __terrane_text = String::new(); match &extracted {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(&__trn_626f78.value).is_none())
    );
    pair = terrane_collection_support::Tuple::<
        String,
    >::new(vec![String::from("borrowed"), String::from("owned")]);
    rows = terrane_collection_support::List::<
        terrane_collection_support::Tuple<String>,
    >::new(
        vec![
            terrane_collection_support::Tuple::< String >::new(vec![String::from("row"),
            String::from("data")])
        ],
    );
    first = {
        let __terrane_conversion_input_0 = String::from("one");
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Observed {
                    name: __terrane_conversion_input_0,
                })
            }(),
            25 /* terrane-site: src/main.trn:53:13-53:44 */,
        )
    };
    second = {
        let __terrane_conversion_input_0 = String::from("two");
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(terrane_recursive_descriptor_witness::Observed {
                    name: __terrane_conversion_input_0,
                })
            }(),
            26 /* terrane-site: src/main.trn:54:14-54:45 */,
        )
    };
    mixed = {
        let __terrane_conversion_input_2 = Some(pair);
        let __terrane_conversion_input_0 = rows.clone();
        let __terrane_conversion_input_1 = terrane_collection_support::UnorderedMap::<
            String,
            Observed,
        >::new(
            vec![
                terrane_collection_support::Entry::new(String::from("first"), first),
                terrane_collection_support::Entry::new(String::from("second"), second)
            ],
        );
        __terrane_raised(
            MixedDescriptor::terrane_construct(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
                __terrane_conversion_input_2,
            ),
            27 /* terrane-site: src/main.trn:55:13-55:116 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(inspect_mixed(&mixed),
        28 /* terrane-site: src/main.trn:56:13-56:33 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
        .length()))
    );
    __terrane_raised(reset_clones(), 29 /* terrane-site: src/main.trn:58:5-58:18 */);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        mixed; __terrane_raised(take_mixed(__terrane_conversion_input_0),
        30 /* terrane-site: src/main.trn:59:13-59:35 */) })
    );
    window = terrane_collection_support::Tuple::<u32>::new(vec![800, 600]);
    reused = __terrane_traced(
        configuration_for(window.clone()),
        31 /* terrane-site: src/main.trn:61:25-61:50 */,
    );
    repeated = __terrane_traced(
        configuration_for(window),
        32 /* terrane-site: src/main.trn:62:27-62:52 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(inspect(&reused),
        33 /* terrane-site: src/main.trn:63:13-63:28 */))
    );
    retained_name = __terrane_raised(
        borrow_name(&reused),
        34 /* terrane-site: src/main.trn:64:21-64:40 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        reused; __terrane_raised(consume(__terrane_conversion_input_0),
        35 /* terrane-site: src/main.trn:65:13-65:33 */) })
    );
    println!("{}", terrane_scalar_support::scalar_text(&retained_name));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        repeated; __terrane_raised(consume(__terrane_conversion_input_0),
        36 /* terrane-site: src/main.trn:67:13-67:35 */) })
    );
    numbers = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(800_i128),
            terrane_int_support::Int::from(600_i128)
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        numbers.clone(); __terrane_raised(pair_sum(__terrane_conversion_input_0),
        37 /* terrane-site: src/main.trn:69:13-69:30 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        numbers.clone(); __terrane_raised(pair_sum(__terrane_conversion_input_0),
        38 /* terrane-site: src/main.trn:70:13-70:30 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        numbers.clone(); __terrane_raised(window_sum(__terrane_conversion_input_0),
        39 /* terrane-site: src/main.trn:71:13-71:32 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        numbers; __terrane_raised(window_sum(__terrane_conversion_input_0),
        40 /* terrane-site: src/main.trn:72:13-72:32 */) })
    );
    too_short = terrane_collection_support::Tuple::<u32>::new(vec![800]);
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                configuration_for(too_short), 41 /* terrane-site: src/main.trn:75:9-75:37 */
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
                    && __terrane_error_2.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("short-constructor-caught"));
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
    too_long = terrane_collection_support::Tuple::<u32>::new(vec![800, 600, 400]);
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                configuration_for(too_long), 42 /* terrane-site: src/main.trn:80:9-80:36 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3
                    && __terrane_error_3.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_3 = true;
                    println!("{}", String::from("long-constructor-caught"));
                }
                if !__terrane_handled_3 {
                    return TerraneCompletion::Error(__terrane_error_3);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_3 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    tickets = terrane_collection_support::Tuple::<
        Ticket,
    >::new(
        vec![
            __terrane_raised(ticket(), 43 /* terrane-site: src/main.trn:83:23-83:30 */), __terrane_raised(ticket(),
            44 /* terrane-site: src/main.trn:83:34-83:41 */)
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        tickets; __terrane_raised(take_pair(__terrane_conversion_input_0),
        45 /* terrane-site: src/main.trn:84:13-84:36 */) })
    );
}
