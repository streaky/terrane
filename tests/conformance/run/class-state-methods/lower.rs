// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: class-state-methods
#[derive(Clone)]
pub struct Counter {
    __terrane_lifetime: std::sync::Arc<()>,
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct(start: terrane_int_support::Int) -> Self {
        let mut value = Self {
            value: terrane_int_support::Int::from(0_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        value.construct(start);
        value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, start: terrane_int_support::Int) {
        self.value = start.clone();
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.value = self.value.clone() + amount.clone();
        return self.value.clone();
    }
    pub fn shifted(&self, amount: terrane_int_support::Int) -> terrane_int_support::Int {
        return self.value.clone() + amount.clone();
    }
    pub fn destruct(&self) {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("destruct")));
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
fn main() {
    let mut first: Counter = Counter::terrane_construct(
        terrane_int_support::Int::from(10_i128),
    );
    let mut second: Counter = first.terrane_separate();
    let shift: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    > = {
        let receiver = first.terrane_separate();
        std::sync::Arc::new(move |argument_0: terrane_int_support::Int| {
            receiver.shifted(argument_0)
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&first
        .increase(terrane_int_support::Int::from(5_i128)))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&second
        .increase(terrane_int_support::Int::from(2_i128)))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&shift(terrane_int_support::Int::from(3_i128)))
    );
}
