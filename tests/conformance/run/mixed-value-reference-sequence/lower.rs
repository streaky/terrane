// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: mixed-value-reference-sequence
fn main() {
    let numbers: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let number_snapshot: terrane_collection_support::List<terrane_int_support::Int>;
    let number_owner: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let number_observer: std::sync::Weak<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let words: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<String>>,
    >;
    let word_snapshot: terrane_collection_support::List<String>;
    let word_owner: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<String>>,
    >;
    let word_observer: std::sync::Weak<
        std::sync::Mutex<terrane_collection_support::List<String>>,
    >;
    numbers = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        ),
    );
    number_snapshot = {
        let __terrane_value = numbers.lock().expect("reference lock poisoned").clone();
        __terrane_value
    }
        .clone();
    numbers
        .lock()
        .expect("reference lock poisoned")
        .append(terrane_int_support::Int::from(2_i128));
    number_owner = numbers.clone();
    number_observer = std::sync::Arc::downgrade(&numbers.clone());
    number_owner
        .lock()
        .expect("shared reference lock poisoned")
        .append(terrane_int_support::Int::from(3_i128));
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(number_snapshot
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = numbers.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_owner = number_observer.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value } .length()))
    );
    words = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<String>::new(vec![String::from("a")]),
        ),
    );
    word_snapshot = {
        let __terrane_value = words.lock().expect("reference lock poisoned").clone();
        __terrane_value
    }
        .clone();
    words.lock().expect("reference lock poisoned").append(String::from("b"));
    word_owner = words.clone();
    word_observer = std::sync::Arc::downgrade(&words.clone());
    word_owner.lock().expect("shared reference lock poisoned").append(String::from("c"));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(word_snapshot
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:20:10-20:26 */)), 0 /* terrane-site: case.trn:20:10-20:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised({ let __terrane_value =
        words.lock().expect("reference lock poisoned").clone(); __terrane_value }
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:20:28-20:36 */)), 1 /* terrane-site: case.trn:20:28-20:36 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised({ let __terrane_owner =
        word_observer.upgrade().expect("reference expired"); let __terrane_value =
        __terrane_owner.lock().expect("reference lock poisoned").clone(); __terrane_value
        }
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        2 /* terrane-site: case.trn:20:38-20:54 */)), 2 /* terrane-site: case.trn:20:38-20:54 */))
    );
}
