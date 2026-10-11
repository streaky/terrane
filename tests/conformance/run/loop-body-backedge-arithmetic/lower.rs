// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/loop-body-backedge-arithmetic
fn inferred() {
    let mut count: terrane_int_support::Int;
    let mut total: terrane_int_support::Int;
    let mut result: terrane_int_support::Int;
    let mut typed_result: terrane_int_support::Int;
    let mut step: terrane_int_support::Int;
    let mut composed: terrane_int_support::Int;
    let mut chained: terrane_int_support::Int;
    let mut comparison: bool;
    let mut previous: Option<terrane_int_support::Int> = None;
    count = terrane_int_support::Int::from(0_i128);
    total = terrane_int_support::Int::from(0_i128);
    while count.clone() < terrane_int_support::Int::from(3_i128) {
        count = count.clone() + terrane_int_support::Int::from(1_i128);
        if count.clone() > terrane_int_support::Int::from(1_i128) {
            result = previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    10,
                    22,
                ))
                .clone()
                .clone() + terrane_int_support::Int::from(1_i128);
            typed_result = previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    11,
                    32,
                ))
                .clone()
                .clone() + terrane_int_support::Int::from(1_i128);
            step = previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    12,
                    20,
                ))
                .clone()
                .clone();
            composed = step.clone() * terrane_int_support::Int::from(2_i128)
                + terrane_int_support::Int::from(1_i128);
            chained = step.clone() + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128)
                + terrane_int_support::Int::from(1_i128);
            comparison = previous
                .as_ref()
                .unwrap_or_else(|| __terrane_uninitialized_binding(
                    "previous",
                    "case.trn",
                    15,
                    26,
                ))
                .clone()
                .clone() > terrane_int_support::Int::from(1_i128);
            total = total.clone() + step.clone();
            println!("{}", terrane_scalar_support::scalar_text(&result));
            println!("{}", terrane_scalar_support::scalar_text(&typed_result));
            println!("{}", terrane_scalar_support::scalar_text(&composed));
            println!("{}", terrane_scalar_support::scalar_text(&chained));
            println!("{}", terrane_scalar_support::scalar_text(&comparison));
        }
        let _ = previous.insert(count.clone());
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
fn typed() {
    let mut total: terrane_int_support::Int;
    let mut count: terrane_int_support::Int;
    let mut previous: Option<terrane_int_support::Int> = None;
    total = terrane_int_support::Int::from(0_i128);
    count = terrane_int_support::Int::from(1_i128);
    '__terrane_break_0: while count.clone() < terrane_int_support::Int::from(4_i128) {
        '__terrane_continue_0: {
            if count.clone() > terrane_int_support::Int::from(1_i128) {
                total = total.clone()
                    + previous
                        .as_ref()
                        .unwrap_or_else(|| __terrane_uninitialized_binding(
                            "previous",
                            "case.trn",
                            29,
                            29,
                        ))
                        .clone()
                        .clone();
            }
            let _ = previous.insert(count.clone());
        }
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
fn main() {
    inferred();
    typed();
}
