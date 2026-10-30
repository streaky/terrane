// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: string-search-delimiters
fn active_content(text: String) -> bool {
    return {
        let __terrane_string_argument_0 = &String::from("javascript:");
        text.contains(std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0))
    }
        || {
            let __terrane_string_argument_0 = &String::from("expression(");
            text.contains(
                std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
            )
        }
        || {
            let __terrane_string_argument_0 = &String::from("-moz-binding");
            text.contains(
                std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
            )
        };
}
fn closing_marker(text: String) -> bool {
    return {
        let __terrane_string_argument_0 = &String::from(")");
        text.contains(std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0))
    }
        || {
            let __terrane_string_argument_0 = &String::from("quote\"(");
            text.contains(
                std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
            )
        }
        || {
            let __terrane_string_argument_0 = &String::from("\\");
            text.contains(
                std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
            )
        };
}
fn main() {
    let native_result: bool;
    let unit_result: ();
    let grouped_result: bool;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(8); __terrane_text.push_str("active: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&active_content(String::from("expression(alert)")),
        &mut __terrane_text); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6); __terrane_text.push_str("safe: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&active_content(String::from("ordinary text")),
        &mut __terrane_text); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("closing: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&closing_marker(String::from("call)")),
        &mut __terrane_text); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(8); __terrane_text.push_str("quoted: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&closing_marker(String::from("quote\"(")),
        &mut __terrane_text); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(11); __terrane_text.push_str("backslash: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&closing_marker(String::from("path\\part")),
        &mut __terrane_text); __terrane_text })
    );
    native_result = r#"("#.contains('(') && b')' == 41;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(8); __terrane_text.push_str("native: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&native_result, &mut
        __terrane_text); __terrane_text })
    );
    unit_result = ();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6); __terrane_text.push_str("unit: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&unit_result, &mut
        __terrane_text); __terrane_text })
    );
    grouped_result = (1, 2) == (1, 2);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("grouped: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&grouped_result, &mut
        __terrane_text); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(12); __terrane_text.push_str("precedence: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_int_support::Int::from(3_i128)
        * terrane_int_support::Int::from(3_i128)), &mut __terrane_text); __terrane_text
        })
    );
}
