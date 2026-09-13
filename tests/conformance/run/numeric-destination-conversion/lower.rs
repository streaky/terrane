// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: numeric-destination-conversion
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S121 {
    Arm0(i8),
    Arm1(i32),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S121 {
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
    let small: i8 = 12;
    let adaptive: i64 = small as i64;
    let wide: i32 = small as i32;
    let mut selected: TerraneUnionF0S121 = TerraneUnionF0S121::Arm0(small);
    let count: i32 = 16777216;
    let total: f64 = count as f64;
    let exact: i64 = 18014398509481984;
    let exact_float: f64 = __terrane_raised(
        terrane_int_support::exact_f64(&exact),
        0 /* terrane-site: case.trn:11:23-11:28 */,
    );
    let whole: f64 = 4.0;
    let converted: terrane_int_support::Int = __terrane_raised(
        terrane_int_support::exact_int_f64(whole),
        1 /* terrane-site: case.trn:13:19-13:24 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&wide));
    println!("{}", terrane_scalar_support::scalar_text(&selected));
    let other: i32 = 13;
    selected = TerraneUnionF0S121::Arm1(other);
    println!(
        "{}", terrane_scalar_support::scalar_text(&matches!(&selected,
        TerraneUnionF0S121::Arm1(_)))
    );
    selected = TerraneUnionF0S121::Arm0(small);
    println!(
        "{}", terrane_scalar_support::scalar_text(&matches!(&selected,
        TerraneUnionF0S121::Arm0(_)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!("{}", terrane_scalar_support::scalar_text(&exact_float));
    println!("{}", terrane_scalar_support::scalar_text(&converted));
    let myvar: i64 = 12;
    let slop: i8;
    slop = {
        let source_value = __terrane_raised(
            terrane_int_support::fixed_addition(myvar, 1),
            2 /* terrane-site: case.trn:27:10-27:19 */,
        );
        __terrane_raised(
            i8::try_from(source_value)
                .map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(
                    &source_value,
                    "int64",
                    "int8",
                    "the value is outside the destination range",
                )),
            2 /* terrane-site: case.trn:27:10-27:19 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&slop));
}
