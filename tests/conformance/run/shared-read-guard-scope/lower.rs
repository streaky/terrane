// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-read-guard-scope
#[derive(Clone)]
pub struct TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78 {
    pub flag: bool,
    pub count: terrane_int_support::Int,
    pub trail: terrane_int_support::Int,
}
impl TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            flag: true,
            count: terrane_int_support::Int::from(1_i128),
            trail: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn update(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = amount.clone();
            self.count = __terrane_value;
        }
    }
    pub fn combine(
        &mut self,
        left: terrane_int_support::Int,
        right: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = left.clone() + right.clone();
            self.count = __terrane_value;
        }
    }
}
fn argument(
    owner: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78,
        >,
    >,
    amount: terrane_int_support::Int,
) -> terrane_int_support::Int {
    {
        let __terrane_owner = owner.clone();
        let __terrane_value = {
            let __terrane_snapshot = owner
                .lock()
                .expect("shared reference lock poisoned")
                .trail
                .clone();
            __terrane_snapshot
        } * terrane_int_support::Int::from(10_i128) + amount.clone();
        __terrane_owner.lock().expect("shared reference lock poisoned").trail = __terrane_value;
    }
    return amount.clone();
}
fn inspect(
    held: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78,
        >,
    >,
) {
    let alias: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78,
        >,
    >;
    let numbers: terrane_collection_support::List<terrane_int_support::Int>;
    alias = held.clone();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held
        .lock().expect("shared reference lock poisoned").flag; __terrane_snapshot }),
        String::from(" "), terrane_scalar_support::scalar_text(&{ let __terrane_snapshot
        = held.lock().expect("shared reference lock poisoned").flag; __terrane_snapshot
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_snapshot = held
        .lock().expect("shared reference lock poisoned").count.clone();
        __terrane_snapshot } + { let __terrane_snapshot = alias.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot }))
    );
    numbers = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            { let __terrane_snapshot = held.lock()
            .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot
            }, { let __terrane_snapshot = alias.lock()
            .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(__terrane_raised(numbers
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:22:12-22:22 */)), 0 /* terrane-site: case.trn:22:12-22:22 */) + __terrane_raised(numbers
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:22:25-22:35 */)), 1 /* terrane-site: case.trn:22:25-22:35 */)))
    );
    {
        let __terrane_owner = held.clone();
        let __terrane_shared_argument_0 = {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .update(__terrane_shared_argument_1);
        __terrane_call_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    {
        let __terrane_owner = held.clone();
        let __terrane_shared_argument_0 = {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        };
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_shared_argument_2 = terrane_int_support::Int::from(1_i128);
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .combine(__terrane_shared_argument_1, __terrane_shared_argument_2);
        __terrane_call_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    {
        let __terrane_owner = held.clone();
        let __terrane_shared_argument_0 = argument(
            alias.clone(),
            terrane_int_support::Int::from(2_i128),
        );
        let __terrane_shared_argument_1 = argument(
            held.clone(),
            terrane_int_support::Int::from(1_i128),
        );
        let __terrane_shared_argument_2 = __terrane_shared_argument_1;
        let __terrane_shared_argument_3 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("shared reference lock poisoned")
            .combine(__terrane_shared_argument_2, __terrane_shared_argument_3);
        __terrane_call_result
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held
        .lock().expect("shared reference lock poisoned").count.clone();
        __terrane_snapshot }), String::from(" "), terrane_scalar_support::scalar_text(&{
        let __terrane_snapshot = held.lock().expect("shared reference lock poisoned")
        .trail.clone(); __terrane_snapshot })
    );
}
fn main() {
    let plain: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78,
        >,
    >;
    let held: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78,
        >,
    >;
    let observer: std::sync::Weak<
        std::sync::Mutex<
            TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78,
        >,
    >;
    plain = std::sync::Arc::new(
        std::sync::Mutex::new(
            TerraneNsSharedReadGuardScopeBoxIdentity2F7368617265642D726561642D67756172642D73636F706500626F78::terrane_construct(),
        ),
    );
    held = plain.clone();
    inspect(held.clone());
    observer = std::sync::Arc::downgrade(&plain.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_owner = observer.upgrade().expect("reference expired"); __terrane_owner
        .lock().expect("reference lock poisoned").flag }; __terrane_snapshot }),
        String::from(" "), terrane_scalar_support::scalar_text(&{ let __terrane_snapshot
        = held.lock().expect("shared reference lock poisoned").flag; __terrane_snapshot
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let __terrane_snapshot = { let
        __terrane_owner = observer.upgrade().expect("reference expired"); __terrane_owner
        .lock().expect("reference lock poisoned").count.clone() }; __terrane_snapshot } +
        { let __terrane_snapshot = held.lock().expect("shared reference lock poisoned")
        .count.clone(); __terrane_snapshot }))
    );
    {
        let __terrane_owner = observer.upgrade().expect("reference expired");
        let __terrane_shared_argument_0 = {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner
            .lock()
            .expect("reference lock poisoned")
            .update(__terrane_shared_argument_1);
        __terrane_call_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_owner = observer.upgrade().expect("reference expired"); __terrane_owner
        .lock().expect("reference lock poisoned").count.clone() }; __terrane_snapshot })
    );
    if {
        let __terrane_snapshot = {
            let __terrane_owner = observer.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").flag
        };
        __terrane_snapshot
    }
        && {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .flag;
            __terrane_snapshot
        }
    {
        println!("{}", String::from("both"));
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held
        .lock().expect("shared reference lock poisoned").flag; __terrane_snapshot }),
        String::from(" "), terrane_scalar_support::scalar_text(&{ let __terrane_snapshot
        = plain.lock().expect("reference lock poisoned").flag; __terrane_snapshot })
    );
}
