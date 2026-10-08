// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: optional-value-narrowing
fn helper() {
    let found: String;
    found = String::from("shadow");
    println!("{}", terrane_scalar_support::scalar_text(&found));
}
fn show(value: Option<i8>) {
    if value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
fn maybe() -> Option<i8> {
    return Some(4);
}
fn missing() -> Option<i8> {
    return None;
}
fn main() {
    let mut value: Option<i8>;
    let other: Option<i8>;
    let returned: Option<i8>;
    let called: Option<i8>;
    let missingvalue: Option<i8>;
    let found: Option<terrane_string_support::TextRange>;
    value = Some(7);
    if value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
        value = None;
        println!("{}", terrane_scalar_support::scalar_text(&true));
    }
    if value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    other = Some(8);
    if other.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &other { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    show(Some(9));
    show(None);
    returned = maybe();
    if returned.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &returned { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    called = maybe();
    if called.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &called { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missingvalue = None;
    if missingvalue.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &missingvalue {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missing();
    println!("{}", terrane_scalar_support::scalar_text(&true));
    helper();
    found = terrane_string_support::find(&String::from("banana"), &String::from("ana"));
    if found.is_some() {
        if {
            let _ = &match &found {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            };
            true
        } {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") } .text()
                .to_owned())
            );
        }
    }
}
