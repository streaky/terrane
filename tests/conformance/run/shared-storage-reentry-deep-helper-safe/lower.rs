// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: reentry-deep-helper-safe
#[derive(Clone)]
pub struct TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            std::sync::Mutex<
                TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
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
fn inner(
    first: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
    second: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
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
fn middle(
    left: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
    right: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
) -> terrane_int_support::Int {
    return inner(left.clone(), right.clone());
}
fn outer(
    alpha: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
    beta: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
) -> terrane_int_support::Int {
    return middle(alpha.clone(), beta.clone());
}
fn cycle_left(
    first: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
    second: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
    depth: terrane_int_support::Int,
) -> terrane_int_support::Int {
    if depth.clone() > terrane_int_support::Int::from(0_i128) {
        return cycle_right(
            second.clone(),
            first.clone(),
            depth.clone() - terrane_int_support::Int::from(1_i128),
        );
    }
    return {
        let __terrane_snapshot = first
            .lock()
            .expect("shared reference lock poisoned")
            .count
            .clone();
        __terrane_snapshot
    }
        + {
            let __terrane_snapshot = second
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        };
}
fn cycle_right(
    first: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
    second: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >,
    depth: terrane_int_support::Int,
) -> terrane_int_support::Int {
    if depth.clone() > terrane_int_support::Int::from(0_i128) {
        return cycle_left(
            first.clone(),
            second.clone(),
            depth.clone() - terrane_int_support::Int::from(1_i128),
        );
    }
    return {
        let __terrane_snapshot = first
            .lock()
            .expect("shared reference lock poisoned")
            .count
            .clone();
        __terrane_snapshot
    }
        + {
            let __terrane_snapshot = second
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        };
}
fn main() {
    let a: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >;
    let b: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >;
    let ha: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >;
    let hb: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >;
    let mut copy: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78,
        >,
    >;
    a = std::sync::Arc::new(
        std::sync::Mutex::new(
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78::terrane_construct(),
        ),
    );
    b = std::sync::Arc::new(
        std::sync::Mutex::new(
            TerraneNsReentryDeepHelperSafeBoxIdentity2F7265656E7472792D646565702D68656C7065722D7361666500626F78::terrane_construct(),
        ),
    );
    {
        let __terrane_owner = b.clone();
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        __terrane_owner.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    ha = a.clone();
    hb = b.clone();
    copy = ha.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(copy.clone(), hb.clone()))
    );
    copy = hb.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer(ha.clone(), copy.clone()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&cycle_right(ha.clone(), ha.clone(),
        terrane_int_support::Int::from(2_i128)))
    );
}
