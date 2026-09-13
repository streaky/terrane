fn terrane_platform_time_wall() -> TerranePlatformResult {
    terrane_platform_support::wall_time()
}

fn terrane_platform_time_wall_seconds(result: &TerranePlatformResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}

fn terrane_platform_time_wall_nanoseconds(result: &TerranePlatformResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.secondary_number)
}

fn terrane_platform_time_domain() -> terrane_int_support::Int {
    terrane_time_domain()
}

fn terrane_platform_time_monotonic() -> terrane_int_support::Int {
    terrane_time_monotonic()
}

async fn terrane_platform_time_sleep_until(target: terrane_int_support::Int) {
    terrane_time_sleep_until(target).await;
}


trait TerraneTimeDivisor {
    fn into_time_int(self) -> terrane_int_support::Int;
}

impl TerraneTimeDivisor for i32 {
    fn into_time_int(self) -> terrane_int_support::Int {
        terrane_int_support::Int::from(i64::from(self))
    }
}

impl TerraneTimeDivisor for terrane_int_support::Int {
    fn into_time_int(self) -> terrane_int_support::Int {
        self
    }
}

fn terrane_platform_time_div(
    value: &terrane_int_support::Int,
    divisor: impl TerraneTimeDivisor,
) -> terrane_int_support::Int {
    value
        .euclidean_div(&divisor.into_time_int())
        .expect("bundled time arithmetic divisors must be nonzero")
}
fn terrane_platform_time_mod(
    value: &terrane_int_support::Int,
    divisor: impl TerraneTimeDivisor,
) -> terrane_int_support::Int {
    value
        .modulo(&divisor.into_time_int())
        .expect("bundled time arithmetic divisors must be nonzero")
}