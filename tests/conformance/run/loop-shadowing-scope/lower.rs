// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: loop-shadowing-scope
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S121 {
    Arm0(terrane_collection_support::List<String>),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S121 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S307 {
    Arm0(
        terrane_collection_support::Entry<
            String,
            terrane_collection_support::List<terrane_int_support::Int>,
        >,
    ),
    Arm1(terrane_int_support::Int),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S307 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn value() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn main() {
    let mut items: TerraneUnionF0S121;
    let groups: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let mut entry: Option<TerraneUnionF0S307> = None;
    let numbers: terrane_collection_support::List<terrane_int_support::Int>;
    let numbers_2: String;
    let mut value_2: i64;
    let copy: terrane_int_support::Int;
    items = TerraneUnionF0S121::Arm0(
        terrane_collection_support::List::<
            String,
        >::new(vec![String::from("ab"), String::from("c")]),
    );
    let __terrane_iterable_0 = match &items {
        TerraneUnionF0S121::Arm0(value) => value,
        _ => unreachable!("flow-proven storage refinement"),
    }
        .clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        items = TerraneUnionF0S121::Arm1(
            match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            },
        );
        let _ = &items;
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &items {
            TerraneUnionF0S121::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    if matches!(&items, TerraneUnionF0S121::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &items {
            TerraneUnionF0S121::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    groups = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_int_support::Int>,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("first"),
            terrane_collection_support::List::< terrane_int_support::Int
            >::new(vec![terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)]))
        ],
    );
    let __terrane_iterable_1 = groups;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        let _ = entry
            .insert(
                TerraneUnionF0S307::Arm0(
                    match __terrane_iterator_1.next() {
                        terrane_collection_support::IterationStep::Item(item) => item,
                        terrane_collection_support::IterationStep::End => break,
                    },
                ),
            );
        let __terrane_iterable_2 = match entry
            .as_ref()
            .expect("flow-proven available binding")
        {
            TerraneUnionF0S307::Arm0(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .value
            .clone();
        let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_2,
        );
        loop {
            let _ = entry
                .insert(
                    TerraneUnionF0S307::Arm1(
                        match __terrane_iterator_2.next() {
                            terrane_collection_support::IterationStep::Item(item) => item,
                            terrane_collection_support::IterationStep::End => break,
                        },
                    ),
                );
            let _ = &entry;
            println!(
                "{}", terrane_scalar_support::scalar_text(&match entry.as_ref()
                .expect("flow-proven available binding") {
                TerraneUnionF0S307::Arm1(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
            );
        }
    }
    if matches!(
        &entry.as_ref().unwrap_or_else(|| __terrane_uninitialized_binding("entry",
        "case.trn", 18, 6)).clone(), TerraneUnionF0S307::Arm1(_)
    ) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match entry.as_ref()
            .unwrap_or_else(| | __terrane_uninitialized_binding("entry", "case.trn", 19,
            12)) { TerraneUnionF0S307::Arm1(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
    numbers = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let _ = &numbers;
    let __terrane_replacement = String::from("ab");
    drop(numbers);
    numbers_2 = __terrane_replacement;
    println!("{}", terrane_scalar_support::scalar_text(&numbers_2));
    println!("{}", terrane_scalar_support::scalar_text(&numbers_2));
    println!("{}", terrane_scalar_support::scalar_text(&value()));
    value_2 = 5;
    copy = terrane_int_support::Int::from(value_2 as i128);
    println!("{}", terrane_scalar_support::scalar_text(&copy));
    value_2 = __terrane_raised(
        terrane_int_support::coerce::<
            i64,
        >(
            &(terrane_int_support::Int::from(value_2 as i128)
                + terrane_int_support::Int::from(1_i128)),
        ),
        0 /* terrane-site: case.trn:31:15-31:24 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&value_2));
}
