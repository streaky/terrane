#[cfg(not(unix))]
compile_error!("`/core/process-signals` is unsupported on this host family");

fn terrane_process_signal_subscribe(
    names: terrane_collection_support::Set<ProcessSignal>,
) -> TerranePlatformResult {
    let mut iterator = terrane_collection_support::Iterable::terrane_iterator(&names);
    let mut signal_names = Vec::new();
    loop {
        match iterator.next() {
            terrane_collection_support::IterationStep::Item(signal) => {
                signal_names.push(signal.name);
            }
            terrane_collection_support::IterationStep::End => break,
        }
    }
    terrane_platform_support::process_signal_subscribe(&signal_names)
}

async fn terrane_process_signal_next(
    capability: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::process_signal_next(capability).await
}

fn terrane_process_signal_close(capability: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::process_signal_close(capability)
}

fn terrane_process_signal_result_exact_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from_decimal(&result.exact_number)
}

fn terrane_process_signal_result_observed(
    result: &TerranePlatformResult,
) -> MonotonicInstant {
    MonotonicInstant {
        domain: terrane_time_domain(),
        elapsed_nanoseconds: terrane_int_support::Int::from_decimal(
            &result.secondary_exact_number,
        ),
    }
}
