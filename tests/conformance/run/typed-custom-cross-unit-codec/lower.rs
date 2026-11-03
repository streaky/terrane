// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: app/main.trn
// Namespace: app
fn main() {
    let item: Child;
    item = Child::terrane_construct();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&item.extra), String::from("|"),
        terrane_scalar_support::scalar_text(&encoded())
    );
}
// Source: derived/child.trn
// Namespace: derived
#[derive(Clone)]
pub struct Child {
    pub value: terrane_int_support::Int,
    pub extra: terrane_int_support::Int,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
            extra: terrane_int_support::Int::from(2_i128),
        }
    }
    pub fn to_document(&self) -> DocumentValue {
        return make_document_string(String::from("custom"));
    }
}
impl SerializableProtocol for Child {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        Child::to_document(&*self)
    }
}
impl From<Child> for Serializable {
    fn from(value: Child) -> Self {
        Self(Box::new(value))
    }
}
// Source: models/base.trn
// Namespace: models
#[derive(Clone)]
pub struct Base {
    pub value: terrane_int_support::Int,
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(1_i128),
        }
    }
    pub fn to_document(&self) -> DocumentValue {
        return make_document_string(String::from("custom"));
    }
}
impl TerraneDocumentEncode for Base {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        terrane_document_support::write_document_value(writer, &self.to_document().raw)
    }
}
impl SerializableProtocol for Base {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        Base::to_document(&*self)
    }
}
impl From<Base> for Serializable {
    fn from(value: Base) -> Self {
        Self(Box::new(value))
    }
}
fn encoded() -> String {
    let result: TerraneDocumentDecodeOutcome<String>;
    result = {
        let __terrane_document_source = &Base::terrane_construct();
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "models/base.trn:9:14",
            "base",
        )
    };
    return result.value.clone();
}
