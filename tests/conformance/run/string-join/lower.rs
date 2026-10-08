// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: string-join
fn separator() -> String {
    println!("{}", terrane_scalar_support::scalar_text(&String::from("side effect")));
    return String::from("--");
}
fn main() {
    let empty_terrane_f0_s105: String;
    let single_terrane_f0_s134: String;
    let many_terrane_f0_s162: String;
    empty_terrane_f0_s105 = {
        let _ = separator();
        String::new()
    };
    single_terrane_f0_s134 = vec![
        terrane_scalar_support::scalar_text(&String::from("one"))
    ]
        .join(&String::from("--"));
    many_terrane_f0_s162 = vec![
        terrane_scalar_support::scalar_text(&String::from("one")),
        terrane_scalar_support::scalar_text(&2),
        terrane_scalar_support::scalar_text(&true)
    ]
        .join(&String::from("--"));
    println!("{}", terrane_scalar_support::scalar_text(&empty_terrane_f0_s105));
    println!("{}", terrane_scalar_support::scalar_text(&single_terrane_f0_s134));
    println!("{}", terrane_scalar_support::scalar_text(&many_terrane_f0_s162));
}
