// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: case.trn
// Namespace: structural-protocol-composition
#[derive(Clone)]
pub struct Gate {}
impl Gate {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn truth(&self) -> bool {
        return true;
    }
}
#[derive(Clone)]
pub struct Cursor {}
impl Cursor {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn next(
        &self,
    ) -> terrane_collection_support::IterationStep<terrane_int_support::Int> {
        return terrane_collection_support::IterationStep::End;
    }
}
#[derive(Clone)]
pub struct Values {}
impl Values {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn iterator(&self) -> Cursor {
        return Cursor::terrane_construct();
    }
}
fn main() {
    let value: Gate = Gate::terrane_construct();
    if value.truth() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("truth")));
    }
    let source: Values = Values::terrane_construct();
    let mut __terrane_iterator_0 = source.iterator();
    loop {
        let item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&item));
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("end")));
}
