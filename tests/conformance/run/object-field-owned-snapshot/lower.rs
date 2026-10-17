// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: object-field-owned-snapshot
#[derive(Clone)]
pub struct Payload {
    pub value: terrane_int_support::Int,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub struct Holder {
    pub required: Option<Payload>,
    pub initialized: Payload,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            required: None,
            initialized: Payload::terrane_construct(),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        self.required = Some(Payload::terrane_construct());
    }
    pub fn required_copy(&self) -> Payload {
        return self.required.as_ref().expect("required field initialized").clone();
    }
    pub fn initialized_copy(&self) -> Payload {
        return self.initialized.clone();
    }
    pub fn change_copy(&self) -> terrane_int_support::Int {
        let mut local: Payload;
        local = self.required.as_ref().expect("required field initialized").clone();
        local.value = terrane_int_support::Int::from(9_i128);
        return local.value.clone();
    }
    pub fn temporary_required_copy(&self) -> Payload {
        return Holder::terrane_construct()
            .required
            .as_ref()
            .expect("required field initialized")
            .clone();
    }
    pub fn required_from_borrow(&self, owner: &Holder) -> Payload {
        return owner.required.as_ref().expect("required field initialized").clone();
    }
    pub fn required_from_shared(
        &self,
        owner: std::sync::Arc<std::sync::Mutex<Holder>>,
    ) -> Payload {
        return owner
            .lock()
            .expect("shared reference lock poisoned")
            .required
            .as_ref()
            .expect("required field initialized")
            .clone();
    }
}
#[derive(Clone)]
pub struct PlainHolder {
    pub value: Payload,
    pub words: terrane_collection_support::List<String>,
    pub text: String,
}
impl PlainHolder {
    pub fn terrane_construct() -> Self {
        Self {
            value: Payload::terrane_construct(),
            words: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("original")]),
            text: String::from("original"),
        }
    }
}
fn make_holder() -> PlainHolder {
    return PlainHolder::terrane_construct();
}
fn borrow_holder<'a>(value: &'a PlainHolder) -> &'a PlainHolder {
    return value;
}
#[derive(Clone)]
pub struct TrackedPayload {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl TrackedPayload {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
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
            "{}{}", String::from("released"), terrane_scalar_support::scalar_text(&self
            .value)
        );
    }
}
impl Drop for TrackedPayload {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct TrackedHolder {
    pub value: TrackedPayload,
}
impl TrackedHolder {
    pub fn terrane_construct() -> Self {
        Self {
            value: TrackedPayload::terrane_construct(),
        }
    }
}
fn check_separated() {
    let named: TrackedHolder;
    let borrowed: &TrackedHolder;
    let mut snapshot: TrackedPayload;
    named = TrackedHolder::terrane_construct();
    borrowed = &named;
    snapshot = borrowed.value.terrane_separate();
    snapshot.value = terrane_int_support::Int::from(17_i128);
    println!("{}", terrane_scalar_support::scalar_text(&snapshot.value));
    println!("{}", terrane_scalar_support::scalar_text(&named.value.value));
}
fn main() {
    let original: Holder;
    let first: Payload;
    let second: Payload;
    let unchanged: Payload;
    let mut from_call: Payload;
    let mut from_constructor: Payload;
    let named: PlainHolder;
    let mut from_borrow: Payload;
    let shared: std::sync::Arc<std::sync::Mutex<PlainHolder>>;
    let mut from_shared: Payload;
    let mut required_borrow: Payload;
    let shared_required: std::sync::Arc<std::sync::Mutex<Holder>>;
    let mut required_shared: Payload;
    let mut words: terrane_collection_support::List<String>;
    let text: String;
    original = Holder::terrane_construct();
    first = original.required_copy();
    second = original.initialized_copy();
    println!("{}", terrane_scalar_support::scalar_text(&first.value));
    println!("{}", terrane_scalar_support::scalar_text(&second.value));
    println!("{}", terrane_scalar_support::scalar_text(&original.change_copy()));
    unchanged = original.required_copy();
    println!("{}", terrane_scalar_support::scalar_text(&unchanged.value));
    from_call = make_holder().value;
    from_call.value = terrane_int_support::Int::from(11_i128);
    from_constructor = PlainHolder::terrane_construct().value;
    from_constructor.value = terrane_int_support::Int::from(12_i128);
    println!("{}", terrane_scalar_support::scalar_text(&from_call.value));
    println!("{}", terrane_scalar_support::scalar_text(&from_constructor.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&original.temporary_required_copy()
        .value)
    );
    named = PlainHolder::terrane_construct();
    from_borrow = borrow_holder(&named).value.clone();
    from_borrow.value = terrane_int_support::Int::from(13_i128);
    println!("{}", terrane_scalar_support::scalar_text(&from_borrow.value));
    println!("{}", terrane_scalar_support::scalar_text(&named.value.value));
    shared = std::sync::Arc::new(
        std::sync::Mutex::new(PlainHolder::terrane_construct()),
    );
    from_shared = shared.lock().expect("shared reference lock poisoned").value.clone();
    from_shared.value = terrane_int_support::Int::from(14_i128);
    println!("{}", terrane_scalar_support::scalar_text(&from_shared.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&shared.lock()
        .expect("shared reference lock poisoned").value.clone().value)
    );
    required_borrow = original.required_from_borrow(&original);
    required_borrow.value = terrane_int_support::Int::from(15_i128);
    println!("{}", terrane_scalar_support::scalar_text(&required_borrow.value));
    println!("{}", terrane_scalar_support::scalar_text(&original.required_copy().value));
    shared_required = std::sync::Arc::new(
        std::sync::Mutex::new(Holder::terrane_construct()),
    );
    required_shared = original.required_from_shared(shared_required.clone());
    required_shared.value = terrane_int_support::Int::from(16_i128);
    println!("{}", terrane_scalar_support::scalar_text(&required_shared.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&original
        .required_from_shared(shared_required.clone()).value)
    );
    words = borrow_holder(&named).words.clone();
    let _ = __terrane_raised(
        words
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    0 /* terrane-site: case.trn:99:5-99:25 */,
                ),
                String::from("changed"),
            ),
        0 /* terrane-site: case.trn:99:5-99:25 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:100:12-100:20 */)), 1 /* terrane-site: case.trn:100:12-100:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(named.words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:101:12-101:26 */)), 2 /* terrane-site: case.trn:101:12-101:26 */))
    );
    text = borrow_holder(&named).text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&text));
    println!("{}", terrane_scalar_support::scalar_text(&named.text));
    check_separated();
}
