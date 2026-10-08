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
    let value_terrane_f0_s366: Gate;
    let source_terrane_f0_s421: Values;
    let mut item_terrane_f0_s453: terrane_int_support::Int;
    value_terrane_f0_s366 = Gate::terrane_construct();
    if value_terrane_f0_s366.truth() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("truth")));
    }
    source_terrane_f0_s421 = Values::terrane_construct();
    let __terrane_iterable_0 = source_terrane_f0_s421;
    let mut __terrane_iterator_0 = __terrane_iterable_0.iterator();
    loop {
        item_terrane_f0_s453 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&item_terrane_f0_s453));
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("end")));
}
