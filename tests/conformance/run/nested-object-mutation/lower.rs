// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct Leaf {
    pub value: terrane_int_support::Int,
}
impl Leaf {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        self.value = initial.clone();
    }
}
#[derive(Clone)]
pub struct Holder {
    pub child: Leaf,
}
impl Holder {
    pub fn terrane_construct(child: Leaf) -> Self {
        let mut value = Self {
            child: Leaf::terrane_construct(terrane_int_support::Int::from(0_i128)),
        };
        value.construct(child);
        value
    }
    pub fn construct(&mut self, child: Leaf) {
        self.child = child;
    }
    pub fn replace(
        &mut self,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.child.value = value.clone();
        return self.child.value.clone();
    }
}
fn main() {
    let value: Leaf = Leaf::terrane_construct(terrane_int_support::Int::from(1_i128));
    let mut container: Holder = Holder::terrane_construct(value);
    println!(
        "{}", terrane_scalar_support::scalar_text(&container
        .replace(terrane_int_support::Int::from(42_i128)))
    );
}
