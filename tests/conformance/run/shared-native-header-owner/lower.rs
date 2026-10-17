// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct HeaderOwner {
    pub headers: Option<
        std::sync::Arc<std::sync::Mutex<http::HeaderMap<http::HeaderValue>>>,
    >,
}
impl HeaderOwner {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { headers: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        let initial: std::sync::Arc<
            std::sync::Mutex<http::HeaderMap<http::HeaderValue>>,
        >;
        initial = std::sync::Arc::new(std::sync::Mutex::new(::http::HeaderMap::new()));
        self.headers = Some(initial.clone());
    }
    pub fn replace(&self, value: String) {
        let headers: std::sync::Arc<
            std::sync::Mutex<http::HeaderMap<http::HeaderValue>>,
        >;
        let _ = &value;
        headers = self.headers.as_ref().expect("required field initialized").clone();
        let _ = &headers;
        {
            let value = value.clone();
            let headers = headers.clone();
            headers
                .lock()
                .expect("lock")
                .insert("x-value", ::http::HeaderValue::from_str(&value).unwrap());
        }
    }
    pub fn read(&self) -> String {
        let headers: std::sync::Arc<
            std::sync::Mutex<http::HeaderMap<http::HeaderValue>>,
        >;
        let value: String;
        headers = self.headers.as_ref().expect("required field initialized").clone();
        let _ = &headers;
        value = {
            let headers = headers.clone();
            headers
                .lock()
                .expect("lock")
                .get("x-value")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        };
        return value;
    }
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
    let __terrane_match_value_972 = value.clone();
    match __terrane_match_value_972 {
        OptionalHeaders::Absent => {
            return terrane_int_support::Int::from(-1_i128);
        }
        OptionalHeaders::Present(__terrane_pattern_1052_owner) => {
            owner = __terrane_pattern_1052_owner;
            return owner
                .lock()
                .expect("shared reference lock poisoned")
                .value
                .clone()
                .clone();
        }
    }
}
fn main() {
    let original: HeaderOwner;
    let copied: HeaderOwner;
    let owner: std::sync::Arc<std::sync::Mutex<Resource>>;
    let value: OptionalHeaders;
    let copied_value: OptionalHeaders;
    original = HeaderOwner::terrane_construct();
    original.replace(String::from("first"));
    copied = original.clone();
    copied.replace(String::from("second"));
    println!("{}", terrane_scalar_support::scalar_text(&original.read()));
    println!("{}", terrane_scalar_support::scalar_text(&copied.read()));
    owner = std::sync::Arc::new(std::sync::Mutex::new(Resource::terrane_construct()));
    value = {
        let __terrane_enum_payload_0 = owner.clone();
        OptionalHeaders::Present(__terrane_enum_payload_0)
    };
    copied_value = value.clone();
    println!("{}", terrane_scalar_support::scalar_text(&count(value)));
    println!("{}", terrane_scalar_support::scalar_text(&count(copied_value)));
}
