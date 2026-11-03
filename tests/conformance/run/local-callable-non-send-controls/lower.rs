// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let state: Local;
    let observer: &Local;
    let shared: std::boxed::Box<dyn Fn() -> () + '_>;
    let mut change: TerraneBorrowedMutableCallable<'_, (), ()>;
    let finish: TerraneBorrowedConsumingCallable<'_, (), ()>;
    state = __terrane_raised(
        terrane_static_trn_4c6f63616c_new(),
        0 /* terrane-site: src/main.trn:4:13-4:24 */,
    );
    observer = &state;
    shared = {
        let observer = observer;
        std::boxed::Box::new(move || -> () {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&* observer).get()) })) { Ok(result)
                => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_callable_send_witness",
                "terrane_callable_send_witness::Local::get")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 1 /* terrane-site: src/main.trn:7:17-7:30 */))
            );
            ()
        })
    };
    shared();
    drop(shared);
    change = {
        let observer = observer;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&* observer).get()) })) { Ok(result)
                => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_callable_send_witness",
                "terrane_callable_send_witness::Local::get")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 2 /* terrane-site: src/main.trn:10:17-10:30 */))
            );
            ()
        })
    };
    change.call(());
    change.call(());
    drop(change);
    finish = {
        let observer = observer;
        TerraneBorrowedConsumingCallable::new(move |(): ()| -> () {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&* observer).get()) })) { Ok(result)
                => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_callable_send_witness",
                "terrane_callable_send_witness::Local::get")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 3 /* terrane-site: src/main.trn:14:17-14:30 */))
            );
            ()
        })
    };
    finish.call(());
}
