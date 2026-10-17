// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: shared-storage-reentry-loop-safe
#[derive(Clone)]
pub struct TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            std::sync::Mutex<
                TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
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
fn go(
    first: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >,
    second: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >,
) -> terrane_int_support::Int {
    return {
        let __terrane_owner = first.clone();
        let __terrane_shared_argument_0 = second.clone();
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .peek(__terrane_shared_argument_1);
        __terrane_call_result
    };
}
fn outer(
    first: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >,
    second: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >,
) -> terrane_int_support::Int {
    return go(first.clone(), second.clone());
}
fn main() {
    let a: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >;
    let b: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >;
    let ha: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >;
    let hb: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >;
    let mut copy: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78,
        >,
    >;
    let mut total: terrane_int_support::Int;
    let mut item: terrane_int_support::Int;
    let empty: terrane_collection_support::List<terrane_int_support::Int>;
    a = std::sync::Arc::new(
        std::sync::Mutex::new(
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78::terrane_construct(),
        ),
    );
    b = std::sync::Arc::new(
        std::sync::Mutex::new(
            TerraneNsSharedStorageReentryLoopSafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6C6F6F702D7361666500626F78::terrane_construct(),
        ),
    );
    {
        let __terrane_owner = b.clone();
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        __terrane_owner.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    ha = a.clone();
    hb = b.clone();
    copy = hb.clone();
    total = terrane_int_support::Int::from(0_i128);
    let __terrane_iterable_0 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        total = total.clone() + item.clone();
        copy = hb.clone();
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    let __terrane_iterable_1 = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        item = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        total = total.clone() + item.clone();
        copy = hb.clone();
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    empty = terrane_collection_support::List::<terrane_int_support::Int>::new(vec![]);
    let __terrane_iterable_2 = empty;
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        item = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        total = total.clone() + item.clone();
        copy = ha.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    let __terrane_iterable_3 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_3,
    );
    loop {
        item = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        if item.clone() == terrane_int_support::Int::from(1_i128) {
            break;
        }
        copy = ha.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    let __terrane_iterable_4 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_4,
    );
    loop {
        item = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        if item.clone() == terrane_int_support::Int::from(1_i128) {
            continue;
        }
        copy = ha.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
}
