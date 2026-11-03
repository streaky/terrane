// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: typed-serialization-matrix
#[derive(Clone)]
pub struct Item {
    pub display: String,
    pub amount: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct() -> Self {
        Self {
            display: String::from(""),
            amount: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl TerraneDocumentDecode for Item {
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
                "item",
                source,
                "case.trn:8:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["displayName", "amount"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "item",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:8:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("displayName");
            let field_path = __terrane_document_child_path(path, "displayName");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:9:5",
                ) {
                    Ok(decoded) => value.display = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("amount");
            let field_path = __terrane_document_child_path(path, "amount");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:10:5",
                ) {
                    Ok(decoded) => value.amount = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode for Item {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("displayName")?;
        TerraneDocumentEncode::terrane_encode_document(&self.display, writer)?;
        writer.key("amount")?;
        TerraneDocumentEncode::terrane_encode_document(&self.amount, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Item {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Item> for DocumentEncodable {
    fn from(value: Item) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol for Item {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Item> for DocumentDecodable {
    fn from(value: Item) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Record {
    pub name: String,
    pub item: Item,
    pub numbers: terrane_collection_support::List<terrane_int_support::Int>,
    pub names: terrane_collection_support::Map<String, String>,
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub optional: Option<String>,
}
impl Record {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from("default"),
            item: Item::terrane_construct(),
            numbers: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            names: terrane_collection_support::Map::<
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
            optional: Some(String::from("kept")),
        }
    }
    pub fn validate_document(&self) -> Option<String> {
        if self.name.as_str() == "bad" {
            return Some(String::from("reserved"));
        }
        return None;
    }
}
impl TerraneDocumentDecode for Record {
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
                "record",
                source,
                "case.trn:12:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "name",
            "item",
            "numbers",
            "names",
            "pair",
            "optional",
        ];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "record",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:12:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("name");
            let field_path = __terrane_document_child_path(path, "name");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:13:5",
                ) {
                    Ok(decoded) => value.name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("item");
            let field_path = __terrane_document_child_path(path, "item");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Item as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:14:5",
                ) {
                    Ok(decoded) => value.item = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("numbers");
            let field_path = __terrane_document_child_path(path, "numbers");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:15:5",
                ) {
                    Ok(decoded) => value.numbers = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("names");
            let field_path = __terrane_document_child_path(path, "names");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Map<
                    String,
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:16:5",
                ) {
                    Ok(decoded) => value.names = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("pair");
            let field_path = __terrane_document_child_path(path, "pair");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:17:5",
                ) {
                    Ok(decoded) => value.pair = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("optional");
            let field_path = __terrane_document_child_path(path, "optional");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Option<
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:18:5",
                ) {
                    Ok(decoded) => value.optional = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() {
            if let Some(message) = value.validate_document() {
                diagnostics
                    .push(
                        __terrane_document_diagnostic(
                            path,
                            "record",
                            "map",
                            "validation",
                            message,
                            source,
                            "case.trn:12:1",
                        ),
                    );
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode for Record {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("name")?;
        TerraneDocumentEncode::terrane_encode_document(&self.name, writer)?;
        writer.key("item")?;
        TerraneDocumentEncode::terrane_encode_document(&self.item, writer)?;
        writer.key("numbers")?;
        TerraneDocumentEncode::terrane_encode_document(&self.numbers, writer)?;
        writer.key("names")?;
        TerraneDocumentEncode::terrane_encode_document(&self.names, writer)?;
        writer.key("pair")?;
        TerraneDocumentEncode::terrane_encode_document(&self.pair, writer)?;
        writer.key("optional")?;
        TerraneDocumentEncode::terrane_encode_document(&self.optional, writer)?;
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
impl DocumentDecodableProtocol for Record {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Record> for DocumentDecodable {
    fn from(value: Record) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentValidatableProtocol for Record {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn validate_document(&self) -> Option<String> {
        Record::validate_document(&*self)
    }
}
impl From<Record> for DocumentValidatable {
    fn from(value: Record) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let mut value: Record;
    let mut child: Item;
    let json: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    let decoded: TerraneDocumentDecodeOutcome<Record>;
    let from_yaml: TerraneDocumentDecodeOutcome<Record>;
    let defaults: TerraneDocumentDecodeOutcome<Record>;
    let invalid: TerraneDocumentDecodeOutcome<Record>;
    let mut diagnostic: TerraneDocumentDiagnostic;
    let validated: TerraneDocumentDecodeOutcome<Record>;
    let nulled: TerraneDocumentDecodeOutcome<Record>;
    value = Record::terrane_construct();
    {
        let __terrane_value = String::from("ok");
        value.name = __terrane_value;
    }
    child = Item::terrane_construct();
    {
        let __terrane_value = String::from("nested");
        child.display = __terrane_value;
    }
    {
        let __terrane_value = terrane_int_support::Int::from(7_i128);
        child.amount = __terrane_value;
    }
    {
        let __terrane_value = child;
        value.item = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(2_i128),
                terrane_int_support::Int::from(4_i128)
            ],
        );
        value.numbers = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::Map::<
            String,
            String,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, String
                >::new(String::from("zone"), String::from("west"))
            ],
        );
        value.names = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(8_i128),
                terrane_int_support::Int::from(9_i128)
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
            "case.trn:34:12",
            "record",
        )
    };
    yaml = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:35:12",
            "record",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(json.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&json.value.clone())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
    decoded = {
        let __terrane_receiver_value_0 = &json.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_0;
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:38:15",
            "record",
            None,
        )
    };
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(decoded
        .diagnostics.length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&decoded.value.clone().name),
        String::from("|"), terrane_scalar_support::scalar_text(&decoded.value.clone()
        .item.display), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &decoded.value.clone();
        __terrane_raised(__terrane_receiver_value_1.numbers
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:39:91-39:115 */)), 0 /* terrane-site: case.trn:39:91-39:115 */) }), String::from("|"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_2 = &decoded
        .value.clone(); __terrane_raised(__terrane_receiver_value_2.names
        .get_or_error(&String::from("zone")), 1 /* terrane-site: case.trn:39:122-39:149 */) }), String::from("|"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_3 = &decoded
        .value.clone(); __terrane_raised(__terrane_receiver_value_3.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:39:156-39:177 */)), 2 /* terrane-site: case.trn:39:156-39:177 */) }), String::from("|"), { let mut __terrane_text =
        String::new(); match &decoded.value.clone().optional { Some(__terrane_present) =>
        { terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    from_yaml = {
        let __terrane_receiver_value_4 = &yaml.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_4;
        let __terrane_document_options = &default_yaml_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_yaml_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
                terrane_limit(&__terrane_document_options.max_alias_nodes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:40:17",
            "record",
            None,
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(from_yaml.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&from_yaml.value.clone().name),
        String::from("|"), terrane_scalar_support::scalar_text(&from_yaml.value.clone()
        .item.amount)
    );
    defaults = {
        let __terrane_document_source = &String::from(
            "{\"item\":{\"displayName\":\"x\",\"amount\":1},\"unknown\":2}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = true;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:42:16",
            "record",
            None,
        )
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(defaults.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&defaults.value.clone().name),
        String::from("|"), { let mut __terrane_text = String::new(); match &defaults
        .value.clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text },
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_5 = &defaults.value.clone();
        __terrane_raised(__terrane_receiver_value_5.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: case.trn:43:90-43:112 */)), 3 /* terrane-site: case.trn:43:90-43:112 */) })
    );
    invalid = {
        let __terrane_document_source = &String::from(
            "{\"name\":\"bad\",\"item\":{\"displayName\":false,\"amount\":\"x\"},\"numbers\":[1,\"x\"],\"pair\":[1]}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:44:15",
            "record",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(invalid
        .diagnostics.clone().length()))
    );
    let __terrane_iterable_0 = invalid.diagnostics.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from("|"), terrane_scalar_support::scalar_text(&diagnostic.reason
            .to_owned())
        );
    }
    validated = {
        let __terrane_document_source = &String::from(
            "{\"name\":\"bad\",\"optional\":null}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:48:17",
            "record",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(validated.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(validated
        .diagnostics.clone().length()))
    );
    nulled = {
        let __terrane_document_source = &String::from("{\"optional\":null}");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:50:14",
            "record",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nulled.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &nulled
        .value.clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
}
