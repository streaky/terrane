// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: explicit-class-operations
#[derive(Clone)]
pub struct Widget {
    pub value: terrane_int_support::Int,
}
impl Widget {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn terrane_static_create() -> Widget {
        return Widget::terrane_construct();
    }
}
fn main() {
    let direct_terrane_f0_s204: Widget;
    let factory_terrane_f0_s234: Widget;
    direct_terrane_f0_s204 = Widget::terrane_construct();
    factory_terrane_f0_s234 = Widget::terrane_static_create();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&direct_terrane_f0_s204.read()),
        terrane_scalar_support::scalar_text(&factory_terrane_f0_s234.read())
    );
}
