fn terrane_platform_secure_random() -> TerranePlatformCapability { terrane_platform_support::secure_random() }
fn terrane_platform_pseudo_random(algorithm: String, seed: Vec<u8>) -> TerranePlatformCapability { terrane_platform_support::pseudo_random(&algorithm, &seed) }
fn terrane_platform_secret_buffer(data: Vec<u8>) -> TerranePlatformCapability { terrane_platform_support::secret_buffer(data) }
fn terrane_platform_destroy_secret(secret: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::destroy_secret(secret) }
fn terrane_platform_random_bytes(source: &TerranePlatformCapability, count: terrane_int_support::Int) -> TerranePlatformResult { terrane_platform_support::random_bytes(source, terrane_platform_i128!(count, "random byte count")) }
fn terrane_platform_random_bounded(source: &TerranePlatformCapability, upper: terrane_int_support::Int) -> TerranePlatformResult { terrane_platform_support::random_bounded(source, terrane_platform_i128!(upper, "random upper bound")) }
fn terrane_platform_random_split(source: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::random_split(source) }
fn terrane_platform_digest(algorithm: &String, data: Vec<u8>) -> TerranePlatformResult { terrane_platform_support::digest(algorithm, &data) }
fn terrane_platform_hmac(algorithm: &String, key: &TerranePlatformCapability, data: Vec<u8>) -> TerranePlatformResult { terrane_platform_support::hmac(algorithm, key, &data) }
fn terrane_platform_constant_time_equal(left: Vec<u8>, right: Vec<u8>) -> bool { terrane_platform_support::constant_time_equal(&left, &right) }
