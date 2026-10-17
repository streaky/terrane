// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: reentry-loop-reassign
#[derive(Clone)]
pub struct Base {
    pub count: terrane_int_support::Int,
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(5_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<TerraneSharedCell<Base>>,
    ) -> terrane_int_support::Int {
        return {
            let __terrane_snapshot = other
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        };
    }
}
fn main() {
    let a: std::sync::Arc<TerraneSharedCell<Base>>;
    let b: std::sync::Arc<TerraneSharedCell<Base>>;
    let ha: std::sync::Arc<TerraneSharedCell<Base>>;
    let hb: std::sync::Arc<TerraneSharedCell<Base>>;
    let mut copy: std::sync::Arc<TerraneSharedCell<Base>>;
    let mut item: terrane_int_support::Int;
    a = std::sync::Arc::new(TerraneSharedCell::new(Base::terrane_construct()));
    b = std::sync::Arc::new(TerraneSharedCell::new(Base::terrane_construct()));
    ha = a.clone();
    hb = b.clone();
    copy = hb.clone();
    let __terrane_iterable_0 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        copy = ha.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = ha.clone();
        let __terrane_shared_argument_0 = copy.clone(); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
}
