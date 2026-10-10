// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: authored-enum-catch-all
#[derive(Clone)]
pub struct Resource {
    __terrane_lifetime: std::sync::Arc<()>,
}
impl Resource {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn destruct(&mut self) {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("cleaned")));
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
pub enum State {
    Ready,
    Held(Resource),
}
fn main() {
    let payload: Resource;
    let value: State;
    payload = Resource::terrane_construct();
    value = {
        let __terrane_enum_payload_0 = payload;
        State::Held(__terrane_enum_payload_0)
    };
    let __terrane_match_value_269 = value;
    match __terrane_match_value_269 {
        State::Ready => {
            println!("{}", terrane_scalar_support::scalar_text(&String::from("ready")));
        }
        _ => {
            println!("{}", terrane_scalar_support::scalar_text(&String::from("other")));
        }
    }
}
