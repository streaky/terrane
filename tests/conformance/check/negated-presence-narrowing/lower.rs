// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: case.trn
// Namespace: negated-presence-narrowing
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S57 {
    Arm0(i8),
    Arm1(()),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S57 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
fn main() {
    let value: TerraneUnionF0S57;
    value = TerraneUnionF0S57::Arm0(7);
    if !matches!(&value, TerraneUnionF0S57::Arm1(_)) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &value {
            TerraneUnionF0S57::Arm0(value) => value, _ =>
            unreachable!("flow-proven storage refinement") })
        );
    }
}
