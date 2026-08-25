// Generated deterministically by Terrane <version>.
fn __terrane_block_on<F: Future>(future: F) -> F::Output {
    struct Wake;
    impl std::task::Wake for Wake {
        fn wake(self: std::sync::Arc<Self>) {}
    }
    let waker = std::task::Waker::from(std::sync::Arc::new(Wake));
    let mut context = std::task::Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(value) => return value,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}
// Source: case.trn
// Namespace: borrow-across-await
async fn answer() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(42_i128);
}
async fn inspect() -> terrane_int_support::Int {
    let value: std::sync::Arc<std::sync::Mutex<terrane_int_support::Int>> = std::sync::Arc::new(
        std::sync::Mutex::new(terrane_int_support::Int::from(7_i128)),
    );
    let observed: std::sync::Weak<std::sync::Mutex<terrane_int_support::Int>> = std::sync::Arc::downgrade(
        &value,
    );
    let result: terrane_int_support::Int = (Box::pin(answer())).await;
    println!(
        "{}", terrane_scalar_support::scalar_text(& (({ let __terrane_owner = observed
        .upgrade().expect("reference expired"); let __terrane_value = __terrane_owner
        .lock().expect("reference lock poisoned").clone(); __terrane_value })))
    );
    return result.clone();
}
fn main() {
    __terrane_block_on(async move {
        let result: terrane_int_support::Int = (Box::pin(inspect())).await;
        println!("{}", terrane_scalar_support::scalar_text(& (result)));
    });
}
