// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: authored-interface-transfer-composition
pub trait ReadableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn ReadableProtocol>;
    fn separate_box(&self) -> Box<dyn ReadableProtocol>;
    fn read(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn ReadableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Readable(Box<dyn ReadableProtocol>);
impl Clone for Readable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Readable {
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
}
#[derive(Clone)]
pub struct Leaf {
    pub value: terrane_int_support::Int,
}
impl Leaf {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(3_i128),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
}
impl ReadableProtocol for Leaf {
    fn clone_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.clone())
    }
    fn read(&self) -> terrane_int_support::Int {
        Leaf::read(&*self)
    }
}
impl From<Leaf> for Readable {
    fn from(value: Leaf) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct DirectComposite {
    pub inner: Readable,
}
impl DirectComposite {
    pub fn terrane_construct() -> Self {
        Self {
            inner: <Readable>::from(Leaf::terrane_construct()),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.inner.read();
    }
}
impl ReadableProtocol for DirectComposite {
    fn clone_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.clone())
    }
    fn read(&self) -> terrane_int_support::Int {
        DirectComposite::read(&*self)
    }
}
impl From<DirectComposite> for Readable {
    fn from(value: DirectComposite) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Holder {
    pub inner: Readable,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            inner: <Readable>::from(Leaf::terrane_construct()),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.inner.read();
    }
}
#[derive(Clone)]
pub struct NestedComposite {
    pub inner: Holder,
}
impl NestedComposite {
    pub fn terrane_construct() -> Self {
        Self {
            inner: Holder::terrane_construct(),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.inner.read();
    }
}
impl ReadableProtocol for NestedComposite {
    fn clone_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn ReadableProtocol> {
        Box::new(self.clone())
    }
    fn read(&self) -> terrane_int_support::Int {
        NestedComposite::read(&*self)
    }
}
impl From<NestedComposite> for Readable {
    fn from(value: NestedComposite) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let direct: Readable = <Readable>::from(DirectComposite::terrane_construct());
    let nested: Readable = <Readable>::from(NestedComposite::terrane_construct());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&direct.read()),
        terrane_scalar_support::scalar_text(&nested.read())
    );
}
