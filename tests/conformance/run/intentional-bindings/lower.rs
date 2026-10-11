// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let __trn_5f6775617264: Probe;
    let __trn_5f75736564: i64;
    {
        let _: Probe = __terrane_raised(
            probe(String::from("untyped")),
            0 /* terrane-site: src/main.trn:6:7-6:23 */,
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-untyped")));
    {
        let _: Probe = __terrane_raised(
            probe(String::from("typed")),
            1 /* terrane-site: src/main.trn:8:13-8:27 */,
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-typed")));
    __trn_5f6775617264 = __terrane_raised(
        probe(String::from("retained")),
        2 /* terrane-site: src/main.trn:10:12-10:29 */,
    );
    let _ = &__trn_5f6775617264;
    println!("{}", terrane_scalar_support::scalar_text(&String::from("after-retained")));
    __trn_5f75736564 = 7;
    println!("{}", terrane_scalar_support::scalar_text(&__trn_5f75736564));
    println!("{}", terrane_scalar_support::scalar_text(&__trn_5f75736564));
}
