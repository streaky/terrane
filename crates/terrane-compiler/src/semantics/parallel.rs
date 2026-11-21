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
pub(super) fn map_ordered<T: Send>(
    len: usize,
    work: usize,
    operation: impl Fn(usize) -> T + Sync,
) -> Vec<T> {
    if len < MIN_ITEMS || work < MIN_WORK {
        return (0..len).map(operation).collect();
    }
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(MAX_WORKERS)
        .min(len);
    if workers < 2 {
        return (0..len).map(operation).collect();
    }

    let next = AtomicUsize::new(0);
    let (sender, receiver) = mpsc::channel();
    let mut results = std::iter::repeat_with(|| None)
        .take(len)
        .collect::<Vec<_>>();
    std::thread::scope(|scope| {
        for _ in 0..workers {
            let sender = sender.clone();
            let operation = &operation;
            let next = &next;
            std::thread::Builder::new()
                .stack_size(WORKER_STACK_SIZE)
                .spawn_scoped(scope, move || {
                    loop {
                        let index = next.fetch_add(1, Ordering::Relaxed);
                        if index >= len {
                            break;
                        }
                        if sender.send((index, operation(index))).is_err() {
                            break;
                        }
                    }
                })
                .expect("failed to spawn semantic analysis worker");
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
