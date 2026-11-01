// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let mut value: terrane_int_support::Int;
    let other: terrane_int_support::Int;
    let readonly: &terrane_int_support::Int;
    let observer: std::boxed::Box<dyn Fn() -> () + '_>;
    value = terrane_int_support::Int::from(0_i128);
    {
        let borrowed: &mut terrane_int_support::Int = &mut value;
        let count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        let mut callback: TerraneBorrowedMutableCallable<'_, (), ()> = {
            let borrowed = borrowed;
            let mut count = count.clone();
            TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
                count = count.clone() + terrane_int_support::Int::from(1_i128);
                *borrowed = count.clone();
                ()
            })
        };
        callback.call(());
        callback.call(());
        drop(callback);
    }
    println!("{}", terrane_scalar_support::scalar_text(&value));
    other = terrane_int_support::Int::from(5_i128);
    readonly = &other;
    observer = {
        let readonly = readonly;
        std::boxed::Box::new(move || -> () {
            println!("{}", terrane_scalar_support::scalar_text(&* readonly));
            ()
        })
    };
    observer();
    drop(observer);
}
