// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-storage-reentry-safe
#[derive(Clone)]
pub struct TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
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
    pub fn add_from(
        &mut self,
        other: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
            >,
        >,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone()
                + {
                    let __terrane_snapshot = other
                        .lock()
                        .expect("shared reference lock poisoned")
                        .count
                        .clone();
                    __terrane_snapshot
                };
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
}
fn distinct(
    first: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >,
    second: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
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
fn pick(
    value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >,
) -> std::sync::Arc<
    TerraneSharedCell<
        TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
    >,
> {
    return value.clone();
}
fn replaced(
    mut value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >,
    replacement: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >,
) -> std::sync::Arc<
    TerraneSharedCell<
        TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
    >,
> {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = value
        .lock().expect("shared reference lock poisoned").count.clone();
        __terrane_snapshot })
    );
    value = replacement.clone();
    return value.clone();
}
fn branch(flag: bool) -> terrane_int_support::Int {
    let first: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let second: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let third: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let a: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let b: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let c: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let mut copy: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    first = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78::terrane_construct(),
        ),
    );
    second = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78::terrane_construct(),
        ),
    );
    third = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78::terrane_construct(),
        ),
    );
    {
        let __terrane_owner = second.clone();
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        __terrane_owner.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    {
        let __terrane_owner = third.clone();
        let __terrane_value = terrane_int_support::Int::from(8_i128);
        __terrane_owner.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    a = first.clone();
    b = second.clone();
    c = third.clone();
    copy = a.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = copy.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    if flag {
        copy = b.clone();
    } else {
        copy = c.clone();
    }
    return {
        let __terrane_owner = a.clone();
        let __terrane_shared_argument_0 = copy.clone();
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .peek(__terrane_shared_argument_1);
        __terrane_call_result
    };
}
fn main() {
    let plain: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let other: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let mut held: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let alternate: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let alias: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    let observer: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78,
        >,
    >;
    plain = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78::terrane_construct(),
        ),
    );
    other = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedStorageReentrySafeBoxIdentity2F7368617265642D73746F726167652D7265656E7472792D7361666500626F78::terrane_construct(),
        ),
    );
    {
        let __terrane_owner = other.clone();
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        __terrane_owner.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    held = plain.clone();
    alternate = other.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&distinct(held.clone(), alternate
        .clone()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = held.clone();
        let __terrane_shared_argument_0 = alternate.clone(); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").add_from(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    alias = held.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    held = alternate.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = alias.clone();
        let __terrane_shared_argument_0 = held.clone(); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = held.clone();
        let __terrane_shared_argument_0 = alias.clone(); let __terrane_shared_argument_1
        = __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = plain.clone();
        let __terrane_shared_argument_0 = held.clone(); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner.lock()
        .expect("reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    observer = std::sync::Arc::downgrade(&plain.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = observer
        .upgrade().expect("reference expired"); let __terrane_shared_argument_0 =
        pick(held.clone()).clone(); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner.lock()
        .expect("reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = alias.clone();
        let __terrane_shared_argument_0 = replaced(alias.clone(), held.clone()).clone();
        let __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner.lock()
        .expect("shared reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!("{}", terrane_scalar_support::scalar_text(&branch(true)));
    println!("{}", terrane_scalar_support::scalar_text(&branch(false)));
}
