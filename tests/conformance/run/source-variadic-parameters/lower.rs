// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: source-variadic-parameters
pub trait SummerProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn SummerProtocol>;
    fn separate_box(&self) -> Box<dyn SummerProtocol>;
    fn sum(
        &self,
        values: terrane_collection_support::List<terrane_int_support::Int>,
    ) -> terrane_int_support::Int;
}
impl Clone for Box<dyn SummerProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Summer(Box<dyn SummerProtocol>);
impl Clone for Summer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Summer {
    pub fn sum(
        &self,
        values: terrane_collection_support::List<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        self.0.sum(values)
    }
}
#[derive(Clone)]
pub struct Calculator {}
impl Calculator {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn sum(
        &self,
        values: terrane_collection_support::List<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let mut result: terrane_int_support::Int;
        let mut value: terrane_int_support::Int;
        result = terrane_int_support::Int::from(0_i128);
        let __terrane_iterable_0 = values;
        let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_0,
        );
        loop {
            value = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            result = result.clone() + value.clone();
        }
        return result.clone();
    }
}
impl SummerProtocol for Calculator {
    fn clone_box(&self) -> Box<dyn SummerProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SummerProtocol> {
        Box::new(self.clone())
    }
    fn sum(
        &self,
        values: terrane_collection_support::List<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        Calculator::sum(&*self, values)
    }
}
impl From<Calculator> for Summer {
    fn from(value: Calculator) -> Self {
        Self(Box::new(value))
    }
}
fn total(
    values: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut result: terrane_int_support::Int;
    let mut value: terrane_int_support::Int;
    result = terrane_int_support::Int::from(0_i128);
    let __terrane_iterable_1 = values;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        result = result.clone() + value.clone();
    }
    return result.clone();
}
fn offset_total(
    offset: terrane_int_support::Int,
    values: terrane_collection_support::List<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut result: terrane_int_support::Int;
    let mut value: terrane_int_support::Int;
    result = offset.clone();
    let __terrane_iterable_2 = values;
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        value = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        result = result.clone() + value.clone();
    }
    return result.clone();
}
fn main() {
    let closure: std::sync::Arc<
        dyn Fn(
            terrane_collection_support::List<terrane_int_support::Int>,
        ) -> terrane_int_support::Int + Send + Sync,
    >;
    let typed: std::sync::Arc<
        dyn Fn(
            terrane_collection_support::List<terrane_int_support::Int>,
        ) -> terrane_int_support::Int + Send + Sync,
    >;
    let concrete: Calculator;
    let __trn_6162737472616374: Summer;
    let bound: std::sync::Arc<
        dyn Fn(
            terrane_collection_support::List<terrane_int_support::Int>,
        ) -> terrane_int_support::Int + Send + Sync,
    >;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&total(terrane_collection_support::List::new(vec![])))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&total(terrane_collection_support::List::new(vec![terrane_int_support::Int::from(1_i128),
        terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128)])))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&offset_total(terrane_int_support::Int::from(10_i128),
        terrane_collection_support::List::new(vec![terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128)])))
    );
    closure = {
        std::sync::Arc::new(move |
            values: terrane_collection_support::List<terrane_int_support::Int>,
        | -> terrane_int_support::Int {
            return terrane_int_support::Int::from(
                terrane_int_support::Int::from(values.length()),
            );
        })
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&closure(terrane_collection_support::List::new(vec![terrane_int_support::Int::from(2_i128),
        terrane_int_support::Int::from(3_i128),
        terrane_int_support::Int::from(4_i128)])))
    );
    typed = std::sync::Arc::new(total);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&typed(terrane_collection_support::List::new(vec![terrane_int_support::Int::from(4_i128),
        terrane_int_support::Int::from(5_i128)])))
    );
    concrete = Calculator::terrane_construct();
    __trn_6162737472616374 = <Summer>::from(concrete.clone());
    println!(
        "{}", terrane_scalar_support::scalar_text(&__trn_6162737472616374
        .sum(terrane_collection_support::List::new(vec![terrane_int_support::Int::from(6_i128),
        terrane_int_support::Int::from(7_i128)])))
    );
    bound = {
        let receiver = concrete;
        std::sync::Arc::new(move |
            argument_0: terrane_collection_support::List<terrane_int_support::Int>|
        receiver.sum(argument_0))
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&bound(terrane_collection_support::List::new(vec![terrane_int_support::Int::from(8_i128),
        terrane_int_support::Int::from(9_i128)])))
    );
}
