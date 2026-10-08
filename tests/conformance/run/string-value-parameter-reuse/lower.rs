// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: case.trn
// Namespace: string-value-parameter-reuse
fn consume(value: String) {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&String::from("consume:")),
        terrane_scalar_support::scalar_text(&value)
    );
}
#[derive(Clone)]
pub struct MessageBox {
    pub message: String,
}
impl MessageBox {
    pub fn terrane_construct(message: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct(message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String) {
        self.message = message.clone();
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("parameter:")),
            terrane_scalar_support::scalar_text(&message)
        );
    }
}
fn main() {
    let entries_terrane_f0_s290: terrane_collection_support::Map<
        String,
        terrane_int_support::Int,
    >;
    let mut key_terrane_f0_s338: String;
    let mut value_terrane_f0_s343: terrane_int_support::Int;
    let holder_terrane_f0_s415: MessageBox;
    entries_terrane_f0_s290 = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("alpha"),
            terrane_int_support::Int::from(1_i128))
        ],
    );
    let __terrane_iterable_0 = entries_terrane_f0_s290;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key_terrane_f0_s338 = __terrane_item_0.key;
        value_terrane_f0_s343 = __terrane_item_0.value;
        consume(key_terrane_f0_s338.clone());
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&String::from("key:")),
            terrane_scalar_support::scalar_text(&key_terrane_f0_s338),
            terrane_scalar_support::scalar_text(&String::from("=")),
            terrane_scalar_support::scalar_text(&value_terrane_f0_s343)
        );
    }
    holder_terrane_f0_s415 = MessageBox::terrane_construct(String::from("saved"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&String::from("field:")),
        terrane_scalar_support::scalar_text(&holder_terrane_f0_s415.message)
    );
}
