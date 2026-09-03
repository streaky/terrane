// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
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
    let direct: Widget = Widget::terrane_construct();
    let factory: Widget = Widget::terrane_static_create();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&direct.read()),
        terrane_scalar_support::scalar_text(&factory.read())
    );
}
