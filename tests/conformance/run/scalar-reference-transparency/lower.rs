// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: scalar-reference-transparency
fn main() {
    let text: std::sync::Arc<parking_lot::Mutex<String>> = std::sync::Arc::new(parking_lot::Mutex::new(String::from("abc")));
    let seen: std::sync::Weak<parking_lot::Mutex<String>> = std::sync::Arc::downgrade(&text);
    println!("{}", terrane_scalar_support::scalar_text(&(terrane_string_support::length(&({ let __terrane_owner = seen.upgrade().expect("reference expired"); let __terrane_value = __terrane_owner.lock().clone(); __terrane_value })) as i128)));
    println!("{}", terrane_scalar_support::scalar_text(&(({ let __terrane_owner = seen.upgrade().expect("reference expired"); let __terrane_value = __terrane_owner.lock().clone(); __terrane_value }))));
    let number: std::sync::Arc<parking_lot::Mutex<i8>> = std::sync::Arc::new(parking_lot::Mutex::new(7));
    let observed: std::sync::Weak<parking_lot::Mutex<i8>> = std::sync::Arc::downgrade(&number);
    println!("{}", terrane_scalar_support::scalar_text(&(({ let __terrane_owner = observed.upgrade().expect("reference expired"); let __terrane_value = __terrane_owner.lock().clone(); __terrane_value }))));
    let owner: std::sync::Arc<parking_lot::Mutex<i8>> = (number).clone();
    println!("{}", terrane_scalar_support::scalar_text(&(({ let __terrane_value = owner.lock().clone(); __terrane_value }))));
}
