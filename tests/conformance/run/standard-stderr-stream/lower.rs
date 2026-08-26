// Generated deterministically by Terrane <version>.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerraneErrorKind {
    ArithmeticOverflow,
    DivisionByZero,
    IntegerConversionOverflow,
    NegativeShiftCount,
    CoercionError,
    DecodeError,
    IndexError,
    MissingKey,
    ResourceError,
    SourceError,
}
impl TerraneErrorKind {
    fn from_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string().trim_start_matches(".decode-error: "),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
}
fn __terrane_uncaught(error: TerraneError) -> ! {
    eprintln!("{}", error.render());
    std::process::exit(1);
}
fn __terrane_generated_defect(message: &str) -> ! {
    eprintln!(
        "internal compiler defect: generated program reached an impossible completion: {message}"
    );
    std::process::exit(5);
}
#[allow(dead_code)]
enum TerraneCompletion<T> {
    Normal,
    Return(T),
    Error(TerraneError),
    Break,
    Continue,
}
async fn __terrane_await<F: Future>(future: F) -> F::Output {
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            context: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            if self.0 {
                std::task::Poll::Ready(())
            } else {
                self.0 = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
    YieldOnce(false).await;
    let output = future.await;
    YieldOnce(false).await;
    output
}
fn __terrane_block_on<F: Future>(future: F) -> F::Output {
    struct Wake;
    impl std::task::Wake for Wake {
        fn wake(self: std::sync::Arc<Self>) {}
    }
    let waker = std::task::Waker::from(std::sync::Arc::new(Wake));
    let mut context = std::task::Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(value) => return value,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}
#[derive(Clone)]
pub struct TerranePlatformStreamHandle(std::sync::Arc<i64>);
impl Default for TerranePlatformStreamHandle {
    fn default() -> Self {
        Self(std::sync::Arc::new(0))
    }
}
impl TerranePlatformStreamHandle {
    fn new(handle: terrane_stream_abi::StreamHandle) -> Self {
        Self(std::sync::Arc::new(handle.id()))
    }
    fn abi_handle(&self) -> terrane_stream_abi::StreamHandle {
        terrane_stream_abi::StreamHandle::from_id(*self.0)
    }
}
#[derive(Clone)]
struct TerranePlatformReadResult {
    data: Vec<u8>,
    completed: terrane_int_support::Int,
    end: bool,
    failed: bool,
    message: String,
}
#[derive(Clone)]
struct TerranePlatformWriteResult {
    completed: terrane_int_support::Int,
    failed: bool,
    message: String,
}
#[derive(Clone)]
struct TerranePlatformUnitResult {
    failed: bool,
    message: String,
}
fn terrane_platform_acquire_stdin() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdin())
}
fn terrane_platform_acquire_stdout() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdout())
}
fn terrane_platform_acquire_stderr() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stderr())
}
fn terrane_platform_read(
    handle: &TerranePlatformStreamHandle,
    limit: terrane_int_support::Int,
) -> TerranePlatformReadResult {
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformReadResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            end: false,
            failed: true,
            message: "stream read count is outside the supported size range".to_owned(),
        };
    };
    match terrane_stream_abi::read(handle.abi_handle(), limit) {
        Ok(outcome) => {
            TerranePlatformReadResult {
                completed: terrane_int_support::Int::from(outcome.data.len() as i128),
                data: outcome.data,
                end: outcome.end,
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformReadResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(0_i64),
                end: false,
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
fn terrane_platform_write(
    handle: &TerranePlatformStreamHandle,
    data: &[u8],
    offset: terrane_int_support::Int,
) -> TerranePlatformWriteResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformWriteResult {
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: "stream write offset is outside the buffer".to_owned(),
        };
    };
    match terrane_stream_abi::write(handle.abi_handle(), &data[offset..]) {
        Ok(completed) => {
            TerranePlatformWriteResult {
                completed: terrane_int_support::Int::from(completed as i128),
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformWriteResult {
                completed: terrane_int_support::Int::from(0_i64),
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
fn terrane_platform_flush(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::flush(handle.abi_handle()))
}
fn terrane_platform_sync_data(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_data(handle.abi_handle()))
}
fn terrane_platform_sync_all(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_all(handle.abi_handle()))
}
fn terrane_platform_close(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::close(handle.abi_handle()))
}
fn terrane_platform_release(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    if std::sync::Arc::strong_count(&handle.0) == 1 {
        terrane_platform_unit(terrane_stream_abi::release(handle.abi_handle()))
    } else {
        TerranePlatformUnitResult {
            failed: false,
            message: String::new(),
        }
    }
}
fn terrane_platform_unit(result: std::io::Result<()>) -> TerranePlatformUnitResult {
    match result {
        Ok(()) => {
            TerranePlatformUnitResult {
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformUnitResult {
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
// Source: case.trn
// Namespace: standard-stderr-stream
fn main() {
    let error: ByteWriter = stderr();
    let written: WriteResult = error
        .write_all(
            Vec::from([116, 121, 112, 101, 100, 32, 115, 116, 100, 101, 114, 114, 10]),
        );
    if written.failed {
        println!("{}", terrane_scalar_support::scalar_text(&written.message));
    }
    error.close();
}
// Source: standard/streams.trn
// Namespace: standard/streams
#[derive(Clone)]
pub struct OperationResult {
    pub failed: bool,
    pub message: String,
}
impl OperationResult {
    pub fn terrane_construct(failed: bool, message: String) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
        };
        value.construct(failed, message);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String) {
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct ReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl ReadResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(data, completed, end, failed, message);
        value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        self.data = data;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct TextReadResult {
    pub text: String,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl TextReadResult {
    pub fn terrane_construct(
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            text: String::from(""),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(text, completed, end, failed, message);
        value
    }
    pub fn construct(
        &mut self,
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        self.text = text;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct WriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
}
impl WriteResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            failed: false,
            message: String::from(""),
        };
        value.construct(data, completed, failed, message);
        value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) {
        self.data = data;
        self.completed = completed.clone();
        self.failed = failed;
        self.message = message;
    }
}
pub struct ByteReader {
    pub handle: TerranePlatformStreamHandle,
}
impl ByteReader {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut value = Self { handle: Default::default() };
        value.construct(handle);
        value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        self.handle = handle;
    }
    pub fn read(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw: TerranePlatformReadResult = terrane_platform_read(&self.handle, count);
        return ReadResult::terrane_construct(
            raw.data.clone().clone(),
            raw.completed.clone(),
            raw.end,
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn read_exact(&self, count: terrane_int_support::Int) -> ReadResult {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < count.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                count.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        if end && completed.clone() < count.clone() && !failed {
            failed = true;
            message = String::from("stream ended before exact byte count");
        }
        return ReadResult::terrane_construct(
            data,
            completed.clone(),
            end,
            failed,
            message,
        );
    }
    pub fn read_all(&self, limit: terrane_int_support::Int) -> ReadResult {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < limit.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                limit.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        return ReadResult::terrane_construct(
            data,
            completed.clone(),
            end,
            failed,
            message,
        );
    }
    pub async fn read_async(&self, count: terrane_int_support::Int) -> ReadResult {
        return self.read(count.clone());
    }
    pub fn text(self, codec: terrane_string_support::Encoding) -> TextReader {
        return TextReader::terrane_construct(self.handle.clone(), codec);
    }
    pub fn close(self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for ByteReader {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct ByteWriter {
    pub handle: TerranePlatformStreamHandle,
}
impl ByteWriter {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut value = Self { handle: Default::default() };
        value.construct(handle);
        value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        self.handle = handle;
    }
    pub fn write(&self, data: Vec<u8>) -> WriteResult {
        let offset: i64 = 0;
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &data,
            terrane_int_support::Int::from(offset.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn write_all(&self, data: Vec<u8>) -> WriteResult {
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            let part: TerranePlatformWriteResult = terrane_platform_write(
                &self.handle,
                &data,
                terrane_int_support::Int::from(completed.clone()),
            );
            completed = completed.clone() + part.completed.clone();
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.failed
            {
                failed = true;
                message = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(data, completed.clone(), failed, message);
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub async fn write_async(&self, data: Vec<u8>) -> WriteResult {
        return self.write(data);
    }
    pub fn text(self, codec: terrane_string_support::Encoding) -> TextWriter {
        return TextWriter::terrane_construct(self.handle.clone(), codec);
    }
    pub fn flush(&self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_flush(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_data(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_all(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn close(self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for ByteWriter {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct TextReader {
    pub handle: TerranePlatformStreamHandle,
    pub codec: terrane_string_support::Encoding,
}
impl TextReader {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut value = Self {
            handle: Default::default(),
            codec: terrane_string_support::Encoding::Utf8,
        };
        value.construct(handle, codec);
        value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        self.handle = handle;
        self.codec = codec;
    }
    pub fn read(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw: TerranePlatformReadResult = terrane_platform_read(&self.handle, count);
        let text: String = terrane_string_support::decode(&raw.data.clone(), self.codec)
            .map_err(|error| {
                TerraneError::from(error)
                    .at("/standard/streams::read (streams.trn:190:23)")
            })?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                raw.completed.clone(),
                raw.end,
                raw.failed,
                raw.message.clone().clone(),
            ),
        );
    }
    pub fn read_exact(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < count.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                count.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        if end && completed.clone() < count.clone() && !failed {
            failed = true;
            message = String::from("stream ended before exact byte count");
        }
        let text: String = terrane_string_support::decode(&data, self.codec)
            .map_err(|error| {
                TerraneError::from(error)
                    .at("/standard/streams::read-exact (streams.trn:212:23)")
            })?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                completed.clone(),
                end,
                failed,
                message,
            ),
        );
    }
    pub fn read_all(
        &self,
        limit: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data: Vec<u8> = Vec::from([]);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut end: bool = false;
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < limit.clone() && !end && !failed {
            let part: TerranePlatformReadResult = terrane_platform_read(
                &self.handle,
                limit.clone() - completed.clone(),
            );
            data = {
                let mut bytes = data;
                bytes.extend(part.data.clone());
                bytes
            };
            completed = completed.clone() + part.completed.clone();
            end = part.end;
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.end && !part.failed
            {
                failed = true;
                message = String::from("stream read made no progress");
            }
        }
        let text: String = terrane_string_support::decode(&data, self.codec)
            .map_err(|error| {
                TerraneError::from(error)
                    .at("/standard/streams::read-all (streams.trn:231:23)")
            })?;
        return Ok(
            TextReadResult::terrane_construct(
                text,
                completed.clone(),
                end,
                failed,
                message,
            ),
        );
    }
    pub async fn read_async(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        return Ok(
            self
                .read(count.clone())
                .map_err(|error| {
                    error.at("/standard/streams::read-async (streams.trn:235:16)")
                })?,
        );
    }
    pub fn close(self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for TextReader {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct TextWriter {
    pub handle: TerranePlatformStreamHandle,
    pub codec: terrane_string_support::Encoding,
}
impl TextWriter {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut value = Self {
            handle: Default::default(),
            codec: terrane_string_support::Encoding::Utf8,
        };
        value.construct(handle, codec);
        value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        self.handle = handle;
        self.codec = codec;
    }
    pub fn write(&self, text: String) -> WriteResult {
        let data: Vec<u8> = terrane_string_support::encode(&text, self.codec);
        let offset: i64 = 0;
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &data,
            terrane_int_support::Int::from(offset.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn write_all(&self, text: String) -> WriteResult {
        let data: Vec<u8> = terrane_string_support::encode(&text, self.codec);
        let mut completed: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let mut failed: bool = false;
        let mut message: String = String::from("");
        while completed.clone() < terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            let part: TerranePlatformWriteResult = terrane_platform_write(
                &self.handle,
                &data,
                terrane_int_support::Int::from(completed.clone()),
            );
            completed = completed.clone() + part.completed.clone();
            failed = part.failed;
            message = part.message.clone().clone();
            if part.completed.clone() == terrane_int_support::Int::from(0_i128)
                && !part.failed
            {
                failed = true;
                message = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(data, completed.clone(), failed, message);
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw: TerranePlatformWriteResult = terrane_platform_write(
            &self.handle,
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw.completed.clone(),
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn line(&self, text: String) -> WriteResult {
        return self
            .write_all(
                format!(
                    "{}{}", terrane_scalar_support::scalar_text(&text),
                    terrane_scalar_support::scalar_text(&String::from("\n"))
                ),
            );
    }
    pub async fn write_async(&self, text: String) -> WriteResult {
        return self.write(text);
    }
    pub fn flush(&self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_flush(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_data(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_sync_all(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn close(self) -> OperationResult {
        let raw: TerranePlatformUnitResult = terrane_platform_close(&self.handle);
        return OperationResult::terrane_construct(
            raw.failed,
            raw.message.clone().clone(),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for TextWriter {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub fn stdin() -> ByteReader {
    return ByteReader::terrane_construct(terrane_platform_acquire_stdin());
}
pub fn stdout() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stdout());
}
pub fn stderr() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stderr());
}
