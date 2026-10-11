// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: concurrency-objects
fn main() {
    let mut counter: IntMutex;
    let shared: IntReadWriteLock;
    let mut atomic: AtomicInt64;
    let updated: ConcurrencyIntResult;
    let invalid_store: ConcurrencyOperationResult;
    let invalid_ordering: ConcurrencyIntResult;
    let local: ThreadLocalInt;
    counter = IntMutex::terrane_construct(terrane_int_support::Int::from(4_i128));
    counter.increase(terrane_int_support::Int::from(3_i128));
    println!("{}", terrane_scalar_support::scalar_text(&counter.load().value));
    shared = IntReadWriteLock::terrane_construct(terrane_int_support::Int::from(8_i128));
    shared.write(terrane_int_support::Int::from(9_i128));
    println!("{}", terrane_scalar_support::scalar_text(&shared.read().value));
    atomic = AtomicInt64::terrane_construct(10);
    updated = atomic.increase(5, acquire_release_order());
    println!("{}", terrane_scalar_support::scalar_text(&updated.failed));
    println!(
        "{}", terrane_scalar_support::scalar_text(&atomic.load(acquire_order()).value)
    );
    invalid_store = atomic.store(16, acquire_release_order());
    println!("{}", terrane_scalar_support::scalar_text(&invalid_store.failed));
    invalid_ordering = atomic.load(release_order());
    println!("{}", terrane_scalar_support::scalar_text(&invalid_ordering.failed));
    println!("{}", terrane_scalar_support::scalar_text(&invalid_ordering.available));
    local = ThreadLocalInt::terrane_construct(terrane_int_support::Int::from(20_i128));
    local.write(terrane_int_support::Int::from(21_i128));
    println!("{}", terrane_scalar_support::scalar_text(&local.get().value));
}
