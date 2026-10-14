// Generated deterministically by Terrane 0.1.0.
include!("main.lowered.support.rs");
// Source: src/main.trn
// Namespace: benchmark-http1-server
#[derive(Clone)]
pub struct BenchmarkFailure {
    pub message: String,
}
impl BenchmarkFailure {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("HTTP benchmark failed"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct Payload {
    pub value: u64,
}
impl Payload {
    pub fn terrane_construct() -> Self {
        Self { value: 0 }
    }
}
impl TerraneDocumentDecode for Payload {
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
                "payload",
                source,
                "src/main.trn:15:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["value"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "payload",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "src/main.trn:15:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("value");
            let field_path = __terrane_document_child_path(path, "value");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <u64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "src/main.trn:16:5",
                ) {
                    Ok(decoded) => value.value = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Payload {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Payload> for DocumentDecodable {
    fn from(value: Payload) -> Self {
        Self(Box::new(value))
    }
}
pub struct HttpConnection {
    pub stream: Option<TcpStream>,
}
impl HttpConnection {
    pub fn terrane_construct(stream: TcpStream) -> Self {
        let mut __terrane_constructed_value = Self { stream: None };
        __terrane_constructed_value.construct(stream);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, stream: TcpStream) {
        self.stream = Some(stream);
    }
    pub async fn read_message(
        &self,
        start_line: String,
    ) -> Result<String, TerraneError> {
        let allocated: ReadBufferResult;
        let mut buffer: OwnedReadBuffer;
        let mut body_length: terrane_int_support::Int;
        let mut header_length: terrane_int_support::Int;
        let mut remaining: terrane_int_support::Int;
        let mut received: BufferedReadResult;
        let mut filled: BufferTextResult;
        let mut text: String;
        let mut parts: Vec<String>;
        let mut headers: Vec<String>;
        let mut header: String;
        let mut length_text: String;
        allocated = make_read_buffer(terrane_int_support::Int::from(4096_i128));
        if allocated.failed {
            return Err({
                let value = BenchmarkFailure::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    message,
                    rendered,
                    0 /* terrane-site: src/main.trn:29:13-29:46 */,
                )
            });
        }
        buffer = allocated.value;
        body_length = terrane_int_support::Int::from(-1_i128);
        header_length = terrane_int_support::Int::from(0_i128);
        while buffer.completed() < buffer.capacity() {
            remaining = buffer.capacity() - buffer.completed();
            received = __terrane_await(
                    (&self.stream.as_ref().expect("required field initialized"))
                        .read_into_async(buffer, remaining.clone(), None),
                )
                .await;
            buffer = received.buffer.take().expect("required field initialized");
            if received.failed || received.end {
                return Err({
                    let value = BenchmarkFailure::terrane_construct();
                    let rendered = value.render();
                    let original = &value.message;
                    let (message, rendered) = if &rendered == original {
                        (rendered, None)
                    } else {
                        (original.clone(), Some(rendered))
                    };
                    TerraneError::raised_with_rendered_message(
                        TerraneErrorKind::Custom(DescriptorId(0)),
                        message,
                        rendered,
                        1 /* terrane-site: src/main.trn:38:17-38:50 */,
                    )
                });
            }
            filled = buffer.filled_text();
            if filled.failed {
                return Err({
                    let value = BenchmarkFailure::terrane_construct();
                    let rendered = value.render();
                    let original = &value.message;
                    let (message, rendered) = if &rendered == original {
                        (rendered, None)
                    } else {
                        (original.clone(), Some(rendered))
                    };
                    TerraneError::raised_with_rendered_message(
                        TerraneErrorKind::Custom(DescriptorId(0)),
                        message,
                        rendered,
                        2 /* terrane-site: src/main.trn:41:17-41:50 */,
                    )
                });
            }
            text = filled.value.clone();
            parts = terrane_string_support::split(&text, &String::from("\r\n\r\n"));
            if terrane_int_support::Int::from(parts.len() as i128)
                > terrane_int_support::Int::from(1_i128)
            {
                if body_length.clone() < terrane_int_support::Int::from(0_i128) {
                    headers = terrane_string_support::split(
                        &__terrane_raised_err(
                            {
                                let __terrane_receiver = &parts;
                                let __terrane_index = __terrane_raised_err(
                                    terrane_collection_support::index_from_int(
                                        &terrane_int_support::Int::from(0_i128),
                                    ),
                                    3 /* terrane-site: src/main.trn:46:31-46:39 */,
                                )?;
                                __terrane_receiver
                                    .get(__terrane_index)
                                    .cloned()
                                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                        __terrane_index,
                                    ))
                            },
                            3 /* terrane-site: src/main.trn:46:31-46:39 */,
                        )?,
                        &String::from("\r\n"),
                    );
                    if __terrane_raised_err(
                            {
                                let __terrane_receiver = &headers;
                                let __terrane_index = __terrane_raised_err(
                                    terrane_collection_support::index_from_int(
                                        &terrane_int_support::Int::from(0_i128),
                                    ),
                                    4 /* terrane-site: src/main.trn:47:24-47:34 */,
                                )?;
                                __terrane_receiver
                                    .get(__terrane_index)
                                    .cloned()
                                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                        __terrane_index,
                                    ))
                            },
                            4 /* terrane-site: src/main.trn:47:24-47:34 */,
                        )?
                        .as_str() != start_line.as_str()
                    {
                        return Err({
                            let value = BenchmarkFailure::terrane_construct();
                            let rendered = value.render();
                            let original = &value.message;
                            let (message, rendered) = if &rendered == original {
                                (rendered, None)
                            } else {
                                (original.clone(), Some(rendered))
                            };
                            TerraneError::raised_with_rendered_message(
                                TerraneErrorKind::Custom(DescriptorId(0)),
                                message,
                                rendered,
                                5 /* terrane-site: src/main.trn:48:25-48:58 */,
                            )
                        });
                    }
                    let __terrane_iterable_0 = headers;
                    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
                        &__terrane_iterable_0,
                    );
                    loop {
                        header = match __terrane_iterator_0.next() {
                            terrane_collection_support::IterationStep::Item(item) => item,
                            terrane_collection_support::IterationStep::End => break,
                        };
                        if header.starts_with(&String::from("Content-Length:")) {
                            length_text = terrane_string_support::trim_start(
                                &header,
                                Some(&String::from("Content-Length:")),
                            );
                            body_length = __terrane_raised_err(
                                terrane_int_support::parse_radix(
                                    &terrane_string_support::trim(&length_text),
                                    &10,
                                ),
                                6 /* terrane-site: src/main.trn:52:43-52:72 */,
                            )?;
                        }
                    }
                    if body_length.clone() < terrane_int_support::Int::from(0_i128) {
                        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
                    }
                    header_length = terrane_int_support::Int::from(
                        __terrane_raised_err(
                                {
                                    let __terrane_receiver = &parts;
                                    let __terrane_index = __terrane_raised_err(
                                        terrane_collection_support::index_from_int(
                                            &terrane_int_support::Int::from(0_i128),
                                        ),
                                        7 /* terrane-site: src/main.trn:55:37-55:45 */,
                                    )?;
                                    __terrane_receiver
                                        .get(__terrane_index)
                                        .cloned()
                                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                            __terrane_index,
                                        ))
                                },
                                7 /* terrane-site: src/main.trn:55:37-55:45 */,
                            )?
                            .len() as i128,
                    ) + terrane_int_support::Int::from(4_i128);
                }
                if buffer.completed() >= header_length.clone() + body_length.clone() {
                    return Ok(
                        __terrane_raised_err(
                            {
                                let __terrane_receiver = &parts;
                                let __terrane_index = __terrane_raised_err(
                                    terrane_collection_support::index_from_int(
                                        &terrane_int_support::Int::from(1_i128),
                                    ),
                                    8 /* terrane-site: src/main.trn:57:28-57:36 */,
                                )?;
                                __terrane_receiver
                                    .get(__terrane_index)
                                    .cloned()
                                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                        __terrane_index,
                                    ))
                            },
                            8 /* terrane-site: src/main.trn:57:28-57:36 */,
                        )?,
                    );
                }
            }
        }
        return Err({
            let value = BenchmarkFailure::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                9 /* terrane-site: src/main.trn:58:9-58:42 */,
            )
        });
    }
    pub fn close(mut self) {
        let stream: TcpStream;
        stream = self.stream.take().expect("required field initialized");
        stream.close();
    }
}
async fn handle_request(stream: TcpStream) -> Result<(), TerraneError> {
    let connection: HttpConnection;
    let body: String;
    let parsed: TerraneDocumentDecodeOutcome<Payload>;
    let text: String;
    let response: String;
    let written: WriteResult;
    connection = HttpConnection::terrane_construct(stream);
    body = __terrane_traced_err(
        __terrane_await((&connection).read_message(String::from("POST /api HTTP/1.1")))
            .await,
        10 /* terrane-site: src/main.trn:66:18-66:63 */,
    )?;
    parsed = {
        let __terrane_document_source = &body;
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(128_i128),
        );
        let __terrane_document_allow_unknown = true;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Payload as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "src/main.trn:67:14",
                    "src/main.trn:67:14",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Payload::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Payload::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "payload", "invalid", "parse", error.message,
                            "src/main.trn:67:14", "src/main.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    if parsed.diagnostics.length() != 0 {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    text = terrane_scalar_support::scalar_text(&parsed.value.clone().value);
    response = {
        let mut __terrane_text = String::with_capacity(56);
        __terrane_text.push_str("HTTP/1.1 200 OK\r\nContent-Length: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &(text.len() as i128),
            &mut __terrane_text,
        );
        __terrane_text.push_str("\r\nConnection: close\r\n\r\n");
        terrane_scalar_support::ScalarDisplay::write_scalar(&text, &mut __terrane_text);
        __terrane_text
    };
    written = __terrane_await(
            (&connection.stream.as_ref().expect("required field initialized"))
                .write_all_async(
                    terrane_string_support::encode(
                        &response,
                        terrane_string_support::Encoding::Utf8,
                    ),
                    None,
                ),
        )
        .await;
    if written.failed {
        return Err({
            let value = BenchmarkFailure::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                11 /* terrane-site: src/main.trn:74:9-74:42 */,
            )
        });
    }
    connection.close();
    return Ok(());
}
async fn serve(
    listener: TcpListener,
    count: terrane_int_support::Int,
) -> Result<(), TerraneError> {
    __terrane_group_frame(async move {
            let handlers: TerraneTaskGroup<()>;
            let mut index: terrane_int_support::Int;
            let mut accepted: StreamResult;
            let outcomes: terrane_collection_support::List<TerraneTaskOutcome<()>>;
            let mut outcome: TerraneTaskOutcome<()>;
            handlers = TerraneTaskGroup::<()>::new();
            index = terrane_int_support::Int::from(0_i128);
            while index.clone() < count.clone() {
                accepted = __terrane_await((&listener).accept(None)).await;
                if accepted.failed {
                    exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
                }
                {
                    let __terrane_child = {
                        let __terrane_scope = handlers.scope();
                        let __terrane_cancel = __terrane_scope.cancellation();
                        let __terrane_deadline = __terrane_scope.deadline.clone();
                        let __terrane_spawned_task = handle_request(accepted.value);
                        TerraneScopedTask::spawn(async move {
                            match __terrane_cancellable(
                                    __terrane_spawned_task,
                                    __terrane_cancel,
                                    __terrane_deadline,
                                )
                                .await
                            {
                                Some(Ok(value)) => TerraneTaskResult::Completed(value),
                                Some(Err(error)) => {
                                    __terrane_scope.cancel();
                                    TerraneTaskResult::Failed(error)
                                }
                                None => TerraneTaskResult::Cancelled,
                            }
                        })
                    };
                    handlers.spawn(__terrane_child);
                };
                index = index.clone() + terrane_int_support::Int::from(1_i128);
            }
            listener.close();
            outcomes = __terrane_await(std::boxed::Box::pin(handlers.prepare_drain()))
                .await;
            let __terrane_iterable_1 = outcomes;
            let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
                &__terrane_iterable_1,
            );
            loop {
                outcome = match __terrane_iterator_1.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                if !outcome.completed {
                    exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
                }
            }
            return Ok(());
        })
        .await
}
async fn send_one(
    address: SocketAddress,
    value: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, TerraneError> {
    let connected: StreamResult;
    let connection: HttpConnection;
    let payload: String;
    let request: String;
    let written: WriteResult;
    let response: String;
    let decoded: terrane_int_support::Int;
    connected = __terrane_await(connect_tcp(address, None, None)).await;
    if connected.failed {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    connection = HttpConnection::terrane_construct(connected.value);
    payload = {
        let mut __terrane_text = String::with_capacity(10);
        __terrane_text.push_str("{\"value\":");
        terrane_scalar_support::ScalarDisplay::write_scalar(&value, &mut __terrane_text);
        __terrane_text.push_str("}");
        __terrane_text
    };
    request = {
        let mut __terrane_text = String::with_capacity(76);
        __terrane_text
            .push_str("POST /api HTTP/1.1\r\nHost: localhost\r\nContent-Length: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &(payload.len() as i128),
            &mut __terrane_text,
        );
        __terrane_text.push_str("\r\nConnection: close\r\n\r\n");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &payload,
            &mut __terrane_text,
        );
        __terrane_text
    };
    written = __terrane_await(
            (&connection.stream.as_ref().expect("required field initialized"))
                .write_all_async(
                    terrane_string_support::encode(
                        &request,
                        terrane_string_support::Encoding::Utf8,
                    ),
                    None,
                ),
        )
        .await;
    if written.failed {
        return Err({
            let value = BenchmarkFailure::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                13 /* terrane-site: src/main.trn:103:9-103:42 */,
            )
        });
    }
    response = __terrane_traced_err(
        __terrane_await((&connection).read_message(String::from("HTTP/1.1 200 OK")))
            .await,
        14 /* terrane-site: src/main.trn:104:22-104:64 */,
    )?;
    connection.close();
    decoded = __terrane_raised_err(
        terrane_int_support::parse_radix(&response, &10),
        15 /* terrane-site: src/main.trn:106:19-106:37 */,
    )?;
    if decoded.clone() != value.clone() {
        return Err({
            let value = BenchmarkFailure::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                16 /* terrane-site: src/main.trn:108:9-108:42 */,
            )
        });
    }
    return Ok(decoded.clone());
}
fn main() {
    __terrane_run(async move {
        __terrane_group_frame(async move {
                let supplied: terrane_collection_support::List<NativeString>;
                let count: terrane_int_support::Int;
                let endpoint: SocketResult;
                let bound: ListenerResult;
                let listener: TcpListener;
                let destination: SocketResult;
                let scope: TerraneTaskScope;
                let server: TerraneScopedTask<()>;
                let requests: TerraneTaskGroup<terrane_int_support::Int>;
                let mut index: terrane_int_support::Int;
                let outcomes: terrane_collection_support::List<
                    TerraneTaskOutcome<terrane_int_support::Int>,
                >;
                let mut total: terrane_int_support::Int;
                let mut result: TerraneTaskOutcome<terrane_int_support::Int>;
                let mut value: Option<terrane_int_support::Int>;
                let outcome: TerraneTaskOutcome<()>;
                supplied = arguments();
                if terrane_int_support::Int::from(
                    terrane_int_support::Int::from(supplied.length()),
                ) != terrane_int_support::Int::from(1_i128)
                {
                    exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
                }
                count = __terrane_raised(
                    terrane_int_support::parse_radix(
                        &__terrane_raised(
                                supplied
                                    .get_or_error(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(
                                                &terrane_int_support::Int::from(0_i128),
                                            ),
                                            17 /* terrane-site: src/main.trn:116:17-116:28 */,
                                        ),
                                    ),
                                17 /* terrane-site: src/main.trn:116:17-116:28 */,
                            )
                            .text,
                        &10,
                    ),
                    18 /* terrane-site: src/main.trn:116:17-116:43 */,
                );
                if count.clone() <= terrane_int_support::Int::from(0_i128) {
                    exit(make_exit_status(terrane_int_support::Int::from(2_i128)));
                }
                endpoint = socket_address_from_string(String::from("127.0.0.1:0"));
                bound = bind_tcp(
                    endpoint.value,
                    Some(
                        TcpListenerOptions::terrane_construct(
                            terrane_int_support::Int::from(1024_i128),
                            false,
                            false,
                            terrane_int_support::Int::from(-1_i128),
                        ),
                    ),
                );
                if bound.failed {
                    exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
                }
                listener = bound.value;
                destination = socket_address_from_string(listener.local_address.clone());
                scope = TerraneTaskScope::new(None);
                server = {
                    let __terrane_scope = scope.clone();
                    let __terrane_cancel = __terrane_scope.cancellation();
                    let __terrane_deadline = __terrane_scope.deadline;
                    let __terrane_spawned_task = serve(listener, count.clone());
                    TerraneScopedTask::spawn(async move {
                        match __terrane_cancellable(
                                __terrane_spawned_task,
                                __terrane_cancel,
                                __terrane_deadline,
                            )
                            .await
                        {
                            Some(Ok(value)) => TerraneTaskResult::Completed(value),
                            Some(Err(error)) => TerraneTaskResult::Failed(error),
                            None => TerraneTaskResult::Cancelled,
                        }
                    })
                };
                requests = TerraneTaskGroup::<terrane_int_support::Int>::new();
                index = terrane_int_support::Int::from(1_i128);
                while index.clone() <= count.clone() {
                    {
                        let __terrane_child = {
                            let __terrane_scope = requests.scope();
                            let __terrane_cancel = __terrane_scope.cancellation();
                            let __terrane_deadline = __terrane_scope.deadline.clone();
                            let __terrane_spawned_task = send_one(
                                destination.value.clone(),
                                index.clone(),
                            );
                            TerraneScopedTask::spawn(async move {
                                match __terrane_cancellable(
                                        __terrane_spawned_task,
                                        __terrane_cancel,
                                        __terrane_deadline,
                                    )
                                    .await
                                {
                                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                                    Some(Err(error)) => {
                                        __terrane_scope.cancel();
                                        TerraneTaskResult::Failed(error)
                                    }
                                    None => TerraneTaskResult::Cancelled,
                                }
                            })
                        };
                        requests.spawn(__terrane_child);
                    };
                    index = index.clone() + terrane_int_support::Int::from(1_i128);
                }
                outcomes = __terrane_await(
                        std::boxed::Box::pin(requests.prepare_drain()),
                    )
                    .await;
                total = terrane_int_support::Int::from(0_i128);
                let __terrane_iterable_2 = outcomes;
                let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
                    &__terrane_iterable_2,
                );
                loop {
                    result = match __terrane_iterator_2.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    };
                    value = result.value.clone();
                    if !result.completed || value.is_none() {
                        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
                    }
                    if value.is_some() {
                        total = total.clone()
                            + match &value {
                                Some(value) => value,
                                _ => unreachable!("flow-proven storage refinement"),
                            }
                                .clone();
                    }
                }
                outcome = __terrane_await(scope.join(server)).await;
                if !outcome.completed {
                    exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
                }
                println!("{}", terrane_scalar_support::scalar_text(&total));
            })
            .await
    });
}
