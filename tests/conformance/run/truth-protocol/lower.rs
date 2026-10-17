// Generated deterministically by Terrane <version>.
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
        {
            let __terrane_value = open;
            self.open = __terrane_value;
        }
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
        println!("{}", String::from("enabled"));
    }
    if disabled.truth() {
        println!("{}", String::from("unexpected"));
    } else {
        println!("{}", String::from("disabled"));
    }
}
