// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
static __TERRANE_GLOBAL_RESULT: std::sync::LazyLock<
    std::sync::Mutex<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Some(add_one())));
static __TERRANE_GLOBAL_SEED: std::sync::LazyLock<
    std::sync::Mutex<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    Some(terrane_int_support::Int::from(7_i128)),
));
fn __terrane_uninitialized_global(
    name: &str,
    path: &str,
    line: usize,
    column: usize,
) -> ! {
    eprintln!(
        "{path}:{line}:{column}: error[T0007]: `{name}` may be read before it is assigned"
    );
    std::process::exit(1);
}
// Source: case.trn
// Namespace: global-initialization-function-read
fn add_one() -> terrane_int_support::Int {
    return __TERRANE_GLOBAL_SEED
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_global("seed", "case.trn", 5, 10))
        .clone() + terrane_int_support::Int::from(1_i128);
}
fn main() {
    println!(
        "{}", terrane_scalar_support::scalar_text(&__TERRANE_GLOBAL_RESULT.lock()
        .expect("program-global lock poisoned").clone().unwrap_or_else(| |
        __terrane_uninitialized_global("result", "case.trn", 7, 10)))
    );
}
