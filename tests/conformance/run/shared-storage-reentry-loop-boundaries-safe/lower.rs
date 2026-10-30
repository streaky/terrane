// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-storage-reentry-loop-boundaries-safe
#[derive(Clone)]
pub struct TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
            >,
        >,
    ) -> terrane_int_support::Int {
        return self.count.clone()
            + {
                let __terrane_snapshot = other
                    .lock()
                    .expect("shared reference lock poisoned")
                    .count
                    .clone();
                __terrane_snapshot
            };
    }
}
fn outer(
    first: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
        >,
    >,
    second: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
        >,
    >,
) -> terrane_int_support::Int {
    return {
        let __terrane_owner_1 = first.clone();
        let __terrane_shared_argument_0 = second.clone();
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner_1
            .lock()
            .expect("shared reference lock poisoned")
            .peek(__terrane_shared_argument_1);
        __terrane_call_result
    };
}
fn main() {
    let a: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
        >,
    >;
    let b: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
        >,
    >;
    let ha: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
        >,
    >;
    let hb: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
        >,
    >;
    let mut copy: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78,
        >,
    >;
    let mut total: terrane_int_support::Int;
    let mut i: terrane_int_support::Int;
    let empty: terrane_collection_support::List<terrane_int_support::Int>;
    let mut outer_item: terrane_int_support::Int;
    let mut inner_item: terrane_int_support::Int;
    let mut item: terrane_int_support::Int;
    a = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78::terrane_construct(),
        ),
    );
    b = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryLoopBoundariesSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D626F756E6461726965732D7361666500626F78::terrane_construct(),
        ),
    );
    {
        let __terrane_owner_2 = b.clone();
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        __terrane_owner_2.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    ha = a.clone();
    hb = b.clone();
    copy = hb.clone();
    total = terrane_int_support::Int::from(0_i128);
    i = terrane_int_support::Int::from(0_i128);
    '__terrane_break_0: while i < terrane_int_support::Int::from(0_i128) {
        '__terrane_continue_0: {
            copy = ha.clone();
        }
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&i));
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    i = terrane_int_support::Int::from(0_i128);
    '__terrane_break_1: while i < terrane_int_support::Int::from(2_i128) {
        '__terrane_continue_1: {
            copy = ha.clone();
            total = total.clone()
                + {
                    let __terrane_snapshot = copy
                        .lock()
                        .expect("shared reference lock poisoned")
                        .count
                        .clone();
                    __terrane_snapshot
                };
            copy = hb.clone();
        }
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&i));
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    empty = terrane_collection_support::List::<terrane_int_support::Int>::new(vec![]);
    let __terrane_iterable_2 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        outer_item = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        total = total.clone() + outer_item.clone();
        let __terrane_iterable_3 = empty.clone();
        let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_3,
        );
        loop {
            inner_item = match __terrane_iterator_3.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            copy = ha.clone();
            total = total.clone()
                + {
                    let __terrane_snapshot = copy
                        .lock()
                        .expect("shared reference lock poisoned")
                        .count
                        .clone();
                    __terrane_snapshot
                } + inner_item.clone();
        }
        copy = hb.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    let __terrane_iterable_4 = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_4,
    );
    loop {
        outer_item = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &outer_item;
        total = total.clone() + outer_item.clone();
        let __terrane_iterable_5 = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(vec![terrane_int_support::Int::from(1_i128)]);
        let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_5,
        );
        loop {
            inner_item = match __terrane_iterator_5.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &inner_item;
            copy = ha.clone();
            total = total.clone()
                + {
                    let __terrane_snapshot = copy
                        .lock()
                        .expect("shared reference lock poisoned")
                        .count
                        .clone();
                    __terrane_snapshot
                } + inner_item.clone();
        }
        copy = hb.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    let __terrane_iterable_6 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_6,
    );
    loop {
        item = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if item == terrane_int_support::Int::from(1_i128) {
            break;
        }
        copy = ha.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
