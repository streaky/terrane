// Generated deterministically by Terrane <version>.
// Source: app/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Selected {}
impl Selected {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        return value;
    }
    pub fn echo(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        return value;
    }
    pub fn terrane_static_associated_value() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(16_i128);
    }
}
impl OperationProtocol<terrane_int_support::Int> for Selected {
    fn clone_box(&self) -> Box<dyn OperationProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn OperationProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        Selected::dispatch(&*self, value)
    }
}
impl From<Selected> for Operation<terrane_int_support::Int> {
    fn from(value: Selected) -> Self {
        Self(Box::new(value))
    }
}
fn through_interface(
    value: Operation<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return value.dispatch(terrane_int_support::Int::from(13_i128));
}
fn main() {
    let value: Selected;
    let closed: Operation<terrane_int_support::Int>;
    let generic: Generic<terrane_int_support::Int>;
    value = Selected::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&value
        .dispatch(terrane_int_support::Int::from(11_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&value
        .echo(terrane_int_support::Int::from(12_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&Selected::terrane_static_associated_value())
    );
    closed = <Operation<terrane_int_support::Int>>::from(value);
    println!("{}", terrane_scalar_support::scalar_text(&through_interface(closed)));
    generic = Generic::<
        terrane_int_support::Int,
    >::terrane_construct(terrane_int_support::Int::from(14_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&generic
        .dispatch(terrane_int_support::Int::from(15_i128)))
    );
}
// Source: contract/operation.trn
// Namespace: contract
pub trait OperationProtocol<TerraneType726573706F6E7365>: Send + Sync {
    fn clone_box(&self) -> Box<dyn OperationProtocol<TerraneType726573706F6E7365>>;
    fn separate_box(&self) -> Box<dyn OperationProtocol<TerraneType726573706F6E7365>>;
    fn dispatch(
        &self,
        value: TerraneType726573706F6E7365,
    ) -> TerraneType726573706F6E7365;
}
impl<TerraneType726573706F6E7365> Clone
for Box<dyn OperationProtocol<TerraneType726573706F6E7365>> {
    fn clone(&self) -> Self {
        OperationProtocol::clone_box(&**self)
    }
}
pub struct Operation<TerraneType726573706F6E7365>(
    Box<dyn OperationProtocol<TerraneType726573706F6E7365>>,
);
impl<TerraneType726573706F6E7365> Clone for Operation<TerraneType726573706F6E7365> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType726573706F6E7365> Operation<TerraneType726573706F6E7365> {
    pub fn dispatch(
        &self,
        value: TerraneType726573706F6E7365,
    ) -> TerraneType726573706F6E7365 {
        self.0.dispatch(value)
    }
}
#[derive(Clone)]
pub struct Base {}
impl Base {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        return value;
    }
    pub fn terrane_static_associated_value() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(16_i128);
    }
}
impl OperationProtocol<terrane_int_support::Int> for Base {
    fn clone_box(&self) -> Box<dyn OperationProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn OperationProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        Base::dispatch(&*self, value)
    }
}
impl From<Base> for Operation<terrane_int_support::Int> {
    fn from(value: Base) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Generic<TerraneType54> {
    pub evidence: Option<TerraneType54>,
}
impl<TerraneType54> Generic<TerraneType54> {
    pub fn terrane_construct(evidence: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self { evidence: None };
        __terrane_constructed_value.construct(evidence);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, evidence: TerraneType54) {
        {
            let __terrane_value = evidence;
            self.evidence = Some(__terrane_value);
        }
    }
    pub fn dispatch(&self, value: TerraneType54) -> TerraneType54 {
        return value;
    }
}
impl<TerraneType54> OperationProtocol<TerraneType54> for Generic<TerraneType54>
where
    Generic<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn OperationProtocol<TerraneType54>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn OperationProtocol<TerraneType54>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, value: TerraneType54) -> TerraneType54 {
        Generic::dispatch(&*self, value)
    }
}
impl<TerraneType54> From<Generic<TerraneType54>> for Operation<TerraneType54>
where
    Generic<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn from(value: Generic<TerraneType54>) -> Self {
        Self(Box::new(value))
    }
}
