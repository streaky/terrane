// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: truth-protocol
#[derive(Clone)]
pub struct Gate {
    pub open: bool,
}
impl Gate {
    pub fn terrane_construct(open: bool) -> Self {
        let mut value = Self { open: false };
        value.construct(open);
        value
    }
    pub fn construct(&mut self, open: bool) {
        self.open = open;
    }
    pub fn truth(&self) -> bool {
        return self.open;
    }
}
fn main() {
    let enabled: Gate = Gate::terrane_construct(true);
    let disabled: Gate = Gate::terrane_construct(false);
    if enabled.truth() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("enabled")));
    }
    if disabled.truth() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("unexpected")));
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("disabled")));
    }
}
