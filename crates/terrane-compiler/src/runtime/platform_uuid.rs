fn terrane_platform_uuid_parse(text: String) -> TerranePlatformResult { terrane_platform_support::uuid_parse(&text) }
fn terrane_platform_uuid_v4(source: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::uuid_v4(source) }
fn terrane_platform_uuid_v7(source: &TerranePlatformCapability, unix_milliseconds: terrane_int_support::Int) -> TerranePlatformResult { terrane_platform_support::uuid_v7(source, terrane_platform_i128!(unix_milliseconds, "UUID v7 timestamp")) }
