// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: borrowed-text-observations
#[derive(Clone)]
pub struct Holder {
    pub text: String,
    pub data: Vec<u8>,
    pub items: terrane_collection_support::List<String>,
    pub calls: terrane_int_support::Int,
    pub n: terrane_int_support::Int,
    pub key: String,
    pub entries: terrane_collection_support::Map<String, String>,
    pub codec: terrane_string_support::Encoding,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            text: String::from(" AéA "),
            data: Vec::<u8>::from([65, 66, 67]),
            items: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("alpha")]),
            calls: terrane_int_support::Int::from(0_i128),
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
            codec: terrane_string_support::Encoding::Utf8,
        }
    }
    pub async fn indexed_read(&mut self) -> String {
        {
            let __terrane_value = terrane_int_support::Int::from(1_i128);
            self.n = __terrane_value;
        }
        return __terrane_raised(
            self
                .items
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&self.n.clone()),
                        0 /* terrane-site: case.trn:19:16-19:34 */,
                    ),
                ),
            0 /* terrane-site: case.trn:19:16-19:34 */,
        );
    }
    pub async fn __terrane_shared_async_indexed_read(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> String {
        {
            let __terrane_value = terrane_int_support::Int::from(1_i128);
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .n = __terrane_value;
        }
        return __terrane_raised(
            {
                let __terrane_snapshot_0 = (*&__terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .items)
                    .clone();
                __terrane_snapshot_0
            }
                .get_or_error(
                    __terrane_raised(
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
                        0 /* terrane-site: case.trn:19:16-19:34 */,
                    ),
                ),
            0 /* terrane-site: case.trn:19:16-19:34 */,
        );
    }
    pub async fn indexed_contains(&mut self) -> bool {
        {
            let __terrane_value = terrane_int_support::Int::from(1_i128);
            self.n = __terrane_value;
        }
        return {
            let __terrane_index_1 = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                1 /* terrane-site: case.trn:23:17-23:30 */,
            );
            let __terrane_index_place_2 = __terrane_raised(
                self
                    .items
                    .get(__terrane_index_1)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_1,
                    )),
                1 /* terrane-site: case.trn:23:17-23:30 */,
            );
            let __terrane_index_3 = __terrane_raised(
                terrane_collection_support::index_from_int(&self.n.clone()),
                2 /* terrane-site: case.trn:23:41-23:59 */,
            );
            let __terrane_index_place_4 = __terrane_raised(
                self
                    .items
                    .get(__terrane_index_3)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_3,
                    )),
                2 /* terrane-site: case.trn:23:41-23:59 */,
            );
            let __terrane_string_argument_0 = &*__terrane_index_place_4;
            __terrane_index_place_2
                .contains(
                    std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
                )
        };
    }
    pub async fn __terrane_shared_async_indexed_contains(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> bool {
        {
            let __terrane_value = terrane_int_support::Int::from(1_i128);
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .n = __terrane_value;
        }
        return {
            let __terrane_string_receiver_9 = {
                let __terrane_read_guard_5 = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned");
                let __terrane_index_6 = __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    1 /* terrane-site: case.trn:23:17-23:30 */,
                );
                let __terrane_index_place_7 = __terrane_raised(
                    __terrane_read_guard_5
                        .items
                        .get(__terrane_index_6)
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index_6,
                        )),
                    1 /* terrane-site: case.trn:23:17-23:30 */,
                );
                let __terrane_snapshot_8 = (*&*__terrane_index_place_7).clone();
                __terrane_snapshot_8
            };
            let __terrane_string_argument_0 = __terrane_raised(
                {
                    let __terrane_snapshot_10 = (*&__terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .items)
                        .clone();
                    __terrane_snapshot_10
                }
                    .get_or_error(
                        __terrane_raised(
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
                            2 /* terrane-site: case.trn:23:41-23:59 */,
                        ),
                    ),
                2 /* terrane-site: case.trn:23:41-23:59 */,
            );
            __terrane_string_receiver_9
                .contains(
                    std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
                )
        };
    }
    pub async fn keyed_read(&mut self) -> String {
        {
            let __terrane_value = String::from("second");
            self.key = __terrane_value;
        }
        return __terrane_raised(
            self.entries.get_or_error(&self.key.clone()),
            3 /* terrane-site: case.trn:27:16-27:38 */,
        );
    }
    pub async fn __terrane_shared_async_keyed_read(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> String {
        {
            let __terrane_value = String::from("second");
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .key = __terrane_value;
        }
        return __terrane_raised(
            {
                let __terrane_snapshot_11 = (*&__terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .entries)
                    .clone();
                __terrane_snapshot_11
            }
                .get_or_error(
                    &{
                        let __terrane_snapshot = __terrane_shared_handle
                            .lock()
                            .expect("shared async receiver lock poisoned")
                            .key
                            .clone();
                        __terrane_snapshot
                    },
                ),
            3 /* terrane-site: case.trn:27:16-27:38 */,
        );
    }
}
fn replace_shared(owner: std::sync::Arc<TerraneSharedCell<Holder>>) -> String {
    {
        let __terrane_owner_12 = owner.clone();
        let __terrane_value = String::from("xyz");
        __terrane_owner_12.lock().expect("shared reference lock poisoned").text = __terrane_value;
    }
    return String::from("x");
}
fn shrink_shared(
    owner: std::sync::Arc<TerraneSharedCell<Holder>>,
) -> terrane_int_support::Int {
    {
        let __terrane_owner_13 = owner.clone();
        let __terrane_value = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("z")]);
        __terrane_owner_13.lock().expect("shared reference lock poisoned").items = __terrane_value;
    }
    return terrane_int_support::Int::from(0_i128);
}
fn counted_key(owner: std::sync::Arc<TerraneSharedCell<Holder>>, key: String) -> String {
    {
        let __terrane_owner_14 = owner.clone();
        let __terrane_value = {
            let __terrane_snapshot = owner
                .lock()
                .expect("shared reference lock poisoned")
                .calls
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        __terrane_owner_14.lock().expect("shared reference lock poisoned").calls = __terrane_value;
    }
    return key;
}
fn decode_input(source: &Vec<u8>) -> Result<String, TerraneError> {
    return Ok({
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(
            terrane_string_support::decode(&source, __terrane_string_argument_0),
            4 /* terrane-site: case.trn:42:12-42:31 */,
        )?
    });
}
fn scan_input(source: &String) -> terrane_int_support::Int {
    let mut total: terrane_int_support::Int;
    let mut byte: u8;
    total = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_0 = &*source;
        let mut __terrane_iterator_0 = __terrane_iterable_0.bytes();
        loop {
            byte = match __terrane_iterator_0.next() {
                Some(item) => item,
                None => break,
            };
            if byte == 65 {
                total = total.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
    }
    return total.clone();
}
async fn pause() {
    return ();
}
async fn scan_later(source: &String) -> terrane_int_support::Int {
    let mut total: terrane_int_support::Int;
    let mut byte: u8;
    total = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_iterable_1 = &*source;
        let mut __terrane_iterator_1 = __terrane_iterable_1.bytes();
        loop {
            byte = match __terrane_iterator_1.next() {
                Some(item) => item,
                None => break,
            };
            __terrane_await(pause()).await;
            if byte == 65 {
                total = total.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
    }
    return total.clone();
}
fn main() {
    __terrane_run(async move {
        let mut item: Holder;
        let copy: Holder;
        let mut values: terrane_collection_support::List<String>;
        let mut source: String;
        let mut scalar: String;
        let mut grapheme: String;
        let mut byte: u8;
        let shared: std::sync::Arc<TerraneSharedCell<Holder>>;
        let maybe: Option<String>;
        let grid: terrane_collection_support::List<
            terrane_collection_support::List<String>,
        >;
        let mut character: String;
        let map_grid: terrane_collection_support::Map<
            String,
            terrane_collection_support::Map<String, String>,
        >;
        let alias: std::sync::Arc<TerraneSharedCell<Holder>>;
        item = Holder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&terrane_string_support::trim(&item
            .text))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(decode_input(&item
            .data), 5 /* terrane-site: case.trn:65:13-65:36 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&scan_input(&item.text)));
        copy = item.clone();
        {
            let __terrane_value = String::from("changed");
            item.text = __terrane_value;
        }
        println!("{}", terrane_scalar_support::scalar_text(&copy.text));
        println!("{}", terrane_scalar_support::scalar_text(&item.text));
        values = terrane_collection_support::List::<
            String,
        >::new(vec![String::from(" nested ")]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_15 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            6 /* terrane-site: case.trn:72:13-72:22 */); let __terrane_index_place_16
            = __terrane_raised(values.get(__terrane_index_15).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_15)),
            6 /* terrane-site: case.trn:72:13-72:22 */);
            terrane_string_support::trim(&__terrane_index_place_16) })
        );
        source = String::from("AéA");
        {
            let __terrane_iterable_2 = &source;
            let mut __terrane_iterator_2 = __terrane_iterable_2
                .chars()
                .map(|value| value.to_string());
            loop {
                scalar = match __terrane_iterator_2.next() {
                    Some(item) => item,
                    None => break,
                };
                println!("{}", terrane_scalar_support::scalar_text(&scalar));
            }
        }
        {
            let __terrane_iterable_3 = &source;
            let mut __terrane_iterator_3 = terrane_string_support::graphemes(
                &__terrane_iterable_3,
            );
            loop {
                grapheme = match __terrane_iterator_3.next() {
                    Some(item) => item,
                    None => break,
                };
                println!("{}", terrane_scalar_support::scalar_text(&grapheme));
            }
        }
        {
            let __terrane_iterable_4 = &source;
            let mut __terrane_iterator_4 = __terrane_iterable_4.bytes();
            loop {
                byte = match __terrane_iterator_4.next() {
                    Some(item) => item,
                    None => break,
                };
                println!("{}", terrane_scalar_support::scalar_text(&byte));
            }
        }
        source = String::from("ab");
        {
            let __terrane_iterable_5 = {
                let __terrane_snapshot_17 = (*&source).clone();
                __terrane_snapshot_17
            };
            let mut __terrane_iterator_5 = __terrane_iterable_5
                .chars()
                .map(|value| value.to_string());
            loop {
                scalar = match __terrane_iterator_5.next() {
                    Some(item) => item,
                    None => break,
                };
                let _ = &scalar;
                source = String::from("changed during traversal");
                println!("{}", terrane_scalar_support::scalar_text(&scalar));
            }
        }
        println!("{}", terrane_scalar_support::scalar_text(&source));
        {
            let __terrane_iterable_6 = &{
                let __terrane_receiver_value_18 = &String::from("temporary");
                terrane_string_support::upper(&__terrane_receiver_value_18)
            };
            let mut __terrane_iterator_6 = __terrane_iterable_6
                .chars()
                .map(|value| value.to_string());
            loop {
                scalar = match __terrane_iterator_6.next() {
                    Some(item) => item,
                    None => break,
                };
                let _ = &scalar;
                println!("{}", terrane_scalar_support::scalar_text(&scalar));
                break;
            }
        }
        shared = std::sync::Arc::new(
            TerraneSharedCell::new(Holder::terrane_construct()),
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_21
            = { let __terrane_owner_19 = &shared; let __terrane_snapshot_20 = (*
            &__terrane_owner_19.lock().expect("shared reference lock poisoned").text)
            .clone(); __terrane_snapshot_20 }; let __terrane_string_argument_0 =
            &replace_shared(shared.clone()); __terrane_string_receiver_21
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_22 = &shared; let __terrane_snapshot_23 = (*
            &__terrane_owner_22.lock().expect("shared reference lock poisoned").items)
            .clone(); __terrane_snapshot_23 }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&shrink_shared(shared
            .clone())), 7 /* terrane-site: case.trn:90:12-90:49 */)),
            7 /* terrane-site: case.trn:90:12-90:49 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_24 =
            &shared; terrane_string_support::trim(&__terrane_owner_24.lock()
            .expect("shared reference lock poisoned").text) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_25 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_25.lock()
            .expect("shared reference lock poisoned"); let __terrane_string_argument_0 =
            &__terrane_native_read_guard_0.text; __terrane_native_read_guard_0.text
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_26 =
            &shared; let __terrane_string_argument_0 =
            terrane_string_support::Encoding::Utf8;
            __terrane_raised(terrane_string_support::decode(&__terrane_owner_26.lock()
            .expect("shared reference lock poisoned").data, __terrane_string_argument_0),
            8 /* terrane-site: case.trn:93:13-93:37 */) })
        );
        maybe = Some(String::from(" optional "));
        if (&maybe).is_some() {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&terrane_string_support::trim(&match
                &maybe { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") }))
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(scan_later(&copy
            .text)). await)
        );
        values = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("al"), String::from("alpha")]);
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_27 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            9 /* terrane-site: case.trn:99:13-99:22 */); let __terrane_index_place_28
            = __terrane_raised(values.get(__terrane_index_27).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_27)),
            9 /* terrane-site: case.trn:99:13-99:22 */); let __terrane_index_29 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            10 /* terrane-site: case.trn:99:33-99:42 */); let __terrane_index_place_30
            = __terrane_raised(values.get(__terrane_index_29).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_29)),
            10 /* terrane-site: case.trn:99:33-99:42 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_30;
            __terrane_index_place_28.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        {
            let __terrane_value = terrane_collection_support::List::<
                String,
            >::new(vec![String::from("al"), String::from("alpha")]);
            item.items = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_31 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            11 /* terrane-site: case.trn:101:13-101:26 */); let
            __terrane_index_place_32 = __terrane_raised(item.items
            .get(__terrane_index_31).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_31)),
            11 /* terrane-site: case.trn:101:13-101:26 */); let __terrane_index_33 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            12 /* terrane-site: case.trn:101:37-101:50 */); let
            __terrane_index_place_34 = __terrane_raised(item.items
            .get(__terrane_index_33).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_33)),
            12 /* terrane-site: case.trn:101:37-101:50 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_34;
            __terrane_index_place_32.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        {
            let __terrane_owner_35 = shared.clone();
            let __terrane_value = terrane_collection_support::List::<
                String,
            >::new(vec![String::from("al"), String::from("alpha")]);
            __terrane_owner_35.lock().expect("shared reference lock poisoned").items = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_36 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_36.lock()
            .expect("shared reference lock poisoned"); let __terrane_index_37 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            13 /* terrane-site: case.trn:103:13-103:28 */); let
            __terrane_index_place_38 = __terrane_raised(__terrane_native_read_guard_0
            .items.get(__terrane_index_37).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_37)),
            13 /* terrane-site: case.trn:103:13-103:28 */); let __terrane_index_39 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            14 /* terrane-site: case.trn:103:39-103:54 */); let
            __terrane_index_place_40 = __terrane_raised(__terrane_native_read_guard_0
            .items.get(__terrane_index_39).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_39)),
            14 /* terrane-site: case.trn:103:39-103:54 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_40;
            __terrane_index_place_38.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_41
            = &String::from("escaped\n雪"); let __terrane_string_argument_0 = "雪";
            __terrane_receiver_value_41.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_42
            = &String::from("x"); let __terrane_string_argument_0 = "b";
            __terrane_receiver_value_42.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        grid = terrane_collection_support::List::<
            terrane_collection_support::List<String>,
        >::new(
            vec![
                terrane_collection_support::List::< String >::new(vec![String::from("a"),
                String::from("b")]), terrane_collection_support::List::< String
                >::new(vec![String::from("c"), String::from("d")])
            ],
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_43 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            15 /* terrane-site: case.trn:107:13-107:20 */); let
            __terrane_index_place_44 = __terrane_raised(grid.get(__terrane_index_43)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_43)),
            15 /* terrane-site: case.trn:107:13-107:20 */); let __terrane_index_45 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            16 /* terrane-site: case.trn:107:13-107:23 */); let
            __terrane_index_place_46 = __terrane_raised(__terrane_index_place_44
            .get(__terrane_index_45).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_45)),
            16 /* terrane-site: case.trn:107:13-107:23 */);
            terrane_string_support::trim(&__terrane_index_place_46) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_47 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            17 /* terrane-site: case.trn:108:13-108:20 */); let
            __terrane_index_place_48 = __terrane_raised(grid.get(__terrane_index_47)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_47)),
            17 /* terrane-site: case.trn:108:13-108:20 */); let __terrane_index_49 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            18 /* terrane-site: case.trn:108:13-108:23 */); let
            __terrane_index_place_50 = __terrane_raised(__terrane_index_place_48
            .get(__terrane_index_49).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_49)),
            18 /* terrane-site: case.trn:108:13-108:23 */); let __terrane_index_51 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            19 /* terrane-site: case.trn:108:34-108:41 */); let
            __terrane_index_place_52 = __terrane_raised(grid.get(__terrane_index_51)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_51)),
            19 /* terrane-site: case.trn:108:34-108:41 */); let __terrane_index_53 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            20 /* terrane-site: case.trn:108:34-108:44 */); let
            __terrane_index_place_54 = __terrane_raised(__terrane_index_place_52
            .get(__terrane_index_53).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_53)),
            20 /* terrane-site: case.trn:108:34-108:44 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_54;
            __terrane_index_place_50.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        {
            let __terrane_index_55 = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                ),
                21 /* terrane-site: case.trn:109:22-109:29 */,
            );
            let __terrane_index_place_56 = __terrane_raised(
                grid
                    .get(__terrane_index_55)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_55,
                    )),
                21 /* terrane-site: case.trn:109:22-109:29 */,
            );
            let __terrane_index_57 = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                22 /* terrane-site: case.trn:109:22-109:32 */,
            );
            let __terrane_index_place_58 = __terrane_raised(
                __terrane_index_place_56
                    .get(__terrane_index_57)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_57,
                    )),
                22 /* terrane-site: case.trn:109:22-109:32 */,
            );
            let __terrane_iterable_7 = &*__terrane_index_place_58;
            let mut __terrane_iterator_7 = __terrane_iterable_7
                .chars()
                .map(|value| value.to_string());
            loop {
                character = match __terrane_iterator_7.next() {
                    Some(item) => item,
                    None => break,
                };
                println!("{}", terrane_scalar_support::scalar_text(&character));
            }
        }
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_map_key_59 =
            String::from("second"); let __terrane_index_place_60 =
            __terrane_raised(map_grid.get(&__terrane_map_key_59)
            .ok_or(terrane_collection_support::MissingKey), 23 /* terrane-site: case.trn:112:13-112:31 */); let __terrane_map_key_61 = String::from("left");
            let __terrane_index_place_62 = __terrane_raised(__terrane_index_place_60
            .get(&__terrane_map_key_61).ok_or(terrane_collection_support::MissingKey),
            24 /* terrane-site: case.trn:112:13-112:39 */); let __terrane_map_key_63 =
            String::from("first"); let __terrane_index_place_64 =
            __terrane_raised(map_grid.get(&__terrane_map_key_63)
            .ok_or(terrane_collection_support::MissingKey), 25 /* terrane-site: case.trn:112:50-112:67 */); let __terrane_map_key_65 = String::from("right");
            let __terrane_index_place_66 = __terrane_raised(__terrane_index_place_64
            .get(&__terrane_map_key_65).ok_or(terrane_collection_support::MissingKey),
            26 /* terrane-site: case.trn:112:50-112:76 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_66;
            __terrane_index_place_62.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_map_key_67 =
            counted_key(shared.clone(), String::from("second")); let
            __terrane_index_place_68 = __terrane_raised(map_grid
            .get(&__terrane_map_key_67).ok_or(terrane_collection_support::MissingKey),
            27 /* terrane-site: case.trn:113:13-113:54 */); let __terrane_map_key_69 =
            counted_key(shared.clone(), String::from("left")); let
            __terrane_index_place_70 = __terrane_raised(__terrane_index_place_68
            .get(&__terrane_map_key_69).ok_or(terrane_collection_support::MissingKey),
            28 /* terrane-site: case.trn:113:13-113:85 */); let __terrane_map_key_71 =
            counted_key(shared.clone(), String::from("first")); let
            __terrane_index_place_72 = __terrane_raised(map_grid
            .get(&__terrane_map_key_71).ok_or(terrane_collection_support::MissingKey),
            29 /* terrane-site: case.trn:113:96-113:136 */); let __terrane_map_key_73
            = counted_key(shared.clone(), String::from("right")); let
            __terrane_index_place_74 = __terrane_raised(__terrane_index_place_72
            .get(&__terrane_map_key_73).ok_or(terrane_collection_support::MissingKey),
            30 /* terrane-site: case.trn:113:96-113:168 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_74;
            __terrane_index_place_70.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = shared
            .lock().expect("shared reference lock poisoned").calls.clone();
            __terrane_snapshot })
        );
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_75 =
                    __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(99_i128)),
                    31 /* terrane-site: case.trn:116:17-116:27 */); let
                    __terrane_index_place_76 = __terrane_raised_completion!(values
                    .get(__terrane_index_75).ok_or_else(| |
                    terrane_collection_support::IndexError::from_usize(__terrane_index_75)),
                    31 /* terrane-site: case.trn:116:17-116:27 */); let
                    __terrane_string_argument_0 = &counted_key(shared.clone(),
                    String::from("a")); __terrane_index_place_76
                    .contains(std::convert::AsRef:: < str >
                    ::as_ref(&__terrane_string_argument_0)) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0
                        && __terrane_error_0.kind == TerraneErrorKind::IndexError
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let
                            __terrane_snapshot = shared.lock()
                            .expect("shared reference lock poisoned").calls.clone();
                            __terrane_snapshot })
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
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_77 =
                    __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(99_i128)),
                    32 /* terrane-site: case.trn:120:17-120:25 */); let
                    __terrane_index_place_78 = __terrane_raised_completion!(grid
                    .get(__terrane_index_77).ok_or_else(| |
                    terrane_collection_support::IndexError::from_usize(__terrane_index_77)),
                    32 /* terrane-site: case.trn:120:17-120:25 */); let
                    __terrane_index_79 =
                    __terrane_raised_completion!(terrane_collection_support::index_from_int(&shrink_shared(shared
                    .clone())), 33 /* terrane-site: case.trn:120:17-120:50 */); let
                    __terrane_index_place_80 =
                    __terrane_raised_completion!(__terrane_index_place_78
                    .get(__terrane_index_79).ok_or_else(| |
                    terrane_collection_support::IndexError::from_usize(__terrane_index_79)),
                    33 /* terrane-site: case.trn:120:17-120:50 */);
                    terrane_string_support::trim(&__terrane_index_place_80) })
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_1 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_1) => {
                    let mut __terrane_handled_1 = false;
                    if !__terrane_handled_1
                        && __terrane_error_1.kind == TerraneErrorKind::IndexError
                    {
                        __terrane_handled_1 = true;
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&{ let
                            __terrane_owner_81 = &shared;
                            __terrane_raised_completion!(__terrane_owner_81.lock()
                            .expect("shared reference lock poisoned").items
                            .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                            34 /* terrane-site: case.trn:122:16-122:31 */)),
                            34 /* terrane-site: case.trn:122:16-122:31 */) })
                        );
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
        {
            let __terrane_owner_82 = shared.clone();
            let __terrane_value = String::from("alpha");
            __terrane_owner_82.lock().expect("shared reference lock poisoned").text = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_83 = &shared; let __terrane_snapshot_84 = (*
            &__terrane_owner_83.lock().expect("shared reference lock poisoned").items)
            .clone(); __terrane_snapshot_84 }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&{
            let __terrane_snapshot = shared.lock()
            .expect("shared reference lock poisoned").n.clone(); __terrane_snapshot }),
            35 /* terrane-site: case.trn:124:12-124:34 */)), 35 /* terrane-site: case.trn:124:12-124:34 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_90
            = { let __terrane_owner_85 = &shared; let __terrane_read_guard_86 =
            __terrane_owner_85.lock().expect("shared reference lock poisoned"); let
            __terrane_index_87 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            36 /* terrane-site: case.trn:125:13-125:28 */); let
            __terrane_index_place_88 = __terrane_raised(__terrane_read_guard_86.items
            .get(__terrane_index_87).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_87)),
            36 /* terrane-site: case.trn:125:13-125:28 */); let __terrane_snapshot_89
            = (* &* __terrane_index_place_88).clone(); __terrane_snapshot_89 }; let
            __terrane_string_argument_0 = __terrane_raised({ let __terrane_owner_91 =
            &shared; let __terrane_snapshot_92 = (* &__terrane_owner_91.lock()
            .expect("shared reference lock poisoned").items).clone();
            __terrane_snapshot_92 }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&{
            let __terrane_snapshot = shared.lock()
            .expect("shared reference lock poisoned").n.clone(); __terrane_snapshot }),
            37 /* terrane-site: case.trn:125:39-125:61 */)), 37 /* terrane-site: case.trn:125:39-125:61 */); __terrane_string_receiver_90
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        alias = shared.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_93 = &shared; let __terrane_snapshot_94 = (*
            &__terrane_owner_93.lock().expect("shared reference lock poisoned").items)
            .clone(); __terrane_snapshot_94 }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&{
            let __terrane_snapshot = alias.lock()
            .expect("shared reference lock poisoned").n.clone(); __terrane_snapshot }),
            38 /* terrane-site: case.trn:127:12-127:33 */)), 38 /* terrane-site: case.trn:127:12-127:33 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_string_receiver_100 = { let __terrane_owner_95 = &shared; let
            __terrane_read_guard_96 = __terrane_owner_95.lock()
            .expect("shared reference lock poisoned"); let __terrane_index_97 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            39 /* terrane-site: case.trn:128:13-128:28 */); let
            __terrane_index_place_98 = __terrane_raised(__terrane_read_guard_96.items
            .get(__terrane_index_97).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_97)),
            39 /* terrane-site: case.trn:128:13-128:28 */); let __terrane_snapshot_99
            = (* &* __terrane_index_place_98).clone(); __terrane_snapshot_99 }; let
            __terrane_string_argument_0 = { let __terrane_owner_101 = &alias;
            __terrane_raised(__terrane_owner_101.lock()
            .expect("shared reference lock poisoned").items
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            40 /* terrane-site: case.trn:128:39-128:53 */)), 40 /* terrane-site: case.trn:128:39-128:53 */) }; __terrane_string_receiver_100
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_string_receiver_104 = { let __terrane_owner_102 = &shared; let
            __terrane_snapshot_103 = (* &__terrane_owner_102.lock()
            .expect("shared reference lock poisoned").text).clone();
            __terrane_snapshot_103 }; let __terrane_string_argument_0 = { let
            __terrane_snapshot = alias.lock().expect("shared reference lock poisoned")
            .text.clone(); __terrane_snapshot }; __terrane_string_receiver_104
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_105 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_105.lock()
            .expect("shared reference lock poisoned"); let __terrane_index_106 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            41 /* terrane-site: case.trn:130:35-130:50 */); let
            __terrane_index_place_107 = __terrane_raised(__terrane_native_read_guard_0
            .items.get(__terrane_index_106).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_106)),
            41 /* terrane-site: case.trn:130:35-130:50 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_107;
            __terrane_native_read_guard_0.text.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_108 = &shared; let __terrane_snapshot_109 = (*
            &__terrane_owner_108.lock().expect("shared reference lock poisoned").entries)
            .clone(); __terrane_snapshot_109 } .get_or_error(&{ let __terrane_snapshot =
            shared.lock().expect("shared reference lock poisoned").key.clone();
            __terrane_snapshot }), 42 /* terrane-site: case.trn:131:12-131:38 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_string_receiver_116 = { let __terrane_index_collection_112 = { let
            __terrane_owner_110 = &shared; let __terrane_read_guard_111 =
            __terrane_owner_110.lock().expect("shared reference lock poisoned");
            __terrane_read_guard_111.entries.clone() }; let __terrane_map_key_113 = { let
            __terrane_snapshot = alias.lock().expect("shared reference lock poisoned")
            .key.clone(); __terrane_snapshot }; let __terrane_index_place_114 =
            __terrane_raised(__terrane_index_collection_112.get(&__terrane_map_key_113)
            .ok_or(terrane_collection_support::MissingKey), 43 /* terrane-site: case.trn:132:13-132:38 */); let __terrane_snapshot_115 = (* &*
            __terrane_index_place_114).clone(); __terrane_snapshot_115 }; let
            __terrane_string_argument_0 = { let __terrane_owner_117 = &shared;
            __terrane_raised(__terrane_owner_117.lock()
            .expect("shared reference lock poisoned").entries
            .get_or_error(&String::from("first")), 44 /* terrane-site: case.trn:132:49-132:72 */) }; __terrane_string_receiver_116
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_string_receiver_120 = { let __terrane_owner_118 = &shared; let
            __terrane_snapshot_119 = (* &__terrane_owner_118.lock()
            .expect("shared reference lock poisoned").text).clone();
            __terrane_snapshot_119 }; let __terrane_string_argument_0 = { let
            __terrane_owner_121 = &shared; __terrane_raised(__terrane_owner_121.lock()
            .expect("shared reference lock poisoned").items
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            45 /* terrane-site: case.trn:133:34-133:49 */)), 45 /* terrane-site: case.trn:133:34-133:49 */) }; let __terrane_string_argument_1 = { let
            __terrane_owner_122 = &alias; __terrane_raised(__terrane_owner_122.lock()
            .expect("shared reference lock poisoned").items
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            46 /* terrane-site: case.trn:133:51-133:65 */)), 46 /* terrane-site: case.trn:133:51-133:65 */) };
            terrane_string_support::replace(&__terrane_string_receiver_120,
            &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0),
            &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_1)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_string_receiver_125 = { let __terrane_owner_123 = &shared; let
            __terrane_snapshot_124 = (* &__terrane_owner_123.lock()
            .expect("shared reference lock poisoned").data).clone();
            __terrane_snapshot_124 }; let __terrane_string_argument_0 = { let
            __terrane_snapshot = shared.lock().expect("shared reference lock poisoned")
            .codec.clone(); __terrane_snapshot };
            __terrane_raised(terrane_string_support::decode(&__terrane_string_receiver_125,
            __terrane_string_argument_0), 47 /* terrane-site: case.trn:134:13-134:45 */) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = shared.clone();
            Holder::__terrane_shared_async_indexed_read(__terrane_shared_call_handle) }).
            await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = shared.clone();
            Holder::__terrane_shared_async_indexed_contains(__terrane_shared_call_handle)
            }). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await({ let
            __terrane_shared_call_handle = shared.clone();
            Holder::__terrane_shared_async_keyed_read(__terrane_shared_call_handle) }).
            await)
        );
    });
}
