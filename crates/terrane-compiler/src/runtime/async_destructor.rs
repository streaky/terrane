// Runtime handles remain entered after shutdown starts. Track admission and
// in-flight destructors per runtime, not just the presence of a current handle.
#[allow(
    dead_code,
    reason = "shared async support also serves packages without awaited destructors"
)]
const TERRANE_RUNTIME_CLOSING: usize = 1 << (usize::BITS - 1);

#[allow(
    dead_code,
    reason = "libraries can use async support without owning an executor"
)]
struct TerraneRuntimeLiveness {
    active: std::sync::atomic::AtomicUsize,
    drain: std::sync::Mutex<()>,
    changed: std::sync::Condvar,
}

thread_local! {
    static TERRANE_RUNTIME_LIVE: std::cell::RefCell<Option<(tokio::runtime::Id, std::sync::Arc<TerraneRuntimeLiveness>)>> =
        const { std::cell::RefCell::new(None) };
}

#[allow(
    dead_code,
    reason = "libraries can use awaited destruction without owning an executor"
)]
struct TerraneRuntimeLivenessGuard {
    live: std::sync::Arc<TerraneRuntimeLiveness>,
    previous: Option<(tokio::runtime::Id, std::sync::Arc<TerraneRuntimeLiveness>)>,
}

impl Drop for TerraneRuntimeLivenessGuard {
    fn drop(&mut self) {
        if self
            .live
            .active
            .fetch_or(TERRANE_RUNTIME_CLOSING, std::sync::atomic::Ordering::AcqRel)
            != 0
        {
            // A finalizer can enter immediately before main returns. Keep the
            // drivers live until it finishes, including on entrypoint unwind.
            let drained = self
                .live
                .drain
                .lock()
                .expect("destructor drain lock poisoned");
            let _drained = self
                .live
                .changed
                .wait_while(drained, |()| {
                    self.live.active.load(std::sync::atomic::Ordering::Acquire)
                        != TERRANE_RUNTIME_CLOSING
                })
                .expect("destructor drain lock poisoned");
        }
        TERRANE_RUNTIME_LIVE.with(|live| *live.borrow_mut() = self.previous.take());
    }
}

#[allow(
    dead_code,
    reason = "libraries can use awaited destruction without owning an executor"
)]
fn __terrane_runtime_liveness_enter(
    runtime: &tokio::runtime::Runtime,
    live: std::sync::Arc<TerraneRuntimeLiveness>,
) -> TerraneRuntimeLivenessGuard {
    let previous = TERRANE_RUNTIME_LIVE.with(|current| {
        current
            .borrow_mut()
            .replace((runtime.handle().id(), live.clone()))
    });
    TerraneRuntimeLivenessGuard { live, previous }
}

#[allow(
    dead_code,
    reason = "shared async support also serves packages without awaited destructors"
)]
struct TerraneDestructorPermit(std::sync::Arc<TerraneRuntimeLiveness>);

impl Drop for TerraneDestructorPermit {
    fn drop(&mut self) {
        if self
            .0
            .active
            .fetch_sub(1, std::sync::atomic::Ordering::AcqRel)
            == TERRANE_RUNTIME_CLOSING + 1
        {
            // Lock only for the last shutdown permit, pairing the notification
            // with the condition check so the root cannot miss a wakeup.
            let _drained = self.0.drain.lock().expect("destructor drain lock poisoned");
            self.0.changed.notify_one();
        }
    }
}

// Completion is synchronous: this future may borrow the object being dropped.
fn __terrane_await_destructor<F>(future: F)
where
    F: Future<Output = ()> + Send,
{
    let permit = tokio::runtime::Handle::try_current()
        .ok()
        .and_then(|handle| {
            if handle.runtime_flavor() != tokio::runtime::RuntimeFlavor::MultiThread {
                return None;
            }
            TERRANE_RUNTIME_LIVE.with(|current| {
                let current = current.borrow();
                let (id, live) = current.as_ref()?;
                if *id != handle.id() {
                    return None;
                }
                live.active
                    .try_update(
                        std::sync::atomic::Ordering::AcqRel,
                        std::sync::atomic::Ordering::Acquire,
                        |active| (active < TERRANE_RUNTIME_CLOSING - 1).then(|| active + 1),
                    )
                    .ok()?;
                Some((handle, TerraneDestructorPermit(live.clone())))
            })
        });
    if let Some((handle, _permit)) = permit {
        tokio::task::block_in_place(|| handle.block_on(future));
        return;
    }

    // Current-thread, foreign, off-runtime and shutdown-time destruction need
    // an independent driver. Never detach a future borrowed from a dropping value.
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Terrane destructor runtime must initialize")
                    .block_on(future);
            })
            .join()
            .expect("Terrane awaited destructor must not panic");
    });
}
