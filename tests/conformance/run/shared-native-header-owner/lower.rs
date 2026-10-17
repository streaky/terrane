// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct RequestBuilderOwner {
    pub builder: Option<std::sync::Arc<std::sync::Mutex<Builder>>>,
}
impl RequestBuilderOwner {
    pub fn terrane_construct(
        builder: std::sync::Arc<std::sync::Mutex<Builder>>,
    ) -> Self {
        let mut __terrane_constructed_value = Self { builder: None };
        __terrane_constructed_value.construct(builder);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, builder: std::sync::Arc<std::sync::Mutex<Builder>>) {
        {
            let __terrane_value = builder.clone();
            self.builder = Some(__terrane_value);
        }
    }
    pub fn set_method(&self, value: String) {
        let builder: std::sync::Arc<std::sync::Mutex<Builder>>;
        let _ = &value;
        builder = self.builder.as_ref().expect("required field initialized").clone();
        let _ = &builder;
        {
            let value = value.clone();
            let builder = builder.clone();
            let replacement = ::http::Request::builder()
                .method(::http::Method::from_bytes(value.as_bytes()).unwrap());
            let _ = std::mem::replace(&mut *builder.lock().expect("lock"), replacement);
        }
    }
    pub fn read_method(&self) -> String {
        let builder: std::sync::Arc<std::sync::Mutex<Builder>>;
        let value: String;
        builder = self.builder.as_ref().expect("required field initialized").clone();
        let _ = &builder;
        value = {
            let builder = builder.clone();
            builder.lock().expect("lock").method_ref().unwrap().as_str().to_owned()
        };
        return value;
    }
}
fn make_builders() -> terrane_collection_support::List<
    std::sync::Arc<std::sync::Mutex<Builder>>,
> {
    let initial: std::sync::Arc<std::sync::Mutex<Builder>>;
    let owner: std::sync::Arc<std::sync::Mutex<Builder>>;
    initial = std::sync::Arc::new(
        std::sync::Mutex::new(::http::Request::builder().method(::http::Method::GET)),
    );
    owner = initial.clone();
    return terrane_collection_support::List::<
        std::sync::Arc<std::sync::Mutex<Builder>>,
    >::new(vec![owner.clone(), owner.clone()]);
}
#[derive(Clone)]
pub struct Resource {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl Resource {
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
        println!("{}", String::from("released"));
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub enum OptionalHeaders {
    Absent,
    Present(std::sync::Arc<std::sync::Mutex<Resource>>),
}
fn count(value: OptionalHeaders) -> terrane_int_support::Int {
    let owner: std::sync::Arc<std::sync::Mutex<Resource>>;
    let __terrane_match_value_1233 = value.clone();
    match __terrane_match_value_1233 {
        OptionalHeaders::Absent => {
            return terrane_int_support::Int::from(-1_i128);
        }
        OptionalHeaders::Present(__terrane_pattern_1313_owner) => {
            owner = __terrane_pattern_1313_owner;
            return {
                let __terrane_snapshot = owner
                    .lock()
                    .expect("shared reference lock poisoned")
                    .value
                    .clone();
                __terrane_snapshot
            };
        }
    }
}
fn main() {
    let builders: terrane_collection_support::List<
        std::sync::Arc<std::sync::Mutex<Builder>>,
    >;
    let mut builder: std::sync::Arc<std::sync::Mutex<Builder>>;
    let mut view: RequestBuilderOwner;
    let first: std::sync::Arc<std::sync::Mutex<Builder>>;
    let original: RequestBuilderOwner;
    let copied: RequestBuilderOwner;
    let second: std::sync::Arc<std::sync::Mutex<Builder>>;
    let other_view: RequestBuilderOwner;
    let owner_resource: std::sync::Arc<std::sync::Mutex<Resource>>;
    let value: OptionalHeaders;
    let copied_value: OptionalHeaders;
    builders = make_builders();
    let __terrane_iterable_0 = builders.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        builder = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        view = RequestBuilderOwner::terrane_construct(builder.clone());
        view.set_method(String::from("POST"));
        println!("{}", terrane_scalar_support::scalar_text(&view.read_method()));
    }
    first = __terrane_raised(
            builders
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        0 /* terrane-site: src/main.trn:53:13-53:24 */,
                    ),
                ),
            0 /* terrane-site: src/main.trn:53:13-53:24 */,
        )
        .clone();
    original = RequestBuilderOwner::terrane_construct(first.clone());
    copied = original.clone();
    copied.set_method(String::from("PATCH"));
    println!("{}", terrane_scalar_support::scalar_text(&original.read_method()));
    println!("{}", terrane_scalar_support::scalar_text(&copied.read_method()));
    second = __terrane_raised(
            builders
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(1_i128),
                        ),
                        1 /* terrane-site: src/main.trn:59:14-59:25 */,
                    ),
                ),
            1 /* terrane-site: src/main.trn:59:14-59:25 */,
        )
        .clone();
    other_view = RequestBuilderOwner::terrane_construct(second.clone());
    println!("{}", terrane_scalar_support::scalar_text(&other_view.read_method()));
    owner_resource = std::sync::Arc::new(
        std::sync::Mutex::new(Resource::terrane_construct()),
    );
    value = {
        let __terrane_enum_payload_0 = owner_resource.clone();
        OptionalHeaders::Present(__terrane_enum_payload_0)
    };
    copied_value = value.clone();
    println!("{}", terrane_scalar_support::scalar_text(&count(value)));
    println!("{}", terrane_scalar_support::scalar_text(&count(copied_value)));
}
