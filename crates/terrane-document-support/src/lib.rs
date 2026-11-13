// Rust justifications: large, externally reviewed, security-critical implementations, plus parser-
// event policy that must run below an expanding representation to enforce resource limits. The
// json-event-parser, yaml-rust2 non-expanding event parser, safe YAML event receiver, and url
// WHATWG state machine stay below the boundary; the public coherent API remains authored in Terrane.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;
use url::Url;
use yaml_rust2::parser::{Event, MarkedEventReceiver, Parser};
use yaml_rust2::scanner::{Marker, TScalarStyle};

const DESCRIPTOR_DEFAULT_DEPTH: usize = 256;
const MAXIMUM_JSON_DEPTH: usize = 512;
const MAXIMUM_YAML_DEPTH: usize = 255;

mod native;
pub use native::{
    JsonDocument, NativeJsonError, document_to_json_value, from_document, json_value_to_document,
    serialize_document, serialize_json_value, to_document,
};
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
        // `encoded` is retained for compatibility, but this internal snapshot must not impose
        // a second, hidden byte budget on parse or incremental document construction.
        let encoded = match write_json_bounded(&value, MAXIMUM_JSON_DEPTH, usize::MAX) {
            Ok(encoded) => encoded,
            Err(error) => return Self::failure(error.to_string(), "$", "bounded document output"),
        };
        Self {
            failed: false,
            message: String::new(),
            path: "$".into(),
            expected: String::new(),
            encoded,
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
    /// Borrow this result's parsed document tree without reparsing the compatibility
    /// `encoded` snapshot or cloning the tree. Returns `None` when parsing failed.
    ///
    /// The returned reference is read-only and is valid only while this result is
    /// borrowed. It is intended for concrete native adapters that can deserialize
    /// directly from the tree.
    #[must_use]
    pub fn value(&self) -> Option<&Document> {
        self.document.as_deref()
    }
}

enum JsonParseFrame {
    List(Vec<Document>),
    Map(BTreeMap<String, Document>, Option<String>),
}

fn attach_json_value(
    frames: &mut [JsonParseFrame],
    root: &mut Option<Document>,
    value: Document,
) -> Result<(), String> {
    match frames.last_mut() {
        Some(JsonParseFrame::List(items)) => items.push(value),
        Some(JsonParseFrame::Map(fields, pending)) => {
            let key = pending
                .take()
                .ok_or_else(|| "object value without key".to_owned())?;
            fields.insert(key, value);
        }
        None if root.is_none() => *root = Some(value),
        None => return Err("multiple JSON values".into()),
    }
    Ok(())
}

fn parse_json_events(input: &str, max_depth: usize) -> Result<Document, String> {
    use json_event_parser::{JsonEvent, SliceJsonParser};
    if input.starts_with('\u{feff}') {
        return Err("JSON byte-order mark is not permitted".into());
    }
    let mut parser = SliceJsonParser::new(input.as_bytes());
    let mut frames = Vec::new();
    let mut root = None;
    loop {
        let event = parser.parse_next().map_err(|error| {
            // The upstream prose can misclassify malformed surrogates as EOF
            // and leading-zero numbers as additional root values. Its structured
            // location remains authoritative; do not maintain a second scanner.
            let position = error.location().start;
            format!(
                "invalid JSON syntax at line {} column {} (byte {})",
                position.line + 1,
                position.column + 1,
                position.offset,
            )
        })?;
        if frames.len() > max_depth
            && matches!(
                event,
                JsonEvent::String(_)
                    | JsonEvent::Number(_)
                    | JsonEvent::Boolean(_)
                    | JsonEvent::Null
            )
        {
            return Err("document depth limit exceeded".into());
        }
        match event {
            JsonEvent::StartArray | JsonEvent::StartObject => {
                if frames.len() > max_depth {
                    return Err("document depth limit exceeded".into());
                }
                frames.push(if matches!(event, JsonEvent::StartArray) {
                    JsonParseFrame::List(Vec::new())
                } else {
                    JsonParseFrame::Map(BTreeMap::new(), None)
                });
            }
            JsonEvent::EndArray => match frames.pop() {
                Some(JsonParseFrame::List(values)) => {
                    attach_json_value(&mut frames, &mut root, Document::List(values))?;
                }
                _ => return Err("unexpected end of array".into()),
            },
            JsonEvent::EndObject => match frames.pop() {
                Some(JsonParseFrame::Map(values, None)) => {
                    attach_json_value(&mut frames, &mut root, Document::Map(values))?;
                }
                _ => return Err("unexpected end of object".into()),
            },
            JsonEvent::ObjectKey(key) => match frames.last_mut() {
                Some(JsonParseFrame::Map(fields, pending)) if pending.is_none() => {
                    let key = key.into_owned();
                    if fields.contains_key(&key) {
                        return Err(format!("duplicate key `{key}`"));
                    }
                    *pending = Some(key);
                }
                _ => return Err("unexpected object key".into()),
            },
            JsonEvent::String(value) => {
                attach_json_value(&mut frames, &mut root, Document::String(value.into_owned()))?;
            }
            JsonEvent::Number(value) => {
                attach_json_value(&mut frames, &mut root, classify_number(&value)?)?;
            }
            JsonEvent::Boolean(value) => {
                attach_json_value(&mut frames, &mut root, Document::Bool(value))?;
            }
            JsonEvent::Null => attach_json_value(&mut frames, &mut root, Document::None)?,
            JsonEvent::Eof if frames.is_empty() => {
                return root.ok_or_else(|| "expected JSON value".into());
            }
            JsonEvent::Eof => return Err("unexpected end of JSON input".into()),
        }
        if frames.len() > max_depth.saturating_add(1) {
            return Err("document depth limit exceeded".into());
        }
    }
}

fn classify_number(value: &str) -> Result<Document, String> {
    let normalized = normalize_number(value)?;
    let (_, exponent) = decimal_parts(&normalized);
    if exponent >= 0 {
        Ok(Document::Integer(normalized))
    } else {
        Ok(Document::Decimal(normalized))
    }
}

/// Parses a bounded JSON tree without canonical serialization.
///
/// # Errors
///
/// Returns the same structured failure as [`parse_json`] for malformed input, duplicate
/// keys, unsupported exact number representations, or exceeded byte/depth limits.
pub fn parse_json_document(
    input: &str,
    max_depth: usize,
    max_bytes: usize,
) -> Result<Document, DataResult> {
    if max_depth > MAXIMUM_JSON_DEPTH {
        return Err(DataResult::failure(
            format!("JSON depth limit cannot exceed {MAXIMUM_JSON_DEPTH}"),
            "$",
            "bounded JSON document",
        ));
    }
    if input.len() > max_bytes {
        return Err(DataResult::failure(
            "document exceeds byte limit",
            "$",
            "bounded JSON document",
        ));
    }
    parse_json_events(input, max_depth)
        .map_err(|error| DataResult::failure(error, "$", "JSON value"))
}

#[must_use]
pub fn parse_json(input: &str, max_depth: usize, max_bytes: usize) -> DataResult {
    match parse_json_document(input, max_depth, max_bytes) {
        Ok(value) => DataResult::success(value),
        Err(error) => error,
    }
}

/// Parses a bounded YAML tree without canonical serialization.
///
/// # Errors
///
/// Returns the same structured failure as [`parse_yaml`] for malformed or unsafe YAML,
/// duplicate keys, or exceeded byte, depth, and alias-expansion limits.
pub fn parse_yaml_document(
    input: &str,
    max_depth: usize,
    max_bytes: usize,
    max_alias_nodes: usize,
) -> Result<Document, DataResult> {
    if max_depth > MAXIMUM_YAML_DEPTH {
        return Err(DataResult::failure(
            format!("YAML depth limit cannot exceed {MAXIMUM_YAML_DEPTH}"),
            "$",
            "bounded YAML document",
        ));
    }
    if input.len() > max_bytes {
        return Err(DataResult::failure(
            "document exceeds byte limit",
            "$",
            "bounded YAML document",
        ));
    }
    let mut builder = YamlBuilder::new(max_depth, max_alias_nodes);
    let parsed = Parser::new_from_str(input).load(&mut builder, false);
    if let Some(error) = builder.error {
        return Err(error);
    }
    if let Err(error) = parsed {
        let message = error.to_string();
        if message.contains("recursion limit exceeded") {
            return Err(DataResult::failure(
                "document depth limit exceeded",
                "$",
                "bounded YAML document",
            ));
        }
        return Err(DataResult::failure(message, "$", "safe YAML core document"));
    }
    builder.root.ok_or_else(|| {
        DataResult::failure("YAML document is empty", "$", "safe YAML core document")
    })
}

#[must_use]
pub fn parse_yaml(
    input: &str,
    max_depth: usize,
    max_bytes: usize,
    max_alias_nodes: usize,
) -> DataResult {
    match parse_yaml_document(input, max_depth, max_bytes, max_alias_nodes) {
        Ok(value) => DataResult::success(value),
        Err(error) => error,
    }
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
    max_alias_nodes: usize,
    error: Option<DataResult>,
}

impl YamlBuilder {
    fn new(max_depth: usize, max_alias_nodes: usize) -> Self {
        Self {
            root: None,
            stack: Vec::new(),
            anchors: BTreeMap::new(),
            expanded_nodes: 0,
            max_depth,
            max_alias_nodes,
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
                if self.expanded_nodes > self.max_alias_nodes {
                    self.fail(
                        "YAML alias node limit exceeded",
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
    use json_event_parser::{JsonEvent, SliceJsonParser};
    let mut parser = SliceJsonParser::new(value.as_bytes());
    matches!(parser.parse_next(), Ok(JsonEvent::Number(number)) if number == value)
        && matches!(parser.parse_next(), Ok(JsonEvent::Eof))
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

pub const MAXIMUM_NUMBER_DIGITS: usize = 4096;
pub const MAXIMUM_NUMBER_EXPONENT: i64 = 100_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentError(pub String);

impl fmt::Display for DocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for DocumentError {}

enum JsonFrame {
    Array(usize),
    Object {
        entries: usize,
        awaiting_value: bool,
    },
}

/// Incremental JSON output with explicit depth and encoded-byte limits.
pub struct JsonWriter {
    output: String,
    frames: Vec<JsonFrame>,
    max_depth: usize,
    max_bytes: usize,
    has_root: bool,
    poisoned: bool,
}

impl JsonWriter {
    /// # Errors
    /// Rejects a depth ceiling above the supported JSON limit.
    pub fn new(max_depth: usize, max_bytes: usize) -> Result<Self, DocumentError> {
        if max_depth > MAXIMUM_JSON_DEPTH {
            return Err(DocumentError(format!(
                "JSON depth limit cannot exceed {MAXIMUM_JSON_DEPTH}"
            )));
        }
        Ok(Self {
            output: String::new(),
            frames: Vec::new(),
            max_depth,
            max_bytes,
            has_root: false,
            poisoned: false,
        })
    }

    fn append(&mut self, text: &str) -> Result<(), DocumentError> {
        if self.poisoned {
            return Err(DocumentError(
                "JSON writer is unusable after an output failure".into(),
            ));
        }
        if self.output.len().saturating_add(text.len()) > self.max_bytes {
            self.poisoned = true;
            return Err(DocumentError("document exceeds byte limit".into()));
        }
        self.output.push_str(text);
        Ok(())
    }

    fn before_value(&mut self) -> Result<(), DocumentError> {
        if self.poisoned {
            return Err(DocumentError(
                "JSON writer is unusable after an output failure".into(),
            ));
        }
        if self.frames.len() > self.max_depth {
            self.poisoned = true;
            return Err(DocumentError("document depth limit exceeded".into()));
        }
        let action = match self.frames.last_mut() {
            Some(JsonFrame::Array(count)) => {
                let comma = *count > 0;
                *count += 1;
                comma
            }
            Some(JsonFrame::Object { awaiting_value, .. }) if *awaiting_value => {
                *awaiting_value = false;
                false
            }
            Some(JsonFrame::Object { .. }) => {
                return Err(DocumentError("JSON object value requires a key".into()));
            }
            None if self.has_root => return Err(DocumentError("multiple JSON root values".into())),
            None => {
                self.has_root = true;
                false
            }
        };
        if action { self.append(",") } else { Ok(()) }
    }

    fn quoted(&mut self, text: &str) -> Result<(), DocumentError> {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        self.append("\"")?;
        for character in text.chars() {
            match character {
                '"' => self.append("\\\"")?,
                '\\' => self.append("\\\\")?,
                '\u{08}' => self.append("\\b")?,
                '\u{0c}' => self.append("\\f")?,
                '\n' => self.append("\\n")?,
                '\r' => self.append("\\r")?,
                '\t' => self.append("\\t")?,
                character if character <= '\u{1f}' => {
                    let code = character as u8;
                    let escaped = [
                        b'\\',
                        b'u',
                        b'0',
                        b'0',
                        HEX[(code >> 4) as usize],
                        HEX[(code & 0xf) as usize],
                    ];
                    self.append(std::str::from_utf8(&escaped).expect("ASCII escape is UTF-8"))?;
                }
                character => {
                    let mut encoded = [0; 4];
                    self.append(character.encode_utf8(&mut encoded))?;
                }
            }
        }
        self.append("\"")
    }

    /// # Errors
    /// Rejects keys outside an object or output exceeding the byte budget.
    pub fn key(&mut self, key: &str) -> Result<(), DocumentError> {
        let comma = match self.frames.last_mut() {
            Some(JsonFrame::Object {
                entries,
                awaiting_value,
            }) if !*awaiting_value => {
                let comma = *entries > 0;
                *entries += 1;
                *awaiting_value = true;
                comma
            }
            _ => return Err(DocumentError("JSON key requires an object".into())),
        };
        if comma {
            self.append(",")?;
        }
        self.quoted(key)?;
        self.append(":")
    }

    /// # Errors
    /// Rejects invalid value placement, excessive depth or exhausted output.
    pub fn begin_array(&mut self) -> Result<(), DocumentError> {
        self.begin(true)
    }
    /// # Errors
    /// Rejects invalid value placement, excessive depth or exhausted output.
    pub fn begin_object(&mut self) -> Result<(), DocumentError> {
        self.begin(false)
    }

    fn begin(&mut self, array: bool) -> Result<(), DocumentError> {
        if self.frames.len() > self.max_depth {
            return Err(DocumentError("document depth limit exceeded".into()));
        }
        self.before_value()?;
        self.append(if array { "[" } else { "{" })?;
        self.frames.push(if array {
            JsonFrame::Array(0)
        } else {
            JsonFrame::Object {
                entries: 0,
                awaiting_value: false,
            }
        });
        Ok(())
    }

    /// # Errors
    /// Rejects an unmatched array end or exhausted output.
    pub fn end_array(&mut self) -> Result<(), DocumentError> {
        self.end(true)
    }
    /// # Errors
    /// Rejects an unmatched object end, a missing value or exhausted output.
    pub fn end_object(&mut self) -> Result<(), DocumentError> {
        self.end(false)
    }

    fn end(&mut self, array: bool) -> Result<(), DocumentError> {
        match self.frames.pop() {
            Some(JsonFrame::Array(_)) if array => self.append("]"),
            Some(JsonFrame::Object {
                awaiting_value: false,
                ..
            }) if !array => self.append("}"),
            Some(frame) => {
                self.frames.push(frame);
                Err(DocumentError("unbalanced JSON writer state".into()))
            }
            None => Err(DocumentError("unbalanced JSON writer state".into())),
        }
    }

    /// # Errors
    /// Rejects invalid value placement, excessive depth or exhausted output.
    pub fn string(&mut self, value: &str) -> Result<(), DocumentError> {
        self.before_value()?;
        self.quoted(value)
    }
    /// # Errors
    /// Rejects invalid value placement, excessive depth or exhausted output.
    pub fn bool(&mut self, value: bool) -> Result<(), DocumentError> {
        self.scalar(if value { "true" } else { "false" })
    }
    /// # Errors
    /// Rejects invalid value placement, excessive depth or exhausted output.
    pub fn null(&mut self) -> Result<(), DocumentError> {
        self.scalar("null")
    }
    /// # Errors
    /// Rejects malformed, fractional or oversized numbers and output failures.
    pub fn integer(&mut self, value: &str) -> Result<(), DocumentError> {
        if value.len() > MAXIMUM_NUMBER_DIGITS + 32
            || !is_json_number(value)
            || !matches!(classify_number(value), Ok(Document::Integer(_)))
        {
            return Err(DocumentError("invalid or oversized JSON integer".into()));
        }
        self.scalar(value)
    }
    /// # Errors
    /// Rejects malformed or oversized numbers and output failures.
    pub fn decimal(&mut self, value: &str) -> Result<(), DocumentError> {
        if value.len() > MAXIMUM_NUMBER_DIGITS + 32
            || !is_json_number(value)
            || normalize_number(value).is_err()
        {
            return Err(DocumentError("invalid or oversized JSON decimal".into()));
        }
        self.scalar(value)
    }
    fn scalar(&mut self, value: &str) -> Result<(), DocumentError> {
        self.before_value()?;
        self.append(value)
    }
    /// # Errors
    /// Rejects invalid document numbers, writer state or exhausted limits.
    pub fn document(&mut self, value: &Document) -> Result<(), DocumentError> {
        match value {
            Document::None => self.null(),
            Document::Bool(value) => self.bool(*value),
            Document::Integer(value) => self.integer(value),
            Document::Decimal(value) => self.decimal(value),
            Document::String(value) => self.string(value),
            Document::List(values) => {
                self.begin_array()?;
                for value in values {
                    self.document(value)?;
                }
                self.end_array()
            }
            Document::Map(values) => {
                self.begin_object()?;
                let mut entries = values.iter().collect::<Vec<_>>();
                entries
                    .sort_by(|(left, _), (right, _)| left.encode_utf16().cmp(right.encode_utf16()));
                for (key, value) in entries {
                    self.key(key)?;
                    self.document(value)?;
                }
                self.end_object()
            }
        }
    }
    /// # Errors
    /// Rejects incomplete documents and writers poisoned by output failures.
    pub fn finish(self) -> Result<String, DocumentError> {
        if self.poisoned {
            return Err(DocumentError(
                "JSON writer is unusable after an output failure".into(),
            ));
        }
        if !self.has_root || !self.frames.is_empty() {
            return Err(DocumentError("incomplete JSON document".into()));
        }
        Ok(self.output)
    }
}

/// # Errors
/// Rejects an unavailable document or a writer state/resource-limit failure.
pub fn write_document_value(
    writer: &mut JsonWriter,
    value: &DataResult,
) -> Result<(), DocumentError> {
    let value = value
        .value()
        .ok_or_else(|| DocumentError("document value is unavailable".into()))?;
    writer.document(value)
}

/// # Errors
/// Rejects invalid numeric representations or exceeded depth/output budgets.
pub fn write_json_bounded(
    value: &Document,
    max_depth: usize,
    max_bytes: usize,
) -> Result<String, DocumentError> {
    fn emit(
        writer: &mut JsonWriter,
        value: &Document,
        depth: usize,
        max_depth: usize,
    ) -> Result<(), DocumentError> {
        if depth > max_depth {
            return Err(DocumentError("document depth limit exceeded".into()));
        }
        match value {
            Document::None => writer.null(),
            Document::Bool(value) => writer.bool(*value),
            Document::Integer(value) => writer.integer(value),
            Document::Decimal(value) => writer.decimal(value),
            Document::String(value) => writer.string(value),
            Document::List(values) => {
                writer.begin_array()?;
                for item in values {
                    emit(writer, item, depth + 1, max_depth)?;
                }
                writer.end_array()
            }
            Document::Map(values) => {
                writer.begin_object()?;
                let mut entries = values.iter().collect::<Vec<_>>();
                entries
                    .sort_by(|(left, _), (right, _)| left.encode_utf16().cmp(right.encode_utf16()));
                for (key, value) in entries {
                    writer.key(key)?;
                    emit(writer, value, depth + 1, max_depth)?;
                }
                writer.end_object()
            }
        }
    }
    let mut writer = JsonWriter::new(max_depth, max_bytes)?;
    emit(&mut writer, value, 0, max_depth)?;
    writer.finish()
}

#[must_use]
pub fn stringify_json_bounded(
    result: &DataResult,
    max_depth: usize,
    max_bytes: usize,
) -> DataResult {
    match result.value() {
        Some(value) => match write_json_bounded(value, max_depth, max_bytes) {
            Ok(encoded) => DataResult {
                failed: false,
                message: String::new(),
                path: "$".into(),
                expected: String::new(),
                encoded,
                document: result.document.clone(),
            },
            Err(error) => DataResult::failure(error.to_string(), "$", "bounded JSON output"),
        },
        None => DataResult::failure("document value is unavailable", "$", "document value"),
    }
}

fn normalize_number(value: &str) -> Result<String, String> {
    let (negative, unsigned) = value
        .strip_prefix('-')
        .map_or((false, value), |unsigned| (true, unsigned));
    let (mantissa, exponent_text) = unsigned.split_once(['e', 'E']).unwrap_or((unsigned, "0"));
    let exponent = exponent_text
        .parse::<i64>()
        .map_err(|_| "JSON number exponent is outside the supported exact range".to_owned())?;
    if exponent.unsigned_abs() > MAXIMUM_NUMBER_EXPONENT as u64 {
        return Err("JSON number exponent exceeds the supported exact range".to_owned());
    }
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    if whole.len().saturating_add(fraction.len()) > MAXIMUM_NUMBER_DIGITS {
        return Err("JSON number exceeds the supported digit limit".to_owned());
    }
    let mut digits = format!("{whole}{fraction}");
    let first_nonzero = digits.find(|character| character != '0');
    let Some(first_nonzero) = first_nonzero else {
        return Ok("0".to_owned());
    };
    if digits.len() > MAXIMUM_NUMBER_DIGITS {
        return Err("JSON number exceeds the supported digit limit".to_owned());
    }
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
    if exponent.unsigned_abs() > MAXIMUM_NUMBER_EXPONENT as u64 {
        return Err("normalized JSON number exponent exceeds the supported exact range".into());
    }
    let coefficient = if digits.len() == 1 {
        digits
    } else {
        format!("{}.{}", &digits[..1], &digits[1..])
    };
    let exponent_sign = if exponent >= 0 { "+" } else { "" };
    Ok(format!("{sign}{coefficient}e{exponent_sign}{exponent}"))
}

fn parse_default(input: &str) -> Option<Document> {
    // Descriptor defaults are compiler-authored schema data, not caller documents. Keep their
    // independent bound explicit so changing a parser option cannot make schema defaults unsafe.
    parse_json_document(input, DESCRIPTOR_DEFAULT_DEPTH, input.len()).ok()
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
pub fn invalid_document_index() -> DataResult {
    DataResult::failure("document index is out of range", "$", "existing list item")
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
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let coefficient = format!("{whole}{fraction}");
    let exponent =
        scientific_exponent.saturating_sub(i64::try_from(fraction.len()).unwrap_or(i64::MAX));
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
