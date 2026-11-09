// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Point {
    pub value: terrane_int_support::Int,
    pub other: terrane_int_support::Int,
}
impl Point {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
            other: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub struct Payload {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn destruct(&mut self) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(10); __terrane_text.push_str("destroyed ");
            terrane_scalar_support::ScalarDisplay::write_scalar(&self.value, &mut
            __terrane_text); __terrane_text })
        );
    }
}
impl Drop for Payload {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn reset(value: &mut terrane_int_support::Int) -> terrane_int_support::Int {
    *value = terrane_int_support::Int::from(8_i128);
    return (*value).clone();
}
fn forward(value: &mut terrane_int_support::Int) {
    reset(&mut *value);
}
fn replace(target: &mut Point, replacement: Point) {
    *target = replacement;
}
fn replace_payload(target: &mut Payload, replacement: Payload) {
    *target = replacement;
}
fn main() {
    let mut scalar: terrane_int_support::Int;
    let borrowed: &mut terrane_int_support::Int;
    let callback: TerraneBorrowedConsumingCallable<'_, (), ()>;
    let mut owner: Point;
    let field_alias: &mut terrane_int_support::Int;
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let mut replacement: Point;
    let owner_alias: &mut Point;
    let mut old_payload: Payload;
    let mut new_payload: Payload;
    scalar = terrane_int_support::Int::from(2_i128);
    reset(&mut scalar);
    borrowed = &mut scalar;
    reset(&mut *borrowed);
    println!("{}", terrane_scalar_support::scalar_text(&* borrowed));
    callback = {
        let borrowed = borrowed;
        TerraneBorrowedConsumingCallable::new(move |(): ()| -> () {
            *borrowed = terrane_int_support::Int::from(9_i128);
            ()
        })
    };
    callback.call(());
    println!("{}", terrane_scalar_support::scalar_text(&scalar));
    forward(&mut scalar);
    println!("{}", terrane_scalar_support::scalar_text(&scalar));
    owner = Point::terrane_construct();
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            4_i128,
        );
        owner.value = __terrane_value;
    }
    reset(&mut owner.value);
    println!("{}", terrane_scalar_support::scalar_text(&owner.value));
    field_alias = &mut owner.value;
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            5_i128,
        );
        owner.other = __terrane_value;
    }
    *field_alias = terrane_int_support::Int::from(6_i128);
    println!("{}", terrane_scalar_support::scalar_text(&owner.value));
    println!("{}", terrane_scalar_support::scalar_text(&owner.other));
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(4_i128)
        ],
    );
    reset({
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            0 /* terrane-site: src/main.trn:45:10-45:19 */,
        );
        __terrane_raised(
            values
                .get_mut(__terrane_index)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index,
                )),
            0 /* terrane-site: src/main.trn:45:10-45:19 */,
        )
    });
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: src/main.trn:46:10-46:19 */)), 1 /* terrane-site: src/main.trn:46:10-46:19 */))
    );
    replacement = Point::terrane_construct();
    replace(&mut owner, replacement.clone());
    owner_alias = &mut owner;
    replace(&mut *owner_alias, replacement.clone());
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            12_i128,
        );
        replacement.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&owner.value));
    old_payload = Payload::terrane_construct();
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            6_i128,
        );
        old_payload.value = __terrane_value;
    }
    new_payload = Payload::terrane_construct();
    {
        let __terrane_value: terrane_int_support::Int = terrane_int_support::Int::from(
            12_i128,
        );
        new_payload.value = __terrane_value;
    }
    replace_payload(&mut old_payload, new_payload);
    println!("{}", terrane_scalar_support::scalar_text(&old_payload.value));
}
