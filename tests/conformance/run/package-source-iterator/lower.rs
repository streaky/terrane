// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: app/main.trn
// Namespace: app
fn main() {
    let values: Counter = Counter::terrane_construct();
    let mut __terrane_iterator_0 = values.iterator();
    loop {
        let value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
}
// Source: app/support/iterator.trn
// Namespace: app/support
#[derive(Clone)]
pub struct Counter {
    pub current: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            current: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn iterator(&self) -> Counter {
        return self.clone();
    }
    pub fn next(
        &mut self,
    ) -> terrane_collection_support::IterationStep<terrane_int_support::Int> {
        if self.current.clone() >= terrane_int_support::Int::from(2_i128) {
            return terrane_collection_support::IterationStep::End;
        }
        let value: terrane_int_support::Int = self.current.clone();
        self.current = self.current.clone() + terrane_int_support::Int::from(1_i128);
        return terrane_collection_support::IterationStep::<
            terrane_int_support::Int,
        >::Item(value.clone());
    }
}
