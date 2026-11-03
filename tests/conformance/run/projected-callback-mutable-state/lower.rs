// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let total: terrane_int_support::Int;
    let callback: TerraneMutableCallable<
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    >;
    total = terrane_int_support::Int::from(0_i128);
    callback = {
        let mut total = total.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            total = total.clone() + value.clone();
            return total.clone();
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(1_i128); let __terrane_conversion_input_1 =
        callback.clone(); __terrane_raised(apply_mutable(__terrane_conversion_input_0,
        __terrane_conversion_input_1), 0 /* terrane-site: src/main.trn:8:13-8:39 */)
        })
    );
}
