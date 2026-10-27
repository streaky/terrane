// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: function-owned-contextual-collections
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S135 {
    Arm0(terrane_collection_support::List<String>),
    Arm1(bool),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S135 {
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
fn main() {
    let mut index: terrane_int_support::Int;
    let mut state: Option<TerraneUnionF0S135> = None;
    let __terrane_iterable_0 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        index = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = state
            .insert(
                TerraneUnionF0S135::Arm0(
                    terrane_collection_support::List::<String>::new(Vec::new()),
                ),
            );
        {
            let __terrane_collection_argument_0 = String::from("first");
            match state.as_mut().expect("flow-proven available binding") {
                TerraneUnionF0S135::Arm0(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .append(__terrane_collection_argument_0);
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match state
            .as_ref().expect("flow-proven available binding") {
            TerraneUnionF0S135::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") }
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&(index
            .clone() - terrane_int_support::Int::from(1_i128))), 0 /* terrane-site: case.trn:9:16-9:32 */)), 0 /* terrane-site: case.trn:9:16-9:32 */))
        );
    }
    let __terrane_iterable_1 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(3_i128)]);
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        index = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &index;
        let _ = state
            .insert(
                TerraneUnionF0S135::Arm1(index == terrane_int_support::Int::from(3_i128)),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match state.as_ref()
            .expect("flow-proven available binding") { TerraneUnionF0S135::Arm1(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
