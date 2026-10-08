// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: unsafe-declaration-contracts
fn choose_terrane_unsafe_declaration_contracts_safe(
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(1_i128);
}
#[allow(unsafe_code)]
#[allow(dead_code)]
unsafe fn choose_terrane_unsafe_declaration_contracts_unsafe(
    value: terrane_int_support::Int,
) -> terrane_int_support::Int {
    return value.clone() + terrane_int_support::Int::from(2_i128);
}
pub trait TerraneNs28UnsafeDeclarationContractsReadableProtocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableProtocol>;
    fn separate_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableProtocol>;
    fn value(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn TerraneNs28UnsafeDeclarationContractsReadableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNs28UnsafeDeclarationContractsReadable(
    Box<dyn TerraneNs28UnsafeDeclarationContractsReadableProtocol>,
);
impl Clone for TerraneNs28UnsafeDeclarationContractsReadable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNs28UnsafeDeclarationContractsReadable {
    pub fn value(&self) -> terrane_int_support::Int {
        self.0.value()
    }
}
#[allow(unsafe_code)]
pub unsafe trait TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol>;
    fn separate_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol>;
    fn value(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<dyn TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNs28UnsafeDeclarationContractsReadableUnsafeContract(
    Box<dyn TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol>,
);
impl Clone for TerraneNs28UnsafeDeclarationContractsReadableUnsafeContract {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNs28UnsafeDeclarationContractsReadableUnsafeContract {
    pub fn value(&self) -> terrane_int_support::Int {
        self.0.value()
    }
}
#[derive(Clone)]
pub struct SafeReader {}
impl SafeReader {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
impl TerraneNs28UnsafeDeclarationContractsReadableProtocol for SafeReader {
    fn clone_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableProtocol> {
        Box::new(self.clone())
    }
    fn value(&self) -> terrane_int_support::Int {
        SafeReader::value(&*self)
    }
}
impl From<SafeReader> for TerraneNs28UnsafeDeclarationContractsReadable {
    fn from(value: SafeReader) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct UnsafeReader {}
impl UnsafeReader {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
#[allow(unsafe_code)]
unsafe impl TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol
for UnsafeReader {
    fn clone_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<dyn TerraneNs28UnsafeDeclarationContractsReadableUnsafeContractProtocol> {
        Box::new(self.clone())
    }
    fn value(&self) -> terrane_int_support::Int {
        UnsafeReader::value(&*self)
    }
}
impl From<UnsafeReader> for TerraneNs28UnsafeDeclarationContractsReadableUnsafeContract {
    fn from(value: UnsafeReader) -> Self {
        Self(Box::new(value))
    }
}
#[allow(unsafe_code)]
pub trait CommandsProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn CommandsProtocol>;
    fn separate_box(&self) -> Box<dyn CommandsProtocol>;
    unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn CommandsProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Commands(Box<dyn CommandsProtocol>);
impl Clone for Commands {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[allow(unsafe_code)]
impl Commands {
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        unsafe { self.0._terrane_unsafe_726177() }
    }
}
#[derive(Clone)]
pub struct Device {}
impl Device {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(4_i128);
    }
}
#[allow(unsafe_code)]
impl CommandsProtocol for Device {
    fn clone_box(&self) -> Box<dyn CommandsProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn CommandsProtocol> {
        Box::new(self.clone())
    }
    unsafe fn _terrane_unsafe_726177(&self) -> terrane_int_support::Int {
        unsafe { Device::_terrane_unsafe_726177(&*self) }
    }
}
impl From<Device> for Commands {
    fn from(value: Device) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct MethodOverloads {}
impl MethodOverloads {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn choose(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(5_i128);
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_63686f6f7365(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(6_i128);
    }
}
#[allow(unsafe_code)]
#[allow(dead_code)]
async unsafe fn later() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
#[allow(unsafe_code)]
fn main() {
    __terrane_run(async move {
        let value: Device;
        let overloads: MethodOverloads;
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&choose_terrane_unsafe_declaration_contracts_safe(terrane_int_support::Int::from(1_i128)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&unsafe {
            choose_terrane_unsafe_declaration_contracts_unsafe(terrane_int_support::Int::from(1_i128))
            })
        );
        value = Device::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&unsafe { value
            ._terrane_unsafe_726177() })
        );
        overloads = MethodOverloads::terrane_construct();
        println!("{}", terrane_scalar_support::scalar_text(&overloads.choose()));
        println!(
            "{}", terrane_scalar_support::scalar_text(&unsafe { overloads
            ._terrane_unsafe_63686f6f7365() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(unsafe { later()
            }). await)
        );
        println!("{}", terrane_scalar_support::scalar_text(&String::from("ok")));
    });
}
