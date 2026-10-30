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
}
fn replace_shared(owner: std::sync::Arc<TerraneSharedCell<Holder>>) -> String {
    {
        let __terrane_owner_0 = owner.clone();
        let __terrane_value = String::from("xyz");
        __terrane_owner_0.lock().expect("shared reference lock poisoned").text = __terrane_value;
    }
    return String::from("x");
}
fn shrink_shared(
    owner: std::sync::Arc<TerraneSharedCell<Holder>>,
) -> terrane_int_support::Int {
    {
        let __terrane_owner_1 = owner.clone();
        let __terrane_value = terrane_collection_support::List::<
            String,
        >::new(vec![String::from("z")]);
        __terrane_owner_1.lock().expect("shared reference lock poisoned").items = __terrane_value;
    }
    return terrane_int_support::Int::from(0_i128);
}
fn counted_key(owner: std::sync::Arc<TerraneSharedCell<Holder>>, key: String) -> String {
    {
        let __terrane_owner_2 = owner.clone();
        let __terrane_value = {
            let __terrane_snapshot = owner
                .lock()
                .expect("shared reference lock poisoned")
                .calls
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        __terrane_owner_2.lock().expect("shared reference lock poisoned").calls = __terrane_value;
    }
    return key;
}
fn decode_input(source: &Vec<u8>) -> Result<String, TerraneError> {
    return Ok({
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(
            terrane_string_support::decode(&source, __terrane_string_argument_0),
            0 /* terrane-site: case.trn:30:12-30:31 */,
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
            .data), 1 /* terrane-site: case.trn:53:13-53:36 */))
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_3 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            2 /* terrane-site: case.trn:60:13-60:22 */); let __terrane_index_place_4 =
            __terrane_raised(values.get(__terrane_index_3).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_3)),
            2 /* terrane-site: case.trn:60:13-60:22 */);
            terrane_string_support::trim(&__terrane_index_place_4) })
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
            let __terrane_iterable_5 = (*&source).clone();
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
                let __terrane_receiver_value_5 = &String::from("temporary");
                terrane_string_support::upper(&__terrane_receiver_value_5)
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_7
            = { let __terrane_owner_6 = &shared; (* &__terrane_owner_6.lock()
            .expect("shared reference lock poisoned").text).clone() }; let
            __terrane_string_argument_0 = &replace_shared(shared.clone());
            __terrane_string_receiver_7.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_8 = &shared; (* &__terrane_owner_8.lock()
            .expect("shared reference lock poisoned").items).clone() }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&shrink_shared(shared
            .clone())), 3 /* terrane-site: case.trn:78:12-78:49 */)),
            3 /* terrane-site: case.trn:78:12-78:49 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_9 = &shared;
            terrane_string_support::trim(&__terrane_owner_9.lock()
            .expect("shared reference lock poisoned").text) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_10 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_10.lock()
            .expect("shared reference lock poisoned"); let __terrane_string_argument_0 =
            &__terrane_native_read_guard_0.text; __terrane_native_read_guard_0.text
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_11 =
            &shared; let __terrane_string_argument_0 =
            terrane_string_support::Encoding::Utf8;
            __terrane_raised(terrane_string_support::decode(&__terrane_owner_11.lock()
            .expect("shared reference lock poisoned").data, __terrane_string_argument_0),
            4 /* terrane-site: case.trn:81:13-81:37 */) })
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_12 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            5 /* terrane-site: case.trn:87:13-87:22 */); let __terrane_index_place_13
            = __terrane_raised(values.get(__terrane_index_12).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_12)),
            5 /* terrane-site: case.trn:87:13-87:22 */); let __terrane_index_14 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            6 /* terrane-site: case.trn:87:33-87:42 */); let __terrane_index_place_15
            = __terrane_raised(values.get(__terrane_index_14).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_14)),
            6 /* terrane-site: case.trn:87:33-87:42 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_15;
            __terrane_index_place_13.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        {
            let __terrane_value = terrane_collection_support::List::<
                String,
            >::new(vec![String::from("al"), String::from("alpha")]);
            item.items = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_16 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            7 /* terrane-site: case.trn:89:13-89:26 */); let __terrane_index_place_17
            = __terrane_raised(item.items.get(__terrane_index_16).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_16)),
            7 /* terrane-site: case.trn:89:13-89:26 */); let __terrane_index_18 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            8 /* terrane-site: case.trn:89:37-89:50 */); let __terrane_index_place_19
            = __terrane_raised(item.items.get(__terrane_index_18).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_18)),
            8 /* terrane-site: case.trn:89:37-89:50 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_19;
            __terrane_index_place_17.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        {
            let __terrane_owner_20 = shared.clone();
            let __terrane_value = terrane_collection_support::List::<
                String,
            >::new(vec![String::from("al"), String::from("alpha")]);
            __terrane_owner_20.lock().expect("shared reference lock poisoned").items = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_21 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_21.lock()
            .expect("shared reference lock poisoned"); let __terrane_index_22 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            9 /* terrane-site: case.trn:91:13-91:28 */); let __terrane_index_place_23
            = __terrane_raised(__terrane_native_read_guard_0.items
            .get(__terrane_index_22).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_22)),
            9 /* terrane-site: case.trn:91:13-91:28 */); let __terrane_index_24 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            10 /* terrane-site: case.trn:91:39-91:54 */); let __terrane_index_place_25
            = __terrane_raised(__terrane_native_read_guard_0.items
            .get(__terrane_index_24).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_24)),
            10 /* terrane-site: case.trn:91:39-91:54 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_25;
            __terrane_index_place_23.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_26
            = &String::from("escaped\n雪"); let __terrane_string_argument_0 = "雪";
            __terrane_receiver_value_26.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_27
            = &String::from("x"); let __terrane_string_argument_0 = "b";
            __terrane_receiver_value_27.contains(std::convert::AsRef:: < str >
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_28 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            11 /* terrane-site: case.trn:95:13-95:20 */); let __terrane_index_place_29
            = __terrane_raised(grid.get(__terrane_index_28).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_28)),
            11 /* terrane-site: case.trn:95:13-95:20 */); let __terrane_index_30 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            12 /* terrane-site: case.trn:95:13-95:23 */); let __terrane_index_place_31
            = __terrane_raised(__terrane_index_place_29.get(__terrane_index_30)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_30)),
            12 /* terrane-site: case.trn:95:13-95:23 */);
            terrane_string_support::trim(&__terrane_index_place_31) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_32 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            13 /* terrane-site: case.trn:96:13-96:20 */); let __terrane_index_place_33
            = __terrane_raised(grid.get(__terrane_index_32).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_32)),
            13 /* terrane-site: case.trn:96:13-96:20 */); let __terrane_index_34 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            14 /* terrane-site: case.trn:96:13-96:23 */); let __terrane_index_place_35
            = __terrane_raised(__terrane_index_place_33.get(__terrane_index_34)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_34)),
            14 /* terrane-site: case.trn:96:13-96:23 */); let __terrane_index_36 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            15 /* terrane-site: case.trn:96:34-96:41 */); let __terrane_index_place_37
            = __terrane_raised(grid.get(__terrane_index_36).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_36)),
            15 /* terrane-site: case.trn:96:34-96:41 */); let __terrane_index_38 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            16 /* terrane-site: case.trn:96:34-96:44 */); let __terrane_index_place_39
            = __terrane_raised(__terrane_index_place_37.get(__terrane_index_38)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_38)),
            16 /* terrane-site: case.trn:96:34-96:44 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_39;
            __terrane_index_place_35.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        {
            let __terrane_index_40 = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                ),
                17 /* terrane-site: case.trn:97:22-97:29 */,
            );
            let __terrane_index_place_41 = __terrane_raised(
                grid
                    .get(__terrane_index_40)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_40,
                    )),
                17 /* terrane-site: case.trn:97:22-97:29 */,
            );
            let __terrane_index_42 = __terrane_raised(
                terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                ),
                18 /* terrane-site: case.trn:97:22-97:32 */,
            );
            let __terrane_index_place_43 = __terrane_raised(
                __terrane_index_place_41
                    .get(__terrane_index_42)
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index_42,
                    )),
                18 /* terrane-site: case.trn:97:22-97:32 */,
            );
            let __terrane_iterable_7 = &*__terrane_index_place_43;
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_map_key_44 =
            String::from("second"); let __terrane_index_place_45 =
            __terrane_raised(map_grid.get(&__terrane_map_key_44)
            .ok_or(terrane_collection_support::MissingKey), 19 /* terrane-site: case.trn:100:13-100:31 */); let __terrane_map_key_46 = String::from("left");
            let __terrane_index_place_47 = __terrane_raised(__terrane_index_place_45
            .get(&__terrane_map_key_46).ok_or(terrane_collection_support::MissingKey),
            20 /* terrane-site: case.trn:100:13-100:39 */); let __terrane_map_key_48 =
            String::from("first"); let __terrane_index_place_49 =
            __terrane_raised(map_grid.get(&__terrane_map_key_48)
            .ok_or(terrane_collection_support::MissingKey), 21 /* terrane-site: case.trn:100:50-100:67 */); let __terrane_map_key_50 = String::from("right");
            let __terrane_index_place_51 = __terrane_raised(__terrane_index_place_49
            .get(&__terrane_map_key_50).ok_or(terrane_collection_support::MissingKey),
            22 /* terrane-site: case.trn:100:50-100:76 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_51;
            __terrane_index_place_47.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_map_key_52 =
            counted_key(shared.clone(), String::from("second")); let
            __terrane_index_place_53 = __terrane_raised(map_grid
            .get(&__terrane_map_key_52).ok_or(terrane_collection_support::MissingKey),
            23 /* terrane-site: case.trn:101:13-101:54 */); let __terrane_map_key_54 =
            counted_key(shared.clone(), String::from("left")); let
            __terrane_index_place_55 = __terrane_raised(__terrane_index_place_53
            .get(&__terrane_map_key_54).ok_or(terrane_collection_support::MissingKey),
            24 /* terrane-site: case.trn:101:13-101:85 */); let __terrane_map_key_56 =
            counted_key(shared.clone(), String::from("first")); let
            __terrane_index_place_57 = __terrane_raised(map_grid
            .get(&__terrane_map_key_56).ok_or(terrane_collection_support::MissingKey),
            25 /* terrane-site: case.trn:101:96-101:136 */); let __terrane_map_key_58
            = counted_key(shared.clone(), String::from("right")); let
            __terrane_index_place_59 = __terrane_raised(__terrane_index_place_57
            .get(&__terrane_map_key_58).ok_or(terrane_collection_support::MissingKey),
            26 /* terrane-site: case.trn:101:96-101:168 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_59;
            __terrane_index_place_55.contains(std::convert::AsRef:: < str >
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
                    "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_60 =
                    __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(99_i128)),
                    27 /* terrane-site: case.trn:104:17-104:27 */); let
                    __terrane_index_place_61 = __terrane_raised_completion!(values
                    .get(__terrane_index_60).ok_or_else(| |
                    terrane_collection_support::IndexError::from_usize(__terrane_index_60)),
                    27 /* terrane-site: case.trn:104:17-104:27 */); let
                    __terrane_string_argument_0 = &counted_key(shared.clone(),
                    String::from("a")); __terrane_index_place_61
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
                    "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_62 =
                    __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(99_i128)),
                    28 /* terrane-site: case.trn:108:17-108:25 */); let
                    __terrane_index_place_63 = __terrane_raised_completion!(grid
                    .get(__terrane_index_62).ok_or_else(| |
                    terrane_collection_support::IndexError::from_usize(__terrane_index_62)),
                    28 /* terrane-site: case.trn:108:17-108:25 */); let
                    __terrane_index_64 =
                    __terrane_raised_completion!(terrane_collection_support::index_from_int(&shrink_shared(shared
                    .clone())), 29 /* terrane-site: case.trn:108:17-108:50 */); let
                    __terrane_index_place_65 =
                    __terrane_raised_completion!(__terrane_index_place_63
                    .get(__terrane_index_64).ok_or_else(| |
                    terrane_collection_support::IndexError::from_usize(__terrane_index_64)),
                    29 /* terrane-site: case.trn:108:17-108:50 */);
                    terrane_string_support::trim(&__terrane_index_place_65) })
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
                            __terrane_owner_66 = &shared;
                            __terrane_raised_completion!(__terrane_owner_66.lock()
                            .expect("shared reference lock poisoned").items
                            .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                            30 /* terrane-site: case.trn:110:16-110:31 */)),
                            30 /* terrane-site: case.trn:110:16-110:31 */) })
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
            let __terrane_owner_67 = shared.clone();
            let __terrane_value = String::from("alpha");
            __terrane_owner_67.lock().expect("shared reference lock poisoned").text = __terrane_value;
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_68 = &shared; (* &__terrane_owner_68.lock()
            .expect("shared reference lock poisoned").items).clone() }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&{
            let __terrane_snapshot = shared.lock()
            .expect("shared reference lock poisoned").n.clone(); __terrane_snapshot }),
            31 /* terrane-site: case.trn:112:12-112:34 */)), 31 /* terrane-site: case.trn:112:12-112:34 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_73
            = { let __terrane_owner_69 = &shared; let __terrane_read_guard_70 =
            __terrane_owner_69.lock().expect("shared reference lock poisoned"); let
            __terrane_index_71 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            32 /* terrane-site: case.trn:113:13-113:28 */); let
            __terrane_index_place_72 = __terrane_raised(__terrane_read_guard_70.items
            .get(__terrane_index_71).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_71)),
            32 /* terrane-site: case.trn:113:13-113:28 */); (* &*
            __terrane_index_place_72).clone() }; let __terrane_string_argument_0 =
            __terrane_raised({ let __terrane_owner_74 = &shared; (* &__terrane_owner_74
            .lock().expect("shared reference lock poisoned").items).clone() }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&{
            let __terrane_snapshot = shared.lock()
            .expect("shared reference lock poisoned").n.clone(); __terrane_snapshot }),
            33 /* terrane-site: case.trn:113:39-113:61 */)), 33 /* terrane-site: case.trn:113:39-113:61 */); __terrane_string_receiver_73
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        alias = shared.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_75 = &shared; (* &__terrane_owner_75.lock()
            .expect("shared reference lock poisoned").items).clone() }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&{
            let __terrane_snapshot = alias.lock()
            .expect("shared reference lock poisoned").n.clone(); __terrane_snapshot }),
            34 /* terrane-site: case.trn:115:12-115:33 */)), 34 /* terrane-site: case.trn:115:12-115:33 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_80
            = { let __terrane_owner_76 = &shared; let __terrane_read_guard_77 =
            __terrane_owner_76.lock().expect("shared reference lock poisoned"); let
            __terrane_index_78 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            35 /* terrane-site: case.trn:116:13-116:28 */); let
            __terrane_index_place_79 = __terrane_raised(__terrane_read_guard_77.items
            .get(__terrane_index_78).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_78)),
            35 /* terrane-site: case.trn:116:13-116:28 */); (* &*
            __terrane_index_place_79).clone() }; let __terrane_string_argument_0 = { let
            __terrane_owner_81 = &alias; __terrane_raised(__terrane_owner_81.lock()
            .expect("shared reference lock poisoned").items
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            36 /* terrane-site: case.trn:116:39-116:53 */)), 36 /* terrane-site: case.trn:116:39-116:53 */) }; __terrane_string_receiver_80
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_83
            = { let __terrane_owner_82 = &shared; (* &__terrane_owner_82.lock()
            .expect("shared reference lock poisoned").text).clone() }; let
            __terrane_string_argument_0 = { let __terrane_snapshot = alias.lock()
            .expect("shared reference lock poisoned").text.clone(); __terrane_snapshot };
            __terrane_string_receiver_83.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_84 =
            &shared; let __terrane_native_read_guard_0 = &* __terrane_owner_84.lock()
            .expect("shared reference lock poisoned"); let __terrane_index_85 =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            37 /* terrane-site: case.trn:118:35-118:50 */); let
            __terrane_index_place_86 = __terrane_raised(__terrane_native_read_guard_0
            .items.get(__terrane_index_85).ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index_85)),
            37 /* terrane-site: case.trn:118:35-118:50 */); let
            __terrane_string_argument_0 = &* __terrane_index_place_86;
            __terrane_native_read_guard_0.text.contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised({ let
            __terrane_owner_87 = &shared; (* &__terrane_owner_87.lock()
            .expect("shared reference lock poisoned").entries).clone() } .get_or_error(&{
            let __terrane_snapshot = shared.lock()
            .expect("shared reference lock poisoned").key.clone(); __terrane_snapshot }),
            38 /* terrane-site: case.trn:119:12-119:38 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_93
            = { let __terrane_index_collection_90 = { let __terrane_owner_88 = &shared;
            let __terrane_read_guard_89 = __terrane_owner_88.lock()
            .expect("shared reference lock poisoned"); __terrane_read_guard_89.entries
            .clone() }; let __terrane_map_key_91 = { let __terrane_snapshot = alias
            .lock().expect("shared reference lock poisoned").key.clone();
            __terrane_snapshot }; let __terrane_index_place_92 =
            __terrane_raised(__terrane_index_collection_90.get(&__terrane_map_key_91)
            .ok_or(terrane_collection_support::MissingKey), 39 /* terrane-site: case.trn:120:13-120:38 */); (* &* __terrane_index_place_92).clone() }; let
            __terrane_string_argument_0 = { let __terrane_owner_94 = &shared;
            __terrane_raised(__terrane_owner_94.lock()
            .expect("shared reference lock poisoned").entries
            .get_or_error(&String::from("first")), 40 /* terrane-site: case.trn:120:49-120:72 */) }; __terrane_string_receiver_93
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_receiver_96
            = { let __terrane_owner_95 = &shared; (* &__terrane_owner_95.lock()
            .expect("shared reference lock poisoned").text).clone() }; let
            __terrane_string_argument_0 = { let __terrane_owner_97 = &shared;
            __terrane_raised(__terrane_owner_97.lock()
            .expect("shared reference lock poisoned").items
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            41 /* terrane-site: case.trn:121:34-121:49 */)), 41 /* terrane-site: case.trn:121:34-121:49 */) }; let __terrane_string_argument_1 = { let
            __terrane_owner_98 = &alias; __terrane_raised(__terrane_owner_98.lock()
            .expect("shared reference lock poisoned").items
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            42 /* terrane-site: case.trn:121:51-121:65 */)), 42 /* terrane-site: case.trn:121:51-121:65 */) };
            terrane_string_support::replace(&__terrane_string_receiver_96,
            &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0),
            &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_1)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let
            __terrane_string_receiver_100 = { let __terrane_owner_99 = &shared; (*
            &__terrane_owner_99.lock().expect("shared reference lock poisoned").data)
            .clone() }; let __terrane_string_argument_0 = { let __terrane_snapshot =
            shared.lock().expect("shared reference lock poisoned").codec.clone();
            __terrane_snapshot };
            __terrane_raised(terrane_string_support::decode(&__terrane_string_receiver_100,
            __terrane_string_argument_0), 43 /* terrane-site: case.trn:122:13-122:45 */) })
        );
    });
}
