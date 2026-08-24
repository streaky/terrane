// Generated deterministically by Terrane <version>.
fn __terrane_block_on<F: Future>(future: F) -> F::Output {
struct Wake;
impl std::task::Wake for Wake { fn wake(self: std::sync::Arc<Self>) {} }
let waker = std::task::Waker::from(std::sync::Arc::new(Wake));
let mut context = std::task::Context::from_waker(&waker);
let mut future = std::pin::pin!(future);
loop { match future.as_mut().poll(&mut context) {
std::task::Poll::Ready(value) => return value,
std::task::Poll::Pending => std::thread::yield_now(),
} }
}
#[derive(Clone)]
pub struct TerraneTaskScope {
cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
deadline: Option<std::time::Instant>,
}
impl TerraneTaskScope {
pub fn new(deadline_ms: Option<u64>) -> Self {
Self { cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), deadline: deadline_ms.map(|value| std::time::Instant::now() + std::time::Duration::from_millis(value)) }
}
pub fn child_scope(&self, deadline_ms: u64) -> Self {
let requested = std::time::Instant::now() + std::time::Duration::from_millis(deadline_ms);
let deadline = Some(self.deadline.map_or(requested, |parent| std::cmp::min(parent, requested)));
Self { cancelled: self.cancelled.clone(), deadline }
}
pub fn cancel(&self) { self.cancelled.store(true, std::sync::atomic::Ordering::Release); }
pub fn join<T>(&self, mut task: TerraneScopedTask<T>) -> TerraneTaskOutcome<T> {
let result = task.result.take().expect("scoped task joined once");
let cancelled = self.cancelled.load(std::sync::atomic::Ordering::Acquire)
|| self.deadline.is_some_and(|deadline| std::time::Instant::now() >= deadline);
match result {
Ok(value) => TerraneTaskOutcome { completed: true, cancelled, value: Some(value), error: String::new() },
Err(error) => TerraneTaskOutcome { completed: false, cancelled, value: None, error },
}
}
}
pub struct TerraneScopedTask<T> { result: Option<Result<T, String>> }
impl<T> TerraneScopedTask<T> {
pub fn spawn<F: FnOnce() -> Result<T, String>>(work: F) -> Self {
Self { result: Some(work()) }
}
}
pub struct TerraneTaskOutcome<T> { pub completed: bool, pub cancelled: bool, pub value: Option<T>, pub error: String }
// Source: app/main.trn
// Namespace: cooperative-task-scope
async fn work() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(9_i128);
}
fn main() {
    let scope: TerraneTaskScope = TerraneTaskScope::new(None);
    let child: TerraneScopedTask<terrane_int_support::Int> = TerraneScopedTask::spawn(move || Ok(__terrane_block_on((work)())));
    let outcome: TerraneTaskOutcome<terrane_int_support::Int> = (scope).join(child);
    println!("{}{}{}", terrane_scalar_support::scalar_text(&((outcome).completed)), terrane_scalar_support::scalar_text(&((outcome).cancelled)), terrane_scalar_support::scalar_text(&((outcome).value.expect("completed task outcome has a value"))));
}
