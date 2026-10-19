// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: optional-value-narrowing
fn helper() {
    let found: String;
    found = String::from("shadow");
    println!("{}", terrane_scalar_support::scalar_text(&found));
}
fn show(value: Option<i8>) {
    if (&value).is_some() {
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
fn enabled() -> bool {
    return true;
}
fn shared_snapshot(value: Option<String>) -> String {
    let value = std::sync::Arc::new(TerraneSharedCell::new(value));
    let __trn_5f6f776e6572: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    __trn_5f6f776e6572 = value.clone();
    let _ = &__trn_5f6f776e6572;
    if {
        let __terrane_present = (&value)
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        return {
            let __terrane_value = match &*(&value)
                .lock()
                .expect("reference lock poisoned")
            {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone();
            __terrane_value
        };
    }
    return String::from("missing");
}
fn main() {
    let mut value: Option<i8>;
    let other: Option<i8>;
    let returned: Option<i8>;
    let called: Option<i8>;
    let missingvalue: Option<i8>;
    let found: Option<terrane_string_support::TextRange>;
    let sharedvalue: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    let __trn_5f6f62736572766572: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    let snapshot: String;
    let sharednumber: std::sync::Arc<TerraneSharedCell<Option<i8>>>;
    let __trn_5f6e756d6265726f776e6572: std::sync::Arc<TerraneSharedCell<Option<i8>>>;
    let mut late: Option<std::sync::Arc<TerraneSharedCell<Option<String>>>> = None;
    let __trn_5f6c6174656f776e6572: std::sync::Arc<TerraneSharedCell<Option<String>>>;
    value = Some(7);
    if (&value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
        value = None;
        println!("{}", terrane_scalar_support::scalar_text(&true));
    }
    if (&value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    other = Some(8);
    if (&other).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &other { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    show(Some(9));
    show(None);
    returned = maybe();
    if (&returned).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &returned { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    called = maybe();
    if (&called).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &called { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missingvalue = None;
    if (&missingvalue).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &missingvalue {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missing();
    println!("{}", terrane_scalar_support::scalar_text(&true));
    helper();
    found = terrane_string_support::find(&String::from("banana"), &String::from("ana"));
    if (&found).is_some() {
        if (&found).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") } .text()
                .to_owned())
            );
        }
    }
    sharedvalue = std::sync::Arc::new(
        TerraneSharedCell::new(Some(String::from("retained"))),
    );
    __trn_5f6f62736572766572 = sharedvalue.clone();
    let _ = &__trn_5f6f62736572766572;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_some();
        __terrane_present }), terrane_scalar_support::scalar_text(&{ let
        __terrane_present = (&sharedvalue).lock().expect("reference lock poisoned")
        .is_none(); __terrane_present })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_some();
        __terrane_present }), { let mut __terrane_text = String::new(); { let
        __terrane_result = match &* sharedvalue.lock().expect("reference lock poisoned")
        { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), }; __terrane_result
        } __terrane_text }
    );
    if {
        let __terrane_present = (&sharedvalue)
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
            (&sharedvalue).lock().expect("reference lock poisoned").is_none();
            __terrane_present })
        );
        println!(
            "{}{}", { let __terrane_result = terrane_scalar_support::scalar_text(&match
            &* (&sharedvalue).lock().expect("reference lock poisoned") { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") });
            __terrane_result }, { let __terrane_result =
            terrane_scalar_support::scalar_text(&match &* (&sharedvalue).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }); __terrane_result }
        );
        snapshot = {
            let __terrane_value = match &*(&sharedvalue)
                .lock()
                .expect("reference lock poisoned")
            {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .clone();
            __terrane_value
        };
        *sharedvalue.lock().expect("reference lock poisoned") = Some(
            String::from("changed"),
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&snapshot), { let mut
            __terrane_text = String::new(); { let __terrane_result = match &* sharedvalue
            .lock().expect("reference lock poisoned") { Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), };
            __terrane_result } __terrane_text }
        );
    }
    *sharedvalue.lock().expect("reference lock poisoned") = None;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_none();
        __terrane_present })
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_present =
        (&sharedvalue).lock().expect("reference lock poisoned").is_some();
        __terrane_present }), terrane_scalar_support::scalar_text(&{ let
        __terrane_present = (&sharedvalue).lock().expect("reference lock poisoned")
        .is_none(); __terrane_present })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&shared_snapshot(Some(String::from("parameter"))))
    );
    sharednumber = std::sync::Arc::new(TerraneSharedCell::new(Some(7)));
    __trn_5f6e756d6265726f776e6572 = sharednumber.clone();
    let _ = &__trn_5f6e756d6265726f776e6572;
    if {
        let __terrane_present = (&sharednumber)
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        println!(
            "{}{}", { let __terrane_result = terrane_scalar_support::scalar_text(&* match
            &* (&sharednumber).lock().expect("reference lock poisoned") { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") });
            __terrane_result }, { let __terrane_result =
            terrane_scalar_support::scalar_text(&* match &* (&sharednumber).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }); __terrane_result }
        );
    }
    if enabled() {
        let _ = late
            .insert(
                std::sync::Arc::new(TerraneSharedCell::new(Some(String::from("late")))),
            );
        __trn_5f6c6174656f776e6572 = late
            .as_ref()
            .expect("flow-proven available binding")
            .clone();
        let _ = &__trn_5f6c6174656f776e6572;
    }
    if {
        let __terrane_present = late
            .as_ref()
            .unwrap_or_else(|| __terrane_uninitialized_binding(
                "late",
                "case.trn",
                69,
                8,
            ))
            .lock()
            .expect("reference lock poisoned")
            .is_some();
        __terrane_present
    } {
        println!(
            "{}{}", { let __terrane_result = terrane_scalar_support::scalar_text(&match
            &* late.as_ref().unwrap_or_else(| | __terrane_uninitialized_binding("late",
            "case.trn", 70, 16)).lock().expect("reference lock poisoned") { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") });
            __terrane_result }, { let __terrane_result =
            terrane_scalar_support::scalar_text(&match &* late.as_ref().unwrap_or_else(|
            | __terrane_uninitialized_binding("late", "case.trn", 70, 22)).lock()
            .expect("reference lock poisoned") { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }); __terrane_result }
        );
    }
}
