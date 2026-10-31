// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-storage-reentry-counter-loop
#[derive(Clone)]
pub struct TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(5_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78,
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
            TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78,
        >,
    >;
    let b: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78,
        >,
    >;
    let ha: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78,
        >,
    >;
    let hb: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78,
        >,
    >;
    let mut copy: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78,
        >,
    >;
    let mut i: terrane_int_support::Int;
    a = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78::terrane_construct(),
        ),
    );
    b = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentryCounterLoopBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D636F756E7465722D6C6F6F7000626F78::terrane_construct(),
        ),
    );
    ha = a.clone();
    hb = b.clone();
    copy = hb.clone();
    i = terrane_int_support::Int::from(0_i128);
    '__terrane_break_0: while i < terrane_int_support::Int::from(2_i128) {
        '__terrane_continue_0: {
            copy = ha.clone();
        }
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_1 = ha.clone();
        let __terrane_shared_argument_0 = copy.clone(); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_1.lock()
        .expect("shared reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
}
