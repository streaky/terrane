use std::collections::HashMap;

pub struct Payload(pub i32);
pub struct Map<T = Payload>(HashMap<String, T>);

impl<T> Map<T> {
    pub fn get(&self) -> Option<&T> {
        self.0.values().next()
    }
}

pub struct Holder(Map<Payload>);

impl Holder {
    pub fn values(&self) -> &Map<Payload> {
        &self.0
    }
}

pub fn make_holder() -> Holder {
    Holder(Map(HashMap::from([("answer".to_owned(), Payload(7))])))
}
