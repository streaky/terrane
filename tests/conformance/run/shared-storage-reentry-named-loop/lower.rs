// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: checked-reentry
#[derive(Clone)]
pub struct TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78 {
    pub count: terrane_int_support::Int,
}
impl TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(5_i128),
        }
    }
    pub fn peek(
        &self,
        other: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
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
#[allow(dead_code)]
fn invoke(
    first: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
        >,
    >,
    second: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
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
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
        >,
    >;
    let b: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
        >,
    >;
    let ha: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
        >,
    >;
    let hb: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
        >,
    >;
    let mut copy: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78,
        >,
    >;
    let items: terrane_collection_support::List<terrane_int_support::Int>;
    let mut item: terrane_int_support::Int;
    a = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78::terrane_construct(),
        ),
    );
    b = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsCheckedReentryBoxIdentity2F636865636B65642D7265656E74727900626F78::terrane_construct(),
        ),
    );
    ha = a.clone();
    hb = b.clone();
    copy = hb.clone();
    items = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let __terrane_iterable_0 = items;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &item;
        copy = ha.clone();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_3 = ha.clone();
        let __terrane_shared_argument_0 = copy.clone(); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_3.lock()
        .expect("shared reference lock poisoned").peek(__terrane_shared_argument_1);
        __terrane_call_result })
    );
}
