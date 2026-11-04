// Vendored support crates: terrane-int-support, terrane-scalar-support
static __TERRANE_GLOBAL_SEEDED: std::sync::LazyLock<
    std::sync::Mutex<Option<terrane_int_support::Int>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(
    Some(
        {
            let mut instance = Item::terrane_construct();
            {
                let __terrane_value = terrane_int_support::Int::from(8_i128);
                instance.value = __terrane_value;
            }
            instance
        }
            .value
            .clone(),
    ),
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
