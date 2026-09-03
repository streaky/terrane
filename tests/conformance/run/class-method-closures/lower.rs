// Generated deterministically by Terrane <version>.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: class-method-closures
#[derive(Clone)]
pub struct Maker {
    pub base: terrane_int_support::Int,
}
impl Maker {
    pub fn terrane_construct() -> Self {
        Self {
            base: terrane_int_support::Int::from(10_i128),
        }
    }
    pub fn offset(
        &self,
    ) -> std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    > {
        return {
            let this = self.clone();
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> terrane_int_support::Int {
                return this.base.clone() + value.clone();
            })
        };
    }
}
fn main() {
    let value: Maker = Maker::terrane_construct();
    let add: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    > = value.offset();
    let result: terrane_int_support::Int = add(terrane_int_support::Int::from(5_i128));
    println!("{}", terrane_scalar_support::scalar_text(&result));
}
