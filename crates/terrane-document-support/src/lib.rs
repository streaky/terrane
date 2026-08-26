use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;
use std::collections::BTreeMap;
use std::fmt;
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Document {
    None,
    Bool(bool),
    Integer(String),
    Decimal(String),
    String(String),
    List(Vec<Document>),
    Map(BTreeMap<String, Document>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataResult {
    pub failed: bool,
    pub message: String,
    pub path: String,
    pub expected: String,
    pub encoded: String,
}

impl DataResult {
    fn success(encoded: String) -> Self {
        Self { failed: false, message: String::new(), path: "$".into(), expected: String::new(), encoded }
    }

    fn failure(message: impl Into<String>, path: impl Into<String>, expected: impl Into<String>) -> Self {
        Self { failed: true, message: message.into(), path: path.into(), expected: expected.into(), encoded: String::new() }
    }
}

struct DocumentSeed { reject_duplicates: bool }

impl<'de> DeserializeSeed<'de> for DocumentSeed {
    type Value = Document;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(DocumentVisitor { reject_duplicates: self.reject_duplicates })
    }
}

struct DocumentVisitor { reject_duplicates: bool }

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = Document;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { formatter.write_str("a document value") }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> { Ok(Document::None) }
    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> { Ok(Document::None) }
    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> { Ok(Document::Bool(value)) }
    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> { Ok(Document::Integer(value.to_string())) }
    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> { Ok(Document::Integer(value.to_string())) }
    fn visit_i128<E: serde::de::Error>(self, value: i128) -> Result<Self::Value, E> { Ok(Document::Integer(value.to_string())) }
    fn visit_u128<E: serde::de::Error>(self, value: u128) -> Result<Self::Value, E> { Ok(Document::Integer(value.to_string())) }
    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> { Ok(Document::Decimal(value.to_string())) }
    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> { Ok(Document::String(value.to_owned())) }
    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> { Ok(Document::String(value)) }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(DocumentSeed { reject_duplicates: self.reject_duplicates })? { values.push(value); }
        Ok(Document::List(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut values = BTreeMap::new();
        while let Some(key) = map.next_key::<String>()? {
            if key == "$serde_json::private::Number" {
                let number = map.next_value::<String>()?;
                return Ok(classify_number(&number));
            }
            if self.reject_duplicates && values.contains_key(&key) { return Err(A::Error::custom(format!("duplicate key `{key}`"))); }
            let value = map.next_value_seed(DocumentSeed { reject_duplicates: self.reject_duplicates })?;
            values.insert(key, value);
        }
        Ok(Document::Map(values))
    }
}

fn classify_number(value: &str) -> Document {
    if value.bytes().any(|byte| matches!(byte, b'.' | b'e' | b'E')) {
        Document::Decimal(value.to_owned())
    } else {
        Document::Integer(value.to_owned())
    }
}


pub fn parse_json(input: &str, reject_duplicates: bool, max_depth: usize, max_bytes: usize) -> DataResult {
    if input.len() > max_bytes { return DataResult::failure("document exceeds byte limit", "$", "bounded JSON document"); }
    let mut deserializer = serde_json::Deserializer::from_str(input);
    if let Err(error) = (DocumentSeed { reject_duplicates }).deserialize(&mut deserializer) {
        return DataResult::failure(error.to_string(), "$", "JSON value");
    }
    if let Err(error) = deserializer.end() {
        return DataResult::failure(error.to_string(), "$", "JSON value");
    }
    let value = match serde_json::from_str::<serde_json::Value>(input)
        .ok()
        .and_then(|value| from_json_value(&value))
    {
        Some(value) => value,
        None => return DataResult::failure("cannot materialize JSON value", "$", "JSON value"),
    };
    if let Err(message) = enforce_depth(&value, 0, max_depth) {
        return DataResult::failure(message, "$", "bounded JSON document");
    }
    DataResult::success(canonical(&value))
}

pub fn parse_yaml(input: &str, max_depth: usize, max_bytes: usize, max_aliases: usize) -> DataResult {
    if input.len() > max_bytes { return DataResult::failure("document exceeds byte limit", "$", "bounded YAML document"); }
    let aliases = input.bytes().filter(|byte| *byte == b'*').count();
    if aliases > max_aliases { return DataResult::failure("YAML alias expansion limit exceeded", "$", "safe YAML core document"); }
    let yaml: serde_yaml::Value = match serde_yaml::from_str(input) {
        Ok(value) => value,
        Err(error) => return DataResult::failure(error.to_string(), "$", "safe YAML core document"),
    };
    match from_yaml(&yaml, "$", 0, max_depth) {
        Ok(value) => DataResult::success(canonical(&value)),
        Err(result) => result,
    }
}

fn from_yaml(value: &serde_yaml::Value, path: &str, depth: usize, max_depth: usize) -> Result<Document, DataResult> {
    if depth > max_depth { return Err(DataResult::failure("document depth limit exceeded", path, "bounded YAML document")); }
    match value {
        serde_yaml::Value::Null => Ok(Document::None),
        serde_yaml::Value::Bool(value) => Ok(Document::Bool(*value)),
        serde_yaml::Value::Number(value) => {
            let text = value.to_string();
            Ok(if text.bytes().any(|byte| matches!(byte, b'.' | b'e' | b'E')) { Document::Decimal(text) } else { Document::Integer(text) })
        }
        serde_yaml::Value::String(value) => Ok(Document::String(value.clone())),
        serde_yaml::Value::Sequence(values) => values.iter().enumerate().map(|(index, value)| from_yaml(value, &format!("{path}[{index}]"), depth + 1, max_depth)).collect::<Result<Vec<_>, _>>().map(Document::List),
        serde_yaml::Value::Mapping(values) => {
            let mut result = BTreeMap::new();
            for (key, value) in values {
                let serde_yaml::Value::String(key) = key else { return Err(DataResult::failure("safe YAML maps require string keys", path, "string map key")); };
                let child_path = format!("{path}.{}", path_segment(key));
                result.insert(key.clone(), from_yaml(value, &child_path, depth + 1, max_depth)?);
            }
            Ok(Document::Map(result))
        }
        serde_yaml::Value::Tagged(_) => Err(DataResult::failure("YAML tags are disabled by the safe schema", path, "untagged safe YAML value")),
    }
}

fn enforce_depth(value: &Document, depth: usize, max_depth: usize) -> Result<(), String> {
    if depth > max_depth { return Err("document depth limit exceeded".into()); }
    match value {
        Document::List(values) => values.iter().try_for_each(|value| enforce_depth(value, depth + 1, max_depth)),
        Document::Map(values) => values.values().try_for_each(|value| enforce_depth(value, depth + 1, max_depth)),
        _ => Ok(()),
    }
}

fn path_segment(key: &str) -> String {
    if key.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-') { key.to_owned() } else { format!("[{}]", serde_json::to_string(key).expect("string serialization cannot fail")) }
}

pub fn canonical_json(input: &str) -> DataResult { parse_json(input, true, 256, usize::MAX) }

fn canonical(value: &Document) -> String {
    match value {
        Document::None => "null".into(),
        Document::Bool(value) => value.to_string(),
        Document::Integer(value) | Document::Decimal(value) => normalize_number(value),
        Document::String(value) => serde_json::to_string(value).expect("string serialization cannot fail"),
        Document::List(values) => format!("[{}]", values.iter().map(canonical).collect::<Vec<_>>().join(",")),
        Document::Map(values) => {
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_by(|(left, _), (right, _)| left.encode_utf16().cmp(right.encode_utf16()));
            format!("{{{}}}", entries.into_iter().map(|(key, value)| format!("{}:{}", serde_json::to_string(key).expect("string serialization cannot fail"), canonical(value))).collect::<Vec<_>>().join(","))
        }
    }
}

fn normalize_number(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    let (mantissa, exponent) = lower.split_once('e').unwrap_or((&lower, "0"));
    let exponent = exponent.parse::<i64>().unwrap_or(0);
    if let Some((whole, fraction)) = mantissa.split_once('.') {
        let trimmed = fraction.trim_end_matches('0');
        if trimmed.is_empty() && exponent == 0 { whole.to_owned() } else if exponent == 0 { format!("{whole}.{trimmed}") } else { format!("{whole}.{trimmed}e{exponent}") }
    } else if exponent == 0 { mantissa.to_owned() } else { format!("{mantissa}e{exponent}") }
}

fn parsed(input: &str) -> Option<Document> { parse_json(input, true, 256, usize::MAX).encoded.parse::<serde_json::Value>().ok().and_then(|value| from_json_value(&value)) }

fn from_json_value(value: &serde_json::Value) -> Option<Document> {
    match value {
        serde_json::Value::Null => Some(Document::None),
        serde_json::Value::Bool(value) => Some(Document::Bool(*value)),
        serde_json::Value::Number(value) => Some(classify_number(&value.to_string())),
        serde_json::Value::String(value) => Some(Document::String(value.clone())),
        serde_json::Value::Array(values) => values.iter().map(from_json_value).collect::<Option<Vec<_>>>().map(Document::List),
        serde_json::Value::Object(values) => values.iter().map(|(key, value)| Some((key.clone(), from_json_value(value)?))).collect::<Option<BTreeMap<_, _>>>().map(Document::Map),
    }
}

pub fn document_kind(encoded: &str) -> String {
    match parsed(encoded) { Some(Document::None) => "none", Some(Document::Bool(_)) => "bool", Some(Document::Integer(_)) => "integer", Some(Document::Decimal(_)) => "decimal", Some(Document::String(_)) => "string", Some(Document::List(_)) => "list", Some(Document::Map(_)) => "map", None => "invalid" }.into()
}

pub fn document_text(encoded: &str) -> String {
    match parsed(encoded) { Some(Document::String(value) | Document::Integer(value) | Document::Decimal(value)) => value, Some(Document::Bool(value)) => value.to_string(), Some(Document::None) => "none".into(), _ => String::new() }
}

pub fn document_length(encoded: &str) -> usize { match parsed(encoded) { Some(Document::List(values)) => values.len(), Some(Document::Map(values)) => values.len(), _ => 0 } }

pub fn document_item(encoded: &str, index: usize) -> DataResult {
    match parsed(encoded) {
        Some(Document::List(values)) => values.get(index).map_or_else(|| DataResult::failure("document index is out of range", format!("$[{index}]"), "existing list item"), |value| DataResult::success(canonical(value))),
        _ => DataResult::failure("document value is not a list", "$", "list"),
    }
}

pub fn document_key(encoded: &str, index: usize) -> String {
    match parsed(encoded) { Some(Document::Map(values)) => values.keys().nth(index).cloned().unwrap_or_default(), _ => String::new() }
}

pub fn document_field(encoded: &str, key: &str) -> DataResult {
    match parsed(encoded) {
        Some(Document::Map(values)) => values.get(key).map_or_else(|| DataResult::failure("required field is missing", format!("$.{}", path_segment(key)), "present field"), |value| DataResult::success(canonical(value))),
        _ => DataResult::failure("document value is not a map", "$", "map"),
    }
}

pub fn validate_mapping(
    encoded: &str,
    expected_kind: &str,
    required_fields: &[String],
    declared_fields: &[String],
    default_fields: &[String],
    default_values: &[String],
    allow_unknown: bool,
) -> DataResult {
    let Some(mut value) = parsed(encoded) else {
        return DataResult::failure("invalid document encoding", "$", expected_kind);
    };
    if kind_of(&value) != expected_kind {
        return DataResult::failure("document value has the wrong kind", "$", expected_kind);
    }
    if default_fields.len() != default_values.len() {
        return DataResult::failure(
            "default field and value counts differ",
            "$",
            "matching descriptor defaults",
        );
    }
    if let Document::Map(values) = &mut value {
        for field in required_fields {
            if !values.contains_key(field) {
                return DataResult::failure(
                    "required field is missing",
                    format!("$.{}", path_segment(field)),
                    "present field",
                );
            }
        }
        if !allow_unknown {
            for field in values.keys() {
                if !declared_fields.contains(field) {
                    return DataResult::failure(
                        "unknown field",
                        format!("$.{}", path_segment(field)),
                        "declared field",
                    );
                }
            }
        }
        for (field, encoded_default) in default_fields.iter().zip(default_values) {
            if !values.contains_key(field) {
                let Some(default) = parsed(encoded_default) else {
                    return DataResult::failure(
                        "invalid encoded default value",
                        format!("$.{}", path_segment(field)),
                        "document value default",
                    );
                };
                values.insert(field.clone(), default);
            }
        }
    }
    DataResult::success(canonical(&value))
}

fn kind_of(value: &Document) -> &'static str { match value { Document::None => "none", Document::Bool(_) => "bool", Document::Integer(_) => "integer", Document::Decimal(_) => "decimal", Document::String(_) => "string", Document::List(_) => "list", Document::Map(_) => "map" } }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UrlResult {
    pub failed: bool,
    pub message: String,
    pub serialized: String,
    pub display: String,
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub path: String,
    pub query: String,
    pub query_entries: Vec<(String, String)>,
    pub fragment: String,
    pub origin: String,
}

pub fn parse_url(input: &str, base: &str) -> UrlResult {
    let parsed = if base.is_empty() { Url::parse(input) } else { Url::parse(base).and_then(|base| base.join(input)) };
    match parsed {
        Ok(url) => {
            let mut safe = url.clone();
            let _ = safe.set_username("");
            let _ = safe.set_password(None);
            UrlResult { failed: false, message: String::new(), serialized: url.to_string(), display: safe.to_string(), scheme: url.scheme().into(), username: url.username().into(), password: url.password().unwrap_or_default().into(), host: url.host_str().unwrap_or_default().into(), port: url.port().map_or_else(String::new, |port| port.to_string()), path: url.path().into(), query: url.query().unwrap_or_default().into(), query_entries: url.query_pairs().map(|(key, value)| (key.into_owned(), value.into_owned())).collect(), fragment: url.fragment().unwrap_or_default().into(), origin: url.origin().ascii_serialization() }
        }
        Err(error) => UrlResult { failed: true, message: error.to_string(), serialized: String::new(), display: String::new(), scheme: String::new(), username: String::new(), password: String::new(), host: String::new(), port: String::new(), path: String::new(), query: String::new(), query_entries: Vec::new(), fragment: String::new(), origin: String::new() },
    }
}

pub fn url_query_length(result: &UrlResult) -> usize { result.query_entries.len() }
pub fn url_query_key(result: &UrlResult, index: usize) -> String { result.query_entries.get(index).map(|entry| entry.0.clone()).unwrap_or_default() }
pub fn url_query_value(result: &UrlResult, index: usize) -> String { result.query_entries.get(index).map(|entry| entry.1.clone()).unwrap_or_default() }
