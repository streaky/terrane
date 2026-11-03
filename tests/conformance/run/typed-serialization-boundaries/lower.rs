// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: typed-serialization-boundaries
#[derive(Clone)]
pub struct Child {
    pub value: String,
}
impl Child {
    pub fn terrane_construct() -> Self {
        Self { value: String::from("") }
    }
}
impl TerraneDocumentEncode for Child {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("value")?;
        TerraneDocumentEncode::terrane_encode_document(&self.value, writer)?;
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
#[derive(Clone)]
pub struct Record {
    pub children: terrane_collection_support::List<Child>,
    pub name: String,
}
impl Record {
    pub fn terrane_construct() -> Self {
        Self {
            children: terrane_collection_support::List::<
                Child,
            >::new(vec![Child::terrane_construct()]),
            name: String::from(""),
        }
    }
}
impl TerraneDocumentEncode for Record {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("children")?;
        TerraneDocumentEncode::terrane_encode_document(&self.children, writer)?;
        writer.key("marker-like")?;
        TerraneDocumentEncode::terrane_encode_document(&self.name, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Record {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Record> for DocumentEncodable {
    fn from(value: Record) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let mut nested: Child;
    let mut value: Record;
    let normal: TerraneDocumentDecodeOutcome<String>;
    let shallow: TerraneDocumentDecodeOutcome<String>;
    let tiny: TerraneDocumentDecodeOutcome<String>;
    let marker: DocumentResult;
    let oversized: DocumentResult;
    let rendered: DocumentResult;
    let scientific: TerraneDocumentDecodeOutcome<terrane_int_support::Int>;
    let expanded: TerraneDocumentDecodeOutcome<terrane_int_support::Int>;
    let rounded: TerraneDocumentDecodeOutcome<f32>;
    let underflow: TerraneDocumentDecodeOutcome<f32>;
    let nonfinite: f64;
    let rejected: TerraneDocumentDecodeOutcome<String>;
    nested = Child::terrane_construct();
    {
        let __terrane_value = String::from("x");
        nested.value = __terrane_value;
    }
    value = Record::terrane_construct();
    {
        let __terrane_value = terrane_collection_support::List::<
            Child,
        >::new(vec![nested]);
        value.children = __terrane_value;
    }
    {
        let __terrane_value = String::from("user-data");
        value.name = __terrane_value;
    }
    normal = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:20:14",
            "record",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(normal.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&normal.value
        .clone())
    );
    shallow = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(1024_i128),
        );
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:22:15",
            "record",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(shallow.diagnostics.length() != 0))
    );
    tiny = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(1_i128),
        );
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:24:12",
            "record",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(tiny.diagnostics.length() != 0))
    );
    marker = parse_json(
        String::from(
            "{\"$serde_json::private::Number\":\"ordinary\",\"value\":123456789012345678901234567890}",
        ),
        default_json_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&marker.failed), String::from("|"),
        terrane_scalar_support::scalar_text(&marker.value.encoded)
    );
    oversized = parse_json(String::from("1e100001"), default_json_options());
    println!("{}", terrane_scalar_support::scalar_text(&oversized.failed));
    rendered = stringify_json(
        marker.value.clone(),
        JsonOptions::terrane_construct(
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(8_i128),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&rendered.failed));
    scientific = {
        let __terrane_document_source = &String::from("1e+3");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_int_support::Int,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || terrane_int_support::Int::from(0_i128),
            __terrane_document_allow_unknown,
            "case.trn:32:18",
            "int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(scientific.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&scientific.value
        .clone())
    );
    expanded = {
        let __terrane_document_source = &String::from("1e100000");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_int_support::Int,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || terrane_int_support::Int::from(0_i128),
            __terrane_document_allow_unknown,
            "case.trn:34:16",
            "int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(expanded.diagnostics.length() != 0))
    );
    rounded = {
        let __terrane_document_source = &String::from(
            "1.000000059604644775390625000000000001",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f32,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <f32>::default(),
            __terrane_document_allow_unknown,
            "case.trn:36:15",
            "float32",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(rounded.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&(rounded.value
        .clone() > 1.0_f32))
    );
    underflow = {
        let __terrane_document_source = &String::from("1e-100000");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f32,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <f32>::default(),
            __terrane_document_allow_unknown,
            "case.trn:38:17",
            "float32",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(underflow.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&(underflow.value
        .clone() == 0.0_f32))
    );
    nonfinite = f64::INFINITY;
    rejected = {
        let __terrane_document_source = &nonfinite;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:42:16",
            "float64",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(rejected.diagnostics.length() != 0))
    );
}
