// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: visibility-object-declarations
#[derive(Clone)]
pub struct PublicClass {
    pub value: terrane_int_support::Int,
}
impl PublicClass {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub struct PrivateClass {
    pub value: terrane_int_support::Int,
}
impl PrivateClass {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
        }
    }
}
#[derive(Clone)]
pub struct ProtectedClass {
    pub value: terrane_int_support::Int,
}
impl ProtectedClass {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(3_i128),
        }
    }
}
pub trait PublicInterfaceProtocol {
    fn clone_box(&self) -> Box<dyn PublicInterfaceProtocol>;
    fn separate_box(&self) -> Box<dyn PublicInterfaceProtocol>;
    fn value(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn PublicInterfaceProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct PublicInterface(Box<dyn PublicInterfaceProtocol>);
impl PublicInterface {
    pub fn value(&self) -> terrane_int_support::Int {
        self.0.value()
    }
}
pub trait PrivateInterfaceProtocol {
    fn clone_box(&self) -> Box<dyn PrivateInterfaceProtocol>;
    fn separate_box(&self) -> Box<dyn PrivateInterfaceProtocol>;
    fn value(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn PrivateInterfaceProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct PrivateInterface(Box<dyn PrivateInterfaceProtocol>);
impl PrivateInterface {
    pub fn value(&self) -> terrane_int_support::Int {
        self.0.value()
    }
}
pub trait ProtectedInterfaceProtocol {
    fn clone_box(&self) -> Box<dyn ProtectedInterfaceProtocol>;
    fn separate_box(&self) -> Box<dyn ProtectedInterfaceProtocol>;
    fn value(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn ProtectedInterfaceProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct ProtectedInterface(Box<dyn ProtectedInterfaceProtocol>);
impl ProtectedInterface {
    pub fn value(&self) -> terrane_int_support::Int {
        self.0.value()
    }
}
fn main() {
    return ();
}
