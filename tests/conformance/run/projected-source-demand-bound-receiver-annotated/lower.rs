// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub trait MakerProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn MakerProtocol>;
    fn separate_box(&self) -> Box<dyn MakerProtocol>;
    fn make(&self) -> Holder;
}
impl Clone for Box<dyn MakerProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Maker(Box<dyn MakerProtocol>);
impl Clone for Maker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Maker {
    pub fn make(&self) -> Holder {
        self.0.make()
    }
}
#[derive(Clone)]
pub struct Producer {}
impl Producer {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn make(&self) -> Holder {
        return __terrane_raised(
            terrane_static_trn_486f6c646572_new(),
            0 /* terrane-site: src/main.trn:11:16-11:28 */,
        );
    }
}
impl MakerProtocol for Producer {
    fn clone_box(&self) -> Box<dyn MakerProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn MakerProtocol> {
        Box::new(self.clone())
    }
    fn make(&self) -> Holder {
        Producer::make(&*self)
    }
}
impl From<Producer> for Maker {
    fn from(value: Producer) -> Self {
        Self(Box::new(value))
    }
}
fn inspect<TerraneType4D: MakerProtocol>(maker: &TerraneType4D) {
    let value: Holder;
    value = maker.make();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&value).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 1 /* terrane-site: src/main.trn:15:13-15:29 */))
    );
}
fn main() {
    let maker: Producer;
    maker = Producer::terrane_construct();
    inspect(&maker);
}
