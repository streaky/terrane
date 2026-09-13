// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: fizz-buzz
fn main() {
    let mut number: terrane_int_support::Int = terrane_int_support::Int::from(1_i128);
    while number.clone() <= terrane_int_support::Int::from(15_i128) {
        if __terrane_raised(
            number.clone().modulo(&terrane_int_support::Int::from(15_i128)),
            0 /* terrane-site: case.trn:6:8-6:19 */,
        ) == terrane_int_support::Int::from(0_i128)
        {
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("FizzBuzz"))
            );
        } else if __terrane_raised(
            number.clone().modulo(&terrane_int_support::Int::from(3_i128)),
            1 /* terrane-site: case.trn:8:13-8:23 */,
        ) == terrane_int_support::Int::from(0_i128)
        {
            println!("{}", terrane_scalar_support::scalar_text(&String::from("Fizz")));
        } else if __terrane_raised(
            number.clone().modulo(&terrane_int_support::Int::from(5_i128)),
            2 /* terrane-site: case.trn:10:13-10:23 */,
        ) == terrane_int_support::Int::from(0_i128)
        {
            println!("{}", terrane_scalar_support::scalar_text(&String::from("Buzz")));
        } else {
            println!("{}", terrane_scalar_support::scalar_text(&number));
        }
        number = number.clone() + terrane_int_support::Int::from(1_i128);
    }
}
