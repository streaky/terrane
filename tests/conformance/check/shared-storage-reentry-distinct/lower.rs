// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-storage-reentry-distinct
#[derive(Clone)]
pub struct TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78,
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
    let plain: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78,
        >,
    >;
    let other: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78,
        >,
    >;
    let held: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78,
        >,
    >;
    plain = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78::terrane_construct(),
        ),
    );
    other = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryDistinctBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D64697374696E637400626F78::terrane_construct(),
        ),
    );
    held = plain.clone();
    {
        let __terrane_owner = held.clone();
        let __terrane_shared_argument_0 = other.clone();
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .peek(__terrane_shared_argument_1);
        __terrane_call_result
    };
}
