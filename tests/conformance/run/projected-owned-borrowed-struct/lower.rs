// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut descriptor: Descriptor;
    let owned: Owned;
    let first: String;
    let defaults: Descriptor;
    let indexed: Owned;
    let noted: Owned;
    let zero: Owned;
    descriptor = Descriptor::terrane_construct(
        String::from("main"),
        terrane_collection_support::List::<
            String,
        >::new(vec![String::from("ab"), String::from("c")]),
        35,
        String::from("ready"),
        terrane_collection_support::List::<String>::new(vec![String::from("five")]),
    );
    owned = terrane_borrowed_struct_witness::Owned {
        label: String::from("owned"),
        count: 7,
        note: None::<String>,
        index: None::<i64>,
    };
    first = __terrane_raised(
        summarize(&descriptor),
        0 /* terrane-site: src/main.trn:9:20-9:41 */,
    );
    {
        let __terrane_value = 36;
        descriptor.offset = __terrane_value;
    }
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&first), String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&owned),
        1 /* terrane-site: src/main.trn:11:25-11:47 */)), String::from(":"),
        terrane_scalar_support::scalar_text(&owned.count), String::from(":"),
        terrane_scalar_support::scalar_text(&__terrane_raised(consume(descriptor),
        2 /* terrane-site: src/main.trn:11:74-11:93 */))
    );
    defaults = Descriptor::terrane_construct(
        String::from("defaults"),
        terrane_collection_support::List::<String>::new(vec![String::from("x")]),
        1,
        None::<String>,
        None::<terrane_collection_support::List<String>>,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(summarize(&defaults),
        3 /* terrane-site: src/main.trn:13:13-13:32 */))
    );
    indexed = terrane_borrowed_struct_witness::Owned {
        label: String::from("indexed"),
        count: 8,
        note: None::<String>,
        index: 42.into(),
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&indexed),
        4 /* terrane-site: src/main.trn:15:13-15:37 */))
    );
    noted = terrane_borrowed_struct_witness::Owned {
        label: String::from("noted"),
        count: 9,
        note: String::from("retained").into(),
        index: None::<i64>,
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&noted),
        5 /* terrane-site: src/main.trn:17:13-17:35 */))
    );
    zero = terrane_borrowed_struct_witness::Owned {
        label: String::from("zero"),
        count: 10,
        note: String::from("both").into(),
        index: 0.into(),
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(summarize_owned(&zero),
        6 /* terrane-site: src/main.trn:19:13-19:34 */))
    );
}
