// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Holder {
    pub text: String,
    pub data: Vec<u8>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            text: String::from("source storage"),
            data: Vec::<u8>::from([65, 66, 67]),
        }
    }
}
#[derive(Clone)]
pub struct Inspector {}
impl Inspector {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn marker(&self) -> bool {
        return true;
    }
}
impl InspectProtocol for Inspector {
    fn clone_box(&self) -> Box<dyn InspectProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InspectProtocol> {
        Box::new(self.clone())
    }
    fn marker(&self) -> bool {
        Inspector::marker(&*self)
    }
    fn inspect_text(&self, left: &str, right: &str) -> bool {
        || -> Result<bool, crate::TerraneForeignError> {
            let __terrane_default = <Inspector as witness::Inspect>::inspect_text(
                &*self,
                left,
                right,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn inspect_strings(
        &self,
        left: &std::vec::Vec<String>,
        right: &std::vec::Vec<String>,
    ) -> bool {
        || -> Result<bool, crate::TerraneForeignError> {
            let __terrane_default = <Inspector as witness::Inspect>::inspect_strings(
                &*self,
                left,
                right,
            );
            Ok(__terrane_default)
        }()
            .unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
impl From<Inspector> for Inspect {
    fn from(value: Inspector) -> Self {
        Self(Box::new(value))
    }
}
impl witness::Inspect for Inspector {
    fn marker(&self) -> bool {
        let __terrane_boundary: Result<bool, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Inspector>::marker(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
fn read_holder(item: &Holder) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item.text,
        &item.text), 0 /* terrane-site: src/main.trn:15:13-15:44 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_string(&item
        .text, &item.text), 1 /* terrane-site: src/main.trn:16:13-16:46 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&item
        .data, &item.data), 2 /* terrane-site: src/main.trn:17:13-17:45 */))
    );
}
fn main() {
    __terrane_run(async move {
        let text: String;
        let data: Vec<u8>;
        let mut item: Holder;
        let mut values: terrane_collection_support::List<String>;
        let nested: terrane_collection_support::List<
            terrane_collection_support::List<String>,
        >;
        let map_grid: terrane_collection_support::Map<
            String,
            terrane_collection_support::Map<String, String>,
        >;
        let values_copy: terrane_collection_support::List<String>;
        let mut native: Record;
        let copy: Record;
        let reader: std::sync::Arc<dyn Fn(String, String) -> bool + Send + Sync>;
        let shared: std::sync::Arc<TerraneSharedCell<Holder>>;
        let pending;
        let async_reader: std::sync::Arc<
            dyn Fn(
                String,
            ) -> std::pin::Pin<Box<dyn Future<Output = String>>> + Send + Sync,
        >;
        let erased: Inspect;
        let observer: &Inspect;
        let inspect_callback: std::sync::Arc<
            dyn Fn(String, String) -> bool + Send + Sync,
        >;
        let mut word: &String;
        let list_reader: std::sync::Arc<
            dyn Fn(
                terrane_collection_support::List<String>,
                terrane_collection_support::List<String>,
            ) -> bool + Send + Sync,
        >;
        let list_inspector: std::sync::Arc<
            dyn Fn(
                terrane_collection_support::List<String>,
                terrane_collection_support::List<String>,
            ) -> bool + Send + Sync,
        >;
        let mut before: String;
        let pending_value;
        let address: terrane_int_support::Int;
        let pending_list;
        let observed: ReadObservation;
        let mut method_input: String;
        text = String::from("ordinary storage");
        data = Vec::<u8>::from([65, 66, 67]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&text,
            &text), 3 /* terrane-site: src/main.trn:22:13-22:34 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_string(&text,
            &text), 4 /* terrane-site: src/main.trn:23:13-23:36 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&data,
            &data), 5 /* terrane-site: src/main.trn:24:13-24:35 */))
        );
        item = Holder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item
            .text, &item.text), 6 /* terrane-site: src/main.trn:26:13-26:44 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&item
            .data, &item.data), 7 /* terrane-site: src/main.trn:27:13-27:45 */))
        );
        read_holder(&item);
        values = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("indexed storage"), String::from("second")]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            8 /* terrane-site: src/main.trn:30:24-30:33 */); let
            __terrane_index_place_1 = __terrane_raised(values.get(__terrane_index_0)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
            8 /* terrane-site: src/main.trn:30:24-30:33 */); let
            __terrane_native_reference_0 = &* __terrane_index_place_1; let
            __terrane_index_2 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            9 /* terrane-site: src/main.trn:30:35-30:44 */); let
            __terrane_index_place_3 = __terrane_raised(values.get(__terrane_index_2)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
            9 /* terrane-site: src/main.trn:30:35-30:44 */); let
            __terrane_native_reference_1 = &* __terrane_index_place_3;
            __terrane_raised(same_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 10 /* terrane-site: src/main.trn:30:13-30:44 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_4 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            11 /* terrane-site: src/main.trn:31:24-31:33 */); let
            __terrane_index_place_5 = __terrane_raised(values.get(__terrane_index_4)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_4)),
            11 /* terrane-site: src/main.trn:31:24-31:33 */); let
            __terrane_native_reference_0 = &* __terrane_index_place_5; let
            __terrane_index_6 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            12 /* terrane-site: src/main.trn:31:35-31:44 */); let
            __terrane_index_place_7 = __terrane_raised(values.get(__terrane_index_6)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_6)),
            12 /* terrane-site: src/main.trn:31:35-31:44 */); let
            __terrane_native_reference_1 = &* __terrane_index_place_7;
            __terrane_raised(join_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 13 /* terrane-site: src/main.trn:31:13-31:44 */) })
        );
        nested = terrane_collection_support::List::<
            terrane_collection_support::List<String>,
        >::new(
            vec![
                terrane_collection_support::List::< String >::new(vec![String::from("a"),
                String::from("b")]), terrane_collection_support::List::< String
                >::new(vec![String::from("c"), String::from("d")])
            ],
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_12 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            14 /* terrane-site: src/main.trn:33:24-33:33 */); let
            __terrane_index_place_13 = __terrane_raised(nested.get(__terrane_index_12)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_12)),
            14 /* terrane-site: src/main.trn:33:24-33:33 */); let __terrane_index_14 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            15 /* terrane-site: src/main.trn:33:24-33:36 */); let
            __terrane_index_place_15 = __terrane_raised(__terrane_index_place_13
            .get(__terrane_index_14).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_14)),
            15 /* terrane-site: src/main.trn:33:24-33:36 */); let
            __terrane_native_reference_0 = &* __terrane_index_place_15; let
            __terrane_index_16 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            16 /* terrane-site: src/main.trn:33:38-33:47 */); let
            __terrane_index_place_17 = __terrane_raised(nested.get(__terrane_index_16)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_16)),
            16 /* terrane-site: src/main.trn:33:38-33:47 */); let __terrane_index_18 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            17 /* terrane-site: src/main.trn:33:38-33:50 */); let
            __terrane_index_place_19 = __terrane_raised(__terrane_index_place_17
            .get(__terrane_index_18).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_18)),
            17 /* terrane-site: src/main.trn:33:38-33:50 */); let
            __terrane_native_reference_1 = &* __terrane_index_place_19;
            __terrane_raised(join_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 18 /* terrane-site: src/main.trn:33:13-33:50 */) })
        );
        map_grid = terrane_collection_support::Map::<
            String,
            terrane_collection_support::Map<String, String>,
        >::new(
            vec![
                terrane_collection_support::Entry::new(String::from("first"),
                terrane_collection_support::Map::< String, String
                >::new(vec![terrane_collection_support::Entry::new(String::from("left"),
                String::from("red")),
                terrane_collection_support::Entry::new(String::from("right"),
                String::from("blue"))])),
                terrane_collection_support::Entry::new(String::from("second"),
                terrane_collection_support::Map::< String, String
                >::new(vec![terrane_collection_support::Entry::new(String::from("left"),
                String::from("green")),
                terrane_collection_support::Entry::new(String::from("right"),
                String::from("gold"))]))
            ],
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_map_key_24 =
            String::from("second"); let __terrane_index_place_25 =
            __terrane_raised(map_grid.get(&__terrane_map_key_24)
            .ok_or(terrane_collection_support::MissingKey), 19 /* terrane-site: src/main.trn:35:24-35:42 */); let __terrane_map_key_26 =
            String::from("left"); let __terrane_index_place_27 =
            __terrane_raised(__terrane_index_place_25.get(&__terrane_map_key_26)
            .ok_or(terrane_collection_support::MissingKey), 20 /* terrane-site: src/main.trn:35:24-35:50 */); let __terrane_native_reference_0 = &*
            __terrane_index_place_27; let __terrane_map_key_28 = String::from("first");
            let __terrane_index_place_29 = __terrane_raised(map_grid
            .get(&__terrane_map_key_28).ok_or(terrane_collection_support::MissingKey),
            21 /* terrane-site: src/main.trn:35:52-35:69 */); let __terrane_map_key_30
            = String::from("right"); let __terrane_index_place_31 =
            __terrane_raised(__terrane_index_place_29.get(&__terrane_map_key_30)
            .ok_or(terrane_collection_support::MissingKey), 22 /* terrane-site: src/main.trn:35:52-35:78 */); let __terrane_native_reference_1 = &*
            __terrane_index_place_31;
            __terrane_raised(join_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 23 /* terrane-site: src/main.trn:35:13-35:78 */) })
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values).as_vec()), 24 /* terrane-site: src/main.trn:36:13-36:42 */))
        );
        values_copy = values.clone();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values_copy).as_vec()), 25 /* terrane-site: src/main.trn:38:13-38:47 */))
        );
        {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                26 /* terrane-site: src/main.trn:39:5-39:31 */,
            );
            let __terrane_value = String::from("changed list");
            let _ = __terrane_raised(
                values.set(__terrane_index, __terrane_value),
                26 /* terrane-site: src/main.trn:39:5-39:31 */,
            );
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values_copy).as_vec()), 27 /* terrane-site: src/main.trn:40:13-40:47 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            28 /* terrane-site: src/main.trn:41:12-41:21 */)), 28 /* terrane-site: src/main.trn:41:12-41:21 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values_copy
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            29 /* terrane-site: src/main.trn:42:12-42:26 */)), 29 /* terrane-site: src/main.trn:42:12-42:26 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::same_generic(&text, &text))) { Ok(value) => Ok(value), Err(payload)
            => Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::same_generic")) }, 30 /* terrane-site: src/main.trn:43:13-43:37 */))
        );
        native = __terrane_raised(
            terrane_static_trn_5265636f7264_new(),
            31 /* terrane-site: src/main.trn:44:14-44:26 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&native
            .text, &native.text), 32 /* terrane-site: src/main.trn:45:13-45:48 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&native)
            .same(&native.text))) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Record::same")) },
            33 /* terrane-site: src/main.trn:46:13-46:37 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265636f7264_inspect(&item
            .text, &item.text), 34 /* terrane-site: src/main.trn:47:13-47:50 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = same_after_yield(&item.text, &item.text); async move {
            __terrane_raised_err(__terrane_future. await, 35 /* terrane-site: src/main.trn:48:18-48:56 */) } }). await, 35 /* terrane-site: src/main.trn:48:18-48:56 */))
        );
        copy = native.clone();
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = native.update();
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Record::update",
                        ),
                    )
                }
            },
            36 /* terrane-site: src/main.trn:50:5-50:19 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!("{}", terrane_scalar_support::scalar_text(&copy.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(owned_text(text
            .clone()), 37 /* terrane-site: src/main.trn:53:13-53:29 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&text));
        {
            let __terrane_value = String::from("changed");
            item.text = __terrane_value;
        }
        println!("{}", terrane_scalar_support::scalar_text(&item.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item
            .text, &item.text), 38 /* terrane-site: src/main.trn:57:13-57:44 */))
        );
        __terrane_raised(
            update_record(&mut native),
            39 /* terrane-site: src/main.trn:58:5-58:26 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!("{}", terrane_scalar_support::scalar_text(&copy.text));
        reader = std::sync::Arc::new(move |argument_0: String, argument_1: String| __terrane_raised(
            same_text(&argument_0, &argument_1),
            40 /* terrane-site: src/main.trn:61:14-61:23 */,
        ));
        println!(
            "{}", terrane_scalar_support::scalar_text(&reader(text.clone(), text
            .clone()))
        );
        shared = std::sync::Arc::new(
            TerraneSharedCell::new(Holder::terrane_construct()),
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_32 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_32.lock()
            .expect("shared reference lock poisoned");
            __terrane_raised(same_text(&__terrane_native_read_guard_0.text,
            &__terrane_native_read_guard_0.text), 41 /* terrane-site: src/main.trn:64:13-64:48 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_33 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_33.lock()
            .expect("shared reference lock poisoned");
            __terrane_raised(same_bytes(&__terrane_native_read_guard_0.data,
            &__terrane_native_read_guard_0.data), 42 /* terrane-site: src/main.trn:65:13-65:49 */) })
        );
        pending = {
            let __terrane_native_value_0 = {
                let __terrane_snapshot = shared
                    .lock()
                    .expect("shared reference lock poisoned")
                    .text
                    .clone();
                __terrane_snapshot
            };
            async move {
                let __terrane_future = echo_after_yield(&__terrane_native_value_0);
                __terrane_raised_err(
                    __terrane_future.await,
                    43 /* terrane-site: src/main.trn:66:15-66:44 */,
                )
            }
        };
        {
            let __terrane_owner_34 = shared.clone();
            let __terrane_value = String::from("replacement before polling");
            __terrane_owner_34.lock().expect("shared reference lock poisoned").text = __terrane_value;
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(pending)
            . await, 44 /* terrane-site: src/main.trn:68:18-68:25 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = shared
            .lock().expect("shared reference lock poisoned").text.clone();
            __terrane_snapshot })
        );
        async_reader = std::sync::Arc::new(move |
            argument_0: String,
        | -> std::pin::Pin<Box<dyn Future<Output = _>>> {
            Box::pin(async move {
                __terrane_raised(
                    echo_after_yield(&argument_0).await,
                    45 /* terrane-site: src/main.trn:70:20-70:36 */,
                )
            })
        });
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_reader(text
            .clone())). await)
        );
        erased = <Inspect>::from(Inspector::terrane_construct());
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&erased).inspect_text(&text,
            &text))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&erased)
            .inspect_strings((&values).as_vec(), (&values).as_vec()))
        );
        observer = &erased;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(&* observer).inspect_text(&text,
            &text))
        );
        inspect_callback = {
            let receiver = erased.clone();
            std::sync::Arc::new(move |argument_0: String, argument_1: String| {
                receiver.inspect_text(&argument_0, &argument_1)
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&inspect_callback(text.clone(),
            text.clone()))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_snapshot_0
            = (* &native.text).clone(); let __terrane_native_input_0 =
            &__terrane_native_snapshot_0; let __terrane_native_input_1 =
            &__terrane_raised(take_text(&mut native), 46 /* terrane-site: src/main.trn:79:38-79:55 */);
            __terrane_raised(join_text(__terrane_native_input_0,
            __terrane_native_input_1), 47 /* terrane-site: src/main.trn:79:13-79:56 */) })
        );
        {
            let __terrane_value = String::from("old receiver");
            native.text = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_35
            = (* &native.text).clone(); let __terrane_string_argument_0 =
            &__terrane_raised(take_text(&mut native), 48 /* terrane-site: src/main.trn:81:36-81:53 */); __terrane_string_receiver_35
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence(&native
            .texts, &native.texts), 49 /* terrane-site: src/main.trn:83:13-83:54 */))
        );
        let __terrane_iterable_0 = &native.texts;
        let mut __terrane_iterator_0 = __terrane_iterable_0.iter();
        loop {
            word = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&* word));
        }
        list_reader = std::sync::Arc::new(move |
            argument_0: terrane_collection_support::List<String>,
            argument_1: terrane_collection_support::List<String>|
        __terrane_raised(
            same_sequence(argument_0.as_vec(), argument_1.as_vec()),
            50 /* terrane-site: src/main.trn:86:19-86:32 */,
        ));
        println!(
            "{}", terrane_scalar_support::scalar_text(&list_reader(values.clone(), values
            .clone()))
        );
        list_inspector = {
            let receiver = erased;
            std::sync::Arc::new(move |
                argument_0: terrane_collection_support::List<String>,
                argument_1: terrane_collection_support::List<String>|
            receiver.inspect_strings(argument_0.as_vec(), argument_1.as_vec()))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&list_inspector(values.clone(),
            values.clone()))
        );
        before = String::from("before replacement");
        pending_value = {
            let __terrane_native_value_0 = before;
            async move {
                let __terrane_future = echo_after_yield(&__terrane_native_value_0);
                __terrane_raised_err(
                    __terrane_future.await,
                    51 /* terrane-site: src/main.trn:91:21-91:45 */,
                )
            }
        };
        before = String::from("after replacement");
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(pending_value)
            . await, 52 /* terrane-site: src/main.trn:93:18-93:31 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&before));
        address = __terrane_raised(
            sequence_address((&values).as_vec()),
            53 /* terrane-site: src/main.trn:95:15-95:39 */,
        );
        pending_list = {
            let __terrane_native_value_0 = values.clone();
            async move {
                let __terrane_future = read_sequence_after_yield(
                    (&__terrane_native_value_0).as_vec(),
                );
                __terrane_raised_err(
                    __terrane_future.await,
                    54 /* terrane-site: src/main.trn:96:20-96:53 */,
                )
            }
        };
        {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                55 /* terrane-site: src/main.trn:97:5-97:38 */,
            );
            let __terrane_value = String::from("after retained read");
            let _ = __terrane_raised(
                values.set(__terrane_index, __terrane_value),
                55 /* terrane-site: src/main.trn:97:5-97:38 */,
            );
        }
        observed = __terrane_traced(
            __terrane_await(pending_list).await,
            56 /* terrane-site: src/main.trn:98:22-98:34 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(observed
            .address as i128) == address))
        );
        println!("{}", terrane_scalar_support::scalar_text(&observed.first));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            57 /* terrane-site: src/main.trn:101:12-101:21 */)),
            57 /* terrane-site: src/main.trn:101:12-101:21 */))
        );
        method_input = String::from("before method");
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = { let __terrane_call = (&native)
            .echo_after_yield(&method_input); Box::pin(async move { match crate
            ::__terrane_dependency_await_unwind(__terrane_call). await { Ok(value) =>
            Ok(value), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "witness", "witness::Record::echo_after_yield")) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 58 /* terrane-site: src/main.trn:103:18-103:55 */) } }). await, 58 /* terrane-site: src/main.trn:103:18-103:55 */))
        );
        method_input = String::from("after method");
        println!("{}", terrane_scalar_support::scalar_text(&method_input));
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_native_value_0 = { let __terrane_snapshot = shared.lock()
            .expect("shared reference lock poisoned").text.clone(); __terrane_snapshot };
            let __terrane_native_future_receiver = &native; async move { let
            __terrane_future = { let __terrane_call = __terrane_native_future_receiver
            .echo_after_yield(&__terrane_native_value_0); Box::pin(async move { match
            crate ::__terrane_dependency_await_unwind(__terrane_call). await { Ok(value)
            => Ok(value), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "witness", "witness::Record::echo_after_yield")) } }) };
            __terrane_raised_err(__terrane_future. await, 59 /* terrane-site: src/main.trn:107:18-107:54 */) } }). await, 59 /* terrane-site: src/main.trn:107:18-107:54 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
    });
}
