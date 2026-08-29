// Delivery principle 9: synchronization primitives require optimiser-visible ordering and host
// thread integration. Public object policy remains in `/standard/concurrency`.
#[allow(dead_code)] fn terrane_platform_int_channel(capacity: terrane_int_support::Int) -> TerranePlatformCapability {
    match terrane_platform_i128(&capacity, "channel capacity") {
        Ok(capacity) => terrane_platform_support::int_channel(capacity),
        Err(error) => error.capability.unwrap_or_default(),
    }
}
#[allow(dead_code)] fn terrane_platform_int_channel_send(channel: &TerranePlatformCapability, value: terrane_int_support::Int) -> TerranePlatformResult {
    let value = terrane_platform_i128!(value, "channel value");
    terrane_platform_support::int_channel_send(channel, value)
}
#[allow(dead_code)] fn terrane_platform_int_channel_receive(channel: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::int_channel_receive(channel)
}
#[allow(dead_code)] fn terrane_platform_int_channel_try_receive(channel: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::int_channel_try_receive(channel)
}
#[allow(dead_code)] fn terrane_platform_int_mutex(initial: terrane_int_support::Int) -> TerranePlatformCapability {
    match terrane_platform_i128(&initial, "mutex initial value") {
        Ok(initial) => terrane_platform_support::int_mutex(initial),
        Err(error) => error.capability.unwrap_or_default(),
    }
}
#[allow(dead_code)] fn terrane_platform_int_mutex_load(value: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::int_mutex_load(value)
}
#[allow(dead_code)] fn terrane_platform_int_mutex_store(value: &TerranePlatformCapability, replacement: terrane_int_support::Int) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "mutex value");
    terrane_platform_support::int_mutex_store(value, replacement)
}
#[allow(dead_code)] fn terrane_platform_int_mutex_add(value: &TerranePlatformCapability, amount: terrane_int_support::Int) -> TerranePlatformResult {
    let amount = terrane_platform_i128!(amount, "mutex update");
    terrane_platform_support::int_mutex_add(value, amount)
}
#[allow(dead_code)] fn terrane_platform_int_rw_lock(initial: terrane_int_support::Int) -> TerranePlatformCapability {
    match terrane_platform_i128(&initial, "read/write lock initial value") {
        Ok(initial) => terrane_platform_support::int_rw_lock(initial),
        Err(error) => error.capability.unwrap_or_default(),
    }
}
#[allow(dead_code)] fn terrane_platform_int_rw_lock_read(value: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::int_rw_lock_read(value)
}
#[allow(dead_code)] fn terrane_platform_int_rw_lock_write(value: &TerranePlatformCapability, replacement: terrane_int_support::Int) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "read/write lock value");
    terrane_platform_support::int_rw_lock_write(value, replacement)
}
#[allow(dead_code)] fn terrane_platform_atomic_int64(initial: i64) -> TerranePlatformCapability {
    terrane_platform_support::atomic_int64(i128::from(initial))
}
#[allow(dead_code)] fn terrane_platform_atomic_int64_load(value: &TerranePlatformCapability, ordering: String) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_load(value, &ordering)
}
#[allow(dead_code)] fn terrane_platform_atomic_int64_store(value: &TerranePlatformCapability, replacement: i64, ordering: String) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_store(value, i128::from(replacement), &ordering)
}
#[allow(dead_code)] fn terrane_platform_atomic_int64_add(value: &TerranePlatformCapability, amount: i64, ordering: String) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_add(value, i128::from(amount), &ordering)
}
#[allow(dead_code)] fn terrane_platform_thread_local_int(initial: terrane_int_support::Int) -> TerranePlatformCapability {
    match terrane_platform_i128(&initial, "thread-local initial value") {
        Ok(initial) => terrane_platform_support::thread_local_int(initial),
        Err(error) => error.capability.unwrap_or_default(),
    }
}
#[allow(dead_code)] fn terrane_platform_thread_local_int_get(value: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::thread_local_int_get(value)
}
#[allow(dead_code)] fn terrane_platform_thread_local_int_set(value: &TerranePlatformCapability, replacement: terrane_int_support::Int) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "thread-local value");
    terrane_platform_support::thread_local_int_set(value, replacement)
}
