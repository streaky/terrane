// Generated deterministically by Terrane <version>.
// Source: src/main.trn
// Namespace: enum-return-cross-file
#[derive(Clone)]
pub enum TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74<
    TerraneType54,
> {
    Success(TerraneType54),
}
fn make() -> TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74<
    String,
> {
    return {
        let __terrane_enum_payload_0 = String::from("ready");
        TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74::<
            String,
        >::Success(__terrane_enum_payload_0)
    };
}
fn main() {
    let number: TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74<
        terrane_int_support::Int,
    >;
    let value: terrane_int_support::Int;
    let item: TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74<
        String,
    >;
    let value_2: String;
    number = other();
    let __terrane_match_value_443 = number.clone();
    match __terrane_match_value_443 {
        TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74::Success(
            __terrane_pattern_464_value,
        ) => {
            value = __terrane_pattern_464_value;
            println!("{}", terrane_scalar_support::scalar_text(&value));
        }
    }
    item = make();
    let __terrane_match_value_538 = item.clone();
    match __terrane_match_value_538 {
        TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74::Success(
            __terrane_pattern_557_value,
        ) => {
            value_2 = __terrane_pattern_557_value;
            println!("{}", terrane_scalar_support::scalar_text(&value_2));
        }
    }
}
// Source: src/other.trn
// Namespace: enum-return-cross-file
fn other() -> TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74<
    terrane_int_support::Int,
> {
    return {
        let __terrane_enum_payload_0 = terrane_int_support::Int::from(1_i128);
        TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74::<
            terrane_int_support::Int,
        >::Success(__terrane_enum_payload_0)
    };
}
