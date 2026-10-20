use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

type Destructor = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Resource {
    finish: fn(Destructor),
    completed: Arc<AtomicUsize>,
    runtime: Option<tokio::runtime::Id>,
    reuse: bool,
}

impl Drop for Resource {
    fn drop(&mut self) {
        let completed = self.completed.clone();
        let expected = self.runtime;
        let reuse = self.reuse;
        (self.finish)(Box::pin(async move {
            let actual = tokio::runtime::Handle::current().id();
            if let Some(expected) = expected {
                if reuse {
                    assert_eq!(actual, expected, "live destruction reuses its runtime");
                } else {
                    assert_ne!(
                        actual, expected,
                        "shutdown/current-thread destruction needs a live driver"
                    );
                }
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
            completed.fetch_add(1, Ordering::Release);
        }));
    }
}

// Bound the whole process: an in-runtime timer cannot time out a hung Runtime::drop.
fn bounded(test: &str, proof: impl FnOnce()) {
    if std::env::var("TERRANE_DESTRUCTOR_PROBE").as_deref() == Ok(test) {
        proof();
        return;
    }
    let mut child = std::process::Command::new(std::env::current_exe().expect("test executable"))
        .args(["--exact", test, "--nocapture"])
        .env("TERRANE_DESTRUCTOR_PROBE", test)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("destructor probe starts");
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if child.try_wait().expect("probe status").is_some() {
            let output = child.wait_with_output().expect("probe output");
            assert!(
                output.status.success(),
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        if Instant::now() >= deadline {
            child.kill().expect("stop hanging probe");
            let output = child.wait_with_output().expect("reap hanging probe");
            panic!(
                "{test} hung: {}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn shutdown(run: impl FnOnce(Destructor), finish: fn(Destructor)) {
    let completed = Arc::new(AtomicUsize::new(0));
    let observed = completed.clone();
    run(Box::pin(async move {
        let runtime = Some(tokio::runtime::Handle::current().id());
        drop(Resource {
            finish,
            completed: observed.clone(),
            runtime,
            reuse: true,
        });
        let (started, ready) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            drop(Resource {
                finish,
                completed: observed.clone(),
                runtime,
                reuse: true,
            });
            let _pending = Resource {
                finish,
                completed: observed,
                runtime,
                reuse: false,
            };
            started.send(()).expect("root waits for pending resource");
            std::future::pending::<()>().await;
        });
        ready
            .await
            .expect("worker owns pending resource before shutdown");
    }));
    assert_eq!(
        completed.load(Ordering::Acquire),
        3,
        "runtime shutdown joins the pending resource's timer destructor"
    );
}

mod simple {
    use super::*;
    include!("../src/runtime/async.rs");
    include!("../src/runtime/async_destructor.rs");
    include!("../src/runtime/executor_parallel.rs");

    #[test]
    fn shutdown() {
        bounded("simple::shutdown", || {
            super::shutdown(__terrane_run, __terrane_await_destructor);
        });
    }

    #[test]
    fn shutdown_drains_active_drop() {
        bounded("simple::shutdown_drains_active_drop", || {
            struct ActiveDrop {
                live: Arc<TerraneRuntimeLiveness>,
                started: Option<tokio::sync::oneshot::Sender<()>>,
                completed: Arc<AtomicUsize>,
            }
            impl Drop for ActiveDrop {
                fn drop(&mut self) {
                    __terrane_await_destructor(async {
                        self.started
                            .take()
                            .expect("one drop")
                            .send(())
                            .expect("root waits");
                        std::future::poll_fn(|context| {
                            if self.live.active.load(Ordering::Acquire) & TERRANE_RUNTIME_CLOSING
                                != 0
                            {
                                std::task::Poll::Ready(())
                            } else {
                                context.waker().wake_by_ref();
                                std::task::Poll::Pending
                            }
                        })
                        .await;
                        tokio::time::sleep(Duration::from_millis(1)).await;
                        self.completed.fetch_add(1, Ordering::Release);
                    });
                }
            }
            let completed = Arc::new(AtomicUsize::new(0));
            let observed = completed.clone();
            __terrane_run(async move {
                let live = TERRANE_RUNTIME_LIVE
                    .with(|current| current.borrow().as_ref().expect("live root").1.clone());
                let (started, ready) = tokio::sync::oneshot::channel();
                tokio::spawn(async move {
                    drop(ActiveDrop {
                        live,
                        started: Some(started),
                        completed: observed,
                    });
                });
                ready.await.expect("destructor enters before shutdown");
            });
            assert_eq!(
                completed.load(Ordering::Acquire),
                1,
                "shutdown drains an already-admitted timer destructor"
            );
        });
    }

    #[test]
    fn fallback() {
        bounded("simple::fallback", || {
            let completed = Arc::new(AtomicUsize::new(0));
            drop(Resource {
                finish: __terrane_await_destructor,
                completed: completed.clone(),
                runtime: None,
                reuse: false,
            });
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("current-thread runtime");
            runtime.block_on(async {
                drop(Resource {
                    finish: __terrane_await_destructor,
                    completed: completed.clone(),
                    runtime: Some(tokio::runtime::Handle::current().id()),
                    reuse: false,
                });
            });
            assert_eq!(completed.load(Ordering::Acquire), 2);
        });
    }
}

#[expect(
    dead_code,
    reason = "embed the complete cancellation-aware runtime ABI"
)]
mod native {
    use super::*;
    #[derive(Clone, Debug)]
    pub struct TerraneError;
    include!("../src/runtime/time_inactive.rs");
    include!("../src/runtime/async_native.rs");
    include!("../src/runtime/async_destructor.rs");
    include!("../src/runtime/executor_parallel.rs");

    #[test]
    fn shutdown() {
        bounded("native::shutdown", || {
            super::shutdown(__terrane_run, __terrane_await_destructor);
        });
    }

    #[test]
    fn projected_cleanup_joins_drop() {
        bounded("native::projected_cleanup_joins_drop", || {
            __terrane_run(async {
                let completed = Arc::new(AtomicUsize::new(0));
                let observed = completed.clone();
                let (started, ready) = tokio::sync::oneshot::channel();
                let mut entry = Box::pin(__terrane_projected_async_entry(async move {
                    let _resource = Resource {
                        finish: __terrane_await_destructor,
                        completed: observed,
                        runtime: Some(tokio::runtime::Handle::current().id()),
                        reuse: true,
                    };
                    started.send(()).expect("entry begins");
                    std::future::pending::<()>().await;
                }));
                std::future::poll_fn(|context| {
                    assert!(entry.as_mut().poll(context).is_pending());
                    std::task::Poll::Ready(())
                })
                .await;
                ready
                    .await
                    .expect("entry owns resource before cancellation");
                drop(entry);
                __terrane_wait_projected_cleanups().await;
                assert_eq!(
                    completed.load(Ordering::Acquire),
                    1,
                    "cleanup wait includes the aborted future's destructor"
                );
            });
        });
    }
}
