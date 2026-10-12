// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: text-families
fn main() {
    let text: String;
    let decomposed: String;
    let rtl: String;
    let searches: terrane_collection_support::List<String>;
    let html_replacements: terrane_collection_support::List<String>;
    let reversed_searches: terrane_collection_support::List<String>;
    let reversed_replacements: terrane_collection_support::List<String>;
    let broadcast_searches: terrane_collection_support::List<String>;
    let priority_searches: terrane_collection_support::List<String>;
    let priority_replacements: terrane_collection_support::List<String>;
    let no_searches: terrane_collection_support::List<String>;
    let no_replacements: terrane_collection_support::List<String>;
    let mismatch_searches: terrane_collection_support::List<String>;
    text = String::from("  Straße  ");
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_string_support::trim(&text))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::trim_start(&text,
        None)),
        terrane_scalar_support::scalar_text(&terrane_string_support::trim_end(&text,
        None))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::case_fold(&text))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::upper_first(&String::from("straße"))),
        terrane_scalar_support::scalar_text(&terrane_string_support::upper_words(&String::from("hello world")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::lower_first(&String::from("Hello")))
    );
    decomposed = String::from("e\u{301}");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::normalise(&decomposed,
        "nfc"))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::replace(&String::from("banana"),
        &String::from("ana"), &String::from("X")))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(terrane_string_support::find_all(&String::from("banana"),
        &String::from("ana")).len() as i128)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(terrane_string_support::find_all(&String::from("banana"),
        &String::from("")).len() as i128))
    );
    rtl = String::from("שלום");
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&rtl
        .starts_with(&String::from("ש"))), terrane_scalar_support::scalar_text(&rtl
        .ends_with(&String::from("ם")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::find_all(&decomposed,
        &String::from("")).len() as i128))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::split(&decomposed,
        &String::from("")).len() as i128))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::replace(&decomposed,
        &String::from(""), &String::from("X")))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(decomposed.len() as i128)),
        terrane_scalar_support::scalar_text(&(decomposed.chars().count() as i128)),
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&decomposed)
        as i128))
    );
    searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("&"), String::from("<")]);
    html_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("&amp;"), String::from("&lt;")]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::replace_many(&String::from("&<"),
        searches.as_slice(), html_replacements.as_slice()), 0 /* terrane-site: case.trn:25:11-25:52 */))
    );
    reversed_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("<"), String::from("&")]);
    reversed_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("&lt;"), String::from("&amp;")]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::replace_many(&String::from("&<"),
        reversed_searches.as_slice(), reversed_replacements.as_slice()),
        1 /* terrane-site: case.trn:28:11-28:65 */))
    );
    broadcast_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("one"), String::from("two")]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::replace_broadcast(&String::from("one and two"),
        broadcast_searches.as_slice(), &String::from("x")))
    );
    priority_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("a"), String::from("ab")]);
    priority_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("first"), String::from("second")]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::replace_many(&String::from("ab"),
        priority_searches.as_slice(), priority_replacements.as_slice()),
        2 /* terrane-site: case.trn:33:11-33:65 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::replace_many(&String::from("abc"),
        terrane_collection_support::List:: < String > ::new(vec![String::from("b")])
        .as_slice(), terrane_collection_support::List:: < String >
        ::new(vec![String::from("abc")]).as_slice()), 3 /* terrane-site: case.trn:34:11-34:52 */))
    );
    no_searches = terrane_collection_support::List::<String>::new(Vec::new());
    no_replacements = terrane_collection_support::List::<String>::new(Vec::new());
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::replace_many(&String::from("text"),
        no_searches.as_slice(), no_replacements.as_slice()), 4 /* terrane-site: case.trn:37:11-37:55 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::replace_many(&decomposed,
        terrane_collection_support::List:: < String > ::new(vec![String::from("")])
        .as_slice(), terrane_collection_support::List:: < String >
        ::new(vec![String::from("-")]).as_slice()), 5 /* terrane-site: case.trn:38:11-38:54 */))
    );
    mismatch_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("x")]);
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_string_support::replace_many(&String::from("x"),
                mismatch_searches.as_slice(), no_replacements.as_slice()),
                6 /* terrane-site: case.trn:41:13-41:60 */))
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
                    && __terrane_error_0.kind == TerraneErrorKind::ReplaceError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("replacement-list-mismatch"));
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
}
