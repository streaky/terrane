use serde::Serialize;
use std::collections::{BTreeMap, VecDeque};
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicU64, Ordering},
};
use tracing::{
    Event, Subscriber,
    field::{Field, Visit},
    span::{Attributes, Id, Record},
};
use tracing_subscriber::{
    Layer, Registry,
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
};

#[derive(Clone, Debug)]
pub struct LogFieldInput {
    pub name: String,
    pub value_json: String,
    pub secret: bool,
    pub source: String,
}

#[derive(Clone, Debug)]
pub struct LogEventInput {
    pub severity: String,
    pub target: String,
    pub message: String,
    pub fields: Vec<LogFieldInput>,
    pub source: String,
    pub spans: Vec<String>,
    pub max_fields: usize,
    pub max_bytes: usize,
    pub origin: String,
}

#[derive(Clone, Copy, Debug)]
enum OverflowPolicy {
    DropNewest,
    DropOldest,
    Reject,
}

impl OverflowPolicy {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "drop-newest" => Ok(Self::DropNewest),
            "drop-oldest" => Ok(Self::DropOldest),
            "reject" => Ok(Self::Reject),
            _ => Err(format!("unknown logging overflow policy `{value}`")),
        }
    }
}

#[derive(Serialize)]
struct RecordedField {
    name: String,
    value: serde_json::Value,
    redacted: bool,
    source: String,
}

#[derive(Serialize)]
struct RecordedEvent {
    timestamp: u64,
    sequence: u64,
    severity: String,
    target: String,
    message: String,
    fields: Vec<RecordedField>,
    source: String,
    spans: Vec<String>,
    origin: String,
}

enum SinkKind {
    Memory(VecDeque<String>),
    Console,
    Failing,
}

struct LogSink {
    kind: SinkKind,
    capacity: usize,
    overflow: OverflowPolicy,
    next_timestamp: u64,
    timestamp_step: u64,
    next_sequence: u64,
    discarded_count: u64,
    reveal_secrets: bool,
}

static NEXT_SINK_ID: AtomicU64 = AtomicU64::new(1);
static SINKS: LazyLock<Mutex<BTreeMap<u64, LogSink>>> =
    LazyLock::new(|| Mutex::new(BTreeMap::new()));
static FALLBACK_DIAGNOSTICS: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

fn register_sink(sink: LogSink) -> Result<u64, String> {
    let id = NEXT_SINK_ID.fetch_add(1, Ordering::Relaxed);
    SINKS
        .lock()
        .map_err(|_| "logging sink registry is poisoned".to_owned())?
        .insert(id, sink);
    Ok(id)
}

pub fn memory_sink(
    capacity: usize,
    overflow: &str,
    start_timestamp: u64,
    timestamp_step: u64,
    reveal_secrets: bool,
) -> Result<u64, String> {
    if capacity == 0 {
        return Err("logging sink capacity must be greater than zero".to_owned());
    }
    register_sink(LogSink {
        kind: SinkKind::Memory(VecDeque::new()),
        capacity,
        overflow: OverflowPolicy::parse(overflow)?,
        next_timestamp: start_timestamp,
        timestamp_step,
        next_sequence: 0,
        discarded_count: 0,
        reveal_secrets,
    })
}

pub fn console_sink(reveal_secrets: bool) -> Result<u64, String> {
    register_sink(LogSink {
        kind: SinkKind::Console,
        capacity: usize::MAX,
        overflow: OverflowPolicy::Reject,
        next_timestamp: 0,
        timestamp_step: 1,
        next_sequence: 0,
        discarded_count: 0,
        reveal_secrets,
    })
}

pub fn failing_sink() -> Result<u64, String> {
    register_sink(LogSink {
        kind: SinkKind::Failing,
        capacity: usize::MAX,
        overflow: OverflowPolicy::Reject,
        next_timestamp: 0,
        timestamp_step: 1,
        next_sequence: 0,
        discarded_count: 0,
        reveal_secrets: false,
    })
}

pub fn emit(id: u64, input: LogEventInput) -> Result<(), String> {
    if input.fields.len() > input.max_fields {
        return Err(format!(
            "log event has {} fields but the configured limit is {}",
            input.fields.len(),
            input.max_fields
        ));
    }
    let mut sinks = SINKS
        .lock()
        .map_err(|_| "logging sink registry is poisoned".to_owned())?;
    let sink = sinks
        .get_mut(&id)
        .ok_or_else(|| "logging sink is closed or unknown".to_owned())?;
    let fields = input
        .fields
        .into_iter()
        .map(|field| {
            let redacted = field.secret && !sink.reveal_secrets;
            let value = if redacted {
                serde_json::Value::String("<redacted>".to_owned())
            } else {
                serde_json::from_str(&field.value_json)
                    .unwrap_or_else(|_| serde_json::Value::String("<invalid-log-value>".to_owned()))
            };
            RecordedField {
                name: field.name,
                value,
                redacted,
                source: field.source,
            }
        })
        .collect();
    let event = RecordedEvent {
        timestamp: sink.next_timestamp,
        sequence: sink.next_sequence,
        severity: input.severity,
        target: input.target,
        message: input.message,
        fields,
        source: input.source,
        spans: input.spans,
        origin: input.origin,
    };
    let encoded = serde_json::to_string(&event)
        .map_err(|error| format!("cannot encode structured log event: {error}"))?;
    if encoded.len() > input.max_bytes {
        return Err(format!(
            "encoded log event has {} bytes but the configured limit is {}",
            encoded.len(),
            input.max_bytes
        ));
    }
    match &mut sink.kind {
        SinkKind::Memory(events) => {
            if events.len() == sink.capacity {
                match sink.overflow {
                    OverflowPolicy::DropNewest => {
                        sink.discarded_count = sink.discarded_count.saturating_add(1);
                        return Ok(());
                    }
                    OverflowPolicy::DropOldest => {
                        events.pop_front();
                        sink.discarded_count = sink.discarded_count.saturating_add(1);
                    }
                    OverflowPolicy::Reject => {
                        return Err("logging sink capacity is exhausted".to_owned());
                    }
                }
            }
            events.push_back(encoded);
        }
        SinkKind::Console => eprintln!("{encoded}"),
        SinkKind::Failing => return Err("event rejected by logging sink".to_owned()),
    }
    sink.next_timestamp = sink.next_timestamp.saturating_add(sink.timestamp_step);
    sink.next_sequence = sink.next_sequence.saturating_add(1);
    Ok(())
}

pub fn drain_memory(id: u64) -> Result<Vec<String>, String> {
    let mut sinks = SINKS
        .lock()
        .map_err(|_| "logging sink registry is poisoned".to_owned())?;
    let sink = sinks
        .get_mut(&id)
        .ok_or_else(|| "logging sink is closed or unknown".to_owned())?;
    match &mut sink.kind {
        SinkKind::Memory(events) => Ok(events.drain(..).collect()),
        _ => Err("logging sink is not a memory sink".to_owned()),
    }
}

pub fn discarded_count(id: u64) -> u64 {
    SINKS
        .lock()
        .ok()
        .and_then(|sinks| sinks.get(&id).map(|sink| sink.discarded_count))
        .unwrap_or(0)
}

pub fn drain_fallback() -> Result<Vec<String>, String> {
    Ok(FALLBACK_DIAGNOSTICS
        .lock()
        .map_err(|_| "logging fallback diagnostics are poisoned".to_owned())?
        .drain(..)
        .collect())
}

pub fn reveals_secrets(id: u64) -> bool {
    SINKS
        .lock()
        .ok()
        .and_then(|sinks| sinks.get(&id).map(|sink| sink.reveal_secrets))
        .unwrap_or(false)
}

pub fn record_fallback(message: impl Into<String>) -> Result<(), String> {
    FALLBACK_DIAGNOSTICS
        .lock()
        .map_err(|_| "logging fallback diagnostics are poisoned".to_owned())?
        .push(message.into());
    Ok(())
}

struct DependencyVisitor {
    message: Option<String>,
    fields: Vec<LogFieldInput>,
    source: String,
    capture_message: bool,
}

impl DependencyVisitor {
    fn new(source: String, capture_message: bool) -> Self {
        Self {
            message: None,
            fields: Vec::new(),
            source,
            capture_message,
        }
    }

    fn record_value(&mut self, field: &Field, value: serde_json::Value) {
        if self.capture_message && field.name() == "message" {
            self.message = Some(match value {
                serde_json::Value::String(value) => value,
                value => value.to_string(),
            });
            return;
        }
        self.fields.push(LogFieldInput {
            name: field.name().to_owned(),
            value_json: value.to_string(),
            secret: false,
            source: self.source.clone(),
        });
    }
}

impl Visit for DependencyVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.record_value(field, serde_json::Value::Bool(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.record_value(field, serde_json::Value::Number(value.into()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.record_value(field, serde_json::Value::Number(value.into()));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        if let Some(value) = serde_json::Number::from_f64(value) {
            self.record_value(field, value.into());
        } else {
            self.fields.push(LogFieldInput {
                name: format!("{}.debug", field.name()),
                value_json: serde_json::Value::String(value.to_string()).to_string(),
                secret: false,
                source: self.source.clone(),
            });
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.record_value(field, serde_json::Value::String(value.to_owned()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if self.capture_message && field.name() == "message" {
            self.message = Some(format!("{value:?}"));
        } else {
            self.fields.push(LogFieldInput {
                name: format!("{}.debug", field.name()),
                value_json: serde_json::Value::String(format!("{value:?}")).to_string(),
                secret: false,
                source: self.source.clone(),
            });
        }
    }
}

#[derive(Clone, Debug)]
struct DependencySpan {
    name: String,
    source: String,
    fields: Vec<LogFieldInput>,
}

pub struct DependencySinkLayer {
    sink: u64,
}

fn stable_dependency_file(value: &str) -> String {
    let path = std::path::Path::new(value);
    if !path.is_absolute() {
        return value.replace('\\', "/");
    }
    let components = path
        .components()
        .filter_map(|component| match component {
            std::path::Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>();
    if let Some(source) = components.iter().position(|component| component == "src") {
        return components[source..].join("/");
    }
    components
        .last()
        .cloned()
        .unwrap_or_else(|| "<unknown-file>".to_owned())
}

fn dependency_source(module: Option<&str>, file: Option<&str>, line: Option<u32>) -> String {
    let module = module.unwrap_or("<unknown-module>");
    let file = stable_dependency_file(file.unwrap_or("<unknown-file>"));
    let line = line.unwrap_or(0);
    format!("dependency://{module}@{file}:{line}")
}

fn emit_dependency(sink: u64, input: LogEventInput) {
    if let Err(error) = emit(sink, input) {
        let _ = record_fallback(format!("dependency log event rejected: {error}"));
    }
}

impl<S> Layer<S> for DependencySinkLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, context: Context<'_, S>) {
        let metadata = attrs.metadata();
        let source = dependency_source(metadata.module_path(), metadata.file(), metadata.line());
        let mut visitor = DependencyVisitor::new(source.clone(), false);
        attrs.record(&mut visitor);
        if let Some(span) = context.span(id) {
            span.extensions_mut().insert(DependencySpan {
                name: metadata.name().to_owned(),
                source,
                fields: visitor.fields,
            });
        }
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, context: Context<'_, S>) {
        let Some(span) = context.span(id) else {
            return;
        };
        let mut extensions = span.extensions_mut();
        let Some(stored) = extensions.get_mut::<DependencySpan>() else {
            return;
        };
        let mut visitor = DependencyVisitor::new(stored.source.clone(), false);
        values.record(&mut visitor);
        for field in visitor.fields {
            if let Some(existing) = stored
                .fields
                .iter_mut()
                .find(|existing| existing.name == field.name)
            {
                *existing = field;
            } else {
                stored.fields.push(field);
            }
        }
    }

    fn on_event(&self, event: &Event<'_>, context: Context<'_, S>) {
        let metadata = event.metadata();
        let mut visitor = DependencyVisitor::new(String::new(), true);
        event.record(&mut visitor);
        let string_field = |name: &str| {
            visitor
                .fields
                .iter()
                .find(|field| field.name == name)
                .and_then(|field| serde_json::from_str::<String>(&field.value_json).ok())
        };
        let target = string_field("log.target").unwrap_or_else(|| metadata.target().to_owned());
        let module = string_field("log.module_path")
            .or_else(|| metadata.module_path().map(str::to_owned))
            .unwrap_or_else(|| "<unknown-module>".to_owned());
        let file = stable_dependency_file(
            &string_field("log.file")
                .or_else(|| metadata.file().map(str::to_owned))
                .unwrap_or_else(|| "<unknown-file>".to_owned()),
        );
        let line = visitor
            .fields
            .iter()
            .find(|field| field.name == "log.line")
            .and_then(|field| field.value_json.parse::<u64>().ok())
            .or_else(|| metadata.line().map(u64::from))
            .unwrap_or(0);
        let source = format!("dependency://{module}@{file}:{line}");
        for field in &mut visitor.fields {
            if field.name == "log.file" {
                field.value_json = serde_json::Value::String(file.clone()).to_string();
            }
            field.source.clone_from(&source);
        }
        let mut spans = Vec::new();
        if let Some(scope) = context.event_scope(event) {
            for span in scope.from_root() {
                let extensions = span.extensions();
                let Some(stored) = extensions.get::<DependencySpan>() else {
                    continue;
                };
                spans.push(stored.name.clone());
                visitor
                    .fields
                    .extend(stored.fields.iter().cloned().map(|mut field| {
                        field.name = format!("span.{}.{}", stored.name, field.name);
                        field
                    }));
            }
        }
        emit_dependency(
            self.sink,
            LogEventInput {
                severity: metadata.level().as_str().to_ascii_lowercase(),
                target,
                message: visitor.message.unwrap_or_default(),
                fields: visitor.fields,
                source,
                spans,
                max_fields: 64,
                max_bytes: 65_536,
                origin: "dependency".to_owned(),
            },
        );
    }
}

struct DependencyLogBridge {
    sink: u64,
}

struct DependencyLogVisitor {
    fields: Vec<LogFieldInput>,
    source: String,
}

impl<'kvs> log::kv::VisitSource<'kvs> for DependencyLogVisitor {
    fn visit_pair(
        &mut self,
        key: log::kv::Key<'kvs>,
        value: log::kv::Value<'kvs>,
    ) -> Result<(), log::kv::Error> {
        let (name, value) = match serde_json::to_value(&value) {
            Ok(value) => (key.as_str().to_owned(), value),
            Err(_) => (
                format!("{}.debug", key.as_str()),
                serde_json::Value::String(format!("{value:?}")),
            ),
        };
        self.fields.push(LogFieldInput {
            name,
            value_json: value.to_string(),
            secret: false,
            source: self.source.clone(),
        });
        Ok(())
    }
}

impl log::Log for DependencyLogBridge {
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let source = dependency_source(record.module_path(), record.file(), record.line());
        let mut visitor = DependencyLogVisitor {
            fields: Vec::new(),
            source: source.clone(),
        };
        if let Err(error) = record.key_values().visit(&mut visitor) {
            visitor.fields.push(LogFieldInput {
                name: "log.key-value-error".to_owned(),
                value_json: serde_json::Value::String(error.to_string()).to_string(),
                secret: false,
                source: source.clone(),
            });
        }
        emit_dependency(
            self.sink,
            LogEventInput {
                severity: record.level().as_str().to_ascii_lowercase(),
                target: record.target().to_owned(),
                message: record.args().to_string(),
                fields: visitor.fields,
                source,
                spans: Vec::new(),
                max_fields: 64,
                max_bytes: 65_536,
                origin: "dependency".to_owned(),
            },
        );
    }

    fn flush(&self) {}
}

fn validate_dependency_sink(id: u64) -> Result<(), String> {
    if SINKS
        .lock()
        .map_err(|_| "logging sink registry is poisoned".to_owned())?
        .contains_key(&id)
    {
        Ok(())
    } else {
        Err("logging sink is closed or unknown".to_owned())
    }
}

pub fn dependency_sink_layer(id: u64) -> Result<DependencySinkLayer, String> {
    validate_dependency_sink(id)?;
    Ok(DependencySinkLayer { sink: id })
}

pub fn dependency_subscriber(
    id: u64,
) -> Result<impl for<'lookup> LookupSpan<'lookup> + Subscriber + Send + Sync + 'static, String> {
    Ok(Registry::default().with(dependency_sink_layer(id)?))
}

pub fn install_dependency_log_bridge(id: u64) -> Result<(), String> {
    validate_dependency_sink(id)?;
    log::set_boxed_logger(Box::new(DependencyLogBridge { sink: id }))
        .map_err(|error| format!("cannot install dependency log bridge: {error}"))?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}

pub fn install_dependency_bridge(id: u64) -> Result<(), String> {
    let subscriber = dependency_subscriber(id)?;
    install_dependency_log_bridge(id)?;
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|error| format!("cannot install dependency tracing bridge: {error}"))
}
