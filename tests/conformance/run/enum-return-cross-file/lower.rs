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
    let item: TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74<
        String,
    >;
    let value: String;
    item = make();
    let __terrane_match_value_440 = item.clone();
    match __terrane_match_value_440 {
        TerraneNsEnumReturnCrossFileResultIdentity2F656E756D2D72657475726E2D63726F73732D66696C6500726573756C74::Success(
            __terrane_pattern_459_value,
        ) => {
            value = __terrane_pattern_459_value;
            println!("{}", terrane_scalar_support::scalar_text(&value));
        }
    }
}
