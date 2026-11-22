use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc,
};

const MIN_ITEMS: usize = 4;
const MIN_WORK: usize = 64 * 1024;
const MAX_WORKERS: usize = 8;
const WORKER_STACK_SIZE: usize = 8 * 1024 * 1024;

/// Run independent work against one read-only snapshot, preserving source order
/// for the caller's reconciliation and diagnostic selection.
pub(crate) fn map_ordered<T: Send>(
    len: usize,
    work: usize,
    operation: impl Fn(usize) -> T + Sync,
) -> Vec<T> {
    #[cfg(test)]
    let work_limit_active = crate::semantics::expressions::inference_work_limit_active();
    #[cfg(not(test))]
    let work_limit_active = false;
    if len < MIN_ITEMS || work < MIN_WORK || work_limit_active {
        return (0..len).map(operation).collect();
    }
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(MAX_WORKERS)
        .min(len);
    if workers < 2 {
        return (0..len).map(operation).collect();
    }

    let cache_enabled = crate::cache_scope::enabled();
    let reporting = crate::compilation_progress::reporting();
    let next = AtomicUsize::new(0);
    let (sender, receiver) = mpsc::channel();
    let mut results = std::iter::repeat_with(|| None)
        .take(len)
        .collect::<Vec<_>>();
    std::thread::scope(|scope| {
        for worker in 0..workers {
            let sender = sender.clone();
            let operation = &operation;
            let next = &next;
            if std::thread::Builder::new()
                .stack_size(WORKER_STACK_SIZE)
                .spawn_scoped(scope, move || {
                    crate::cache_scope::with_enabled(cache_enabled, || {
                        crate::compilation_progress::with_reporting(reporting, || {
                            loop {
                                let index = next.fetch_add(1, Ordering::Relaxed);
                                if index >= len || sender.send((index, operation(index))).is_err() {
                                    break;
                                }
                            }
                        });
                    });
                })
                .is_err()
            {
                if worker == 0 {
                    // No job has been claimed; let the caller do the whole batch.
                    for (index, result) in results.iter_mut().enumerate() {
                        *result = Some(operation(index));
                    }
                }
                break;
            }
        }
        drop(sender);
        for (index, result) in receiver {
            results[index] = Some(result);
        }
    });
    results
        .into_iter()
        .map(|result| result.expect("semantic analysis result missing"))
        .collect()
}
