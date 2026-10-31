// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: inferred-reference-provenance
#[derive(Clone)]
pub struct TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78 {
    pub flag: bool,
}
impl TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78 {
    pub fn terrane_construct() -> Self {
        Self { flag: true }
    }
}
fn shared_flag(
    value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >,
) -> bool {
    return {
        let __terrane_snapshot = value
            .lock()
            .expect("shared reference lock poisoned")
            .flag;
        __terrane_snapshot
    };
}
fn count(
    values: &terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(values.clone().length());
}
fn early_observer() {
    let named: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    let observer: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    let alias: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    let held: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    named = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78::terrane_construct(),
        ),
    );
    observer = std::sync::Arc::downgrade(&named.clone());
    alias = observer.clone();
    held = named.clone();
    {
        let __terrane_owner_0 = held.clone();
        let __terrane_value = false;
        __terrane_owner_0.lock().expect("shared reference lock poisoned").flag = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_owner = observer.upgrade().expect("reference expired"); __terrane_owner
        .lock().expect("reference lock poisoned").flag }; __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_owner = alias.upgrade().expect("reference expired"); __terrane_owner
        .lock().expect("reference lock poisoned").flag }; __terrane_snapshot })
    );
    println!("{}", terrane_scalar_support::scalar_text(&shared_flag(held.clone())));
}
fn main() {
    let value: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let owner: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let mut observer: std::sync::Weak<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let alias: std::sync::Weak<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let replacement: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let replacement_owner: std::sync::Arc<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let mut ordinary: terrane_collection_support::List<terrane_int_support::Int>;
    let mut ordinary_observer: &terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let ordinary_replacement: terrane_collection_support::List<terrane_int_support::Int>;
    let mut typed_observer: &terrane_collection_support::List<terrane_int_support::Int>;
    let mut shared_observer: std::sync::Weak<
        TerraneSharedCell<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    early_observer();
    value = std::sync::Arc::new(
        TerraneSharedCell::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        ),
    );
    owner = value.clone();
    observer = std::sync::Arc::downgrade(&value.clone());
    {
        let __terrane_owner_1 = observer.upgrade().expect("reference expired");
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(2_i128);
        __terrane_owner_1
            .lock()
            .expect("reference lock poisoned")
            .append(__terrane_collection_argument_0);
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = value.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())), terrane_scalar_support::scalar_text(&{ let
        __terrane_owner_2 = &owner; __terrane_raised(__terrane_owner_2.lock()
        .expect("shared reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:29:24-29:32 */)), 0 /* terrane-site: case.trn:29:24-29:32 */) })
    );
    alias = observer.clone();
    {
        let __terrane_owner_3 = alias.upgrade().expect("reference expired");
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(3_i128);
        __terrane_owner_3
            .lock()
            .expect("reference lock poisoned")
            .append(__terrane_collection_argument_0);
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = value.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())), terrane_scalar_support::scalar_text(&{ let
        __terrane_owner_4 = (&observer).upgrade().expect("reference expired");
        __terrane_raised(__terrane_owner_4.lock().expect("reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        1 /* terrane-site: case.trn:32:24-32:35 */)), 1 /* terrane-site: case.trn:32:24-32:35 */) })
    );
    replacement = std::sync::Arc::new(
        TerraneSharedCell::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(4_i128)]),
        ),
    );
    replacement_owner = replacement.clone();
    observer = std::sync::Arc::downgrade(&replacement.clone());
    {
        let __terrane_owner_5 = observer.upgrade().expect("reference expired");
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(5_i128);
        __terrane_owner_5
            .lock()
            .expect("reference lock poisoned")
            .append(__terrane_collection_argument_0);
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = replacement.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())), terrane_scalar_support::scalar_text(&{ let
        __terrane_owner_6 = &replacement_owner; __terrane_raised(__terrane_owner_6.lock()
        .expect("shared reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:38:30-38:50 */)), 2 /* terrane-site: case.trn:38:30-38:50 */) })
    );
    ordinary = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(6_i128)]);
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(7_i128);
        ordinary.append(__terrane_collection_argument_0);
    };
    ordinary_observer = &ordinary;
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(ordinary
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(ordinary_observer
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:43:27-43:47 */)), 3 /* terrane-site: case.trn:43:27-43:47 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&count(ordinary_observer)));
    ordinary_replacement = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(9_i128)
        ],
    );
    typed_observer = &ordinary;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(typed_observer
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:48:10-48:27 */)), 4 /* terrane-site: case.trn:48:10-48:27 */))
    );
    ordinary_observer = &ordinary_replacement;
    typed_observer = &ordinary_replacement;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&count(ordinary_observer)),
        terrane_scalar_support::scalar_text(&__terrane_raised(typed_observer
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        5 /* terrane-site: case.trn:51:38-51:55 */)), 5 /* terrane-site: case.trn:51:38-51:55 */))
    );
    shared_observer = std::sync::Arc::downgrade(&value.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_7 =
        (&shared_observer).upgrade().expect("reference expired");
        __terrane_raised(__terrane_owner_7.lock().expect("reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:54:10-54:28 */)), 6 /* terrane-site: case.trn:54:10-54:28 */) })
    );
    shared_observer = std::sync::Arc::downgrade(&replacement.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_8 =
        (&shared_observer).upgrade().expect("reference expired");
        __terrane_raised(__terrane_owner_8.lock().expect("reference lock poisoned")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:56:10-56:28 */)), 7 /* terrane-site: case.trn:56:10-56:28 */) })
    );
}
