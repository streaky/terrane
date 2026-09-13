// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: scalar-reference-transparency
fn main() {
    let text: String = String::from("abc");
    let seen: &String = &text;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(terrane_string_support::length(&seen
        .clone()) as i128))
    );
    println!("{}", terrane_scalar_support::scalar_text(&seen.clone()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&format!("{}{}",
        terrane_scalar_support::scalar_text(&seen.clone()),
        terrane_scalar_support::scalar_text(&String::from("!"))))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&vec![terrane_scalar_support::scalar_text(&String::from("x")),
        terrane_scalar_support::scalar_text(&String::from("y"))] .join(&seen.clone()))
    );
    let encoded: Vec<u8> = terrane_string_support::encode(
        &text,
        terrane_string_support::Encoding::Utf8,
    );
    let decoded: &Vec<u8> = &encoded;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&decoded
        .clone(), terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:13:13-13:33 */))
    );
    let number: std::sync::Arc<std::sync::Mutex<i8>> = std::sync::Arc::new(
        std::sync::Mutex::new(7),
    );
    let observed: std::sync::Weak<std::sync::Mutex<i8>> = std::sync::Arc::downgrade(
        &number.clone(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = observed
        .upgrade().expect("reference expired"); let __terrane_value = __terrane_owner
        .lock().expect("reference lock poisoned").clone(); __terrane_value })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_owner = observed.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value } as i128))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition({
        let __terrane_owner = observed.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value }, 2), 1 /* terrane-site: case.trn:18:13-18:28 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition({
        let __terrane_owner = observed.upgrade().expect("reference expired"); let
        __terrane_value = __terrane_owner.lock().expect("reference lock poisoned")
        .clone(); __terrane_value }, 3), 2 /* terrane-site: case.trn:19:12-19:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&- { let __terrane_owner = observed
        .upgrade().expect("reference expired"); let __terrane_value = __terrane_owner
        .lock().expect("reference lock poisoned").clone(); __terrane_value })
    );
    let owner: std::sync::Arc<std::sync::Mutex<i8>> = number.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value = owner.lock()
        .expect("shared reference lock poisoned").clone(); __terrane_value })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_multiplication({
        let __terrane_value = owner.lock().expect("shared reference lock poisoned")
        .clone(); __terrane_value }, 2), 3 /* terrane-site: case.trn:23:12-23:21 */))
    );
}
