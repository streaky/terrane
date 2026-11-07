// Vendored support crates: terrane-scalar-support
static __TERRANE_GLOBAL_CALLS: std::sync::LazyLock<std::sync::Mutex<Option<f64>>> = std::sync::LazyLock::new(||
std::sync::Mutex::new(Some(0.5)));
fn __terrane_uninitialized_binding(
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
