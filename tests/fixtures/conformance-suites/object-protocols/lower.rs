// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: tests/integration/abstract-category-membership/case.trn
// Namespace: abstract-category-membership
fn test_scenario_terrane_abstract_category_membership() {
    let signed: i8;
    let unsigned: u8;
    let adaptive: i64;
    let decimal: f32;
    signed = -1;
    unsigned = 1;
    adaptive = 1;
    decimal = 1.5_f32;
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &signed; false }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &unsigned; true }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &adaptive; false }));
    println!("{}", terrane_scalar_support::scalar_text(&{ let _ = &decimal; true }));
}
// Source: tests/integration/applied-class-type-control/case.trn
// Namespace: applied-class-type-control
#[derive(Clone)]
pub struct TerraneNsAppliedClassTypeControlBoxIdentity2F6170706C6965642D636C6173732D747970652D636F6E74726F6C00426F78<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsAppliedClassTypeControlBoxIdentity2F6170706C6965642D636C6173732D747970652D636F6E74726F6C00426F78<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
fn identity_terrane_applied_class_type_control<TerraneType54>(
    value: TerraneType54,
) -> TerraneType54 {
    return value;
}
fn test_scenario_terrane_applied_class_type_control() {
    let __trn_626f78: TerraneNsAppliedClassTypeControlBoxIdentity2F6170706C6965642D636C6173732D747970652D636F6E74726F6C00426F78<
        terrane_int_support::Int,
    >;
    let selected: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    __trn_626f78 = TerraneNsAppliedClassTypeControlBoxIdentity2F6170706C6965642D636C6173732D747970652D636F6E74726F6C00426F78::<
        terrane_int_support::Int,
    >::terrane_construct(terrane_int_support::Int::from(1_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__trn_626f78.value.as_ref()
        .expect("required field initialized"))
    );
    selected = std::sync::Arc::new(
        identity_terrane_applied_class_type_control::<terrane_int_support::Int>,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&selected(terrane_int_support::Int::from(2_i128)))
    );
}
// Source: tests/integration/authored-enum-catch-all/case.trn
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
        println!("{}", String::from("cleaned"));
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
fn test_scenario_terrane_authored_enum_catch_all() {
    let payload: Resource;
    let value: State;
    payload = Resource::terrane_construct();
    value = {
        let __terrane_enum_payload_0 = payload;
        State::Held(__terrane_enum_payload_0)
    };
    let __terrane_match_value_278 = value;
    match __terrane_match_value_278 {
        State::Ready => {
            println!("{}", String::from("ready"));
        }
        _ => {
            println!("{}", String::from("other"));
        }
    }
}
// Source: tests/integration/authored-enum-resource-transfer/case.trn
// Namespace: authored-enum-resource-transfer
pub struct TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E {
    pub input: ByteReader,
}
impl TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E {
    pub fn terrane_construct() -> Self {
        Self { input: stdin() }
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("connection closed"));
    }
}
impl Drop
for TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub enum AcquiredConnection {
    Unavailable,
    Available(
        TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E,
    ),
}
fn inspect_terrane_authored_enum_resource_transfer(
    acquired: &AcquiredConnection,
) -> bool {
    match &acquired {
        AcquiredConnection::Unavailable => {
            return false;
        }
        AcquiredConnection::Available(_) => {
            return true;
        }
    }
}
fn extract(
    acquired: AcquiredConnection,
) -> Option<
    TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E,
> {
    let value: TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E;
    let __terrane_match_value_568 = acquired;
    match __terrane_match_value_568 {
        AcquiredConnection::Unavailable => {
            return None;
        }
        AcquiredConnection::Available(__terrane_pattern_666_value) => {
            value = __terrane_pattern_666_value;
            return Some(value);
        }
    }
}
fn release(
    value: Option<
        TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E,
    >,
) {
    let _ = &value;
    return ();
}
fn test_scenario_terrane_authored_enum_resource_transfer() {
    let acquired: AcquiredConnection;
    let result: Option<
        TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E,
    >;
    acquired = {
        let __terrane_enum_payload_0 = TerraneNsAuthoredEnumResourceTransferConnectionIdentity2F617574686F7265642D656E756D2D7265736F757263652D7472616E7366657200636F6E6E656374696F6E::terrane_construct();
        AcquiredConnection::Available(__terrane_enum_payload_0)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&inspect_terrane_authored_enum_resource_transfer(&acquired))
    );
    result = extract(acquired);
    release(result);
}
// Source: tests/integration/authored-generic-class-phantom-parameter/case.trn
// Namespace: authored-generic-class-phantom-parameter
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericClassPhantomParameterHolderIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D6574657200686F6C646572<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub count: terrane_int_support::Int,
}
impl<
    TerraneType54,
> TerraneNsAuthoredGenericClassPhantomParameterHolderIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D6574657200686F6C646572<
    TerraneType54,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericClassPhantomParameterPairIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D657465720070616972<
    TerraneType54,
    TerraneType55,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54, TerraneType55)>,
    pub count: terrane_int_support::Int,
}
impl<
    TerraneType54,
    TerraneType55,
> TerraneNsAuthoredGenericClassPhantomParameterPairIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D657465720070616972<
    TerraneType54,
    TerraneType55,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
fn test_scenario_terrane_authored_generic_class_phantom_parameter() {
    let mut text_holder: TerraneNsAuthoredGenericClassPhantomParameterHolderIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D6574657200686F6C646572<
        String,
    >;
    let number_holder: TerraneNsAuthoredGenericClassPhantomParameterHolderIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D6574657200686F6C646572<
        terrane_int_support::Int,
    >;
    let value: TerraneNsAuthoredGenericClassPhantomParameterPairIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D657465720070616972<
        String,
        terrane_int_support::Int,
    >;
    text_holder = TerraneNsAuthoredGenericClassPhantomParameterHolderIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D6574657200686F6C646572::<
        String,
    >::terrane_construct();
    number_holder = TerraneNsAuthoredGenericClassPhantomParameterHolderIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D6574657200686F6C646572::<
        terrane_int_support::Int,
    >::terrane_construct();
    {
        let __terrane_value = terrane_int_support::Int::from(7_i128);
        text_holder.count = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&text_holder.count));
    println!("{}", terrane_scalar_support::scalar_text(&number_holder.count));
    value = TerraneNsAuthoredGenericClassPhantomParameterPairIdentity2F617574686F7265642D67656E657269632D636C6173732D7068616E746F6D2D706172616D657465720070616972::<
        String,
        terrane_int_support::Int,
    >::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.count));
}
// Source: tests/integration/authored-generic-contracts/case.trn
// Namespace: authored-generic-contracts
#[derive(Clone)]
pub struct Envelope<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub payload: Option<TerraneType54>,
}
impl<TerraneType54> Envelope<TerraneType54> {
    pub fn terrane_construct(payload: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            payload: None,
        };
        __terrane_constructed_value.construct(payload);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, payload: TerraneType54) {
        {
            let __terrane_value = payload;
            self.payload = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericContractsSlotIdentity2F617574686F7265642D67656E657269632D636F6E74726163747300736C6F74<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsAuthoredGenericContractsSlotIdentity2F617574686F7265642D67656E657269632D636F6E74726163747300736C6F74<
    TerraneType54,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        }
    }
    pub fn set(&mut self, value: TerraneType54) {
        {
            let __terrane_value = Some(value);
            self.value = __terrane_value;
        }
    }
}
fn identity_terrane_authored_generic_contracts<TerraneType54>(
    value: TerraneType54,
) -> TerraneType54 {
    return value;
}
fn choose_terrane_authored_generic_contracts<TerraneType54>(
    first: TerraneType54,
    second: TerraneType54,
    use_first: bool,
) -> TerraneType54 {
    if use_first {
        return first;
    }
    return second;
}
fn apply_terrane_authored_generic_contracts<TerraneType54, TerraneType55>(
    value: TerraneType54,
    transform: std::sync::Arc<dyn Fn(TerraneType54) -> TerraneType55 + Send + Sync>,
) -> TerraneType55 {
    return transform(value);
}
fn label(value: terrane_int_support::Int) -> String {
    if value == terrane_int_support::Int::from(41_i128) {
        return String::from("forty-one");
    }
    return String::from("other");
}
fn test_scenario_terrane_authored_generic_contracts() {
    let number_envelope: Envelope<terrane_int_support::Int>;
    let text_envelope: Envelope<String>;
    let selected: terrane_int_support::Int;
    let transformed: String;
    let keep_int: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let mut pending: TerraneNsAuthoredGenericContractsSlotIdentity2F617574686F7265642D67656E657269632D636F6E74726163747300736C6F74<
        String,
    >;
    let value: Option<String>;
    number_envelope = Envelope::<
        terrane_int_support::Int,
    >::terrane_construct(terrane_int_support::Int::from(41_i128));
    text_envelope = Envelope::<String>::terrane_construct(String::from("hello"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&number_envelope.payload.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&text_envelope.payload.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&identity_terrane_authored_generic_contracts::
        < terrane_int_support::Int > (terrane_int_support::Int::from(42_i128)))
    );
    selected = choose_terrane_authored_generic_contracts(
        terrane_int_support::Int::from(11_i128),
        terrane_int_support::Int::from(23_i128),
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&selected));
    transformed = apply_terrane_authored_generic_contracts(
        terrane_int_support::Int::from(41_i128),
        std::sync::Arc::new(label),
    );
    println!("{}", terrane_scalar_support::scalar_text(&transformed));
    keep_int = std::sync::Arc::new(
        identity_terrane_authored_generic_contracts::<terrane_int_support::Int>,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&keep_int(terrane_int_support::Int::from(44_i128)))
    );
    pending = TerraneNsAuthoredGenericContractsSlotIdentity2F617574686F7265642D67656E657269632D636F6E74726163747300736C6F74::<
        String,
    >::terrane_construct();
    pending.set(String::from("ready"));
    value = pending.value;
    if (&value).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &value { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
// Source: tests/integration/authored-generic-enum-states/case.trn
// Namespace: authored-generic-enum-states
#[derive(Clone)]
pub enum OperationState<TerraneType54> {
    Pending,
    Running(terrane_int_support::Int),
    Completed(TerraneType54),
    Failed(String),
}
fn report(state: OperationState<String>) -> String {
    let progress: &terrane_int_support::Int;
    let result: &String;
    let error: &String;
    match &state {
        OperationState::Pending => {
            return String::from("pending");
        }
        OperationState::Running(__terrane_pattern_307_progress) => {
            progress = __terrane_pattern_307_progress;
            println!("{}", terrane_scalar_support::scalar_text(&* progress));
            return String::from("running");
        }
        OperationState::Completed(__terrane_pattern_412_result) => {
            result = __terrane_pattern_412_result;
            return result.clone();
        }
        OperationState::Failed(__terrane_pattern_486_error) => {
            error = __terrane_pattern_486_error;
            return error.clone();
        }
    }
}
fn optional_report(state: Option<OperationState<String>>) -> String {
    let result: &String;
    let error: &String;
    match &state {
        Some(OperationState::Pending) => {
            return String::from("pending");
        }
        Some(OperationState::Running(_)) => {
            return String::from("running");
        }
        Some(OperationState::Completed(__terrane_pattern_785_result)) => {
            result = __terrane_pattern_785_result;
            return result.clone();
        }
        Some(OperationState::Failed(__terrane_pattern_859_error)) => {
            error = __terrane_pattern_859_error;
            return error.clone();
        }
        None => {
            return String::from("absent");
        }
    }
}
#[derive(Clone)]
pub enum Decision {
    Accepted(String),
    Rejected(String),
}
fn explain(choice: Decision) -> String {
    let explanation: String;
    let explanation_2: String;
    let __terrane_match_value_1093 = choice.clone();
    match __terrane_match_value_1093 {
        Decision::Accepted(__terrane_pattern_1114_explanation) => {
            explanation = __terrane_pattern_1114_explanation;
            return explanation;
        }
        Decision::Rejected(__terrane_pattern_1190_explanation) => {
            explanation_2 = __terrane_pattern_1190_explanation;
            return explanation_2;
        }
    }
}
fn test_scenario_terrane_authored_generic_enum_states() {
    let pending: OperationState<String>;
    let running: OperationState<String>;
    let completed: OperationState<String>;
    let failed: OperationState<String>;
    let accepted: Decision;
    let rejected: Decision;
    pending = OperationState::<String>::Pending;
    running = {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(7_i128);
        OperationState::<String>::Running(__terrane_enum_payload_0)
    };
    completed = {
        let __terrane_enum_payload_0 = String::from("done");
        OperationState::<String>::Completed(__terrane_enum_payload_0)
    };
    failed = {
        let __terrane_enum_payload_0 = String::from("failed");
        OperationState::<String>::Failed(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&report(pending)));
    println!("{}", terrane_scalar_support::scalar_text(&report(running)));
    println!("{}", terrane_scalar_support::scalar_text(&report(completed)));
    println!("{}", terrane_scalar_support::scalar_text(&report(failed)));
    println!("{}", terrane_scalar_support::scalar_text(&optional_report(None)));
    accepted = {
        let __terrane_enum_payload_0 = String::from("accepted");
        Decision::Accepted(__terrane_enum_payload_0)
    };
    rejected = {
        let __terrane_enum_payload_0 = String::from("rejected");
        Decision::Rejected(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&explain(accepted)));
    println!("{}", terrane_scalar_support::scalar_text(&explain(rejected)));
}
// Source: tests/integration/authored-generic-finite-recursion/case.trn
// Namespace: authored-generic-finite-recursion
fn count_down<TerraneType54>(
    value: TerraneType54,
    count: terrane_int_support::Int,
) -> terrane_int_support::Int {
    if count == terrane_int_support::Int::from(0_i128) {
        return terrane_int_support::Int::from(0_i128);
    }
    return count_down(value, count.clone() - terrane_int_support::Int::from(1_i128));
}
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78<
    TerraneType54,
> {
    pub fn terrane_construct(input: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(input);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, input: TerraneType54) {
        {
            let __terrane_value = input;
            self.value = Some(__terrane_value);
        }
    }
    pub fn count_down(
        &self,
        count: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        if count == terrane_int_support::Int::from(0_i128) {
            return terrane_int_support::Int::from(0_i128);
        }
        return self.count_down(count.clone() - terrane_int_support::Int::from(1_i128));
    }
}
fn test_scenario_terrane_authored_generic_finite_recursion() {
    let boxed: TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78<
        String,
    >;
    println!(
        "{}", terrane_scalar_support::scalar_text(&count_down:: < String >
        (String::from("same"), terrane_int_support::Int::from(3_i128)))
    );
    boxed = TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78::<
        String,
    >::terrane_construct(String::from("same"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&boxed
        .count_down(terrane_int_support::Int::from(3_i128)))
    );
}
// Source: tests/integration/authored-generic-interfaces/case.trn
// Namespace: authored-generic-interfaces
pub trait TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
    >;
    fn describe(&self) -> String;
}
impl Clone
for Box<
    dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65(
    Box<
        dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
    >,
);
impl Clone
for TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65 {
    pub fn describe(&self) -> String {
        self.0.describe()
    }
}
#[derive(Clone)]
pub struct Message {
    pub text: String,
}
impl Message {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
    pub fn describe(&self) -> String {
        return self.text.clone();
    }
}
impl TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol
for Message {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn describe(&self) -> String {
        Message::describe(&*self)
    }
}
impl From<Message>
for TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65 {
    fn from(value: Message) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Address {
    pub port: terrane_int_support::Int,
}
impl Address {
    pub fn terrane_construct() -> Self {
        Self {
            port: terrane_int_support::Int::from(80_i128),
        }
    }
    pub fn describe(&self) -> String {
        return String::from("peer");
    }
}
impl TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol
for Address {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn describe(&self) -> String {
        Address::describe(&*self)
    }
}
impl From<Address>
for TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65 {
    fn from(value: Address) -> Self {
        Self(Box::new(value))
    }
}
fn choose_terrane_authored_generic_interfaces<TerraneType54>(
    first: TerraneType54,
    second: TerraneType54,
    use_first: bool,
) -> TerraneType54 {
    if use_first {
        return first;
    }
    return second;
}
fn preserve<
    TerraneType54: TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
>(value: TerraneType54) -> TerraneType54 {
    return value;
}
fn describe_value<
    TerraneType54: TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65Protocol,
>(value: TerraneType54) -> String {
    return value.describe();
}
pub trait SourceProtocol<TerraneType54>: Send + Sync {
    fn clone_box(&self) -> Box<dyn SourceProtocol<TerraneType54>>;
    fn separate_box(&self) -> Box<dyn SourceProtocol<TerraneType54>>;
    fn next(&mut self) -> Option<TerraneType54>;
}
impl<TerraneType54> Clone for Box<dyn SourceProtocol<TerraneType54>> {
    fn clone(&self) -> Self {
        SourceProtocol::clone_box(&**self)
    }
}
pub struct Source<TerraneType54>(Box<dyn SourceProtocol<TerraneType54>>);
impl<TerraneType54> Clone for Source<TerraneType54> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType54> Source<TerraneType54> {
    pub fn next(&mut self) -> Option<TerraneType54> {
        self.0.next()
    }
}
#[derive(Clone)]
pub struct MessageSource {
    pub value: Option<String>,
}
impl MessageSource {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("first")),
        }
    }
    pub fn next(&mut self) -> Option<String> {
        let result: Option<String>;
        result = self.value.clone();
        {
            let __terrane_value = None;
            self.value = __terrane_value;
        }
        return result;
    }
}
impl SourceProtocol<String> for MessageSource {
    fn clone_box(&self) -> Box<dyn SourceProtocol<String>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SourceProtocol<String>> {
        Box::new(self.clone())
    }
    fn next(&mut self) -> Option<String> {
        MessageSource::next(&mut *self)
    }
}
impl From<MessageSource> for Source<String> {
    fn from(value: MessageSource) -> Self {
        Self(Box::new(value))
    }
}
fn next_string(mut input: Source<String>) -> Option<String> {
    return input.next();
}
fn test_scenario_terrane_authored_generic_interfaces() {
    let message: Message;
    let kept: Message;
    let mut input: Source<String>;
    let first: Option<String>;
    let second: Option<String>;
    let peer: Address;
    let erased: TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65;
    message = Message::terrane_construct(String::from("preserved"));
    kept = preserve(message.clone());
    println!("{}", terrane_scalar_support::scalar_text(&kept.describe()));
    println!("{}", terrane_scalar_support::scalar_text(&describe_value(kept)));
    input = <Source<String>>::from(MessageSource::terrane_construct());
    first = next_string(input.clone());
    if (&first).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &first { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    input.next();
    second = input.next();
    if (&second).is_none() {
        println!("{}", String::from("none"));
    }
    peer = Address::terrane_construct();
    erased = choose_terrane_authored_generic_interfaces::<
        TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65,
    >(
        <TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65>::from(
            message,
        ),
        <TerraneNsAuthoredGenericInterfacesDescribableIdentity2F617574686F7265642D67656E657269632D696E7465726661636573006465736372696261626C65>::from(
            peer,
        ),
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&erased.describe()));
}
// Source: tests/integration/authored-generic-partial-construction/case.trn
// Namespace: authored-generic-partial-construction
pub struct TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E {
    pub input: ByteReader,
    pub tag: String,
}
impl TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E {
    pub fn terrane_construct(tag: String) -> Self {
        let mut __terrane_constructed_value = Self {
            input: stdin(),
            tag: String::from(""),
        };
        __terrane_constructed_value.construct(tag);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, tag: String) {
        {
            let __terrane_value = tag;
            self.tag = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!("{}", terrane_scalar_support::scalar_text(&self.tag));
    }
}
impl Drop
for TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E {
    fn drop(&mut self) {
        self.destruct();
    }
}
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericPartialConstructionPairIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E0070616972<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub first: Option<TerraneType54>,
    pub second: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsAuthoredGenericPartialConstructionPairIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E0070616972<
    TerraneType54,
> {
    pub fn terrane_construct(
        first: TerraneType54,
        second: TerraneType54,
        fail: bool,
    ) -> Result<Self, TerraneError> {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            first: None,
            second: None,
        };
        __terrane_constructed_value.construct(first, second, fail)?;
        Ok(__terrane_constructed_value)
    }
    pub fn construct(
        &mut self,
        first: TerraneType54,
        second: TerraneType54,
        fail: bool,
    ) -> Result<(), TerraneError> {
        {
            let __terrane_value = first;
            self.first = Some(__terrane_value);
        }
        if fail {
            return Err(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,

                        0 /* terrane-site: tests/integration/authored-generic-partial-construction/case.trn:19:13-19:33 */,
                ),
            );
        }
        {
            let __terrane_value = second;
            self.second = Some(__terrane_value);
        }
        Ok(())
    }
}
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78 {
    pub fn terrane_construct(
        value: terrane_int_support::Int,
        __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value
            .construct(
                value,
                __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565,
            );
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        value: terrane_int_support::Int,
        __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = value.clone()
                + __trn_5f5f74657272616e655f636f6e73747275637465645f76616c7565.clone();
            self.value = __terrane_value;
        }
    }
}
fn test_scenario_terrane_authored_generic_partial_construction() {
    let __trn_626f78: TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78;
    let first: TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E;
    let second: TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E;
    let mut value: Option<
        TerraneNsAuthoredGenericPartialConstructionPairIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E0070616972<
            TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E,
        >,
    > = None;
    __trn_626f78 = TerraneNsAuthoredGenericPartialConstructionBoxIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00626F78::terrane_construct(
        terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(22_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(&__trn_626f78.value));
    first = TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E::terrane_construct(
        String::from("initialized closed"),
    );
    second = TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E::terrane_construct(
        String::from("unassigned closed"),
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = value
                .insert(
                    __terrane_traced_completion!(
                        TerraneNsAuthoredGenericPartialConstructionPairIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E0070616972::<
                        TerraneNsAuthoredGenericPartialConstructionConnectionIdentity2F617574686F7265642D67656E657269632D7061727469616C2D636F6E737472756374696F6E00636F6E6E656374696F6E
                        >::terrane_construct(first, second, true), 1 /* terrane-site: tests/integration/authored-generic-partial-construction/case.trn:33:17-33:61 */
                    ),
                );
            let _ = &value;
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("construction failed"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/authored-generic-resource-choice/case.trn
// Namespace: authored-generic-resource-choice
pub struct TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E {
    pub input: ByteReader,
    pub tag: String,
}
impl TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E {
    pub fn terrane_construct(tag: String) -> Self {
        let mut __terrane_constructed_value = Self {
            input: stdin(),
            tag: String::from(""),
        };
        __terrane_constructed_value.construct(tag);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, tag: String) {
        {
            let __terrane_value = tag;
            self.tag = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!("{}", terrane_scalar_support::scalar_text(&self.tag));
    }
}
impl Drop
for TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E {
    fn drop(&mut self) {
        self.destruct();
    }
}
fn choose_terrane_authored_generic_resource_choice<TerraneType54>(
    first: TerraneType54,
    second: TerraneType54,
    use_first: bool,
) -> TerraneType54 {
    if use_first {
        return first;
    }
    return second;
}
fn test_scenario_terrane_authored_generic_resource_choice() {
    let first: TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E;
    let second: TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E;
    let selected: TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E;
    first = TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E::terrane_construct(
        String::from("first closed"),
    );
    second = TerraneNsAuthoredGenericResourceChoiceConnectionIdentity2F617574686F7265642D67656E657269632D7265736F757263652D63686F69636500636F6E6E656374696F6E::terrane_construct(
        String::from("second closed"),
    );
    selected = choose_terrane_authored_generic_resource_choice(first, second, true);
    println!("{}", terrane_scalar_support::scalar_text(&selected.tag));
}
// Source: tests/integration/authored-generic-selection/case.trn
// Namespace: authored-generic-selection
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericSelectionSlotIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E00736C6F74<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsAuthoredGenericSelectionSlotIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E00736C6F74<
    TerraneType54,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        }
    }
}
fn identity_terrane_authored_generic_selection<TerraneType54>(
    value: TerraneType54,
) -> TerraneType54 {
    return value;
}
fn absent<TerraneType54>() -> Option<TerraneType54> {
    return None;
}
fn apply_terrane_authored_generic_selection<TerraneType54, TerraneType55>(
    value: TerraneType54,
    transform: std::sync::Arc<dyn Fn(TerraneType54) -> TerraneType55 + Send + Sync>,
) -> TerraneType55 {
    return transform(value);
}
fn render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("mapped");
}
#[derive(Clone)]
pub enum TerraneNsAuthoredGenericSelectionPairIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E0070616972 {
    Values(terrane_int_support::Int, terrane_int_support::Int),
}
fn first_terrane_authored_generic_selection() -> terrane_int_support::Int {
    println!("{}", String::from("first"));
    return terrane_int_support::Int::from(11_i128);
}
fn second() -> terrane_int_support::Int {
    println!("{}", String::from("second"));
    return terrane_int_support::Int::from(23_i128);
}
fn test_scenario_terrane_authored_generic_selection() {
    let empty: TerraneNsAuthoredGenericSelectionSlotIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E00736C6F74<
        String,
    >;
    let missing: Option<terrane_int_support::Int>;
    let keep: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let state: TerraneNsAuthoredGenericSelectionPairIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E0070616972;
    let left: terrane_int_support::Int;
    let right: terrane_int_support::Int;
    empty = TerraneNsAuthoredGenericSelectionSlotIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E00736C6F74::<
        String,
    >::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&(&empty.value).is_none()));
    missing = absent();
    println!("{}", terrane_scalar_support::scalar_text(&(&missing).is_none()));
    keep = std::sync::Arc::new(identity_terrane_authored_generic_selection);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&keep(terrane_int_support::Int::from(44_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&apply_terrane_authored_generic_selection(terrane_int_support::Int::from(4_i128),
        std::sync::Arc::new(render)))
    );
    state = {
        let __terrane_enum_payload_0 = second();
        let __terrane_enum_payload_1 = first_terrane_authored_generic_selection();
        TerraneNsAuthoredGenericSelectionPairIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E0070616972::Values(
            __terrane_enum_payload_1,
            __terrane_enum_payload_0,
        )
    };
    let __terrane_match_value_822 = state.clone();
    match __terrane_match_value_822 {
        TerraneNsAuthoredGenericSelectionPairIdentity2F617574686F7265642D67656E657269632D73656C656374696F6E0070616972::Values(
            __terrane_pattern_842_left,
            __terrane_pattern_842_right,
        ) => {
            left = __terrane_pattern_842_left;
            right = __terrane_pattern_842_right;
            println!(
                "{}", terrane_scalar_support::scalar_text(&(left.clone() + right
                .clone()))
            );
        }
    }
}
// Source: tests/integration/authored-interface-transfer-composition/case.trn
// Namespace: authored-interface-transfer-composition
pub trait TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    >;
    fn read(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65(
    Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    >,
);
impl Clone
for TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65 {
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
}
#[derive(Clone)]
pub struct TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(3_i128),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
}
impl TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol
for TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn read(&self) -> terrane_int_support::Int {
        TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166::read(
            &*self,
        )
    }
}
impl From<
    TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166,
>
for TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65 {
    fn from(
        value: TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct DirectComposite {
    pub inner: TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65,
}
impl DirectComposite {
    pub fn terrane_construct() -> Self {
        Self {
            inner: <TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65>::from(
                TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166::terrane_construct(),
            ),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.inner.read();
    }
}
impl TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol
for DirectComposite {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn read(&self) -> terrane_int_support::Int {
        DirectComposite::read(&*self)
    }
}
impl From<DirectComposite>
for TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65 {
    fn from(value: DirectComposite) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsAuthoredInterfaceTransferCompositionHolderIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E00686F6C646572 {
    pub inner: TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65,
}
impl TerraneNsAuthoredInterfaceTransferCompositionHolderIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E00686F6C646572 {
    pub fn terrane_construct() -> Self {
        Self {
            inner: <TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65>::from(
                TerraneNsAuthoredInterfaceTransferCompositionLeafIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E006C656166::terrane_construct(),
            ),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.inner.read();
    }
}
#[derive(Clone)]
pub struct NestedComposite {
    pub inner: TerraneNsAuthoredInterfaceTransferCompositionHolderIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E00686F6C646572,
}
impl NestedComposite {
    pub fn terrane_construct() -> Self {
        Self {
            inner: TerraneNsAuthoredInterfaceTransferCompositionHolderIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E00686F6C646572::terrane_construct(),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.inner.read();
    }
}
impl TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol
for NestedComposite {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn read(&self) -> terrane_int_support::Int {
        NestedComposite::read(&*self)
    }
}
impl From<NestedComposite>
for TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65 {
    fn from(value: NestedComposite) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_authored_interface_transfer_composition() {
    let direct: TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65;
    let nested: TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65;
    direct = <TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65>::from(
        DirectComposite::terrane_construct(),
    );
    nested = <TerraneNsAuthoredInterfaceTransferCompositionReadableIdentity2F617574686F7265642D696E746572666163652D7472616E736665722D636F6D706F736974696F6E007265616461626C65>::from(
        NestedComposite::terrane_construct(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&direct.read()),
        terrane_scalar_support::scalar_text(&nested.read())
    );
}
// Source: tests/integration/borrowed-inherited-copy-fields/case.trn
// Namespace: borrowed-inherited-copy-fields
#[derive(Clone)]
pub struct TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78Storage {
    pub flag: bool,
    pub count: i32,
    pub ratio: f64,
}
impl TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78Storage {
    pub fn terrane_construct() -> Self {
        Self {
            flag: true,
            count: 3,
            ratio: 1.5,
        }
    }
}
#[derive(Clone)]
pub enum TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78 {
    Own(
        TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78Storage,
    ),
    TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
        TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78,
    ),
}
impl TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78Storage::terrane_construct(),
        )
    }
    pub fn terrane_field_flag(&self) -> &bool {
        match self {
            Self::Own(value) => &value.flag,
            Self::TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
                value,
            ) => &value.flag,
        }
    }
    pub fn terrane_field_flag_mut(&mut self) -> &mut bool {
        match self {
            Self::Own(value) => &mut value.flag,
            Self::TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
                value,
            ) => &mut value.flag,
        }
    }
    pub fn terrane_field_count(&self) -> &i32 {
        match self {
            Self::Own(value) => &value.count,
            Self::TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
                value,
            ) => &value.count,
        }
    }
    pub fn terrane_field_count_mut(&mut self) -> &mut i32 {
        match self {
            Self::Own(value) => &mut value.count,
            Self::TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
                value,
            ) => &mut value.count,
        }
    }
    pub fn terrane_field_ratio(&self) -> &f64 {
        match self {
            Self::Own(value) => &value.ratio,
            Self::TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
                value,
            ) => &value.ratio,
        }
    }
    pub fn terrane_field_ratio_mut(&mut self) -> &mut f64 {
        match self {
            Self::Own(value) => &mut value.ratio,
            Self::TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
                value,
            ) => &mut value.ratio,
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78 {
    pub flag: bool,
    pub count: i32,
    pub ratio: f64,
    pub extra: terrane_int_support::Int,
}
impl TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            flag: true,
            count: 3,
            ratio: 1.5,
            extra: terrane_int_support::Int::from(1_i128),
        }
    }
}
fn make_derived() -> TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78 {
    let mut value: TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78;
    value = TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78::terrane_construct();
    {
        let __terrane_value = false;
        value.flag = __terrane_value;
    }
    {
        let __terrane_value = 9;
        value.count = __terrane_value;
    }
    {
        let __terrane_value = 2.5;
        value.ratio = __terrane_value;
    }
    return TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78::TerraneNsBorrowedInheritedCopyFieldsDerivedBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300646572697665642D626F78(
        value,
    );
}
fn read_flag(
    value: &TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78,
) -> bool {
    return {
        let __terrane_snapshot = *value.terrane_field_flag();
        __terrane_snapshot
    };
}
fn read_count(
    value: &TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78,
) -> i32 {
    return {
        let __terrane_snapshot = *value.terrane_field_count();
        __terrane_snapshot
    };
}
fn read_ratio(
    value: &TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78,
) -> f64 {
    return {
        let __terrane_snapshot = *value.terrane_field_ratio();
        __terrane_snapshot
    };
}
fn show_copy(flag: bool, count: i32, ratio: f64) {
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&flag),
        terrane_scalar_support::scalar_text(&count),
        terrane_scalar_support::scalar_text(&ratio)
    );
}
fn check_borrow(
    value: &TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78,
) {
    let flag: bool;
    let count: i32;
    let ratio: f64;
    println!("{}", terrane_scalar_support::scalar_text(&read_flag(value)));
    println!("{}", terrane_scalar_support::scalar_text(&read_count(value)));
    println!("{}", terrane_scalar_support::scalar_text(&read_ratio(value)));
    flag = {
        let __terrane_snapshot = *value.terrane_field_flag();
        __terrane_snapshot
    };
    count = {
        let __terrane_snapshot = *value.terrane_field_count();
        __terrane_snapshot
    };
    ratio = {
        let __terrane_snapshot = *value.terrane_field_ratio();
        __terrane_snapshot
    };
    show_copy(flag, count, ratio);
    show_copy(
        {
            let __terrane_snapshot = *value.terrane_field_flag();
            __terrane_snapshot
        },
        {
            let __terrane_snapshot = *value.terrane_field_count();
            __terrane_snapshot
        },
        {
            let __terrane_snapshot = *value.terrane_field_ratio();
            __terrane_snapshot
        },
    );
}
fn check_shared(
    value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78,
        >,
    >,
) {
    let flag: bool;
    let count: i32;
    let ratio: f64;
    flag = {
        let __terrane_snapshot = *value
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_flag();
        __terrane_snapshot
    };
    count = {
        let __terrane_snapshot = *value
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_count();
        __terrane_snapshot
    };
    ratio = {
        let __terrane_snapshot = *value
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_ratio();
        __terrane_snapshot
    };
    show_copy(flag, count, ratio);
    show_copy(
        {
            let __terrane_snapshot = *value
                .lock()
                .expect("shared reference lock poisoned")
                .terrane_field_flag();
            __terrane_snapshot
        },
        count,
        ratio,
    );
    show_copy(
        flag,
        {
            let __terrane_snapshot = *value
                .lock()
                .expect("shared reference lock poisoned")
                .terrane_field_count();
            __terrane_snapshot
        },
        ratio,
    );
    show_copy(
        flag,
        count,
        {
            let __terrane_snapshot = *value
                .lock()
                .expect("shared reference lock poisoned")
                .terrane_field_ratio();
            __terrane_snapshot
        },
    );
}
fn test_scenario_terrane_borrowed_inherited_copy_fields() {
    let base: TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78;
    let derived: TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78;
    let shared_base: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78,
        >,
    >;
    let shared_derived: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78,
        >,
    >;
    base = TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78::terrane_construct();
    check_borrow(&base);
    derived = make_derived();
    check_borrow(&derived);
    shared_base = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsBorrowedInheritedCopyFieldsBaseBoxIdentity2F626F72726F7765642D696E686572697465642D636F70792D6669656C647300626173652D626F78::terrane_construct(),
        ),
    );
    check_shared(shared_base.clone());
    shared_derived = std::sync::Arc::new(TerraneSharedCell::new(make_derived()));
    check_shared(shared_derived.clone());
}
// Source: tests/integration/bounded-borrowed-collections/case.trn
// Namespace: bounded-borrowed-collections
pub trait TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564Protocol: Send + Sync {
    fn name(&self) -> terrane_int_support::Int;
    fn finish(self: Box<Self>) -> terrane_int_support::Int;
}
pub struct TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564(
    Box<
        dyn TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564Protocol,
    >,
);
impl TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(7_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564Protocol
for TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D {
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D::name(
            &*self,
        )
    }
    fn finish(self: Box<Self>) -> terrane_int_support::Int {
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D::finish(
            *self,
        )
    }
}
impl From<
    TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
>
for TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564 {
    fn from(
        value: TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn observe_list<
    TerraneType54: TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564Protocol,
>(values: &terrane_collection_support::List<TerraneType54>) {
    let mut value: &TerraneType54;
    let __terrane_iterable_0 = values;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value.name()));
    }
}
fn observe_map<
    TerraneType54: TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564Protocol,
>(
    values: terrane_collection_support::Map<String, TerraneType54>,
) -> terrane_int_support::Int {
    return {
        let __terrane_map_key_0 = String::from("item");
        let __terrane_index_place_1 = __terrane_raised(
            values
                .get(&__terrane_map_key_0)
                .ok_or(terrane_collection_support::MissingKey),

                2 /* terrane-site: tests/integration/bounded-borrowed-collections/case.trn:16:12-16:26 */,
        );
        __terrane_index_place_1.name()
    };
}
fn observe_unordered<
    TerraneType54: TerraneNsBoundedBorrowedCollectionsNamedIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73006E616D6564Protocol,
>(
    values: &terrane_collection_support::UnorderedMap<String, TerraneType54>,
) -> terrane_int_support::Int {
    return {
        let __terrane_map_key_2 = String::from("item");
        let __terrane_index_place_3 = __terrane_raised(
            values
                .get(&__terrane_map_key_2)
                .ok_or(terrane_collection_support::MissingKey),

                3 /* terrane-site: tests/integration/bounded-borrowed-collections/case.trn:18:12-18:26 */,
        );
        __terrane_index_place_3.name()
    };
}
fn test_scenario_terrane_bounded_borrowed_collections() {
    let values: terrane_collection_support::List<
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
    >;
    let entries: terrane_collection_support::Map<
        String,
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
    >;
    let entries2: terrane_collection_support::UnorderedMap<
        String,
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
    >;
    values = terrane_collection_support::List::<
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
    >::new(
        vec![
            TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D::terrane_construct()
        ],
    );
    observe_list(&values);
    entries = terrane_collection_support::Map::<
        String,
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D::terrane_construct())
        ],
    );
    println!("{}", terrane_scalar_support::scalar_text(&observe_map(entries)));
    entries2 = terrane_collection_support::UnorderedMap::<
        String,
        TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("item"),
            TerraneNsBoundedBorrowedCollectionsItemIdentity2F626F756E6465642D626F72726F7765642D636F6C6C656374696F6E73004974656D::terrane_construct())
        ],
    );
    println!("{}", terrane_scalar_support::scalar_text(&observe_unordered(&entries2)));
}
// Source: tests/integration/bounded-consuming-generic-field/case.trn
// Namespace: bounded-consuming-generic-field
pub trait TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564Protocol: Send + Sync {
    fn name(&self) -> terrane_int_support::Int;
    fn finish(self: Box<Self>) -> terrane_int_support::Int;
}
pub struct TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564(
    Box<
        dyn TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564Protocol,
    >,
);
impl TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(7_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564Protocol
for TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D {
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D::name(
            &*self,
        )
    }
    fn finish(self: Box<Self>) -> terrane_int_support::Int {
        TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D::finish(
            *self,
        )
    }
}
impl From<
    TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D,
>
for TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564 {
    fn from(
        value: TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedConsumingGenericFieldBoxIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C6400426F78<
    TerraneType54: TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
    pub other: terrane_int_support::Int,
}
impl<
    TerraneType54: TerraneNsBoundedConsumingGenericFieldNamedIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64006E616D6564Protocol,
> TerraneNsBoundedConsumingGenericFieldBoxIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C6400426F78<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
            other: terrane_int_support::Int::from(9_i128),
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
    pub fn finish(mut self) -> terrane_int_support::Int {
        let result: terrane_int_support::Int;
        result = Box::new(self.value.take().expect("required field initialized"))
            .finish();
        println!("{}", terrane_scalar_support::scalar_text(&self.other));
        return result.clone();
    }
}
fn test_scenario_terrane_bounded_consuming_generic_field() {
    let __trn_626f78: TerraneNsBoundedConsumingGenericFieldBoxIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C6400426F78<
        TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D,
    >;
    __trn_626f78 = TerraneNsBoundedConsumingGenericFieldBoxIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C6400426F78::<
        TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D,
    >::terrane_construct(
        TerraneNsBoundedConsumingGenericFieldItemIdentity2F626F756E6465642D636F6E73756D696E672D67656E657269632D6669656C64004974656D::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&__trn_626f78.finish()));
}
// Source: tests/integration/bounded-generic-bound-forwarding/case.trn
// Namespace: bounded-generic-bound-forwarding
pub trait TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564(
    Box<
        dyn TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
pub trait OtherProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn OtherProtocol>;
    fn separate_box(&self) -> Box<dyn OtherProtocol>;
    fn other(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn OtherProtocol> {
    fn clone(&self) -> Self {
        OtherProtocol::clone_box(&**self)
    }
}
pub struct Other(Box<dyn OtherProtocol>);
impl Clone for Other {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Other {
    pub fn other(&self) -> terrane_int_support::Int {
        self.0.other()
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565 {}
impl TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
}
impl TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol
for TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565,
>
for TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564 {
    fn from(
        value: TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Unrelated {}
impl Unrelated {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn other(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
}
impl OtherProtocol for Unrelated {
    fn clone_box(&self) -> Box<dyn OtherProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn OtherProtocol> {
        Box::new(self.clone())
    }
    fn other(&self) -> terrane_int_support::Int {
        Unrelated::other(&*self)
    }
}
impl From<Unrelated> for Other {
    fn from(value: Unrelated) -> Self {
        Self(Box::new(value))
    }
}
fn target_terrane_bounded_generic_bound_forwarding<
    TerraneType54: TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    return item.name();
}
fn inferred_forward<
    TerraneType55: TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
>(item: TerraneType55) -> terrane_int_support::Int {
    return target_terrane_bounded_generic_bound_forwarding(item);
}
fn explicit_forward<
    TerraneType55: TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
>(item: TerraneType55) -> terrane_int_support::Int {
    return target_terrane_bounded_generic_bound_forwarding::<TerraneType55>(item);
}
#[derive(Clone)]
pub struct GenericHolder<
    TerraneType54: TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub item: Option<TerraneType54>,
}
impl<
    TerraneType54: TerraneNsBoundedGenericBoundForwardingNamedIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E67006E616D6564Protocol,
> GenericHolder<TerraneType54> {
    pub fn terrane_construct(item: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            item: None,
        };
        __terrane_constructed_value.construct(item);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, item: TerraneType54) {
        {
            let __terrane_value = item;
            self.item = Some(__terrane_value);
        }
    }
    pub fn class_forward(mut self) -> terrane_int_support::Int {
        return target_terrane_bounded_generic_bound_forwarding(
            self.item.take().expect("required field initialized"),
        );
    }
}
fn test_scenario_terrane_bounded_generic_bound_forwarding() {
    let mut item: TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565;
    let holder: GenericHolder<
        TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565,
    >;
    item = TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&inferred_forward(item)));
    item = TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&explicit_forward(item)));
    holder = GenericHolder::<
        TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565,
    >::terrane_construct(
        TerraneNsBoundedGenericBoundForwardingValueIdentity2F626F756E6465642D67656E657269632D626F756E642D666F7277617264696E670076616C7565::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&holder.class_forward()));
}
// Source: tests/integration/bounded-interface-receiver-modes/case.trn
// Namespace: bounded-interface-receiver-modes
pub trait TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564Protocol: Send + Sync {
    fn name(&self) -> terrane_int_support::Int;
    fn increment(&mut self) -> terrane_int_support::Int;
    fn finish(self: Box<Self>) -> terrane_int_support::Int;
    fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int;
}
pub struct TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564(
    Box<
        dyn TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564Protocol,
    >,
);
impl TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        self.0.increment()
    }
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        self.0.add(amount)
    }
}
#[derive(Clone)]
pub struct TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565 {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565 {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.amount.clone()
                + terrane_int_support::Int::from(1_i128);
            self.amount = __terrane_value;
        }
        return self.amount.clone();
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.amount.clone() + amount.clone();
    }
}
impl TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564Protocol
for TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565 {
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565::name(
            &*self,
        )
    }
    fn increment(&mut self) -> terrane_int_support::Int {
        TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565::increment(
            &mut *self,
        )
    }
    fn finish(self: Box<Self>) -> terrane_int_support::Int {
        TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565::finish(
            *self,
        )
    }
    fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565::add(
            &*self,
            amount,
        )
    }
}
impl From<
    TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565,
>
for TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564 {
    fn from(
        value: TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn observe_terrane_bounded_interface_receiver_modes<
    TerraneType54: TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564Protocol,
>(values: terrane_collection_support::List<TerraneType54>) -> terrane_int_support::Int {
    return {
        let __terrane_index_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                4 /* terrane-site: tests/integration/bounded-interface-receiver-modes/case.trn:24:12-24:21 */,
        );
        let __terrane_index_place_1 = __terrane_raised(
            values
                .get(__terrane_index_0)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index_0,
                )),

                4 /* terrane-site: tests/integration/bounded-interface-receiver-modes/case.trn:24:12-24:21 */,
        );
        __terrane_index_place_1.name()
    };
}
fn mutate_terrane_bounded_interface_receiver_modes<
    TerraneType54: TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564Protocol,
>(item: &mut TerraneType54) -> terrane_int_support::Int {
    return item.increment();
}
fn consume_terrane_bounded_interface_receiver_modes<
    TerraneType54: TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    return Box::new(item).finish();
}
fn bounded_add<
    TerraneType54: TerraneNsBoundedInterfaceReceiverModesNamedIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F646573006E616D6564Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    return item.add(terrane_int_support::Int::from(2_i128));
}
fn test_scenario_terrane_bounded_interface_receiver_modes() {
    let values: terrane_collection_support::List<
        TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565,
    >;
    let mut item: TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565;
    values = terrane_collection_support::List::<
        TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565,
    >::new(
        vec![
            TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565::terrane_construct()
        ],
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&observe_terrane_bounded_interface_receiver_modes(values))
    );
    item = TerraneNsBoundedInterfaceReceiverModesValueIdentity2F626F756E6465642D696E746572666163652D72656365697665722D6D6F6465730076616C7565::terrane_construct();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&mutate_terrane_bounded_interface_receiver_modes(&mut
        item))
    );
    println!("{}", terrane_scalar_support::scalar_text(&item.name()));
    println!("{}", terrane_scalar_support::scalar_text(&bounded_add(item.clone())));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(1_i128) +
        terrane_int_support::Int::from(2_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&consume_terrane_bounded_interface_receiver_modes(item))
    );
}
// Source: tests/integration/class-field-metadata/case.trn
// Namespace: class-field-metadata
#[derive(Clone)]
pub struct ServiceOptions {
    pub internal_name: String,
    pub retry_limit: terrane_int_support::Int,
    pub note: Option<String>,
    pub credential: String,
}
impl ServiceOptions {
    pub fn terrane_construct() -> Self {
        Self {
            internal_name: String::from("primary"),
            retry_limit: terrane_int_support::Int::from(3_i128),
            note: None,
            credential: String::from("hidden"),
        }
    }
}
fn test_scenario_terrane_class_field_metadata() {
    let descriptor: TerraneDescriptor;
    descriptor = TerraneDescriptor {
        identity: "/class-field-metadata::service-options",
        name: "service-options",
        kind: "class",
        inherently_identity_bearing: false,
        fields: &[
            TerraneFieldMetadata {
                name: "internal-name",
                external_name: "serviceName",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "retry-limit",
                external_name: "retry-limit",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "note",
                external_name: "note",
                defaulted: true,
                optional: true,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "credential",
                external_name: "credential",
                defaulted: true,
                optional: false,
                secret: true,
            },
        ],
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(descriptor
        .fields.len() as i128))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
        &descriptor.fields.iter().map(| field | field.name.to_owned()).collect:: < Vec <
        String > > (); __terrane_raised({ let __terrane_receiver =
        &__terrane_receiver_value_0; let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: tests/integration/class-field-metadata/case.trn:12:12-12:37 */);
        __terrane_receiver.get(__terrane_index).cloned().ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)) },
        5 /* terrane-site: tests/integration/class-field-metadata/case.trn:12:12-12:37 */) }),
        String::from(":"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &descriptor.fields.iter().map(| field | field
        .external_name.to_owned()).collect:: < Vec < String > > (); __terrane_raised({
        let __terrane_receiver = &__terrane_receiver_value_1; let __terrane_index =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: tests/integration/class-field-metadata/case.trn:12:44-12:78 */);
        __terrane_receiver.get(__terrane_index).cloned().ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index)) },
        6 /* terrane-site: tests/integration/class-field-metadata/case.trn:12:44-12:78 */) })
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_2 =
        &terrane_collection_support::List::new(descriptor.fields.iter().map(| field |
        field.defaulted).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_2
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        7 /* terrane-site: tests/integration/class-field-metadata/case.trn:13:12-13:41 */)),
        7 /* terrane-site: tests/integration/class-field-metadata/case.trn:13:12-13:41 */) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_3 =
        &terrane_collection_support::List::new(descriptor.fields.iter().map(| field |
        field.optional).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        8 /* terrane-site: tests/integration/class-field-metadata/case.trn:13:43-13:71 */)),
        8 /* terrane-site: tests/integration/class-field-metadata/case.trn:13:43-13:71 */) }),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_4 =
        &terrane_collection_support::List::new(descriptor.fields.iter().map(| field |
        field.secret).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_4
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        9 /* terrane-site: tests/integration/class-field-metadata/case.trn:13:73-13:99 */)),
        9 /* terrane-site: tests/integration/class-field-metadata/case.trn:13:73-13:99 */) })
    );
}
// Source: tests/integration/class-instance-independence/case.trn
// Namespace: class-instance-independence
#[derive(Clone)]
pub struct TerraneNsClassInstanceIndependenceCounterIdentity2F636C6173732D696E7374616E63652D696E646570656E64656E636500636F756E746572 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsClassInstanceIndependenceCounterIdentity2F636C6173732D696E7374616E63652D696E646570656E64656E636500636F756E746572 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn increase(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
}
fn test_scenario_terrane_class_instance_independence() {
    let mut first: TerraneNsClassInstanceIndependenceCounterIdentity2F636C6173732D696E7374616E63652D696E646570656E64656E636500636F756E746572;
    let mut second: TerraneNsClassInstanceIndependenceCounterIdentity2F636C6173732D696E7374616E63652D696E646570656E64656E636500636F756E746572;
    let first_value: terrane_int_support::Int;
    let second_value: terrane_int_support::Int;
    let next_first: terrane_int_support::Int;
    first = TerraneNsClassInstanceIndependenceCounterIdentity2F636C6173732D696E7374616E63652D696E646570656E64656E636500636F756E746572::terrane_construct();
    second = TerraneNsClassInstanceIndependenceCounterIdentity2F636C6173732D696E7374616E63652D696E646570656E64656E636500636F756E746572::terrane_construct();
    first_value = first.increase();
    second_value = second.increase();
    next_first = first.increase();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&first_value),
        terrane_scalar_support::scalar_text(&second_value),
        terrane_scalar_support::scalar_text(&next_first)
    );
}
// Source: tests/integration/class-method-closures/case.trn
// Namespace: class-method-closures
#[derive(Clone)]
pub struct Maker {
    pub base: terrane_int_support::Int,
}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {
            base: terrane_int_support::Int::from(10_i128),
        }
    }
    pub fn offset(
        &self,
    ) -> std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    > {
        return {
            let this = self.clone();
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> terrane_int_support::Int {
                return this.base.clone() + value.clone();
            })
        };
    }
}
fn test_scenario_terrane_class_method_closures() {
    let value: Maker;
    let add: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    let result: terrane_int_support::Int;
    value = Maker::terrane_construct();
    add = value.offset();
    result = add(terrane_int_support::Int::from(5_i128));
    println!("{}", terrane_scalar_support::scalar_text(&result));
}
// Source: tests/integration/class-state-methods/case.trn
// Namespace: class-state-methods
#[derive(Clone)]
pub struct TerraneNsClassStateMethodsCounterIdentity2F636C6173732D73746174652D6D6574686F647300636F756E746572 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl TerraneNsClassStateMethodsCounterIdentity2F636C6173732D73746174652D6D6574686F647300636F756E746572 {
    pub fn terrane_construct(start: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(start);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, start: terrane_int_support::Int) {
        {
            let __terrane_value = start.clone();
            self.value = __terrane_value;
        }
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone() + amount.clone();
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
    pub fn shifted(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.value.clone() + amount.clone();
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("destruct"));
    }
}
impl Drop
for TerraneNsClassStateMethodsCounterIdentity2F636C6173732D73746174652D6D6574686F647300636F756E746572 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn test_scenario_terrane_class_state_methods() {
    let mut first: TerraneNsClassStateMethodsCounterIdentity2F636C6173732D73746174652D6D6574686F647300636F756E746572;
    let mut second: TerraneNsClassStateMethodsCounterIdentity2F636C6173732D73746174652D6D6574686F647300636F756E746572;
    let shift: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >;
    first = TerraneNsClassStateMethodsCounterIdentity2F636C6173732D73746174652D6D6574686F647300636F756E746572::terrane_construct(
        terrane_int_support::Int::from(10_i128),
    );
    second = first.terrane_separate();
    shift = {
        let receiver = first.terrane_separate();
        std::sync::Arc::new(move |argument_0: terrane_int_support::Int| {
            receiver.shifted(argument_0)
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&first
        .increase(terrane_int_support::Int::from(5_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&second
        .increase(terrane_int_support::Int::from(2_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&shift(terrane_int_support::Int::from(3_i128)))
    );
}
// Source: tests/integration/codec-class-methods/case.trn
// Namespace: codec-class-methods
fn test_scenario_terrane_codec_class_methods() -> Result<(), TerraneError> {
    let hexadecimal: HexCodec;
    let encoded_hex: String;
    let decoded_hex: DecodeResult;
    let standard_base64: Base64Codec;
    let encoded_base64: String;
    let decoded_base64: DecodeResult;
    hexadecimal = hex();
    encoded_hex = hexadecimal.encode(Vec::<u8>::from([97, 98, 99]));
    decoded_hex = hexadecimal.decode(encoded_hex);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&decoded_hex.value,
        __terrane_string_argument_0), 10 /* terrane-site: tests/integration/codec-class-methods/case.trn:8:13-8:43 */) ? })
    );
    standard_base64 = base64();
    encoded_base64 = standard_base64.encode(Vec::<u8>::from([97, 98, 99]), true);
    decoded_base64 = standard_base64.decode(encoded_base64, true);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0 =
        terrane_string_support::Encoding::Utf8;
        __terrane_raised_err(terrane_string_support::decode(&decoded_base64.value,
        __terrane_string_argument_0), 11 /* terrane-site: tests/integration/codec-class-methods/case.trn:13:13-13:46 */) ? })
    );
    Ok(())
}
// Source: tests/integration/collection-generic-keys-and-get/case.trn
// Namespace: collection-generic-keys-and-get
fn keys<TerraneType54>(
    values: terrane_collection_support::Map<String, TerraneType54>,
) -> terrane_collection_support::List<String> {
    return {
        let __terrane_result = values.keys();
        __terrane_result
    };
}
fn unordered_keys<TerraneType54>(
    values: terrane_collection_support::UnorderedMap<String, TerraneType54>,
) -> terrane_collection_support::List<String> {
    return {
        let __terrane_result = values.keys();
        __terrane_result
    };
}
fn test_scenario_terrane_collection_generic_keys_and_get() {
    let values: terrane_collection_support::Map<String, terrane_int_support::Int>;
    let names: terrane_collection_support::List<String>;
    let found: terrane_int_support::Int;
    let other: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >;
    let other_names: terrane_collection_support::List<String>;
    let unordered_found: terrane_int_support::Int;
    let numbers: terrane_collection_support::List<terrane_int_support::Int>;
    let fixed: terrane_collection_support::Tuple<terrane_int_support::Int>;
    values = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_int_support::Int::from(7_i128)),
            terrane_collection_support::Entry::new(String::from("b"),
            terrane_int_support::Int::from(9_i128))
        ],
    );
    names = keys(values.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(names
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        12 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:10:12-10:20 */)),
        12 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:10:12-10:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(names
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        13 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:11:12-11:20 */)),
        13 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:11:12-11:20 */))
    );
    found = __terrane_raised(
        values.get_or_error(&String::from("b")),

            14 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:12:13-12:28 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&found));
    other = terrane_collection_support::UnorderedMap::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("k"),
            terrane_int_support::Int::from(11_i128))
        ],
    );
    other_names = unordered_keys(other.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(other_names
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        15 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:16:12-16:26 */)),
        15 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:16:12-16:26 */))
    );
    unordered_found = __terrane_raised(
        other.get_or_error(&String::from("k")),

            16 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:17:23-17:37 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&unordered_found));
    numbers = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(13_i128)]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
        .and_then(| index | numbers.get_or_error(index)), 17 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:20:13-20:27 */))
    );
    fixed = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(17_i128)]);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
        .and_then(| index | fixed.get_or_error(index)), 18 /* terrane-site: tests/integration/collection-generic-keys-and-get/case.trn:22:13-22:25 */))
    );
}
// Source: tests/integration/completion-callable-rewrite/case.trn
// Namespace: completion-callable-rewrite
fn one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn eat_terrane_completion_callable_rewrite(
    f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
) -> terrane_int_support::Int {
    return f();
}
fn test_scenario_terrane_completion_callable_rewrite() {
    let mut o: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    o = std::sync::Arc::new(one);
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&eat_terrane_completion_callable_rewrite(o
                .clone()))
            );
            o = std::sync::Arc::new(one);
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&2));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&o()));
}
// Source: tests/integration/completion-finally-generic-post-read/case.trn
// Namespace: completion-finally-generic-post-read
fn eat_terrane_completion_finally_generic_post_read<TerraneType54>(
    k: TerraneType54,
) -> terrane_int_support::Int {
    let _ = &k;
    return terrane_int_support::Int::from(1_i128);
}
fn g<TerraneType54>(v: TerraneType54, w: TerraneType54) -> TerraneType54 {
    let mut o: Option<TerraneType54> = None;
    let _ = o.insert(v);
    let mut __terrane_completion_0: TerraneCompletion<TerraneType54> = (|| {
        let __terrane_try_0: TerraneCompletion<TerraneType54> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&eat_terrane_completion_finally_generic_post_read(o
                .take().expect("flow-proven availability")))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<TerraneType54> = (|| {
        let _ = o.insert(w);
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    return o.take().expect("flow-proven availability");
}
fn test_scenario_terrane_completion_finally_generic_post_read() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&g(terrane_int_support::Int::from(1_i128),
        terrane_int_support::Int::from(2_i128)))
    );
}
// Source: tests/integration/completion-finally-loop-break/case.trn
// Namespace: completion-finally-loop-break
fn eat_terrane_completion_finally_loop_break(
    k: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return __terrane_raised(
        k
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        19 /* terrane-site: tests/integration/completion-finally-loop-break/case.trn:5:12-5:16 */,
                ),
            ),

            19 /* terrane-site: tests/integration/completion-finally-loop-break/case.trn:5:12-5:16 */,
    );
}
fn test_scenario_terrane_completion_finally_loop_break() {
    let mut o: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    let i: i64;
    let _ = o
        .insert(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        );
    i = 0;
    while i < 2 {
        let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&eat_terrane_completion_finally_loop_break(o
                    .as_ref().expect("flow-proven available binding").clone()))
                );
                return TerraneCompletion::Break;
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_0: TerraneCompletion<()> = (|| {
            let _ = o
                .insert(
                    terrane_collection_support::List::<
                        terrane_int_support::Int,
                    >::new(vec![terrane_int_support::Int::from(2_i128)]),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_0 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_0 = replacement,
        }
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {
                __terrane_generated_defect("non-fallthrough try completed normally")
            }
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => {
                __terrane_generated_defect("non-continuing try produced continue")
            }
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(o.as_ref()
        .expect("flow-proven available binding")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        20 /* terrane-site: tests/integration/completion-finally-loop-break/case.trn:16:12-16:16 */)),
        20 /* terrane-site: tests/integration/completion-finally-loop-break/case.trn:16:12-16:16 */))
    );
}
// Source: tests/integration/completion-finally-loop-repeat/case.trn
// Namespace: completion-finally-loop-repeat
fn eat_terrane_completion_finally_loop_repeat(
    k: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return __terrane_raised(
        k
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        21 /* terrane-site: tests/integration/completion-finally-loop-repeat/case.trn:5:12-5:16 */,
                ),
            ),

            21 /* terrane-site: tests/integration/completion-finally-loop-repeat/case.trn:5:12-5:16 */,
    );
}
fn test_scenario_terrane_completion_finally_loop_repeat() {
    let mut o: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    let mut i: terrane_int_support::Int;
    let _ = o
        .insert(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        );
    i = terrane_int_support::Int::from(0_i128);
    while i < terrane_int_support::Int::from(2_i128) {
        let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&eat_terrane_completion_finally_loop_repeat(o
                    .as_ref().expect("flow-proven available binding").clone()))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_0: TerraneCompletion<()> = (|| {
            let _ = o
                .insert(
                    terrane_collection_support::List::<
                        terrane_int_support::Int,
                    >::new(vec![terrane_int_support::Int::from(2_i128)]),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_0 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_0 = replacement,
        }
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => {
                __terrane_generated_defect("non-continuing try produced continue")
            }
        }
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
}
// Source: tests/integration/completion-finally-nested-post-read/case.trn
// Namespace: completion-finally-nested-post-read
fn eat_terrane_completion_finally_nested_post_read(
    k: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return __terrane_raised(
        k
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        22 /* terrane-site: tests/integration/completion-finally-nested-post-read/case.trn:5:12-5:16 */,
                ),
            ),

            22 /* terrane-site: tests/integration/completion-finally-nested-post-read/case.trn:5:12-5:16 */,
    );
}
fn test_scenario_terrane_completion_finally_nested_post_read() {
    let mut o: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    let _ = o
        .insert(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let mut __terrane_completion_1: TerraneCompletion<()> = (|| {
                let __terrane_try_1: TerraneCompletion<()> = (|| {
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&eat_terrane_completion_finally_nested_post_read(o
                        .as_ref().expect("flow-proven available binding").clone()))
                    );
                    TerraneCompletion::Normal
                })();
                match __terrane_try_1 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_1) => {
                        let mut __terrane_handled_1 = false;
                        if !__terrane_handled_1 {
                            return TerraneCompletion::Error(__terrane_error_1);
                        }
                    }
                }
                TerraneCompletion::Normal
            })();
            let __terrane_finally_1: TerraneCompletion<()> = (|| {
                let _ = o
                    .insert(
                        terrane_collection_support::List::<
                            terrane_int_support::Int,
                        >::new(vec![terrane_int_support::Int::from(2_i128)]),
                    );
                TerraneCompletion::Normal
            })();
            match __terrane_finally_1 {
                TerraneCompletion::Normal => {}
                replacement => __terrane_completion_1 = replacement,
            }
            match __terrane_completion_1 {
                TerraneCompletion::Normal => {}
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
            }
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(o
                .as_ref().expect("flow-proven available binding")
                .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                23 /* terrane-site: tests/integration/completion-finally-nested-post-read/case.trn:14:16-14:20 */)),
                23 /* terrane-site: tests/integration/completion-finally-nested-post-read/case.trn:14:16-14:20 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&9));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/completion-finally-post-read/case.trn
// Namespace: completion-finally-post-read
fn eat_terrane_completion_finally_post_read(
    k: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return __terrane_raised(
        k
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        24 /* terrane-site: tests/integration/completion-finally-post-read/case.trn:5:12-5:16 */,
                ),
            ),

            24 /* terrane-site: tests/integration/completion-finally-post-read/case.trn:5:12-5:16 */,
    );
}
fn test_scenario_terrane_completion_finally_post_read() {
    let mut o: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    let _ = o
        .insert(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&eat_terrane_completion_finally_post_read(o
                .as_ref().expect("flow-proven available binding").clone()))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        let _ = o
            .insert(
                terrane_collection_support::List::<
                    terrane_int_support::Int,
                >::new(vec![terrane_int_support::Int::from(2_i128)]),
            );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(o.as_ref()
        .expect("flow-proven available binding")
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        25 /* terrane-site: tests/integration/completion-finally-post-read/case.trn:13:12-13:16 */)),
        25 /* terrane-site: tests/integration/completion-finally-post-read/case.trn:13:12-13:16 */))
    );
}
// Source: tests/integration/completion-finally-string-post-read/case.trn
// Namespace: completion-finally-string-post-read
fn eat_terrane_completion_finally_string_post_read(
    s: String,
) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(terrane_string_support::length(&s) as i128);
}
fn test_scenario_terrane_completion_finally_string_post_read() {
    let mut o: Option<String> = None;
    let _ = o.insert(String::from("ab"));
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&eat_terrane_completion_finally_string_post_read(o
                .as_ref().expect("flow-proven available binding").clone()))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        let _ = o.insert(String::from("c"));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&o.as_ref()
        .expect("flow-proven available binding"))
    );
}
// Source: tests/integration/constructor-if-initialization/case.trn
// Namespace: constructor-if-initialization
#[derive(Clone)]
pub struct RequiredValue {
    pub text: String,
}
impl RequiredValue {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct Configured {
    pub slot: Option<RequiredValue>,
}
impl Configured {
    pub fn terrane_construct(value: String) -> Self {
        let mut __terrane_constructed_value = Self { slot: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: String) {
        if value.as_str() == "a" {
            {
                let __terrane_value = RequiredValue::terrane_construct(
                    String::from("first"),
                );
                self.slot = Some(__terrane_value);
            }
        } else if value.as_str() == "b" {
            {
                let __terrane_value = RequiredValue::terrane_construct(
                    String::from("second"),
                );
                self.slot = Some(__terrane_value);
            }
        } else {
            {
                let __terrane_value = RequiredValue::terrane_construct(
                    String::from("other"),
                );
                self.slot = Some(__terrane_value);
            }
        }
    }
}
#[derive(Clone)]
pub struct ConfiguredWithRead {
    pub slot: Option<RequiredValue>,
}
impl ConfiguredWithRead {
    pub fn terrane_construct(value: String) -> Self {
        let mut __terrane_constructed_value = Self { slot: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: String) {
        if value.as_str() == "a" {
            {
                let __terrane_value = RequiredValue::terrane_construct(
                    String::from("first"),
                );
                self.slot = Some(__terrane_value);
            }
        } else {
            {
                let __terrane_value = RequiredValue::terrane_construct(
                    String::from("other"),
                );
                self.slot = Some(__terrane_value);
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&self.slot.as_ref()
            .expect("required field initialized").text)
        );
    }
}
fn test_scenario_terrane_constructor_if_initialization() {
    let first: Configured;
    let second: Configured;
    let third: Configured;
    let read_first: ConfiguredWithRead;
    let read_other: ConfiguredWithRead;
    first = Configured::terrane_construct(String::from("a"));
    second = Configured::terrane_construct(String::from("b"));
    third = Configured::terrane_construct(String::from("c"));
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&first.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&second.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&third.slot.as_ref()
        .expect("required field initialized").text)
    );
    read_first = ConfiguredWithRead::terrane_construct(String::from("a"));
    read_other = ConfiguredWithRead::terrane_construct(String::from("b"));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&read_first.slot.as_ref()
        .expect("required field initialized").text), String::from("|"),
        terrane_scalar_support::scalar_text(&read_other.slot.as_ref()
        .expect("required field initialized").text)
    );
}
// Source: tests/integration/constructor-none-defaults/case.trn
// Namespace: constructor-none-defaults
#[derive(Clone)]
pub struct TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572 {
    pub tag: String,
    pub value: Option<
        TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164,
    >,
}
impl TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572 {
    pub fn terrane_construct(
        tag: String,
        value: Option<
            TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164,
        >,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            tag: String::new(),
            value: None,
        };
        __terrane_constructed_value.construct(tag, value);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        tag: String,
        value: Option<
            TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164,
        >,
    ) {
        {
            let __terrane_value = tag;
            self.tag = __terrane_value;
        }
        {
            let __terrane_value = value;
            self.value = __terrane_value;
        }
    }
}
fn describe(
    value: Option<
        TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164,
    >,
) -> String {
    if (&value).is_some() {
        return {
            let mut __terrane_text = String::with_capacity(6);
            __terrane_text.push_str("value=");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &match &value {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .value,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
    return String::from("missing");
}
fn test_scenario_terrane_constructor_none_defaults() {
    let omitted: TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572;
    let partial: TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572;
    let present: TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572;
    let owned: Option<
        TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164,
    >;
    omitted = TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572::terrane_construct(
        String::from("default"),
        None,
    );
    partial = TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572::terrane_construct(
        String::from("partial"),
        None,
    );
    present = TerraneNsConstructorNoneDefaultsHolderIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C747300686F6C646572::terrane_construct(
        String::from("present"),
        Some(
            TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164::terrane_construct(),
        ),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&omitted.tag), String::from(":"),
        terrane_scalar_support::scalar_text(&(&omitted.value).is_none())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&partial.tag), String::from(":"),
        terrane_scalar_support::scalar_text(&(&partial.value).is_none())
    );
    owned = present.value.clone();
    if (&owned).is_some() {
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&present.tag),
            String::from(":"), terrane_scalar_support::scalar_text(&match &owned {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .value)
        );
    }
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&describe(None)),
        String::from(":"), terrane_scalar_support::scalar_text(&describe(None)),
        String::from(":"),
        terrane_scalar_support::scalar_text(&describe(Some(TerraneNsConstructorNoneDefaultsPayloadIdentity2F636F6E7374727563746F722D6E6F6E652D64656661756C7473007061796C6F6164::terrane_construct())))
    );
}
// Source: tests/integration/deep-inheritance-field-forwarding/case.trn
// Namespace: deep-inheritance-field-forwarding
#[derive(Clone)]
pub struct Level0Storage {
    pub f0: terrane_int_support::Int,
}
impl Level0Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level0 {
    Own(Level0Storage),
    Level1(Level1),
    Level2(Level2),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level0 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level0Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level1(value) => value.terrane_field_f0(),
            Self::Level2(value) => value.terrane_field_f0(),
            Self::Level3(value) => value.terrane_field_f0(),
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level1(value) => value.terrane_field_f0_mut(),
            Self::Level2(value) => value.terrane_field_f0_mut(),
            Self::Level3(value) => value.terrane_field_f0_mut(),
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
}
#[derive(Clone)]
pub struct Level1Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
}
impl Level1Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level1 {
    Own(Level1Storage),
    Level2(Level2),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level1 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level1Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level2(value) => value.terrane_field_f0(),
            Self::Level3(value) => value.terrane_field_f0(),
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level2(value) => value.terrane_field_f0_mut(),
            Self::Level3(value) => value.terrane_field_f0_mut(),
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level2(value) => value.terrane_field_f1(),
            Self::Level3(value) => value.terrane_field_f1(),
            Self::Level4(value) => value.terrane_field_f1(),
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level2(value) => value.terrane_field_f1_mut(),
            Self::Level3(value) => value.terrane_field_f1_mut(),
            Self::Level4(value) => value.terrane_field_f1_mut(),
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
}
#[derive(Clone)]
pub struct Level2Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
}
impl Level2Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level2 {
    Own(Level2Storage),
    Level3(Level3),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level2 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level2Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level3(value) => value.terrane_field_f0(),
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level3(value) => value.terrane_field_f0_mut(),
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level3(value) => value.terrane_field_f1(),
            Self::Level4(value) => value.terrane_field_f1(),
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level3(value) => value.terrane_field_f1_mut(),
            Self::Level4(value) => value.terrane_field_f1_mut(),
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level3(value) => value.terrane_field_f2(),
            Self::Level4(value) => value.terrane_field_f2(),
            Self::Level5(value) => value.terrane_field_f2(),
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level3(value) => value.terrane_field_f2_mut(),
            Self::Level4(value) => value.terrane_field_f2_mut(),
            Self::Level5(value) => value.terrane_field_f2_mut(),
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
}
#[derive(Clone)]
pub struct Level3Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
}
impl Level3Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level3 {
    Own(Level3Storage),
    Level4(Level4),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level3 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level3Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level4(value) => value.terrane_field_f0(),
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level4(value) => value.terrane_field_f0_mut(),
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level4(value) => value.terrane_field_f1(),
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level4(value) => value.terrane_field_f1_mut(),
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level4(value) => value.terrane_field_f2(),
            Self::Level5(value) => value.terrane_field_f2(),
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level4(value) => value.terrane_field_f2_mut(),
            Self::Level5(value) => value.terrane_field_f2_mut(),
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level4(value) => value.terrane_field_f3(),
            Self::Level5(value) => value.terrane_field_f3(),
            Self::Level6(value) => value.terrane_field_f3(),
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level4(value) => value.terrane_field_f3_mut(),
            Self::Level5(value) => value.terrane_field_f3_mut(),
            Self::Level6(value) => value.terrane_field_f3_mut(),
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
}
#[derive(Clone)]
pub struct Level4Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
}
impl Level4Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level4 {
    Own(Level4Storage),
    Level5(Level5),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level4 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level4Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level5(value) => value.terrane_field_f0(),
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level5(value) => value.terrane_field_f0_mut(),
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level5(value) => value.terrane_field_f1(),
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level5(value) => value.terrane_field_f1_mut(),
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level5(value) => value.terrane_field_f2(),
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level5(value) => value.terrane_field_f2_mut(),
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level5(value) => value.terrane_field_f3(),
            Self::Level6(value) => value.terrane_field_f3(),
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level5(value) => value.terrane_field_f3_mut(),
            Self::Level6(value) => value.terrane_field_f3_mut(),
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level5(value) => value.terrane_field_f4(),
            Self::Level6(value) => value.terrane_field_f4(),
            Self::Level7(value) => value.terrane_field_f4(),
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level5(value) => value.terrane_field_f4_mut(),
            Self::Level6(value) => value.terrane_field_f4_mut(),
            Self::Level7(value) => value.terrane_field_f4_mut(),
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
}
#[derive(Clone)]
pub struct Level5Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
}
impl Level5Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level5 {
    Own(Level5Storage),
    Level6(Level6),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level5 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level5Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level6(value) => value.terrane_field_f0(),
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level6(value) => value.terrane_field_f0_mut(),
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level6(value) => value.terrane_field_f1(),
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level6(value) => value.terrane_field_f1_mut(),
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level6(value) => value.terrane_field_f2(),
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level6(value) => value.terrane_field_f2_mut(),
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level6(value) => value.terrane_field_f3(),
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level6(value) => value.terrane_field_f3_mut(),
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level6(value) => value.terrane_field_f4(),
            Self::Level7(value) => value.terrane_field_f4(),
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level6(value) => value.terrane_field_f4_mut(),
            Self::Level7(value) => value.terrane_field_f4_mut(),
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level6(value) => value.terrane_field_f5(),
            Self::Level7(value) => value.terrane_field_f5(),
            Self::Level8(value) => value.terrane_field_f5(),
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level6(value) => value.terrane_field_f5_mut(),
            Self::Level7(value) => value.terrane_field_f5_mut(),
            Self::Level8(value) => value.terrane_field_f5_mut(),
            Self::Level9(value) => &mut value.f5,
        }
    }
}
#[derive(Clone)]
pub struct Level6Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
}
impl Level6Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level6 {
    Own(Level6Storage),
    Level7(Level7),
    Level8(Level8),
    Level9(Level9),
}
impl Level6 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level6Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level7(value) => value.terrane_field_f0(),
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level7(value) => value.terrane_field_f0_mut(),
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level7(value) => value.terrane_field_f1(),
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level7(value) => value.terrane_field_f1_mut(),
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level7(value) => value.terrane_field_f2(),
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level7(value) => value.terrane_field_f2_mut(),
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level7(value) => value.terrane_field_f3(),
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level7(value) => value.terrane_field_f3_mut(),
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level7(value) => value.terrane_field_f4(),
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level7(value) => value.terrane_field_f4_mut(),
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level7(value) => value.terrane_field_f5(),
            Self::Level8(value) => value.terrane_field_f5(),
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level7(value) => value.terrane_field_f5_mut(),
            Self::Level8(value) => value.terrane_field_f5_mut(),
            Self::Level9(value) => &mut value.f5,
        }
    }
    pub fn terrane_field_f6(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f6,
            Self::Level7(value) => value.terrane_field_f6(),
            Self::Level8(value) => value.terrane_field_f6(),
            Self::Level9(value) => &value.f6,
        }
    }
    pub fn terrane_field_f6_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f6,
            Self::Level7(value) => value.terrane_field_f6_mut(),
            Self::Level8(value) => value.terrane_field_f6_mut(),
            Self::Level9(value) => &mut value.f6,
        }
    }
}
#[derive(Clone)]
pub struct Level7Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
    pub f7: terrane_int_support::Int,
}
impl Level7Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
            f7: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level7 {
    Own(Level7Storage),
    Level8(Level8),
    Level9(Level9),
}
impl Level7 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level7Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level8(value) => value.terrane_field_f0(),
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level8(value) => value.terrane_field_f0_mut(),
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level8(value) => value.terrane_field_f1(),
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level8(value) => value.terrane_field_f1_mut(),
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level8(value) => value.terrane_field_f2(),
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level8(value) => value.terrane_field_f2_mut(),
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level8(value) => value.terrane_field_f3(),
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level8(value) => value.terrane_field_f3_mut(),
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level8(value) => value.terrane_field_f4(),
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level8(value) => value.terrane_field_f4_mut(),
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level8(value) => value.terrane_field_f5(),
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level8(value) => value.terrane_field_f5_mut(),
            Self::Level9(value) => &mut value.f5,
        }
    }
    pub fn terrane_field_f6(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f6,
            Self::Level8(value) => value.terrane_field_f6(),
            Self::Level9(value) => &value.f6,
        }
    }
    pub fn terrane_field_f6_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f6,
            Self::Level8(value) => value.terrane_field_f6_mut(),
            Self::Level9(value) => &mut value.f6,
        }
    }
    pub fn terrane_field_f7(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f7,
            Self::Level8(value) => value.terrane_field_f7(),
            Self::Level9(value) => &value.f7,
        }
    }
    pub fn terrane_field_f7_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f7,
            Self::Level8(value) => value.terrane_field_f7_mut(),
            Self::Level9(value) => &mut value.f7,
        }
    }
}
#[derive(Clone)]
pub struct Level8Storage {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
    pub f7: terrane_int_support::Int,
    pub f8: terrane_int_support::Int,
}
impl Level8Storage {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
            f7: terrane_int_support::Int::from(7_i128),
            f8: terrane_int_support::Int::from(8_i128),
        }
    }
}
#[derive(Clone)]
pub enum Level8 {
    Own(Level8Storage),
    Level9(Level9),
}
impl Level8 {
    pub fn terrane_construct() -> Self {
        Self::Own(Level8Storage::terrane_construct())
    }
    pub fn terrane_field_f0(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f0,
            Self::Level9(value) => &value.f0,
        }
    }
    pub fn terrane_field_f0_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f0,
            Self::Level9(value) => &mut value.f0,
        }
    }
    pub fn terrane_field_f1(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f1,
            Self::Level9(value) => &value.f1,
        }
    }
    pub fn terrane_field_f1_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f1,
            Self::Level9(value) => &mut value.f1,
        }
    }
    pub fn terrane_field_f2(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f2,
            Self::Level9(value) => &value.f2,
        }
    }
    pub fn terrane_field_f2_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f2,
            Self::Level9(value) => &mut value.f2,
        }
    }
    pub fn terrane_field_f3(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f3,
            Self::Level9(value) => &value.f3,
        }
    }
    pub fn terrane_field_f3_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f3,
            Self::Level9(value) => &mut value.f3,
        }
    }
    pub fn terrane_field_f4(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f4,
            Self::Level9(value) => &value.f4,
        }
    }
    pub fn terrane_field_f4_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f4,
            Self::Level9(value) => &mut value.f4,
        }
    }
    pub fn terrane_field_f5(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f5,
            Self::Level9(value) => &value.f5,
        }
    }
    pub fn terrane_field_f5_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f5,
            Self::Level9(value) => &mut value.f5,
        }
    }
    pub fn terrane_field_f6(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f6,
            Self::Level9(value) => &value.f6,
        }
    }
    pub fn terrane_field_f6_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f6,
            Self::Level9(value) => &mut value.f6,
        }
    }
    pub fn terrane_field_f7(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f7,
            Self::Level9(value) => &value.f7,
        }
    }
    pub fn terrane_field_f7_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f7,
            Self::Level9(value) => &mut value.f7,
        }
    }
    pub fn terrane_field_f8(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.f8,
            Self::Level9(value) => &value.f8,
        }
    }
    pub fn terrane_field_f8_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.f8,
            Self::Level9(value) => &mut value.f8,
        }
    }
}
#[derive(Clone)]
pub struct Level9 {
    pub f0: terrane_int_support::Int,
    pub f1: terrane_int_support::Int,
    pub f2: terrane_int_support::Int,
    pub f3: terrane_int_support::Int,
    pub f4: terrane_int_support::Int,
    pub f5: terrane_int_support::Int,
    pub f6: terrane_int_support::Int,
    pub f7: terrane_int_support::Int,
    pub f8: terrane_int_support::Int,
    pub f9: terrane_int_support::Int,
}
impl Level9 {
    pub fn terrane_construct() -> Self {
        Self {
            f0: terrane_int_support::Int::from(0_i128),
            f1: terrane_int_support::Int::from(1_i128),
            f2: terrane_int_support::Int::from(2_i128),
            f3: terrane_int_support::Int::from(3_i128),
            f4: terrane_int_support::Int::from(4_i128),
            f5: terrane_int_support::Int::from(5_i128),
            f6: terrane_int_support::Int::from(6_i128),
            f7: terrane_int_support::Int::from(7_i128),
            f8: terrane_int_support::Int::from(8_i128),
            f9: terrane_int_support::Int::from(9_i128),
        }
    }
}
fn test_scenario_terrane_deep_inheritance_field_forwarding() {
    let leaf: Level9;
    let mut root: Level0;
    leaf = Level9::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&leaf.f0),
        terrane_scalar_support::scalar_text(&leaf.f5),
        terrane_scalar_support::scalar_text(&leaf.f9)
    );
    root = Level0::Level9(leaf.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&root.terrane_field_f0().clone())
    );
    {
        let __terrane_value = terrane_int_support::Int::from(10_i128);
        *root.terrane_field_f0_mut() = __terrane_value;
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&root.terrane_field_f0().clone()),
        terrane_scalar_support::scalar_text(&leaf.f0)
    );
}
// Source: tests/integration/explicit-class-operations/case.trn
// Namespace: explicit-class-operations
#[derive(Clone)]
pub struct TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn terrane_static_create() -> TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574 {
        return TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574::terrane_construct();
    }
}
fn test_scenario_terrane_explicit_class_operations() {
    let direct: TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574;
    let factory: TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574;
    direct = TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574::terrane_construct();
    factory = TerraneNsExplicitClassOperationsWidgetIdentity2F6578706C696369742D636C6173732D6F7065726174696F6E7300776964676574::terrane_static_create();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&direct.read()),
        terrane_scalar_support::scalar_text(&factory.read())
    );
}
// Source: tests/integration/explicit-generic-mutable-closure/case.trn
// Namespace: explicit-generic-mutable-closure
#[derive(Clone)]
pub struct TerraneNsExplicitGenericMutableClosureBoxIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500426F78<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsExplicitGenericMutableClosureBoxIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500426F78<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsExplicitGenericMutableClosureCounterIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500436F756E746572 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsExplicitGenericMutableClosureCounterIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500436F756E746572 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn touch<TerraneType54>(
        &mut self,
        value: TerraneType54,
    ) -> TerraneNsExplicitGenericMutableClosureBoxIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500426F78<
        TerraneType54,
    > {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
        return TerraneNsExplicitGenericMutableClosureBoxIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500426F78::<
            TerraneType54,
        >::terrane_construct(value);
    }
}
fn test_scenario_terrane_explicit_generic_mutable_closure() {
    let c: TerraneNsExplicitGenericMutableClosureCounterIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500436F756E746572;
    let f: TerraneMutableCallable<(), terrane_int_support::Int>;
    c = TerraneNsExplicitGenericMutableClosureCounterIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500436F756E746572::terrane_construct();
    f = {
        let mut c = c.clone();
        TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
            let b: TerraneNsExplicitGenericMutableClosureBoxIdentity2F6578706C696369742D67656E657269632D6D757461626C652D636C6F7375726500426F78<
                terrane_int_support::Int,
            >;
            b = c
                .touch::<
                    terrane_int_support::Int,
                >(terrane_int_support::Int::from(7_i128));
            println!(
                "{}", terrane_scalar_support::scalar_text(&b.value.as_ref()
                .expect("required field initialized"))
            );
            return c.value.clone();
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&f.call(())));
    println!("{}", terrane_scalar_support::scalar_text(&f.call(())));
}
// Source: tests/integration/explicit-generic-unused/case.trn
// Namespace: explicit-generic-unused
fn constant<TerraneType54>() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn test_scenario_terrane_explicit_generic_unused() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&constant:: < terrane_int_support::Int
        > ())
    );
}
// Source: tests/integration/field-default-assignability/case.trn
// Namespace: field-default-assignability
pub static TERRANE_STATIC_TERRANENSFIELDDEFAULTASSIGNABILITYDEFAULTSIDENTITY2F6669656C642D64656661756C742D61737369676E6162696C6974790044656661756C7473_LABEL: std::sync::LazyLock<
    TerraneSharedCell<String>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(String::from("ready")));
#[derive(Clone)]
pub struct TerraneNsFieldDefaultAssignabilityDefaultsIdentity2F6669656C642D64656661756C742D61737369676E6162696C6974790044656661756C7473 {
    pub integer: terrane_int_support::Int,
    pub optional: Option<terrane_int_support::Int>,
    pub nothing: (),
    pub lines: terrane_collection_support::List<String>,
    pub labels: terrane_collection_support::Map<String, String>,
}
impl TerraneNsFieldDefaultAssignabilityDefaultsIdentity2F6669656C642D64656661756C742D61737369676E6162696C6974790044656661756C7473 {
    pub fn terrane_construct() -> Self {
        Self {
            integer: terrane_int_support::Int::from(4_i128),
            optional: None,
            nothing: (),
            lines: terrane_collection_support::List::<String>::new(vec![]),
            labels: terrane_collection_support::Map::<String, String>::new(vec![]),
        }
    }
}
fn test_scenario_terrane_field_default_assignability() {
    let value: TerraneNsFieldDefaultAssignabilityDefaultsIdentity2F6669656C642D64656661756C742D61737369676E6162696C6974790044656661756C7473;
    value = TerraneNsFieldDefaultAssignabilityDefaultsIdentity2F6669656C642D64656661756C742D61737369676E6162696C6974790044656661756C7473::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.integer));
    println!("{}", terrane_scalar_support::scalar_text(&(&value.optional).is_none()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = value.nothing; true })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value
        .lines.length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value
        .labels.length()))
    );
    println!(
        "{}", { let __terrane_result = terrane_scalar_support::scalar_text(&*
        TERRANE_STATIC_TERRANENSFIELDDEFAULTASSIGNABILITYDEFAULTSIDENTITY2F6669656C642D64656661756C742D61737369676E6162696C6974790044656661756C7473_LABEL
        .lock().expect("static field lock poisoned")); __terrane_result }
    );
}
// Source: tests/integration/field-grouped-consuming-once/case.trn
// Namespace: field-grouped-consuming-once
pub trait TerraneNsFieldGroupedConsumingOnceNamedIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365006E616D6564Protocol: Send + Sync {
    fn name(&self) -> terrane_int_support::Int;
    fn finish(self: Box<Self>) -> terrane_int_support::Int;
}
pub struct TerraneNsFieldGroupedConsumingOnceNamedIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365006E616D6564(
    Box<
        dyn TerraneNsFieldGroupedConsumingOnceNamedIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365006E616D6564Protocol,
    >,
);
impl TerraneNsFieldGroupedConsumingOnceNamedIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
}
#[derive(Clone)]
pub struct TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(7_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsFieldGroupedConsumingOnceNamedIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365006E616D6564Protocol
for TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D {
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D::name(
            &*self,
        )
    }
    fn finish(self: Box<Self>) -> terrane_int_support::Int {
        TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D::finish(
            *self,
        )
    }
}
impl From<
    TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D,
>
for TerraneNsFieldGroupedConsumingOnceNamedIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365006E616D6564 {
    fn from(
        value: TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsFieldGroupedConsumingOnceHolderIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E636500486F6C646572 {
    pub item: Option<
        TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D,
    >,
}
impl TerraneNsFieldGroupedConsumingOnceHolderIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E636500486F6C646572 {
    pub fn terrane_construct(
        item: TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D,
    ) -> Self {
        let mut __terrane_constructed_value = Self { item: None };
        __terrane_constructed_value.construct(item);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        item: TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D,
    ) {
        {
            let __terrane_value = item;
            self.item = Some(__terrane_value);
        }
    }
    pub fn finish(mut self) -> terrane_int_support::Int {
        return self.item.take().expect("required field initialized").finish();
    }
}
fn test_scenario_terrane_field_grouped_consuming_once() {
    let holder: TerraneNsFieldGroupedConsumingOnceHolderIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E636500486F6C646572;
    holder = TerraneNsFieldGroupedConsumingOnceHolderIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E636500486F6C646572::terrane_construct(
        TerraneNsFieldGroupedConsumingOnceItemIdentity2F6669656C642D67726F757065642D636F6E73756D696E672D6F6E6365004974656D::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&holder.finish()));
}
// Source: tests/integration/finite-generic-class-type/case.trn
// Namespace: finite-generic-class-type
#[derive(Clone)]
pub struct TerraneNsFiniteGenericClassTypeNodeIdentity2F66696E6974652D67656E657269632D636C6173732D74797065006E6F6465<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
    pub next: terrane_collection_support::List<
        TerraneNsFiniteGenericClassTypeNodeIdentity2F66696E6974652D67656E657269632D636C6173732D74797065006E6F6465<
            TerraneType54,
        >,
    >,
}
impl<
    TerraneType54,
> TerraneNsFiniteGenericClassTypeNodeIdentity2F66696E6974652D67656E657269632D636C6173732D74797065006E6F6465<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
            next: terrane_collection_support::List::<
                TerraneNsFiniteGenericClassTypeNodeIdentity2F66696E6974652D67656E657269632D636C6173732D74797065006E6F6465<
                    TerraneType54,
                >,
            >::new(Vec::new()),
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_finite_generic_class_type() {
    let n: TerraneNsFiniteGenericClassTypeNodeIdentity2F66696E6974652D67656E657269632D636C6173732D74797065006E6F6465<
        terrane_int_support::Int,
    >;
    n = TerraneNsFiniteGenericClassTypeNodeIdentity2F66696E6974652D67656E657269632D636C6173732D74797065006E6F6465::<
        terrane_int_support::Int,
    >::terrane_construct(terrane_int_support::Int::from(1_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&n.value.as_ref()
        .expect("required field initialized"))
    );
}
// Source: tests/integration/generic-conditional-clone-and-inherited-interface/case.trn
// Namespace: generic-conditional-clone-and-inherited-interface
#[derive(Clone)]
pub struct TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct Wrap<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<
        TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
            TerraneType54,
        >,
    >,
}
impl<TerraneType54> Wrap<TerraneType54> {
    pub fn terrane_construct(
        value: TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
            TerraneType54,
        >,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        value: TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
            TerraneType54,
        >,
    ) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct Nested<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<Wrap<Wrap<TerraneType54>>>,
}
impl<TerraneType54> Nested<TerraneType54> {
    pub fn terrane_construct(value: Wrap<Wrap<TerraneType54>>) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: Wrap<Wrap<TerraneType54>>) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct Generic<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub direct: Option<
        TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
            TerraneType54,
        >,
    >,
    pub nested: Option<Nested<TerraneType54>>,
    pub values: terrane_collection_support::List<TerraneType54>,
    pub fixed: terrane_int_support::Int,
}
impl<TerraneType54> Generic<TerraneType54> {
    pub fn terrane_construct(
        direct: TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
            TerraneType54,
        >,
        nested: Nested<TerraneType54>,
        values: terrane_collection_support::List<TerraneType54>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            direct: None,
            nested: None,
            values: terrane_collection_support::List::<TerraneType54>::new(Vec::new()),
            fixed: terrane_int_support::Int::from(19_i128),
        };
        __terrane_constructed_value.construct(direct, nested, values);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        direct: TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
            TerraneType54,
        >,
        nested: Nested<TerraneType54>,
        values: terrane_collection_support::List<TerraneType54>,
    ) {
        {
            let __terrane_value = direct;
            self.direct = Some(__terrane_value);
        }
        {
            let __terrane_value = nested;
            self.nested = Some(__terrane_value);
        }
        {
            let __terrane_value = values;
            self.values = __terrane_value;
        }
    }
}
pub trait TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol,
    >;
    fn base_value(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365(
    Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol,
    >,
);
impl Clone
for TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365 {
    pub fn base_value(&self) -> terrane_int_support::Int {
        self.0.base_value()
    }
}
pub trait TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol: Send + Sync + TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
    >;
    fn sub_value(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562(
    Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
    >,
);
impl From<
    TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562,
>
for TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365 {
    fn from(
        value: TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562 {
    pub fn base_value(&self) -> terrane_int_support::Int {
        self.0.base_value()
    }
    pub fn sub_value(&self) -> terrane_int_support::Int {
        self.0.sub_value()
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D {}
impl TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn base_value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(11_i128);
    }
    pub fn sub_value(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(13_i128);
    }
}
impl TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol
for TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn sub_value(&self) -> terrane_int_support::Int {
        TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D::sub_value(
            &*self,
        )
    }
}
impl From<
    TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D,
>
for TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562 {
    fn from(
        value: TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol
for TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn base_value(&self) -> terrane_int_support::Int {
        TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D::base_value(
            &*self,
        )
    }
}
impl From<
    TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D,
>
for TerraneNsGenericConditionalCloneAndInheritedInterfaceBaseIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E746572666163650062617365 {
    fn from(
        value: TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn target_terrane_generic_conditional_clone_and_inherited_interface<
    TerraneType54: TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
>(value: TerraneType54) -> terrane_int_support::Int {
    let _ = &value;
    return terrane_int_support::Int::from(17_i128);
}
fn forward_terrane_generic_conditional_clone_and_inherited_interface<
    TerraneType55: TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
>(value: TerraneType55) -> terrane_int_support::Int {
    let callback: std::sync::Arc<
        dyn Fn(TerraneType55) -> terrane_int_support::Int + Send + Sync,
    >;
    callback = std::sync::Arc::new(
        target_terrane_generic_conditional_clone_and_inherited_interface,
    );
    return callback(value);
}
fn constrained<
    TerraneType55: TerraneNsGenericConditionalCloneAndInheritedInterfaceSubIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500737562Protocol,
>(value: TerraneType55) -> terrane_int_support::Int {
    return value.base_value();
}
fn test_scenario_terrane_generic_conditional_clone_and_inherited_interface() {
    let direct: TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
        terrane_int_support::Int,
    >;
    let leaf: TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
        terrane_int_support::Int,
    >;
    let inner_wrapper: Wrap<terrane_int_support::Int>;
    let nested_box: TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78<
        Wrap<terrane_int_support::Int>,
    >;
    let outer_wrapper: Wrap<Wrap<terrane_int_support::Int>>;
    let nested_value: Nested<terrane_int_support::Int>;
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let generic: Generic<terrane_int_support::Int>;
    let copied: Generic<terrane_int_support::Int>;
    let listed: terrane_collection_support::List<Generic<terrane_int_support::Int>>;
    let nested: Nested<terrane_int_support::Int>;
    let copy_nested: Nested<terrane_int_support::Int>;
    direct = TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78::<
        terrane_int_support::Int,
    >::terrane_construct(terrane_int_support::Int::from(5_i128));
    leaf = TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78::<
        terrane_int_support::Int,
    >::terrane_construct(terrane_int_support::Int::from(6_i128));
    inner_wrapper = Wrap::<terrane_int_support::Int>::terrane_construct(leaf);
    nested_box = TerraneNsGenericConditionalCloneAndInheritedInterfaceBoxIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E7465726661636500426F78::<
        Wrap<terrane_int_support::Int>,
    >::terrane_construct(inner_wrapper);
    outer_wrapper = Wrap::<
        Wrap<terrane_int_support::Int>,
    >::terrane_construct(nested_box);
    nested_value = Nested::<terrane_int_support::Int>::terrane_construct(outer_wrapper);
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(7_i128)]);
    generic = Generic::<
        terrane_int_support::Int,
    >::terrane_construct(direct, nested_value, values);
    copied = generic.clone();
    listed = terrane_collection_support::List::<
        Generic<terrane_int_support::Int>,
    >::new(vec![generic.clone()]);
    nested = generic.nested.as_ref().expect("required field initialized").clone();
    copy_nested = nested;
    println!(
        "{}", terrane_scalar_support::scalar_text(&copied.direct.as_ref()
        .expect("required field initialized").value.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        26 /* terrane-site: tests/integration/generic-conditional-clone-and-inherited-interface/case.trn:56:12-56:21 */);
        let __terrane_index_place_1 = __terrane_raised(listed.get(__terrane_index_0)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
        26 /* terrane-site: tests/integration/generic-conditional-clone-and-inherited-interface/case.trn:56:12-56:21 */);
        __terrane_raised(__terrane_index_place_1.values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        27 /* terrane-site: tests/integration/generic-conditional-clone-and-inherited-interface/case.trn:56:12-56:31 */)),
        27 /* terrane-site: tests/integration/generic-conditional-clone-and-inherited-interface/case.trn:56:12-56:31 */)
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&copy_nested.value.as_ref()
        .expect("required field initialized").value.as_ref()
        .expect("required field initialized").value.as_ref()
        .expect("required field initialized").value.as_ref()
        .expect("required field initialized").value.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&constrained(TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D::terrane_construct()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&forward_terrane_generic_conditional_clone_and_inherited_interface(TerraneNsGenericConditionalCloneAndInheritedInterfaceItemIdentity2F67656E657269632D636F6E646974696F6E616C2D636C6F6E652D616E642D696E686572697465642D696E74657266616365004974656D::terrane_construct()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&copied.fixed));
}
// Source: tests/integration/generic-field-cloneable-control/case.trn
// Namespace: generic-field-cloneable-control
#[derive(Clone)]
pub struct F<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub number: terrane_int_support::Int,
}
impl<TerraneType54> F<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            number: terrane_int_support::Int::from(3_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericFieldCloneableControlGIdentity2F67656E657269632D6669656C642D636C6F6E6561626C652D636F6E74726F6C0047 {
    pub value: Option<F<String>>,
}
impl TerraneNsGenericFieldCloneableControlGIdentity2F67656E657269632D6669656C642D636C6F6E6561626C652D636F6E74726F6C0047 {
    pub fn terrane_construct(value: F<String>) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: F<String>) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_generic_field_cloneable_control() {
    let original: TerraneNsGenericFieldCloneableControlGIdentity2F67656E657269632D6669656C642D636C6F6E6561626C652D636F6E74726F6C0047;
    let mut copied: TerraneNsGenericFieldCloneableControlGIdentity2F67656E657269632D6669656C642D636C6F6E6561626C652D636F6E74726F6C0047;
    original = TerraneNsGenericFieldCloneableControlGIdentity2F67656E657269632D6669656C642D636C6F6E6561626C652D636F6E74726F6C0047::terrane_construct(
        F::<String>::terrane_construct(),
    );
    copied = original.clone();
    {
        let __terrane_value = terrane_int_support::Int::from(9_i128);
        copied.value.as_mut().expect("required field initialized").number = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&original.value.as_ref()
        .expect("required field initialized").number)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&copied.value.as_ref()
        .expect("required field initialized").number)
    );
}
// Source: tests/integration/generic-index-borrow-controls/case.trn
// Namespace: generic-index-borrow-controls
pub trait TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol,
    >;
    fn report(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564(
    Box<
        dyn TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564 {
    pub fn report(&self) -> terrane_int_support::Int {
        self.0.report()
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D {}
impl TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn report(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128);
    }
}
impl TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol
for TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn report(&self) -> terrane_int_support::Int {
        TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D::report(
            &*self,
        )
    }
}
impl From<
    TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D,
>
for TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564 {
    fn from(
        value: TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn first_terrane_generic_index_borrow_controls<
    TerraneType54: TerraneNsGenericIndexBorrowControlsNamedIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73006E616D6564Protocol,
>(items: terrane_collection_support::List<TerraneType54>) -> terrane_int_support::Int {
    return {
        let __terrane_index_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                28 /* terrane-site: tests/integration/generic-index-borrow-controls/case.trn:9:12-9:20 */,
        );
        let __terrane_index_place_1 = __terrane_raised(
            items
                .get(__terrane_index_0)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index_0,
                )),

                28 /* terrane-site: tests/integration/generic-index-borrow-controls/case.trn:9:12-9:20 */,
        );
        __terrane_index_place_1.report()
    };
}
fn observe_terrane_generic_index_borrow_controls<TerraneType54>(
    value: &TerraneType54,
) -> terrane_int_support::Int {
    let _ = &value;
    return terrane_int_support::Int::from(9_i128);
}
fn borrowed_first<TerraneType54>(
    items: terrane_collection_support::List<TerraneType54>,
) -> terrane_int_support::Int {
    return observe_terrane_generic_index_borrow_controls({
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                29 /* terrane-site: tests/integration/generic-index-borrow-controls/case.trn:13:21-13:29 */,
        );
        __terrane_raised(
            items
                .get(__terrane_index)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index,
                )),

                29 /* terrane-site: tests/integration/generic-index-borrow-controls/case.trn:13:21-13:29 */,
        )
    });
}
fn test_scenario_terrane_generic_index_borrow_controls() {
    let items: terrane_collection_support::List<
        TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D,
    >;
    items = terrane_collection_support::List::<
        TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D,
    >::new(
        vec![
            TerraneNsGenericIndexBorrowControlsItemIdentity2F67656E657269632D696E6465782D626F72726F772D636F6E74726F6C73004974656D::terrane_construct()
        ],
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&first_terrane_generic_index_borrow_controls(items
        .clone()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&borrowed_first(items)));
}
// Source: tests/integration/generic-interface-implementation-bound/case.trn
// Namespace: generic-interface-implementation-bound
pub trait TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564(
    Box<
        dyn TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
pub trait DescribedProtocol<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
>: Send + Sync {
    fn clone_box(&self) -> Box<dyn DescribedProtocol<TerraneType54>>;
    fn separate_box(&self) -> Box<dyn DescribedProtocol<TerraneType54>>;
    fn describe(&self) -> terrane_int_support::Int;
}
impl<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> Clone for Box<dyn DescribedProtocol<TerraneType54>> {
    fn clone(&self) -> Self {
        DescribedProtocol::clone_box(&**self)
    }
}
pub struct Described<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
>(
    Box<dyn DescribedProtocol<TerraneType54>>,
);
impl<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> Clone for Described<TerraneType54> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> Described<TerraneType54> {
    pub fn describe(&self) -> terrane_int_support::Int {
        self.0.describe()
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D {}
impl TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
}
impl TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol
for TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D,
>
for TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564 {
    fn from(
        value: TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
    pub fn describe(&self) -> terrane_int_support::Int {
        return self.value.as_ref().expect("required field initialized").name();
    }
}
impl<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> DescribedProtocol<TerraneType54>
for TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
    TerraneType54,
>
where
    TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
        TerraneType54,
    >: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn DescribedProtocol<TerraneType54>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DescribedProtocol<TerraneType54>> {
        Box::new(self.clone())
    }
    fn describe(&self) -> terrane_int_support::Int {
        TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78::describe(
            &*self,
        )
    }
}
impl<
    TerraneType54: TerraneNsGenericInterfaceImplementationBoundNamedIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64006E616D6564Protocol,
> From<
    TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
        TerraneType54,
    >,
> for Described<TerraneType54>
where
    TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
        TerraneType54,
    >: Clone + Send + Sync + 'static,
{
    fn from(
        value: TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
            TerraneType54,
        >,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_generic_interface_implementation_bound() {
    let __trn_626f78: TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78<
        TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D,
    >;
    __trn_626f78 = TerraneNsGenericInterfaceImplementationBoundBoxIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E6400426F78::<
        TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D,
    >::terrane_construct(
        TerraneNsGenericInterfaceImplementationBoundItemIdentity2F67656E657269632D696E746572666163652D696D706C656D656E746174696F6E2D626F756E64004974656D::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&__trn_626f78.describe()));
}
// Source: tests/integration/generic-method-nested-result/case.trn
// Namespace: generic-method-nested-result
#[derive(Clone)]
pub struct TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
    pub fn take(mut self) -> TerraneType54 {
        return self.value.take().expect("required field initialized");
    }
}
#[derive(Clone)]
pub struct Factory {}
impl Factory {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn both<TerraneType42>(
        &self,
        value: TerraneType42,
    ) -> TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        TerraneType42,
    > {
        return TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78::<
            TerraneType42,
        >::terrane_construct(value);
    }
    pub fn direct<TerraneType42>(&self, value: TerraneType42) -> TerraneType42 {
        return value;
    }
    pub fn terrane_static_static_both<TerraneType42>(
        value: TerraneType42,
    ) -> TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        TerraneType42,
    > {
        return TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78::<
            TerraneType42,
        >::terrane_construct(value);
    }
}
fn free_both<TerraneType42>(
    value: TerraneType42,
) -> TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
    TerraneType42,
> {
    return TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78::<
        TerraneType42,
    >::terrane_construct(value);
}
#[allow(dead_code)]
fn unbox<TerraneType54>(
    __trn_626f78: TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        TerraneType54,
    >,
) -> TerraneType54 {
    return __trn_626f78.take();
}
fn test_scenario_terrane_generic_method_nested_result() {
    let factory: Factory;
    let inferred: TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        String,
    >;
    let typed: TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        String,
    >;
    let explicit: TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        String,
    >;
    let free: TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        String,
    >;
    let __trn_737461746963: TerraneNsGenericMethodNestedResultBoxIdentity2F67656E657269632D6D6574686F642D6E65737465642D726573756C7400426F78<
        String,
    >;
    factory = Factory::terrane_construct();
    inferred = factory.both(String::from("inferred"));
    println!("{}", terrane_scalar_support::scalar_text(&inferred.take()));
    typed = factory.both(String::from("typed"));
    println!("{}", terrane_scalar_support::scalar_text(&typed.take()));
    explicit = factory.both::<String>(String::from("explicit"));
    println!("{}", terrane_scalar_support::scalar_text(&explicit.take()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&factory
        .direct(String::from("direct")))
    );
    free = free_both(String::from("free"));
    println!("{}", terrane_scalar_support::scalar_text(&free.take()));
    __trn_737461746963 = Factory::terrane_static_static_both(String::from("static"));
    println!("{}", terrane_scalar_support::scalar_text(&__trn_737461746963.take()));
}
// Source: tests/integration/generic-mutable-explicit-application/case.trn
// Namespace: generic-mutable-explicit-application
#[derive(Clone)]
pub struct TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct P {
    pub n: terrane_int_support::Int,
}
impl P {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn both<TerraneType54>(
        &mut self,
        value: TerraneType54,
    ) -> TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
        TerraneType54,
    > {
        {
            let __terrane_value = self.n.clone()
                + terrane_int_support::Int::from(1_i128);
            self.n = __terrane_value;
        }
        return TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278::<
            TerraneType54,
        >::terrane_construct(value);
    }
    pub fn again(&mut self) -> String {
        let result: TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
            String,
        >;
        result = self.both::<String>(String::from("this"));
        return result.value.as_ref().expect("required field initialized").clone();
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericMutableExplicitApplicationQIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E0051 {
    pub p: Option<P>,
}
impl TerraneNsGenericMutableExplicitApplicationQIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E0051 {
    pub fn terrane_construct(p: P) -> Self {
        let mut __terrane_constructed_value = Self { p: None };
        __terrane_constructed_value.construct(p);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, p: P) {
        {
            let __terrane_value = p;
            self.p = Some(__terrane_value);
        }
    }
}
fn borrowed(p: &mut P) -> String {
    let result: TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
        String,
    >;
    result = p.both::<String>(String::from("borrowed"));
    return result.value.as_ref().expect("required field initialized").clone();
}
fn test_scenario_terrane_generic_mutable_explicit_application() {
    let mut p: P;
    let plain: TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
        String,
    >;
    let mut q: TerraneNsGenericMutableExplicitApplicationQIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E0051;
    let nested: TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
        String,
    >;
    let mut later: Option<P> = None;
    let hoisted: TerraneNsGenericMutableExplicitApplicationBxIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E004278<
        String,
    >;
    p = P::terrane_construct();
    plain = p.both::<String>(String::from("plain"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&plain.value.as_ref()
        .expect("required field initialized"))
    );
    println!("{}", terrane_scalar_support::scalar_text(&p.n));
    q = TerraneNsGenericMutableExplicitApplicationQIdentity2F67656E657269632D6D757461626C652D6578706C696369742D6170706C69636174696F6E0051::terrane_construct(
        P::terrane_construct(),
    );
    nested = q
        .p
        .as_mut()
        .expect("required field initialized")
        .both::<String>(String::from("nested"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&nested.value.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&q.p.as_ref()
        .expect("required field initialized").n)
    );
    println!("{}", terrane_scalar_support::scalar_text(&borrowed(&mut p)));
    println!("{}", terrane_scalar_support::scalar_text(&p.n));
    println!("{}", terrane_scalar_support::scalar_text(&p.again()));
    println!("{}", terrane_scalar_support::scalar_text(&p.n));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = later.insert(P::terrane_construct());
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    let _ = later.insert(P::terrane_construct());
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    hoisted = later
        .as_mut()
        .expect("flow-proven available binding")
        .both::<String>(String::from("hoisted"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&hoisted.value.as_ref()
        .expect("required field initialized"))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&later.as_ref()
        .expect("flow-proven available binding").clone().n)
    );
}
// Source: tests/integration/generic-required-field-consuming/case.trn
// Namespace: generic-required-field-consuming
#[derive(Clone)]
pub struct V<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<TerraneType54> V<TerraneType54> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
    pub fn done(self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(3_i128);
    }
}
#[derive(Clone)]
pub struct R {
    pub f: Option<V<terrane_int_support::Int>>,
}
impl R {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { f: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = V::<
                terrane_int_support::Int,
            >::terrane_construct(terrane_int_support::Int::from(7_i128));
            self.f = Some(__terrane_value);
        }
    }
}
pub trait TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
    >;
    fn name(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564(
    Box<
        dyn TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D {}
impl TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
impl TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol
for TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D::name(
            &*self,
        )
    }
}
impl From<
    TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D,
>
for TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564 {
    fn from(
        value: TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericRequiredFieldConsumingInnerIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E6700496E6E6572<
    TerraneType54: TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54: TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
> TerraneNsGenericRequiredFieldConsumingInnerIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E6700496E6E6572<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
    pub fn done(self) -> terrane_int_support::Int {
        return self.value.as_ref().expect("required field initialized").name();
    }
}
#[derive(Clone)]
pub struct TerraneNsGenericRequiredFieldConsumingOuterIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004F75746572<
    TerraneType54: TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub inner: Option<
        TerraneNsGenericRequiredFieldConsumingInnerIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E6700496E6E6572<
            TerraneType54,
        >,
    >,
}
impl<
    TerraneType54: TerraneNsGenericRequiredFieldConsumingNamedIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67006E616D6564Protocol,
> TerraneNsGenericRequiredFieldConsumingOuterIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004F75746572<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            inner: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = TerraneNsGenericRequiredFieldConsumingInnerIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E6700496E6E6572::<
                TerraneType54,
            >::terrane_construct(value);
            self.inner = Some(__terrane_value);
        }
    }
    pub fn done(mut self) -> terrane_int_support::Int {
        return self.inner.take().expect("required field initialized").done();
    }
}
fn test_scenario_terrane_generic_required_field_consuming() {
    let mut r: R;
    let generic: TerraneNsGenericRequiredFieldConsumingOuterIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004F75746572<
        TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D,
    >;
    r = R::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&r.f.take()
        .expect("required field initialized").done())
    );
    generic = TerraneNsGenericRequiredFieldConsumingOuterIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004F75746572::<
        TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D,
    >::terrane_construct(
        TerraneNsGenericRequiredFieldConsumingItemIdentity2F67656E657269632D72657175697265642D6669656C642D636F6E73756D696E67004974656D::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&generic.done()));
}
// Source: tests/integration/grouped-consuming-field-and-nested-generic/case.trn
// Namespace: grouped-consuming-field-and-nested-generic
pub trait TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564Protocol: Send + Sync {
    fn name(&self) -> terrane_int_support::Int;
    fn finish(self: Box<Self>) -> terrane_int_support::Int;
}
pub struct TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564(
    Box<
        dyn TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564Protocol,
    >,
);
impl TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564 {
    pub fn name(&self) -> terrane_int_support::Int {
        self.0.name()
    }
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
}
#[derive(Clone)]
pub struct TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D {
    pub amount: terrane_int_support::Int,
}
impl TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D {
    pub fn terrane_construct() -> Self {
        Self {
            amount: terrane_int_support::Int::from(7_i128),
        }
    }
    pub fn name(&self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.amount.clone();
    }
}
impl TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564Protocol
for TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D {
    fn name(&self) -> terrane_int_support::Int {
        TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D::name(
            &*self,
        )
    }
    fn finish(self: Box<Self>) -> terrane_int_support::Int {
        TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D::finish(
            *self,
        )
    }
}
impl From<
    TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D,
>
for TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564 {
    fn from(
        value: TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsGroupedConsumingFieldAndNestedGenericInnerIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E6572696300496E6E6572<
    TerraneType54: TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54: TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564Protocol,
> TerraneNsGroupedConsumingFieldAndNestedGenericInnerIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E6572696300496E6E6572<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsGroupedConsumingFieldAndNestedGenericOuterIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004F75746572<
    TerraneType54: TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub inner: Option<
        TerraneNsGroupedConsumingFieldAndNestedGenericInnerIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E6572696300496E6E6572<
            TerraneType54,
        >,
    >,
}
impl<
    TerraneType54: TerraneNsGroupedConsumingFieldAndNestedGenericNamedIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963006E616D6564Protocol,
> TerraneNsGroupedConsumingFieldAndNestedGenericOuterIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004F75746572<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            inner: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = TerraneNsGroupedConsumingFieldAndNestedGenericInnerIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E6572696300496E6E6572::<
                TerraneType54,
            >::terrane_construct(value);
            self.inner = Some(__terrane_value);
        }
    }
    pub fn finish(mut self) -> terrane_int_support::Int {
        return Box::new(
                self
                    .inner
                    .as_mut()
                    .expect("required field initialized")
                    .value
                    .take()
                    .expect("required field initialized"),
            )
            .finish();
    }
    pub fn finish_via_local(mut self) -> terrane_int_support::Int {
        let mut nested: TerraneNsGroupedConsumingFieldAndNestedGenericInnerIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E6572696300496E6E6572<
            TerraneType54,
        >;
        nested = self.inner.take().expect("required field initialized");
        return Box::new(nested.value.take().expect("required field initialized"))
            .finish();
    }
}
fn test_scenario_terrane_grouped_consuming_field_and_nested_generic() {
    let item: TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D;
    item = TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D::terrane_construct();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&TerraneNsGroupedConsumingFieldAndNestedGenericOuterIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004F75746572::
        <
        TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D
        > ::terrane_construct(item).finish())
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&TerraneNsGroupedConsumingFieldAndNestedGenericOuterIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004F75746572::
        <
        TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D
        >
        ::terrane_construct(TerraneNsGroupedConsumingFieldAndNestedGenericItemIdentity2F67726F757065642D636F6E73756D696E672D6669656C642D616E642D6E65737465642D67656E65726963004974656D::terrane_construct())
        .finish_via_local())
    );
}
// Source: tests/integration/implicit-class-field-defaults/case.trn
// Namespace: implicit-class-field-defaults
pub static TERRANE_STATIC_TERRANENSIMPLICITCLASSFIELDDEFAULTSDEFAULTSIDENTITY2F696D706C696369742D636C6173732D6669656C642D64656661756C74730064656661756C7473_SHARED: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct TerraneNsImplicitClassFieldDefaultsDefaultsIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C74730064656661756C7473 {
    pub message: String,
    pub path: String,
    pub id: terrane_int_support::Int,
    pub active: bool,
    pub something: i8,
    pub ratio: f32,
    pub data: Vec<u8>,
    pub note: Option<String>,
    pub items: terrane_collection_support::List<terrane_int_support::Int>,
    pub names: terrane_collection_support::Map<String, terrane_int_support::Int>,
    pub tags: terrane_collection_support::Set<String>,
    pub unordered_names: terrane_collection_support::UnorderedMap<
        String,
        terrane_int_support::Int,
    >,
    pub unordered_tags: terrane_collection_support::UnorderedSet<String>,
    pub explicit: terrane_int_support::Int,
}
impl TerraneNsImplicitClassFieldDefaultsDefaultsIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C74730064656661756C7473 {
    pub fn terrane_construct(message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            message: String::new(),
            path: String::new(),
            id: terrane_int_support::Int::from(0_i128),
            active: false,
            something: 0,
            ratio: 0.0_f32,
            data: Vec::new(),
            note: None,
            items: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            names: terrane_collection_support::Map::<
                String,
                terrane_int_support::Int,
            >::new(Vec::new()),
            tags: terrane_collection_support::Set::<String>::new(Vec::new()),
            unordered_names: terrane_collection_support::UnorderedMap::<
                String,
                terrane_int_support::Int,
            >::new(Vec::new()),
            unordered_tags: terrane_collection_support::UnorderedSet::<
                String,
            >::new(Vec::new()),
            explicit: terrane_int_support::Int::from(7_i128),
        };
        __terrane_constructed_value.construct(message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, message: String) {
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsImplicitClassFieldDefaultsPlainIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C747300706C61696E {
    pub count: terrane_int_support::Int,
    pub text: String,
}
impl TerraneNsImplicitClassFieldDefaultsPlainIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C747300706C61696E {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(0_i128),
            text: String::new(),
        }
    }
}
fn test_scenario_terrane_implicit_class_field_defaults() {
    let value: TerraneNsImplicitClassFieldDefaultsDefaultsIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C74730064656661756C7473;
    let descriptor: TerraneDescriptor;
    let plain_value: TerraneNsImplicitClassFieldDefaultsPlainIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C747300706C61696E;
    value = TerraneNsImplicitClassFieldDefaultsDefaultsIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C74730064656661756C7473::terrane_construct(
        String::from("ready"),
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&value.message),
        String::from(":"), terrane_scalar_support::scalar_text(&value.path),
        String::from(":"), terrane_scalar_support::scalar_text(&value.id),
        String::from(":"), terrane_scalar_support::scalar_text(&value.active),
        String::from(":"), terrane_scalar_support::scalar_text(&value.something),
        String::from(":"), terrane_scalar_support::scalar_text(&value.ratio)
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(value.data.len()
        as i128)), String::from(":"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.items
        .length())), String::from(":"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.names
        .length())), String::from(":"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.tags
        .length())), String::from(":"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value
        .unordered_names.length())), String::from(":"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value
        .unordered_tags.length()))
    );
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(&value.note).is_none()),
        String::from(":"), terrane_scalar_support::scalar_text(&value.explicit),
        String::from(":"), { let __terrane_result =
        terrane_scalar_support::scalar_text(&*
        TERRANE_STATIC_TERRANENSIMPLICITCLASSFIELDDEFAULTSDEFAULTSIDENTITY2F696D706C696369742D636C6173732D6669656C642D64656661756C74730064656661756C7473_SHARED
        .lock().expect("static field lock poisoned")); __terrane_result }
    );
    descriptor = TerraneDescriptor {
        identity: "/implicit-class-field-defaults::defaults",
        name: "defaults",
        kind: "class",
        inherently_identity_bearing: false,
        fields: &[
            TerraneFieldMetadata {
                name: "message",
                external_name: "message",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "path",
                external_name: "path",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "id",
                external_name: "id",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "active",
                external_name: "active",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "something",
                external_name: "something",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "ratio",
                external_name: "ratio",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "data",
                external_name: "data",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "note",
                external_name: "note",
                defaulted: true,
                optional: true,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "items",
                external_name: "items",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "names",
                external_name: "names",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "tags",
                external_name: "tags",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "unordered-names",
                external_name: "unordered-names",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "unordered-tags",
                external_name: "unordered-tags",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "explicit",
                external_name: "explicit",
                defaulted: true,
                optional: false,
                secret: false,
            },
        ],
    };
    plain_value = TerraneNsImplicitClassFieldDefaultsPlainIdentity2F696D706C696369742D636C6173732D6669656C642D64656661756C747300706C61696E::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&plain_value.count),
        String::from(":"), terrane_scalar_support::scalar_text(&plain_value.text)
    );
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(descriptor
        .fields.len() as i128)), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
        &terrane_collection_support::List::new(descriptor.fields.iter().map(| field |
        field.defaulted).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        30 /* terrane-site: tests/integration/implicit-class-field-defaults/case.trn:38:39-38:68 */)),
        30 /* terrane-site: tests/integration/implicit-class-field-defaults/case.trn:38:39-38:68 */) }),
        String::from(":"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &terrane_collection_support::List::new(descriptor
        .fields.iter().map(| field | field.defaulted).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(13_i128)),
        31 /* terrane-site: tests/integration/implicit-class-field-defaults/case.trn:38:75-38:105 */)),
        31 /* terrane-site: tests/integration/implicit-class-field-defaults/case.trn:38:75-38:105 */) })
    );
}
// Source: tests/integration/indirect-recursive-class-type/case.trn
// Namespace: indirect-recursive-class-type
#[derive(Clone)]
pub struct TerraneNsIndirectRecursiveClassTypeNodeIdentity2F696E6469726563742D7265637572736976652D636C6173732D74797065004E6F6465 {
    pub children: terrane_collection_support::List<
        TerraneNsIndirectRecursiveClassTypeNodeIdentity2F696E6469726563742D7265637572736976652D636C6173732D74797065004E6F6465,
    >,
}
impl TerraneNsIndirectRecursiveClassTypeNodeIdentity2F696E6469726563742D7265637572736976652D636C6173732D74797065004E6F6465 {
    pub fn terrane_construct() -> Self {
        Self {
            children: terrane_collection_support::List::<
                TerraneNsIndirectRecursiveClassTypeNodeIdentity2F696E6469726563742D7265637572736976652D636C6173732D74797065004E6F6465,
            >::new(Vec::new()),
        }
    }
}
fn test_scenario_terrane_indirect_recursive_class_type() {
    let node: TerraneNsIndirectRecursiveClassTypeNodeIdentity2F696E6469726563742D7265637572736976652D636C6173732D74797065004E6F6465;
    node = TerraneNsIndirectRecursiveClassTypeNodeIdentity2F696E6469726563742D7265637572736976652D636C6173732D74797065004E6F6465::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(node
        .children.length()))
    );
}
// Source: tests/integration/inherited-ancestor-library/case.trn
// Namespace: inherited-ancestor-library
pub trait TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
    >;
    fn b(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365(
    Box<
        dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
    >,
);
impl Clone
for TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365 {
    pub fn b(&self) -> terrane_int_support::Int {
        self.0.b()
    }
}
pub trait TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol: Send + Sync + TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
    >;
    fn other(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562(
    Box<
        dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
    >,
);
impl From<
    TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562,
>
for TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365 {
    fn from(
        value: TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562 {
    pub fn b(&self) -> terrane_int_support::Int {
        self.0.b()
    }
    pub fn other(&self) -> terrane_int_support::Int {
        self.0.other()
    }
}
// Source: tests/integration/inherited-base-fields/case.trn
// Namespace: inherited-base-fields
#[derive(Clone)]
pub struct TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365Storage {
    pub value: terrane_int_support::Int,
}
impl TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365Storage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
        }
    }
}
#[derive(Clone)]
pub enum TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365 {
    Own(
        TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365Storage,
    ),
    TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64(
        TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64,
    ),
}
impl TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365Storage::terrane_construct(),
        )
    }
    pub fn terrane_field_value(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.value,
            Self::TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64(
                value,
            ) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.value,
            Self::TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64(
                value,
            ) => &mut value.value,
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64 {
    pub value: terrane_int_support::Int,
    pub extra: terrane_int_support::Int,
}
impl TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(2_i128),
            extra: terrane_int_support::Int::from(3_i128),
        }
    }
}
fn test_scenario_terrane_inherited_base_fields() {
    let concrete: TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64;
    let mut view: TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365;
    concrete = TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64::terrane_construct();
    view = TerraneNsInheritedBaseFieldsBaseIdentity2F696E686572697465642D626173652D6669656C64730062617365::TerraneNsInheritedBaseFieldsChildIdentity2F696E686572697465642D626173652D6669656C6473006368696C64(
        concrete,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&view.terrane_field_value().clone())
    );
    {
        let __terrane_value = terrane_int_support::Int::from(9_i128);
        *view.terrane_field_value_mut() = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&view.terrane_field_value().clone())
    );
}
// Source: tests/integration/inherited-generic-bound-forwarding/case.trn
// Namespace: inherited-generic-bound-forwarding
pub trait TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
    TerraneType54,
>: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
            TerraneType54,
        >,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
            TerraneType54,
        >,
    >;
    fn b(&self) -> TerraneType54;
}
impl<TerraneType54> Clone
for Box<
    dyn TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
        TerraneType54,
    >,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365<
    TerraneType54,
>(
    Box<
        dyn TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
            TerraneType54,
        >,
    >,
);
impl<TerraneType54> Clone
for TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365<
    TerraneType54,
> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<
    TerraneType54,
> TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365<
    TerraneType54,
> {
    pub fn b(&self) -> TerraneType54 {
        self.0.b()
    }
}
pub trait TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol: Send + Sync + TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
        String,
    > {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
    >;
    fn other(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562(
    Box<
        dyn TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
    >,
);
impl From<
    TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562,
>
for TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365<
    String,
> {
    fn from(
        value: TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562 {
    pub fn b(&self) -> String {
        self.0.b()
    }
    pub fn other(&self) -> terrane_int_support::Int {
        self.0.other()
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053 {}
impl TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn b(&self) -> String {
        return String::from("x");
    }
    pub fn other(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol
for TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn other(&self) -> terrane_int_support::Int {
        TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053::other(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053,
>
for TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562 {
    fn from(
        value: TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
    String,
>
for TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
            String,
        >,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365Protocol<
            String,
        >,
    > {
        Box::new(self.clone())
    }
    fn b(&self) -> String {
        TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053::b(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053,
>
for TerraneNsInheritedGenericBoundForwardingBaseIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670062617365<
    String,
> {
    fn from(
        value: TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn up_terrane_inherited_generic_bound_forwarding<
    TerraneType55: TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
>(x: TerraneType55) -> terrane_int_support::Int {
    let _ = &x;
    return terrane_int_support::Int::from(1_i128);
}
fn go_terrane_inherited_generic_bound_forwarding<
    TerraneType54: TerraneNsInheritedGenericBoundForwardingSubIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E6700737562Protocol,
>(x: TerraneType54) -> terrane_int_support::Int {
    return up_terrane_inherited_generic_bound_forwarding(x);
}
fn test_scenario_terrane_inherited_generic_bound_forwarding() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_inherited_generic_bound_forwarding(TerraneNsInheritedGenericBoundForwardingSIdentity2F696E686572697465642D67656E657269632D626F756E642D666F7277617264696E670053::terrane_construct()))
    );
}
// Source: tests/integration/inherited-interface-bounded-dispatch/case.trn
// Namespace: inherited-interface-bounded-dispatch
pub trait TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365Protocol: Send + Sync {
    fn read(&self) -> terrane_int_support::Int;
    fn increment(&mut self) -> terrane_int_support::Int;
    fn finish(self: Box<Self>) -> terrane_int_support::Int;
}
pub struct TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365(
    Box<
        dyn TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365Protocol,
    >,
);
impl TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365 {
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        self.0.increment()
    }
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
}
pub trait TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol: Send + Sync + TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365Protocol {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
    >;
    fn extra(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562(
    Box<
        dyn TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
    >,
);
impl From<
    TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562,
>
for TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365 {
    fn from(
        value: TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562 {
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        self.0.increment()
    }
    pub fn finish(self) -> terrane_int_support::Int {
        self.0.finish()
    }
    pub fn extra(&self) -> terrane_int_support::Int {
        self.0.extra()
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D {
    pub value: terrane_int_support::Int,
}
impl TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn extra(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(9_i128);
    }
}
impl TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol
for TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn extra(&self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D::extra(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D,
>
for TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562 {
    fn from(
        value: TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365Protocol
for TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D {
    fn read(&self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D::read(
            &*self,
        )
    }
    fn increment(&mut self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D::increment(
            &mut *self,
        )
    }
    fn finish(self: Box<Self>) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D::finish(
            *self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D,
>
for TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365 {
    fn from(
        value: TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn forward_terrane_inherited_interface_bounded_dispatch<
    TerraneType54: TerraneNsInheritedInterfaceBoundedDispatchBaseIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D64697370617463680062617365Protocol,
>(value: TerraneType54) -> terrane_int_support::Int {
    return value.read();
}
fn inspect_terrane_inherited_interface_bounded_dispatch<
    TerraneType54: TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
>(values: terrane_collection_support::List<TerraneType54>) -> terrane_int_support::Int {
    return {
        let __terrane_index_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                32 /* terrane-site: tests/integration/inherited-interface-bounded-dispatch/case.trn:23:12-23:21 */,
        );
        let __terrane_index_place_1 = __terrane_raised(
            values
                .get(__terrane_index_0)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index_0,
                )),

                32 /* terrane-site: tests/integration/inherited-interface-bounded-dispatch/case.trn:23:12-23:21 */,
        );
        __terrane_index_place_1.read()
    };
}
fn mutate_terrane_inherited_interface_bounded_dispatch<
    TerraneType54: TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
>(item: &mut TerraneType54) -> terrane_int_support::Int {
    return item.increment();
}
fn consume_terrane_inherited_interface_bounded_dispatch<
    TerraneType54: TerraneNsInheritedInterfaceBoundedDispatchSubIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D646973706174636800737562Protocol,
>(item: TerraneType54) -> terrane_int_support::Int {
    return Box::new(item).finish();
}
fn test_scenario_terrane_inherited_interface_bounded_dispatch() {
    let values: terrane_collection_support::List<
        TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D,
    >;
    let mut item: TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D;
    values = terrane_collection_support::List::<
        TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D,
    >::new(
        vec![
            TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D::terrane_construct()
        ],
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&inspect_terrane_inherited_interface_bounded_dispatch(values))
    );
    item = TerraneNsInheritedInterfaceBoundedDispatchItemIdentity2F696E686572697465642D696E746572666163652D626F756E6465642D6469737061746368004974656D::terrane_construct();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&mutate_terrane_inherited_interface_bounded_dispatch(&mut
        item))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&forward_terrane_inherited_interface_bounded_dispatch(item
        .clone()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&consume_terrane_inherited_interface_bounded_dispatch(item))
    );
}
// Source: tests/integration/inherited-interface-generic-ancestor/case.trn
// Namespace: inherited-interface-generic-ancestor
pub trait RootProtocol<TerraneType54>: Send + Sync {
    fn clone_box(&self) -> Box<dyn RootProtocol<TerraneType54>>;
    fn separate_box(&self) -> Box<dyn RootProtocol<TerraneType54>>;
    fn value(&self) -> TerraneType54;
}
impl<TerraneType54> Clone for Box<dyn RootProtocol<TerraneType54>> {
    fn clone(&self) -> Self {
        RootProtocol::clone_box(&**self)
    }
}
pub struct Root<TerraneType54>(Box<dyn RootProtocol<TerraneType54>>);
impl<TerraneType54> Clone for Root<TerraneType54> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType54> Root<TerraneType54> {
    pub fn value(&self) -> TerraneType54 {
        self.0.value()
    }
}
pub trait MiddleProtocol<
    TerraneType54,
>: Send + Sync + RootProtocol<terrane_collection_support::List<TerraneType54>> {
    fn clone_box(&self) -> Box<dyn MiddleProtocol<TerraneType54>>;
    fn separate_box(&self) -> Box<dyn MiddleProtocol<TerraneType54>>;
}
impl<TerraneType54> Clone for Box<dyn MiddleProtocol<TerraneType54>> {
    fn clone(&self) -> Self {
        MiddleProtocol::clone_box(&**self)
    }
}
pub struct Middle<TerraneType54>(Box<dyn MiddleProtocol<TerraneType54>>);
impl<TerraneType54> From<Middle<TerraneType54>>
for Root<terrane_collection_support::List<TerraneType54>> {
    fn from(value: Middle<TerraneType54>) -> Self {
        Self(value.0)
    }
}
impl<TerraneType54> Clone for Middle<TerraneType54> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType54> Middle<TerraneType54> {
    pub fn value(&self) -> terrane_collection_support::List<TerraneType54> {
        self.0.value()
    }
}
pub trait TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol: Send + Sync + MiddleProtocol<
        terrane_int_support::Int,
    > {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol,
    >;
}
impl Clone
for Box<
    dyn TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166(
    Box<
        dyn TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol,
    >,
);
impl From<
    TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166,
> for Middle<terrane_int_support::Int> {
    fn from(
        value: TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166,
    ) -> Self {
        Self(value.0)
    }
}
impl From<
    TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166,
> for Root<terrane_collection_support::List<terrane_int_support::Int>> {
    fn from(
        value: TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166 {
    pub fn value(&self) -> terrane_collection_support::List<terrane_int_support::Int> {
        self.0.value()
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D {}
impl TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn value(&self) -> terrane_collection_support::List<terrane_int_support::Int> {
        return terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(1_i128),
                terrane_int_support::Int::from(2_i128)
            ],
        );
    }
}
impl TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol
for TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol,
    > {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D,
>
for TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166 {
    fn from(
        value: TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl MiddleProtocol<terrane_int_support::Int>
for TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D {
    fn clone_box(&self) -> Box<dyn MiddleProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn MiddleProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D,
> for Middle<terrane_int_support::Int> {
    fn from(
        value: TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl RootProtocol<terrane_collection_support::List<terrane_int_support::Int>>
for TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D {
    fn clone_box(
        &self,
    ) -> Box<
        dyn RootProtocol<terrane_collection_support::List<terrane_int_support::Int>>,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn RootProtocol<terrane_collection_support::List<terrane_int_support::Int>>,
    > {
        Box::new(self.clone())
    }
    fn value(&self) -> terrane_collection_support::List<terrane_int_support::Int> {
        TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D::value(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D,
> for Root<terrane_collection_support::List<terrane_int_support::Int>> {
    fn from(
        value: TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn inspect_terrane_inherited_interface_generic_ancestor<
    TerraneType54: TerraneNsInheritedInterfaceGenericAncestorLeafIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004C656166Protocol,
>(value: TerraneType54) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(value.value().length());
}
fn test_scenario_terrane_inherited_interface_generic_ancestor() {
    let item: TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D;
    item = TerraneNsInheritedInterfaceGenericAncestorItemIdentity2F696E686572697465642D696E746572666163652D67656E657269632D616E636573746F72004974656D::terrane_construct();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&inspect_terrane_inherited_interface_generic_ancestor(item))
    );
}
// Source: tests/integration/inherited-interface-imported-bound/case.trn
// Namespace: inherited-interface-imported-bound
#[derive(Clone)]
pub struct TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053 {}
impl TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn b(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn other(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol
for TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn other(&self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053::other(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053,
>
for TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562 {
    fn from(
        value: TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol
for TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn b(&self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053::b(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053,
>
for TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365 {
    fn from(
        value: TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_inherited_interface_imported_bound<
    TerraneType54: TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
>(x: TerraneType54) -> terrane_int_support::Int {
    return x.b() + x.other();
}
fn test_scenario_terrane_inherited_interface_imported_bound() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_inherited_interface_imported_bound(TerraneNsInheritedInterfaceImportedBoundSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D626F756E640053::terrane_construct()))
    );
}
// Source: tests/integration/inherited-interface-imported-value/case.trn
// Namespace: inherited-interface-imported-value
#[derive(Clone)]
pub struct TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053 {}
impl TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn b(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn other(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol
for TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn other(&self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053::other(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053,
>
for TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562 {
    fn from(
        value: TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol
for TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn b(&self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053::b(
            &*self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053,
>
for TerraneNsInheritedAncestorLibraryBaseIdentity2F696E686572697465642D616E636573746F722D6C6962726172790062617365 {
    fn from(
        value: TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_inherited_interface_imported_value(
    x: TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562,
) -> terrane_int_support::Int {
    return x.b() + x.other();
}
fn test_scenario_terrane_inherited_interface_imported_value() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_inherited_interface_imported_value(<
        TerraneNsInheritedAncestorLibrarySubIdentity2F696E686572697465642D616E636573746F722D6C69627261727900737562
        >
        ::from(TerraneNsInheritedInterfaceImportedValueSIdentity2F696E686572697465642D696E746572666163652D696D706F727465642D76616C75650053::terrane_construct())))
    );
}
// Source: tests/integration/inherited-interface-mutable-receiver/case.trn
// Namespace: inherited-interface-mutable-receiver
pub trait TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol,
    >;
    fn increment(&mut self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365(
    Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol,
    >,
);
impl Clone
for TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365 {
    pub fn increment(&mut self) -> terrane_int_support::Int {
        self.0.increment()
    }
}
pub trait TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol: Send + Sync + TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol,
    >;
}
impl Clone
for Box<
    dyn TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562(
    Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol,
    >,
);
impl From<
    TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562,
>
for TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365 {
    fn from(
        value: TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562 {
    pub fn increment(&mut self) -> terrane_int_support::Int {
        self.0.increment()
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(4_i128),
        }
    }
    pub fn increment(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
}
impl TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol
for TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol,
    > {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572,
>
for TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562 {
    fn from(
        value: TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol
for TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn increment(&mut self) -> terrane_int_support::Int {
        TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572::increment(
            &mut *self,
        )
    }
}
impl From<
    TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572,
>
for TerraneNsInheritedInterfaceMutableReceiverBaseIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D72656365697665720062617365 {
    fn from(
        value: TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn update<
    TerraneType54: TerraneNsInheritedInterfaceMutableReceiverSubIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200737562Protocol,
>(value: &mut TerraneType54) -> terrane_int_support::Int {
    return value.increment();
}
fn test_scenario_terrane_inherited_interface_mutable_receiver() {
    let mut counter: TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572;
    counter = TerraneNsInheritedInterfaceMutableReceiverCounterIdentity2F696E686572697465642D696E746572666163652D6D757461626C652D726563656976657200436F756E746572::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&update(&mut counter)));
}
// Source: tests/integration/inherited-self-return/case.trn
// Namespace: inherited-self-return
#[derive(Clone)]
pub struct TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365Storage {}
impl TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365Storage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn copy(
        &self,
    ) -> TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365 {
        return TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365::Own(
            self.clone(),
        );
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
#[derive(Clone)]
pub enum TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365 {
    Own(
        TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365Storage,
    ),
    TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64(
        TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64,
    ),
}
impl TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365Storage::terrane_construct(),
        )
    }
    pub fn copy(
        &self,
    ) -> TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365 {
        match self {
            Self::Own(value) => value.copy(),
            Self::TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64(
                value,
            ) => value.copy(),
        }
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.marker(),
            Self::TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64(
                value,
            ) => value.marker(),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64 {}
impl TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn copy(
        &self,
    ) -> TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365 {
        return TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365::TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64(
            self.clone(),
        );
    }
    pub fn marker(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
fn test_scenario_terrane_inherited_self_return() {
    let value: TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64;
    let result: TerraneNsInheritedSelfReturnBaseIdentity2F696E686572697465642D73656C662D72657475726E0062617365;
    value = TerraneNsInheritedSelfReturnChildIdentity2F696E686572697465642D73656C662D72657475726E006368696C64::terrane_construct();
    result = value.copy();
    println!("{}", terrane_scalar_support::scalar_text(&result.marker()));
}
// Source: tests/integration/inherited-shared-field-writes/case.trn
// Namespace: inherited-shared-field-writes
#[derive(Clone)]
pub struct TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D {
    pub value: terrane_int_support::Int,
}
impl TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365Storage {
    pub flag: bool,
    pub count: terrane_int_support::Int,
    pub values: terrane_collection_support::List<terrane_int_support::Int>,
    pub required: Option<
        TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D,
    >,
}
impl TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365Storage {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            flag: true,
            count: terrane_int_support::Int::from(5_i128),
            values: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
            required: None,
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D::terrane_construct();
            self.required = Some(__terrane_value);
        }
    }
    pub fn update(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = amount.clone();
            self.count = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub enum TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365 {
    Own(
        TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365Storage,
    ),
    TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
        TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564,
    ),
}
impl TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365Storage::terrane_construct(),
        )
    }
    pub fn update(&mut self, amount: terrane_int_support::Int) {
        match self {
            Self::Own(value) => value.update(amount),
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => value.update(amount),
        }
    }
    pub fn terrane_field_flag(&self) -> &bool {
        match self {
            Self::Own(value) => &value.flag,
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => &value.flag,
        }
    }
    pub fn terrane_field_flag_mut(&mut self) -> &mut bool {
        match self {
            Self::Own(value) => &mut value.flag,
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => &mut value.flag,
        }
    }
    pub fn terrane_field_count(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.count,
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => &value.count,
        }
    }
    pub fn terrane_field_count_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.count,
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => &mut value.count,
        }
    }
    pub fn terrane_field_values(
        &self,
    ) -> &terrane_collection_support::List<terrane_int_support::Int> {
        match self {
            Self::Own(value) => &value.values,
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => &value.values,
        }
    }
    pub fn terrane_field_values_mut(
        &mut self,
    ) -> &mut terrane_collection_support::List<terrane_int_support::Int> {
        match self {
            Self::Own(value) => &mut value.values,
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => &mut value.values,
        }
    }
    pub fn terrane_field_required(
        &self,
    ) -> &TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D {
        match self {
            Self::Own(value) => {
                value.required.as_ref().expect("required field initialized")
            }
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => value.required.as_ref().expect("required field initialized"),
        }
    }
    pub fn terrane_field_required_mut(
        &mut self,
    ) -> &mut TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D {
        match self {
            Self::Own(value) => {
                value.required.as_mut().expect("required field initialized")
            }
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => value.required.as_mut().expect("required field initialized"),
        }
    }
    pub fn terrane_field_required_slot_mut(
        &mut self,
    ) -> &mut Option<
        TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D,
    > {
        match self {
            Self::Own(value) => &mut value.required,
            Self::TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564(
                value,
            ) => &mut value.required,
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564 {
    pub flag: bool,
    pub count: terrane_int_support::Int,
    pub values: terrane_collection_support::List<terrane_int_support::Int>,
    pub required: Option<
        TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D,
    >,
    pub extra: terrane_int_support::Int,
}
impl TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564 {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            flag: true,
            count: terrane_int_support::Int::from(5_i128),
            values: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(1_i128)]),
            required: None,
            extra: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D::terrane_construct();
            self.required = Some(__terrane_value);
        }
    }
    pub fn update(&mut self, amount: terrane_int_support::Int) {
        {
            let __terrane_value = amount.clone();
            self.count = __terrane_value;
        }
    }
}
fn assign(
    value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564,
        >,
    >,
) {
    {
        let __terrane_owner_0 = value.clone();
        let __terrane_value = terrane_int_support::Int::from(9_i128);
        __terrane_owner_0.lock().expect("shared reference lock poisoned").count = __terrane_value;
    }
}
fn test_scenario_terrane_inherited_shared_field_writes() {
    let plain: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564,
        >,
    >;
    let held: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564,
        >,
    >;
    let weak: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564,
        >,
    >;
    let other: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365,
        >,
    >;
    let other_held: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365,
        >,
    >;
    let other_observer: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365,
        >,
    >;
    let next: TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D;
    let mut replacement: TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D;
    plain = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsInheritedSharedFieldWritesDerivedIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730064657269766564::terrane_construct(),
        ),
    );
    held = plain.clone();
    {
        let __terrane_owner_1 = held.clone();
        let __terrane_value = false;
        __terrane_owner_1.lock().expect("shared reference lock poisoned").flag = __terrane_value;
    }
    {
        let __terrane_owner_2 = held.clone();
        let __terrane_value = {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        __terrane_owner_2.lock().expect("shared reference lock poisoned").count = __terrane_value;
    }
    {
        let __terrane_owner_3 = held.clone();
        let __terrane_collection_argument_0 = {
            let __terrane_snapshot = held
                .lock()
                .expect("shared reference lock poisoned")
                .count
                .clone();
            __terrane_snapshot
        };
        __terrane_owner_3
            .lock()
            .expect("shared reference lock poisoned")
            .values
            .append(__terrane_collection_argument_0);
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = {
        let __terrane_result = plain.lock().expect("reference lock poisoned").flag;
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").count.clone();
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_snapshot = { let __terrane_result = plain.lock()
        .expect("reference lock poisoned").values.clone(); __terrane_result };
        __terrane_snapshot } .length()))
    );
    assign(held.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").count.clone();
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    weak = std::sync::Arc::downgrade(&plain.clone());
    {
        let __terrane_owner_4 = weak.upgrade().expect("reference expired");
        let __terrane_value = terrane_int_support::Int::from(12_i128);
        __terrane_owner_4.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").count.clone();
        __terrane_result }; __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
    {
        let __terrane_owner_5 = weak.upgrade().expect("reference expired");
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                33 /* terrane-site: tests/integration/inherited-shared-field-writes/case.trn:39:5-39:32 */,
        );
        let __terrane_value = {
            let __terrane_snapshot = {
                let __terrane_owner = weak.upgrade().expect("reference expired");
                __terrane_owner.lock().expect("reference lock poisoned").count.clone()
            };
            __terrane_snapshot
        };
        let _ = __terrane_raised(
            __terrane_owner_5
                .lock()
                .expect("reference lock poisoned")
                .values
                .set(__terrane_index, __terrane_value),

                33 /* terrane-site: tests/integration/inherited-shared-field-writes/case.trn:39:5-39:32 */,
        );
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_6 = &plain;
        __terrane_raised(__terrane_owner_6.lock().expect("reference lock poisoned")
        .values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        34 /* terrane-site: tests/integration/inherited-shared-field-writes/case.trn:40:12-40:27 */)),
        34 /* terrane-site: tests/integration/inherited-shared-field-writes/case.trn:40:12-40:27 */) }),
        String::from(" "), terrane_scalar_support::scalar_text(&{ let __terrane_owner_7 =
        &held; __terrane_raised(__terrane_owner_7.lock()
        .expect("shared reference lock poisoned").values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        35 /* terrane-site: tests/integration/inherited-shared-field-writes/case.trn:40:34-40:48 */)),
        35 /* terrane-site: tests/integration/inherited-shared-field-writes/case.trn:40:34-40:48 */) })
    );
    other = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsInheritedSharedFieldWritesBaseIdentity2F696E686572697465642D7368617265642D6669656C642D7772697465730062617365::terrane_construct(),
        ),
    );
    other_held = other.clone();
    {
        let __terrane_owner_8 = other_held.clone();
        let __terrane_value = terrane_int_support::Int::from(14_i128);
        *__terrane_owner_8
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_count_mut() = __terrane_value;
    }
    other_observer = std::sync::Arc::downgrade(&other.clone());
    {
        let __terrane_owner_9 = other_observer.upgrade().expect("reference expired");
        let __terrane_value = false;
        *__terrane_owner_9
            .lock()
            .expect("reference lock poisoned")
            .terrane_field_flag_mut() = __terrane_value;
    }
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = {
        let __terrane_result = other.lock().expect("reference lock poisoned")
        .terrane_field_count().clone(); __terrane_result }; __terrane_snapshot }),
        String::from(" "), terrane_scalar_support::scalar_text(&{ let __terrane_snapshot
        = other_held.lock().expect("shared reference lock poisoned")
        .terrane_field_count().clone(); __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = * other.lock().expect("reference lock poisoned")
        .terrane_field_flag(); __terrane_result }; __terrane_snapshot })
    );
    {
        let __terrane_owner_10 = other_held.clone();
        let __terrane_collection_argument_0 = {
            let __terrane_snapshot = {
                let __terrane_owner = other_observer
                    .upgrade()
                    .expect("reference expired");
                __terrane_owner
                    .lock()
                    .expect("reference lock poisoned")
                    .terrane_field_count()
                    .clone()
            };
            __terrane_snapshot
        };
        __terrane_owner_10
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_values_mut()
            .append(__terrane_collection_argument_0);
    };
    next = TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D::terrane_construct();
    {
        let __terrane_owner_11 = other_held.clone();
        let __terrane_value = next;
        *__terrane_owner_11
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_required_slot_mut() = Some(__terrane_value);
    }
    {
        let __terrane_owner_12 = other_observer.upgrade().expect("reference expired");
        let __terrane_value = terrane_int_support::Int::from(25_i128);
        __terrane_owner_12
            .lock()
            .expect("reference lock poisoned")
            .terrane_field_required_mut()
            .value = __terrane_value;
    }
    {
        let __terrane_owner_14 = other_held.clone();
        let __terrane_shared_argument_0 = {
            let __terrane_snapshot = {
                let __terrane_owner = other_observer
                    .upgrade()
                    .expect("reference expired");
                __terrane_owner
                    .lock()
                    .expect("reference lock poisoned")
                    .terrane_field_count()
                    .clone()
            };
            __terrane_snapshot
        } + terrane_int_support::Int::from(1_i128);
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_owner_14
            .lock()
            .expect("shared reference lock poisoned")
            .update(__terrane_shared_argument_1);
        __terrane_call_result
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot =
        other_held.lock().expect("shared reference lock poisoned").terrane_field_count()
        .clone(); __terrane_snapshot }), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = other.lock().expect("reference lock poisoned")
        .terrane_field_required().clone(); __terrane_result }; __terrane_snapshot }
        .value), String::from(" "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_snapshot = { let __terrane_result = other.lock()
        .expect("reference lock poisoned").terrane_field_values().clone();
        __terrane_result }; __terrane_snapshot } .length()))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = *
        other_held.lock().expect("shared reference lock poisoned").terrane_field_flag();
        __terrane_snapshot }), String::from(" "), terrane_scalar_support::scalar_text(&{
        let __terrane_snapshot = { let __terrane_owner = other_observer.upgrade()
        .expect("reference expired"); * __terrane_owner.lock()
        .expect("reference lock poisoned").terrane_field_flag() }; __terrane_snapshot })
    );
    {
        let __terrane_owner_15 = held.clone();
        let __terrane_value = terrane_int_support::Int::from(17_i128);
        __terrane_owner_15
            .lock()
            .expect("shared reference lock poisoned")
            .required
            .as_mut()
            .expect("required field initialized")
            .value = __terrane_value;
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").required
        .as_ref().expect("required field initialized").clone(); __terrane_result };
        __terrane_snapshot } .value), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").required.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot } .value)
    );
    replacement = TerraneNsInheritedSharedFieldWritesItemIdentity2F696E686572697465642D7368617265642D6669656C642D777269746573006974656D::terrane_construct();
    {
        let __terrane_value = terrane_int_support::Int::from(23_i128);
        replacement.value = __terrane_value;
    }
    {
        let __terrane_owner_16 = weak.upgrade().expect("reference expired");
        let __terrane_value = replacement;
        __terrane_owner_16.lock().expect("reference lock poisoned").required = Some(
            __terrane_value,
        );
    }
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result = plain.lock().expect("reference lock poisoned").required
        .as_ref().expect("required field initialized").clone(); __terrane_result };
        __terrane_snapshot } .value), String::from(" "),
        terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").required.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot } .value)
    );
    {
        let __terrane_owner_17 = plain.clone();
        let __terrane_value = terrane_int_support::Int::from(19_i128);
        __terrane_owner_17.lock().expect("reference lock poisoned").count = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = held.lock()
        .expect("shared reference lock poisoned").count.clone(); __terrane_snapshot })
    );
}
// Source: tests/integration/inherited-static-factory/case.trn
// Namespace: inherited-static-factory
#[derive(Clone)]
pub struct TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616CStorage {}
impl TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616CStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "/inherited-static-factory::animal",
            name: "animal",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[],
        }
            .name
            .to_owned();
    }
}
#[derive(Clone)]
pub enum TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616C {
    Own(
        TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616CStorage,
    ),
    TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67(
        TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67,
    ),
}
impl TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616C {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616CStorage::terrane_construct(),
        )
    }
    pub fn terrane_static_create() -> TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616C {
        return TerraneNsInheritedStaticFactoryAnimalIdentity2F696E686572697465642D7374617469632D666163746F727900616E696D616C::terrane_construct();
    }
    pub fn class_name(&self) -> String {
        match self {
            Self::Own(value) => value.class_name(),
            Self::TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67(
                value,
            ) => value.class_name(),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67 {}
impl TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "/inherited-static-factory::dog",
            name: "dog",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[],
        }
            .name
            .to_owned();
    }
    pub fn terrane_static_create() -> TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67 {
        return TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67::terrane_construct();
    }
}
fn test_scenario_terrane_inherited_static_factory() {
    let pet: TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67;
    pet = TerraneNsInheritedStaticFactoryDogIdentity2F696E686572697465642D7374617469632D666163746F727900646F67::terrane_static_create();
    println!("{}", terrane_scalar_support::scalar_text(&pet.class_name()));
}
// Source: tests/integration/inherited-static-state/case.trn
// Namespace: inherited-static-state
pub static TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEANIMALIDENTITY2F696E686572697465642D7374617469632D737461746500616E696D616C_CALLS: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616CStorage {}
impl TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616CStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
#[derive(Clone)]
pub enum TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616C {
    Own(
        TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616CStorage,
    ),
    TerraneNsInheritedStaticStateDogIdentity2F696E686572697465642D7374617469632D737461746500646F67(
        TerraneNsInheritedStaticStateDogIdentity2F696E686572697465642D7374617469632D737461746500646F67,
    ),
}
impl TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616C {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616CStorage::terrane_construct(),
        )
    }
    pub fn terrane_static_increase() -> terrane_int_support::Int {
        {
            let __terrane_static_value = {
                let __terrane_value = TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEANIMALIDENTITY2F696E686572697465642D7374617469632D737461746500616E696D616C_CALLS
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            } + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEANIMALIDENTITY2F696E686572697465642D7374617469632D737461746500616E696D616C_CALLS
                .lock()
                .expect("static field lock poisoned") = __terrane_static_value;
        }
        return {
            let __terrane_value = TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEANIMALIDENTITY2F696E686572697465642D7374617469632D737461746500616E696D616C_CALLS
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        };
    }
}
pub static TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEDOGIDENTITY2F696E686572697465642D7374617469632D737461746500646F67_CALLS: std::sync::LazyLock<
    TerraneSharedCell<terrane_int_support::Int>,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(
    terrane_int_support::Int::from(0_i128),
));
#[derive(Clone)]
pub struct TerraneNsInheritedStaticStateDogIdentity2F696E686572697465642D7374617469632D737461746500646F67 {}
impl TerraneNsInheritedStaticStateDogIdentity2F696E686572697465642D7374617469632D737461746500646F67 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_increase() -> terrane_int_support::Int {
        {
            let __terrane_static_value = {
                let __terrane_value = TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEDOGIDENTITY2F696E686572697465642D7374617469632D737461746500646F67_CALLS
                    .lock()
                    .expect("static field lock poisoned")
                    .clone();
                __terrane_value
            } + terrane_int_support::Int::from(1_i128);
            *TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEDOGIDENTITY2F696E686572697465642D7374617469632D737461746500646F67_CALLS
                .lock()
                .expect("static field lock poisoned") = __terrane_static_value;
        }
        return {
            let __terrane_value = TERRANE_STATIC_TERRANENSINHERITEDSTATICSTATEDOGIDENTITY2F696E686572697465642D7374617469632D737461746500646F67_CALLS
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        };
    }
}
fn test_scenario_terrane_inherited_static_state() {
    let animal_first: terrane_int_support::Int;
    let dog_first: terrane_int_support::Int;
    let animal_second: terrane_int_support::Int;
    animal_first = TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616C::terrane_static_increase();
    dog_first = TerraneNsInheritedStaticStateDogIdentity2F696E686572697465642D7374617469632D737461746500646F67::terrane_static_increase();
    animal_second = TerraneNsInheritedStaticStateAnimalIdentity2F696E686572697465642D7374617469632D737461746500616E696D616C::terrane_static_increase();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&animal_first),
        terrane_scalar_support::scalar_text(&dog_first),
        terrane_scalar_support::scalar_text(&animal_second)
    );
}
// Source: tests/integration/interface-borrowed-wrapper/case.trn
// Namespace: interface-borrowed-wrapper
pub trait TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol,
    >;
    fn m(&mut self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071(
    Box<
        dyn TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol,
    >,
);
impl Clone
for TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071 {
    pub fn m(&mut self) -> terrane_int_support::Int {
        self.0.m()
    }
}
#[derive(Clone)]
pub struct TerraneNsInterfaceBorrowedWrapperSIdentity2F696E746572666163652D626F72726F7765642D777261707065720053 {
    pub k: terrane_int_support::Int,
}
impl TerraneNsInterfaceBorrowedWrapperSIdentity2F696E746572666163652D626F72726F7765642D777261707065720053 {
    pub fn terrane_construct() -> Self {
        Self {
            k: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn m(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.k.clone()
                + terrane_int_support::Int::from(1_i128);
            self.k = __terrane_value;
        }
        return self.k.clone();
    }
}
impl TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol
for TerraneNsInterfaceBorrowedWrapperSIdentity2F696E746572666163652D626F72726F7765642D777261707065720053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071Protocol,
    > {
        Box::new(self.clone())
    }
    fn m(&mut self) -> terrane_int_support::Int {
        TerraneNsInterfaceBorrowedWrapperSIdentity2F696E746572666163652D626F72726F7765642D777261707065720053::m(
            &mut *self,
        )
    }
}
impl From<
    TerraneNsInterfaceBorrowedWrapperSIdentity2F696E746572666163652D626F72726F7765642D777261707065720053,
>
for TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071 {
    fn from(
        value: TerraneNsInterfaceBorrowedWrapperSIdentity2F696E746572666163652D626F72726F7765642D777261707065720053,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_interface_borrowed_wrapper(
    x: &mut TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071,
) -> terrane_int_support::Int {
    return x.m();
}
fn test_scenario_terrane_interface_borrowed_wrapper() {
    let mut s: TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071;
    s = <TerraneNsInterfaceBorrowedWrapperQIdentity2F696E746572666163652D626F72726F7765642D777261707065720071>::from(
        TerraneNsInterfaceBorrowedWrapperSIdentity2F696E746572666163652D626F72726F7765642D777261707065720053::terrane_construct(),
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_interface_borrowed_wrapper(&mut
        s))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_interface_borrowed_wrapper(&mut
        s))
    );
}
// Source: tests/integration/interface-invocation-mode-widening/case.trn
// Namespace: interface-invocation-mode-widening
pub trait MutableCounterProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn MutableCounterProtocol>;
    fn separate_box(&self) -> Box<dyn MutableCounterProtocol>;
    fn value(&mut self, offset: terrane_int_support::Int) -> terrane_int_support::Int;
}
impl Clone for Box<dyn MutableCounterProtocol> {
    fn clone(&self) -> Self {
        MutableCounterProtocol::clone_box(&**self)
    }
}
pub struct MutableCounter(Box<dyn MutableCounterProtocol>);
impl Clone for MutableCounter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl MutableCounter {
    pub fn value(
        &mut self,
        offset: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.0.value(offset)
    }
}
pub trait ConsumingLabelProtocol: Send + Sync {
    fn label(self: Box<Self>) -> String;
}
pub struct ConsumingLabel(Box<dyn ConsumingLabelProtocol>);
impl ConsumingLabel {
    pub fn label(self) -> String {
        self.0.label()
    }
}
pub trait ConsumingCounterProtocol: Send + Sync {
    fn redeem(self: Box<Self>) -> terrane_int_support::Int;
}
pub struct ConsumingCounter(Box<dyn ConsumingCounterProtocol>);
impl ConsumingCounter {
    pub fn redeem(self) -> terrane_int_support::Int {
        self.0.redeem()
    }
}
#[derive(Clone)]
pub struct Sample {
    pub total: terrane_int_support::Int,
}
impl Sample {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn value(&self, offset: terrane_int_support::Int) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(7_i128) + offset.clone();
    }
    pub fn label(&self) -> String {
        return String::from("ready");
    }
}
impl MutableCounterProtocol for Sample {
    fn clone_box(&self) -> Box<dyn MutableCounterProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn MutableCounterProtocol> {
        Box::new(self.clone())
    }
    fn value(&mut self, offset: terrane_int_support::Int) -> terrane_int_support::Int {
        Sample::value(&*self, offset)
    }
}
impl From<Sample> for MutableCounter {
    fn from(value: Sample) -> Self {
        Self(Box::new(value))
    }
}
impl ConsumingLabelProtocol for Sample {
    fn label(self: Box<Self>) -> String {
        Sample::label(&*self)
    }
}
impl From<Sample> for ConsumingLabel {
    fn from(value: Sample) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct MutableSample {
    pub total: terrane_int_support::Int,
}
impl MutableSample {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn redeem(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone()
                + terrane_int_support::Int::from(1_i128);
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
}
impl ConsumingCounterProtocol for MutableSample {
    fn redeem(mut self: Box<Self>) -> terrane_int_support::Int {
        MutableSample::redeem(&mut *self)
    }
}
impl From<MutableSample> for ConsumingCounter {
    fn from(value: MutableSample) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_interface_invocation_mode_widening() {
    let mut mutable_view: MutableCounter;
    let consuming_view: ConsumingLabel;
    let mutable_consuming_view: ConsumingCounter;
    mutable_view = <MutableCounter>::from(Sample::terrane_construct());
    consuming_view = <ConsumingLabel>::from(Sample::terrane_construct());
    mutable_consuming_view = <ConsumingCounter>::from(
        MutableSample::terrane_construct(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&mutable_view
        .value(terrane_int_support::Int::from(5_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&consuming_view.label()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&mutable_consuming_view.redeem())
    );
}
// Source: tests/integration/interface-owned-inherited-mutation/case.trn
// Namespace: interface-owned-inherited-mutation
pub trait TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol,
    >;
    fn b(&self) -> terrane_int_support::Int;
    fn m(&mut self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365(
    Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol,
    >,
);
impl Clone
for TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365 {
    pub fn b(&self) -> terrane_int_support::Int {
        self.0.b()
    }
    pub fn m(&mut self) -> terrane_int_support::Int {
        self.0.m()
    }
}
pub trait TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol: Send + Sync + TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol,
    >;
    fn other(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562(
    Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol,
    >,
);
impl From<
    TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562,
>
for TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365 {
    fn from(
        value: TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562 {
    pub fn b(&self) -> terrane_int_support::Int {
        self.0.b()
    }
    pub fn m(&mut self) -> terrane_int_support::Int {
        self.0.m()
    }
    pub fn other(&self) -> terrane_int_support::Int {
        self.0.other()
    }
}
#[derive(Clone)]
pub struct TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053 {
    pub k: terrane_int_support::Int,
}
impl TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053 {
    pub fn terrane_construct() -> Self {
        Self {
            k: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn b(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn m(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.k.clone()
                + terrane_int_support::Int::from(1_i128);
            self.k = __terrane_value;
        }
        return self.k.clone();
    }
    pub fn other(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol
for TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn other(&self) -> terrane_int_support::Int {
        TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053::other(
            &*self,
        )
    }
}
impl From<
    TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053,
>
for TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562 {
    fn from(
        value: TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol
for TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn b(&self) -> terrane_int_support::Int {
        TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053::b(
            &*self,
        )
    }
    fn m(&mut self) -> terrane_int_support::Int {
        TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053::m(
            &mut *self,
        )
    }
}
impl From<
    TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053,
>
for TerraneNsInterfaceOwnedInheritedMutationBaseIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0062617365 {
    fn from(
        value: TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn go_terrane_interface_owned_inherited_mutation(
    mut x: TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562,
) -> terrane_int_support::Int {
    return x.m();
}
fn test_scenario_terrane_interface_owned_inherited_mutation() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&go_terrane_interface_owned_inherited_mutation(<
        TerraneNsInterfaceOwnedInheritedMutationSubIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E00737562
        >
        ::from(TerraneNsInterfaceOwnedInheritedMutationSIdentity2F696E746572666163652D6F776E65642D696E686572697465642D6D75746174696F6E0053::terrane_construct())))
    );
}
// Source: tests/integration/interface-owned-upcast/case.trn
// Namespace: interface-owned-upcast
pub trait TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol,
    >;
    fn b(&self) -> terrane_int_support::Int;
    fn m(&mut self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365(
    Box<
        dyn TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol,
    >,
);
impl Clone
for TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365 {
    pub fn b(&self) -> terrane_int_support::Int {
        self.0.b()
    }
    pub fn m(&mut self) -> terrane_int_support::Int {
        self.0.m()
    }
}
pub trait TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol: Send + Sync + TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol,
    >;
    fn other(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562(
    Box<
        dyn TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol,
    >,
);
impl From<
    TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562,
>
for TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365 {
    fn from(
        value: TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562,
    ) -> Self {
        Self(value.0)
    }
}
impl Clone
for TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562 {
    pub fn b(&self) -> terrane_int_support::Int {
        self.0.b()
    }
    pub fn m(&mut self) -> terrane_int_support::Int {
        self.0.m()
    }
    pub fn other(&self) -> terrane_int_support::Int {
        self.0.other()
    }
}
#[derive(Clone)]
pub struct TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053 {
    pub k: terrane_int_support::Int,
}
impl TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053 {
    pub fn terrane_construct() -> Self {
        Self {
            k: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn b(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
    pub fn m(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.k.clone()
                + terrane_int_support::Int::from(1_i128);
            self.k = __terrane_value;
        }
        return self.k.clone();
    }
    pub fn other(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
impl TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol
for TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562Protocol,
    > {
        Box::new(self.clone())
    }
    fn other(&self) -> terrane_int_support::Int {
        TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053::other(
            &*self,
        )
    }
}
impl From<
    TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053,
>
for TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562 {
    fn from(
        value: TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol
for TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365Protocol,
    > {
        Box::new(self.clone())
    }
    fn b(&self) -> terrane_int_support::Int {
        TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053::b(
            &*self,
        )
    }
    fn m(&mut self) -> terrane_int_support::Int {
        TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053::m(
            &mut *self,
        )
    }
}
impl From<
    TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053,
>
for TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365 {
    fn from(
        value: TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn up_terrane_interface_owned_upcast(
    x: TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365,
) -> terrane_int_support::Int {
    return x.b();
}
fn go_terrane_interface_owned_upcast(
    x: TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562,
) -> terrane_int_support::Int {
    return up_terrane_interface_owned_upcast(
        <TerraneNsInterfaceOwnedUpcastBaseIdentity2F696E746572666163652D6F776E65642D7570636173740062617365>::from(
            x,
        ),
    );
}
fn test_scenario_terrane_interface_owned_upcast() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&go_terrane_interface_owned_upcast(<
        TerraneNsInterfaceOwnedUpcastSubIdentity2F696E746572666163652D6F776E65642D75706361737400737562
        >
        ::from(TerraneNsInterfaceOwnedUpcastSIdentity2F696E746572666163652D6F776E65642D7570636173740053::terrane_construct())))
    );
}
// Source: tests/integration/interface-value-separation/case.trn
// Namespace: interface-value-separation
pub trait TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol,
    >;
    fn read(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65(
    Box<
        dyn TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol,
    >,
);
impl Clone
for TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65 {
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
    fn terrane_separate(&self) -> Self {
        Self(
            TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol::separate_box(
                &*self.0,
            ),
        )
    }
}
#[derive(Clone)]
pub struct TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572 {
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
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("destruct"));
    }
}
impl TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol
for TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65Protocol,
    > {
        Box::new(self.terrane_separate())
    }
    fn read(&self) -> terrane_int_support::Int {
        TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572::read(
            &*self,
        )
    }
}
impl From<
    TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572,
>
for TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65 {
    fn from(
        value: TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl Drop
for TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
fn test_scenario_terrane_interface_value_separation() {
    let original: TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65;
    let copied: TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65;
    original = <TerraneNsInterfaceValueSeparationReadableIdentity2F696E746572666163652D76616C75652D73657061726174696F6E007265616461626C65>::from(
        TerraneNsInterfaceValueSeparationCounterIdentity2F696E746572666163652D76616C75652D73657061726174696F6E00636F756E746572::terrane_construct(),
    );
    copied = original.terrane_separate();
    println!("{}", terrane_scalar_support::scalar_text(&original.read()));
    println!("{}", terrane_scalar_support::scalar_text(&copied.read()));
}
// Source: tests/integration/method-without-receiver-use/case.trn
// Namespace: method-without-receiver-use
#[derive(Clone)]
pub struct Answerer {}
impl Answerer {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn answer(&self) -> terrane_int_support::Int {
        return terrane_int_support::Int::from(42_i128);
    }
}
fn test_scenario_terrane_method_without_receiver_use() {
    let value: Answerer;
    value = Answerer::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.answer()));
}
// Source: tests/integration/mutating-interface-methods/case.trn
// Namespace: mutating-interface-methods
pub trait AdjustableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol>;
    fn separate_box(&self) -> Box<dyn AdjustableProtocol>;
    fn increase(&mut self, amount: terrane_int_support::Int) -> terrane_int_support::Int;
    fn read(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn AdjustableProtocol> {
    fn clone(&self) -> Self {
        AdjustableProtocol::clone_box(&**self)
    }
}
pub struct Adjustable(Box<dyn AdjustableProtocol>);
impl Clone for Adjustable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Adjustable {
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.0.increase(amount)
    }
    pub fn read(&self) -> terrane_int_support::Int {
        self.0.read()
    }
}
#[derive(Clone)]
pub struct TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone() + amount.clone();
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
}
impl AdjustableProtocol
for TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572 {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn AdjustableProtocol> {
        Box::new(self.clone())
    }
    fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572::increase(
            &mut *self,
            amount,
        )
    }
    fn read(&self) -> terrane_int_support::Int {
        TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572::read(
            &*self,
        )
    }
}
impl From<
    TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572,
> for Adjustable {
    fn from(
        value: TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_mutating_interface_methods() {
    let mut value: Adjustable;
    value = <Adjustable>::from(
        TerraneNsMutatingInterfaceMethodsCounterIdentity2F6D75746174696E672D696E746572666163652D6D6574686F647300636F756E746572::terrane_construct(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&value
        .increase(terrane_int_support::Int::from(4_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.read()));
}
// Source: tests/integration/nested-object-mutation/case.trn
// Namespace: nested-object-mutation
#[derive(Clone)]
pub struct TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166 {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        {
            let __terrane_value = initial.clone();
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsNestedObjectMutationHolderIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E00686F6C646572 {
    pub child: TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166,
}
impl TerraneNsNestedObjectMutationHolderIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E00686F6C646572 {
    pub fn terrane_construct(
        child: TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            child: TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166::terrane_construct(
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(child);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        child: TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166,
    ) {
        {
            let __terrane_value = child;
            self.child = __terrane_value;
        }
    }
    pub fn replace(
        &mut self,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = value.clone();
            self.child.value = __terrane_value;
        }
        return self.child.value.clone();
    }
}
fn test_scenario_terrane_nested_object_mutation() {
    let value: TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166;
    let mut container: TerraneNsNestedObjectMutationHolderIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E00686F6C646572;
    value = TerraneNsNestedObjectMutationLeafIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E006C656166::terrane_construct(
        terrane_int_support::Int::from(1_i128),
    );
    container = TerraneNsNestedObjectMutationHolderIdentity2F6E65737465642D6F626A6563742D6D75746174696F6E00686F6C646572::terrane_construct(
        value,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&container
        .replace(terrane_int_support::Int::from(42_i128)))
    );
}
// Source: tests/integration/object-contracts-reuse/case.trn
// Namespace: object-contracts-reuse
pub trait TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol,
    >;
    fn describe(&self, prefix: String) -> String;
}
impl Clone
for Box<
    dyn TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65(
    Box<
        dyn TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol,
    >,
);
impl Clone
for TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65 {
    pub fn describe(&self, prefix: String) -> String {
        self.0.describe(prefix)
    }
}
#[derive(Clone)]
pub struct TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365Storage {
    pub value: terrane_int_support::Int,
}
impl TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365Storage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn describe(&self, prefix: String) -> String {
        return {
            let mut __terrane_text = String::with_capacity(4);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &prefix,
                &mut __terrane_text,
            );
            __terrane_text.push_str("base");
            __terrane_text
        };
    }
}
#[derive(Clone)]
pub enum TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365 {
    Own(
        TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365Storage,
    ),
    TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64(
        TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64,
    ),
}
impl TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365Storage::terrane_construct(),
        )
    }
    pub fn describe(&self, prefix: String) -> String {
        match self {
            Self::Own(value) => value.describe(prefix),
            Self::TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64(
                value,
            ) => value.describe(prefix),
        }
    }
    pub fn terrane_field_value(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.value,
            Self::TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64(
                value,
            ) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.value,
            Self::TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64(
                value,
            ) => &mut value.value,
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64 {
    pub value: terrane_int_support::Int,
    pub tag: String,
    pub extra: terrane_int_support::Int,
}
impl TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            tag: String::from("!"),
            extra: terrane_int_support::Int::from(2_i128),
        }
    }
    pub fn describe(&self, prefix: String) -> String {
        return {
            let mut __terrane_text = String::with_capacity(5);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &prefix,
                &mut __terrane_text,
            );
            __terrane_text.push_str("child");
            __terrane_text
        };
    }
    pub fn tagged_value(&self) -> String {
        return self.tag.clone();
    }
}
impl TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol
for TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn describe(&self, prefix: String) -> String {
        TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64::describe(
            &*self,
            prefix,
        )
    }
}
impl From<
    TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64,
>
for TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65 {
    fn from(
        value: TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_object_contracts_reuse() {
    let value: TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64;
    let view: TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65;
    let base_view: TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365;
    value = TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64::terrane_construct();
    view = <TerraneNsObjectContractsReuseDescribableIdentity2F6F626A6563742D636F6E7472616374732D7265757365006465736372696261626C65>::from(
        value.clone(),
    );
    base_view = TerraneNsObjectContractsReuseBaseIdentity2F6F626A6563742D636F6E7472616374732D72657573650062617365::TerraneNsObjectContractsReuseChildIdentity2F6F626A6563742D636F6E7472616374732D7265757365006368696C64(
        value.clone(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&view.describe(String::from("a-")))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&base_view
        .describe(String::from("b-")))
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.tagged_value()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(value.value.clone() + value.extra
        .clone()))
    );
}
// Source: tests/integration/object-field-owned-snapshot/case.trn
// Namespace: object-field-owned-snapshot
#[derive(Clone)]
pub struct TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(7_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572 {
    pub required: Option<
        TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164,
    >,
    pub initialized: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164,
}
impl TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572 {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            required: None,
            initialized: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164::terrane_construct(),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164::terrane_construct();
            self.required = Some(__terrane_value);
        }
    }
    pub fn required_copy(
        &self,
    ) -> TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164 {
        return self.required.as_ref().expect("required field initialized").clone();
    }
    pub fn initialized_copy(
        &self,
    ) -> TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164 {
        return self.initialized.clone();
    }
    pub fn change_copy(&self) -> terrane_int_support::Int {
        let mut local: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
        local = self.required.as_ref().expect("required field initialized").clone();
        {
            let __terrane_value = terrane_int_support::Int::from(9_i128);
            local.value = __terrane_value;
        }
        return local.value.clone();
    }
    pub fn temporary_required_copy(
        &self,
    ) -> TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164 {
        return TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572::terrane_construct()
            .required
            .as_ref()
            .expect("required field initialized")
            .clone();
    }
    pub fn required_from_borrow(
        &self,
        owner: &TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572,
    ) -> TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164 {
        return {
            let __terrane_snapshot = owner
                .required
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        };
    }
    pub fn required_from_shared(
        &self,
        owner: std::sync::Arc<
            TerraneSharedCell<
                TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572,
            >,
        >,
    ) -> TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164 {
        return {
            let __terrane_snapshot = owner
                .lock()
                .expect("shared reference lock poisoned")
                .required
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        };
    }
}
#[derive(Clone)]
pub struct PlainHolder {
    pub value: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164,
    pub words: terrane_collection_support::List<String>,
    pub text: String,
}
impl PlainHolder {
    pub fn terrane_construct() -> Self {
        Self {
            value: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164::terrane_construct(),
            words: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("original")]),
            text: String::from("original"),
        }
    }
}
fn make_holder() -> PlainHolder {
    return PlainHolder::terrane_construct();
}
fn borrow_holder<'a>(value: &'a PlainHolder) -> &'a PlainHolder {
    return value;
}
#[derive(Clone)]
pub struct TrackedPayload {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl TrackedPayload {
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
        println!(
            "{}{}", String::from("released"), terrane_scalar_support::scalar_text(&self
            .value)
        );
    }
}
impl Drop for TrackedPayload {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct TrackedHolder {
    pub value: TrackedPayload,
}
impl TrackedHolder {
    pub fn terrane_construct() -> Self {
        Self {
            value: TrackedPayload::terrane_construct(),
        }
    }
}
fn check_separated() {
    let named: TrackedHolder;
    let borrowed: &TrackedHolder;
    let mut snapshot: TrackedPayload;
    named = TrackedHolder::terrane_construct();
    borrowed = &named;
    snapshot = {
        let __terrane_snapshot = borrowed.value.terrane_separate();
        __terrane_snapshot
    };
    {
        let __terrane_value = terrane_int_support::Int::from(17_i128);
        snapshot.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&snapshot.value));
    println!("{}", terrane_scalar_support::scalar_text(&named.value.value));
}
fn test_scenario_terrane_object_field_owned_snapshot() {
    let original: TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572;
    let first: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let second: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let unchanged: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let mut from_call: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let mut from_constructor: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let named: PlainHolder;
    let mut from_borrow: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let shared: std::sync::Arc<TerraneSharedCell<PlainHolder>>;
    let mut from_shared: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let mut required_borrow: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let shared_required: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572,
        >,
    >;
    let mut required_shared: TerraneNsObjectFieldOwnedSnapshotPayloadIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F74007061796C6F6164;
    let mut words: terrane_collection_support::List<String>;
    let text: String;
    original = TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572::terrane_construct();
    first = original.required_copy();
    second = original.initialized_copy();
    println!("{}", terrane_scalar_support::scalar_text(&first.value));
    println!("{}", terrane_scalar_support::scalar_text(&second.value));
    println!("{}", terrane_scalar_support::scalar_text(&original.change_copy()));
    unchanged = original.required_copy();
    println!("{}", terrane_scalar_support::scalar_text(&unchanged.value));
    from_call = make_holder().value;
    {
        let __terrane_value = terrane_int_support::Int::from(11_i128);
        from_call.value = __terrane_value;
    }
    from_constructor = PlainHolder::terrane_construct().value;
    {
        let __terrane_value = terrane_int_support::Int::from(12_i128);
        from_constructor.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&from_call.value));
    println!("{}", terrane_scalar_support::scalar_text(&from_constructor.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&original.temporary_required_copy()
        .value)
    );
    named = PlainHolder::terrane_construct();
    from_borrow = {
        let __terrane_snapshot = borrow_holder(&named).value.clone();
        __terrane_snapshot
    };
    {
        let __terrane_value = terrane_int_support::Int::from(13_i128);
        from_borrow.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&from_borrow.value));
    println!("{}", terrane_scalar_support::scalar_text(&named.value.value));
    shared = std::sync::Arc::new(
        TerraneSharedCell::new(PlainHolder::terrane_construct()),
    );
    from_shared = {
        let __terrane_snapshot = shared
            .lock()
            .expect("shared reference lock poisoned")
            .value
            .clone();
        __terrane_snapshot
    };
    {
        let __terrane_value = terrane_int_support::Int::from(14_i128);
        from_shared.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&from_shared.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = shared
        .lock().expect("shared reference lock poisoned").value.clone();
        __terrane_snapshot } .value)
    );
    required_borrow = original.required_from_borrow(&original);
    {
        let __terrane_value = terrane_int_support::Int::from(15_i128);
        required_borrow.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&required_borrow.value));
    println!("{}", terrane_scalar_support::scalar_text(&original.required_copy().value));
    shared_required = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsObjectFieldOwnedSnapshotHolderIdentity2F6F626A6563742D6669656C642D6F776E65642D736E617073686F7400686F6C646572::terrane_construct(),
        ),
    );
    required_shared = original.required_from_shared(shared_required.clone());
    {
        let __terrane_value = terrane_int_support::Int::from(16_i128);
        required_shared.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&required_shared.value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&original
        .required_from_shared(shared_required.clone()).value)
    );
    words = {
        let __terrane_snapshot = borrow_holder(&named).words.clone();
        __terrane_snapshot
    };
    {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),

                36 /* terrane-site: tests/integration/object-field-owned-snapshot/case.trn:99:5-99:25 */,
        );
        let __terrane_value = String::from("changed");
        let _ = __terrane_raised(
            words.set(__terrane_index, __terrane_value),

                36 /* terrane-site: tests/integration/object-field-owned-snapshot/case.trn:99:5-99:25 */,
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        37 /* terrane-site: tests/integration/object-field-owned-snapshot/case.trn:100:12-100:20 */)),
        37 /* terrane-site: tests/integration/object-field-owned-snapshot/case.trn:100:12-100:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(named.words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        38 /* terrane-site: tests/integration/object-field-owned-snapshot/case.trn:101:12-101:26 */)),
        38 /* terrane-site: tests/integration/object-field-owned-snapshot/case.trn:101:12-101:26 */))
    );
    text = {
        let __terrane_snapshot = borrow_holder(&named).text.clone();
        __terrane_snapshot
    };
    println!("{}", terrane_scalar_support::scalar_text(&text));
    println!("{}", terrane_scalar_support::scalar_text(&named.text));
    check_separated();
}
// Source: tests/integration/object-inheritance-lifecycle-matrix/case.trn
// Namespace: object-inheritance-lifecycle-matrix
pub trait TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
    >;
    fn report(&self) -> terrane_int_support::Int;
}
impl Clone
for Box<
    dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564(
    Box<
        dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564 {
    pub fn report(&self) -> terrane_int_support::Int {
        self.0.report()
    }
    fn terrane_separate(&self) -> Self {
        Self(
            TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol::separate_box(
                &*self.0,
            ),
        )
    }
}
#[derive(Clone)]
pub struct TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365Storage {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365Storage {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn report(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        {
            let __terrane_value = value.clone();
            self.value = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("base-destruct"));
    }
}
#[derive(Clone)]
pub enum TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365 {
    Own(
        TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365Storage,
    ),
    TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64(
        TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64,
    ),
}
impl TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365Storage::terrane_construct(),
        )
    }
    pub fn terrane_separate(&self) -> Self {
        match self {
            Self::Own(value) => Self::Own(value.terrane_separate()),
            Self::TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64(
                value,
            ) => {
                Self::TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64(
                    value.terrane_separate(),
                )
            }
        }
    }
    pub fn report(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.report(),
            Self::TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64(
                value,
            ) => value.report(),
        }
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        match self {
            Self::Own(value_) => value_.set(value),
            Self::TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64(
                value_,
            ) => value_.set(value),
        }
    }
    pub fn terrane_field_value(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.value,
            Self::TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64(
                value,
            ) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.value,
            Self::TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64(
                value,
            ) => &mut value.value,
        }
    }
}
impl TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol
for TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
    > {
        Box::new(self.terrane_separate())
    }
    fn report(&self) -> terrane_int_support::Int {
        TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365::report(
            &*self,
        )
    }
}
impl From<
    TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365,
>
for TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564 {
    fn from(
        value: TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl Drop
for TerraneNsObjectInheritanceLifecycleMatrixBaseIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D61747269780062617365Storage {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64 {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
    pub extra: terrane_int_support::Int,
}
impl TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            extra: terrane_int_support::Int::from(2_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        }
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn report(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
    pub fn set(&mut self, value: terrane_int_support::Int) {
        {
            let __terrane_value = value.clone();
            self.value = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("child-destruct"));
    }
    pub fn terrane_destruct_0(&mut self) {
        println!("{}", String::from("base-destruct"));
    }
}
impl TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol
for TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64 {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564Protocol,
    > {
        Box::new(self.terrane_separate())
    }
    fn report(&self) -> terrane_int_support::Int {
        TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64::report(
            &*self,
        )
    }
}
impl From<
    TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64,
>
for TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564 {
    fn from(
        value: TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl Drop
for TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64 {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
        self.terrane_destruct_0();
    }
}
fn test_scenario_terrane_object_inheritance_lifecycle_matrix() {
    let mut value: TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64;
    let view: TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564;
    let copied: TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564;
    value = TerraneNsObjectInheritanceLifecycleMatrixChildIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006368696C64::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.report()));
    value.set(terrane_int_support::Int::from(4_i128));
    println!("{}", terrane_scalar_support::scalar_text(&value.report()));
    view = <TerraneNsObjectInheritanceLifecycleMatrixNamedIdentity2F6F626A6563742D696E6865726974616E63652D6C6966656379636C652D6D6174726978006E616D6564>::from(
        value.terrane_separate(),
    );
    copied = view.terrane_separate();
    println!("{}", terrane_scalar_support::scalar_text(&copied.report()));
}
// Source: tests/integration/optional-field-presence/case.trn
// Namespace: optional-field-presence
#[derive(Clone)]
pub struct PlainBox {
    pub value: Option<String>,
}
impl PlainBox {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("kept")),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78Storage {
    pub value: Option<String>,
}
impl TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78Storage {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("kept")),
        }
    }
}
#[derive(Clone)]
pub enum TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78 {
    Own(
        TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78Storage,
    ),
    TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78(
        TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78,
    ),
}
impl TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78Storage::terrane_construct(),
        )
    }
    pub fn terrane_field_value(&self) -> &Option<String> {
        match self {
            Self::Own(value) => &value.value,
            Self::TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78(
                value,
            ) => &value.value,
        }
    }
    pub fn terrane_field_value_mut(&mut self) -> &mut Option<String> {
        match self {
            Self::Own(value) => &mut value.value,
            Self::TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78(
                value,
            ) => &mut value.value,
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78 {
    pub value: Option<String>,
    pub extra: terrane_int_support::Int,
}
impl TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78 {
    pub fn terrane_construct() -> Self {
        Self {
            value: Some(String::from("kept")),
            extra: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub struct RequiredBox {
    pub value: Option<String>,
}
impl RequiredBox {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = Some(String::from("required"));
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct OuterBox {
    pub inner: TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78,
}
impl OuterBox {
    pub fn terrane_construct() -> Self {
        Self {
            inner: TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78::terrane_construct(),
        }
    }
}
fn make_box() -> PlainBox {
    println!("{}", String::from("called"));
    return PlainBox::terrane_construct();
}
fn inspect_terrane_optional_field_presence(
    value: &TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78,
) {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.terrane_field_value()
        .is_some()), terrane_scalar_support::scalar_text(&value.terrane_field_value()
        .is_none())
    );
}
fn inspect_shared(
    value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78,
        >,
    >,
) {
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_0 = &value;
        __terrane_owner_0.lock().expect("shared reference lock poisoned")
        .terrane_field_value().is_some() }), terrane_scalar_support::scalar_text(&{ let
        __terrane_owner_1 = &value; __terrane_owner_1.lock()
        .expect("shared reference lock poisoned").terrane_field_value().is_none() })
    );
}
fn test_scenario_terrane_optional_field_presence() {
    let mut plain: PlainBox;
    let base: TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78;
    let derived: TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78;
    let shared_owner: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78,
        >,
    >;
    let shared: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78,
        >,
    >;
    let observer: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78,
        >,
    >;
    let mut required: RequiredBox;
    let outer: std::sync::Arc<TerraneSharedCell<OuterBox>>;
    plain = PlainBox::terrane_construct();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(&plain.value).is_some()),
        terrane_scalar_support::scalar_text(&(&plain.value).is_none())
    );
    {
        let __terrane_value = None;
        plain.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&(&plain.value).is_none()));
    base = TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78::terrane_construct();
    inspect_terrane_optional_field_presence(&base);
    derived = TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78::TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78(
        TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78::terrane_construct(),
    );
    inspect_terrane_optional_field_presence(&derived);
    shared_owner = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsOptionalFieldPresenceBaseBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500626173652D626F78::TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78(
                TerraneNsOptionalFieldPresenceDerivedBoxIdentity2F6F7074696F6E616C2D6669656C642D70726573656E636500646572697665642D626F78::terrane_construct(),
            ),
        ),
    );
    shared = shared_owner.clone();
    inspect_shared(shared.clone());
    observer = std::sync::Arc::downgrade(&shared_owner.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_2 =
        (&observer).upgrade().expect("reference expired"); __terrane_owner_2.lock()
        .expect("reference lock poisoned").terrane_field_value().is_some() }),
        terrane_scalar_support::scalar_text(&{ let __terrane_owner_3 = (&observer)
        .upgrade().expect("reference expired"); __terrane_owner_3.lock()
        .expect("reference lock poisoned").terrane_field_value().is_none() })
    );
    {
        let __terrane_owner_4 = shared.clone();
        let __terrane_value = None;
        *__terrane_owner_4
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_value_mut() = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_5 = &shared;
        __terrane_owner_5.lock().expect("shared reference lock poisoned")
        .terrane_field_value().is_none() })
    );
    required = RequiredBox::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&(&required.value).is_some()));
    {
        let __terrane_value = None;
        required.value = __terrane_value;
    }
    println!("{}", terrane_scalar_support::scalar_text(&(&required.value).is_none()));
    outer = std::sync::Arc::new(TerraneSharedCell::new(OuterBox::terrane_construct()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_6 = &outer;
        __terrane_owner_6.lock().expect("shared reference lock poisoned").inner
        .terrane_field_value().is_some() })
    );
    {
        let __terrane_owner_7 = outer.clone();
        let __terrane_value = None;
        *__terrane_owner_7
            .lock()
            .expect("shared reference lock poisoned")
            .inner
            .terrane_field_value_mut() = __terrane_value;
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_8 = &outer;
        __terrane_owner_8.lock().expect("shared reference lock poisoned").inner
        .terrane_field_value().is_none() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_9 =
        &make_box(); (&__terrane_receiver_value_9.value).is_some() })
    );
}
// Source: tests/integration/optional-generic-field-default/case.trn
// Namespace: optional-generic-field-default
#[derive(Clone)]
pub struct TerraneNsOptionalGenericFieldDefaultBxIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74004278<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsOptionalGenericFieldDefaultBxIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74004278<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsOptionalGenericFieldDefaultHolderIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C7400486F6C646572 {
    pub value: Option<
        TerraneNsOptionalGenericFieldDefaultBxIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74004278<
            terrane_int_support::Int,
        >,
    >,
}
impl TerraneNsOptionalGenericFieldDefaultHolderIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C7400486F6C646572 {
    pub fn terrane_construct() -> Self {
        Self { value: None }
    }
}
pub trait TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
    >;
    fn name(&self) -> String;
}
impl Clone
for Box<
    dyn TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
> {
    fn clone(&self) -> Self {
        TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol::clone_box(
            &**self,
        )
    }
}
pub struct TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564(
    Box<
        dyn TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
    >,
);
impl Clone
for TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564 {
    pub fn name(&self) -> String {
        self.0.name()
    }
}
#[derive(Clone)]
pub struct Wrapped<
    TerraneType54: TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54: TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
> Wrapped<TerraneType54> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
#[derive(Clone)]
pub struct Forwarded<
    TerraneType55: TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType55)>,
    pub value: Option<Wrapped<TerraneType55>>,
}
impl<
    TerraneType55: TerraneNsOptionalGenericFieldDefaultNamedIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C74006E616D6564Protocol,
> Forwarded<TerraneType55> {
    pub fn terrane_construct(value: Wrapped<TerraneType55>) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: Wrapped<TerraneType55>) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_optional_generic_field_default() {
    let holder: TerraneNsOptionalGenericFieldDefaultHolderIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C7400486F6C646572;
    holder = TerraneNsOptionalGenericFieldDefaultHolderIdentity2F6F7074696F6E616C2D67656E657269632D6669656C642D64656661756C7400486F6C646572::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&holder.value.is_none()));
}
// Source: tests/integration/recursive-generic-phantom/case.trn
// Namespace: recursive-generic-phantom
#[derive(Clone)]
pub struct TerraneNsRecursiveGenericPhantomNIdentity2F7265637572736976652D67656E657269632D7068616E746F6D004E<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub next: terrane_collection_support::List<
        TerraneNsRecursiveGenericPhantomNIdentity2F7265637572736976652D67656E657269632D7068616E746F6D004E<
            TerraneType54,
        >,
    >,
    pub n: terrane_int_support::Int,
}
impl<
    TerraneType54,
> TerraneNsRecursiveGenericPhantomNIdentity2F7265637572736976652D67656E657269632D7068616E746F6D004E<
    TerraneType54,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            next: terrane_collection_support::List::<
                TerraneNsRecursiveGenericPhantomNIdentity2F7265637572736976652D67656E657269632D7068616E746F6D004E<
                    TerraneType54,
                >,
            >::new(vec![]),
            n: terrane_int_support::Int::from(1_i128),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsRecursiveGenericPhantomAIdentity2F7265637572736976652D67656E657269632D7068616E746F6D0041<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub next: terrane_collection_support::List<B<TerraneType54>>,
}
impl<
    TerraneType54,
> TerraneNsRecursiveGenericPhantomAIdentity2F7265637572736976652D67656E657269632D7068616E746F6D0041<
    TerraneType54,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            next: terrane_collection_support::List::<B<TerraneType54>>::new(vec![]),
        }
    }
}
#[derive(Clone)]
pub struct B<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub next: terrane_collection_support::List<
        TerraneNsRecursiveGenericPhantomAIdentity2F7265637572736976652D67656E657269632D7068616E746F6D0041<
            TerraneType54,
        >,
    >,
}
impl<TerraneType54> B<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            next: terrane_collection_support::List::<
                TerraneNsRecursiveGenericPhantomAIdentity2F7265637572736976652D67656E657269632D7068616E746F6D0041<
                    TerraneType54,
                >,
            >::new(vec![]),
        }
    }
}
fn test_scenario_terrane_recursive_generic_phantom() {
    let g: TerraneNsRecursiveGenericPhantomNIdentity2F7265637572736976652D67656E657269632D7068616E746F6D004E<
        terrane_int_support::Int,
    >;
    let a: TerraneNsRecursiveGenericPhantomAIdentity2F7265637572736976652D67656E657269632D7068616E746F6D0041<
        terrane_int_support::Int,
    >;
    g = TerraneNsRecursiveGenericPhantomNIdentity2F7265637572736976652D67656E657269632D7068616E746F6D004E::<
        terrane_int_support::Int,
    >::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&g.n));
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(g.next
        .length()))
    );
    a = TerraneNsRecursiveGenericPhantomAIdentity2F7265637572736976652D67656E657269632D7068616E746F6D0041::<
        terrane_int_support::Int,
    >::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(a.next
        .length()))
    );
}
// Source: tests/integration/required-field-reference/case.trn
// Namespace: required-field-reference
#[derive(Clone)]
pub struct TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(4_i128),
        }
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572Storage {
    pub inner: Option<
        TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    >,
}
impl TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572Storage {
    pub fn terrane_construct(
        inner: TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    ) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        inner: TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    ) {
        {
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        let alias: &TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572;
        alias = self.inner.as_ref().expect("required field initialized");
        return alias.clone().peek();
    }
}
#[derive(Clone)]
pub enum TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572 {
    Own(
        TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572Storage,
    ),
    DerivedOuter(DerivedOuter),
}
impl TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572 {
    pub fn terrane_construct(
        inner: TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    ) -> Self {
        Self::Own(
            TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572Storage::terrane_construct(
                inner,
            ),
        )
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.peek(),
            Self::DerivedOuter(value) => value.peek(),
        }
    }
    pub fn terrane_field_inner(
        &self,
    ) -> &TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572 {
        match self {
            Self::Own(value) => value.inner.as_ref().expect("required field initialized"),
            Self::DerivedOuter(value) => {
                value.inner.as_ref().expect("required field initialized")
            }
        }
    }
    pub fn terrane_field_inner_mut(
        &mut self,
    ) -> &mut TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572 {
        match self {
            Self::Own(value) => value.inner.as_mut().expect("required field initialized"),
            Self::DerivedOuter(value) => {
                value.inner.as_mut().expect("required field initialized")
            }
        }
    }
    pub fn terrane_field_inner_slot_mut(
        &mut self,
    ) -> &mut Option<
        TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    > {
        match self {
            Self::Own(value) => &mut value.inner,
            Self::DerivedOuter(value) => &mut value.inner,
        }
    }
}
#[derive(Clone)]
pub struct DerivedOuter {
    pub inner: Option<
        TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    >,
}
impl DerivedOuter {
    pub fn terrane_construct(
        inner: TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    ) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        inner: TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572,
    ) {
        {
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
    }
    pub fn peek(&self) -> terrane_int_support::Int {
        let alias: &TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572;
        alias = self.inner.as_ref().expect("required field initialized");
        return alias.clone().peek();
    }
}
fn test_scenario_terrane_required_field_reference() {
    let o: TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572;
    let alias: &TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572;
    let d: DerivedOuter;
    let derived_alias: &TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572;
    o = TerraneNsRequiredFieldReferenceOuterIdentity2F72657175697265642D6669656C642D7265666572656E6365006F75746572::terrane_construct(
        TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572::terrane_construct(),
    );
    alias = o.terrane_field_inner();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&alias.clone().peek()),
        terrane_scalar_support::scalar_text(&o.peek())
    );
    d = DerivedOuter::terrane_construct(
        TerraneNsRequiredFieldReferenceHolderIdentity2F72657175697265642D6669656C642D7265666572656E636500686F6C646572::terrane_construct(),
    );
    derived_alias = d.inner.as_ref().expect("required field initialized");
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&derived_alias.clone().peek()),
        terrane_scalar_support::scalar_text(&d.peek())
    );
}
// Source: tests/integration/shared-object-field-comparisons/case.trn
// Namespace: shared-object-field-comparisons
#[derive(Clone)]
pub struct TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572Storage {
    pub a: Duration,
    pub b: Duration,
}
impl TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572Storage {
    pub fn terrane_construct() -> Self {
        Self {
            a: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),

                    39 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:5:18-5:38 */,
            ),
            b: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(2_i128)),

                    40 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:6:18-6:38 */,
            ),
        }
    }
}
#[derive(Clone)]
pub enum TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572 {
    Own(
        TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572Storage,
    ),
    InheritedHolder(InheritedHolder),
}
impl TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572Storage::terrane_construct(),
        )
    }
    pub fn terrane_field_a(&self) -> &Duration {
        match self {
            Self::Own(value) => &value.a,
            Self::InheritedHolder(value) => &value.a,
        }
    }
    pub fn terrane_field_a_mut(&mut self) -> &mut Duration {
        match self {
            Self::Own(value) => &mut value.a,
            Self::InheritedHolder(value) => &mut value.a,
        }
    }
    pub fn terrane_field_b(&self) -> &Duration {
        match self {
            Self::Own(value) => &value.b,
            Self::InheritedHolder(value) => &value.b,
        }
    }
    pub fn terrane_field_b_mut(&mut self) -> &mut Duration {
        match self {
            Self::Own(value) => &mut value.b,
            Self::InheritedHolder(value) => &mut value.b,
        }
    }
}
#[derive(Clone)]
pub struct InheritedHolder {
    pub a: Duration,
    pub b: Duration,
}
impl InheritedHolder {
    pub fn terrane_construct() -> Self {
        Self {
            a: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),

                    39 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:5:18-5:38 */,
            ),
            b: __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(2_i128)),

                    40 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:6:18-6:38 */,
            ),
        }
    }
}
#[derive(Clone)]
pub struct RequiredHolder {
    pub a: Option<Duration>,
    pub b: Option<Duration>,
}
impl RequiredHolder {
    pub fn terrane_construct(a: Duration, b: Duration) -> Self {
        let mut __terrane_constructed_value = Self { a: None, b: None };
        __terrane_constructed_value.construct(a, b);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, a: Duration, b: Duration) {
        {
            let __terrane_value = a;
            self.a = Some(__terrane_value);
        }
        {
            let __terrane_value = b;
            self.b = Some(__terrane_value);
        }
    }
}
fn replace(
    owner: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572,
        >,
    >,
) -> Result<Duration, TerraneError> {
    {
        let __terrane_owner_0 = owner.clone();
        let __terrane_value = __terrane_traced_err(
            Duration::terrane_static_seconds(terrane_int_support::Int::from(2_i128)),

                41 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:18:15-18:35 */,
        )?;
        *__terrane_owner_0
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_a_mut() = __terrane_value;
    }
    return Ok({
        let __terrane_snapshot = owner
            .lock()
            .expect("shared reference lock poisoned")
            .terrane_field_a()
            .clone();
        __terrane_snapshot
    });
}
fn test_scenario_terrane_shared_object_field_comparisons() -> Result<(), TerraneError> {
    let h: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572,
        >,
    >;
    let other: Duration;
    let alias: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572,
        >,
    >;
    let observer: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572,
        >,
    >;
    let ordinary: TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572;
    let inherited: std::sync::Arc<TerraneSharedCell<InheritedHolder>>;
    let initialized: std::sync::Arc<TerraneSharedCell<RequiredHolder>>;
    h = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572::terrane_construct(),
        ),
    );
    other = __terrane_traced_err(
        Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),

            42 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:23:13-23:33 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &other; < Duration as std::cmp::PartialEq > ::eq(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
        &other; let __terrane_comparison_right = &{ let __terrane_snapshot = h.lock()
        .expect("shared reference lock poisoned").terrane_field_a().clone();
        __terrane_snapshot }; < Duration as std::cmp::PartialEq >
        ::eq(__terrane_comparison_left, __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_b().clone(); __terrane_snapshot }; < Duration as
        std::cmp::PartialOrd > ::lt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_b().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; < Duration as
        std::cmp::PartialOrd > ::gt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    alias = h.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = alias.lock()
        .expect("shared reference lock poisoned").terrane_field_b().clone();
        __terrane_snapshot }; < Duration as std::cmp::PartialOrd >
        ::le(__terrane_comparison_left, __terrane_comparison_right) })
    );
    observer = std::sync::Arc::downgrade(&h);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = { let __terrane_owner = observer.upgrade()
        .expect("reference expired"); __terrane_owner.lock()
        .expect("reference lock poisoned").terrane_field_a().clone() };
        __terrane_snapshot }; let __terrane_comparison_right = &{ let __terrane_snapshot
        = h.lock().expect("shared reference lock poisoned").terrane_field_b().clone();
        __terrane_snapshot }; < Duration as std::cmp::PartialOrd >
        ::lt(__terrane_comparison_left, __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &__terrane_traced_err(replace(alias.clone()), 43 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:32:19-32:33 */) ?; <
        Duration as std::cmp::PartialOrd > ::lt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_a().clone(); __terrane_snapshot }; let __terrane_comparison_right
        = &{ let __terrane_snapshot = h.lock().expect("shared reference lock poisoned")
        .terrane_field_b().clone(); __terrane_snapshot }; < Duration as
        std::cmp::PartialEq > ::eq(__terrane_comparison_left, __terrane_comparison_right)
        })
    );
    ordinary = TerraneNsSharedObjectFieldComparisonsHolderIdentity2F7368617265642D6F626A6563742D6669656C642D636F6D70617269736F6E7300686F6C646572::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left =
        &ordinary.terrane_field_a(); let __terrane_comparison_right = &ordinary
        .terrane_field_b(); < Duration as std::cmp::PartialOrd >
        ::lt(__terrane_comparison_left, __terrane_comparison_right) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&ordinary.terrane_field_a().clone()
        .seconds)
    );
    inherited = std::sync::Arc::new(
        TerraneSharedCell::new(InheritedHolder::terrane_construct()),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = inherited.lock()
        .expect("shared reference lock poisoned").a.clone(); __terrane_snapshot }; let
        __terrane_comparison_right = &{ let __terrane_snapshot = inherited.lock()
        .expect("shared reference lock poisoned").b.clone(); __terrane_snapshot }; <
        Duration as std::cmp::PartialOrd > ::lt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    initialized = std::sync::Arc::new(
        TerraneSharedCell::new(
            RequiredHolder::terrane_construct(
                __terrane_traced_err(
                    Duration::terrane_static_seconds(
                        terrane_int_support::Int::from(1_i128),
                    ),

                        44 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:39:58-39:78 */,
                )?,
                __terrane_traced_err(
                    Duration::terrane_static_seconds(
                        terrane_int_support::Int::from(2_i128),
                    ),

                        45 /* terrane-site: tests/integration/shared-object-field-comparisons/case.trn:39:82-39:102 */,
                )?,
            ),
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_comparison_left = &{
        let __terrane_snapshot = initialized.lock()
        .expect("shared reference lock poisoned").a.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot }; let
        __terrane_comparison_right = &{ let __terrane_snapshot = initialized.lock()
        .expect("shared reference lock poisoned").b.as_ref()
        .expect("required field initialized").clone(); __terrane_snapshot }; < Duration
        as std::cmp::PartialOrd > ::lt(__terrane_comparison_left,
        __terrane_comparison_right) })
    );
    Ok(())
}
// Source: tests/integration/source-associated-interface/case.trn
// Namespace: source-associated-interface
pub trait RequestProtocol<TerraneType726573706F6E7365>: Send + Sync {
    fn clone_box(&self) -> Box<dyn RequestProtocol<TerraneType726573706F6E7365>>;
    fn separate_box(&self) -> Box<dyn RequestProtocol<TerraneType726573706F6E7365>>;
    fn dispatch(&self, input: String) -> TerraneType726573706F6E7365;
}
impl<TerraneType726573706F6E7365> Clone
for Box<dyn RequestProtocol<TerraneType726573706F6E7365>> {
    fn clone(&self) -> Self {
        RequestProtocol::clone_box(&**self)
    }
}
pub struct Request<TerraneType726573706F6E7365>(
    Box<dyn RequestProtocol<TerraneType726573706F6E7365>>,
);
impl<TerraneType726573706F6E7365> Clone for Request<TerraneType726573706F6E7365> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<TerraneType726573706F6E7365> Request<TerraneType726573706F6E7365> {
    pub fn dispatch(&self, input: String) -> TerraneType726573706F6E7365 {
        self.0.dispatch(input)
    }
}
#[derive(Clone)]
pub struct NumericRequest {
    pub __trn_74797065: terrane_int_support::Int,
}
impl NumericRequest {
    pub fn terrane_construct() -> Self {
        Self {
            __trn_74797065: terrane_int_support::Int::from(17_i128),
        }
    }
    pub fn dispatch(&self, input: String) -> terrane_int_support::Int {
        let _ = &input;
        return terrane_int_support::Int::from(40_i128);
    }
}
impl RequestProtocol<terrane_int_support::Int> for NumericRequest {
    fn clone_box(&self) -> Box<dyn RequestProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RequestProtocol<terrane_int_support::Int>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, input: String) -> terrane_int_support::Int {
        NumericRequest::dispatch(&*self, input)
    }
}
impl From<NumericRequest> for Request<terrane_int_support::Int> {
    fn from(value: NumericRequest) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TextRequest {}
impl TextRequest {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn dispatch(&self, input: String) -> String {
        return input;
    }
}
impl RequestProtocol<String> for TextRequest {
    fn clone_box(&self) -> Box<dyn RequestProtocol<String>> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn RequestProtocol<String>> {
        Box::new(self.clone())
    }
    fn dispatch(&self, input: String) -> String {
        TextRequest::dispatch(&*self, input)
    }
}
impl From<TextRequest> for Request<String> {
    fn from(value: TextRequest) -> Self {
        Self(Box::new(value))
    }
}
fn test_scenario_terrane_source_associated_interface() {
    let ordinary: NumericRequest;
    let numeric: Request<terrane_int_support::Int>;
    let count: terrane_int_support::Int;
    let textual: Request<String>;
    let message: String;
    ordinary = NumericRequest::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&ordinary.__trn_74797065));
    numeric = <Request<
        terrane_int_support::Int,
    >>::from(NumericRequest::terrane_construct());
    count = numeric.dispatch(String::from("ignored"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(count.clone() +
        terrane_int_support::Int::from(2_i128)))
    );
    textual = <Request<String>>::from(TextRequest::terrane_construct());
    message = textual.dispatch(String::from("distinct"));
    println!("{}", terrane_scalar_support::scalar_text(&message));
}
// Source: tests/integration/source-finally-only-rewrite/case.trn
// Namespace: source-finally-only-rewrite
#[derive(Clone)]
pub struct K {
    pub n: terrane_int_support::Int,
}
impl K {
    pub fn terrane_construct() -> Self {
        Self {
            n: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn finish(self) -> terrane_int_support::Int {
        return self.n.clone();
    }
}
fn eat_terrane_source_finally_only_rewrite(
    values: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    return __terrane_raised(
        values
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),

                        46 /* terrane-site: tests/integration/source-finally-only-rewrite/case.trn:8:12-8:21 */,
                ),
            ),

            46 /* terrane-site: tests/integration/source-finally-only-rewrite/case.trn:8:12-8:21 */,
    );
}
fn test_scenario_terrane_source_finally_only_rewrite() {
    let mut k: Option<K> = None;
    let mut values: Option<terrane_collection_support::List<terrane_int_support::Int>> = None;
    let _ = k.insert(K::terrane_construct());
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&k.take()
                .expect("flow-proven availability").finish())
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        let _ = k.insert(K::terrane_construct());
        println!(
            "{}", terrane_scalar_support::scalar_text(&k.as_ref()
            .expect("flow-proven available binding").clone().n)
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let _ = values
        .insert(
            terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(3_i128)]),
        );
    let mut __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&eat_terrane_source_finally_only_rewrite(values
                .as_ref().expect("flow-proven available binding").clone()))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_1: TerraneCompletion<()> = (|| {
        let _ = values
            .insert(
                terrane_collection_support::List::<
                    terrane_int_support::Int,
                >::new(vec![terrane_int_support::Int::from(4_i128)]),
            );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(values
            .as_ref().expect("flow-proven available binding")
            .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            47 /* terrane-site: tests/integration/source-finally-only-rewrite/case.trn:21:16-21:25 */)),
            47 /* terrane-site: tests/integration/source-finally-only-rewrite/case.trn:21:16-21:25 */))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/source-generic-field-read/case.trn
// Namespace: source-generic-field-read
#[derive(Clone)]
pub struct TerraneNsSourceGenericFieldReadBoxIdentity2F736F757263652D67656E657269632D6669656C642D7265616400426F78<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsSourceGenericFieldReadBoxIdentity2F736F757263652D67656E657269632D6669656C642D7265616400426F78<
    TerraneType54,
> {
    pub fn terrane_construct(value: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_phantom: std::marker::PhantomData,
            value: None,
        };
        __terrane_constructed_value.construct(value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, value: TerraneType54) {
        {
            let __terrane_value = value;
            self.value = Some(__terrane_value);
        }
    }
    pub fn read(mut self) -> TerraneType54 {
        return self.value.take().expect("required field initialized");
    }
}
fn test_scenario_terrane_source_generic_field_read() {
    let __trn_626f78: TerraneNsSourceGenericFieldReadBoxIdentity2F736F757263652D67656E657269632D6669656C642D7265616400426F78<
        String,
    >;
    __trn_626f78 = TerraneNsSourceGenericFieldReadBoxIdentity2F736F757263652D67656E657269632D6669656C642D7265616400426F78::<
        String,
    >::terrane_construct(String::from("hello"));
    println!("{}", terrane_scalar_support::scalar_text(&__trn_626f78.read()));
}
// Source: tests/integration/source-method-builtin-name-dispatch/case.trn
// Namespace: source-method-builtin-name-dispatch
#[derive(Clone)]
pub struct TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone() * amount.clone();
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn parse(&self) -> terrane_int_support::Int {
        return self.count.clone();
    }
    pub fn radix(
        &self,
        first: terrane_int_support::Int,
        second: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return self.count.clone() + first.clone() + second.clone();
    }
}
#[derive(Clone)]
pub struct TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365Storage {
    pub count: terrane_int_support::Int,
}
impl TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365Storage {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone() * amount.clone();
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn parse(&self) -> terrane_int_support::Int {
        return self.count.clone();
    }
    pub fn radix(
        &self,
        first: terrane_int_support::Int,
        second: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return self.count.clone() + first.clone() + second.clone();
    }
}
#[derive(Clone)]
pub enum TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365 {
    Own(
        TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365Storage,
    ),
    TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
        TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564,
    ),
}
impl TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365 {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365Storage::terrane_construct(),
        )
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.add(amount),
            Self::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                value,
            ) => value.add(amount),
        }
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.multiply(amount),
            Self::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                value,
            ) => value.multiply(amount),
        }
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.encode(amount),
            Self::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                value,
            ) => value.encode(amount),
        }
    }
    pub fn parse(&self) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.parse(),
            Self::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                value,
            ) => value.parse(),
        }
    }
    pub fn radix(
        &self,
        first: terrane_int_support::Int,
        second: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.radix(first, second),
            Self::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                value,
            ) => value.radix(first, second),
        }
    }
    pub fn terrane_field_count(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.count,
            Self::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                value,
            ) => &value.count,
        }
    }
    pub fn terrane_field_count_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.count,
            Self::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                value,
            ) => &mut value.count,
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564 {
    pub count: terrane_int_support::Int,
    pub marker: bool,
}
impl TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564 {
    pub fn terrane_construct() -> Self {
        Self {
            count: terrane_int_support::Int::from(1_i128),
            marker: true,
        }
    }
    pub fn add(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn multiply(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.count.clone() * amount.clone();
            self.count = __terrane_value;
        }
        return self.count.clone();
    }
    pub fn encode(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.count.clone() + amount.clone();
    }
    pub fn parse(&self) -> terrane_int_support::Int {
        return self.count.clone();
    }
    pub fn radix(
        &self,
        first: terrane_int_support::Int,
        second: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return self.count.clone() + first.clone() + second.clone();
    }
}
fn ordinary_plain(
    value: &TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E,
) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .add(terrane_int_support::Int::from(2_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.clone().parse()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .radix(terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .encode(terrane_int_support::Int::from(3_i128)))
    );
}
fn ordinary_base(
    value: &TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365,
) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .add(terrane_int_support::Int::from(2_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .encode(terrane_int_support::Int::from(3_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.clone().parse()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&value.clone()
        .radix(terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128)))
    );
}
fn shared_plain(
    value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E,
        >,
    >,
) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_1 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_1.lock()
        .expect("shared reference lock poisoned").add(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_3 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_3.lock()
        .expect("shared reference lock poisoned").multiply(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_5 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(3_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_5.lock()
        .expect("shared reference lock poisoned").encode(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_7 = value
        .clone(); let __terrane_call_result = __terrane_owner_7.lock()
        .expect("shared reference lock poisoned").parse(); __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_9 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        terrane_int_support::Int::from(3_i128); let __terrane_shared_argument_2 =
        __terrane_shared_argument_0; let __terrane_shared_argument_3 =
        __terrane_shared_argument_1; let __terrane_call_result = __terrane_owner_9.lock()
        .expect("shared reference lock poisoned").radix(__terrane_shared_argument_2,
        __terrane_shared_argument_3); __terrane_call_result })
    );
}
fn shared_base(
    value: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365,
        >,
    >,
) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_11 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_11
        .lock().expect("shared reference lock poisoned")
        .add(__terrane_shared_argument_1); __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_13 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_13
        .lock().expect("shared reference lock poisoned")
        .multiply(__terrane_shared_argument_1); __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_15 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(3_i128); let __terrane_shared_argument_1 =
        __terrane_shared_argument_0; let __terrane_call_result = __terrane_owner_15
        .lock().expect("shared reference lock poisoned")
        .encode(__terrane_shared_argument_1); __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_17 = value
        .clone(); let __terrane_call_result = __terrane_owner_17.lock()
        .expect("shared reference lock poisoned").parse(); __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_19 = value
        .clone(); let __terrane_shared_argument_0 =
        terrane_int_support::Int::from(2_i128); let __terrane_shared_argument_1 =
        terrane_int_support::Int::from(3_i128); let __terrane_shared_argument_2 =
        __terrane_shared_argument_0; let __terrane_shared_argument_3 =
        __terrane_shared_argument_1; let __terrane_call_result = __terrane_owner_19
        .lock().expect("shared reference lock poisoned")
        .radix(__terrane_shared_argument_2, __terrane_shared_argument_3);
        __terrane_call_result })
    );
}
fn test_scenario_terrane_source_method_builtin_name_dispatch() {
    let first: TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E;
    let second: TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365;
    let third: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E,
        >,
    >;
    let fourth: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365,
        >,
    >;
    let fifth: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E,
        >,
    >;
    let fifth_held: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E,
        >,
    >;
    let fifth_observer: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E,
        >,
    >;
    let sixth: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365,
        >,
    >;
    let sixth_held: std::sync::Arc<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365,
        >,
    >;
    let sixth_observer: std::sync::Weak<
        TerraneSharedCell<
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365,
        >,
    >;
    first = TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&first.parse()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&first
        .radix(terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128)))
    );
    ordinary_plain(&first);
    second = TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
        TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564::terrane_construct(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&second.parse()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&second
        .radix(terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128)))
    );
    ordinary_base(&second);
    third = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E::terrane_construct(),
        ),
    );
    shared_plain(third.clone());
    fourth = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564::terrane_construct(),
            ),
        ),
    );
    shared_base(fourth.clone());
    fifth = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSourceMethodBuiltinNameDispatchPlainIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D646973706174636800706C61696E::terrane_construct(),
        ),
    );
    fifth_held = fifth.clone();
    fifth_observer = std::sync::Arc::downgrade(&fifth.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_21 =
        fifth_observer.upgrade().expect("reference expired"); let
        __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner_21.lock()
        .expect("reference lock poisoned").multiply(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = fifth_held
        .lock().expect("shared reference lock poisoned").count.clone();
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_23 =
        fifth_observer.upgrade().expect("reference expired"); let __terrane_call_result =
        __terrane_owner_23.lock().expect("reference lock poisoned").parse();
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_25 =
        fifth_observer.upgrade().expect("reference expired"); let
        __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = terrane_int_support::Int::from(3_i128); let
        __terrane_shared_argument_2 = __terrane_shared_argument_0; let
        __terrane_shared_argument_3 = __terrane_shared_argument_1; let
        __terrane_call_result = __terrane_owner_25.lock()
        .expect("reference lock poisoned").radix(__terrane_shared_argument_2,
        __terrane_shared_argument_3); __terrane_call_result })
    );
    sixth = std::sync::Arc::new(
        TerraneSharedCell::new(
            TerraneNsSourceMethodBuiltinNameDispatchBaseIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680062617365::TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564(
                TerraneNsSourceMethodBuiltinNameDispatchDerivedIdentity2F736F757263652D6D6574686F642D6275696C74696E2D6E616D652D64697370617463680064657269766564::terrane_construct(),
            ),
        ),
    );
    sixth_held = sixth.clone();
    sixth_observer = std::sync::Arc::downgrade(&sixth.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_27 =
        sixth_observer.upgrade().expect("reference expired"); let
        __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = __terrane_shared_argument_0; let
        __terrane_call_result = __terrane_owner_27.lock()
        .expect("reference lock poisoned").multiply(__terrane_shared_argument_1);
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = sixth_held
        .lock().expect("shared reference lock poisoned").terrane_field_count().clone();
        __terrane_snapshot })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_29 =
        sixth_observer.upgrade().expect("reference expired"); let __terrane_call_result =
        __terrane_owner_29.lock().expect("reference lock poisoned").parse();
        __terrane_call_result })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner_31 =
        sixth_observer.upgrade().expect("reference expired"); let
        __terrane_shared_argument_0 = terrane_int_support::Int::from(2_i128); let
        __terrane_shared_argument_1 = terrane_int_support::Int::from(3_i128); let
        __terrane_shared_argument_2 = __terrane_shared_argument_0; let
        __terrane_shared_argument_3 = __terrane_shared_argument_1; let
        __terrane_call_result = __terrane_owner_31.lock()
        .expect("reference lock poisoned").radix(__terrane_shared_argument_2,
        __terrane_shared_argument_3); __terrane_call_result })
    );
}
// Source: tests/integration/source-mutable-receiver-replace/case.trn
// Namespace: source-mutable-receiver-replace
#[derive(Clone)]
pub struct TerraneNsSourceMutableReceiverReplaceAIdentity2F736F757263652D6D757461626C652D72656365697665722D7265706C6163650041 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsSourceMutableReceiverReplaceAIdentity2F736F757263652D6D757461626C652D72656365697665722D7265706C6163650041 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn replace(&mut self) {
        *self = TerraneNsSourceMutableReceiverReplaceAIdentity2F736F757263652D6D757461626C652D72656365697665722D7265706C6163650041::terrane_construct();
        {
            let __terrane_value = terrane_int_support::Int::from(7_i128);
            self.value = __terrane_value;
        }
    }
}
fn test_scenario_terrane_source_mutable_receiver_replace() {
    let mut a: TerraneNsSourceMutableReceiverReplaceAIdentity2F736F757263652D6D757461626C652D72656365697665722D7265706C6163650041;
    a = TerraneNsSourceMutableReceiverReplaceAIdentity2F736F757263652D6D757461626C652D72656365697665722D7265706C6163650041::terrane_construct();
    a.replace();
    println!("{}", terrane_scalar_support::scalar_text(&a.value));
}
// Source: tests/integration/static-class-singleton/case.trn
// Namespace: static-class-singleton
pub static TERRANE_STATIC_TERRANENSSTATICCLASSSINGLETONWIDGETIDENTITY2F7374617469632D636C6173732D73696E676C65746F6E00776964676574_CURRENT: std::sync::LazyLock<
    TerraneSharedCell<
        Option<
            TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574,
        >,
    >,
> = std::sync::LazyLock::new(|| TerraneSharedCell::new(None));
#[derive(Clone)]
pub struct TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574 {}
impl TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn class_name(&self) -> String {
        return TerraneDescriptor {
            identity: "/static-class-singleton::widget",
            name: "widget",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[],
        }
            .name
            .to_owned();
    }
    pub fn terrane_static_shared() -> TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574 {
        if {
            let __terrane_value = TERRANE_STATIC_TERRANENSSTATICCLASSSINGLETONWIDGETIDENTITY2F7374617469632D636C6173732D73696E676C65746F6E00776964676574_CURRENT
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        }
            .is_none()
        {
            {
                let __terrane_static_value = Some(
                    TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574::terrane_construct(),
                );
                *TERRANE_STATIC_TERRANENSSTATICCLASSSINGLETONWIDGETIDENTITY2F7374617469632D636C6173732D73696E676C65746F6E00776964676574_CURRENT
                    .lock()
                    .expect("static field lock poisoned") = __terrane_static_value;
            }
        }
        return {
            let __terrane_value = TERRANE_STATIC_TERRANENSSTATICCLASSSINGLETONWIDGETIDENTITY2F7374617469632D636C6173732D73696E676C65746F6E00776964676574_CURRENT
                .lock()
                .expect("static field lock poisoned")
                .clone();
            __terrane_value
        }
            .as_ref()
            .expect("semantic optional narrowing")
            .clone();
    }
}
fn test_scenario_terrane_static_class_singleton() {
    let first: TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574;
    let second: TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574;
    first = TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574::terrane_static_shared();
    second = TerraneNsStaticClassSingletonWidgetIdentity2F7374617469632D636C6173732D73696E676C65746F6E00776964676574::terrane_static_shared();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&first.class_name()),
        terrane_scalar_support::scalar_text(&second.class_name())
    );
}
// Source: tests/integration/static-explicit-generic/case.trn
// Namespace: static-explicit-generic
#[derive(Clone)]
pub struct TerraneNsStaticExplicitGenericNIdentity2F7374617469632D6578706C696369742D67656E65726963004E {}
impl TerraneNsStaticExplicitGenericNIdentity2F7374617469632D6578706C696369742D67656E65726963004E {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_g<TerraneType54>() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(1_i128);
    }
}
#[derive(Clone)]
pub struct TerraneNsStaticExplicitGenericGIdentity2F7374617469632D6578706C696369742D67656E657269630047<
    TerraneType54,
> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
}
impl<
    TerraneType54,
> TerraneNsStaticExplicitGenericGIdentity2F7374617469632D6578706C696369742D67656E657269630047<
    TerraneType54,
> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
        }
    }
    pub fn terrane_static_g<TerraneType55>() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
}
fn test_scenario_terrane_static_explicit_generic() {
    let f: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&TerraneNsStaticExplicitGenericNIdentity2F7374617469632D6578706C696369742D67656E65726963004E::terrane_static_g::
        < terrane_int_support::Int > ())
    );
    f = std::sync::Arc::new(
        TerraneNsStaticExplicitGenericNIdentity2F7374617469632D6578706C696369742D67656E65726963004E::terrane_static_g::<
            String,
        >,
    );
    println!("{}", terrane_scalar_support::scalar_text(&f()));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&TerraneNsStaticExplicitGenericGIdentity2F7374617469632D6578706C696369742D67656E657269630047::
        < terrane_int_support::Int > ::terrane_static_g:: < String > ())
    );
}
// Source: tests/integration/static-field-member-assignment/case.trn
// Namespace: static-field-member-assignment
#[derive(Clone)]
pub struct TerraneNsStaticFieldMemberAssignmentInnerIdentity2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400696E6E6572 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsStaticFieldMemberAssignmentInnerIdentity2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400696E6E6572 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
}
pub static TERRANE_STATIC_TERRANENSSTATICFIELDMEMBERASSIGNMENTHOLDERIDENTITY2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400686F6C646572_SLOT: std::sync::LazyLock<
    TerraneStaticObject<
        TerraneNsStaticFieldMemberAssignmentInnerIdentity2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400696E6E6572,
    >,
> = std::sync::LazyLock::new(|| TerraneStaticObject::new(
    TerraneNsStaticFieldMemberAssignmentInnerIdentity2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400696E6E6572::terrane_construct(),
));
#[derive(Clone)]
pub struct TerraneNsStaticFieldMemberAssignmentHolderIdentity2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400686F6C646572 {}
impl TerraneNsStaticFieldMemberAssignmentHolderIdentity2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400686F6C646572 {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_update() {
        {
            let __terrane_owner_0 = TERRANE_STATIC_TERRANENSSTATICFIELDMEMBERASSIGNMENTHOLDERIDENTITY2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400686F6C646572_SLOT
                .handle();
            let __terrane_value = terrane_int_support::Int::from(5_i128);
            __terrane_owner_0.lock().expect("static field lock poisoned").value = __terrane_value;
        }
    }
}
fn test_scenario_terrane_static_field_member_assignment() {
    TerraneNsStaticFieldMemberAssignmentHolderIdentity2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400686F6C646572::terrane_static_update();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
        __terrane_result =
        TERRANE_STATIC_TERRANENSSTATICFIELDMEMBERASSIGNMENTHOLDERIDENTITY2F7374617469632D6669656C642D6D656D6265722D61737369676E6D656E7400686F6C646572_SLOT
        .handle().lock().expect("static field lock poisoned").value.clone();
        __terrane_result }; __terrane_snapshot })
    );
}
// Source: tests/integration/trait-field-defaults/case.trn
// Namespace: trait-field-defaults
#[derive(Clone)]
pub struct TerraneNsTraitFieldDefaultsPayloadIdentity2F74726169742D6669656C642D64656661756C7473007061796C6F6164 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsTraitFieldDefaultsPayloadIdentity2F74726169742D6669656C642D64656661756C7473007061796C6F6164 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(9_i128),
        }
    }
}
#[derive(Clone)]
pub struct User {
    pub slot: terrane_int_support::Int,
    pub payload: TerraneNsTraitFieldDefaultsPayloadIdentity2F74726169742D6669656C642D64656661756C7473007061796C6F6164,
}
impl User {
    pub fn terrane_construct() -> Self {
        Self {
            slot: terrane_int_support::Int::from(0_i128),
            payload: TerraneNsTraitFieldDefaultsPayloadIdentity2F74726169742D6669656C642D64656661756C7473007061796C6F6164::terrane_construct(),
        }
    }
}
fn test_scenario_terrane_trait_field_defaults() {
    let value: User;
    let descriptor: TerraneDescriptor;
    value = User::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&value.slot), String::from(":"),
        terrane_scalar_support::scalar_text(&value.payload.value)
    );
    descriptor = TerraneDescriptor {
        identity: "/trait-field-defaults::user",
        name: "user",
        kind: "class",
        inherently_identity_bearing: false,
        fields: &[
            TerraneFieldMetadata {
                name: "slot",
                external_name: "slot",
                defaulted: true,
                optional: false,
                secret: false,
            },
            TerraneFieldMetadata {
                name: "payload",
                external_name: "payload",
                defaulted: true,
                optional: false,
                secret: false,
            },
        ],
    };
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(descriptor
        .fields.len() as i128)), String::from(":"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_0 =
        &terrane_collection_support::List::new(descriptor.fields.iter().map(| field |
        field.defaulted).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        48 /* terrane-site: tests/integration/trait-field-defaults/case.trn:17:39-17:68 */)),
        48 /* terrane-site: tests/integration/trait-field-defaults/case.trn:17:39-17:68 */) }),
        String::from(":"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &terrane_collection_support::List::new(descriptor
        .fields.iter().map(| field | field.defaulted).collect:: < Vec < bool > > ());
        __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        49 /* terrane-site: tests/integration/trait-field-defaults/case.trn:17:75-17:104 */)),
        49 /* terrane-site: tests/integration/trait-field-defaults/case.trn:17:75-17:104 */) })
    );
}
// Source: <compiler-generated test registry>
// Namespace: core/testing/runner
fn __terrane_test_hex(value: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
fn __terrane_test_record(outcome: &str, error: &TerraneError) {
    let Ok(path) = std::env::var("TERRANE_TEST_RESULT") else {
        return;
    };
    let details = error.structured_details();
    let frames = error.source_frames();
    let mut record = format!(
        "1\n{}\n{}\n{}\n{}\n{}\n", outcome, __terrane_test_hex(&error.descriptor_name()),
        __terrane_test_hex(error.message()), details.len(), frames.len(),
    );
    for detail in details {
        record.push_str(&__terrane_test_hex(detail));
        record.push('\n');
    }
    for frame in frames {
        record.push_str(&__terrane_test_hex(&frame));
        record.push('\n');
    }
    let _ = std::fs::write(path, record);
}
fn main() {
    let selected = std::env::args().nth(1).unwrap_or_default();
    match selected.as_str() {
        "0" => {
            test_scenario_terrane_abstract_category_membership();
        }
        "1" => {
            test_scenario_terrane_applied_class_type_control();
        }
        "2" => {
            test_scenario_terrane_authored_enum_catch_all();
        }
        "3" => {
            test_scenario_terrane_authored_enum_resource_transfer();
        }
        "4" => {
            test_scenario_terrane_authored_generic_class_phantom_parameter();
        }
        "5" => {
            test_scenario_terrane_authored_generic_contracts();
        }
        "6" => {
            test_scenario_terrane_authored_generic_enum_states();
        }
        "7" => {
            test_scenario_terrane_authored_generic_finite_recursion();
        }
        "8" => {
            test_scenario_terrane_authored_generic_interfaces();
        }
        "9" => {
            test_scenario_terrane_authored_generic_partial_construction();
        }
        "10" => {
            test_scenario_terrane_authored_generic_resource_choice();
        }
        "11" => {
            test_scenario_terrane_authored_generic_selection();
        }
        "12" => {
            test_scenario_terrane_authored_interface_transfer_composition();
        }
        "13" => {
            test_scenario_terrane_borrowed_inherited_copy_fields();
        }
        "14" => {
            test_scenario_terrane_bounded_borrowed_collections();
        }
        "15" => {
            test_scenario_terrane_bounded_consuming_generic_field();
        }
        "16" => {
            test_scenario_terrane_bounded_generic_bound_forwarding();
        }
        "17" => {
            test_scenario_terrane_bounded_interface_receiver_modes();
        }
        "18" => {
            test_scenario_terrane_class_field_metadata();
        }
        "19" => {
            test_scenario_terrane_class_instance_independence();
        }
        "20" => {
            test_scenario_terrane_class_method_closures();
        }
        "21" => {
            test_scenario_terrane_class_state_methods();
        }
        "22" => {
            if let Err(error) = test_scenario_terrane_codec_class_methods() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "23" => {
            test_scenario_terrane_collection_generic_keys_and_get();
        }
        "24" => {
            test_scenario_terrane_completion_callable_rewrite();
        }
        "25" => {
            test_scenario_terrane_completion_finally_generic_post_read();
        }
        "26" => {
            test_scenario_terrane_completion_finally_loop_break();
        }
        "27" => {
            test_scenario_terrane_completion_finally_loop_repeat();
        }
        "28" => {
            test_scenario_terrane_completion_finally_nested_post_read();
        }
        "29" => {
            test_scenario_terrane_completion_finally_post_read();
        }
        "30" => {
            test_scenario_terrane_completion_finally_string_post_read();
        }
        "31" => {
            test_scenario_terrane_constructor_if_initialization();
        }
        "32" => {
            test_scenario_terrane_constructor_none_defaults();
        }
        "33" => {
            test_scenario_terrane_deep_inheritance_field_forwarding();
        }
        "34" => {
            test_scenario_terrane_explicit_class_operations();
        }
        "35" => {
            test_scenario_terrane_explicit_generic_mutable_closure();
        }
        "36" => {
            test_scenario_terrane_explicit_generic_unused();
        }
        "37" => {
            test_scenario_terrane_field_default_assignability();
        }
        "38" => {
            test_scenario_terrane_field_grouped_consuming_once();
        }
        "39" => {
            test_scenario_terrane_finite_generic_class_type();
        }
        "40" => {
            test_scenario_terrane_generic_conditional_clone_and_inherited_interface();
        }
        "41" => {
            test_scenario_terrane_generic_field_cloneable_control();
        }
        "42" => {
            test_scenario_terrane_generic_index_borrow_controls();
        }
        "43" => {
            test_scenario_terrane_generic_interface_implementation_bound();
        }
        "44" => {
            test_scenario_terrane_generic_method_nested_result();
        }
        "45" => {
            test_scenario_terrane_generic_mutable_explicit_application();
        }
        "46" => {
            test_scenario_terrane_generic_required_field_consuming();
        }
        "47" => {
            test_scenario_terrane_grouped_consuming_field_and_nested_generic();
        }
        "48" => {
            test_scenario_terrane_implicit_class_field_defaults();
        }
        "49" => {
            test_scenario_terrane_indirect_recursive_class_type();
        }
        "50" => {
            test_scenario_terrane_inherited_base_fields();
        }
        "51" => {
            test_scenario_terrane_inherited_generic_bound_forwarding();
        }
        "52" => {
            test_scenario_terrane_inherited_interface_bounded_dispatch();
        }
        "53" => {
            test_scenario_terrane_inherited_interface_generic_ancestor();
        }
        "54" => {
            test_scenario_terrane_inherited_interface_imported_bound();
        }
        "55" => {
            test_scenario_terrane_inherited_interface_imported_value();
        }
        "56" => {
            test_scenario_terrane_inherited_interface_mutable_receiver();
        }
        "57" => {
            test_scenario_terrane_inherited_self_return();
        }
        "58" => {
            test_scenario_terrane_inherited_shared_field_writes();
        }
        "59" => {
            test_scenario_terrane_inherited_static_factory();
        }
        "60" => {
            test_scenario_terrane_inherited_static_state();
        }
        "61" => {
            test_scenario_terrane_interface_borrowed_wrapper();
        }
        "62" => {
            test_scenario_terrane_interface_invocation_mode_widening();
        }
        "63" => {
            test_scenario_terrane_interface_owned_inherited_mutation();
        }
        "64" => {
            test_scenario_terrane_interface_owned_upcast();
        }
        "65" => {
            test_scenario_terrane_interface_value_separation();
        }
        "66" => {
            test_scenario_terrane_method_without_receiver_use();
        }
        "67" => {
            test_scenario_terrane_mutating_interface_methods();
        }
        "68" => {
            test_scenario_terrane_nested_object_mutation();
        }
        "69" => {
            test_scenario_terrane_object_contracts_reuse();
        }
        "70" => {
            test_scenario_terrane_object_field_owned_snapshot();
        }
        "71" => {
            test_scenario_terrane_object_inheritance_lifecycle_matrix();
        }
        "72" => {
            test_scenario_terrane_optional_field_presence();
        }
        "73" => {
            test_scenario_terrane_optional_generic_field_default();
        }
        "74" => {
            test_scenario_terrane_recursive_generic_phantom();
        }
        "75" => {
            test_scenario_terrane_required_field_reference();
        }
        "76" => {
            if let Err(error) = test_scenario_terrane_shared_object_field_comparisons() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "77" => {
            test_scenario_terrane_source_associated_interface();
        }
        "78" => {
            test_scenario_terrane_source_finally_only_rewrite();
        }
        "79" => {
            test_scenario_terrane_source_generic_field_read();
        }
        "80" => {
            test_scenario_terrane_source_method_builtin_name_dispatch();
        }
        "81" => {
            test_scenario_terrane_source_mutable_receiver_replace();
        }
        "82" => {
            test_scenario_terrane_static_class_singleton();
        }
        "83" => {
            test_scenario_terrane_static_explicit_generic();
        }
        "84" => {
            test_scenario_terrane_static_field_member_assignment();
        }
        "85" => {
            test_scenario_terrane_trait_field_defaults();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
