// Delivery principle 9: the support crate owns the platform ABI and translates host failures.
// This crossing copies the host name into an owned string; no borrowed data or host handle escapes.
#[allow(dead_code)] fn terrane_platform_system_host_name() -> TerranePlatformResult {
    terrane_platform_support::system_host_name()
}
