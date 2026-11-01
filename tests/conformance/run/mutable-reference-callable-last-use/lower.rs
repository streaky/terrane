// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: mutable-reference-callable-last-use
fn branch(condition: bool) {
    let mut value: terrane_int_support::Int;
    let borrowed: &mut terrane_int_support::Int;
    let mut callback: TerraneBorrowedMutableCallable<'_, (), ()>;
    value = terrane_int_support::Int::from(0_i128);
    borrowed = &mut value;
    callback = {
        let borrowed = borrowed;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
            *borrowed = (*borrowed).clone() + terrane_int_support::Int::from(1_i128);
            ()
        })
    };
    if condition {
        callback.call(());
    } else {
        callback.call(());
        callback.call(());
    }
    drop(callback);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn __trn_6c6f6f70() {
    let mut value: terrane_int_support::Int;
    let borrowed: &mut terrane_int_support::Int;
    let mut callback: TerraneBorrowedMutableCallable<'_, (), ()>;
    let mut count: terrane_int_support::Int;
    value = terrane_int_support::Int::from(0_i128);
    borrowed = &mut value;
    callback = {
        let borrowed = borrowed;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
            *borrowed = (*borrowed).clone() + terrane_int_support::Int::from(1_i128);
            ()
        })
    };
    count = terrane_int_support::Int::from(0_i128);
    while count < terrane_int_support::Int::from(3_i128) {
        callback.call(());
        count = count.clone() + terrane_int_support::Int::from(1_i128);
    }
    drop(callback);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn transferred() {
    let mut value: terrane_int_support::Int;
    let borrowed: &mut terrane_int_support::Int;
    let callback: TerraneBorrowedMutableCallable<'_, (), ()>;
    let mut alias: TerraneBorrowedMutableCallable<'_, (), ()>;
    value = terrane_int_support::Int::from(0_i128);
    borrowed = &mut value;
    callback = {
        let borrowed = borrowed;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
            *borrowed = (*borrowed).clone() + terrane_int_support::Int::from(1_i128);
            ()
        })
    };
    alias = callback;
    alias.call(());
    drop(alias);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn unused_transfer() {
    let mut value: terrane_int_support::Int;
    let borrowed: &mut terrane_int_support::Int;
    let callback: TerraneBorrowedMutableCallable<'_, (), ()>;
    let alias: TerraneBorrowedMutableCallable<'_, (), ()>;
    value = terrane_int_support::Int::from(0_i128);
    borrowed = &mut value;
    callback = {
        let borrowed = borrowed;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
            *borrowed = (*borrowed).clone() + terrane_int_support::Int::from(1_i128);
            ()
        })
    };
    alias = callback;
    let _ = &alias;
    drop(alias);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn handle_transfer() {
    let mut value: terrane_int_support::Int;
    let first: &mut terrane_int_support::Int;
    let second: &mut terrane_int_support::Int;
    value = terrane_int_support::Int::from(0_i128);
    first = &mut value;
    second = first;
    *second = terrane_int_support::Int::from(3_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn unused_observer() {
    let mut value: terrane_int_support::Int;
    let observer: &terrane_int_support::Int;
    let exclusive: &mut terrane_int_support::Int;
    value = terrane_int_support::Int::from(0_i128);
    observer = &value;
    let _ = &observer;
    exclusive = &mut value;
    *exclusive = terrane_int_support::Int::from(2_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn two_captures() {
    let mut left: terrane_int_support::Int;
    let mut right: terrane_int_support::Int;
    let left_borrow: &mut terrane_int_support::Int;
    let right_borrow: &mut terrane_int_support::Int;
    let mut callback: TerraneBorrowedMutableCallable<'_, (), ()>;
    left = terrane_int_support::Int::from(0_i128);
    right = terrane_int_support::Int::from(0_i128);
    left_borrow = &mut left;
    right_borrow = &mut right;
    callback = {
        let left_borrow = left_borrow;
        let right_borrow = right_borrow;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
            *left_borrow = (*left_borrow).clone()
                + terrane_int_support::Int::from(1_i128);
            *right_borrow = (*right_borrow).clone()
                + terrane_int_support::Int::from(2_i128);
            ()
        })
    };
    callback.call(());
    drop(callback);
    println!("{}", terrane_scalar_support::scalar_text(&left));
    println!("{}", terrane_scalar_support::scalar_text(&right));
}
fn nested_branch(outer: bool, inner: bool) {
    let mut value: terrane_int_support::Int;
    let borrowed: &mut terrane_int_support::Int;
    let mut callback: TerraneBorrowedMutableCallable<'_, (), ()>;
    value = terrane_int_support::Int::from(0_i128);
    borrowed = &mut value;
    callback = {
        let borrowed = borrowed;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> () {
            *borrowed = (*borrowed).clone() + terrane_int_support::Int::from(1_i128);
            ()
        })
    };
    if outer {
        if inner {
            callback.call(());
        } else {
            callback.call(());
        }
    }
    drop(callback);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn readonly_carriers() {
    let mut value: terrane_int_support::Int;
    let observer: &terrane_int_support::Int;
    let first: std::boxed::Box<dyn Fn() -> terrane_int_support::Int + '_>;
    let second: std::boxed::Box<dyn Fn() -> terrane_int_support::Int + '_>;
    value = terrane_int_support::Int::from(0_i128);
    observer = &value;
    first = {
        let observer = observer;
        std::boxed::Box::new(move || -> terrane_int_support::Int {
            return observer.clone();
        })
    };
    second = {
        let observer = observer;
        std::boxed::Box::new(move || -> terrane_int_support::Int {
            return observer.clone();
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&first()));
    drop(first);
    println!("{}", terrane_scalar_support::scalar_text(&second()));
    drop(second);
    value = terrane_int_support::Int::from(7_i128);
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn returned_call() -> terrane_int_support::Int {
    let mut value: terrane_int_support::Int;
    let borrowed: &mut terrane_int_support::Int;
    let mut callback: TerraneBorrowedMutableCallable<'_, (), terrane_int_support::Int>;
    value = terrane_int_support::Int::from(0_i128);
    borrowed = &mut value;
    callback = {
        let borrowed = borrowed;
        TerraneBorrowedMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
            *borrowed = (*borrowed).clone() + terrane_int_support::Int::from(1_i128);
            return (*borrowed).clone();
        })
    };
    return callback.call(());
}
fn main() {
    branch(true);
    branch(false);
    __trn_6c6f6f70();
    transferred();
    unused_transfer();
    handle_transfer();
    unused_observer();
    two_captures();
    nested_branch(true, true);
    nested_branch(false, false);
    readonly_carriers();
    println!("{}", terrane_scalar_support::scalar_text(&returned_call()));
}
