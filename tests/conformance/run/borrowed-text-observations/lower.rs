// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: borrowed-text-observations
#[derive(Clone)]
pub struct Holder {
    pub text: String,
    pub data: Vec<u8>,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            text: String::from(" AéA "),
            data: Vec::<u8>::from([65, 66, 67]),
        }
    }
}
fn decode_input(source: &Vec<u8>) -> Result<String, TerraneError> {
    return Ok({
        let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(
            terrane_string_support::decode(&source, __terrane_string_argument_0),
            0 /* terrane-site: case.trn:11:12-11:31 */,
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
        let values: terrane_collection_support::List<String>;
        let mut source: String;
        let mut scalar: String;
        let mut grapheme: String;
        let mut byte: u8;
        let shared: std::sync::Arc<TerraneSharedCell<Holder>>;
        let maybe: Option<String>;
        item = Holder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&terrane_string_support::trim(&item
            .text))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(decode_input(&item
            .data), 1 /* terrane-site: case.trn:34:13-34:36 */))
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index =
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            2 /* terrane-site: case.trn:41:13-41:22 */);
            terrane_string_support::trim(&__terrane_raised(values.get(__terrane_index)
            .ok_or_else(| |
            terrane_collection_support::IndexError::from_usize(__terrane_index)),
            2 /* terrane-site: case.trn:41:13-41:22 */)) })
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
                let __terrane_receiver_value = &String::from("temporary");
                terrane_string_support::upper(&__terrane_receiver_value)
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
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &shared;
            terrane_string_support::trim(&__terrane_owner.lock()
            .expect("shared reference lock poisoned").text) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &shared;
            let __terrane_native_read_guard_0 = &* __terrane_owner.lock()
            .expect("shared reference lock poisoned"); let __terrane_string_argument_0 =
            &__terrane_native_read_guard_0.text; __terrane_native_read_guard_0.text
            .contains(std::convert::AsRef:: < str >
            ::as_ref(&__terrane_string_argument_0)) })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = &shared;
            let __terrane_string_argument_0 = terrane_string_support::Encoding::Utf8;
            __terrane_raised(terrane_string_support::decode(&__terrane_owner.lock()
            .expect("shared reference lock poisoned").data, __terrane_string_argument_0),
            3 /* terrane-site: case.trn:60:13-60:37 */) })
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
    });
}
