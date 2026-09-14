fn terrane_decode_platform_value(value: String) -> std::ffi::OsString {
    if let Some(text) = value.strip_prefix("text:") {
        return std::ffi::OsString::from(text);
    }
    let bytes = value
        .strip_prefix("raw:")
        .map(terrane_unhex)
        .unwrap_or_default();
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt as _;
        std::ffi::OsString::from_vec(bytes)
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt as _;
        let wide = bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]));
        std::ffi::OsString::from_wide(&wide.collect::<Vec<_>>())
    }
}

fn terrane_read_process_output(mut stream: impl std::io::Read) -> Vec<u8> {
    let mut output = Vec::new();
    let _ = stream.read_to_end(&mut output);
    output
}

fn terrane_spawn_process_reader(
    stream: impl std::io::Read + Send + 'static,
) -> std::sync::mpsc::Receiver<Vec<u8>> {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let _ = sender.send(terrane_read_process_output(stream));
    });
    receiver
}

fn terrane_wait_fixture_child(
    child: &mut std::process::Child,
    timeout: std::time::Duration,
) -> Result<(std::process::ExitStatus, bool), String> {
    // `std::process` has no portable timed wait. This bounded 5 ms poll keeps the generated runtime
    // dependency-free while capping deadline overshoot; the child is killed and synchronously reaped.
    let started = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok((status, false)),
            Ok(None) if started.elapsed() < timeout => {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            Ok(None) => {
                child
                    .kill()
                    .map_err(|error| format!("cannot terminate timed-out fixture: {error}"))?;
                return child
                    .wait()
                    .map(|status| (status, true))
                    .map_err(|error| format!("cannot reap timed-out fixture: {error}"));
            }
            Err(error) => return Err(format!("cannot observe process fixture: {error}")),
        }
    }
}

pub fn terrane_test_spawn(
    artifact: String,
    arguments: terrane_collection_support::List<String>,
    environment: terrane_collection_support::List<String>,
    input: Vec<u8>,
    deadline_milliseconds: terrane_int_support::Int,
) -> TerranePlatformResult {
    use std::io::Write as _;
    use std::process::Stdio;
    use std::time::Duration;

    let Some(timeout) = terrane_int_support::checked_coerce::<u64>(&deadline_milliseconds) else {
        return TerranePlatformResult::error("process deadline does not fit milliseconds");
    };
    let environment = environment.into_iter().collect::<Vec<_>>();
    if !environment.len().is_multiple_of(2) {
        return TerranePlatformResult::error("process fixture environment is not paired");
    }
    let mut command = std::process::Command::new(artifact);
    command
        .args(arguments.into_iter().map(terrane_decode_platform_value))
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for pair in environment.chunks_exact(2) {
        command.env(
            terrane_decode_platform_value(pair[0].clone()),
            terrane_decode_platform_value(pair[1].clone()),
        );
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return TerranePlatformResult::error(format!("cannot spawn fixture: {error}"));
        }
    };
    let stdout = terrane_spawn_process_reader(child.stdout.take().expect("captured child stdout"));
    let stderr = terrane_spawn_process_reader(child.stderr.take().expect("captured child stderr"));
    if let Some(mut stdin) = child.stdin.take() {
        std::thread::spawn(move || {
            let _ = stdin.write_all(&input);
        });
    }
    let (status, deadline_exceeded) = match terrane_wait_fixture_child(
        &mut child,
        Duration::from_millis(timeout),
    ) {
        Ok(result) => result,
        Err(error) => return TerranePlatformResult::error(error),
    };
    let reader_deadline = Duration::from_millis(250);
    let output = match stdout.recv_timeout(reader_deadline) {
        Ok(output) => output,
        Err(_) => return TerranePlatformResult::error("fixture stdout did not close after exit"),
    };
    let error = match stderr.recv_timeout(reader_deadline) {
        Ok(error) => error,
        Err(_) => return TerranePlatformResult::error("fixture stderr did not close after exit"),
    };
    TerranePlatformResult {
        failed: false,
        deadline_exceeded,
        data: output,
        entries: vec![format!("raw:{}", terrane_hex(&error))],
        number: status.code().map_or(-1, i128::from),
        flag: status.code().is_none(),
        ..TerranePlatformResult::default()
    }
}

pub fn terrane_test_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}

pub fn terrane_test_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}

pub fn terrane_test_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}

pub fn terrane_test_result_exit_code(result: &TerranePlatformResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}

pub fn terrane_test_result_crashed(result: &TerranePlatformResult) -> bool {
    result.flag
}

pub fn terrane_test_result_stdout(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}

pub fn terrane_test_result_stderr(result: &TerranePlatformResult) -> Vec<u8> {
    result
        .entries
        .first()
        .and_then(|value| value.strip_prefix("raw:"))
        .map(terrane_unhex)
        .unwrap_or_default()
}

fn terrane_test_render_int(value: terrane_int_support::Int) -> String {
    value.to_string()
}

fn terrane_test_render_float64(value: f64) -> String {
    value.to_string()
}

fn terrane_test_render_bytes(value: Vec<u8>) -> String {
    format!("0x{}", terrane_hex(&value))
}

fn terrane_test_render_bool(value: bool) -> String {
    value.to_string()
}
fn terrane_test_deadline_nanoseconds() -> terrane_int_support::Int {
    let value = std::env::var("TERRANE_TEST_DEADLINE_NANOSECONDS")
        .ok()
        .and_then(|value| value.parse::<u128>().ok())
        .unwrap_or_default();
    terrane_int_support::Int::from_u128(value)
}

static TERRANE_TEST_TIME_NANOS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
static TERRANE_TEST_TIME_WAKERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<std::task::Waker>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

fn terrane_test_time_advance(nanoseconds: terrane_int_support::Int) -> bool {
    let Some(nanoseconds) = terrane_int_support::checked_coerce::<u64>(&nanoseconds) else {
        return false;
    };

    if TERRANE_TEST_TIME_NANOS
        .fetch_update(
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            |current| current.checked_add(nanoseconds),
        )
        .is_err()
    {
        return false;
    }
    for waker in TERRANE_TEST_TIME_WAKERS
        .lock()
        .expect("controlled time waker lock poisoned")
        .drain(..)
    {
        waker.wake();
    }
    true
}
