// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: authored-generic-interfaces
pub trait DescribableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DescribableProtocol>;
    fn separate_box(&self) -> Box<dyn DescribableProtocol>;
    fn describe(&self) -> String;
}
impl Clone for Box<dyn DescribableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Describable(Box<dyn DescribableProtocol>);
impl Clone for Describable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Describable {
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
        let mut value = Self { text: String::from("") };
        value.construct(text);
        value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
    pub fn describe(&self) -> String {
        return self.text.clone();
    }
}
impl DescribableProtocol for Message {
    fn clone_box(&self) -> Box<dyn DescribableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DescribableProtocol> {
        Box::new(self.clone())
    }
    fn describe(&self) -> String {
        Message::describe(&*self)
    }
}
impl From<Message> for Describable {
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
impl DescribableProtocol for Address {
    fn clone_box(&self) -> Box<dyn DescribableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DescribableProtocol> {
        Box::new(self.clone())
    }
    fn describe(&self) -> String {
        Address::describe(&*self)
    }
}
impl From<Address> for Describable {
    fn from(value: Address) -> Self {
        Self(Box::new(value))
    }
}
fn choose<TerraneType54>(
    first: TerraneType54,
    second: TerraneType54,
    use_first: bool,
) -> TerraneType54 {
    if use_first {
        return first;
    }
    return second;
}
fn preserve<TerraneType54: DescribableProtocol>(value: TerraneType54) -> TerraneType54 {
    return value;
}
fn describe_value<TerraneType54: DescribableProtocol>(value: TerraneType54) -> String {
    return value.describe();
}
pub trait SourceProtocol<TerraneType54>: Send + Sync {
    fn clone_box(&self) -> Box<dyn SourceProtocol<TerraneType54>>;
    fn separate_box(&self) -> Box<dyn SourceProtocol<TerraneType54>>;
    fn next(&mut self) -> Option<TerraneType54>;
}
impl<TerraneType54> Clone for Box<dyn SourceProtocol<TerraneType54>> {
    fn clone(&self) -> Self {
        self.clone_box()
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
        let result: Option<String> = self.value.clone();
        self.value = None;
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
fn main() {
    let message: Message = Message::terrane_construct(String::from("preserved"));
    let kept: Message = preserve(message.clone());
    println!("{}", terrane_scalar_support::scalar_text(&kept.describe()));
    println!("{}", terrane_scalar_support::scalar_text(&describe_value(kept)));
    let mut input: Source<String> = <Source<
        String,
    >>::from(MessageSource::terrane_construct());
    let first: Option<String> = next_string(input.clone());
    if first.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&first.as_ref()
            .expect("semantic optional narrowing").clone())
        );
    }
    input.next();
    let second: Option<String> = input.next();
    if second.is_none() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("none")));
    }
    let peer: Address = Address::terrane_construct();
    let erased: Describable = choose::<
        Describable,
    >(<Describable>::from(message), <Describable>::from(peer), false);
    println!("{}", terrane_scalar_support::scalar_text(&erased.describe()));
}
