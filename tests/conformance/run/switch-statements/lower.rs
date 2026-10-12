// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: switch-statements
fn choose() -> terrane_int_support::Int {
    {
        let value = __TERRANE_GLOBAL_CALLS
            .lock()
            .expect("program-global lock poisoned")
            .clone()
            .unwrap_or_else(|| __terrane_uninitialized_binding(
                "calls",
                "case.trn",
                6,
                20,
            ))
            .clone() + terrane_int_support::Int::from(1_i128);
        *__TERRANE_GLOBAL_CALLS.lock().expect("program-global lock poisoned") = Some(
            value,
        );
    }
    println!("{}", String::from("selector"));
    return terrane_int_support::Int::from(2_i128);
}
fn candidate(value: terrane_int_support::Int) -> terrane_int_support::Int {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("candidate:");
        terrane_scalar_support::ScalarDisplay::write_scalar(&value, &mut __terrane_text);
        __terrane_text })
    );
    return value.clone();
}
fn condition(value: bool, name: String) -> bool {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("condition:");
        terrane_scalar_support::ScalarDisplay::write_scalar(&name, &mut __terrane_text);
        __terrane_text })
    );
    return value;
}
fn describe(value: terrane_int_support::Int) -> String {
    if value.clone() == terrane_int_support::Int::from(0_i128) {
        return String::from("empty");
    } else if value.clone() < terrane_int_support::Int::from(10_i128) {
        return String::from("small");
    } else {
        return String::from("large");
    }
}
fn narrow(value: Option<String>) -> String {
    if value.is_some() {
        return match value {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    } else {
        return String::from("absent");
    }
}
fn main() {
    let mut joined: Option<String> = None;
    let __terrane_switch_value_685 = &choose();
    if __terrane_switch_value_685 == &candidate(terrane_int_support::Int::from(1_i128)) {
        println!("{}", String::from("wrong"));
    } else if __terrane_switch_value_685
        == &candidate(terrane_int_support::Int::from(2_i128))
        || __terrane_switch_value_685
            == &candidate(terrane_int_support::Int::from(3_i128))
    {
        println!("{}", String::from("selected"));
    } else {
        println!("{}", String::from("wrong-default"));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("selectors:");
        terrane_scalar_support::ScalarDisplay::write_scalar(&__TERRANE_GLOBAL_CALLS
        .lock().expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_binding("calls", "case.trn", 42, 27)), &mut
        __terrane_text); __terrane_text })
    );
    if condition(false, String::from("first")) {
        println!("{}", String::from("wrong-first"));
    } else if condition(true, String::from("second")) {
        println!("{}", String::from("predicate"));
    } else if condition(true, String::from("skipped")) {
        println!("{}", String::from("wrong-late"));
    } else {
        println!("{}", String::from("wrong-default"));
    }
    let __terrane_switch_value_1204 = &4;
    if __terrane_switch_value_1204 == &1 {
        println!("{}", String::from("wrong"));
    }
    let __terrane_switch_value_1259 = &String::from("b");
    if __terrane_switch_value_1259.as_str() == "a" {
        println!("{}", String::from("wrong"));
    } else if __terrane_switch_value_1259.as_str() == "b"
        || __terrane_switch_value_1259.as_str() == "c"
    {
        let _ = joined.insert(String::from("joined"));
    } else {
        let _ = joined.insert(String::from("default"));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&joined.as_ref().unwrap_or_else(| |
        __terrane_uninitialized_binding("joined", "case.trn", 62, 12)))
    );
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&describe(terrane_int_support::Int::from(3_i128))),
        String::from("|"),
        terrane_scalar_support::scalar_text(&describe(terrane_int_support::Int::from(0_i128))),
        String::from("|"),
        terrane_scalar_support::scalar_text(&describe(terrane_int_support::Int::from(20_i128)))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&narrow(None)), String::from("|"),
        terrane_scalar_support::scalar_text(&narrow(Some(String::from("present"))))
    );
}
