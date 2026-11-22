use std::sync::{Mutex, mpsc};

const MIN_ITEMS: usize = 4;
const MIN_WORK: usize = 64 * 1024;
const MAX_WORKERS: usize = 8;
const WORKER_STACK_SIZE: usize = 8 * 1024 * 1024;

/// Run independent work in parallel and return results in input order.
pub(crate) fn map_ordered<I, T>(
    jobs: I,
    work: usize,
    operation: impl Fn(I::Item) -> T + Sync,
) -> Vec<T>
where
    I: ExactSizeIterator + Send,
    T: Send,
{
    let workers = worker_count(jobs.len(), work);
    if workers < 2 {
        return jobs.map(operation).collect();
    }
    map_parallel(jobs, workers, operation)
}

/// Preserve serial short-circuiting for small batches and select the first
/// source-ordered failure, not the first worker to report a failure.
pub(super) fn try_for_each_unit<U: Send, E: Send>(
    mut jobs: impl ExactSizeIterator<Item = U> + Send,
    work: usize,
    operation: impl Fn(U) -> Result<(), E> + Sync,
) -> Result<(), E> {
    let workers = worker_count(jobs.len(), work);
    if workers < 2 {
        return jobs.try_for_each(operation);
    }
    map_parallel(jobs, workers, operation).into_iter().collect()
}

pub(super) fn source_work(units: &[super::SemanticUnit]) -> usize {
    units.iter().map(|unit| unit.source.text().len()).sum()
}

fn worker_count(len: usize, work: usize) -> usize {
    #[cfg(test)]
    let work_limit_active = crate::semantics::expressions::inference_work_limit_active();
    #[cfg(not(test))]
    let work_limit_active = false;
    if len < MIN_ITEMS || work < MIN_WORK || work_limit_active {
        return 1;
    }
    std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(MAX_WORKERS)
        .min(len)
}

fn map_parallel<I, T>(jobs: I, workers: usize, operation: impl Fn(I::Item) -> T + Sync) -> Vec<T>
where
    I: ExactSizeIterator + Send,
    T: Send,
{
    let len = jobs.len();
    let cache_enabled = crate::cache_scope::enabled();
    let reporting = crate::compilation_progress::reporting();
    let jobs = Mutex::new(jobs.enumerate());
    let (sender, receiver) = mpsc::channel();
    let mut results = std::iter::repeat_with(|| None)
        .take(len)
        .collect::<Vec<_>>();
    std::thread::scope(|scope| {
        for worker in 0..workers {
            let sender = sender.clone();
            let operation = &operation;
            let jobs = &jobs;
            if std::thread::Builder::new()
                .stack_size(WORKER_STACK_SIZE)
                .spawn_scoped(scope, move || {
                    crate::cache_scope::with_enabled(cache_enabled, || {
                        crate::compilation_progress::with_reporting(reporting, || {
                            loop {
                                let job = jobs.lock().expect("semantic work queue poisoned").next();
                                let Some((index, job)) = job else { break };
                                if sender.send((index, operation(job))).is_err() {
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
                    for (index, job) in jobs.lock().expect("semantic work queue poisoned").by_ref()
                    {
                        results[index] = Some(operation(job));
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
