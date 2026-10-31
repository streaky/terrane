// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: application-lowering-regressions
async fn observe(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone();
}
fn inspect_bytes(data: Vec<u8>) {
    println!("{}", terrane_scalar_support::scalar_text(&(data.len() as i128)));
}
fn branch_name(left: bool) -> String {
    let selected: String;
    let selected_2: String;
    if left {
        selected = String::from("left");
        return selected;
    }
    selected_2 = String::from("right");
    return selected_2;
}
fn main() {
    __terrane_run(async move {
        let data: Vec<u8>;
        let mut part: String;
        let mut counter: terrane_int_support::Int;
        __terrane_await(observe(terrane_int_support::Int::from(42_i128))).await;
        data = Vec::<u8>::from([97, 98, 99]);
        inspect_bytes(data.clone());
        println!("{}", terrane_scalar_support::scalar_text(&(data.len() as i128)));
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&branch_name(true)),
            terrane_scalar_support::scalar_text(&branch_name(false))
        );
        let __terrane_iterable_0 = {
            let __terrane_receiver_value_0 = &String::from("red,green");
            let __terrane_string_argument_0 = ",";
            terrane_string_support::split(
                &__terrane_receiver_value_0,
                &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
            )
        };
        let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
            &__terrane_iterable_0,
        );
        loop {
            part = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&part));
        }
        counter = terrane_int_support::Int::from(0_i128);
        while counter < terrane_int_support::Int::from(2_i128) {
            println!("{}", terrane_scalar_support::scalar_text(&counter));
            counter = counter.clone() + terrane_int_support::Int::from(1_i128);
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0
            = terrane_string_support::Encoding::Utf8;
            __terrane_raised(terrane_string_support::decode(&data,
            __terrane_string_argument_0), 0 /* terrane-site: case.trn:30:11-30:28 */)
            })
        );
    });
}
