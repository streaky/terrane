// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support
// Source: case.trn
// Namespace: source-defined-iterator
#[derive(Clone)]
pub struct Counter {
    pub current: terrane_int_support::Int,
    pub stop: terrane_int_support::Int,
    pub ended: bool,
    pub advances: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct(stop: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            current: terrane_int_support::Int::from(0_i128),
            stop: terrane_int_support::Int::from(0_i128),
            ended: false,
            advances: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(stop);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, stop: terrane_int_support::Int) {
        self.stop = stop.clone();
    }
    pub fn iterator(&self) -> Counter {
        return self.clone();
    }
    pub fn next(
        &mut self,
    ) -> terrane_collection_support::IterationStep<terrane_int_support::Int> {
        let item_terrane_f0_s528: terrane_int_support::Int;
        if self.ended {
            return terrane_collection_support::IterationStep::End;
        }
        self.advances = self.advances.clone() + terrane_int_support::Int::from(1_i128);
        if self.current.clone() >= self.stop.clone() {
            self.ended = true;
            return terrane_collection_support::IterationStep::End;
        }
        item_terrane_f0_s528 = self.current.clone();
        self.current = self.current.clone() + terrane_int_support::Int::from(1_i128);
        return terrane_collection_support::IterationStep::<
            terrane_int_support::Int,
        >::Item(item_terrane_f0_s528.clone());
    }
}
#[derive(Clone)]
pub struct NoneOnce {
    pub emitted: bool,
}
impl NoneOnce {
    pub fn terrane_construct() -> Self {
        Self { emitted: false }
    }
    pub fn iterator(&self) -> NoneOnce {
        return self.clone();
    }
    pub fn next(&mut self) -> terrane_collection_support::IterationStep<()> {
        if self.emitted {
            return terrane_collection_support::IterationStep::End;
        }
        self.emitted = true;
        return terrane_collection_support::IterationStep::<()>::Item(());
    }
}
fn main() {
    let values_terrane_f0_s885: Counter;
    let mut value_terrane_f0_s920: terrane_int_support::Int;
    let mut probe_terrane_f0_s956: Counter;
    let fourth_terrane_f0_s1028: terrane_collection_support::IterationStep<
        terrane_int_support::Int,
    >;
    let fifth_terrane_f0_s1073: terrane_collection_support::IterationStep<
        terrane_int_support::Int,
    >;
    let mut missing_terrane_f0_s1168: ();
    values_terrane_f0_s885 = Counter::terrane_construct(
        terrane_int_support::Int::from(3_i128),
    );
    let __terrane_iterable_0 = values_terrane_f0_s885;
    let mut __terrane_iterator_0 = __terrane_iterable_0.iterator();
    loop {
        value_terrane_f0_s920 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s920));
    }
    probe_terrane_f0_s956 = Counter::terrane_construct(
        terrane_int_support::Int::from(3_i128),
    );
    probe_terrane_f0_s956.next();
    probe_terrane_f0_s956.next();
    probe_terrane_f0_s956.next();
    fourth_terrane_f0_s1028 = probe_terrane_f0_s956.next();
    fifth_terrane_f0_s1073 = probe_terrane_f0_s956.next();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&matches!(&fourth_terrane_f0_s1028,
        terrane_collection_support::IterationStep::End)),
        terrane_scalar_support::scalar_text(&matches!(&fifth_terrane_f0_s1073,
        terrane_collection_support::IterationStep::End)),
        terrane_scalar_support::scalar_text(&probe_terrane_f0_s956.advances)
    );
    let __terrane_iterable_1 = NoneOnce::terrane_construct();
    let mut __terrane_iterator_1 = __terrane_iterable_1.iterator();
    loop {
        missing_terrane_f0_s1168 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&missing_terrane_f0_s1168));
    }
}
