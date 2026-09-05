// Generated deterministically by Terrane <version>.
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
        let mut value = Self { message: String::from("") };
        value.construct(message);
        value
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
    let entries: terrane_collection_support::Map<String, terrane_int_support::Int> = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("alpha"),
            terrane_int_support::Int::from(1_i128))
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &entries,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let key = __terrane_item_0.key;
        let value = __terrane_item_0.value;
        consume(key.clone());
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&String::from("key:")),
            terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&String::from("=")),
            terrane_scalar_support::scalar_text(&value)
        );
    }
    let holder: MessageBox = MessageBox::terrane_construct(String::from("saved"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&String::from("field:")),
        terrane_scalar_support::scalar_text(&holder.message)
    );
}
