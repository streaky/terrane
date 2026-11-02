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
fn configuration() -> Descriptor {
    let nested: terrane_collection_support::Tuple<Option<u32>>;
    let window: terrane_collection_support::Tuple<u32>;
    nested = terrane_collection_support::Tuple::<
        Option<u32>,
    >::new(vec![Some(2), Some(3)]);
    window = terrane_collection_support::Tuple::<u32>::new(vec![800, 600]);
    return Descriptor::terrane_construct(
        __terrane_raised(
            path_value(),
            0 /* terrane-site: src/main.trn:15:80-15:91 */,
        ),
        __terrane_raised(
            timeout_value(),
            1 /* terrane-site: src/main.trn:15:105-15:119 */,
        ),
        terrane_collection_support::List::<String>::new(vec![String::from("alpha")]),
        terrane_collection_support::List::<
            String,
        >::new(vec![String::from("one"), String::from("two")]),
        terrane_collection_support::List::<
            String,
        >::new(vec![String::from("left"), String::from("right")]),
        Some(window),
        Some(nested),
        Some(
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
        ),
        Some(String::from("ready")),
    );
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
    config = configuration();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(inspect(&config),
        2 /* terrane-site: src/main.trn:19:13-19:28 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(inspect_shared(&config),
        3 /* terrane-site: src/main.trn:20:13-20:35 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_descriptor_owner_1084
        = &config; let __terrane_descriptor_view_1084 =
        terrane_recursive_descriptor_witness::Descriptor { path :
        __terrane_descriptor_owner_1084.path.clone(), timeout :
        __terrane_descriptor_owner_1084.timeout, args : __terrane_descriptor_owner_1084
        .args.iter().map(| item | std::ffi::OsStr::new(&* item)).collect(), names :
        __terrane_descriptor_owner_1084.names.iter().map(| item | (* item).as_str())
        .collect(), strings : __terrane_descriptor_owner_1084.strings.as_slice(),
        window_size : __terrane_descriptor_owner_1084.window_size, nested :
        __terrane_descriptor_owner_1084.nested, labels : __terrane_descriptor_owner_1084
        .labels.clone(), note : __terrane_descriptor_owner_1084.note.as_ref().map(| value
        | (* value).as_str()) }; __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        __terrane_descriptor_view_1084.inspect_method())) { Ok(value) => Ok(value),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
        "terrane_recursive_descriptor_witness",
        "terrane_recursive_descriptor_witness::Descriptor::inspect_method")) },
        4 /* terrane-site: src/main.trn:21:13-21:35 */) })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_466163746f7279_inspect_static(&config),
        5 /* terrane-site: src/main.trn:22:13-22:44 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                fail_early(&config), 6 /* terrane-site: src/main.trn:24:9-24:27 */
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
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(consume(config),
        7 /* terrane-site: src/main.trn:27:13-27:33 */))
    );
    config2 = configuration();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_descriptor_owner_1332
        = config2; let __terrane_descriptor_view_1332 =
        terrane_recursive_descriptor_witness::Descriptor { path :
        __terrane_descriptor_owner_1332.path, timeout : __terrane_descriptor_owner_1332
        .timeout, args : __terrane_descriptor_owner_1332.args.iter().map(| item |
        std::ffi::OsStr::new(&* item)).collect(), names : __terrane_descriptor_owner_1332
        .names.iter().map(| item | (* item).as_str()).collect(), strings :
        __terrane_descriptor_owner_1332.strings.as_slice(), window_size :
        __terrane_descriptor_owner_1332.window_size, nested :
        __terrane_descriptor_owner_1332.nested, labels : __terrane_descriptor_owner_1332
        .labels, note : __terrane_descriptor_owner_1332.note.as_ref().map(| value | (*
        value).as_str()) }; __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        __terrane_descriptor_view_1332.consume_method())) { Ok(value) => Ok(value),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
        "terrane_recursive_descriptor_witness",
        "terrane_recursive_descriptor_witness::Descriptor::consume_method")) },
        8 /* terrane-site: src/main.trn:29:13-29:36 */) })
    );
    config3 = configuration();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_466163746f7279_consume_static(config3),
        9 /* terrane-site: src/main.trn:31:13-31:50 */))
    );
    unique = UniqueDescriptor::terrane_construct(
        __terrane_raised(ticket(), 10 /* terrane-site: src/main.trn:32:51-32:58 */),
        String::from("unique"),
    );
    result = __terrane_raised(
        take_unique(unique),
        11 /* terrane-site: src/main.trn:33:14-33:38 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&result));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(ticket_drops(),
        12 /* terrane-site: src/main.trn:35:13-35:26 */))
    );
    failing = UniqueDescriptor::terrane_construct(
        __terrane_raised(ticket(), 13 /* terrane-site: src/main.trn:36:52-36:59 */),
        String::from("failure"),
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            __terrane_raised_completion!(
                fail_unique(failing), 14 /* terrane-site: src/main.trn:38:9-38:34 */
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
        15 /* terrane-site: src/main.trn:41:13-41:26 */))
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
    first = terrane_recursive_descriptor_witness::Observed {
        name: String::from("one"),
    };
    second = terrane_recursive_descriptor_witness::Observed {
        name: String::from("two"),
    };
    mixed = MixedDescriptor::terrane_construct(
        rows,
        terrane_collection_support::UnorderedMap::<
            String,
            Observed,
        >::new(
            vec![
                terrane_collection_support::Entry::new(String::from("first"), first),
                terrane_collection_support::Entry::new(String::from("second"), second)
            ],
        ),
        Some(pair),
    );
    __terrane_raised(reset_clones(), 16 /* terrane-site: src/main.trn:51:5-51:18 */);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(take_mixed(mixed),
        17 /* terrane-site: src/main.trn:52:13-52:35 */))
    );
}
