// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: reference-return-callable-parameter
fn main() {
    let flag: bool;
    let callback: std::sync::Arc<dyn Fn(&String) -> &String + Send + Sync>;
    let value: String;
    let returned: &String;
    let borrowed_value: &String;
    let snapshot: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
    flag = true;
    callback = {
        let flag = flag.clone();
        std::sync::Arc::new(move |source: &String| -> &String {
            if flag {
                return source;
            }
            return source;
        })
    };
    value = String::from("parameter loan");
    returned = callback(&value);
    println!("{}", terrane_scalar_support::scalar_text(&* returned));
    borrowed_value = &value;
    snapshot = {
        let borrowed_value = borrowed_value;
        std::sync::Arc::new(move || -> String {
            return borrowed_value.clone();
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&snapshot()));
}
