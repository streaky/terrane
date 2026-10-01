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
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
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
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => unsafe { value._terrane_unsafe_726177() }
            Self::SafeOverride(value) => unsafe { value._terrane_unsafe_726177() }
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
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
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
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(5_i128);
    }
}
#[allow(unsafe_code)]
pub trait PairedSourceProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn PairedSourceProtocol>;
    fn separate_box(&self) -> Box<dyn PairedSourceProtocol>;
    fn raw(&self) -> terrane_int_support::Int;
    unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int;
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
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        unsafe { self.0._terrane_unsafe_726177() }
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
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
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
    unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        unsafe { Source::_terrane_unsafe_726177(&*self) }
    }
}
impl From<Source> for PairedSource {
    fn from(value: Source) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct SameOwnerCollision {}
impl SameOwnerCollision {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(8_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(9_i128);
    }
}
#[derive(Clone)]
pub struct InheritedCollisionBaseStorage {}
impl InheritedCollisionBaseStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(10_i128);
    }
}
#[derive(Clone)]
pub enum InheritedCollisionBase {
    Own(InheritedCollisionBaseStorage),
    InheritedCollision(InheritedCollision),
}
impl InheritedCollisionBase {
    pub fn terrane_construct() -> Self {
        Self::Own(InheritedCollisionBaseStorage::terrane_construct())
    }
    pub fn raw_unsafe(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.raw_unsafe(),
            Self::InheritedCollision(value) => value.raw_unsafe(),
        }
    }
}
#[derive(Clone)]
pub struct InheritedCollision {}
impl InheritedCollision {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(10_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(11_i128);
    }
}
#[allow(unsafe_code)]
pub trait InterfaceCollisionProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn InterfaceCollisionProtocol>;
    fn separate_box(&self) -> Box<dyn InterfaceCollisionProtocol>;
    fn raw_unsafe(&self) -> terrane_int_support::Int;
    unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn InterfaceCollisionProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct InterfaceCollision(Box<dyn InterfaceCollisionProtocol>);
impl Clone for InterfaceCollision {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[allow(unsafe_code)]
impl InterfaceCollision {
    pub fn raw_unsafe(&self) -> terrane_int_support::Int {
        self.0.raw_unsafe()
    }
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        unsafe { self.0._terrane_unsafe_726177() }
    }
}
#[derive(Clone)]
pub struct InterfaceCollisionSource {}
impl InterfaceCollisionSource {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn raw_unsafe(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(12_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(13_i128);
    }
}
#[allow(unsafe_code)]
impl InterfaceCollisionProtocol for InterfaceCollisionSource {
    fn clone_box(&self) -> Box<dyn InterfaceCollisionProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn InterfaceCollisionProtocol> {
        Box::new(self.clone())
    }
    fn raw_unsafe(&self) -> terrane_int_support::Int {
        InterfaceCollisionSource::raw_unsafe(&*self)
    }
    unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        unsafe { InterfaceCollisionSource::_terrane_unsafe_726177(&*self) }
    }
}
impl From<InterfaceCollisionSource> for InterfaceCollision {
    fn from(value: InterfaceCollisionSource) -> Self {
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
        "{}", terrane_scalar_support::scalar_text(&unsafe { overridden
        ._terrane_unsafe_726177() })
    );
    let extended: UnsafeAddition = UnsafeAddition::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&extended.raw()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&unsafe { extended
        ._terrane_unsafe_726177() })
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
    let same_owner: SameOwnerCollision = SameOwnerCollision::terrane_construct();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&same_owner.raw_unsafe()),
        terrane_scalar_support::scalar_text(&unsafe { same_owner._terrane_unsafe_726177()
        })
    );
    let inherited: InheritedCollision = InheritedCollision::terrane_construct();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&inherited.raw_unsafe()),
        terrane_scalar_support::scalar_text(&unsafe { inherited._terrane_unsafe_726177()
        })
    );
    let interface_concrete: InterfaceCollisionSource = InterfaceCollisionSource::terrane_construct();
    let interface_value: InterfaceCollision = <InterfaceCollision>::from(
        interface_concrete,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&interface_value.raw_unsafe()),
        terrane_scalar_support::scalar_text(&unsafe { interface_value
        ._terrane_unsafe_726177() })
    );
}
