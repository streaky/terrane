// Rust justification: large, externally reviewed, security-critical implementations.
// serde_json's scanner, yaml-rust2's non-expanding event parser, and url's WHATWG state
// machine are kept below the boundary; document policy, limits, and the public API stay in Terrane.

use serde::Deserializer;
use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use url::Url;
use yaml_rust2::parser::{Event, MarkedEventReceiver, Parser};
use yaml_rust2::scanner::{Marker, TScalarStyle};

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
    document: Option<Arc<Document>>,
}

impl DataResult {
    fn success(value: Document) -> Self {
        Self {
            failed: false,
            message: String::new(),
            path: "$".into(),
            expected: String::new(),
            encoded: canonical(&value),
            document: Some(Arc::new(value)),
        }
    }

    fn failure(
        message: impl Into<String>,
        path: impl Into<String>,
        expected: impl Into<String>,
    ) -> Self {
        Self {
            failed: true,
            message: message.into(),
            path: path.into(),
            expected: expected.into(),
            encoded: String::new(),
            document: None,
        }
    }

    fn value(&self) -> Option<&Document> {
        self.document.as_deref()
    }
}

struct DocumentSeed {
    reject_duplicates: bool,
    depth: usize,
    max_depth: usize,
}

impl<'de> DeserializeSeed<'de> for DocumentSeed {
    type Value = Document;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        if self.depth > self.max_depth {
            return Err(serde::de::Error::custom("document depth limit exceeded"));
        }
        deserializer.deserialize_any(DocumentVisitor {
            reject_duplicates: self.reject_duplicates,
            depth: self.depth,
            max_depth: self.max_depth,
        })
    }
}

struct DocumentVisitor {
    reject_duplicates: bool,
    depth: usize,
    max_depth: usize,
}

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = Document;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a document value")
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(Document::None)
    }
    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(Document::None)
    }
    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Document::Bool(value))
    }
    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Document::Integer(value.to_string()))
    }
    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Document::Integer(value.to_string()))
    }
    fn visit_i128<E: serde::de::Error>(self, value: i128) -> Result<Self::Value, E> {
        Ok(Document::Integer(value.to_string()))
    }
    fn visit_u128<E: serde::de::Error>(self, value: u128) -> Result<Self::Value, E> {
        Ok(Document::Integer(value.to_string()))
    }
    fn visit_f64<E: serde::de::Error>(self, _value: f64) -> Result<Self::Value, E> {
        Err(E::custom(
            "lossy floating-point document numbers are disabled",
        ))
    }
    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Document::String(value.to_owned()))
    }
    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(Document::String(value))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(DocumentSeed {
            reject_duplicates: self.reject_duplicates,
            depth: self.depth + 1,
            max_depth: self.max_depth,
        })? {
            values.push(value);
        }
        Ok(Document::List(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut values = BTreeMap::new();
        while let Some(key) = map.next_key::<String>()? {
            if key == "$serde_json::private::Number" {
                let number = map.next_value::<String>()?;
                return classify_number(&number).map_err(A::Error::custom);
            }
            if self.reject_duplicates && values.contains_key(&key) {
                return Err(A::Error::custom(format!("duplicate key `{key}`")));
            }
            let value = map.next_value_seed(DocumentSeed {
                reject_duplicates: self.reject_duplicates,
                depth: self.depth + 1,
                max_depth: self.max_depth,
            })?;
            values.insert(key, value);
        }
        Ok(Document::Map(values))
    }
}

fn classify_number(value: &str) -> Result<Document, String> {
    let normalized = normalize_number(value)?;
    if value.bytes().any(|byte| matches!(byte, b'.' | b'e' | b'E')) {
        Ok(Document::Decimal(normalized))
    } else {
        Ok(Document::Integer(normalized))
    }
}

#[must_use]
pub fn parse_json(
    input: &str,
    reject_duplicates: bool,
    max_depth: usize,
    max_bytes: usize,
) -> DataResult {
    if input.len() > max_bytes {
        return DataResult::failure("document exceeds byte limit", "$", "bounded JSON document");
    }
    let mut deserializer = serde_json::Deserializer::from_str(input);
    deserializer.disable_recursion_limit();
    let value = match (DocumentSeed {
        reject_duplicates,
        depth: 0,
        max_depth,
    })
    .deserialize(&mut deserializer)
    {
        Ok(value) => value,
        Err(error) => return DataResult::failure(error.to_string(), "$", "JSON value"),
    };
    if let Err(error) = deserializer.end() {
        return DataResult::failure(error.to_string(), "$", "JSON value");
    }
    DataResult::success(value)
}

#[must_use]
pub fn parse_yaml(
    input: &str,
    max_depth: usize,
    max_bytes: usize,
    max_aliases: usize,
) -> DataResult {
    if input.len() > max_bytes {
        return DataResult::failure("document exceeds byte limit", "$", "bounded YAML document");
    }
    let mut builder = YamlBuilder::new(max_depth, max_aliases);
    if let Err(error) = Parser::new_from_str(input).load(&mut builder, false) {
        return DataResult::failure(error.to_string(), "$", "safe YAML core document");
    }
    if let Some(error) = builder.error {
        return error;
    }
    builder.root.map_or_else(
        || DataResult::failure("YAML document is empty", "$", "safe YAML core document"),
        DataResult::success,
    )
}

enum YamlFrame {
    List {
        anchor: usize,
        values: Vec<Document>,
    },
    Map {
        anchor: usize,
        values: BTreeMap<String, Document>,
        key: Option<String>,
    },
}

struct YamlBuilder {
    root: Option<Document>,
    stack: Vec<YamlFrame>,
    anchors: BTreeMap<usize, Document>,
    expanded_nodes: usize,
    max_depth: usize,
    max_aliases: usize,
    error: Option<DataResult>,
}

impl YamlBuilder {
    fn new(max_depth: usize, max_aliases: usize) -> Self {
        Self {
            root: None,
            stack: Vec::new(),
            anchors: BTreeMap::new(),
            expanded_nodes: 0,
            max_depth,
            max_aliases,
            error: None,
        }
    }

    fn fail(&mut self, message: impl Into<String>, expected: impl Into<String>) {
        if self.error.is_none() {
            self.error = Some(DataResult::failure(message, "$", expected));
        }
    }

    fn begin(&mut self, frame: YamlFrame) {
        if self.stack.len() >= self.max_depth {
            self.fail("document depth limit exceeded", "bounded YAML document");
        } else {
            self.stack.push(frame);
        }
    }

    fn insert(&mut self, value: Document, anchor: usize) {
        if self.error.is_some() {
            return;
        }
        if anchor != 0 {
            self.anchors.insert(anchor, value.clone());
        }
        match self.stack.last_mut() {
            Some(YamlFrame::List { values, .. }) => values.push(value),
            Some(YamlFrame::Map { values, key, .. }) => {
                if let Some(name) = key.take() {
                    if values.insert(name.clone(), value).is_some() {
                        self.fail(format!("duplicate key `{name}`"), "unique YAML map key");
                    }
                } else if let Document::String(name) = value {
                    *key = Some(name);
                } else {
                    self.fail("safe YAML maps require string keys", "string map key");
                }
            }
            None if self.root.is_none() => self.root = Some(value),
            None => self.fail(
                "multiple YAML documents are not supported",
                "one YAML document",
            ),
        }
    }

    fn finish(&mut self, list: bool) {
        let Some(frame) = self.stack.pop() else {
            self.fail("unbalanced YAML collection", "balanced YAML document");
            return;
        };
        let (value, anchor) = match frame {
            YamlFrame::List { anchor, values } if list => (Document::List(values), anchor),
            YamlFrame::Map {
                anchor,
                values,
                key: None,
            } if !list => (Document::Map(values), anchor),
            YamlFrame::Map { key: Some(_), .. } => {
                self.fail("YAML map is missing a value", "complete YAML map entry");
                return;
            }
            _ => {
                self.fail("mismatched YAML collection", "balanced YAML document");
                return;
            }
        };
        self.insert(value, anchor);
    }
}

impl MarkedEventReceiver for YamlBuilder {
    fn on_event(&mut self, event: Event, _marker: Marker) {
        if self.error.is_some() {
            return;
        }
        match event {
            Event::Scalar(value, style, anchor, tag) => {
                if tag.is_some() {
                    self.fail(
                        "YAML tags are disabled by the safe schema",
                        "untagged safe YAML value",
                    );
                    return;
                }
                match yaml_scalar(&value, style) {
                    Ok(value) => self.insert(value, anchor),
                    Err(message) => self.fail(message, "safe YAML scalar"),
                }
            }
            Event::SequenceStart(anchor, tag) => {
                if tag.is_some() {
                    self.fail(
                        "YAML tags are disabled by the safe schema",
                        "untagged safe YAML value",
                    );
                } else {
                    self.begin(YamlFrame::List {
                        anchor,
                        values: Vec::new(),
                    });
                }
            }
            Event::MappingStart(anchor, tag) => {
                if tag.is_some() {
                    self.fail(
                        "YAML tags are disabled by the safe schema",
                        "untagged safe YAML value",
                    );
                } else {
                    self.begin(YamlFrame::Map {
                        anchor,
                        values: BTreeMap::new(),
                        key: None,
                    });
                }
            }
            Event::SequenceEnd => self.finish(true),
            Event::MappingEnd => self.finish(false),
            Event::Alias(anchor) => {
                let Some(value) = self.anchors.get(&anchor).cloned() else {
                    self.fail(
                        "YAML alias refers to an unknown anchor",
                        "defined YAML anchor",
                    );
                    return;
                };
                self.expanded_nodes = self.expanded_nodes.saturating_add(document_nodes(&value));
                if self.expanded_nodes > self.max_aliases {
                    self.fail(
                        "YAML alias expansion limit exceeded",
                        "bounded YAML alias expansion",
                    );
                } else {
                    self.insert(value, 0);
                }
            }
            Event::Nothing
            | Event::StreamStart
            | Event::StreamEnd
            | Event::DocumentStart
            | Event::DocumentEnd => {}
        }
    }
}

fn yaml_scalar(value: &str, style: TScalarStyle) -> Result<Document, String> {
    if style != TScalarStyle::Plain {
        return Ok(Document::String(value.to_owned()));
    }
    match value {
        "" | "~" | "null" | "Null" | "NULL" => Ok(Document::None),
        "true" | "True" | "TRUE" => Ok(Document::Bool(true)),
        "false" | "False" | "FALSE" => Ok(Document::Bool(false)),
        ".inf" | ".Inf" | ".INF" | "-.inf" | "-.Inf" | "-.INF" | ".nan" | ".NaN" | ".NAN" => {
            Err("non-finite YAML numbers are not supported".to_owned())
        }
        _ if is_json_number(value) => classify_number(value),
        _ => Ok(Document::String(value.to_owned())),
    }
}

fn is_json_number(value: &str) -> bool {
    serde_json::from_str::<serde_json::Number>(value).is_ok()
}

fn document_nodes(value: &Document) -> usize {
    match value {
        Document::List(values) => 1usize.saturating_add(
            values
                .iter()
                .map(document_nodes)
                .fold(0usize, usize::saturating_add),
        ),
        Document::Map(values) => 1usize.saturating_add(
            values
                .values()
                .map(document_nodes)
                .fold(0usize, usize::saturating_add),
        ),
        _ => 1,
    }
}

fn path_segment(key: &str) -> String {
    if key
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        key.to_owned()
    } else {
        format!(
            "[{}]",
            serde_json::to_string(key).expect("string serialization cannot fail")
        )
    }
}

#[must_use]
pub fn canonical_json(value: &DataResult) -> DataResult {
    value.value().cloned().map_or_else(
        || DataResult::failure("document value is unavailable", "$", "document value"),
        DataResult::success,
    )
}

fn canonical(value: &Document) -> String {
    match value {
        Document::None => "null".into(),
        Document::Bool(value) => value.to_string(),
        Document::Integer(value) | Document::Decimal(value) => value.clone(),
        Document::String(value) => {
            serde_json::to_string(value).expect("string serialization cannot fail")
        }
        Document::List(values) => format!(
            "[{}]",
            values.iter().map(canonical).collect::<Vec<_>>().join(",")
        ),
        Document::Map(values) => {
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_by(|(left, _), (right, _)| left.encode_utf16().cmp(right.encode_utf16()));
            format!(
                "{{{}}}",
                entries
                    .into_iter()
                    .map(|(key, value)| format!(
                        "{}:{}",
                        serde_json::to_string(key).expect("string serialization cannot fail"),
                        canonical(value)
                    ))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}

fn normalize_number(value: &str) -> Result<String, String> {
    let (negative, unsigned) = value
        .strip_prefix('-')
        .map_or((false, value), |unsigned| (true, unsigned));
    let (mantissa, exponent_text) = unsigned
        .split_once(['e', 'E'])
        .map_or((unsigned, "0"), |parts| parts);
    let exponent = exponent_text
        .parse::<i64>()
        .map_err(|_| "JSON number exponent is outside the supported exact range".to_owned())?;
    let (whole, fraction) = mantissa
        .split_once('.')
        .map_or((mantissa, ""), |parts| parts);
    let mut digits = format!("{whole}{fraction}");
    let first_nonzero = digits.find(|character| character != '0');
    let Some(first_nonzero) = first_nonzero else {
        return Ok("0".to_owned());
    };
    digits.drain(..first_nonzero);
    let decimal_position = i64::try_from(whole.len())
        .map_err(|_| "JSON number is too large".to_owned())?
        .checked_add(exponent)
        .and_then(|position| {
            i64::try_from(first_nonzero)
                .ok()
                .and_then(|leading| position.checked_sub(leading))
        })
        .ok_or_else(|| "JSON number exponent is outside the supported exact range".to_owned())?;
    while digits.ends_with('0') {
        digits.pop();
    }
    let digit_count =
        i64::try_from(digits.len()).map_err(|_| "JSON number is too large".to_owned())?;
    let sign = if negative { "-" } else { "" };
    if decimal_position > 0 && decimal_position <= 21 {
        if decimal_position >= digit_count {
            let zeros = usize::try_from(decimal_position - digit_count)
                .map_err(|_| "JSON number is too large".to_owned())?;
            return Ok(format!("{sign}{digits}{}", "0".repeat(zeros)));
        }
        let split =
            usize::try_from(decimal_position).map_err(|_| "JSON number is too large".to_owned())?;
        return Ok(format!("{sign}{}.{}", &digits[..split], &digits[split..]));
    }
    if decimal_position <= 0 && decimal_position > -6 {
        let zeros = usize::try_from(-decimal_position)
            .map_err(|_| "JSON number is too large".to_owned())?;
        return Ok(format!("{sign}0.{}{digits}", "0".repeat(zeros)));
    }
    let exponent = decimal_position - 1;
    let coefficient = if digits.len() == 1 {
        digits
    } else {
        format!("{}.{}", &digits[..1], &digits[1..])
    };
    let exponent_sign = if exponent >= 0 { "+" } else { "" };
    Ok(format!("{sign}{coefficient}e{exponent_sign}{exponent}"))
}

fn parse_default(input: &str) -> Option<Document> {
    parse_json(input, true, 256, input.len()).value().cloned()
}

#[must_use]
pub fn document_kind(result: &DataResult) -> String {
    result.value().map_or("invalid", kind_of).into()
}

#[must_use]
pub fn document_text(result: &DataResult) -> String {
    match result.value() {
        Some(Document::String(value) | Document::Integer(value) | Document::Decimal(value)) => {
            value.clone()
        }
        Some(Document::Bool(value)) => value.to_string(),
        Some(Document::None) => "none".into(),
        _ => String::new(),
    }
}

#[must_use]
pub fn document_length(result: &DataResult) -> usize {
    match result.value() {
        Some(Document::List(values)) => values.len(),
        Some(Document::Map(values)) => values.len(),
        _ => 0,
    }
}

#[must_use]
pub fn document_item(result: &DataResult, index: usize) -> DataResult {
    match result.value() {
        Some(Document::List(values)) => values.get(index).cloned().map_or_else(
            || {
                DataResult::failure(
                    "document index is out of range",
                    format!("$[{index}]"),
                    "existing list item",
                )
            },
            DataResult::success,
        ),
        _ => DataResult::failure("document value is not a list", "$", "list"),
    }
}

#[must_use]
pub fn document_coefficient(result: &DataResult) -> String {
    match result.value() {
        Some(Document::Decimal(value)) => decimal_parts(value).0,
        _ => String::new(),
    }
}

#[must_use]
pub fn document_exponent(result: &DataResult) -> i64 {
    match result.value() {
        Some(Document::Decimal(value)) => decimal_parts(value).1,
        _ => 0,
    }
}

fn decimal_parts(value: &str) -> (String, i64) {
    let (mantissa, scientific_exponent) = value
        .split_once('e')
        .map_or((value, 0), |(mantissa, exponent)| {
            (mantissa, exponent.parse::<i64>().unwrap_or(0))
        });
    let (whole, fraction) = mantissa
        .split_once('.')
        .map_or((mantissa, ""), |parts| parts);
    let coefficient = format!("{whole}{fraction}");
    let exponent = scientific_exponent - i64::try_from(fraction.len()).unwrap_or(i64::MAX);
    (coefficient, exponent)
}

#[must_use]
pub fn document_none() -> DataResult {
    DataResult::success(Document::None)
}

#[must_use]
pub fn document_bool(value: bool) -> DataResult {
    DataResult::success(Document::Bool(value))
}

#[must_use]
pub fn document_string(value: String) -> DataResult {
    DataResult::success(Document::String(value))
}

#[must_use]
pub fn document_integer(value: &str) -> DataResult {
    match classify_number(value) {
        Ok(Document::Integer(value)) => DataResult::success(Document::Integer(value)),
        _ => DataResult::failure("invalid exact document integer", "$", "document-integer"),
    }
}

#[must_use]
pub fn document_decimal(value: &str) -> DataResult {
    match classify_number(value) {
        Ok(Document::Decimal(value)) => DataResult::success(Document::Decimal(value)),
        _ => DataResult::failure("invalid exact document decimal", "$", "document-decimal"),
    }
}

#[must_use]
pub fn document_list() -> DataResult {
    DataResult::success(Document::List(Vec::new()))
}

#[must_use]
pub fn document_list_append(list: &DataResult, value: &DataResult) -> DataResult {
    let (Some(Document::List(values)), Some(value)) = (list.value(), value.value()) else {
        return DataResult::failure(
            "document list construction requires successful document values",
            "$",
            "document-value",
        );
    };
    let mut values = values.clone();
    values.push(value.clone());
    DataResult::success(Document::List(values))
}

#[must_use]
pub fn document_map() -> DataResult {
    DataResult::success(Document::Map(BTreeMap::new()))
}

#[must_use]
pub fn document_map_insert(map: &DataResult, key: String, value: &DataResult) -> DataResult {
    let (Some(Document::Map(values)), Some(value)) = (map.value(), value.value()) else {
        return DataResult::failure(
            "document map construction requires successful document values",
            "$",
            "document-value",
        );
    };
    if values.contains_key(&key) {
        return DataResult::failure(
            format!("duplicate key `{key}`"),
            format!("$.{}", path_segment(&key)),
            "unique document map key",
        );
    }
    let mut values = values.clone();
    values.insert(key, value.clone());
    DataResult::success(Document::Map(values))
}

#[must_use]
pub fn document_key(result: &DataResult, index: usize) -> String {
    match result.value() {
        Some(Document::Map(values)) => values.keys().nth(index).cloned().unwrap_or_default(),
        _ => String::new(),
    }
}

#[must_use]
pub fn document_field(result: &DataResult, key: &str) -> DataResult {
    match result.value() {
        Some(Document::Map(values)) => values.get(key).cloned().map_or_else(
            || {
                DataResult::failure(
                    "required field is missing",
                    format!("$.{}", path_segment(key)),
                    "present field",
                )
            },
            DataResult::success,
        ),
        _ => DataResult::failure("document value is not a map", "$", "map"),
    }
}

#[must_use]
pub fn validate_mapping(
    result: &DataResult,
    expected_kind: &str,
    required_fields: &[String],
    declared_fields: &[String],
    default_fields: &[String],
    default_values: &[String],
    allow_unknown: bool,
) -> DataResult {
    let Some(mut value) = result.value().cloned() else {
        return DataResult::failure("invalid document value", "$", expected_kind);
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
                let Some(default) = parse_default(encoded_default) else {
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
    DataResult::success(value)
}

fn kind_of(value: &Document) -> &'static str {
    match value {
        Document::None => "none",
        Document::Bool(_) => "bool",
        Document::Integer(_) => "integer",
        Document::Decimal(_) => "decimal",
        Document::String(_) => "string",
        Document::List(_) => "list",
        Document::Map(_) => "map",
    }
}

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
    let parsed = if base.is_empty() {
        Url::parse(input)
    } else {
        Url::parse(base).and_then(|base| base.join(input))
    };
    match parsed {
        Ok(url) => {
            let mut safe = url.clone();
            let _ = safe.set_username("");
            let _ = safe.set_password(None);
            UrlResult {
                failed: false,
                message: String::new(),
                serialized: url.to_string(),
                display: safe.to_string(),
                scheme: url.scheme().into(),
                username: url.username().into(),
                password: url.password().unwrap_or_default().into(),
                host: url.host_str().unwrap_or_default().into(),
                port: url.port().map_or_else(String::new, |port| port.to_string()),
                path: url.path().into(),
                query: url.query().unwrap_or_default().into(),
                query_entries: url
                    .query_pairs()
                    .map(|(key, value)| (key.into_owned(), value.into_owned()))
                    .collect(),
                fragment: url.fragment().unwrap_or_default().into(),
                origin: url.origin().ascii_serialization(),
            }
        }
        Err(error) => UrlResult {
            failed: true,
            message: error.to_string(),
            serialized: String::new(),
            display: String::new(),
            scheme: String::new(),
            username: String::new(),
            password: String::new(),
            host: String::new(),
            port: String::new(),
            path: String::new(),
            query: String::new(),
            query_entries: Vec::new(),
            fragment: String::new(),
            origin: String::new(),
        },
    }
}

#[must_use]
pub fn url_query_length(result: &UrlResult) -> usize {
    result.query_entries.len()
}
#[must_use]
pub fn url_query_key(result: &UrlResult, index: usize) -> String {
    result
        .query_entries
        .get(index)
        .map(|entry| entry.0.clone())
        .unwrap_or_default()
}
#[must_use]
pub fn url_query_value(result: &UrlResult, index: usize) -> String {
    result
        .query_entries
        .get(index)
        .map(|entry| entry.1.clone())
        .unwrap_or_default()
}
