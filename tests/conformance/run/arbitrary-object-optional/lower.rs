// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-scalar-support
// Source: src/main.trn
// Namespace: app
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S44 {
    Arm0(Widget),
    Arm1(()),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S44 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(_) => {
                unreachable!("semantic display refinement excludes this union arm")
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
#[derive(Clone)]
pub struct Widget {}
impl Widget {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    let value_terrane_f0_s44: TerraneUnionF0S44;
    value_terrane_f0_s44 = TerraneUnionF0S44::Arm1(());
    if matches!(&value_terrane_f0_s44, TerraneUnionF0S44::Arm1(_)) {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("empty")));
    }
}
