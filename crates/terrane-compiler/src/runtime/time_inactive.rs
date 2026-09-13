#[allow(dead_code, reason = "native cancellation omits deadline polling when no clock package is used")]
fn terrane_time_deadline_expired(_target: &terrane_int_support::Int) -> bool {
    false
}

#[allow(dead_code, reason = "native cancellation omits deadline polling when no clock package is used")]
async fn terrane_time_sleep_until(_target: terrane_int_support::Int) {
    std::future::pending::<()>().await;
}
