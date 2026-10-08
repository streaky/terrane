// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: mixed-width-comparisons
fn main() {
    let small_terrane_f0_s52: i8;
    let wide_terrane_f0_s69: i32;
    small_terrane_f0_s52 = 5;
    wide_terrane_f0_s69 = 9;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(small_terrane_f0_s52 as i32 ==
        wide_terrane_f0_s69))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(small_terrane_f0_s52 as i32 !=
        wide_terrane_f0_s69))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&((small_terrane_f0_s52 as i32) <
        wide_terrane_f0_s69))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(small_terrane_f0_s52 as i32 <=
        wide_terrane_f0_s69))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(small_terrane_f0_s52 as i32 >
        wide_terrane_f0_s69))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(small_terrane_f0_s52 as i32 >=
        wide_terrane_f0_s69))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(wide_terrane_f0_s69 ==
        small_terrane_f0_s52 as i32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(wide_terrane_f0_s69 !=
        small_terrane_f0_s52 as i32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(wide_terrane_f0_s69 <
        small_terrane_f0_s52 as i32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(wide_terrane_f0_s69 <=
        small_terrane_f0_s52 as i32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(wide_terrane_f0_s69 >
        small_terrane_f0_s52 as i32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(wide_terrane_f0_s69 >=
        small_terrane_f0_s52 as i32))
    );
}
