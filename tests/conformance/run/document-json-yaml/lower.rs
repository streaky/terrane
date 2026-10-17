// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/document-json-yaml
#[derive(Clone)]
pub struct Note {
    pub text: String,
}
impl Note {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
    pub fn to_document(&self) -> DocumentValue {
        return make_document_string(self.text.clone());
    }
}
impl SerializableProtocol for Note {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        Note::to_document(&*self)
    }
}
impl From<Note> for Serializable {
    fn from(value: Note) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let options: JsonOptions;
    let parsed: DocumentResult;
    let integer: DocumentResult;
    let decimal: DocumentResult;
    let canonical: DocumentResult;
    let reparsed: DocumentResult;
    let equivalent: DocumentResult;
    let stringify_input: DocumentResult;
    let stringified: DocumentResult;
    let yaml_input: DocumentResult;
    let yaml_written: DocumentResult;
    let list_value: DocumentResult;
    let list_document: DocumentValue;
    let first: DocumentResult;
    let second: DocumentResult;
    let nested: DocumentResult;
    let negative_item: DocumentResult;
    let duplicate: DocumentResult;
    let fields: terrane_collection_support::List<String>;
    let optional_fields: terrane_collection_support::List<String>;
    let default_fields: terrane_collection_support::List<String>;
    let default_values: terrane_collection_support::List<String>;
    let mut mapping: DocumentMapping;
    let mut missing: DocumentResult;
    let mut unknown: DocumentResult;
    let mut mapped: DocumentResult;
    let active: DocumentResult;
    let constructor_mapping: DocumentMapping;
    let constructor_input: DocumentResult;
    let constructor_result: DocumentResult;
    let encoded_note: DocumentResult;
    let encoded_yaml_note: DocumentResult;
    let exact_integer: DocumentResult;
    let exact_decimal: DocumentResult;
    let values: terrane_collection_support::List<DocumentValue>;
    let built_list: DocumentResult;
    let mut entries: DocumentMapEntries;
    let count_value: DocumentResult;
    let built_map: DocumentResult;
    let mut duplicate_entries: DocumentMapEntries;
    let duplicate_map: DocumentResult;
    let canonical_integer: DocumentResult;
    let decoded_through_interface: DocumentResult;
    let negative_limits: JsonOptions;
    let limited: DocumentResult;
    let excessive_depth: DocumentResult;
    let trailing_json: DocumentResult;
    let yaml_limits: YamlOptions;
    let yaml_value: DocumentResult;
    let yaml_integer: DocumentResult;
    let yaml_decimal: DocumentResult;
    let yaml_decoded: DocumentResult;
    let ordinary_star: DocumentResult;
    let bomb: DocumentResult;
    let yaml_depth: DocumentResult;
    let excessive_yaml_depth: DocumentResult;
    options = default_json_options();
    parsed = parse_json(
        String::from("{\"a\":123456789012345678901234567890,\"z\":1.2300}"),
        options.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&parsed.failed),
        terrane_scalar_support::scalar_text(&parsed.value.kind)
    );
    integer = parsed.value.field(String::from("a"));
    decimal = parsed.value.field(String::from("z"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&integer.value.kind),
        terrane_scalar_support::scalar_text(&integer.value.scalar)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&decimal.value.kind),
        terrane_scalar_support::scalar_text(&decimal.value.scalar)
    );
    println!("{}", terrane_scalar_support::scalar_text(&integer.value.integer.text));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&decimal.value.decimal.coefficient),
        terrane_scalar_support::scalar_text(&decimal.value.decimal.exponent)
    );
    canonical = canonical_json(parsed.value.clone());
    println!("{}", terrane_scalar_support::scalar_text(&canonical.value.encoded));
    reparsed = parse_json(canonical.value.encoded.clone(), options.clone());
    equivalent = parse_json(
        String::from("{\"a\":1.2345678901234567890123456789e+29,\"z\":1.23}"),
        options.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&reparsed.failed),
        terrane_scalar_support::scalar_text(&(canonical.value.encoded.as_str() ==
        equivalent.value.encoded.as_str()))
    );
    stringify_input = parse_json(String::from("{\"a\":1,\"z\":1.23}"), options.clone());
    stringified = stringify_json(stringify_input.value.clone(), options.clone());
    yaml_input = parse_json(String::from("{\"a\":1,\"z\":1.23}"), options.clone());
    yaml_written = stringify_yaml(yaml_input.value.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&stringified.failed),
        terrane_scalar_support::scalar_text(&yaml_written.failed)
    );
    list_value = parse_json(
        String::from("[\"first\",{\"nested\":true}]"),
        options.clone(),
    );
    list_document = list_value.value.clone();
    first = list_document.item(terrane_int_support::Int::from(0_i128));
    second = list_document.item(terrane_int_support::Int::from(1_i128));
    nested = second.value.field(String::from("nested"));
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&list_document.length()),
        terrane_scalar_support::scalar_text(&list_document
        .key(terrane_int_support::Int::from(0_i128))),
        terrane_scalar_support::scalar_text(&first.value.scalar),
        terrane_scalar_support::scalar_text(&nested.value.scalar)
    );
    negative_item = list_document.item(terrane_int_support::Int::from(-1_i128));
    println!("{}", terrane_scalar_support::scalar_text(&negative_item.failed));
    duplicate = parse_json(String::from("{\"key\":1,\"key\":2}"), options.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&duplicate.failed),
        terrane_scalar_support::scalar_text(&duplicate.message
        .contains(&String::from("duplicate key")))
    );
    fields = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("name"), String::from("nickname"), String::from("active")]);
    optional_fields = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("nickname")]);
    default_fields = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("active")]);
    default_values = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("true")]);
    mapping = DocumentMapping::terrane_construct(
        String::from("person"),
        String::from("map"),
        false,
    );
    mapping.field_names = fields;
    mapping.optional_fields = optional_fields;
    mapping.default_fields = default_fields;
    mapping.default_values = default_values;
    missing = parse_json(String::from("{}"), options.clone());
    missing = decode_document(missing.value.clone(), mapping.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&missing.failed),
        terrane_scalar_support::scalar_text(&missing.path),
        terrane_scalar_support::scalar_text(&missing.expected)
    );
    unknown = parse_json(
        String::from("{\"name\":\"Ada\",\"extra\":1}"),
        options.clone(),
    );
    unknown = decode_document(unknown.value.clone(), mapping.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&unknown.failed),
        terrane_scalar_support::scalar_text(&unknown.path),
        terrane_scalar_support::scalar_text(&unknown.expected)
    );
    mapped = parse_json(
        String::from("{\"name\":\"Ada\",\"nickname\":\"A\"}"),
        options.clone(),
    );
    mapped = decode_document(mapped.value.clone(), mapping.clone());
    active = mapped.value.field(String::from("active"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&mapped.failed),
        terrane_scalar_support::scalar_text(&active.value.scalar)
    );
    constructor_mapping = DocumentMapping::terrane_construct(
        String::from("open-map"),
        String::from("map"),
        true,
    );
    constructor_input = parse_json(String::from("{\"a\":1}"), options.clone());
    constructor_result = decode_document(
        constructor_input.value.clone(),
        constructor_mapping,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&constructor_result.failed),
        terrane_scalar_support::scalar_text(&constructor_result.value.encoded)
    );
    encoded_note = encode_json(
        <Serializable>::from(Note::terrane_construct(String::from("hello"))),
        options.clone(),
    );
    encoded_yaml_note = encode_yaml(
        <Serializable>::from(Note::terrane_construct(String::from("hello"))),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&encoded_note.value.encoded),
        terrane_scalar_support::scalar_text(&encoded_yaml_note.value.encoded)
    );
    exact_integer = make_document_integer(
        String::from("123456789012345678901234567890"),
    );
    exact_decimal = make_document_decimal(String::from("1.2300"));
    values = terrane_collection_support::List::<
        DocumentValue,
    >::new(
        vec![
            exact_integer.value.clone(), exact_decimal.value.clone(),
            make_document_none()
        ],
    );
    built_list = make_document_list(values);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&built_list.failed),
        terrane_scalar_support::scalar_text(&built_list.value.encoded)
    );
    entries = DocumentMapEntries::terrane_construct();
    entries = append_document_map_entry(
        entries,
        String::from("message"),
        make_document_string(String::from("hello")),
    );
    count_value = make_document_integer(String::from("123456789012345678901234567890"));
    entries = append_document_map_entry(
        entries,
        String::from("count"),
        count_value.value.clone(),
    );
    built_map = make_document_map(entries);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&built_map.failed),
        terrane_scalar_support::scalar_text(&built_map.value.encoded)
    );
    duplicate_entries = DocumentMapEntries::terrane_construct();
    duplicate_entries = append_document_map_entry(
        duplicate_entries,
        String::from("same"),
        make_document_string(String::from("first")),
    );
    duplicate_entries = append_document_map_entry(
        duplicate_entries,
        String::from("same"),
        make_document_string(String::from("second")),
    );
    duplicate_map = make_document_map(duplicate_entries);
    canonical_integer = make_document_integer(
        String::from("1.2345678901234567890123456789e+29"),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&duplicate_map.failed),
        terrane_scalar_support::scalar_text(&duplicate_map.path),
        terrane_scalar_support::scalar_text(&canonical_integer.failed)
    );
    decoded_through_interface = decode_json(
        String::from("{\"name\":\"Ada\"}"),
        <Deserializable>::from(mapping.clone()),
        options.clone(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&decoded_through_interface.failed)
    );
    negative_limits = JsonOptions::terrane_construct(
        terrane_int_support::Int::from(-1_i128),
        terrane_int_support::Int::from(-1_i128),
    );
    limited = parse_json(String::from("{}"), negative_limits);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&limited.failed),
        terrane_scalar_support::scalar_text(&limited.message
        .contains(&String::from("byte limit")))
    );
    excessive_depth = parse_json(
        String::from("[]"),
        JsonOptions::terrane_construct(
            terrane_int_support::Int::from(1000000_i128),
            terrane_int_support::Int::from(1024_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&excessive_depth.failed),
        terrane_scalar_support::scalar_text(&excessive_depth.message
        .contains(&String::from("cannot exceed")))
    );
    trailing_json = parse_json(String::from("{\"a\":1} garbage"), options);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&trailing_json.failed),
        terrane_scalar_support::scalar_text(&trailing_json.message
        .contains(&String::from("trailing characters")))
    );
    yaml_limits = make_yaml_options(
        terrane_int_support::Int::from(32_i128),
        terrane_int_support::Int::from(2048_i128),
        terrane_int_support::Int::from(20_i128),
    );
    yaml_value = parse_yaml(
        String::from(
            "integer: 123456789012345678901234567890\ndecimal: 3.141592653589793238462643383279",
        ),
        yaml_limits.clone(),
    );
    yaml_integer = yaml_value.value.field(String::from("integer"));
    yaml_decimal = yaml_value.value.field(String::from("decimal"));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_value.failed),
        terrane_scalar_support::scalar_text(&yaml_integer.value.integer.text)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_decimal.value.decimal
        .coefficient), terrane_scalar_support::scalar_text(&yaml_decimal.value.decimal
        .exponent)
    );
    yaml_decoded = decode_yaml(
        String::from("name: Ada"),
        <Deserializable>::from(mapping),
        make_yaml_options(
            terrane_int_support::Int::from(32_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&yaml_decoded.failed));
    ordinary_star = parse_yaml(
        String::from("glob: \"a * b * c\""),
        make_yaml_options(
            terrane_int_support::Int::from(32_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(0_i128),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&ordinary_star.failed));
    bomb = parse_yaml(
        String::from(
            "leaf: &leaf [1, 2, 3, 4]\na: &a [*leaf, *leaf, *leaf, *leaf]\nb: [*a, *a, *a, *a]",
        ),
        yaml_limits,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&bomb.failed),
        terrane_scalar_support::scalar_text(&bomb.message
        .contains(&String::from("alias node limit")))
    );
    yaml_depth = parse_yaml(
        String::from("a: [[[[]]]]"),
        make_yaml_options(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&yaml_depth.failed),
        terrane_scalar_support::scalar_text(&yaml_depth.message
        .contains(&String::from("depth limit")))
    );
    excessive_yaml_depth = parse_yaml(
        String::from("[]"),
        make_yaml_options(
            terrane_int_support::Int::from(256_i128),
            terrane_int_support::Int::from(1024_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&excessive_yaml_depth.failed),
        terrane_scalar_support::scalar_text(&excessive_yaml_depth.message
        .contains(&String::from("cannot exceed 255")))
    );
}
