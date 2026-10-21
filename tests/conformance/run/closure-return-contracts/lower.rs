// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    let callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
    >;
    let empty: std::sync::Arc<dyn Fn() -> () + Send + Sync>;
    callback = {
        std::sync::Arc::new(move |value: terrane_int_support::Int| -> String {
            fn named() -> bool {
                return true;
            }
            let inner: std::sync::Arc<
                dyn Fn(bool) -> terrane_int_support::Int + Send + Sync,
            >;
            let text: String;
            inner = {
                let value = value.clone();
                std::sync::Arc::new(move |flag: bool| -> terrane_int_support::Int {
                    if flag {
                        return value.clone();
                    } else {
                        return terrane_int_support::Int::from(0_i128);
                    }
                })
            };
            println!("{}", terrane_scalar_support::scalar_text(&inner(named())));
            text = String::from("typed");
            return text;
        })
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&callback(terrane_int_support::Int::from(7_i128)))
    );
    empty = {
        std::sync::Arc::new(move || -> () {
            return ();
        })
    };
    empty();
}
