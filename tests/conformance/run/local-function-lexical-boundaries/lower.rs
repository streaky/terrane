// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: local-function-lexical-boundaries
static __TERRANE_F0_SHARED: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(10_i128));
fn helper_terrane_local_function_lexical_boundaries() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn outer() -> String {
    fn helper_terrane_local_function_lexical_boundaries() -> String {
        return String::from("local");
    }
    let value: String;
    value = helper_terrane_local_function_lexical_boundaries();
    return value;
}
fn factorial() -> terrane_int_support::Int {
    fn fact(n: terrane_int_support::Int) -> terrane_int_support::Int {
        if n <= terrane_int_support::Int::from(1_i128) {
            return terrane_int_support::Int::from(1_i128);
        }
        return n.clone() * fact(n.clone() - terrane_int_support::Int::from(1_i128));
    }
    return fact(terrane_int_support::Int::from(5_i128));
}
fn forward() -> terrane_int_support::Int {
    fn later() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(2_i128);
    }
    let value: terrane_int_support::Int;
    value = later();
    return value.clone();
}
fn parity(value: terrane_int_support::Int) -> terrane_int_support::Int {
    fn even(n: terrane_int_support::Int) -> bool {
        if n == terrane_int_support::Int::from(0_i128) {
            return true;
        }
        return odd(n.clone() - terrane_int_support::Int::from(1_i128));
    }
    fn odd(n: terrane_int_support::Int) -> bool {
        if n == terrane_int_support::Int::from(0_i128) {
            return false;
        }
        return even(n.clone() - terrane_int_support::Int::from(1_i128));
    }
    if even(value.clone()) {
        return terrane_int_support::Int::from(1_i128);
    }
    return terrane_int_support::Int::from(0_i128);
}
fn own_closure() -> terrane_int_support::Int {
    fn nested(value: terrane_int_support::Int) -> terrane_int_support::Int {
        let local: terrane_int_support::Int;
        let callback: std::sync::Arc<dyn Fn() -> terrane_int_support::Int + Send + Sync>;
        local = value.clone() + (*__TERRANE_F0_SHARED).clone();
        callback = {
            let local = local.clone();
            std::sync::Arc::new(move || -> terrane_int_support::Int {
                return local.clone()
                    + helper_terrane_local_function_lexical_boundaries();
            })
        };
        return callback();
    }
    return nested(terrane_int_support::Int::from(1_i128));
}
fn main() {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&helper_terrane_local_function_lexical_boundaries())
    );
    println!("{}", terrane_scalar_support::scalar_text(&outer()));
    println!("{}", terrane_scalar_support::scalar_text(&factorial()));
    println!("{}", terrane_scalar_support::scalar_text(&own_closure()));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&parity(terrane_int_support::Int::from(8_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&parity(terrane_int_support::Int::from(7_i128)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&forward()));
}
