// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: local-function-rust-owned-bindings
fn main() {
    fn helper(parameter: terrane_int_support::Int) -> terrane_int_support::Int {
        let local: terrane_int_support::Int;
        local = parameter.clone() + terrane_int_support::Int::from(1_i128);
        return parameter + local;
    }
    fn math() -> terrane_int_support::Int {
        return terrane_int_support::Int::from(std::cmp::max(1 as i128, 2 as i128));
    }
    fn shadow() -> terrane_int_support::Int {
        return {
            struct Local {
                field: i128,
            }
            let value = Local { field: 41 };
            let values = [value.field];
            terrane_int_support::Int::from(values[0] + values.len() as i128)
        };
    }
    let max: i64;
    let value: String;
    let len: i64;
    let field: i64;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&helper(terrane_int_support::Int::from(4_i128)))
    );
    max = 3;
    value = String::from("outer");
    len = 4;
    field = 5;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&math()),
        terrane_scalar_support::scalar_text(&shadow())
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&max),
        terrane_scalar_support::scalar_text(&value),
        terrane_scalar_support::scalar_text(&len),
        terrane_scalar_support::scalar_text(&field)
    );
}
