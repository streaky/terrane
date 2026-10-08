// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: app/main.trn
// Namespace: app
fn main() {
    let values_terrane_f0_s64: Counter;
    let mut value_terrane_f0_s97: terrane_int_support::Int;
    values_terrane_f0_s64 = Counter::terrane_construct();
    let __terrane_iterable_0 = values_terrane_f0_s64;
    let mut __terrane_iterator_0 = __terrane_iterable_0.iterator();
    loop {
        value_terrane_f0_s97 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s97));
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
        let value_terrane_f1_s263: terrane_int_support::Int;
        if self.current.clone() >= terrane_int_support::Int::from(2_i128) {
            return terrane_collection_support::IterationStep::End;
        }
        value_terrane_f1_s263 = self.current.clone();
        self.current = self.current.clone() + terrane_int_support::Int::from(1_i128);
        return terrane_collection_support::IterationStep::<
            terrane_int_support::Int,
        >::Item(value_terrane_f1_s263.clone());
    }
}
