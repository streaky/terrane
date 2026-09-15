// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: inline-rust-boundaries
fn add(
    left: terrane_int_support::Int,
    right: terrane_int_support::Int,
) -> terrane_int_support::Int {
    let _ = (&left, &right);
    let result: terrane_int_support::Int = {
        let left = left.clone();
        let right = right.clone();
        left + right
    };
    return result.clone();
}
fn inspect_pointer() -> u8 {
    let value: u8 = unsafe { *(&7u8 as *const u8) };
    return value;
}
fn rust_statement() {
    {
        println!("inline statement");
    }
}
fn inspect_cloned_input(input: String) -> terrane_int_support::Int {
    let size: terrane_int_support::Int = {
        let input = input.clone();
        terrane_int_support::Int::from(input.len() as i64)
    };
    println!("{}", terrane_scalar_support::scalar_text(&input));
    return size.clone();
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&add(terrane_int_support::Int::from(20_i128),
        terrane_int_support::Int::from(22_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&inspect_pointer()));
    rust_statement();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&inspect_cloned_input(String::from("copy me")))
    );
}
