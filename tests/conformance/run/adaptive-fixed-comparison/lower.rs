// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: adaptive-fixed-comparison
#[derive(Clone)]
pub struct FixedCount {
    pub expected: i64,
}
impl FixedCount {
    pub fn terrane_construct() -> Self {
        Self { expected: 3 }
    }
}
fn main() {
    let values: terrane_collection_support::List<terrane_int_support::Int>;
    let equal: i64;
    let lower: i64;
    let higher: i64;
    let negative: i64;
    let samples: terrane_collection_support::List<FixedCount>;
    let mut sample: FixedCount;
    let beyond: terrane_int_support::Int;
    let maximum: i64;
    let large: terrane_int_support::Int;
    let current: Option<terrane_int_support::Int>;
    let mut bound: terrane_int_support::Int;
    let mut step: terrane_int_support::Int;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    equal = 3;
    lower = 2;
    higher = 4;
    negative = -1;
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) == terrane_int_support::Int::from(equal as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(equal as
        i128) == terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) != terrane_int_support::Int::from(lower as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(lower as
        i128) != terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) > terrane_int_support::Int::from(lower as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(lower as
        i128) < terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) >= terrane_int_support::Int::from(equal as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(equal as
        i128) <= terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) < terrane_int_support::Int::from(higher as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(higher as
        i128) > terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) <= terrane_int_support::Int::from(equal as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(equal as
        i128) >= terrane_int_support::Int::from(values.length())))
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
        .length()) > terrane_int_support::Int::from(negative as i128))),
        String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(negative as
        i128) < terrane_int_support::Int::from(values.length())))
    );
    samples = terrane_collection_support::List::<
        FixedCount,
    >::new(vec![FixedCount::terrane_construct()]);
    let __terrane_iterable_0 = samples;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        sample = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}",
            terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
            .length()) != terrane_int_support::Int::from(sample.expected as i128))),
            String::from(","),
            terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(values
            .length()) == terrane_int_support::Int::from(sample.expected as i128)))
        );
    }
    beyond = terrane_int_support::Int::from(9223372036854775808_i128);
    maximum = 9223372036854775807;
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(beyond >
        terrane_int_support::Int::from(maximum as i128))), String::from(","),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(maximum as
        i128) < beyond))
    );
    large = terrane_int_support::Int::from_decimal(
        "340282366920938463463374607431768211456",
    );
    current = Some(large.clone());
    bound = large.clone();
    let __terrane_iterable_1 = values;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        step = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if (&current).is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&(* match &current {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                } >= bound)), String::from(","),
                terrane_scalar_support::scalar_text(&(bound == * match &current {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                }))
            );
            bound = bound.clone() + step.clone();
        }
    }
    println!("{}", terrane_scalar_support::scalar_text(&large));
}
