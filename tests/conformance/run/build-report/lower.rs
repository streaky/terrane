// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: build-report
static __TERRANE_F0_TITLE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| String::from(
    "Terrane",
));
fn report(name: String, passed: bool, attempts: terrane_int_support::Int) {
    if passed {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(23);
            terrane_scalar_support::ScalarDisplay::write_scalar(&name, &mut
            __terrane_text); __terrane_text.push_str(": passed in ");
            terrane_scalar_support::ScalarDisplay::write_scalar(&attempts, &mut
            __terrane_text); __terrane_text.push_str(" attempt(s)"); __terrane_text })
        );
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(8);
            terrane_scalar_support::ScalarDisplay::write_scalar(&name, &mut
            __terrane_text); __terrane_text.push_str(": failed"); __terrane_text })
        );
    }
}
fn main() {
    report(String::from("lexer"), true, terrane_int_support::Int::from(1_i128));
    report(String::from("parser"), true, terrane_int_support::Int::from(2_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); terrane_scalar_support::ScalarDisplay::write_scalar(&&*
        __TERRANE_F0_TITLE, &mut __terrane_text); __terrane_text.push_str(" length: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_string_support::length(&&*
        __TERRANE_F0_TITLE) as i128), &mut __terrane_text); __terrane_text })
    );
}
