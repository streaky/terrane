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
        let mut __terrane_constructed_value = Self { open: false };
        __terrane_constructed_value.construct(open);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, open: bool) {
        self.open = open;
    }
    pub fn truth(&self) -> bool {
        return self.open;
    }
}
fn main() {
    let enabled: Gate;
    let disabled: Gate;
    enabled = Gate::terrane_construct(true);
    disabled = Gate::terrane_construct(false);
    if enabled.truth() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("enabled")));
    }
    if disabled.truth() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("unexpected")));
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("disabled")));
    }
}
