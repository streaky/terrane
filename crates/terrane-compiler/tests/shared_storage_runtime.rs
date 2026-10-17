include!("../src/runtime/shared_storage.rs");

fn conflict(operation: impl FnOnce()) {
    let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation))
        .expect_err("incompatible reentry must fail");
    let message = error
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| error.downcast_ref::<String>().map(String::as_str))
        .expect("borrow conflict must have a diagnostic");
    assert!(
        message.contains("shared reference borrow conflict"),
        "{message}"
    );
}

fn exercise_projection() {
    let cell = std::sync::Arc::new(TerraneSharedCell::new((1, 2)));
    let projected = TerraneSharedProjection::new(
        cell.clone(),
        |value: &(i32, i32)| &value.1,
        |value: &mut (i32, i32)| &mut value.1,
    );
    let mut guard = projected.lock().unwrap();
    *guard += 3;
    conflict(|| drop(cell.lock()));
    drop(guard);
    assert_eq!(*cell.lock().unwrap(), (1, 5));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut guard = projected.lock().unwrap();
        *guard = 9;
        panic!("deliberate projected unwind");
    }));
    assert!(result.is_err());
    let mut recovered = match projected.lock() {
        Ok(_) => panic!("projection must preserve poison"),
        Err(error) => error.into_inner(),
    };
    assert_eq!(*recovered, 9);
    *recovered = 12;
    drop(recovered);
    assert_eq!(*cell.as_ref().lock().unwrap_err().into_inner(), (1, 12));
}

fn exercise(mode: &str) {
    match mode {
        "nested" => {
            let cell = TerraneSharedCell::new(1);
            let guard = cell.lock().unwrap();
            conflict(|| {
                drop(cell.lock());
            });
            drop(guard);
            *cell.lock().unwrap() += 2;
            assert_eq!(*cell.lock().unwrap(), 3);
        }
        "cycle" => {
            let a = TerraneSharedCell::new(1);
            let b = TerraneSharedCell::new(2);
            let first = a.lock().unwrap();
            let second = b.lock().unwrap();
            conflict(|| {
                drop(a.lock());
            });
            assert_eq!(*first + *second, 3);
            drop(second);
            drop(first);
            assert_eq!(*a.lock().unwrap(), 1);
            assert_eq!(*b.lock().unwrap(), 2);
        }
        "poison" => {
            let cell = TerraneSharedCell::new(0);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut guard = cell.lock().unwrap();
                *guard = 9;
                panic!("deliberate unwind");
            }));
            assert!(result.is_err());
            let mut recovered = cell.lock().unwrap_err().into_inner();
            assert_eq!(*recovered, 9);
            conflict(|| {
                drop(cell.lock());
            });
            *recovered = 12;
            drop(recovered);
            assert_eq!(*cell.lock().unwrap_err().into_inner(), 12);
        }
        "contention" => {
            use std::sync::{Arc, mpsc};
            use std::time::Duration;
            let cell = Arc::new(TerraneSharedCell::new(4));
            let mut guard = cell.lock().unwrap();
            let (ready_tx, ready_rx) = mpsc::channel();
            let (done_tx, done_rx) = mpsc::channel();
            let other = cell.clone();
            let worker = std::thread::spawn(move || {
                ready_tx.send(()).unwrap();
                let mut guard = other.lock().unwrap();
                *guard += 1;
                done_tx.send(*guard).unwrap();
            });
            ready_rx.recv_timeout(Duration::from_secs(2)).unwrap();
            assert!(matches!(
                done_rx.recv_timeout(Duration::from_millis(25)),
                Err(mpsc::RecvTimeoutError::Timeout)
            ));
            *guard = 8;
            drop(guard);
            assert_eq!(done_rx.recv_timeout(Duration::from_secs(2)).unwrap(), 9);
            worker.join().unwrap();
            assert_eq!(*cell.lock().unwrap(), 9);
        }
        "serialization" => {
            let cell = std::sync::Arc::new(TerraneSharedCell::new(0usize));
            let workers: Vec<_> = (0..8)
                .map(|_| {
                    let cell = cell.clone();
                    std::thread::spawn(move || {
                        for _ in 0..1000 {
                            *cell.lock().unwrap() += 1;
                        }
                    })
                })
                .collect();
            for worker in workers {
                worker.join().unwrap();
            }
            assert_eq!(*cell.lock().unwrap(), 8000);
        }
        "projection" => exercise_projection(),
        _ => panic!("unknown scenario {mode}"),
    }
}

#[test]
fn checked_shared_storage_behaviors_are_bounded() {
    const MODE: &str = "TERRANE_SHARED_STORAGE_TEST_MODE";
    if let Ok(mode) = std::env::var(MODE) {
        exercise(&mode);
        return;
    }
    for mode in [
        "nested",
        "cycle",
        "poison",
        "contention",
        "serialization",
        "projection",
    ] {
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "checked_shared_storage_behaviors_are_bounded",
                "--nocapture",
            ])
            .env(MODE, mode)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            if child.try_wait().unwrap().is_some() {
                break;
            }
            if std::time::Instant::now() >= deadline {
                let _ = child.kill();
                let output = child.wait_with_output().unwrap();
                panic!("{mode} hung: {}", String::from_utf8_lossy(&output.stderr));
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{mode}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
