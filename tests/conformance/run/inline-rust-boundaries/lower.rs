// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: inline-rust-boundaries
fn add(
    left: terrane_int_support::Int,
    right: terrane_int_support::Int,
) -> terrane_int_support::Int {
    let result: terrane_int_support::Int;
    result = left + right;
    return result.clone();
}
#[allow(unsafe_code)]
fn inspect_pointer() -> u8 {
    let value: u8;
    value = unsafe { *(&7u8 as *const u8) };
    return value;
}
fn rust_statement() {
    {
        println!("inline statement");
    }
}
fn inspect_cloned_input(input: String) -> terrane_int_support::Int {
    let size: terrane_int_support::Int;
    size = {
        let input = input.clone();
        terrane_int_support::Int::from(input.len() as i64)
    };
    println!("{}", terrane_scalar_support::scalar_text(&input));
    return size.clone();
}
fn borrowed_address(input: &String) -> u64 {
    return input.as_ptr() as u64;
}
fn owned_address(input: String) -> u64 {
    return input.as_ptr() as u64;
}
fn identity(value: String) -> String {
    return value;
}
fn main() {
    let input: String;
    let address: u64;
    let value: String;
    let copied: String;
    let mut observed: String;
    let mut index: terrane_int_support::Int;
    let callback: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
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
    input = String::from("owned handoff");
    address = borrowed_address(&input);
    println!(
        "{}", terrane_scalar_support::scalar_text(&(address == owned_address(input)))
    );
    value = String::from("repeat");
    copied = identity(value.clone());
    observed = {
        let value = value.clone();
        value
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(copied.as_str() == observed
        .as_str()))
    );
    index = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(2_i128) {
        observed = {
            let value = value.clone();
            value
        };
        println!("{}", terrane_scalar_support::scalar_text(&observed));
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    callback = {
        std::sync::Arc::new(move || -> String {
            return {
                let value = value.clone();
                value
            };
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&callback()));
    println!("{}", terrane_scalar_support::scalar_text(&callback()));
}
