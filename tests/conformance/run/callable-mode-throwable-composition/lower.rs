// Generated deterministically by Terrane <version>.
// Runtime support: consuming_callable.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: callable-mode-throwable-composition
#[derive(Clone)]
pub struct Accumulator {
    pub total: terrane_int_support::Int,
}
impl Accumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn add(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        if delta.clone() < terrane_int_support::Int::from(0_i128) {
            return Err(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    0 /* terrane-site: case.trn:9:7-9:27 */,
                ),
            );
        }
        self.total = self.total.clone() + delta.clone();
        return Ok(self.total.clone());
    }
}
fn main() {
    let value: Accumulator = Accumulator::terrane_construct();
    let operation: TerraneConsumingCallable<
        (terrane_int_support::Int,),
        Result<terrane_int_support::Int, TerraneError>,
    > = {
        let mut receiver = value;
        TerraneConsumingCallable::new(move |(argument_0,): (terrane_int_support::Int,)| {
            receiver.add(argument_0)
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = operation; "consuming"
        .to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = operation; "throwable"
        .to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(operation
        .call((terrane_int_support::Int::from(4_i128),)), 1 /* terrane-site: case.trn:18:11-18:23 */))
    );
}
