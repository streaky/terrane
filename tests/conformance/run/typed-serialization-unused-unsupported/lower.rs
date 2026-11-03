// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct Unused {
    pub callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>,
}
impl Unused {
    pub fn terrane_construct() -> Self {
        Self {
            callback: {
                std::sync::Arc::new(move || -> terrane_int_support::Int {
                    return terrane_int_support::Int::from(1_i128);
                })
            },
        }
    }
}
impl DocumentEncodableProtocol for Unused {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Unused> for DocumentEncodable {
    fn from(value: Unused) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol for Unused {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Unused> for DocumentDecodable {
    fn from(value: Unused) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let encoded: TerraneDocumentDecodeOutcome<String>;
    let decoded: TerraneDocumentDecodeOutcome<terrane_int_support::Int>;
    encoded = {
        let __terrane_document_source = &1;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:8:15",
            "int",
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&encoded.value.clone()));
    decoded = {
        let __terrane_document_source = &String::from("3");
        let __terrane_document_fallback = terrane_int_support::Int::from(0_i128);
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
            || __terrane_document_fallback.clone(),
            __terrane_document_allow_unknown,
            "case.trn:10:15",
            "int",
            None,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&decoded.value.clone()));
}
