// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: rust-only-closure-captures
#[allow(unsafe_code)]
fn main() {
    let mut text: String;
    let outer_read: std::sync::Arc<dyn Fn(i64) -> i64 + Send + Sync>;
    let block_text: String;
    let block_read: std::sync::Arc<dyn Fn() -> i64 + Send + Sync>;
    let unsafe_text: String;
    let unsafe_read: std::sync::Arc<dyn Fn() -> i64 + Send + Sync>;
    text = String::from("outside");
    outer_read = {
        let text = text.clone();
        std::sync::Arc::new(move |other: i64| -> i64 {
            return {
                let text = text.clone();
                text.len() as i64 + other
            };
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&outer_read(1)));
    println!("{}", terrane_scalar_support::scalar_text(&outer_read(2)));
    println!("{}", terrane_scalar_support::scalar_text(&text));
    text = String::from("x");
    println!("{}", terrane_scalar_support::scalar_text(&outer_read(3)));
    println!("{}", terrane_scalar_support::scalar_text(&text));
    block_text = String::from("abc");
    block_read = {
        let block_text = block_text.clone();
        std::sync::Arc::new(move || -> i64 {
            let result: i64;
            result = {
                let block_text = block_text.clone();
                block_text.len() as i64
            };
            return result;
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&block_read()));
    println!("{}", terrane_scalar_support::scalar_text(&block_text));
    unsafe_text = String::from("unsafe");
    unsafe_read = {
        let unsafe_text = unsafe_text.clone();
        std::sync::Arc::new(move || -> i64 {
            return unsafe {
                let unsafe_text = unsafe_text.clone();
                *unsafe_text.as_bytes().as_ptr() as i64
            };
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&unsafe_read()));
    println!("{}", terrane_scalar_support::scalar_text(&unsafe_text));
}
