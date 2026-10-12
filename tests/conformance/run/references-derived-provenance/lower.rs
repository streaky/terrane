// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: references-derived-provenance
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct(amount: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(amount);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, amount: terrane_int_support::Int) {
        self.value = amount.clone();
    }
}
fn pass<'a>(value: &'a terrane_int_support::Int) -> &'a terrane_int_support::Int {
    return value;
}
fn consume(__trn_5f76616c7565: &terrane_int_support::Int) {
    let _ = &__trn_5f76616c7565;
}
fn main() {
    let owner: Holder;
    let field: &terrane_int_support::Int;
    let show_field: std::sync::Arc<dyn Fn() -> () + Send + Sync>;
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let element: &terrane_int_support::Int;
    let returned: &terrane_int_support::Int;
    let borrowed: &terrane_collection_support::List<terrane_int_support::Int>;
    let mut item: &terrane_int_support::Int;
    owner = Holder::terrane_construct(terrane_int_support::Int::from(41_i128));
    field = &owner.value;
    show_field = {
        let field = field;
        std::sync::Arc::new(move || -> () {
            consume(field);
            println!("{}", String::from("captured"));
            ()
        })
    };
    show_field();
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(42_i128)
        ],
    );
    element = {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(1_i128),
            ),
            0 /* terrane-site: case.trn:23:25-23:34 */,
        );
        __terrane_raised(
            values
                .get(__terrane_index)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index,
                )),
            0 /* terrane-site: case.trn:23:25-23:34 */,
        )
    };
    returned = pass(element);
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&* field),
        terrane_scalar_support::scalar_text(&* element),
        terrane_scalar_support::scalar_text(&* returned)
    );
    borrowed = &values;
    let __terrane_iterable_0 = borrowed;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&* item));
    }
}
