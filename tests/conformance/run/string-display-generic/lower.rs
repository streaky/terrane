// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: string-display-generic
pub trait DisplayableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DisplayableProtocol>;
    fn separate_box(&self) -> Box<dyn DisplayableProtocol>;
    fn to_string(&self) -> String;
}
impl Clone for Box<dyn DisplayableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Displayable(Box<dyn DisplayableProtocol>);
impl Clone for Displayable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Displayable {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Clone)]
pub struct Label {
    pub text: String,
}
impl Label {
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
    pub fn to_string(&self) -> String {
        return self.text.clone();
    }
}
impl DisplayableProtocol for Label {
    fn clone_box(&self) -> Box<dyn DisplayableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DisplayableProtocol> {
        Box::new(self.clone())
    }
    fn to_string(&self) -> String {
        Label::to_string(&*self)
    }
}
impl From<Label> for Displayable {
    fn from(value: Label) -> Self {
        Self(Box::new(value))
    }
}
fn decorate<TerraneType54: DisplayableProtocol>(value: TerraneType54) -> String {
    println!("{}", value.to_string());
    return {
        let mut __terrane_text = String::with_capacity(8);
        __terrane_text.push_str("generic:");
        __terrane_text.push_str(&value.to_string());
        __terrane_text
    };
}
fn compose<TerraneType54: DisplayableProtocol>(value: TerraneType54) -> String {
    return {
        let mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &{
                let mut __terrane_text = String::with_capacity(7);
                __terrane_text.push_str("prefix ");
                __terrane_text.push_str(&value.to_string());
                __terrane_text
            },
            &mut __terrane_text,
        );
        __terrane_text.push_str("|");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &{
                let __terrane_separator = &":";
                let mut __terrane_text = String::with_capacity(1);
                __terrane_text.push_str(&value.to_string());
                __terrane_text.push_str(__terrane_separator);
                __terrane_text.push_str(&value.to_string());
                __terrane_text
            },
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn main() {
    let value: Label;
    let erased: Displayable;
    value = Label::terrane_construct(String::from("label"));
    println!("{}", terrane_scalar_support::scalar_text(&decorate(value.clone())));
    println!("{}", terrane_scalar_support::scalar_text(&compose(value.clone())));
    erased = <Displayable>::from(value);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("interface:"); __terrane_text
        .push_str(&erased.to_string()); __terrane_text })
    );
}
