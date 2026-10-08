// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: text-families
fn main() {
    let text_terrane_f0_s41: String;
    let decomposed_terrane_f0_s262: String;
    let rtl_terrane_f0_s432: String;
    text_terrane_f0_s41 = String::from("  Straße  ");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::trim(&text_terrane_f0_s41))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::trim_start(&text_terrane_f0_s41,
        None)),
        terrane_scalar_support::scalar_text(&terrane_string_support::trim_end(&text_terrane_f0_s41,
        None))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::case_fold(&text_terrane_f0_s41))
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
    decomposed_terrane_f0_s262 = String::from("e\u{301}");
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::normalise(&decomposed_terrane_f0_s262,
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
    rtl_terrane_f0_s432 = String::from("שלום");
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&rtl_terrane_f0_s432
        .starts_with(&String::from("ש"))),
        terrane_scalar_support::scalar_text(&rtl_terrane_f0_s432
        .ends_with(&String::from("ם")))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::find_all(&decomposed_terrane_f0_s262,
        &String::from("")).len() as i128))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_string_support::split(&decomposed_terrane_f0_s262,
        &String::from("")).len() as i128))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_string_support::replace(&decomposed_terrane_f0_s262,
        &String::from(""), &String::from("X")))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(decomposed_terrane_f0_s262.len()
        as i128)), terrane_scalar_support::scalar_text(&(decomposed_terrane_f0_s262
        .chars().count() as i128)),
        terrane_scalar_support::scalar_text(&(terrane_string_support::length(&decomposed_terrane_f0_s262)
        as i128))
    );
}
