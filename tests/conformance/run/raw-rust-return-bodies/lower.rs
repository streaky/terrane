// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: raw-rust-return-bodies
fn ordinary() -> i64 {
    return {
        let text = r#"a=>b == c"#;
        (text.len() + 1) as i64
    };
}
#[allow(unsafe_code)]
fn unsafe_result() -> i64 {
    return unsafe {
        {
            let value = 41_i64;
            std::ptr::read(&value) + 1
        }
    };
}
#[allow(unsafe_code)]
fn main() {
    let mut value: u8;
    println!("{}", terrane_scalar_support::scalar_text(&ordinary()));
    println!("{}", terrane_scalar_support::scalar_text(&unsafe_result()));
    value = 7;
    unsafe {
        *(&mut value as *mut u8) = 8;
    }
    println!("{}", terrane_scalar_support::scalar_text(&value));
    println!("{}", String::from("ok"));
}
