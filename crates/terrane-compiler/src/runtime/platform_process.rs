
pub fn terrane_unhex(text: &str) -> Vec<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .filter_map(|pair| Some((digit(pair[0])? << 4) | digit(pair[1])?))
        .collect()
}

fn terrane_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}

pub fn terrane_platform_value(value: std::ffi::OsString) -> String {
    terrane_platform_support::platform_value(value)
}


pub fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}

pub fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}

pub fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value
        .strip_prefix("raw:")
        .map(terrane_unhex)
        .unwrap_or_default()
}

pub fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}

pub fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [terrane_platform_value(name), terrane_platform_value(value)])
        .collect()
}

pub fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}

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

pub fn terrane_test_spawn(
    artifact: String,
    arguments: terrane_collection_support::List<String>,
    environment: terrane_collection_support::List<String>,
    input: Vec<u8>,
    deadline_milliseconds: terrane_int_support::Int,
) -> TerranePlatformResult {
    use std::io::Write as _;
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let Some(timeout) = terrane_int_support::checked_coerce::<u64>(&deadline_milliseconds) else {
        return TerranePlatformResult::error("process deadline does not fit milliseconds");
    };
    let mut command = std::process::Command::new(artifact);
    command
        .args(arguments.into_iter().map(terrane_decode_platform_value))
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let environment = environment.into_iter().collect::<Vec<_>>();
    for pair in environment.chunks_exact(2) {
        command.env(
            terrane_decode_platform_value(pair[0].clone()),
            terrane_decode_platform_value(pair[1].clone()),
        );
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => return TerranePlatformResult::error(format!("cannot spawn fixture: {error}")),
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(&input);
    }
    let stdout = child.stdout.take().expect("captured child stdout");
    let stderr = child.stderr.take().expect("captured child stderr");
    let stdout = std::thread::spawn(move || terrane_read_process_output(stdout));
    let stderr = std::thread::spawn(move || terrane_read_process_output(stderr));
    let started = Instant::now();
    let (status, deadline_exceeded) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (Some(status), false),
            Ok(None) if started.elapsed() < Duration::from_millis(timeout) => {
                std::thread::sleep(Duration::from_millis(5));
            }
            Ok(None) => {
                let _ = child.kill();
                break (child.wait().ok(), true);
            }
            Err(error) => {
                return TerranePlatformResult::error(format!(
                    "cannot observe process fixture: {error}"
                ));
            }
        }
    };
    let output = stdout.join().unwrap_or_default();
    let error = stderr.join().unwrap_or_default();
    TerranePlatformResult {
        failed: false,
        deadline_exceeded,
        data: output,
        entries: vec![format!("raw:{}", terrane_hex(&error))],
        number: status
            .as_ref()
            .and_then(std::process::ExitStatus::code)
            .map_or(-1, i128::from),
        flag: status.is_some_and(|status| status.code().is_none()),
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

static TERRANE_TEST_TIME_NANOS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
static TERRANE_TEST_TIME_WAKERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<std::task::Waker>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

fn terrane_test_time_advance(nanoseconds: terrane_int_support::Int) -> bool {
    let Some(nanoseconds) = terrane_int_support::checked_coerce::<u64>(&nanoseconds) else {
        return false;
    };
    TERRANE_TEST_TIME_NANOS.fetch_add(nanoseconds, std::sync::atomic::Ordering::AcqRel);
    for waker in TERRANE_TEST_TIME_WAKERS
        .lock()
        .expect("controlled time waker lock poisoned")
        .drain(..)
    {
        waker.wake();
    }
    true
}
