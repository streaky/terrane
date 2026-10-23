// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: string-display-boundaries
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S628 {
    Arm0(Label),
    Arm1(terrane_int_support::Int),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S628 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[derive(Clone)]
pub struct LabelStorage {}
impl LabelStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_746f2d737472696e67(&self) -> String {
        return String::from("unsafe");
    }
    pub fn to_string(&self) -> String {
        return String::from("label");
    }
}
#[derive(Clone)]
pub enum Label {
    Own(LabelStorage),
    Child(Child),
}
impl Label {
    pub fn terrane_construct() -> Self {
        Self::Own(LabelStorage::terrane_construct())
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_746f2d737472696e67(&self) -> String {
        match self {
            Self::Own(value) => unsafe { value._terrane_unsafe_746f2d737472696e67() }
            Self::Child(value) => unsafe { value._terrane_unsafe_746f2d737472696e67() }
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Own(value) => value.to_string(),
            Self::Child(value) => value.to_string(),
        }
    }
}
#[derive(Clone)]
pub struct Child {
    pub extra: terrane_int_support::Int,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            extra: terrane_int_support::Int::from(3_i128),
        }
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_746f2d737472696e67(&self) -> String {
        return String::from("unsafe");
    }
    pub fn to_string(&self) -> String {
        return String::from("label");
    }
}
fn describe_borrow(value: &Label) -> String {
    return {
        let mut __terrane_text = String::with_capacity(9);
        __terrane_text.push_str("borrowed:");
        __terrane_text.push_str(&(*value).to_string());
        __terrane_text
    };
}
fn describe_text(value: &String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(5);
        __terrane_text.push_str("ref:");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &**value,
            &mut __terrane_text,
        );
        __terrane_text.push_str("|");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &terrane_scalar_support::scalar_text(&**value),
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn main() {
    let item: Child;
    let mut optional: Option<Label>;
    let mut mixed: TerraneUnionF0S628;
    let mut value: terrane_int_support::Int;
    let render: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
    let borrowed: Label;
    let text: String;
    let shared_text: std::sync::Arc<TerraneSharedCell<String>>;
    let shared: std::sync::Arc<TerraneSharedCell<String>>;
    item = Child::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("inherited:"); __terrane_text
        .push_str(&item.to_string()); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6); __terrane_text.push_str("again:"); __terrane_text
        .push_str(&item.to_string()); __terrane_text })
    );
    optional = None;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("optional:"); match &optional {
        Some(__terrane_present) => { __terrane_text.push_str(&__terrane_present
        .to_string()); }, None => __terrane_text.push_str("none"), } __terrane_text })
    );
    optional = Some(Label::terrane_construct());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("optional:"); match &optional {
        Some(__terrane_present) => { __terrane_text.push_str(&__terrane_present
        .to_string()); }, None => __terrane_text.push_str("none"), } __terrane_text })
    );
    mixed = TerraneUnionF0S628::Arm1(terrane_int_support::Int::from(7_i128));
    if item.extra.clone() == terrane_int_support::Int::from(3_i128) {
        mixed = TerraneUnionF0S628::Arm0(Label::terrane_construct());
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6); __terrane_text.push_str("union:"); match &mixed {
        TerraneUnionF0S628::Arm0(__terrane_arm) => { __terrane_text
        .push_str(&__terrane_arm.to_string()); }, TerraneUnionF0S628::Arm1(__terrane_arm)
        => { terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text })
    );
    value = terrane_int_support::Int::from(42_i128);
    render = {
        let receiver = value.clone();
        std::sync::Arc::new(move || terrane_scalar_support::scalar_text(&receiver))
    };
    value = terrane_int_support::Int::from(99_i128);
    println!(
        "{}{}{}{}", String::from("bound:"),
        terrane_scalar_support::scalar_text(&render()), String::from("|current:"),
        terrane_scalar_support::scalar_text(&value)
    );
    borrowed = Label::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&describe_borrow(&borrowed)));
    text = String::from("text");
    println!("{}", terrane_scalar_support::scalar_text(&describe_text(&text)));
    shared_text = std::sync::Arc::new(TerraneSharedCell::new(String::from("text")));
    shared = shared_text.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("shared:");
        terrane_scalar_support::ScalarDisplay::write_scalar(&* shared.lock()
        .expect("shared reference lock poisoned"), &mut __terrane_text); __terrane_text
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&*
        shared.lock().expect("shared reference lock poisoned")))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&* shared.lock()
        .expect("shared reference lock poisoned"))
    );
}
