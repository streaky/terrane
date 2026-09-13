// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-scalar-support
// Source: case.trn
// Namespace: rust-reserved-identifiers
fn main() {
    let __trn_6162737472616374: i64 = 1;
    let __trn_6265636f6d65: i64 = 2;
    let __trn_626f78: i64 = 3;
    let __trn_646f: i64 = 4;
    let __trn_66696e616c: i64 = 5;
    let __trn_6d6163726f: i64 = 6;
    let __trn_6f76657272696465: i64 = 7;
    let __trn_70726976: i64 = 8;
    let __trn_747970656f66: i64 = 9;
    let __trn_756e73697a6564: i64 = 10;
    let __trn_7669727475616c: i64 = 11;
    let __trn_67656e: i64 = 12;
    let total: terrane_int_support::Int = terrane_int_support::Int::from(
        __trn_6162737472616374 as i128,
    ) + terrane_int_support::Int::from(__trn_6265636f6d65 as i128)
        + terrane_int_support::Int::from(__trn_626f78 as i128)
        + terrane_int_support::Int::from(__trn_646f as i128)
        + terrane_int_support::Int::from(__trn_66696e616c as i128)
        + terrane_int_support::Int::from(__trn_6d6163726f as i128)
        + terrane_int_support::Int::from(__trn_6f76657272696465 as i128)
        + terrane_int_support::Int::from(__trn_70726976 as i128)
        + terrane_int_support::Int::from(__trn_747970656f66 as i128)
        + terrane_int_support::Int::from(__trn_756e73697a6564 as i128)
        + terrane_int_support::Int::from(__trn_7669727475616c as i128)
        + terrane_int_support::Int::from(__trn_67656e as i128);
    println!("{}", terrane_scalar_support::scalar_text(&total));
}
