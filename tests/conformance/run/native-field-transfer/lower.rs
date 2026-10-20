// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub struct Holder {
    pub value: Option<Builder>,
}
impl Holder {
    pub fn terrane_construct(value: Option<Builder>) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: Option<Builder>) {
        {
            let __terrane_value = value;
            self.value = __terrane_value;
        }
    }
    pub fn check(&self) -> bool {
        return (&self.value).is_none();
    }
    pub fn take(mut self) {
        let first: Option<Builder>;
        let second: Option<Builder>;
        first = std::mem::take(&mut self.value);
        second = std::mem::take(&mut self.value);
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&(&first).is_some()),
            terrane_scalar_support::scalar_text(&(&second).is_none())
        );
    }
}
fn main() {
    let mut h: Holder;
    let value: Option<Builder>;
    let other: Holder;
    h = Holder::terrane_construct(
        Some(
            __terrane_raised(
                terrane_static_trn_4275696c646572_new(),
                0 /* terrane-site: src/main.trn:14:27-14:40 */,
            ),
        ),
    );
    value = std::mem::take(&mut h.value);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(&value).is_some()),
        terrane_scalar_support::scalar_text(&h.check())
    );
    other = Holder::terrane_construct(
        Some(
            __terrane_raised(
                terrane_static_trn_4275696c646572_new(),
                1 /* terrane-site: src/main.trn:17:31-17:44 */,
            ),
        ),
    );
    other.take();
}
