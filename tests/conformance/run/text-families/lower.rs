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
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
        &String::from("straße");
        terrane_string_support::upper_first(&__terrane_receiver_value_0) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_1 =
        &String::from("hello world");
        terrane_string_support::upper_words(&__terrane_receiver_value_1) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_2 =
        &String::from("Hello");
        terrane_string_support::lower_first(&__terrane_receiver_value_2) })
    );
    decomposed = String::from("e\u{301}");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::normalise(&decomposed,
        "nfc"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_3 =
        &String::from("banana"); let __terrane_string_argument_0 = "ana"; let
        __terrane_string_argument_1 = "X";
        terrane_string_support::replace(&__terrane_receiver_value_3,
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0),
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_1)) })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_4 =
        &String::from("banana"); let __terrane_string_argument_0 = "ana";
        terrane_int_support::Int::from(terrane_string_support::find_all(&__terrane_receiver_value_4,
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0)).len() as
        i128) }), terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_5 =
        &String::from("banana"); let __terrane_string_argument_0 = "";
        terrane_int_support::Int::from(terrane_string_support::find_all(&__terrane_receiver_value_5,
        &std::convert::AsRef:: < str > ::as_ref(&__terrane_string_argument_0)).len() as
        i128) })
    );
    rtl = String::from("שלום");
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        "ש"; rtl.starts_with(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 = "ם";
        rtl.ends_with(std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_string_argument_0 =
        ""; terrane_string_support::find_all(&decomposed, &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) } .len() as i128))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_string_argument_0 =
        ""; terrane_string_support::split(&decomposed, &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0)) } .len() as i128))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        ""; let __terrane_string_argument_1 = "X";
        terrane_string_support::replace(&decomposed, &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_0), &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_1)) })
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
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_6 =
        &String::from("&<"); let __terrane_string_argument_0 = &searches; let
        __terrane_string_argument_1 = &html_replacements;
        __terrane_raised(terrane_string_support::replace_many(&__terrane_receiver_value_6,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        0 /* terrane-site: case.trn:25:11-25:52 */) })
    );
    reversed_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("<"), String::from("&")]);
    reversed_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("&lt;"), String::from("&amp;")]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_7 =
        &String::from("&<"); let __terrane_string_argument_0 = &reversed_searches; let
        __terrane_string_argument_1 = &reversed_replacements;
        __terrane_raised(terrane_string_support::replace_many(&__terrane_receiver_value_7,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        1 /* terrane-site: case.trn:28:11-28:65 */) })
    );
    broadcast_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("one"), String::from("two")]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_8 =
        &String::from("one and two"); let __terrane_string_argument_0 =
        &broadcast_searches; let __terrane_string_argument_1 = "x";
        terrane_string_support::replace_broadcast(&__terrane_receiver_value_8,
        __terrane_string_argument_0.as_slice(), &std::convert::AsRef:: < str >
        ::as_ref(&__terrane_string_argument_1)) })
    );
    priority_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("a"), String::from("ab")]);
    priority_replacements = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("first"), String::from("second")]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_9 =
        &String::from("ab"); let __terrane_string_argument_0 = &priority_searches; let
        __terrane_string_argument_1 = &priority_replacements;
        __terrane_raised(terrane_string_support::replace_many(&__terrane_receiver_value_9,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        2 /* terrane-site: case.trn:33:11-33:65 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_10 =
        &String::from("abc"); let __terrane_string_argument_0 =
        &terrane_collection_support::List:: < String > ::new(vec![String::from("b")]);
        let __terrane_string_argument_1 = &terrane_collection_support::List:: < String >
        ::new(vec![String::from("abc")]);
        __terrane_raised(terrane_string_support::replace_many(&__terrane_receiver_value_10,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        3 /* terrane-site: case.trn:34:11-34:52 */) })
    );
    no_searches = terrane_collection_support::List::<String>::new(Vec::new());
    no_replacements = terrane_collection_support::List::<String>::new(Vec::new());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_11 =
        &String::from("text"); let __terrane_string_argument_0 = &no_searches; let
        __terrane_string_argument_1 = &no_replacements;
        __terrane_raised(terrane_string_support::replace_many(&__terrane_receiver_value_11,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        4 /* terrane-site: case.trn:37:11-37:55 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        &terrane_collection_support::List:: < String > ::new(vec![String::from("")]); let
        __terrane_string_argument_1 = &terrane_collection_support::List:: < String >
        ::new(vec![String::from("-")]);
        __terrane_raised(terrane_string_support::replace_many(&decomposed,
        __terrane_string_argument_0.as_slice(), __terrane_string_argument_1.as_slice()),
        5 /* terrane-site: case.trn:38:11-38:54 */) })
    );
    mismatch_searches = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("x")]);
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let
                __terrane_receiver_value_12 = &String::from("x"); let
                __terrane_string_argument_0 = &mismatch_searches; let
                __terrane_string_argument_1 = &no_replacements;
                __terrane_raised_completion!(terrane_string_support::replace_many(&__terrane_receiver_value_12,
                __terrane_string_argument_0.as_slice(), __terrane_string_argument_1
                .as_slice()), 6 /* terrane-site: case.trn:41:13-41:60 */) })
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
