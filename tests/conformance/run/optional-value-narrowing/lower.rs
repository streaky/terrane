// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: optional-value-narrowing
fn helper() {
    let found_terrane_f0_s56: String;
    found_terrane_f0_s56 = String::from("shadow");
    println!("{}", terrane_scalar_support::scalar_text(&found_terrane_f0_s56));
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
    let mut value_terrane_f0_s272: Option<i8>;
    let other_terrane_f0_s423: Option<i8>;
    let returned_terrane_f0_s518: Option<i8>;
    let called_terrane_f0_s588: Option<i8>;
    let missingvalue_terrane_f0_s662: Option<i8>;
    let found_terrane_f0_s793: Option<terrane_string_support::TextRange>;
    value_terrane_f0_s272 = Some(7);
    if value_terrane_f0_s272.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value_terrane_f0_s272 {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
        value_terrane_f0_s272 = None;
        println!("{}", terrane_scalar_support::scalar_text(&true));
    }
    if value_terrane_f0_s272.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value_terrane_f0_s272 {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    other_terrane_f0_s423 = Some(8);
    if other_terrane_f0_s423.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &other_terrane_f0_s423 {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    show(Some(9));
    show(None);
    returned_terrane_f0_s518 = maybe();
    if returned_terrane_f0_s518.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &returned_terrane_f0_s518
            { Some(value) => value, _ => unreachable!("flow-proven storage refinement")
            })
        );
    }
    called_terrane_f0_s588 = maybe();
    if called_terrane_f0_s588.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &called_terrane_f0_s588 {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missingvalue_terrane_f0_s662 = None;
    if missingvalue_terrane_f0_s662.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match
            &missingvalue_terrane_f0_s662 { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    missing();
    println!("{}", terrane_scalar_support::scalar_text(&true));
    helper();
    found_terrane_f0_s793 = terrane_string_support::find(
        &String::from("banana"),
        &String::from("ana"),
    );
    if found_terrane_f0_s793.is_some() {
        if {
            let _ = &match &found_terrane_f0_s793 {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            };
            true
        } {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &found_terrane_f0_s793 {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                } .text().to_owned())
            );
        }
    }
}
