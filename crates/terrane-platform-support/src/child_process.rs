use super::{Capability, CapabilityInner, ResultValue};
use rustix::process::{Pid, Signal, kill_process};
use std::ffi::OsString;
use std::io;
use std::process::{ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use tokio::sync::{Mutex as AsyncMutex, mpsc, oneshot, watch};

#[derive(Clone, Copy)]
pub struct ChildLaunch<'a> {
    pub program: &'a str,
    pub args: &'a [String],
    pub directory: &'a str,
    pub env: &'a [String],
    pub remove_env: &'a [String],
    pub inherit: bool,
    pub new_process_group: bool,
}

#[derive(Clone, Copy)]
pub struct ChildCapture<'a> {
    pub input: &'a [u8],
    pub timeout_ms: u64,
    pub stdout_limit: usize,
    pub stderr_limit: usize,
}

type Outcome = Result<ExitStatus, String>;

enum SignalError {
    Exited,
    Io(io::Error),
}

type SignalRequest = (Signal, oneshot::Sender<Result<(), SignalError>>);

const CHILD_EXITED: &str = "child process has exited";

pub(super) struct ChildState {
    stdin: AsyncMutex<Option<ChildStdin>>,
    stdout: AsyncMutex<ChildStdout>,
    stderr: AsyncMutex<ChildStderr>,
    status: watch::Receiver<Option<Outcome>>,
    terminate: Mutex<Option<oneshot::Sender<()>>>,
    signals: mpsc::Sender<SignalRequest>,
}

fn native_value(value: &str) -> io::Result<OsString> {
    use std::os::unix::ffi::OsStringExt as _;

    if let Some(text) = value.strip_prefix("text:") {
        return Ok(OsString::from(text));
    }
    let invalid = || io::Error::new(io::ErrorKind::InvalidInput, "invalid native process value");
    let decoded = super::hex_decode(value.strip_prefix("raw:").ok_or_else(invalid)?);
    if decoded.failed {
        return Err(invalid());
    }
    Ok(OsString::from_vec(decoded.data))
}

//# SECURITY-BEGIN [process.direct-execution]
fn launch(request: ChildLaunch<'_>) -> io::Result<Child> {
    let mut command = Command::new(request.program);
    command.args(request.args);
    if !request.directory.is_empty() {
        command.current_dir(request.directory);
    }
    if request.new_process_group {
        command.process_group(0);
    }
    if !request.inherit {
        command.env_clear();
    }
    let (entries, remainder) = request.env.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unpaired environment entry",
        ));
    }
    for [name, value] in entries {
        command.env(native_value(name)?, native_value(value)?);
    }
    for name in request.remove_env {
        command.env_remove(native_value(name)?);
    }
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
}
//# SECURITY-END [process.direct-execution]

fn child_state(cap: &Capability) -> Result<&ChildState, ResultValue> {
    match cap.0.as_ref() {
        CapabilityInner::ChildProcess(state) => Ok(state),
        other => Err(super::capability_type_error(other, "child process")),
    }
}

fn observation(status: ExitStatus) -> ResultValue {
    let code = status.code();
    ResultValue {
        number: code.map_or(-1, i128::from),
        flag: true,
        secondary_flag: code.is_none(),
        ..ResultValue::default()
    }
}

async fn terminate_and_reap(child: &mut Child) -> io::Result<ExitStatus> {
    if let Some(status) = child.try_wait()? {
        return Ok(status);
    }
    if let Err(error) = child.start_kill() {
        return child.try_wait()?.ok_or(error);
    }
    child.wait().await
}

//# SECURITY-BEGIN [process.owned-child-signalling]
fn signal_child(child: &mut Child, signal: Signal) -> Result<(), SignalError> {
    // The supervisor is the sole reaper for this Child. It cannot poll wait
    // again until delivery returns, so an intervening exit leaves the PID reserved.
    if child.try_wait().map_err(SignalError::Io)?.is_some() {
        return Err(SignalError::Exited);
    }
    let pid = child
        .id()
        .and_then(|id| i32::try_from(id).ok())
        .and_then(Pid::from_raw)
        .ok_or_else(|| SignalError::Io(io::Error::other("missing child process ID")))?;
    kill_process(pid, signal).map_err(|error| SignalError::Io(error.into()))
}
//# SECURITY-END [process.owned-child-signalling]

//# SECURITY-BEGIN [process.owned-child-signalling]
// Async because pipe registration and child supervision require a live reactor.
pub async fn child_spawn(request: ChildLaunch<'_>) -> ResultValue {
    let mut child = match launch(request) {
        Ok(child) => child,
        Err(error) => return ResultValue::error(format!("cannot spawn child: {error}")),
    };
    let stdin = child.stdin.take().expect("piped child stdin");
    let stdout = child.stdout.take().expect("piped child stdout");
    let stderr = child.stderr.take().expect("piped child stderr");
    let (status_tx, status) = watch::channel(None);
    let (terminate, mut terminated) = oneshot::channel();
    let (signals, mut pending_signals) = mpsc::channel::<SignalRequest>(1);
    // Wait, termination, and signal delivery share exclusive ownership of the Child.
    tokio::spawn(async move {
        let result = loop {
            tokio::select! {
                biased;
                status = child.wait() => break status,
                _ = &mut terminated => break terminate_and_reap(&mut child).await,
                Some((signal, reply)) = pending_signals.recv() => {
                    let _ = reply.send(signal_child(&mut child, signal));
                }
            }
        };
        status_tx.send_replace(Some(result.map_err(|error| error.to_string())));
    });
    ResultValue {
        capability: Some(Capability(Arc::new(CapabilityInner::ChildProcess(
            Box::new(ChildState {
                stdin: AsyncMutex::new(Some(stdin)),
                stdout: AsyncMutex::new(stdout),
                stderr: AsyncMutex::new(stderr),
                status,
                terminate: Mutex::new(Some(terminate)),
                signals,
            }),
        )))),
        ..ResultValue::default()
    }
}
//# SECURITY-END [process.owned-child-signalling]

fn observed(status: &Outcome) -> ResultValue {
    match status {
        Ok(exit) => observation(*exit),
        Err(error) => ResultValue::error(error.clone()),
    }
}

pub fn child_poll(cap: &Capability) -> ResultValue {
    let state = match child_state(cap) {
        Ok(state) => state,
        Err(error) => return error,
    };
    if let Some(outcome) = state.status.borrow().as_ref() {
        return observed(outcome);
    }
    if state.status.has_changed().is_err() {
        return state.status.borrow().as_ref().map_or_else(
            || ResultValue::error("child process supervisor stopped"),
            observed,
        );
    }
    ResultValue::default()
}

pub async fn child_wait(cap: &Capability, timeout_ms: u64) -> ResultValue {
    let mut status = match child_state(cap) {
        Ok(state) => state.status.clone(),
        Err(error) => return error,
    };
    let observe = async {
        loop {
            if let Some(outcome) = status.borrow_and_update().as_ref() {
                return observed(outcome);
            }
            if status.changed().await.is_err() {
                return ResultValue::error("child process supervisor stopped");
            }
        }
    };
    match tokio::time::timeout(Duration::from_millis(timeout_ms), observe).await {
        Ok(result) => result,
        Err(_) => ResultValue {
            deadline_exceeded: true,
            ..ResultValue::default()
        },
    }
}

pub fn child_terminate(cap: &Capability) -> ResultValue {
    let state = match child_state(cap) {
        Ok(state) => state,
        Err(error) => return error,
    };
    let mut slot = state
        .terminate
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(request) = slot.take() {
        let _ = request.send(());
    }
    ResultValue::default()
}

//# SECURITY-BEGIN [process.owned-child-signalling]
// Resolve the opaque child capability, never a caller-supplied PID.
pub async fn child_send_signal(cap: &Capability, signal: &str) -> ResultValue {
    let state = match child_state(cap) {
        Ok(state) => state,
        Err(error) => return error,
    };
    let signal = match signal {
        "interrupt" => Signal::INT,
        "terminate" => Signal::TERM,
        "hangup" => Signal::HUP,
        "quit" => Signal::QUIT,
        _ => return ResultValue::error("invalid child process signal"),
    };
    let stopped = || {
        let outcome = state.status.borrow();
        match outcome.as_ref() {
            Some(Ok(_)) => ResultValue::error(CHILD_EXITED),
            Some(Err(error)) => ResultValue::error(error.clone()),
            None => ResultValue::error("child process supervisor stopped"),
        }
    };
    let (reply, response) = oneshot::channel();
    if state.signals.send((signal, reply)).await.is_err() {
        return stopped();
    }
    match response.await {
        Ok(Ok(())) => ResultValue::default(),
        Ok(Err(SignalError::Exited)) => ResultValue::error(CHILD_EXITED),
        Ok(Err(SignalError::Io(error))) => {
            ResultValue::error(format!("cannot signal child: {error}"))
        }
        Err(_) => stopped(),
    }
}
//# SECURITY-END [process.owned-child-signalling]

pub fn child_release(cap: &Capability) {
    let _ = child_terminate(cap);
}

pub async fn child_write(cap: &Capability, data: &[u8]) -> ResultValue {
    let state = match child_state(cap) {
        Ok(state) => state,
        Err(error) => return error,
    };
    let mut stdin = state.stdin.lock().await;
    let Some(stdin) = stdin.as_mut() else {
        return ResultValue::error("child stdin is closed");
    };
    match stdin.write(data).await {
        Ok(written) => ResultValue {
            number: written as i128,
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("cannot write child stdin: {error}")),
    }
}

pub async fn child_close_input(cap: &Capability) -> ResultValue {
    let state = match child_state(cap) {
        Ok(state) => state,
        Err(error) => return error,
    };
    state.stdin.lock().await.take();
    ResultValue::default()
}

async fn read_lane<R: AsyncRead + Unpin>(pipe: &AsyncMutex<R>, limit: usize) -> ResultValue {
    if limit == 0 {
        return ResultValue::default();
    }
    // `read` returns a chunk, not exactly `limit` bytes; cap the allocation.
    let mut data = vec![0; limit.min(64 * 1024)];
    match pipe.lock().await.read(&mut data).await {
        Ok(count) => {
            data.truncate(count);
            ResultValue {
                data,
                flag: count == 0,
                ..ResultValue::default()
            }
        }
        Err(error) => ResultValue::error(format!("cannot read child output: {error}")),
    }
}

pub async fn child_read_stdout(cap: &Capability, limit: usize) -> ResultValue {
    match child_state(cap) {
        Ok(state) => read_lane(&state.stdout, limit).await,
        Err(error) => error,
    }
}

pub async fn child_read_stderr(cap: &Capability, limit: usize) -> ResultValue {
    match child_state(cap) {
        Ok(state) => read_lane(&state.stderr, limit).await,
        Err(error) => error,
    }
}

//# SECURITY-BEGIN [process.bounded-capture]
#[derive(Default)]
struct Captured {
    data: Vec<u8>,
    truncated: bool,
}

async fn drain(
    mut reader: impl AsyncRead + Unpin,
    limit: usize,
    output: &mut Captured,
) -> io::Result<()> {
    let mut buffer = vec![0; 8192];
    loop {
        let count = reader.read(&mut buffer).await?;
        if count == 0 {
            return Ok(());
        }
        let retained = count.min(limit - output.data.len());
        let required = output.data.len() + retained;
        if required > output.data.capacity() {
            let capacity = required
                .max(output.data.capacity().saturating_mul(2))
                .min(limit);
            output
                .data
                .try_reserve_exact(capacity - output.data.len())
                .map_err(io::Error::other)?;
        }
        output.data.extend_from_slice(&buffer[..retained]);
        output.truncated |= retained != count;
    }
}

pub async fn child_capture_async(
    request: ChildLaunch<'_>,
    options: ChildCapture<'_>,
) -> ResultValue {
    let mut child = match launch(request) {
        Ok(child) => child,
        Err(error) => return ResultValue::error(format!("cannot spawn child: {error}")),
    };
    let mut stdin = child.stdin.take().expect("piped child stdin");
    let stdout = child.stdout.take().expect("piped child stdout");
    let stderr = child.stderr.take().expect("piped child stderr");
    let mut output = Captured::default();
    let mut errors = Captured::default();
    let capture = async {
        let write_input = async move {
            match stdin.write_all(options.input).await {
                Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
                result => result,
            }
        };
        tokio::try_join!(
            write_input,
            drain(stdout, options.stdout_limit, &mut output),
            drain(stderr, options.stderr_limit, &mut errors),
            child.wait(),
        )
        .map(|((), (), (), status)| status)
    };
    let (status, timed_out) =
        match tokio::time::timeout(Duration::from_millis(options.timeout_ms), capture).await {
            Ok(Ok(status)) => (status, false),
            Ok(Err(error)) => {
                let _ = terminate_and_reap(&mut child).await;
                return ResultValue::error(format!("child capture failed: {error}"));
            }
            Err(_) => match terminate_and_reap(&mut child).await {
                Ok(status) => (status, true),
                Err(error) => {
                    return ResultValue::error(format!("cannot reap timed-out child: {error}"));
                }
            },
        };
    let observed = observation(status);
    ResultValue {
        number: observed.number,
        flag: observed.secondary_flag,
        deadline_exceeded: timed_out,
        data: output.data,
        secondary_data: errors.data,
        truncated: output.truncated,
        secondary_flag: errors.truncated,
        ..ResultValue::default()
    }
}
//# SECURITY-END [process.bounded-capture]

pub fn child_capture(request: ChildLaunch<'_>, options: ChildCapture<'_>) -> ResultValue {
    let capture = || match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime.block_on(child_capture_async(request, options)),
        Err(error) => {
            ResultValue::error(format!("cannot initialize child process runtime: {error}"))
        }
    };
    if tokio::runtime::Handle::try_current().is_err() {
        return capture();
    }
    std::thread::scope(
        |scope| match std::thread::Builder::new().spawn_scoped(scope, capture) {
            Ok(worker) => worker
                .join()
                .unwrap_or_else(|_| ResultValue::error("child capture worker panicked")),
            Err(error) => ResultValue::error(format!("cannot start child capture worker: {error}")),
        },
    )
}
