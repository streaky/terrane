// Vendored support crates: terrane-int-support, terrane-scalar-support
static __TERRANE_GLOBAL_GLOBAL_BODY: std::sync::LazyLock<
    std::sync::Mutex<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    Some(terrane_int_support::Int::from(2_i128)),
));
static __TERRANE_GLOBAL_GLOBAL_ROOT: std::sync::LazyLock<
    std::sync::Mutex<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    Some(terrane_int_support::Int::from(1_i128)),
));
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
