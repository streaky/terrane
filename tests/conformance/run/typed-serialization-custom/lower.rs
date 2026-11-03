// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: typed-serialization-custom
#[derive(Clone)]
pub struct Code {
    pub hidden: String,
    pub text: String,
}
impl Code {
    pub fn terrane_construct() -> Self {
        Self {
            hidden: String::from("must-not-leak"),
            text: String::from("public-code"),
        }
    }
    pub fn to_document(&self) -> DocumentValue {
        return make_document_string(self.text.clone());
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return parse_json(value.scalar.clone(), default_json_options());
    }
}
impl TerraneDocumentEncode for Code {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        terrane_document_support::write_document_value(writer, &self.to_document().raw)
    }
}
impl SerializableProtocol for Code {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        Code::to_document(&*self)
    }
}
impl From<Code> for Serializable {
    fn from(value: Code) -> Self {
        Self(Box::new(value))
    }
}
impl DeserializableProtocol for Code {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn from_document(&self, value: DocumentValue) -> DocumentResult {
        Code::from_document(&*self, value)
    }
}
impl From<Code> for Deserializable {
    fn from(value: Code) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Envelope {
    pub code: Code,
}
impl Envelope {
    pub fn terrane_construct() -> Self {
        Self {
            code: Code::terrane_construct(),
        }
    }
}
impl TerraneDocumentEncode for Envelope {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("code")?;
        TerraneDocumentEncode::terrane_encode_document(&self.code, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Envelope {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Envelope> for DocumentEncodable {
    fn from(value: Envelope) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let custom: Code;
    let direct: TerraneDocumentDecodeOutcome<String>;
    let wrapped: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    let decoded: DocumentResult;
    let from_yaml: DocumentResult;
    custom = Code::terrane_construct();
    direct = {
        let __terrane_document_source = &custom;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:19:14",
            "code",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(direct.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&direct.value
        .clone())
    );
    wrapped = {
        let __terrane_document_source = &Envelope::terrane_construct();
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:21:15",
            "envelope",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(wrapped.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&wrapped.value
        .clone())
    );
    yaml = {
        let __terrane_document_source = &custom;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:23:12",
            "code",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
    decoded = decode_json(
        String::from("\"7\""),
        <Deserializable>::from(custom.clone()),
        default_json_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&decoded.failed),
        String::from("|"), terrane_scalar_support::scalar_text(&decoded.value.scalar)
    );
    from_yaml = decode_yaml(
        String::from("\"9\""),
        <Deserializable>::from(custom),
        default_yaml_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&from_yaml.failed),
        String::from("|"), terrane_scalar_support::scalar_text(&from_yaml.value.scalar)
    );
}
