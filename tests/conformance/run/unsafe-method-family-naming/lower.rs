// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: unsafe-method-family-naming
#[derive(Clone)]
pub struct PairedBaseStorage {}
impl PairedBaseStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
#[derive(Clone)]
pub enum PairedBase {
    Own(PairedBaseStorage),
    SafeOverride(SafeOverride),
}
impl PairedBase {
    pub fn terrane_construct() -> Self {
        Self::Own(PairedBaseStorage::terrane_construct())
    }
    pub fn raw(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.raw(),
            Self::SafeOverride(value) => value.raw(),
        }
    }
    #[allow(unsafe_code)]
    pub unsafe fn raw_unsafe(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => unsafe { value.raw_unsafe() }
            Self::SafeOverride(value) => unsafe { value.raw_unsafe() }
        }
    }
}
#[derive(Clone)]
pub struct SafeOverride {}
impl SafeOverride {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
#[derive(Clone)]
pub struct SafeBaseStorage {}
impl SafeBaseStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(4_i128);
    }
}
#[derive(Clone)]
pub enum SafeBase {
    Own(SafeBaseStorage),
    UnsafeAddition(UnsafeAddition),
}
impl SafeBase {
    pub fn terrane_construct() -> Self {
        Self::Own(SafeBaseStorage::terrane_construct())
    }
    pub fn raw(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.raw(),
            Self::UnsafeAddition(value) => value.raw(),
        }
    }
}
#[derive(Clone)]
pub struct UnsafeAddition {}
impl UnsafeAddition {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(4_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(5_i128);
    }
}
#[allow(unsafe_code)]
pub trait PairedSourceProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn PairedSourceProtocol>;
    fn separate_box(&self) -> Box<dyn PairedSourceProtocol>;
    fn raw(&self) -> terrane_int_support::Int;
    unsafe fn raw_unsafe(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn PairedSourceProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct PairedSource(Box<dyn PairedSourceProtocol>);
impl Clone for PairedSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[allow(unsafe_code)]
impl PairedSource {
    pub fn raw(&self) -> terrane_int_support::Int {
        self.0.raw()
    }
    pub unsafe fn raw_unsafe(&self) -> terrane_int_support::Int {
        unsafe { self.0.raw_unsafe() }
    }
}
#[derive(Clone)]
pub struct Source {}
impl Source {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(6_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
}
#[allow(unsafe_code)]
impl PairedSourceProtocol for Source {
    fn clone_box(&self) -> Box<dyn PairedSourceProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn PairedSourceProtocol> {
        Box::new(self.clone())
    }
    fn raw(&self) -> terrane_int_support::Int {
        Source::raw(&*self)
    }
    unsafe fn raw_unsafe(&self) -> terrane_int_support::Int {
        unsafe { Source::raw_unsafe(&*self) }
    }
}
impl From<Source> for PairedSource {
    fn from(value: Source) -> Self {
        Self(Box::new(value))
    }
}
#[allow(unsafe_code)]
fn main() {
    let overridden: PairedBase = PairedBase::SafeOverride(
        SafeOverride::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&overridden.raw()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&unsafe { overridden.raw_unsafe() })
    );
    let extended: UnsafeAddition = UnsafeAddition::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&extended.raw()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&unsafe { extended.raw_unsafe() })
    );
    let concrete: Source = Source::terrane_construct();
    let class_operation: std::sync::Arc<
        dyn Fn() -> terrane_int_support::Int + Send + Sync,
    > = {
        let receiver = concrete.clone();
        std::sync::Arc::new(move || receiver.raw())
    };
    let __trn_6162737472616374: PairedSource = <PairedSource>::from(concrete);
    let interface_operation: std::sync::Arc<
        dyn Fn() -> terrane_int_support::Int + Send + Sync,
    > = {
        let receiver = __trn_6162737472616374;
        std::sync::Arc::new(move || receiver.raw())
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&class_operation()),
        terrane_scalar_support::scalar_text(&interface_operation())
    );
}
