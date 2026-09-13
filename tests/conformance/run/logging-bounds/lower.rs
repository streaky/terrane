// Generated deterministically by Terrane <version>.
// Runtime support: platform_data_base.rs, platform_documents.rs, platform_capability_types.rs, platform_result_type.rs, platform_capability_base.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-document-support, terrane-platform-support
// Source: case.trn
// Namespace: logging-bounds
fn main() {
    let empty: terrane_collection_support::List<LogField> = terrane_collection_support::List::<
        LogField,
    >::new(vec![]);
    let dropping_sink: LogSinkResult = memory_sink(
        terrane_int_support::Int::from(1_i128),
        String::from("drop-oldest"),
        terrane_int_support::Int::from(10_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    let dropping: Logger = default_logger(dropping_sink.value);
    let first: LogOutcome = emit_at(
        dropping.clone(),
        info_level(),
        String::from("first").clone(),
        "case.trn:12:13".to_owned(),
        empty.clone(),
    );
    let second: LogOutcome = emit_at(
        dropping.clone(),
        info_level(),
        String::from("second").clone(),
        "case.trn:13:14".to_owned(),
        empty.clone(),
    );
    let dropped_records: terrane_collection_support::List<String> = drain_memory(
        dropping.clone(),
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&first.failed),
        terrane_scalar_support::scalar_text(&second.failed),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(dropped_records
        .length())), terrane_scalar_support::scalar_text(&discarded_count(dropping
        .clone()))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(dropped_records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:16:13-16:31 */)), 0 /* terrane-site: case.trn:16:13-16:31 */).contains(&String::from("\"message\":\"second\""))),
        terrane_scalar_support::scalar_text(&__terrane_raised(dropped_records
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:16:66-16:84 */)), 1 /* terrane-site: case.trn:16:66-16:84 */).contains(&String::from("\"message\":\"first\"")))
    );
    let rejecting_sink: LogSinkResult = memory_sink(
        terrane_int_support::Int::from(1_i128),
        String::from("reject"),
        terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    let rejecting: Logger = default_logger(rejecting_sink.value);
    let accepted: LogOutcome = emit_at(
        rejecting.clone(),
        info_level(),
        String::from("accepted").clone(),
        "case.trn:20:16".to_owned(),
        empty.clone(),
    );
    let rejected: LogOutcome = emit_at(
        rejecting.clone(),
        info_level(),
        String::from("rejected").clone(),
        "case.trn:21:16".to_owned(),
        empty.clone(),
    );
    let fallback: terrane_collection_support::List<String> = drain_fallback();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&accepted.failed),
        terrane_scalar_support::scalar_text(&rejected.failed),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fallback
        .length()))
    );
    let one_field: terrane_collection_support::List<LogField> = terrane_collection_support::List::<
        LogField,
    >::new(
        vec![
            field_at(String::from("value").clone(), log_text(String::from("bounded")),
            false, "case.trn:25:24".to_owned())
        ],
    );
    let strict_fields: Logger = make_logger(
        memory_sink(
                terrane_int_support::Int::from(2_i128),
                String::from("reject"),
                terrane_int_support::Int::from(30_i128),
                terrane_int_support::Int::from(1_i128),
                false,
            )
            .value,
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("bounded.fields"),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(4096_i128),
        ),
    );
    let too_many: LogOutcome = emit_at(
        strict_fields.clone(),
        info_level(),
        String::from("too-many").clone(),
        "case.trn:27:16".to_owned(),
        one_field.clone(),
    );
    let strict_bytes: Logger = make_logger(
        memory_sink(
                terrane_int_support::Int::from(2_i128),
                String::from("reject"),
                terrane_int_support::Int::from(40_i128),
                terrane_int_support::Int::from(1_i128),
                false,
            )
            .value,
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("bounded.bytes"),
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(32_i128),
        ),
    );
    let too_large: LogOutcome = emit_at(
        strict_bytes.clone(),
        info_level(),
        String::from("too-large").clone(),
        "case.trn:29:17".to_owned(),
        empty.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&too_many.failed),
        terrane_scalar_support::scalar_text(&too_large.failed)
    );
    let invalid_sink: LogSinkResult = memory_sink(
        terrane_int_support::Int::from(-1_i128),
        String::from("reject"),
        terrane_int_support::Int::from(0_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&invalid_sink.failed));
    let negative_sink: LogSinkResult = memory_sink(
        terrane_int_support::Int::from(1_i128),
        String::from("reject"),
        terrane_int_support::Int::from(50_i128),
        terrane_int_support::Int::from(1_i128),
        false,
    );
    let negative_limit: Logger = make_logger(
        negative_sink.value,
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("bounded.negative"),
            terrane_int_support::Int::from(-1_i128),
            terrane_int_support::Int::from(4096_i128),
        ),
    );
    let invalid_limit: LogOutcome = emit_at(
        negative_limit.clone(),
        info_level(),
        String::from("invalid-limit").clone(),
        "case.trn:37:21".to_owned(),
        empty.clone(),
    );
    println!("{}", terrane_scalar_support::scalar_text(&invalid_limit.failed));
}
// Source: core/logging.trn
// Namespace: core/logging
#[derive(Clone)]
pub struct LogLevel {
    pub name: String,
    pub rank: terrane_int_support::Int,
}
impl LogLevel {
    pub fn terrane_construct(name: String, rank: terrane_int_support::Int) -> Self {
        let mut value = Self {
            name: String::from("info"),
            rank: terrane_int_support::Int::from(30_i128),
        };
        value.construct(name, rank);
        value
    }
    pub fn construct(&mut self, name: String, rank: terrane_int_support::Int) {
        self.name = name;
        self.rank = rank.clone();
    }
}
pub fn trace_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("trace"),
        terrane_int_support::Int::from(10_i128),
    );
}
pub fn debug_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("debug"),
        terrane_int_support::Int::from(20_i128),
    );
}
pub fn info_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("info"),
        terrane_int_support::Int::from(30_i128),
    );
}
pub fn warning_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("warning"),
        terrane_int_support::Int::from(40_i128),
    );
}
pub fn error_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("error"),
        terrane_int_support::Int::from(50_i128),
    );
}
pub fn critical_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("critical"),
        terrane_int_support::Int::from(60_i128),
    );
}
pub trait LogValueProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn LogValueProtocol>;
    fn separate_box(&self) -> Box<dyn LogValueProtocol>;
    fn render(&self) -> DocumentValue;
}
impl Clone for Box<dyn LogValueProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct LogValue(Box<dyn LogValueProtocol>);
impl Clone for LogValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl LogValue {
    pub fn render(&self) -> DocumentValue {
        self.0.render()
    }
}
#[derive(Clone)]
pub struct DocumentLogValue {
    pub value: DocumentValue,
}
impl DocumentLogValue {
    pub fn terrane_construct(input: DocumentValue) -> Self {
        let mut value = Self {
            value: make_document_none(),
        };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: DocumentValue) {
        self.value = input.clone();
    }
    pub fn render(&self) -> DocumentValue {
        return self.value.to_document();
    }
}
impl LogValueProtocol for DocumentLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        DocumentLogValue::render(&*self)
    }
}
impl From<DocumentLogValue> for LogValue {
    fn from(value: DocumentLogValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TextLogValue {
    pub value: String,
}
impl TextLogValue {
    pub fn terrane_construct(input: String) -> Self {
        let mut value = Self { value: String::from("") };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: String) {
        self.value = input;
    }
    pub fn render(&self) -> DocumentValue {
        return make_document_string(self.value.clone());
    }
}
impl LogValueProtocol for TextLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        TextLogValue::render(&*self)
    }
}
impl From<TextLogValue> for LogValue {
    fn from(value: TextLogValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct ErrorLogValue {
    pub rendered: String,
}
impl ErrorLogValue {
    pub fn terrane_construct(rendered: String) -> Self {
        let mut value = Self { rendered: String::from("") };
        value.construct(rendered);
        value
    }
    pub fn construct(&mut self, rendered: String) {
        self.rendered = rendered;
    }
    pub fn render(&self) -> DocumentValue {
        return make_document_string(self.rendered.clone());
    }
}
impl LogValueProtocol for ErrorLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        ErrorLogValue::render(&*self)
    }
}
impl From<ErrorLogValue> for LogValue {
    fn from(value: ErrorLogValue) -> Self {
        Self(Box::new(value))
    }
}
pub fn log_document(value: DocumentValue) -> LogValue {
    return <LogValue>::from(DocumentLogValue::terrane_construct(value.clone()));
}
pub fn log_text(value: String) -> LogValue {
    return <LogValue>::from(TextLogValue::terrane_construct(value));
}
pub fn log_error(value: TerraneError) -> LogValue {
    return <LogValue>::from(ErrorLogValue::terrane_construct(value.render()));
}
#[derive(Clone)]
pub struct LogField {
    pub name: String,
    pub value: LogValue,
    pub secret: bool,
    pub source: String,
}
impl LogField {
    pub fn terrane_construct(
        name: String,
        input: LogValue,
        secret: bool,
        source: String,
    ) -> Self {
        let mut value = Self {
            name: String::from(""),
            value: <LogValue>::from(TextLogValue::terrane_construct(String::from(""))),
            secret: false,
            source: String::from(""),
        };
        value.construct(name, input, secret, source);
        value
    }
    pub fn construct(
        &mut self,
        name: String,
        input: LogValue,
        secret: bool,
        source: String,
    ) {
        self.name = name;
        self.value = input.clone();
        self.secret = secret;
        self.source = source;
    }
}
pub fn field_at(
    name: String,
    value: LogValue,
    secret: bool,
    source: String,
) -> LogField {
    return LogField::terrane_construct(name, value.clone(), secret, source);
}
pub fn field(name: String, value: LogValue) -> LogField {
    return field_at(
        name,
        value.clone(),
        false,
        String::from("builtin://core/logging.trn"),
    );
}
pub fn secret_field(name: String, value: LogValue) -> LogField {
    return field_at(
        name,
        value.clone(),
        true,
        String::from("builtin://core/logging.trn"),
    );
}
pub fn empty_log_fields() -> terrane_collection_support::List<LogField> {
    return terrane_collection_support::List::new(Vec::new());
}
#[derive(Clone)]
pub struct LogSink {
    pub handle: TerranePlatformCapability,
}
impl LogSink {
    pub fn terrane_construct(handle: TerranePlatformCapability) -> Self {
        let mut value = Self {
            handle: terrane_platform_support::logging_no_sink(),
        };
        value.construct(handle);
        value
    }
    pub fn construct(&mut self, handle: TerranePlatformCapability) {
        self.handle = handle;
    }
}
#[derive(Clone)]
pub struct LogSinkResult {
    pub failed: bool,
    pub message: String,
    pub value: LogSink,
}
impl LogSinkResult {
    pub fn terrane_construct(failed: bool, message: String, sink: LogSink) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: LogSink::terrane_construct(
                terrane_platform_support::logging_no_sink(),
            ),
        };
        value.construct(failed, message, sink);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, sink: LogSink) {
        self.failed = failed;
        self.message = message;
        self.value = sink.clone();
    }
}
pub fn memory_sink(
    capacity: terrane_int_support::Int,
    overflow: String,
    start: terrane_int_support::Int,
    step: terrane_int_support::Int,
    reveal: bool,
) -> LogSinkResult {
    let raw: TerranePlatformResult = terrane_platform_support::logging_memory_sink(
        terrane_int_support::checked_coerce::<i128>(&capacity.clone()),
        &overflow,
        terrane_int_support::checked_coerce::<i128>(&start.clone()),
        terrane_int_support::checked_coerce::<i128>(&step.clone()),
        reveal,
    );
    let sink: LogSink = LogSink::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return LogSinkResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        sink,
    );
}
pub fn console_sink(reveal: bool) -> LogSinkResult {
    let raw: TerranePlatformResult = terrane_platform_support::logging_console_sink(
        reveal,
    );
    let sink: LogSink = LogSink::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return LogSinkResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        sink,
    );
}
pub fn failing_sink() -> LogSinkResult {
    let raw: TerranePlatformResult = terrane_platform_support::logging_failing_sink();
    let sink: LogSink = LogSink::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return LogSinkResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        sink,
    );
}
#[derive(Clone)]
pub struct LoggerOptions {
    pub minimum: LogLevel,
    pub target_prefix: String,
    pub target: String,
    pub max_fields: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
}
impl LoggerOptions {
    pub fn terrane_construct(
        minimum: LogLevel,
        target_prefix: String,
        target: String,
        max_fields: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            minimum: info_level(),
            target_prefix: String::from(""),
            target: String::from("application"),
            max_fields: terrane_int_support::Int::from(64_i128),
            max_bytes: terrane_int_support::Int::from(65536_i128),
        };
        value.construct(minimum, target_prefix, target, max_fields, max_bytes);
        value
    }
    pub fn construct(
        &mut self,
        minimum: LogLevel,
        target_prefix: String,
        target: String,
        max_fields: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) {
        self.minimum = minimum.clone();
        self.target_prefix = target_prefix;
        self.target = target;
        self.max_fields = max_fields.clone();
        self.max_bytes = max_bytes.clone();
    }
}
pub fn make_logger_options(
    minimum: LogLevel,
    target_prefix: String,
    target: String,
    max_fields: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
) -> LoggerOptions {
    return LoggerOptions::terrane_construct(
        minimum.clone(),
        target_prefix,
        target,
        max_fields.clone(),
        max_bytes.clone(),
    );
}
#[derive(Clone)]
pub struct LogContext {
    pub fields: terrane_collection_support::List<LogField>,
    pub spans: terrane_collection_support::List<String>,
}
impl LogContext {
    pub fn terrane_construct() -> Self {
        Self {
            fields: terrane_collection_support::List::new(Vec::new()),
            spans: terrane_collection_support::List::<String>::new(Vec::new()),
        }
    }
    pub fn append_field(&mut self, item: LogField) {
        let mut updated: terrane_collection_support::List<LogField> = terrane_collection_support::List::new(
            Vec::new(),
        );
        let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
            &self.fields,
        );
        {
            let __terrane_list_append_0 = updated.make_unique();
            loop {
                let existing = match __terrane_iterator_0.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                __terrane_list_append_0.push(existing);
            }
        }
        updated.append(item.clone());
        self.fields = updated.clone();
    }
    pub fn append_span(&mut self, item: String) {
        let mut updated: terrane_collection_support::List<String> = terrane_collection_support::List::<
            String,
        >::new(Vec::new());
        let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
            &self.spans,
        );
        {
            let __terrane_list_append_1 = updated.make_unique();
            loop {
                let existing = match __terrane_iterator_1.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                __terrane_list_append_1.push(existing);
            }
        }
        updated.append(item);
        self.spans = updated.clone();
    }
    pub fn adding_field(&self, addition: LogField) -> LogContext {
        let mut value: LogContext = LogContext::terrane_construct();
        let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
            &self.fields,
        );
        loop {
            let existing = match __terrane_iterator_2.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_field(existing);
        }
        value.append_field(addition.clone());
        let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
            &self.spans,
        );
        loop {
            let existing = match __terrane_iterator_3.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_span(existing);
        }
        return value.clone();
    }
    pub fn adding_span(&self, span: String) -> LogContext {
        let mut value: LogContext = LogContext::terrane_construct();
        let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
            &self.fields,
        );
        loop {
            let existing = match __terrane_iterator_4.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_field(existing);
        }
        let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
            &self.spans,
        );
        loop {
            let existing = match __terrane_iterator_5.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_span(existing);
        }
        value.append_span(span);
        return value.clone();
    }
}
#[derive(Clone)]
pub struct Logger {
    pub sink: LogSink,
    pub options: LoggerOptions,
    pub context: LogContext,
}
impl Logger {
    pub fn terrane_construct(
        sink: LogSink,
        options: LoggerOptions,
        context: LogContext,
    ) -> Self {
        let mut value = Self {
            sink: LogSink::terrane_construct(
                terrane_platform_support::logging_no_sink(),
            ),
            options: LoggerOptions::terrane_construct(
                info_level(),
                String::from(""),
                String::from("application"),
                terrane_int_support::Int::from(64_i128),
                terrane_int_support::Int::from(65536_i128),
            ),
            context: LogContext::terrane_construct(),
        };
        value.construct(sink, options, context);
        value
    }
    pub fn construct(
        &mut self,
        sink: LogSink,
        options: LoggerOptions,
        context: LogContext,
    ) {
        self.sink = sink.clone();
        self.options = options.clone();
        self.context = context.clone();
    }
}
pub fn make_logger(sink: LogSink, options: LoggerOptions) -> Logger {
    return Logger::terrane_construct(
        sink.clone(),
        options.clone(),
        LogContext::terrane_construct(),
    );
}
pub fn default_logger(sink: LogSink) -> Logger {
    return make_logger(
        sink.clone(),
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("application"),
            terrane_int_support::Int::from(64_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
}
pub fn named_logger(sink: LogSink, target: String) -> Logger {
    return make_logger(
        sink.clone(),
        make_logger_options(
            info_level(),
            target.clone(),
            target,
            terrane_int_support::Int::from(64_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
}
pub fn with_field(value: Logger, addition: LogField) -> Logger {
    return Logger::terrane_construct(
        value.sink,
        value.options,
        value.context.adding_field(addition.clone()),
    );
}
pub fn with_span(value: Logger, span: String) -> Logger {
    return Logger::terrane_construct(
        value.sink,
        value.options,
        value.context.adding_span(span),
    );
}
#[derive(Clone)]
pub struct LogOutcome {
    pub filtered: bool,
    pub failed: bool,
    pub message: String,
}
impl LogOutcome {
    pub fn terrane_construct(filtered: bool, failed: bool, message: String) -> Self {
        let mut value = Self {
            filtered: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(filtered, failed, message);
        value
    }
    pub fn construct(&mut self, filtered: bool, failed: bool, message: String) {
        self.filtered = filtered;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct LogEvent {
    pub level: LogLevel,
    pub message: String,
    pub fields: terrane_collection_support::List<LogField>,
    pub source: String,
}
impl LogEvent {
    pub fn terrane_construct(
        level: LogLevel,
        message: String,
        source: String,
        fields: terrane_collection_support::List<LogField>,
    ) -> Self {
        let mut value = Self {
            level: info_level(),
            message: String::from(""),
            fields: terrane_collection_support::List::new(Vec::new()),
            source: String::from(""),
        };
        value.construct(level, message, source, fields);
        value
    }
    pub fn construct(
        &mut self,
        level: LogLevel,
        message: String,
        source: String,
        fields: terrane_collection_support::List<LogField>,
    ) {
        self.level = level.clone();
        self.message = message;
        self.fields = fields.clone();
        self.source = source;
    }
}
pub fn make_event_at(
    level: LogLevel,
    message: String,
    source: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogEvent {
    return LogEvent::terrane_construct(level.clone(), message, source, fields.clone());
}
pub fn make_event(
    level: LogLevel,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogEvent {
    return make_event_at(
        level.clone(),
        message,
        String::from("builtin://core/logging.trn"),
        fields.clone(),
    );
}
pub fn emit_at(
    v: Logger,
    l: LogLevel,
    m: String,
    source: String,
    f: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    if l.rank.clone() < v.options.minimum.rank.clone() {
        return LogOutcome::terrane_construct(true, false, String::from(""));
    }
    if v.options.target_prefix != String::from("")
        && !v.options.target.starts_with(&v.options.target_prefix)
    {
        return LogOutcome::terrane_construct(true, false, String::from(""));
    }
    let mut combined: terrane_collection_support::List<LogField> = terrane_collection_support::List::new(
        Vec::new(),
    );
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &v.context.fields,
    );
    {
        let __terrane_list_append_2 = combined.make_unique();
        loop {
            let item = match __terrane_iterator_6.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_2.push(item);
        }
    }
    let mut __terrane_iterator_7 = terrane_collection_support::Iterable::terrane_iterator(
        &f,
    );
    {
        let __terrane_list_append_3 = combined.make_unique();
        loop {
            let item = match __terrane_iterator_7.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_3.push(item);
        }
    }
    let options: LoggerOptions = v.options;
    let raw: TerranePlatformResult = {
        let sink = v.sink.handle;
        let severity = l.name;
        let target = options.target;
        let message = m;
        let raw_fields = combined;
        let source = source;
        let spans = v.context.spans;
        let max_fields_value = options.max_fields.clone();
        let max_bytes_value = options.max_bytes.clone();
        match (
            terrane_collection_support::index_from_int(&max_fields_value),
            terrane_collection_support::index_from_int(&max_bytes_value),
        ) {
            (Ok(max_fields), Ok(max_bytes)) => {
                let reveal_secrets = terrane_platform_support::logging_reveals_secrets(
                    &sink,
                );
                let fields = raw_fields
                    .into_vec()
                    .into_iter()
                    .map(|field| {
                        let value_json = if field.secret && !reveal_secrets {
                            "null".to_owned()
                        } else {
                            field.value.render().encoded.clone()
                        };
                        terrane_platform_support::LogFieldInput {
                            name: field.name,
                            value_json,
                            secret: field.secret,
                            source: field.source,
                        }
                    })
                    .collect::<Vec<_>>();
                terrane_platform_support::logging_emit(
                    &sink,
                    terrane_platform_support::LogEventInput {
                        severity,
                        target,
                        message,
                        fields,
                        source,
                        spans: spans.into_vec(),
                        max_fields,
                        max_bytes,
                        origin: "terrane".to_owned(),
                    },
                )
            }
            (Err(_), _) => {
                terrane_platform_support::ResultValue::error(
                    "logging field limit must be non-negative and fit this target",
                )
            }
            (_, Err(_)) => {
                terrane_platform_support::ResultValue::error(
                    "logging byte limit must be non-negative and fit this target",
                )
            }
        }
    };
    return LogOutcome::terrane_construct(
        false,
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
    );
}
pub fn emit(
    value: Logger,
    level: LogLevel,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        level.clone(),
        message,
        String::from("builtin://core/logging.trn"),
        fields.clone(),
    );
}
pub fn debug(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        debug_level().clone(),
        message.clone(),
        "core/logging.trn:246:12".to_owned(),
        fields.clone(),
    );
}
pub fn write_event(value: Logger, event: LogEvent) -> LogOutcome {
    return emit_at(
        value.clone(),
        event.level,
        event.message.clone(),
        event.source.clone(),
        event.fields,
    );
}
pub fn install_dependency_bridge(value: Logger) -> LogOutcome {
    let raw: TerranePlatformResult = terrane_platform_support::logging_install_dependency_bridge(
        &value.sink.handle,
    );
    return LogOutcome::terrane_construct(
        false,
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
    );
}
pub fn info(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        info_level().clone(),
        message.clone(),
        "core/logging.trn:256:12".to_owned(),
        fields.clone(),
    );
}
pub fn warning(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        warning_level().clone(),
        message.clone(),
        "core/logging.trn:259:12".to_owned(),
        fields.clone(),
    );
}
pub fn error(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        error_level().clone(),
        message.clone(),
        "core/logging.trn:262:12".to_owned(),
        fields.clone(),
    );
}
pub fn discarded_count(value: Logger) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        i128::from(terrane_platform_support::logging_discarded_count(&value.sink.handle)),
    );
}
pub fn drain_memory(value: Logger) -> terrane_collection_support::List<String> {
    return terrane_collection_support::List::new(
        terrane_platform_result_entries(
            &terrane_platform_support::logging_drain(&value.sink.handle),
        ),
    );
}
pub fn drain_fallback() -> terrane_collection_support::List<String> {
    return terrane_collection_support::List::new(
        terrane_platform_result_entries(
            &terrane_platform_support::logging_drain_fallback(),
        ),
    );
}
// Source: core/documents.trn
// Namespace: core/documents
#[derive(Clone)]
pub struct DocumentInteger {
    pub text: String,
}
impl DocumentInteger {
    pub fn terrane_construct(text: String) -> Self {
        let mut value = Self { text: String::from("0") };
        value.construct(text);
        value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
}
#[derive(Clone)]
pub struct DocumentDecimal {
    pub coefficient: String,
    pub exponent: terrane_int_support::Int,
    pub text: String,
}
impl DocumentDecimal {
    pub fn terrane_construct(
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) -> Self {
        let mut value = Self {
            coefficient: String::from("0"),
            exponent: terrane_int_support::Int::from(0_i128),
            text: String::from("0"),
        };
        value.construct(coefficient, exponent, text);
        value
    }
    pub fn construct(
        &mut self,
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) {
        self.coefficient = coefficient;
        self.exponent = exponent.clone();
        self.text = text;
    }
}
pub trait SerializableProtocol {
    fn clone_box(&self) -> Box<dyn SerializableProtocol>;
    fn separate_box(&self) -> Box<dyn SerializableProtocol>;
    fn to_document(&self) -> DocumentValue;
}
impl Clone for Box<dyn SerializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Serializable(Box<dyn SerializableProtocol>);
impl Clone for Serializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Serializable {
    pub fn to_document(&self) -> DocumentValue {
        self.0.to_document()
    }
}
pub trait DeserializableProtocol {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol>;
    fn separate_box(&self) -> Box<dyn DeserializableProtocol>;
    fn from_document(&self, value: DocumentValue) -> DocumentResult;
}
impl Clone for Box<dyn DeserializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Deserializable(Box<dyn DeserializableProtocol>);
impl Clone for Deserializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Deserializable {
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        self.0.from_document(value)
    }
}
pub trait DocumentDecodableProtocol {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol>;
}
impl Clone for Box<dyn DocumentDecodableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[allow(
    dead_code,
    reason = "marker interface storage is materialized only when a value is erased to that marker"
)]
pub struct DocumentDecodable(Box<dyn DocumentDecodableProtocol>);
impl Clone for DocumentDecodable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentDecodable {}
pub trait DocumentValidatableProtocol {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn validate_document(&self) -> Option<String>;
}
impl Clone for Box<dyn DocumentValidatableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct DocumentValidatable(Box<dyn DocumentValidatableProtocol>);
impl Clone for DocumentValidatable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentValidatable {
    pub fn validate_document(&self) -> Option<String> {
        self.0.validate_document()
    }
}
#[derive(Clone)]
pub struct DocumentValue {
    pub raw: terrane_document_support::DataResult,
    pub encoded: String,
    pub kind: String,
    pub scalar: String,
    pub integer: DocumentInteger,
    pub decimal: DocumentDecimal,
}
impl DocumentValue {
    pub fn terrane_construct(raw: terrane_document_support::DataResult) -> Self {
        let mut value = Self {
            raw: terrane_empty_document(),
            encoded: String::from(""),
            kind: String::from("invalid"),
            scalar: String::from(""),
            integer: DocumentInteger::terrane_construct(String::from("0")),
            decimal: DocumentDecimal::terrane_construct(
                String::from("0"),
                terrane_int_support::Int::from(0_i128),
                String::from("0"),
            ),
        };
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: terrane_document_support::DataResult) {
        self.kind = terrane_document_kind(&raw);
        self.scalar = terrane_document_text(&raw);
        self.encoded = terrane_data_encoded(&raw);
        if self.kind == String::from("integer") {
            self.integer = DocumentInteger::terrane_construct(self.scalar.clone());
        }
        if self.kind == String::from("decimal") {
            self.decimal = DocumentDecimal::terrane_construct(
                terrane_document_coefficient(&raw),
                terrane_document_exponent(&raw),
                self.scalar.clone(),
            );
        }
        self.raw = raw;
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_document_length(&self.raw);
    }
    pub fn to_document(&self) -> DocumentValue {
        return self.clone();
    }
    pub fn item(&self, index: terrane_int_support::Int) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_item(
            &self.raw,
            index.clone(),
        );
        return make_document_result(raw);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.raw, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_field(
            &self.raw,
            name,
        );
        return make_document_result(raw);
    }
}
impl SerializableProtocol for DocumentValue {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        DocumentValue::to_document(&*self)
    }
}
impl From<DocumentValue> for Serializable {
    fn from(value: DocumentValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct DocumentResult {
    pub failed: bool,
    pub message: String,
    pub path: String,
    pub expected: String,
    pub value: DocumentValue,
}
impl DocumentResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            path: String::from("$"),
            expected: String::from(""),
            value: DocumentValue::terrane_construct(terrane_empty_document()),
        };
        value.construct(failed, message, path, expected, raw);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) {
        self.failed = failed;
        self.message = message;
        self.path = path;
        self.expected = expected;
        self.value = DocumentValue::terrane_construct(raw);
    }
}
#[derive(Clone)]
pub struct DocumentMapping {
    pub descriptor_name: String,
    pub expected_kind: String,
    pub field_names: terrane_collection_support::List<String>,
    pub optional_fields: terrane_collection_support::List<String>,
    pub default_fields: terrane_collection_support::List<String>,
    pub default_values: terrane_collection_support::List<String>,
    pub allow_unknown: bool,
}
impl DocumentMapping {
    pub fn terrane_construct(
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) -> Self {
        let mut value = Self {
            descriptor_name: String::from("document-value"),
            expected_kind: String::from("map"),
            field_names: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            optional_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            allow_unknown: false,
        };
        value.construct(descriptor_name, expected_kind, allow_unknown);
        value
    }
    pub fn construct(
        &mut self,
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) {
        self.descriptor_name = descriptor_name;
        self.expected_kind = expected_kind;
        self.allow_unknown = allow_unknown;
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return decode_document(value.clone(), self.clone());
    }
}
impl DeserializableProtocol for DocumentMapping {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn from_document(&self, value: DocumentValue) -> DocumentResult {
        DocumentMapping::from_document(&*self, value)
    }
}
impl From<DocumentMapping> for Deserializable {
    fn from(value: DocumentMapping) -> Self {
        Self(Box::new(value))
    }
}
pub fn serialize_document(value: Serializable) -> DocumentValue {
    return value.to_document();
}
pub fn deserialize_document(
    value: DocumentValue,
    destination: Deserializable,
) -> DocumentResult {
    return destination.from_document(value.clone());
}
pub fn make_document_result(
    raw: terrane_document_support::DataResult,
) -> DocumentResult {
    return DocumentResult::terrane_construct(
        terrane_data_failed(&raw),
        terrane_data_message(&raw),
        terrane_data_path(&raw),
        terrane_data_expected(&raw),
        raw,
    );
}
pub fn make_document_none() -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_none());
}
pub fn make_document_bool(value: bool) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_bool(value));
}
pub fn make_document_string(value: String) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_string(value));
}
pub fn make_document_integer(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_integer(value));
}
pub fn make_document_decimal(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_decimal(value));
}
#[derive(Clone)]
pub struct DocumentMapEntries {
    pub raw: terrane_document_support::DataResult,
}
impl DocumentMapEntries {
    pub fn terrane_construct() -> Self {
        let mut value = Self {
            raw: terrane_make_document_map(),
        };
        value.construct();
        value
    }
    pub fn construct(&mut self) {
        self.raw = terrane_make_document_map();
    }
    pub fn append(&mut self, key: String, value: DocumentValue) {
        self.raw = terrane_document_map_insert(&self.raw, key, &value.raw);
    }
}
pub fn append_document_map_entry(
    mut entries: DocumentMapEntries,
    key: String,
    value: DocumentValue,
) -> DocumentMapEntries {
    entries.append(key, value.clone());
    return entries.clone();
}
pub fn make_document_list(
    values: terrane_collection_support::List<DocumentValue>,
) -> DocumentResult {
    let mut raw: terrane_document_support::DataResult = terrane_make_document_list();
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(values.length()))
    {
        raw = terrane_document_list_append(
            &raw,
            &__terrane_raised(
                    values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                2 /* terrane-site: core/documents.trn:140:47-140:60 */,
                            ),
                        ),
                    2 /* terrane-site: core/documents.trn:140:47-140:60 */,
                )
                .raw,
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return make_document_result(raw);
}
pub fn make_document_map(entries: DocumentMapEntries) -> DocumentResult {
    return make_document_result(entries.raw);
}
pub fn mapping_required_fields(
    mapping: DocumentMapping,
) -> terrane_collection_support::List<String> {
    let fields: terrane_collection_support::List<String> = mapping.field_names;
    let optional_fields: terrane_collection_support::List<String> = mapping
        .optional_fields;
    let default_fields: terrane_collection_support::List<String> = mapping
        .default_fields;
    let mut required: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = required.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(fields.length()),
            )
        {
            let field: String = __terrane_raised(
                fields
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            3 /* terrane-site: core/documents.trn:153:17-153:30 */,
                        ),
                    ),
                3 /* terrane-site: core/documents.trn:153:17-153:30 */,
            );
            let mut optional: bool = false;
            let mut optional_index: terrane_int_support::Int = terrane_int_support::Int::from(
                0_i128,
            );
            while optional_index.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(optional_fields.length()),
                )
            {
                if __terrane_raised(
                    optional_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &optional_index.clone(),
                                ),
                                4 /* terrane-site: core/documents.trn:157:16-157:47 */,
                            ),
                        ),
                    4 /* terrane-site: core/documents.trn:157:16-157:47 */,
                ) == field
                {
                    optional = true;
                }
                optional_index = optional_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            let mut defaulted: bool = false;
            let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
                0_i128,
            );
            while default_index.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(default_fields.length()),
                )
            {
                if __terrane_raised(
                    default_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                ),
                                5 /* terrane-site: core/documents.trn:163:16-163:45 */,
                            ),
                        ),
                    5 /* terrane-site: core/documents.trn:163:16-163:45 */,
                ) == field
                {
                    defaulted = true;
                }
                default_index = default_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            if field != String::from("") && !optional && !defaulted {
                __terrane_list_append_0.push(field);
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return required.clone();
}
pub fn decode_document(
    value: DocumentValue,
    mapping: DocumentMapping,
) -> DocumentResult {
    let required: terrane_collection_support::List<String> = mapping_required_fields(
        mapping.clone(),
    );
    let mut declared_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut field_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    {
        let __terrane_list_append_1 = declared_fields.make_unique();
        while field_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.field_names.length()),
            )
        {
            if __terrane_raised(
                mapping
                    .field_names
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &field_index.clone(),
                            ),
                            6 /* terrane-site: core/documents.trn:176:12-176:44 */,
                        ),
                    ),
                6 /* terrane-site: core/documents.trn:176:12-176:44 */,
            ) != String::from("")
            {
                __terrane_list_append_1
                    .push(
                        __terrane_raised(
                            mapping
                                .field_names
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &field_index.clone(),
                                        ),
                                        7 /* terrane-site: core/documents.trn:177:37-177:69 */,
                                    ),
                                ),
                            7 /* terrane-site: core/documents.trn:177:37-177:69 */,
                        ),
                    );
            }
            field_index = field_index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut default_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_values: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while default_index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(mapping.default_fields.length()),
        )
        && default_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.default_values.length()),
            )
    {
        if __terrane_raised(
            mapping
                .default_fields
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &default_index.clone(),
                        ),
                        8 /* terrane-site: core/documents.trn:183:12-183:49 */,
                    ),
                ),
            8 /* terrane-site: core/documents.trn:183:12-183:49 */,
        ) != String::from("")
        {
            default_fields
                .append(
                    __terrane_raised(
                        mapping
                            .default_fields
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    9 /* terrane-site: core/documents.trn:184:36-184:73 */,
                                ),
                            ),
                        9 /* terrane-site: core/documents.trn:184:36-184:73 */,
                    ),
                );
            default_values
                .append(
                    __terrane_raised(
                        mapping
                            .default_values
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    10 /* terrane-site: core/documents.trn:185:36-185:73 */,
                                ),
                            ),
                        10 /* terrane-site: core/documents.trn:185:36-185:73 */,
                    ),
                );
        }
        default_index = default_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let raw: terrane_document_support::DataResult = terrane_validate_mapping(
        &value.raw,
        mapping.expected_kind,
        required,
        declared_fields,
        default_fields,
        default_values,
        mapping.allow_unknown,
    );
    let mut result: DocumentResult = make_document_result(raw);
    if result.failed {
        result.expected = mapping.descriptor_name.clone();
    }
    return result.clone();
}
