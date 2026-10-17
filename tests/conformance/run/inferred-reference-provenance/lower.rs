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
        std::sync::Mutex<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >,
) -> bool {
    return value.lock().expect("shared reference lock poisoned").flag;
}
fn count(
    values: &terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        terrane_int_support::Int::from(values.clone().length()),
    );
}
fn early_observer() {
    let named: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    let observer: std::sync::Weak<
        std::sync::Mutex<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    let alias: std::sync::Weak<
        std::sync::Mutex<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    let held: std::sync::Arc<
        std::sync::Mutex<
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78,
        >,
    >;
    named = std::sync::Arc::new(
        std::sync::Mutex::new(
            TerraneNsInferredReferenceProvenanceBoxIdentity2F696E6665727265642D7265666572656E63652D70726F76656E616E636500626F78::terrane_construct(),
        ),
    );
    observer = std::sync::Arc::downgrade(&named.clone());
    alias = observer.clone();
    held = named.clone();
    held.lock().expect("shared reference lock poisoned").flag = false;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = observer
        .upgrade().expect("reference expired"); __terrane_owner.lock()
        .expect("reference lock poisoned").flag.clone() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = alias
        .upgrade().expect("reference expired"); __terrane_owner.lock()
        .expect("reference lock poisoned").flag.clone() })
    );
    println!("{}", terrane_scalar_support::scalar_text(&shared_flag(held.clone())));
}
fn main() {
    let value: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let owner: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let mut observer: std::sync::Weak<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let alias: std::sync::Weak<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let replacement: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let replacement_owner: std::sync::Arc<
        std::sync::Mutex<terrane_collection_support::List<terrane_int_support::Int>>,
    >;
    let mut ordinary: terrane_collection_support::List<terrane_int_support::Int>;
    let ordinary_observer: &terrane_collection_support::List<terrane_int_support::Int>;
    early_observer();
    value = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        ),
    );
    owner = value.clone();
    observer = std::sync::Arc::downgrade(&value.clone());
    {
        let __terrane_owner = observer.upgrade().expect("reference expired");
        __terrane_owner
            .lock()
            .expect("reference lock poisoned")
            .append(terrane_int_support::Int::from(2_i128))
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = value.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised({ let __terrane_value =
        owner.lock().expect("shared reference lock poisoned").clone(); __terrane_value }
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:29:24-29:32 */)), 0 /* terrane-site: case.trn:29:24-29:32 */))
    );
    alias = observer.clone();
    {
        let __terrane_owner = alias.upgrade().expect("reference expired");
        __terrane_owner
            .lock()
            .expect("reference lock poisoned")
            .append(terrane_int_support::Int::from(3_i128))
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = value.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised({ let __terrane_owner =
        observer.upgrade().expect("reference expired"); let __terrane_value =
        __terrane_owner.lock().expect("reference lock poisoned").clone(); __terrane_value
        }
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        1 /* terrane-site: case.trn:32:24-32:35 */)), 1 /* terrane-site: case.trn:32:24-32:35 */))
    );
    replacement = std::sync::Arc::new(
        std::sync::Mutex::new(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(4_i128)]),
        ),
    );
    replacement_owner = replacement.clone();
    observer = std::sync::Arc::downgrade(&replacement.clone());
    {
        let __terrane_owner = observer.upgrade().expect("reference expired");
        __terrane_owner
            .lock()
            .expect("reference lock poisoned")
            .append(terrane_int_support::Int::from(5_i128))
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_value = replacement.lock().expect("reference lock poisoned").clone();
        __terrane_value } .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised({ let __terrane_value =
        replacement_owner.lock().expect("shared reference lock poisoned").clone();
        __terrane_value }
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:38:30-38:50 */)), 2 /* terrane-site: case.trn:38:30-38:50 */))
    );
    ordinary = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(6_i128)]);
    ordinary.append(terrane_int_support::Int::from(7_i128));
    ordinary_observer = &ordinary;
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(ordinary
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(ordinary_observer.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:43:27-43:47 */)), 3 /* terrane-site: case.trn:43:27-43:47 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&count(ordinary_observer)));
}
