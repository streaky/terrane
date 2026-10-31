// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-storage-reentry-nested-loop
#[derive(Clone)]
pub struct TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(5_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78,
            >,
        >,
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
    let a: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78,
        >,
    >;
    let b: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78,
        >,
    >;
    let ha: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78,
        >,
    >;
    let hb: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78,
        >,
    >;
    let mut copy: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78,
        >,
    >;
    let mut outer: terrane_int_support::Int;
    let mut inner: terrane_int_support::Int;
    a = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78::terrane_construct(),
        ),
    );
    b = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryNestedLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D6E65737465642D6C6F6F7000626F78::terrane_construct(),
        ),
    );
    ha = a.clone();
    hb = b.clone();
    copy = hb.clone();
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
        outer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &outer;
        let __terrane_iterable_1 = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(3_i128),
                terrane_int_support::Int::from(4_i128)
            ],
        );
        let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_1,
        );
        loop {
            inner = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &inner;
            copy = ha.clone();
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_1 = ha.clone();
        let __terrane_shared_argument_0 = copy.clone(); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_1.lock()
        .expect("shared reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
}
