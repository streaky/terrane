// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: grouped-initializer-free-types
#[derive(Clone)]
pub struct Direct {
    pub slot: Option<terrane_int_support::Int>,
}
impl Direct {
    pub fn terrane_construct() -> Self {
        Self { slot: None }
    }
}
#[derive(Clone)]
pub struct Composed {
    pub slot: Option<terrane_int_support::Int>,
}
impl Composed {
    pub fn terrane_construct() -> Self {
        Self { slot: None }
    }
}
fn main() {
    let mut first: Direct;
    let mut second: Composed;
    let mut local: Option<terrane_int_support::Int>;
    first = Direct::terrane_construct();
    second = Composed::terrane_construct();
    println!(
        "{}", { let mut __terrane_text = String::new(); match &first.slot {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &second.slot {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    local = Some(terrane_int_support::Int::from(3_i128));
    println!(
        "{}", { let mut __terrane_text = String::new(); match &local {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    local = None;
    println!(
        "{}", { let mut __terrane_text = String::new(); match &local {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    {
        let __terrane_value = Some(terrane_int_support::Int::from(5_i128));
        first.slot = __terrane_value;
    }
    {
        let __terrane_value = Some(terrane_int_support::Int::from(7_i128));
        second.slot = __terrane_value;
    }
    println!(
        "{}", { let mut __terrane_text = String::new(); match &first.slot {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &second.slot {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
}
