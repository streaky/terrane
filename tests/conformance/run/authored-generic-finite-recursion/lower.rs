// Generated deterministically by Terrane <version>.
// Source: case.trn
// Namespace: authored-generic-finite-recursion
fn count_down<TerraneType54>(
    value: TerraneType54,
    count: terrane_int_support::Int,
) -> terrane_int_support::Int {
    if count == terrane_int_support::Int::from(0_i128) {
        return terrane_int_support::Int::from(0_i128);
    }
    return count_down(value, count.clone() - terrane_int_support::Int::from(1_i128));
}
#[derive(Clone)]
pub struct TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78<
    TerraneType54,
> {
    pub value: Option<TerraneType54>,
}
impl<
    TerraneType54,
> TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78<
    TerraneType54,
> {
    pub fn terrane_construct(input: TerraneType54) -> Self {
        let mut __terrane_constructed_value = Self { value: None };
        __terrane_constructed_value.construct(input);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, input: TerraneType54) {
        {
            let __terrane_value = input;
            self.value = Some(__terrane_value);
        }
    }
    pub fn count_down(
        &self,
        count: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        if count == terrane_int_support::Int::from(0_i128) {
            return terrane_int_support::Int::from(0_i128);
        }
        return self.count_down(count.clone() - terrane_int_support::Int::from(1_i128));
    }
}
fn main() {
    let boxed: TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78<
        String,
    >;
    println!(
        "{}", terrane_scalar_support::scalar_text(&count_down:: < String >
        (String::from("same"), terrane_int_support::Int::from(3_i128)))
    );
    boxed = TerraneNsAuthoredGenericFiniteRecursionBoxIdentity2F617574686F7265642D67656E657269632D66696E6974652D726563757273696F6E00626F78::<
        String,
    >::terrane_construct(String::from("same"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&boxed
        .count_down(terrane_int_support::Int::from(3_i128)))
    );
}
