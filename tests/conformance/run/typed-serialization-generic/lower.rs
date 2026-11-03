// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct Holder<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub count: terrane_int_support::Int,
}
impl<TerraneType54> Holder<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl<TerraneType54> TerraneDocumentDecode for Holder<TerraneType54> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "holder",
                source,
                "case.trn:4:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["count"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "holder",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:4:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("count");
            let field_path = __terrane_document_child_path(path, "count");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:5:5",
                ) {
                    Ok(decoded) => value.count = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl<TerraneType54> TerraneDocumentEncode for Holder<TerraneType54> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("count")?;
        TerraneDocumentEncode::terrane_encode_document(&self.count, writer)?;
        writer.end_object()
    }
}
impl<TerraneType54> DocumentEncodableProtocol for Holder<TerraneType54>
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl<TerraneType54> From<Holder<TerraneType54>> for DocumentEncodable
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn from(value: Holder<TerraneType54>) -> Self {
        Self(Box::new(value))
    }
}
impl<TerraneType54> DocumentDecodableProtocol for Holder<TerraneType54>
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl<TerraneType54> From<Holder<TerraneType54>> for DocumentDecodable
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn from(value: Holder<TerraneType54>) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let encoded: TerraneDocumentDecodeOutcome<String>;
    let decoded: TerraneDocumentDecodeOutcome<Holder<String>>;
    encoded = {
        let __terrane_document_source = &Holder::<String>::terrane_construct();
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:7:15",
            "holder of string",
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&encoded.value.clone()));
    decoded = {
        let __terrane_document_source = &String::from("{\"count\":9}");
        let __terrane_document_fallback = &Holder::<String>::terrane_construct();
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Holder<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:9:15",
            "holder of string",
            None,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&decoded.value.clone().count));
}
