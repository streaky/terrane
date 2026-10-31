// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Holder {
    pub text: String,
    pub data: Vec<u8>,
    pub items: terrane_collection_support::List<String>,
    pub n: terrane_int_support::Int,
    pub key: String,
    pub entries: terrane_collection_support::Map<String, String>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            text: String::from("source storage"),
            data: Vec::<u8>::from([65, 66, 67]),
            items: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("al"), String::from("alpha")]),
            n: terrane_int_support::Int::from(1_i128),
            key: String::from("second"),
            entries: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::new(String::from("first"),
                    String::from("al")),
                    terrane_collection_support::Entry::new(String::from("second"),
                    String::from("alpha"))
                ],
            ),
        }
    }
    pub async fn joined_read(&mut self) -> String {
        {
            let __terrane_value = terrane_int_support::Int::from(1_i128);
            self.n = __terrane_value;
        }
        return {
            let __terrane_index_0 = __terrane_raised(
                terrane_collection_support::index_from_int(&self.n.clone()),
                0 /* terrane-site: src/main.trn:16:28-16:46 */,
            );
            let __terrane_index_place_1 = __terrane_raised(
                self
                    .items
                    .get(__terrane_index_0)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_0,
                    )),
                0 /* terrane-site: src/main.trn:16:28-16:46 */,
            );
            let __terrane_native_reference_0 = &*__terrane_index_place_1;
            __terrane_raised(
                join_text(__terrane_native_reference_0, "tail"),
                1 /* terrane-site: src/main.trn:16:17-16:54 */,
            )
        };
    }
    pub async fn __terrane_shared_async_joined_read(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> String {
        {
            let __terrane_value = terrane_int_support::Int::from(1_i128);
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .n = __terrane_value;
        }
        return {
            let __terrane_index_collection_4 = {
                let __terrane_read_guard_3 = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned");
                __terrane_read_guard_3.items.clone()
            };
            let __terrane_index_5 = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &{
                        let __terrane_snapshot = __terrane_shared_handle
                            .lock()
                            .expect("shared async receiver lock poisoned")
                            .n
                            .clone();
                        __terrane_snapshot
                    },
                ),
                0 /* terrane-site: src/main.trn:16:28-16:46 */,
            );
            let __terrane_index_place_6 = __terrane_raised(
                __terrane_index_collection_4
                    .get(__terrane_index_5)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_5,
                    )),
                0 /* terrane-site: src/main.trn:16:28-16:46 */,
            );
            let __terrane_native_reference_0 = &*__terrane_index_place_6;
            __terrane_raised(
                join_text(__terrane_native_reference_0, "tail"),
                1 /* terrane-site: src/main.trn:16:17-16:54 */,
            )
        };
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
        &item.text), 2 /* terrane-site: src/main.trn:23:13-23:44 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_string(&item
        .text, &item.text), 3 /* terrane-site: src/main.trn:24:13-24:46 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&item
        .data, &item.data), 4 /* terrane-site: src/main.trn:25:13-25:45 */))
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
        let alias: std::sync::Arc<TerraneSharedCell<Holder>>;
        text = String::from("ordinary storage");
        data = Vec::<u8>::from([65, 66, 67]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&text,
            &text), 5 /* terrane-site: src/main.trn:30:13-30:34 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_string(&text,
            &text), 6 /* terrane-site: src/main.trn:31:13-31:36 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&data,
            &data), 7 /* terrane-site: src/main.trn:32:13-32:35 */))
        );
        item = Holder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item
            .text, &item.text), 8 /* terrane-site: src/main.trn:34:13-34:44 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_bytes(&item
            .data, &item.data), 9 /* terrane-site: src/main.trn:35:13-35:45 */))
        );
        read_holder(&item);
        values = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("indexed storage"), String::from("second")]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_7 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            10 /* terrane-site: src/main.trn:38:24-38:33 */); let
            __terrane_index_place_8 = __terrane_raised(values.get(__terrane_index_7)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_7)),
            10 /* terrane-site: src/main.trn:38:24-38:33 */); let
            __terrane_native_reference_0 = &* __terrane_index_place_8; let
            __terrane_index_9 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            11 /* terrane-site: src/main.trn:38:35-38:44 */); let
            __terrane_index_place_10 = __terrane_raised(values.get(__terrane_index_9)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_9)),
            11 /* terrane-site: src/main.trn:38:35-38:44 */); let
            __terrane_native_reference_1 = &* __terrane_index_place_10;
            __terrane_raised(same_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 12 /* terrane-site: src/main.trn:38:13-38:44 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_11 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            13 /* terrane-site: src/main.trn:39:24-39:33 */); let
            __terrane_index_place_12 = __terrane_raised(values.get(__terrane_index_11)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_11)),
            13 /* terrane-site: src/main.trn:39:24-39:33 */); let
            __terrane_native_reference_0 = &* __terrane_index_place_12; let
            __terrane_index_13 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            14 /* terrane-site: src/main.trn:39:35-39:44 */); let
            __terrane_index_place_14 = __terrane_raised(values.get(__terrane_index_13)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_13)),
            14 /* terrane-site: src/main.trn:39:35-39:44 */); let
            __terrane_native_reference_1 = &* __terrane_index_place_14;
            __terrane_raised(join_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 15 /* terrane-site: src/main.trn:39:13-39:44 */) })
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_19 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            16 /* terrane-site: src/main.trn:41:24-41:33 */); let
            __terrane_index_place_20 = __terrane_raised(nested.get(__terrane_index_19)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_19)),
            16 /* terrane-site: src/main.trn:41:24-41:33 */); let __terrane_index_21 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            17 /* terrane-site: src/main.trn:41:24-41:36 */); let
            __terrane_index_place_22 = __terrane_raised(__terrane_index_place_20
            .get(__terrane_index_21).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_21)),
            17 /* terrane-site: src/main.trn:41:24-41:36 */); let
            __terrane_native_reference_0 = &* __terrane_index_place_22; let
            __terrane_index_23 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            18 /* terrane-site: src/main.trn:41:38-41:47 */); let
            __terrane_index_place_24 = __terrane_raised(nested.get(__terrane_index_23)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_23)),
            18 /* terrane-site: src/main.trn:41:38-41:47 */); let __terrane_index_25 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            19 /* terrane-site: src/main.trn:41:38-41:50 */); let
            __terrane_index_place_26 = __terrane_raised(__terrane_index_place_24
            .get(__terrane_index_25).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_25)),
            19 /* terrane-site: src/main.trn:41:38-41:50 */); let
            __terrane_native_reference_1 = &* __terrane_index_place_26;
            __terrane_raised(join_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 20 /* terrane-site: src/main.trn:41:13-41:50 */) })
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_map_key_31 =
            String::from("second"); let __terrane_index_place_32 =
            __terrane_raised(map_grid.get(&__terrane_map_key_31)
            .ok_or(terrane_collection_support::MissingKey), 21 /* terrane-site: src/main.trn:43:24-43:42 */); let __terrane_map_key_33 =
            String::from("left"); let __terrane_index_place_34 =
            __terrane_raised(__terrane_index_place_32.get(&__terrane_map_key_33)
            .ok_or(terrane_collection_support::MissingKey), 22 /* terrane-site: src/main.trn:43:24-43:50 */); let __terrane_native_reference_0 = &*
            __terrane_index_place_34; let __terrane_map_key_35 = String::from("first");
            let __terrane_index_place_36 = __terrane_raised(map_grid
            .get(&__terrane_map_key_35).ok_or(terrane_collection_support::MissingKey),
            23 /* terrane-site: src/main.trn:43:52-43:69 */); let __terrane_map_key_37
            = String::from("right"); let __terrane_index_place_38 =
            __terrane_raised(__terrane_index_place_36.get(&__terrane_map_key_37)
            .ok_or(terrane_collection_support::MissingKey), 24 /* terrane-site: src/main.trn:43:52-43:78 */); let __terrane_native_reference_1 = &*
            __terrane_index_place_38;
            __terrane_raised(join_text(__terrane_native_reference_0,
            __terrane_native_reference_1), 25 /* terrane-site: src/main.trn:43:13-43:78 */) })
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values).as_vec()), 26 /* terrane-site: src/main.trn:44:13-44:42 */))
        );
        values_copy = values.clone();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values_copy).as_vec()), 27 /* terrane-site: src/main.trn:46:13-46:47 */))
        );
        {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                28 /* terrane-site: src/main.trn:47:5-47:31 */,
            );
            let __terrane_value = String::from("changed list");
            let _ = __terrane_raised(
                values.set(__terrane_index, __terrane_value),
                28 /* terrane-site: src/main.trn:47:5-47:31 */,
            );
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence((&values)
            .as_vec(), (&values_copy).as_vec()), 29 /* terrane-site: src/main.trn:48:13-48:47 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            30 /* terrane-site: src/main.trn:49:12-49:21 */)), 30 /* terrane-site: src/main.trn:49:12-49:21 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values_copy
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            31 /* terrane-site: src/main.trn:50:12-50:26 */)), 31 /* terrane-site: src/main.trn:50:12-50:26 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::same_generic(&text, &text))) { Ok(value) => Ok(value), Err(payload)
            => Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::same_generic")) }, 32 /* terrane-site: src/main.trn:51:13-51:37 */))
        );
        native = __terrane_raised(
            terrane_static_trn_5265636f7264_new(),
            33 /* terrane-site: src/main.trn:52:14-52:26 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&native
            .text, &native.text), 34 /* terrane-site: src/main.trn:53:13-53:48 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&native)
            .same(&native.text))) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Record::same")) },
            35 /* terrane-site: src/main.trn:54:13-54:37 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_5265636f7264_inspect(&item
            .text, &item.text), 36 /* terrane-site: src/main.trn:55:13-55:50 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = same_after_yield(&item.text, &item.text); async move {
            __terrane_raised_err(__terrane_future. await, 37 /* terrane-site: src/main.trn:56:18-56:56 */) } }). await, 37 /* terrane-site: src/main.trn:56:18-56:56 */))
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
            38 /* terrane-site: src/main.trn:58:5-58:19 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!("{}", terrane_scalar_support::scalar_text(&copy.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(owned_text(text
            .clone()), 39 /* terrane-site: src/main.trn:61:13-61:29 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&text));
        {
            let __terrane_value = String::from("changed");
            item.text = __terrane_value;
        }
        println!("{}", terrane_scalar_support::scalar_text(&item.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_text(&item
            .text, &item.text), 40 /* terrane-site: src/main.trn:65:13-65:44 */))
        );
        __terrane_raised(
            update_record(&mut native),
            41 /* terrane-site: src/main.trn:66:5-66:26 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!("{}", terrane_scalar_support::scalar_text(&copy.text));
        reader = std::sync::Arc::new(move |argument_0: String, argument_1: String| __terrane_raised(
            same_text(&argument_0, &argument_1),
            42 /* terrane-site: src/main.trn:69:14-69:23 */,
        ));
        println!(
            "{}", terrane_scalar_support::scalar_text(&reader(text.clone(), text
            .clone()))
        );
        shared = std::sync::Arc::new(
            TerraneSharedCell::new(Holder::terrane_construct()),
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_39 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_39.lock()
            .expect("shared reference lock poisoned");
            __terrane_raised(same_text(&__terrane_native_read_guard_0.text,
            &__terrane_native_read_guard_0.text), 43 /* terrane-site: src/main.trn:72:13-72:48 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_40 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_40.lock()
            .expect("shared reference lock poisoned");
            __terrane_raised(same_bytes(&__terrane_native_read_guard_0.data,
            &__terrane_native_read_guard_0.data), 44 /* terrane-site: src/main.trn:73:13-73:49 */) })
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
                    45 /* terrane-site: src/main.trn:74:15-74:44 */,
                )
            }
        };
        {
            let __terrane_owner_41 = shared.clone();
            let __terrane_value = String::from("replacement before polling");
            __terrane_owner_41.lock().expect("shared reference lock poisoned").text = __terrane_value;
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(pending)
            . await, 46 /* terrane-site: src/main.trn:76:18-76:25 */))
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
                    47 /* terrane-site: src/main.trn:78:20-78:36 */,
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
            = { let __terrane_snapshot_42 = (* &native.text).clone();
            __terrane_snapshot_42 }; let __terrane_native_input_0 =
            &__terrane_native_snapshot_0; let __terrane_native_input_1 =
            &__terrane_raised(take_text(&mut native), 48 /* terrane-site: src/main.trn:87:38-87:55 */);
            __terrane_raised(join_text(__terrane_native_input_0,
            __terrane_native_input_1), 49 /* terrane-site: src/main.trn:87:13-87:56 */) })
        );
        {
            let __terrane_value = String::from("old receiver");
            native.text = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_44
            = { let __terrane_snapshot_43 = (* &native.text).clone();
            __terrane_snapshot_43 }; let __terrane_string_argument_0 =
            &__terrane_raised(take_text(&mut native), 50 /* terrane-site: src/main.trn:89:36-89:53 */); __terrane_string_receiver_44
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(same_sequence(&native
            .texts, &native.texts), 51 /* terrane-site: src/main.trn:91:13-91:54 */))
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
            52 /* terrane-site: src/main.trn:94:19-94:32 */,
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
                    53 /* terrane-site: src/main.trn:99:21-99:45 */,
                )
            }
        };
        before = String::from("after replacement");
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(pending_value)
            . await, 54 /* terrane-site: src/main.trn:101:18-101:31 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&before));
        address = __terrane_raised(
            sequence_address((&values).as_vec()),
            55 /* terrane-site: src/main.trn:103:15-103:39 */,
        );
        pending_list = {
            let __terrane_native_value_0 = values.clone();
            async move {
                let __terrane_future = read_sequence_after_yield(
                    (&__terrane_native_value_0).as_vec(),
                );
                __terrane_raised_err(
                    __terrane_future.await,
                    56 /* terrane-site: src/main.trn:104:20-104:53 */,
                )
            }
        };
        {
            let __terrane_index = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                57 /* terrane-site: src/main.trn:105:5-105:38 */,
            );
            let __terrane_value = String::from("after retained read");
            let _ = __terrane_raised(
                values.set(__terrane_index, __terrane_value),
                57 /* terrane-site: src/main.trn:105:5-105:38 */,
            );
        }
        observed = __terrane_traced(
            __terrane_await(pending_list).await,
            58 /* terrane-site: src/main.trn:106:22-106:34 */,
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
            59 /* terrane-site: src/main.trn:109:12-109:21 */)),
            59 /* terrane-site: src/main.trn:109:12-109:21 */))
        );
        method_input = String::from("before method");
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future = { let __terrane_call = (&native)
            .echo_after_yield(&method_input); Box::pin(async move { match crate
            ::__terrane_dependency_await_unwind(__terrane_call). await { Ok(value) =>
            Ok(value), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "witness", "witness::Record::echo_after_yield")) } }) }; async move {
            __terrane_raised_err(__terrane_future. await, 60 /* terrane-site: src/main.trn:111:18-111:55 */) } }). await, 60 /* terrane-site: src/main.trn:111:18-111:55 */))
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
            __terrane_raised_err(__terrane_future. await, 61 /* terrane-site: src/main.trn:115:18-115:54 */) } }). await, 61 /* terrane-site: src/main.trn:115:18-115:54 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&native.text));
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_index_collection_49 = { let __terrane_owner_47 = &shared; let
            __terrane_read_guard_48 = __terrane_owner_47.lock()
            .expect("shared reference lock poisoned"); __terrane_read_guard_48.items
            .clone() }; let __terrane_index_50 =
            __terrane_raised(terrane_collection_support::index_from_int(&{ let
            __terrane_snapshot = shared.lock().expect("shared reference lock poisoned").n
            .clone(); __terrane_snapshot }), 62 /* terrane-site: src/main.trn:117:24-117:46 */); let __terrane_index_place_51 =
            __terrane_raised(__terrane_index_collection_49.get(__terrane_index_50)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_50)),
            62 /* terrane-site: src/main.trn:117:24-117:46 */); let
            __terrane_native_reference_0 = &* __terrane_index_place_51;
            __terrane_raised(join_text(__terrane_native_reference_0, "tail"),
            63 /* terrane-site: src/main.trn:117:13-117:54 */) })
        );
        alias = shared.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_snapshot_0
            = { let __terrane_index_collection_57 = { let __terrane_owner_55 = &shared;
            let __terrane_read_guard_56 = __terrane_owner_55.lock()
            .expect("shared reference lock poisoned"); __terrane_read_guard_56.items
            .clone() }; let __terrane_index_58 =
            __terrane_raised(terrane_collection_support::index_from_int(&{ let
            __terrane_snapshot = alias.lock().expect("shared reference lock poisoned").n
            .clone(); __terrane_snapshot }), 64 /* terrane-site: src/main.trn:119:24-119:45 */); let __terrane_index_place_59 =
            __terrane_raised(__terrane_index_collection_57.get(__terrane_index_58)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_58)),
            64 /* terrane-site: src/main.trn:119:24-119:45 */); let
            __terrane_snapshot_60 = (* &* __terrane_index_place_59).clone();
            __terrane_snapshot_60 }; let __terrane_native_snapshot_1 = { let
            __terrane_owner_61 = &alias; let __terrane_read_guard_62 = __terrane_owner_61
            .lock().expect("shared reference lock poisoned"); let __terrane_index_63 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            65 /* terrane-site: src/main.trn:119:47-119:61 */); let
            __terrane_index_place_64 = __terrane_raised(__terrane_read_guard_62.items
            .get(__terrane_index_63).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_63)),
            65 /* terrane-site: src/main.trn:119:47-119:61 */); let
            __terrane_snapshot_65 = (* &* __terrane_index_place_64).clone();
            __terrane_snapshot_65 };
            __terrane_raised(join_text(&__terrane_native_snapshot_0,
            &__terrane_native_snapshot_1), 66 /* terrane-site: src/main.trn:119:13-119:61 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_snapshot_0
            = { let __terrane_index_collection_71 = { let __terrane_owner_69 = &shared;
            let __terrane_read_guard_70 = __terrane_owner_69.lock()
            .expect("shared reference lock poisoned"); __terrane_read_guard_70.entries
            .clone() }; let __terrane_map_key_72 = { let __terrane_snapshot = alias
            .lock().expect("shared reference lock poisoned").key.clone();
            __terrane_snapshot }; let __terrane_index_place_73 =
            __terrane_raised(__terrane_index_collection_71.get(&__terrane_map_key_72)
            .ok_or(terrane_collection_support::MissingKey), 67 /* terrane-site: src/main.trn:120:24-120:49 */); let __terrane_snapshot_74 = (* &*
            __terrane_index_place_73).clone(); __terrane_snapshot_74 }; let
            __terrane_native_snapshot_1 = { let __terrane_owner_75 = &alias; let
            __terrane_read_guard_76 = __terrane_owner_75.lock()
            .expect("shared reference lock poisoned"); let __terrane_map_key_77 =
            String::from("first"); let __terrane_index_place_78 =
            __terrane_raised(__terrane_read_guard_76.entries.get(&__terrane_map_key_77)
            .ok_or(terrane_collection_support::MissingKey), 68 /* terrane-site: src/main.trn:120:51-120:73 */); let __terrane_snapshot_79 = (* &*
            __terrane_index_place_78).clone(); __terrane_snapshot_79 };
            __terrane_raised(join_text(&__terrane_native_snapshot_0,
            &__terrane_native_snapshot_1), 69 /* terrane-site: src/main.trn:120:13-120:73 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_snapshot_0
            = { let __terrane_owner_84 = &shared; let __terrane_read_guard_85 =
            __terrane_owner_84.lock().expect("shared reference lock poisoned"); let
            __terrane_index_86 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            70 /* terrane-site: src/main.trn:121:24-121:39 */); let
            __terrane_index_place_87 = __terrane_raised(__terrane_read_guard_85.items
            .get(__terrane_index_86).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_86)),
            70 /* terrane-site: src/main.trn:121:24-121:39 */); let
            __terrane_snapshot_88 = (* &* __terrane_index_place_87).clone();
            __terrane_snapshot_88 }; let __terrane_native_snapshot_1 = { let
            __terrane_owner_89 = &shared; let __terrane_read_guard_90 =
            __terrane_owner_89.lock().expect("shared reference lock poisoned"); let
            __terrane_index_91 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            71 /* terrane-site: src/main.trn:121:41-121:56 */); let
            __terrane_index_place_92 = __terrane_raised(__terrane_read_guard_90.items
            .get(__terrane_index_91).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_91)),
            71 /* terrane-site: src/main.trn:121:41-121:56 */); let
            __terrane_snapshot_93 = (* &* __terrane_index_place_92).clone();
            __terrane_snapshot_93 }; let __terrane_native_snapshot_2 = { let
            __terrane_owner_94 = &alias; let __terrane_read_guard_95 = __terrane_owner_94
            .lock().expect("shared reference lock poisoned"); let __terrane_index_96 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            72 /* terrane-site: src/main.trn:121:58-121:72 */); let
            __terrane_index_place_97 = __terrane_raised(__terrane_read_guard_95.items
            .get(__terrane_index_96).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_96)),
            72 /* terrane-site: src/main.trn:121:58-121:72 */); let
            __terrane_snapshot_98 = (* &* __terrane_index_place_97).clone();
            __terrane_snapshot_98 }; let __terrane_native_snapshot_3 = { let
            __terrane_owner_99 = &alias; let __terrane_read_guard_100 =
            __terrane_owner_99.lock().expect("shared reference lock poisoned"); let
            __terrane_index_101 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            73 /* terrane-site: src/main.trn:121:74-121:88 */); let
            __terrane_index_place_102 = __terrane_raised(__terrane_read_guard_100.items
            .get(__terrane_index_101).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_101)),
            73 /* terrane-site: src/main.trn:121:74-121:88 */); let
            __terrane_snapshot_103 = (* &* __terrane_index_place_102).clone();
            __terrane_snapshot_103 };
            __terrane_raised(join_four(&__terrane_native_snapshot_0,
            &__terrane_native_snapshot_1, &__terrane_native_snapshot_2,
            &__terrane_native_snapshot_3), 74 /* terrane-site: src/main.trn:121:13-121:88 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_snapshot_0
            = { let __terrane_owner_105 = &shared; let __terrane_read_guard_106 =
            __terrane_owner_105.lock().expect("shared reference lock poisoned"); let
            __terrane_index_107 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            75 /* terrane-site: src/main.trn:122:24-122:39 */); let
            __terrane_index_place_108 = __terrane_raised(__terrane_read_guard_106.items
            .get(__terrane_index_107).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_107)),
            75 /* terrane-site: src/main.trn:122:24-122:39 */); let
            __terrane_snapshot_109 = (* &* __terrane_index_place_108).clone();
            __terrane_snapshot_109 }; let __terrane_index_110 =
            __terrane_raised(terrane_collection_support::index_from_int(&{ let
            __terrane_snapshot = shared.lock().expect("shared reference lock poisoned").n
            .clone(); __terrane_snapshot }), 76 /* terrane-site: src/main.trn:122:41-122:57 */); let __terrane_index_place_111 =
            __terrane_raised(values.get(__terrane_index_110).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_110)),
            76 /* terrane-site: src/main.trn:122:41-122:57 */); let
            __terrane_native_reference_1 = &* __terrane_index_place_111;
            __terrane_raised(join_text(&__terrane_native_snapshot_0,
            __terrane_native_reference_1), 77 /* terrane-site: src/main.trn:122:13-122:57 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = shared.clone();
            Holder::__terrane_shared_async_joined_read(__terrane_shared_call_handle) }).
            await)
        );
    });
}
