// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: declaration-annotations
static __TERRANE_F0_SHARED_TEXT: std::sync::LazyLock<String> = std::sync::LazyLock::new(||
String::from("shared"));
#[derive(Clone)]
pub struct Tag {
    pub value: String,
}
impl Tag {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { value: String::from("") };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        println!("{}", String::from("ANNOTATION CONSTRUCTOR MUST NOT RUN"));
    }
}
#[derive(Clone)]
pub struct DataTag {
    pub values: terrane_collection_support::List<String>,
    pub keys: terrane_collection_support::Set<terrane_int_support::Int>,
    pub pairs: terrane_collection_support::Map<String, terrane_int_support::Int>,
    pub raw: Vec<u8>,
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub scale: f32,
    pub enabled: bool,
    pub target: Option<Descriptor>,
}
impl DataTag {
    pub fn terrane_construct(target: Descriptor) -> Self {
        let mut __terrane_constructed_value = Self {
            values: terrane_collection_support::List::<String>::new(Vec::new()),
            keys: terrane_collection_support::Set::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            pairs: terrane_collection_support::Map::<
                String,
                terrane_int_support::Int,
            >::new(Vec::new()),
            raw: Vec::<u8>::from([]),
            pair: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(3_i128),
                    terrane_int_support::Int::from(4_i128)
                ],
            ),
            scale: 0.0_f32,
            enabled: false,
            target: None,
        };
        __terrane_constructed_value.construct(target);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, target: Descriptor) {
        self.target = Some(target);
    }
}
#[derive(Clone)]
pub struct ParentStorage {
    pub field: String,
    pub secret: String,
    pub credential: String,
}
impl ParentStorage {
    pub fn terrane_construct() -> Self {
        Self {
            field: String::from("field"),
            secret: String::from("private initializer"),
            credential: String::from("secret initializer"),
        }
    }
    pub fn inherited(
        &self,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return value.clone() + terrane_int_support::Int::from(1_i128);
    }
    pub fn hidden(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        return value.clone();
    }
}
#[derive(Clone)]
pub enum Parent {
    Own(ParentStorage),
    Child(Child),
    OverrideChild(OverrideChild),
}
impl Parent {
    pub fn terrane_construct() -> Self {
        Self::Own(ParentStorage::terrane_construct())
    }
    pub fn inherited(
        &self,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value_) => value_.inherited(value),
            Self::Child(value_) => value_.inherited(value),
            Self::OverrideChild(value_) => value_.inherited(value),
        }
    }
    pub fn hidden(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        match self {
            Self::Own(value_) => value_.hidden(value),
            Self::Child(value_) => value_.hidden(value),
            Self::OverrideChild(value_) => value_.hidden(value),
        }
    }
    pub fn terrane_field_field(&self) -> &String {
        match self {
            Self::Own(value) => &value.field,
            Self::Child(value) => &value.field,
            Self::OverrideChild(value) => &value.field,
        }
    }
    pub fn terrane_field_field_mut(&mut self) -> &mut String {
        match self {
            Self::Own(value) => &mut value.field,
            Self::Child(value) => &mut value.field,
            Self::OverrideChild(value) => &mut value.field,
        }
    }
    pub fn terrane_field_secret(&self) -> &String {
        match self {
            Self::Own(value) => &value.secret,
            Self::Child(value) => &value.secret,
            Self::OverrideChild(value) => &value.secret,
        }
    }
    pub fn terrane_field_secret_mut(&mut self) -> &mut String {
        match self {
            Self::Own(value) => &mut value.secret,
            Self::Child(value) => &mut value.secret,
            Self::OverrideChild(value) => &mut value.secret,
        }
    }
    pub fn terrane_field_credential(&self) -> &String {
        match self {
            Self::Own(value) => &value.credential,
            Self::Child(value) => &value.credential,
            Self::OverrideChild(value) => &value.credential,
        }
    }
    pub fn terrane_field_credential_mut(&mut self) -> &mut String {
        match self {
            Self::Own(value) => &mut value.credential,
            Self::Child(value) => &mut value.credential,
            Self::OverrideChild(value) => &mut value.credential,
        }
    }
}
#[derive(Clone)]
pub struct Child {
    pub field: String,
    pub secret: String,
    pub credential: String,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            field: String::from("field"),
            secret: String::from("private initializer"),
            credential: String::from("secret initializer"),
        }
    }
    pub fn inherited(
        &self,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return value.clone() + terrane_int_support::Int::from(1_i128);
    }
    pub fn hidden(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        return value.clone();
    }
}
#[derive(Clone)]
pub struct OverrideChild {
    pub field: String,
    pub secret: String,
    pub credential: String,
}
impl OverrideChild {
    pub fn terrane_construct() -> Self {
        Self {
            field: String::from("field"),
            secret: String::from("private initializer"),
            credential: String::from("secret initializer"),
        }
    }
    pub fn inherited(
        &self,
        value: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        return value.clone() + terrane_int_support::Int::from(2_i128);
    }
    pub fn hidden(&self, value: terrane_int_support::Int) -> terrane_int_support::Int {
        return value.clone();
    }
}
#[derive(Clone)]
pub struct CallableTarget {}
impl CallableTarget {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
#[derive(Clone)]
pub struct NoneTarget {}
impl NoneTarget {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn helper(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
fn main() {
    let item: Child;
    let override_item: OverrideChild;
    item = Child::terrane_construct();
    override_item = OverrideChild::terrane_construct();
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&helper(terrane_int_support::Int::from(4_i128))),
        String::from(":"), terrane_scalar_support::scalar_text(&item
        .inherited(terrane_int_support::Int::from(3_i128))), String::from(":"),
        terrane_scalar_support::scalar_text(&override_item
        .inherited(terrane_int_support::Int::from(3_i128)))
    );
}
