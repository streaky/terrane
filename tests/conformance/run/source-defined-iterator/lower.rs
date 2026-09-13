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
        let mut value = Self {
            current: terrane_int_support::Int::from(0_i128),
            stop: terrane_int_support::Int::from(0_i128),
            ended: false,
            advances: terrane_int_support::Int::from(0_i128),
        };
        value.construct(stop);
        value
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
        if self.ended {
            return terrane_collection_support::IterationStep::End;
        }
        self.advances = self.advances.clone() + terrane_int_support::Int::from(1_i128);
        if self.current.clone() >= self.stop.clone() {
            self.ended = true;
            return terrane_collection_support::IterationStep::End;
        }
        let item: terrane_int_support::Int = self.current.clone();
        self.current = self.current.clone() + terrane_int_support::Int::from(1_i128);
        return terrane_collection_support::IterationStep::<
            terrane_int_support::Int,
        >::Item(item.clone());
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
    let values: Counter = Counter::terrane_construct(
        terrane_int_support::Int::from(3_i128),
    );
    let mut __terrane_iterator_0 = values.iterator();
    loop {
        let value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let mut probe: Counter = Counter::terrane_construct(
        terrane_int_support::Int::from(3_i128),
    );
    probe.next();
    probe.next();
    probe.next();
    let fourth: terrane_collection_support::IterationStep<terrane_int_support::Int> = probe
        .next();
    let fifth: terrane_collection_support::IterationStep<terrane_int_support::Int> = probe
        .next();
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&matches!(&fourth,
        terrane_collection_support::IterationStep::End)),
        terrane_scalar_support::scalar_text(&matches!(&fifth,
        terrane_collection_support::IterationStep::End)),
        terrane_scalar_support::scalar_text(&probe.advances)
    );
    let mut __terrane_iterator_1 = NoneOnce::terrane_construct().iterator();
    loop {
        let missing = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&missing));
    }
}
