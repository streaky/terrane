// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct Direct {
    pub __trn_726566: Option<terrane_int_support::Int>,
    pub not: Option<bool>,
    pub __trn_6d6f7665: Option<String>,
}
impl Direct {
    pub fn terrane_construct() -> Self {
        Self {
            __trn_726566: None,
            not: None,
            __trn_6d6f7665: None,
        }
    }
}
#[derive(Clone)]
pub struct Composed {
    pub __trn_726566: Option<terrane_int_support::Int>,
    pub not: Option<bool>,
    pub __trn_6d6f7665: Option<String>,
}
impl Composed {
    pub fn terrane_construct() -> Self {
        Self {
            __trn_726566: None,
            not: None,
            __trn_6d6f7665: None,
        }
    }
}
fn main() {
    let first: Direct;
    let second: Composed;
    first = Direct::terrane_construct();
    second = Composed::terrane_construct();
    println!(
        "{}", { let mut __terrane_text = String::new(); match &first.__trn_726566 {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &first.not {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &first.__trn_6d6f7665 {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &second.__trn_726566 {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &second.not {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &second.__trn_6d6f7665 {
        Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
}
