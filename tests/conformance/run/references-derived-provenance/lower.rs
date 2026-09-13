// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: references-derived-provenance
#[derive(Clone)]
pub struct Holder {
    pub value: terrane_int_support::Int,
}
impl Holder {
    pub fn terrane_construct(amount: terrane_int_support::Int) -> Self {
        let mut value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(amount);
        value
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
    let owner: Holder = Holder::terrane_construct(
        terrane_int_support::Int::from(41_i128),
    );
    let field: &terrane_int_support::Int = &owner.value;
    let show_field: std::sync::Arc<dyn Fn() -> () + Send + Sync> = {
        let field = field;
        std::sync::Arc::new(move || -> () {
            consume(field);
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("captured"))
            );
            ()
        })
    };
    show_field();
    let values: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(42_i128)
        ],
    );
    let element: &terrane_int_support::Int = {
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
    let returned: &terrane_int_support::Int = pass(element);
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&field.clone()),
        terrane_scalar_support::scalar_text(&element.clone()),
        terrane_scalar_support::scalar_text(&returned.clone())
    );
    let borrowed: &terrane_collection_support::List<terrane_int_support::Int> = &values;
    let mut __terrane_iterator_0 = borrowed.terrane_borrowing_iterator();
    loop {
        let item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&item.clone()));
    }
}
