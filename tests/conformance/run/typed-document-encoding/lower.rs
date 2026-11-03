// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: typed-document-encoding
#[derive(Clone)]
pub struct Child {
    pub display: String,
    pub count: terrane_int_support::Int,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self {
            display: String::from(""),
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl TerraneDocumentEncode for Child {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("displayName")?;
        TerraneDocumentEncode::terrane_encode_document(&self.display, writer)?;
        writer.key("count")?;
        TerraneDocumentEncode::terrane_encode_document(&self.count, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Child {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Child> for DocumentEncodable {
    fn from(value: Child) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol for Child {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Child> for DocumentDecodable {
    fn from(value: Child) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Payload {
    pub title: String,
    pub child: Child,
    pub enabled: bool,
    pub scores: terrane_collection_support::List<terrane_int_support::Int>,
    pub tags: terrane_collection_support::Map<String, String>,
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub note: Option<String>,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self {
            title: String::from(""),
            child: Child::terrane_construct(),
            enabled: false,
            scores: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            tags: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::< String, String
                    >::new(String::from(""), String::from(""))
                ],
            ),
            pair: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            note: None,
        }
    }
}
impl TerraneDocumentEncode for Payload {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("title")?;
        TerraneDocumentEncode::terrane_encode_document(&self.title, writer)?;
        writer.key("child")?;
        TerraneDocumentEncode::terrane_encode_document(&self.child, writer)?;
        writer.key("enabled")?;
        TerraneDocumentEncode::terrane_encode_document(&self.enabled, writer)?;
        writer.key("scores")?;
        TerraneDocumentEncode::terrane_encode_document(&self.scores, writer)?;
        writer.key("tags")?;
        TerraneDocumentEncode::terrane_encode_document(&self.tags, writer)?;
        writer.key("pair")?;
        TerraneDocumentEncode::terrane_encode_document(&self.pair, writer)?;
        writer.key("note")?;
        TerraneDocumentEncode::terrane_encode_document(&self.note, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Payload {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Payload> for DocumentEncodable {
    fn from(value: Payload) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol for Payload {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Payload> for DocumentDecodable {
    fn from(value: Payload) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let mut value: Payload;
    let mut nested: Child;
    let json: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    value = Payload::terrane_construct();
    {
        let __terrane_value = String::from("sample");
        value.title = __terrane_value;
    }
    nested = Child::terrane_construct();
    {
        let __terrane_value = String::from("nested");
        nested.display = __terrane_value;
    }
    {
        let __terrane_value = terrane_int_support::Int::from(4_i128);
        nested.count = __terrane_value;
    }
    {
        let __terrane_value = nested;
        value.child = __terrane_value;
    }
    {
        let __terrane_value = true;
        value.enabled = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(7_i128),
                terrane_int_support::Int::from(9_i128)
            ],
        );
        value.scores = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::Map::<
            String,
            String,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, String
                >::new(String::from("tier"), String::from("edge"))
            ],
        );
        value.tags = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(2_i128),
                terrane_int_support::Int::from(3_i128)
            ],
        );
        value.pair = __terrane_value;
    }
    json = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:32:12",
            "payload",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(json.diagnostics.length() != 0)),
        String::from(" "), terrane_scalar_support::scalar_text(&json.value.clone())
    );
    yaml = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:34:12",
            "payload",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from(" "), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
}
