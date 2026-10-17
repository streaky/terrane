// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: structured-logging
#[derive(Clone)]
pub struct RequestData {
    pub token: String,
}
impl RequestData {
    pub fn terrane_construct(token: String) -> Self {
        let mut __terrane_constructed_value = Self { token: String::from("") };
        __terrane_constructed_value.construct(token);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, token: String) {
        self.token = token;
    }
}
#[derive(Clone)]
pub struct CountedValue {
    pub value: String,
}
impl CountedValue {
    pub fn terrane_construct(input: String) -> Self {
        let mut __terrane_constructed_value = Self { value: String::from("") };
        __terrane_constructed_value.construct(input);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, input: String) {
        self.value = input;
    }
    pub fn render(&self) -> DocumentValue {
        println!(
            "{}{}", String::from("rendered:"), terrane_scalar_support::scalar_text(&self
            .value)
        );
        return make_document_string(self.value.clone());
    }
}
impl LogValueProtocol for CountedValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        CountedValue::render(&*self)
    }
}
impl From<CountedValue> for LogValue {
    fn from(value: CountedValue) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    __terrane_run(async move {
        let sink_result: LogSinkResult;
        let mut writer: Logger;
        let lazy: CountedValue;
        let mut token_field: LogField;
        let descriptor: TerraneDescriptor;
        let additions: terrane_collection_support::List<LogField>;
        let filtered: LogOutcome;
        let written: LogOutcome;
        let records: terrane_collection_support::List<String>;
        let other_sink: LogSinkResult;
        let other: Logger;
        let target_filtered: LogOutcome;
        let empty_fields: terrane_collection_support::List<LogField>;
        let mut caught: Option<TerraneError> = None;
        let mut error_fields: Option<terrane_collection_support::List<LogField>> = None;
        let mut error_write: Option<LogOutcome> = None;
        let mut error_records: Option<terrane_collection_support::List<String>> = None;
        let failed_sink: LogSinkResult;
        let failed_target: LogSink;
        let failed_logger: Logger;
        let failed_write: LogOutcome;
        let fallback: terrane_collection_support::List<String>;
        let console_result: LogSinkResult;
        let console: Logger;
        let console_write: LogOutcome;
        let adapter_pair: TerraneChannelPair<LogEvent>;
        let adapter_producer: TerraneChannelSender<LogEvent>;
        let adapter_receiver: TerraneChannelReceiver<LogEvent>;
        let adapter_sent: bool;
        let adapter_received: TerraneChannelReceiveOutcome<LogEvent>;
        let async_sink: LogSinkResult;
        let async_target: LogSink;
        let async_writer: Logger;
        let pair: TerraneChannelPair<LogEvent>;
        let producer: TerraneChannelSender<LogEvent>;
        let receiver: TerraneChannelReceiver<LogEvent>;
        let consumer_scope: TerraneTaskScope;
        let consumer: TerraneScopedTask<bool>;
        let first_sent: TerraneChannelSendOutcome<LogEvent>;
        let second_sent: TerraneChannelSendOutcome<LogEvent>;
        let consumed: TerraneTaskOutcome<bool>;
        let consumed_value: Option<bool>;
        let transported_records: terrane_collection_support::List<String>;
        let failing_pair: TerraneChannelPair<LogEvent>;
        let failing_producer: TerraneChannelSender<LogEvent>;
        let failing_receiver: TerraneChannelReceiver<LogEvent>;
        let failing_scope: TerraneTaskScope;
        let failing_consumer: TerraneScopedTask<bool>;
        let failing_sent: TerraneChannelSendOutcome<LogEvent>;
        let failing_consumed: TerraneTaskOutcome<bool>;
        let failing_consumed_value: Option<bool>;
        let async_fallback: terrane_collection_support::List<String>;
        sink_result = memory_sink(
            terrane_int_support::Int::from(4_i128),
            String::from("drop-oldest"),
            terrane_int_support::Int::from(1000_i128),
            terrane_int_support::Int::from(5_i128),
            false,
        );
        writer = make_logger(
            sink_result.value.clone(),
            make_logger_options(
                info_level(),
                String::from("service."),
                String::from("service.worker"),
                terrane_int_support::Int::from(8_i128),
                terrane_int_support::Int::from(4096_i128),
            ),
        );
        writer = with_field(
            writer,
            field_at(
                String::from("request").clone(),
                log_text(String::from("r-1")),
                false,
                "case.trn:30:35".to_owned(),
            ),
        );
        writer = with_span(writer, String::from("serve"));
        lazy = CountedValue::terrane_construct(String::from("rendered"));
        token_field = field_at(
            String::from("token").clone(),
            <LogValue>::from(
                CountedValue::terrane_construct(String::from("raw-secret")),
            ),
            false,
            "case.trn:34:19".to_owned(),
        );
        descriptor = TerraneDescriptor {
            identity: "/structured-logging::request-data",
            name: "request-data",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[
                TerraneFieldMetadata {
                    name: "token",
                    external_name: "token",
                    defaulted: true,
                    optional: false,
                    secret: true,
                },
            ],
        };
        if __terrane_raised(
            terrane_collection_support::List::new(
                    descriptor
                        .fields
                        .iter()
                        .map(|field| field.secret)
                        .collect::<Vec<bool>>(),
                )
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        0 /* terrane-site: case.trn:36:8-36:34 */,
                    ),
                ),
            0 /* terrane-site: case.trn:36:8-36:34 */,
        ) {
            token_field = field_at(
                String::from("token").clone(),
                <LogValue>::from(
                    CountedValue::terrane_construct(String::from("raw-secret")),
                ),
                true,
                "case.trn:37:23".to_owned(),
            );
        }
        additions = terrane_collection_support::List::<
            LogField,
        >::new(
            vec![
                field_at(String::from("value").clone(), < LogValue > ::from(lazy), false,
                "case.trn:38:24".to_owned()), token_field
            ],
        );
        filtered = emit_at(
            writer.clone(),
            debug_level().clone(),
            String::from("filtered").clone(),
            "case.trn:39:16".to_owned(),
            additions.clone(),
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&filtered.filtered),
            terrane_scalar_support::scalar_text(&filtered.failed)
        );
        written = emit_at(
            writer.clone(),
            info_level(),
            String::from("handled").clone(),
            "case.trn:42:15".to_owned(),
            additions.clone(),
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&written.filtered),
            terrane_scalar_support::scalar_text(&written.failed)
        );
        records = drain_memory(writer.clone());
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(records
            .length()))
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            1 /* terrane-site: case.trn:47:13-47:23 */)), 1 /* terrane-site: case.trn:47:13-47:23 */).contains(&String::from("\"timestamp\":1000"))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            2 /* terrane-site: case.trn:47:56-47:66 */)), 2 /* terrane-site: case.trn:47:56-47:66 */).contains(&String::from("\"sequence\":0"))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            3 /* terrane-site: case.trn:47:95-47:105 */)), 3 /* terrane-site: case.trn:47:95-47:105 */).contains(&String::from("\"severity\":\"info\"")))
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            4 /* terrane-site: case.trn:48:13-48:23 */)), 4 /* terrane-site: case.trn:48:13-48:23 */)
            .contains(&String::from("\"target\":\"service.worker\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            5 /* terrane-site: case.trn:48:65-48:75 */)), 5 /* terrane-site: case.trn:48:65-48:75 */).contains(&String::from("\"message\":\"handled\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            6 /* terrane-site: case.trn:48:111-48:121 */)), 6 /* terrane-site: case.trn:48:111-48:121 */).contains(&String::from("\"serve\"")))
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            7 /* terrane-site: case.trn:49:13-49:23 */)), 7 /* terrane-site: case.trn:49:13-49:23 */).contains(&String::from("\"request\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            8 /* terrane-site: case.trn:49:49-49:59 */)), 8 /* terrane-site: case.trn:49:49-49:59 */).contains(&String::from("\"<redacted>\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            9 /* terrane-site: case.trn:49:88-49:98 */)), 9 /* terrane-site: case.trn:49:88-49:98 */).contains(&String::from("raw-secret")))
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            10 /* terrane-site: case.trn:50:13-50:23 */)), 10 /* terrane-site: case.trn:50:13-50:23 */).contains(&String::from("case.trn"))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            11 /* terrane-site: case.trn:50:48-50:58 */)), 11 /* terrane-site: case.trn:50:48-50:58 */).contains(&String::from("\"origin\":\"terrane\"")))
        );
        other_sink = memory_sink(
            terrane_int_support::Int::from(2_i128),
            String::from("drop-oldest"),
            terrane_int_support::Int::from(2000_i128),
            terrane_int_support::Int::from(1_i128),
            false,
        );
        other = make_logger(
            other_sink.value.clone(),
            make_logger_options(
                info_level(),
                String::from("service."),
                String::from("other.worker"),
                terrane_int_support::Int::from(8_i128),
                terrane_int_support::Int::from(4096_i128),
            ),
        );
        target_filtered = emit_at(
            other.clone(),
            info_level().clone(),
            String::from("not-written").clone(),
            "case.trn:54:23".to_owned(),
            additions.clone(),
        );
        println!("{}", terrane_scalar_support::scalar_text(&target_filtered.filtered));
        empty_fields = terrane_collection_support::List::<LogField>::new(vec![]);
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                return TerraneCompletion::Error(
                    TerraneError::raised(
                        TerraneErrorKind::CoercionError,
                        12 /* terrane-site: case.trn:60:9-60:29 */,
                    ),
                );
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0
                        && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_0 = true;
                        let _ = caught.insert(__terrane_error_0.clone());
                        let _ = error_fields
                            .insert(
                                terrane_collection_support::List::<
                                    LogField,
                                >::new(
                                    vec![
                                        field_at(String::from("failure").clone(),
                                        terrane_log_error(caught.as_ref()
                                        .expect("flow-proven available binding").clone().clone()),
                                        false, "case.trn:62:31".to_owned())
                                    ],
                                ),
                            );
                        let _ = error_write
                            .insert(
                                emit_at(
                                    writer.clone(),
                                    info_level(),
                                    String::from("failed-operation").clone(),
                                    "case.trn:63:23".to_owned(),
                                    error_fields
                                        .as_ref()
                                        .expect("flow-proven available binding")
                                        .clone()
                                        .clone(),
                                ),
                            );
                        let _ = error_records.insert(drain_memory(writer));
                        println!(
                            "{}{}{}", terrane_scalar_support::scalar_text(&error_write
                            .as_ref().expect("flow-proven available binding").clone()
                            .failed),
                            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(error_records
                            .as_ref().expect("flow-proven available binding").clone()
                            .length())),
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(error_records
                            .as_ref().expect("flow-proven available binding").clone()
                            .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                            13 /* terrane-site: case.trn:65:59-65:75 */)),
                            13 /* terrane-site: case.trn:65:59-65:75 */)
                            .contains(&String::from("coercion-error")))
                        );
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        failed_sink = failing_sink();
        failed_target = failed_sink.value.clone();
        failed_logger = default_logger(failed_target.clone());
        failed_write = emit_at(
            failed_logger.clone(),
            info_level(),
            String::from("cannot-write").clone(),
            "case.trn:69:20".to_owned(),
            empty_fields.clone(),
        );
        fallback = drain_fallback();
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&failed_write.failed),
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fallback
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(fallback
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            14 /* terrane-site: case.trn:71:51-71:62 */)), 14 /* terrane-site: case.trn:71:51-71:62 */).contains(&String::from("event rejected")))
        );
        console_result = console_sink(false);
        console = default_logger(console_result.value.clone());
        console_write = emit_at(
            console.clone(),
            info_level(),
            String::from("visible").clone(),
            "case.trn:75:21".to_owned(),
            empty_fields.clone(),
        );
        println!("{}", terrane_scalar_support::scalar_text(&console_write.failed));
        adapter_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        adapter_producer = adapter_pair.sender;
        adapter_receiver = adapter_pair.receiver;
        adapter_sent = __terrane_await(
                send_event(
                    adapter_producer,
                    make_event_at(
                        info_level().clone(),
                        String::from("adapter").clone(),
                        "case.trn:81:57".to_owned(),
                        empty_fields.clone(),
                    ),
                ),
            )
            .await;
        adapter_received = __terrane_await(Box::pin(adapter_receiver.receive())).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&adapter_sent),
            terrane_scalar_support::scalar_text(&adapter_received.available)
        );
        async_sink = memory_sink(
            terrane_int_support::Int::from(4_i128),
            String::from("reject"),
            terrane_int_support::Int::from(3000_i128),
            terrane_int_support::Int::from(1_i128),
            false,
        );
        async_target = async_sink.value.clone();
        async_writer = default_logger(async_target.clone());
        pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        producer = pair.sender;
        receiver = pair.receiver;
        consumer_scope = TerraneTaskScope::new(None);
        consumer = {
            let __terrane_scope = consumer_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = consume_events(async_target, receiver);
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        first_sent = __terrane_await(
                Box::pin(
                    producer
                        .send(
                            make_event_at(
                                info_level().clone(),
                                String::from("transported-one").clone(),
                                "case.trn:93:40".to_owned(),
                                empty_fields.clone(),
                            ),
                        ),
                ),
            )
            .await;
        second_sent = __terrane_await(
                Box::pin(
                    producer
                        .send(
                            make_event_at(
                                info_level().clone(),
                                String::from("transported-two").clone(),
                                "case.trn:94:41".to_owned(),
                                empty_fields.clone(),
                            ),
                        ),
                ),
            )
            .await;
        producer.close();
        consumed = __terrane_await(consumer_scope.join(consumer)).await;
        consumed_value = consumed.value.clone();
        if consumed_value.is_some() {
            println!(
                "{}{}{}{}", terrane_scalar_support::scalar_text(&first_sent.accepted),
                terrane_scalar_support::scalar_text(&second_sent.accepted),
                terrane_scalar_support::scalar_text(&consumed.completed),
                terrane_scalar_support::scalar_text(&* match &consumed_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        transported_records = drain_memory(async_writer);
        println!(
            "{}{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(transported_records
            .length())),
            terrane_scalar_support::scalar_text(&__terrane_raised(transported_records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            15 /* terrane-site: case.trn:101:41-101:63 */)), 15 /* terrane-site: case.trn:101:41-101:63 */)
            .contains(&String::from("\"message\":\"transported-one\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(transported_records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            16 /* terrane-site: case.trn:101:107-101:129 */)), 16 /* terrane-site: case.trn:101:107-101:129 */)
            .contains(&String::from("\"message\":\"transported-two\"")))
        );
        failing_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        failing_producer = failing_pair.sender;
        failing_receiver = failing_pair.receiver;
        failing_scope = TerraneTaskScope::new(None);
        failing_consumer = {
            let __terrane_scope = failing_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = consume_events(failed_target, failing_receiver);
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        failing_sent = __terrane_await(
                Box::pin(
                    failing_producer
                        .send(
                            make_event_at(
                                info_level().clone(),
                                String::from("transport-failure").clone(),
                                "case.trn:108:50".to_owned(),
                                empty_fields.clone(),
                            ),
                        ),
                ),
            )
            .await;
        failing_producer.close();
        failing_consumed = __terrane_await(failing_scope.join(failing_consumer)).await;
        failing_consumed_value = failing_consumed.value.clone();
        if failing_consumed_value.is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&failing_sent.accepted),
                terrane_scalar_support::scalar_text(&failing_consumed.completed),
                terrane_scalar_support::scalar_text(&* match &failing_consumed_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        async_fallback = drain_fallback();
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(async_fallback
            .length())),
            terrane_scalar_support::scalar_text(&__terrane_raised(async_fallback
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            17 /* terrane-site: case.trn:115:36-115:53 */)), 17 /* terrane-site: case.trn:115:36-115:53 */).contains(&String::from("event rejected")))
        );
    });
}
