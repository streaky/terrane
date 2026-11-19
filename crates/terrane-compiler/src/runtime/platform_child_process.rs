fn terrane_child_capture_options<'a>(
    input: &'a [u8],
    timeout_ms: &terrane_int_support::Int,
    stdout_limit: &terrane_int_support::Int,
    stderr_limit: &terrane_int_support::Int,
) -> Result<terrane_platform_support::ChildCapture<'a>, TerranePlatformResult> {
    let timeout_ms = terrane_int_support::checked_coerce::<u64>(timeout_ms)
        .ok_or_else(|| TerranePlatformResult::error("child timeout must be a non-negative 64-bit integer"))?;
    let stdout_limit = stdout_limit.as_usize()
        .ok_or_else(|| TerranePlatformResult::error("stdout limit must be a non-negative platform-sized integer"))?;
    let stderr_limit = stderr_limit.as_usize()
        .ok_or_else(|| TerranePlatformResult::error("stderr limit must be a non-negative platform-sized integer"))?;
    Ok(terrane_platform_support::ChildCapture { input, timeout_ms, stdout_limit, stderr_limit })
}

pub async fn terrane_platform_child_spawn(
    program: &str,
    arguments: &[String],
    directory: &str,
    environment: &[String],
    remove_environment: &[String],
    inherit: bool,
) -> TerranePlatformResult {
    terrane_platform_support::child_spawn(terrane_platform_support::ChildLaunch {
        program,
        args: arguments,
        directory,
        env: environment,
        remove_env: remove_environment,
        inherit,
    }).await
}

pub fn terrane_platform_child_capture(
    program: &str,
    arguments: &[String],
    directory: &str,
    environment: &[String],
    remove_environment: &[String],
    inherit: bool,
    input: &[u8],
    timeout_ms: terrane_int_support::Int,
    stdout_limit: terrane_int_support::Int,
    stderr_limit: terrane_int_support::Int,
) -> TerranePlatformResult {
    let options = match terrane_child_capture_options(input, &timeout_ms, &stdout_limit, &stderr_limit) {
        Ok(options) => options,
        Err(error) => return error,
    };
    terrane_platform_support::child_capture(
        terrane_platform_support::ChildLaunch {
            program,
            args: arguments,
            directory,
            env: environment,
            remove_env: remove_environment,
            inherit,
        },
        options,
    )
}

pub async fn terrane_platform_child_capture_async(
    program: &str,
    arguments: &[String],
    directory: &str,
    environment: &[String],
    remove_environment: &[String],
    inherit: bool,
    input: &[u8],
    timeout_ms: terrane_int_support::Int,
    stdout_limit: terrane_int_support::Int,
    stderr_limit: terrane_int_support::Int,
) -> TerranePlatformResult {
    let options = match terrane_child_capture_options(input, &timeout_ms, &stdout_limit, &stderr_limit) {
        Ok(options) => options,
        Err(error) => return error,
    };
    terrane_platform_support::child_capture_async(
        terrane_platform_support::ChildLaunch {
            program,
            args: arguments,
            directory,
            env: environment,
            remove_env: remove_environment,
            inherit,
        },
        options,
    ).await
}

pub fn terrane_platform_child_poll(handle: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::child_poll(handle)
}

pub async fn terrane_platform_child_wait(
    handle: &TerranePlatformCapability,
    timeout_ms: terrane_int_support::Int,
) -> TerranePlatformResult {
    let Some(timeout_ms) = terrane_int_support::checked_coerce::<u64>(&timeout_ms) else {
        return TerranePlatformResult::error("child wait timeout must be a non-negative 64-bit integer");
    };
    terrane_platform_support::child_wait(handle, timeout_ms).await
}

pub fn terrane_platform_child_terminate(handle: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::child_terminate(handle)
}

pub async fn terrane_platform_child_write(
    handle: &TerranePlatformCapability,
    data: &[u8],
) -> TerranePlatformResult {
    terrane_platform_support::child_write(handle, data).await
}

pub async fn terrane_platform_child_close_input(
    handle: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::child_close_input(handle).await
}

pub async fn terrane_platform_child_read_stdout(
    handle: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
) -> TerranePlatformResult {
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformResult::error("stdout limit must be a non-negative platform-sized integer");
    };
    terrane_platform_support::child_read_stdout(handle, limit).await
}

pub async fn terrane_platform_child_read_stderr(
    handle: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
) -> TerranePlatformResult {
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformResult::error("stderr limit must be a non-negative platform-sized integer");
    };
    terrane_platform_support::child_read_stderr(handle, limit).await
}

pub fn terrane_platform_child_release(handle: &TerranePlatformCapability) {
    terrane_platform_support::child_release(handle);
}

pub fn terrane_platform_child_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
pub fn terrane_platform_child_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
pub fn terrane_platform_child_result_number(result: &TerranePlatformResult) -> terrane_int_support::Int {
    result.number.into()
}
pub fn terrane_platform_child_result_flag(result: &TerranePlatformResult) -> bool {
    result.flag
}
pub fn terrane_platform_child_result_secondary_flag(result: &TerranePlatformResult) -> bool {
    result.secondary_flag
}
pub fn terrane_platform_child_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
pub fn terrane_platform_child_result_data(result: TerranePlatformResult) -> Vec<u8> {
    result.data
}
pub fn terrane_platform_child_result_capability(result: &TerranePlatformResult) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
