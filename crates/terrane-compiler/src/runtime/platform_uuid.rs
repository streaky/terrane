fn terrane_platform_uuid_parse(text: String) -> TerranePlatformResult { terrane_platform_support::uuid_parse(&text) }
fn terrane_platform_uuid_v4(source: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::uuid_v4(source) }
fn terrane_platform_uuid_v7() -> TerranePlatformResult { terrane_platform_support::uuid_v7() }
