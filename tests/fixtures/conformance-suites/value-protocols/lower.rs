// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: tests/integration/checked-coercion/case.trn
// Namespace: checked-coercion
static __TERRANE_F1_SHARED: std::sync::LazyLock<terrane_int_support::Int> = std::sync::LazyLock::new(||
terrane_int_support::Int::from(100_i128));
fn convert(item: terrane_int_support::Int) -> Result<i8, TerraneError> {
    let mut result: i8;
    result = 0;
    if item > terrane_int_support::Int::from(0_i128) {
        result = __terrane_raised_err(
            terrane_int_support::coerce::<i8>(&item),

                0 /* terrane-site: tests/integration/checked-coercion/case.trn:9:14-9:18 */,
        )?;
    }
    return Ok(result);
}
fn helper() {
    println!("{}", String::from("helper"));
}
fn test_values_terrane_checked_coercion() -> Result<(), TerraneError> {
    let value: i64;
    let within: i64;
    let coerced: i8;
    let renamed_checked: Option<i8>;
    let shared_coerced: i8;
    let parameter_coerced: i8;
    let checked: Option<i8>;
    let absent: bool;
    let present: bool;
    let shadow_safe: i8;
    helper();
    value = 300;
    within = 100;
    coerced = __terrane_raised_err(
        terrane_int_support::coerce::<i8>(&within),
        1 /* terrane-site: tests/integration/checked-coercion/case.trn:19:13-19:19 */,
    )?;
    renamed_checked = terrane_int_support::checked_coerce::<i8>(&within);
    shared_coerced = __terrane_raised_err(
        terrane_int_support::coerce::<i8>(&*__TERRANE_F1_SHARED),
        2 /* terrane-site: tests/integration/checked-coercion/case.trn:21:20-21:33 */,
    )?;
    parameter_coerced = __terrane_traced_err(
        convert(terrane_int_support::Int::from(within as i128)),
        3 /* terrane-site: tests/integration/checked-coercion/case.trn:22:23-22:38 */,
    )?;
    checked = terrane_int_support::checked_coerce::<i8>(&value);
    absent = checked.is_none();
    present = checked.is_some();
    shadow_safe = terrane_int_support::saturating_coerce::<i8>(&value);
    println!("{}", terrane_scalar_support::scalar_text(&coerced));
    println!("{}", terrane_scalar_support::scalar_text(&renamed_checked.is_some()));
    println!("{}", terrane_scalar_support::scalar_text(&shared_coerced));
    println!("{}", terrane_scalar_support::scalar_text(&parameter_coerced));
    println!("{}", terrane_scalar_support::scalar_text(&absent));
    println!("{}", terrane_scalar_support::scalar_text(&present));
    println!("{}", terrane_scalar_support::scalar_text(&shadow_safe));
    Ok(())
}
// Source: tests/integration/explicit-numeric-coercions/case.trn
// Namespace: explicit-numeric-coercions
fn divide(numerator: f64, denominator: f64) -> f64 {
    return numerator / denominator;
}
fn test_values_terrane_explicit_numeric_coercions() -> Result<(), TerraneError> {
    let quantity: i64;
    let rounded64: f64;
    let rounded32: f32;
    let signed: i8;
    let signed_float: f32;
    let maximum: u128;
    let maximum_float: f64;
    let precise: f64;
    let narrowed: f32;
    let widened: f64;
    let infinity64: f64;
    let infinity32: f32;
    let nan64: f64;
    let nan32: f32;
    let checked_integer: Option<f32>;
    let base: f64;
    let too_large: f64;
    let checked_float: Option<f32>;
    let mut impossible: Option<f32> = None;
    quantity = 9007199254740993;
    rounded64 = __terrane_raised_err(
        terrane_int_support::coerce_to_f64(&quantity),

            4 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:10:23-10:38 */,
    )?;
    rounded32 = __terrane_raised_err(
        terrane_int_support::coerce_to_f32(&quantity),

            5 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:11:23-11:38 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&rounded64));
    println!("{}", terrane_scalar_support::scalar_text(&rounded32));
    signed = -7;
    signed_float = signed as f32;
    maximum = 340282366920938463463374607431768211455;
    maximum_float = maximum as f64;
    println!("{}", terrane_scalar_support::scalar_text(&signed_float));
    println!("{}", terrane_scalar_support::scalar_text(&maximum_float));
    precise = 16777217.0;
    narrowed = __terrane_raised_err(
        terrane_int_support::coerce_f64_to_f32(precise),

            6 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:23:22-23:36 */,
    )?;
    widened = narrowed as f64;
    println!("{}", terrane_scalar_support::scalar_text(&narrowed));
    println!("{}", terrane_scalar_support::scalar_text(&widened));
    infinity64 = divide(1.0, 0.0);
    infinity32 = __terrane_raised_err(
        terrane_int_support::coerce_f64_to_f32(infinity64),

            7 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:29:24-29:41 */,
    )?;
    nan64 = divide(0.0, 0.0);
    nan32 = __terrane_raised_err(
        terrane_int_support::coerce_f64_to_f32(nan64),

            8 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:31:19-31:31 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&infinity32.is_infinite()));
    println!("{}", terrane_scalar_support::scalar_text(&nan32.is_nan()));
    checked_integer = terrane_int_support::coerce_fixed_to_f32(maximum).ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_integer.is_none()));
    base = 65536.0;
    too_large = base * base * base * base * base * base * base * base;
    checked_float = terrane_int_support::coerce_f64_to_f32(too_large).ok();
    println!("{}", terrane_scalar_support::scalar_text(&checked_float.is_none()));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = impossible
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::coerce_fixed_to_f32(maximum),
                        9 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:44:26-44:40 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&impossible.as_ref()
                .expect("flow-proven available binding"))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("integer overflow caught"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            let _ = impossible
                .insert(
                    __terrane_raised_completion!(
                        terrane_int_support::coerce_f64_to_f32(too_large),
                        10 /* terrane-site: tests/integration/explicit-numeric-coercions/case.trn:50:26-50:42 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&impossible.as_ref()
                .expect("flow-proven available binding"))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("float overflow caught"));
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/integer-coercions/case.trn
// Namespace: integer-coercions
fn test_values_terrane_integer_coercions() -> Result<(), TerraneError> {
    let value: i64;
    let same: terrane_int_support::Int;
    let exact: i8;
    let wrapped: u8;
    let saturated: u8;
    value = 300;
    same = terrane_int_support::Int::from(value as i128);
    exact = __terrane_raised_err(
        terrane_int_support::coerce::<i8>(&120),
        11 /* terrane-site: tests/integration/integer-coercions/case.trn:6:11-6:14 */,
    )?;
    wrapped = terrane_int_support::wrapping_coerce::<
        u8,
    >(
        &(terrane_int_support::Int::from(value as i128)
            + terrane_int_support::Int::from(0_i128)),
    );
    saturated = terrane_int_support::saturating_coerce::<u8>(&value);
    println!("{}", terrane_scalar_support::scalar_text(&exact));
    println!("{}", terrane_scalar_support::scalar_text(&wrapped));
    println!("{}", terrane_scalar_support::scalar_text(&saturated));
    println!("{}", terrane_scalar_support::scalar_text(&same));
    Ok(())
}
// Source: tests/integration/mixed-width-arithmetic/case.trn
// Namespace: mixed-width-arithmetic
fn test_values_terrane_mixed_width_arithmetic() {
    let left: i8;
    let right: i32;
    let unsigned: u8;
    let total: i32;
    let combined: i16;
    left = 100;
    right = 2;
    unsigned = 120;
    total = __terrane_raised(
        terrane_int_support::fixed_addition(left as i32, right),

            12 /* terrane-site: tests/integration/mixed-width-arithmetic/case.trn:7:17-7:29 */,
    );
    combined = __terrane_raised(
        terrane_int_support::fixed_addition(left as i16, unsigned as i16),

            13 /* terrane-site: tests/integration/mixed-width-arithmetic/case.trn:8:20-8:35 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!("{}", terrane_scalar_support::scalar_text(&combined));
}
// Source: tests/integration/mixed-width-comparisons/case.trn
// Namespace: mixed-width-comparisons
fn test_values_terrane_mixed_width_comparisons() {
    let small: i8;
    let wide: i32;
    small = 5;
    wide = 9;
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 == wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 != wide)));
    println!("{}", terrane_scalar_support::scalar_text(&((small as i32) < wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 <= wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 > wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(small as i32 >= wide)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide == small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide != small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide < small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide <= small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide > small as i32)));
    println!("{}", terrane_scalar_support::scalar_text(&(wide >= small as i32)));
}
// Source: tests/integration/numeric-destination-conversion/case.trn
// Namespace: numeric-destination-conversion
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF6S128 {
    Arm0(i8),
    Arm1(i32),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF6S128 {
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
fn test_values_terrane_numeric_destination_conversion() -> Result<(), TerraneError> {
    let small: i8;
    let adaptive: i64;
    let wide: i32;
    let mut selected: TerraneUnionF6S128;
    let count: i32;
    let total: f64;
    let exact: i64;
    let exact_float: f64;
    let whole: f64;
    let converted: terrane_int_support::Int;
    let other: i32;
    let myvar: i64;
    let slop: i8;
    small = 12;
    adaptive = small as i64;
    wide = small as i32;
    selected = TerraneUnionF6S128::Arm0(small);
    count = 16777216;
    total = count as f64;
    exact = 18014398509481984;
    exact_float = __terrane_raised_err(
        terrane_int_support::exact_f64(&exact),

            14 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:11:23-11:28 */,
    )?;
    whole = 4.0;
    converted = __terrane_raised_err(
        terrane_int_support::exact_int_f64(whole),

            15 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:13:19-13:24 */,
    )?;
    println!("{}", terrane_scalar_support::scalar_text(&adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&wide));
    println!("{}", terrane_scalar_support::scalar_text(&selected));
    other = 13;
    selected = TerraneUnionF6S128::Arm1(other);
    println!(
        "{}", terrane_scalar_support::scalar_text(&matches!(&selected,
        TerraneUnionF6S128::Arm1(_)))
    );
    selected = TerraneUnionF6S128::Arm0(small);
    println!(
        "{}", terrane_scalar_support::scalar_text(&matches!(&selected,
        TerraneUnionF6S128::Arm0(_)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&total));
    println!("{}", terrane_scalar_support::scalar_text(&exact_float));
    println!("{}", terrane_scalar_support::scalar_text(&converted));
    myvar = 12;
    slop = {
        let source_value = __terrane_raised_err(
            terrane_int_support::fixed_addition(myvar, 1),

                16 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:27:10-27:19 */,
        )?;
        __terrane_raised_err(
            i8::try_from(source_value)
                .map_err(|_| terrane_int_support::ArithmeticError::conversion_overflow(
                    &source_value,
                    "int64",
                    "int8",
                    "the value is outside the destination range",
                )),

                16 /* terrane-site: tests/integration/numeric-destination-conversion/case.trn:27:10-27:19 */,
        )?
    };
    println!("{}", terrane_scalar_support::scalar_text(&slop));
    Ok(())
}
// Source: tests/integration/numeric-literals/case.trn
// Namespace: numeric-literals
fn test_values_terrane_numeric_literals() {
    let adaptive: i64;
    let single: f32;
    let double: f64;
    let inferred: f64;
    let signed_value: i8;
    let unsigned_value: u8;
    let minimum: i8;
    let negative_hex: i8;
    adaptive = 16;
    single = 1.5_f32;
    double = 2.25;
    inferred = 3.5;
    signed_value = 127;
    unsigned_value = 255;
    minimum = -128;
    negative_hex = -16;
    println!("{}", terrane_scalar_support::scalar_text(&adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&single));
    println!("{}", terrane_scalar_support::scalar_text(&double));
    println!("{}", terrane_scalar_support::scalar_text(&inferred));
    println!("{}", terrane_scalar_support::scalar_text(&signed_value));
    println!("{}", terrane_scalar_support::scalar_text(&unsigned_value));
    println!("{}", terrane_scalar_support::scalar_text(&minimum));
    println!("{}", terrane_scalar_support::scalar_text(&negative_hex));
}
// Source: tests/integration/string-display-boundaries/case.trn
// Namespace: string-display-boundaries
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF8S639 {
    Arm0(
        TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C,
    ),
    Arm1(terrane_int_support::Int),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF8S639 {
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
pub struct TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656CStorage {}
impl TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656CStorage {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_746f2d737472696e67(&self) -> String {
        return String::from("unsafe");
    }
    pub fn to_string(&self) -> String {
        return String::from("label");
    }
}
#[derive(Clone)]
pub enum TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C {
    Own(
        TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656CStorage,
    ),
    TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64(
        TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64,
    ),
}
impl TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C {
    pub fn terrane_construct() -> Self {
        Self::Own(
            TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656CStorage::terrane_construct(),
        )
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_746f2d737472696e67(&self) -> String {
        match self {
            Self::Own(value) => unsafe { value._terrane_unsafe_746f2d737472696e67() }
            Self::TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64(
                value,
            ) => unsafe { value._terrane_unsafe_746f2d737472696e67() }
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Own(value) => value.to_string(),
            Self::TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64(
                value,
            ) => value.to_string(),
        }
    }
}
#[derive(Clone)]
pub struct TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64 {
    pub extra: terrane_int_support::Int,
}
impl TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64 {
    pub fn terrane_construct() -> Self {
        Self {
            extra: terrane_int_support::Int::from(3_i128),
        }
    }
    #[allow(unsafe_code)]
    pub unsafe fn _terrane_unsafe_746f2d737472696e67(&self) -> String {
        return String::from("unsafe");
    }
    pub fn to_string(&self) -> String {
        return String::from("label");
    }
}
fn describe_borrow(
    value: &TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C,
) -> String {
    return {
        let mut __terrane_text = String::with_capacity(9);
        __terrane_text.push_str("borrowed:");
        __terrane_text.push_str(&(*value).to_string());
        __terrane_text
    };
}
fn describe_text(value: &String) -> String {
    return {
        let mut __terrane_text = String::with_capacity(5);
        __terrane_text.push_str("ref:");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &*value,
            &mut __terrane_text,
        );
        __terrane_text.push_str("|");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &terrane_scalar_support::scalar_text(&*value),
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn test_boundaries_terrane_string_display_boundaries() {
    let item: TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64;
    let mut optional: Option<
        TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C,
    >;
    let mut mixed: TerraneUnionF8S639;
    let mut value: terrane_int_support::Int;
    let render: std::sync::Arc<dyn Fn() -> String + Send + Sync>;
    let borrowed: TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C;
    let text: String;
    let shared_text: std::sync::Arc<TerraneSharedCell<String>>;
    let shared: std::sync::Arc<TerraneSharedCell<String>>;
    item = TerraneNsStringDisplayBoundariesChildIdentity2F737472696E672D646973706C61792D626F756E646172696573006368696C64::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("inherited:"); __terrane_text
        .push_str(&item.to_string()); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6); __terrane_text.push_str("again:"); __terrane_text
        .push_str(&item.to_string()); __terrane_text })
    );
    optional = None;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("optional:"); match &optional {
        Some(__terrane_present) => { __terrane_text.push_str(&__terrane_present
        .to_string()); }, None => __terrane_text.push_str("none"), } __terrane_text })
    );
    optional = Some(
        TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C::terrane_construct(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(9); __terrane_text.push_str("optional:"); match &optional {
        Some(__terrane_present) => { __terrane_text.push_str(&__terrane_present
        .to_string()); }, None => __terrane_text.push_str("none"), } __terrane_text })
    );
    mixed = TerraneUnionF8S639::Arm1(terrane_int_support::Int::from(7_i128));
    if item.extra.clone() == terrane_int_support::Int::from(3_i128) {
        mixed = TerraneUnionF8S639::Arm0(
            TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C::terrane_construct(),
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6); __terrane_text.push_str("union:"); match &mixed {
        TerraneUnionF8S639::Arm0(__terrane_arm) => { __terrane_text
        .push_str(&__terrane_arm.to_string()); }, TerraneUnionF8S639::Arm1(__terrane_arm)
        => { terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_arm, &mut
        __terrane_text); } } __terrane_text })
    );
    value = terrane_int_support::Int::from(42_i128);
    render = {
        let receiver = value.clone();
        std::sync::Arc::new(move || terrane_scalar_support::scalar_text(&receiver))
    };
    value = terrane_int_support::Int::from(99_i128);
    println!(
        "{}{}{}{}", String::from("bound:"),
        terrane_scalar_support::scalar_text(&render()), String::from("|current:"),
        terrane_scalar_support::scalar_text(&value)
    );
    borrowed = TerraneNsStringDisplayBoundariesLabelIdentity2F737472696E672D646973706C61792D626F756E646172696573006C6162656C::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&describe_borrow(&borrowed)));
    text = String::from("text");
    println!("{}", terrane_scalar_support::scalar_text(&describe_text(&text)));
    shared_text = std::sync::Arc::new(TerraneSharedCell::new(String::from("text")));
    shared = shared_text.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("shared:");
        terrane_scalar_support::ScalarDisplay::write_scalar(&* shared.lock()
        .expect("shared reference lock poisoned"), &mut __terrane_text); __terrane_text
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&*
        shared.lock().expect("shared reference lock poisoned")))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&* shared.lock()
        .expect("shared reference lock poisoned"))
    );
}
// Source: tests/integration/string-display-composition/case.trn
// Namespace: string-display-composition
#[derive(Clone)]
pub struct RenderError {
    pub message: String,
}
impl RenderError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("cannot render"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C {
    pub text: String,
}
impl TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
    pub fn to_string(&self) -> String {
        println!(
            "{}{}", String::from("render:"), terrane_scalar_support::scalar_text(&self
            .text)
        );
        return self.text.clone();
    }
}
pub trait TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    >;
    fn to_string(&self) -> Result<String, TerraneError>;
}
impl Clone
for Box<
    dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65(
    Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    >,
);
impl Clone
for TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    pub fn to_string(&self) -> Result<String, TerraneError> {
        self.0.to_string()
    }
}
#[derive(Clone)]
pub struct Broken {}
impl Broken {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn to_string(&self) -> Result<String, TerraneError> {
        return Err({
            let value = RenderError::terrane_construct();
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,

                    17 /* terrane-site: tests/integration/string-display-composition/case.trn:23:9-23:37 */,
            )
        });
    }
}
impl TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol
for Broken {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn to_string(&self) -> Result<String, TerraneError> {
        Broken::to_string(&*self)
    }
}
impl From<Broken>
for TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    fn from(value: Broken) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Calm {}
impl Calm {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn to_string(&self) -> String {
        return String::from("calm");
    }
}
impl TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol
for Calm {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn to_string(&self) -> Result<String, TerraneError> {
        Ok(Calm::to_string(&*self))
    }
}
impl From<Calm>
for TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65 {
    fn from(value: Calm) -> Self {
        Self(Box::new(value))
    }
}
fn number(value: terrane_int_support::Int) -> terrane_int_support::Int {
    println!(
        "{}{}", String::from("number:"), terrane_scalar_support::scalar_text(&value)
    );
    return value.clone();
}
fn render_generic<
    TerraneType54: TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65Protocol,
>(value: TerraneType54) -> Result<String, TerraneError> {
    return Ok({
        let mut __terrane_text = String::with_capacity(8);
        __terrane_text.push_str("generic:");
        __terrane_text
            .push_str(
                &__terrane_traced_err(
                    value.to_string(),

                        18 /* terrane-site: tests/integration/string-display-composition/case.trn:34:25-34:30 */,
                )?,
            );
        __terrane_text
    });
}
fn test_composition() -> Result<(), TerraneError> {
    let item: TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C;
    let erased: TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(6);
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_int_support::Int::from(2_i128)
        + terrane_int_support::Int::from(3_i128)), &mut __terrane_text); __terrane_text
        .push_str(" items"); __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("items: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&2, &mut __terrane_text);
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("items: ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&(terrane_int_support::Int::from(2_i128)
        + terrane_int_support::Int::from(3_i128)), &mut __terrane_text); __terrane_text
        })
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(1); __terrane_text.push_str("2");
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text }), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(&3, &mut __terrane_text);
        __terrane_text.push_str("2"); __terrane_text })
    );
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&true)),
        String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&())),
        String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scalar_text(&42))
    );
    item = TerraneNsStringDisplayCompositionLabelIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E006C6162656C::terrane_construct(
        String::from("label"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&item.to_string()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(13); __terrane_text.push_str("before "); __terrane_text
        .push_str(&item.to_string()); __terrane_text.push_str(" after"); __terrane_text
        })
    );
    println!("{}", item.to_string());
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(7); __terrane_text.push_str("prefix "); __terrane_text
        .push_str(&item.to_string());
        terrane_scalar_support::ScalarDisplay::write_scalar(&7, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_separator = &":"; let
        mut __terrane_text = String::with_capacity(1); __terrane_text.push_str(&item
        .to_string()); __terrane_text.push_str(__terrane_separator);
        terrane_scalar_support::ScalarDisplay::write_scalar(&7, &mut __terrane_text);
        __terrane_text })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(8); __terrane_text.push_str("ordered ");
        terrane_scalar_support::ScalarDisplay::write_scalar(&number(terrane_int_support::Int::from(2_i128)),
        &mut __terrane_text); __terrane_text.push_str(&item.to_string());
        terrane_scalar_support::ScalarDisplay::write_scalar(&number(terrane_int_support::Int::from(3_i128)),
        &mut __terrane_text); __terrane_text })
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
                String::with_capacity(7); __terrane_text.push_str("broken ");
                __terrane_text
                .push_str(&__terrane_traced_completion!(Broken::terrane_construct()
                .to_string(), 19 /* terrane-site: tests/integration/string-display-composition/case.trn:50:29-50:45 */));
                __terrane_text })
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("caught addition"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}", __terrane_traced_completion!(Broken::terrane_construct()
                .to_string(), 20 /* terrane-site: tests/integration/string-display-composition/case.trn:54:17-54:33 */)
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("caught print"));
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            {
                let _: String = {
                    let mut __terrane_text = String::with_capacity(0);
                    __terrane_text.push_str("");
                    __terrane_text
                        .push_str(
                            &__terrane_traced_completion!(
                                Broken::terrane_construct().to_string(),
                                21 /* terrane-site: tests/integration/string-display-composition/case.trn:58:25-58:41 */
                            ),
                        );
                    __terrane_text
                };
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_2 = true;
                    println!("{}", String::from("caught concat"));
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_3: TerraneCompletion<()> = (|| {
        let __terrane_try_3: TerraneCompletion<()> = (|| {
            {
                let _: String = {
                    let __terrane_separator = &":";
                    let mut __terrane_text = String::with_capacity(0);
                    __terrane_text
                        .push_str(
                            &__terrane_traced_completion!(
                                Broken::terrane_construct().to_string(),
                                22 /* terrane-site: tests/integration/string-display-composition/case.trn:62:24-62:40 */
                            ),
                        );
                    __terrane_text
                };
            }
            TerraneCompletion::Normal
        })();
        match __terrane_try_3 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_3) => {
                let mut __terrane_handled_3 = false;
                if !__terrane_handled_3
                    && __terrane_error_3.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_3 = true;
                    println!("{}", String::from("caught join"));
                }
                if !__terrane_handled_3 {
                    return TerraneCompletion::Error(__terrane_error_3);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_3 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_4: TerraneCompletion<()> = (|| {
        let __terrane_try_4: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(render_generic(Broken::terrane_construct()),
                23 /* terrane-site: tests/integration/string-display-composition/case.trn:66:17-66:51 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_4 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_4) => {
                let mut __terrane_handled_4 = false;
                if !__terrane_handled_4
                    && __terrane_error_4.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_4 = true;
                    println!("{}", String::from("caught generic"));
                }
                if !__terrane_handled_4 {
                    return TerraneCompletion::Error(__terrane_error_4);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_4 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(render_generic(Calm::terrane_construct()),
        24 /* terrane-site: tests/integration/string-display-composition/case.trn:69:13-69:45 */) ?)
    );
    erased = <TerraneNsStringDisplayCompositionDisplayableIdentity2F737472696E672D646973706C61792D636F6D706F736974696F6E00646973706C617961626C65>::from(
        Calm::terrane_construct(),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("interface:"); __terrane_text
        .push_str(&__terrane_traced_err(erased.to_string(), 25 /* terrane-site: tests/integration/string-display-composition/case.trn:71:27-71:33 */) ?);
        __terrane_text })
    );
    Ok(())
}
// Source: tests/integration/string-display-generic/case.trn
// Namespace: string-display-generic
pub trait TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol: Send + Sync {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    >;
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    >;
    fn to_string(&self) -> String;
}
impl Clone
for Box<
    dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65(
    Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    >,
);
impl Clone
for TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65 {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Clone)]
pub struct TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C {
    pub text: String,
}
impl TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::new() };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
    pub fn to_string(&self) -> String {
        return self.text.clone();
    }
}
impl TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol
for TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C {
    fn clone_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn separate_box(
        &self,
    ) -> Box<
        dyn TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
    > {
        Box::new(self.clone())
    }
    fn to_string(&self) -> String {
        TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C::to_string(
            &*self,
        )
    }
}
impl From<
    TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C,
>
for TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65 {
    fn from(
        value: TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn decorate<
    TerraneType54: TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
>(value: TerraneType54) -> String {
    println!("{}", value.to_string());
    return {
        let mut __terrane_text = String::with_capacity(8);
        __terrane_text.push_str("generic:");
        __terrane_text.push_str(&value.to_string());
        __terrane_text
    };
}
fn compose<
    TerraneType54: TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65Protocol,
>(value: TerraneType54) -> String {
    return {
        let mut __terrane_text = String::with_capacity(1);
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &{
                let mut __terrane_text = String::with_capacity(7);
                __terrane_text.push_str("prefix ");
                __terrane_text.push_str(&value.to_string());
                __terrane_text
            },
            &mut __terrane_text,
        );
        __terrane_text.push_str("|");
        terrane_scalar_support::ScalarDisplay::write_scalar(
            &{
                let __terrane_separator = &":";
                let mut __terrane_text = String::with_capacity(1);
                __terrane_text.push_str(&value.to_string());
                __terrane_text.push_str(__terrane_separator);
                __terrane_text.push_str(&value.to_string());
                __terrane_text
            },
            &mut __terrane_text,
        );
        __terrane_text
    };
}
fn test_generic_terrane_string_display_generic() {
    let value: TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C;
    let erased: TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65;
    value = TerraneNsStringDisplayGenericLabelIdentity2F737472696E672D646973706C61792D67656E65726963006C6162656C::terrane_construct(
        String::from("label"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&decorate(value.clone())));
    println!("{}", terrane_scalar_support::scalar_text(&compose(value.clone())));
    erased = <TerraneNsStringDisplayGenericDisplayableIdentity2F737472696E672D646973706C61792D67656E6572696300646973706C617961626C65>::from(
        value,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
        String::with_capacity(10); __terrane_text.push_str("interface:"); __terrane_text
        .push_str(&erased.to_string()); __terrane_text })
    );
}
// Source: tests/integration/typed-serialization-boundaries/case.trn
// Namespace: typed-serialization-boundaries
#[derive(Clone)]
pub struct TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    pub value: String,
}
impl TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    pub fn terrane_construct() -> Self {
        Self { value: String::from("") }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("value")?;
        TerraneDocumentEncode::terrane_encode_document(&self.value, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
    ) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    pub children: terrane_collection_support::List<
        TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
    >,
    pub name: String,
}
impl TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    pub fn terrane_construct() -> Self {
        Self {
            children: terrane_collection_support::List::<
                TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
            >::new(
                vec![
                    TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64::terrane_construct()
                ],
            ),
            name: String::from(""),
        }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("children")?;
        TerraneDocumentEncode::terrane_encode_document(&self.children, writer)?;
        writer.key("marker-like")?;
        TerraneDocumentEncode::terrane_encode_document(&self.name, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_boundaries_terrane_typed_serialization_boundaries() {
    let mut nested: TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64;
    let mut value: TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264;
    let normal: TerraneDocumentDecodeOutcome<String>;
    let shallow: TerraneDocumentDecodeOutcome<String>;
    let tiny: TerraneDocumentDecodeOutcome<String>;
    let marker: DocumentResult;
    let oversized: DocumentResult;
    let rendered: DocumentResult;
    let scientific: TerraneDocumentDecodeOutcome<terrane_int_support::Int>;
    let expanded: TerraneDocumentDecodeOutcome<terrane_int_support::Int>;
    let rounded: TerraneDocumentDecodeOutcome<f32>;
    let underflow: TerraneDocumentDecodeOutcome<f32>;
    let nonfinite: f64;
    let rejected: TerraneDocumentDecodeOutcome<String>;
    nested = TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64::terrane_construct();
    {
        let __terrane_value = String::from("x");
        nested.value = __terrane_value;
    }
    value = TerraneNsTypedSerializationBoundariesRecordIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573007265636F7264::terrane_construct();
    {
        let __terrane_value = terrane_collection_support::List::<
            TerraneNsTypedSerializationBoundariesChildIdentity2F74797065642D73657269616C697A6174696F6E2D626F756E646172696573006368696C64,
        >::new(vec![nested]);
        value.children = __terrane_value;
    }
    {
        let __terrane_value = String::from("user-data");
        value.name = __terrane_value;
    }
    normal = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:20:14",
            "record",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(normal.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&normal.value
        .clone())
    );
    shallow = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(1024_i128),
        );
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:22:15",
            "record",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(shallow.diagnostics.length() != 0))
    );
    tiny = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(1_i128),
        );
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:24:12",
            "record",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(tiny.diagnostics.length() != 0))
    );
    marker = parse_json(
        String::from(
            "{\"$serde_json::private::Number\":\"ordinary\",\"value\":123456789012345678901234567890}",
        ),
        default_json_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&marker.failed), String::from("|"),
        terrane_scalar_support::scalar_text(&marker.value.encoded)
    );
    oversized = parse_json(String::from("1e100001"), default_json_options());
    println!("{}", terrane_scalar_support::scalar_text(&oversized.failed));
    rendered = stringify_json(
        marker.value.clone(),
        JsonOptions::terrane_construct(
            terrane_int_support::Int::from(8_i128),
            terrane_int_support::Int::from(8_i128),
        ),
    );
    println!("{}", terrane_scalar_support::scalar_text(&rendered.failed));
    scientific = {
        let __terrane_document_source = &String::from("1e+3");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_int_support::Int,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || terrane_int_support::Int::from(0_i128),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:32:18",
            "int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(scientific.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&scientific.value
        .clone())
    );
    expanded = {
        let __terrane_document_source = &String::from("1e100000");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_int_support::Int,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || terrane_int_support::Int::from(0_i128),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:34:16",
            "int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(expanded.diagnostics.length() != 0))
    );
    rounded = {
        let __terrane_document_source = &String::from(
            "1.000000059604644775390625000000000001",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f32,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <f32>::default(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:36:15",
            "float32",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(rounded.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&(rounded.value
        .clone() > 1.0_f32))
    );
    underflow = {
        let __terrane_document_source = &String::from("1e-100000");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f32,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <f32>::default(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-boundaries/case.trn:38:17",
            "float32",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(underflow.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&(underflow.value
        .clone() == 0.0_f32))
    );
    nonfinite = f64::INFINITY;
    rejected = {
        let __terrane_document_source = &nonfinite;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-boundaries/case.trn:42:16",
            "float64",
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(rejected.diagnostics.length() != 0))
    );
}
// Source: tests/integration/typed-serialization-custom/case.trn
// Namespace: typed-serialization-custom
#[derive(Clone)]
pub struct Code {
    pub hidden: String,
    pub text: String,
}
impl Code {
    pub fn terrane_construct() -> Self {
        Self {
            hidden: String::from("must-not-leak"),
            text: String::from("public-code"),
        }
    }
    pub fn to_document(&self) -> DocumentValue {
        return make_document_string(self.text.clone());
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return parse_json(value.scalar.clone(), default_json_options());
    }
}
impl TerraneDocumentEncode for Code {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        terrane_document_support::write_document_value(writer, &self.to_document().raw)
    }
}
impl SerializableProtocol for Code {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        Code::to_document(&*self)
    }
}
impl From<Code> for Serializable {
    fn from(value: Code) -> Self {
        Self(Box::new(value))
    }
}
impl DeserializableProtocol for Code {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn from_document(&self, value: DocumentValue) -> DocumentResult {
        Code::from_document(&*self, value)
    }
}
impl From<Code> for Deserializable {
    fn from(value: Code) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Envelope {
    pub code: Code,
}
impl Envelope {
    pub fn terrane_construct() -> Self {
        Self {
            code: Code::terrane_construct(),
        }
    }
}
impl TerraneDocumentEncode for Envelope {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("code")?;
        TerraneDocumentEncode::terrane_encode_document(&self.code, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Envelope {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Envelope> for DocumentEncodable {
    fn from(value: Envelope) -> Self {
        Self(Box::new(value))
    }
}
fn test_custom() {
    let custom: Code;
    let direct: TerraneDocumentDecodeOutcome<String>;
    let wrapped: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    let decoded: DocumentResult;
    let from_yaml: DocumentResult;
    custom = Code::terrane_construct();
    direct = {
        let __terrane_document_source = &custom;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-custom/case.trn:19:14",
            "code",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(direct.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&direct.value
        .clone())
    );
    wrapped = {
        let __terrane_document_source = &Envelope::terrane_construct();
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-custom/case.trn:21:15",
            "envelope",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(wrapped.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&wrapped.value
        .clone())
    );
    yaml = {
        let __terrane_document_source = &custom;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-custom/case.trn:23:12",
            "code",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
    decoded = decode_json(
        String::from("\"7\""),
        <Deserializable>::from(custom.clone()),
        default_json_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&decoded.failed),
        String::from("|"), terrane_scalar_support::scalar_text(&decoded.value.scalar)
    );
    from_yaml = decode_yaml(
        String::from("\"9\""),
        <Deserializable>::from(custom),
        default_yaml_options(),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&from_yaml.failed),
        String::from("|"), terrane_scalar_support::scalar_text(&from_yaml.value.scalar)
    );
}
// Source: tests/integration/typed-serialization-destinations/case.trn
// Namespace: typed-serialization-destinations
fn test_destinations() {
    let options: JsonOptions;
    let scalar: TerraneDocumentDecodeOutcome<String>;
    let numbers: TerraneDocumentDecodeOutcome<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let keys: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Map<String, terrane_int_support::Int>,
    >;
    let pair: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    let wrong: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    let prototype: Option<String>;
    let nulled: TerraneDocumentDecodeOutcome<Option<String>>;
    let present: TerraneDocumentDecodeOutcome<Option<String>>;
    let invalid: TerraneDocumentDecodeOutcome<Option<String>>;
    let encoded: TerraneDocumentDecodeOutcome<String>;
    let encoded_yaml: TerraneDocumentDecodeOutcome<String>;
    let encoded_none: TerraneDocumentDecodeOutcome<String>;
    options = default_json_options();
    scalar = {
        let __terrane_document_source = &String::from("\"hello\"");
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            String,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <String>::default(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:8:14",
            "string",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(scalar.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&scalar.value
        .clone())
    );
    numbers = {
        let __terrane_document_source = &String::from("[3,5]");
        let __terrane_document_fallback = &terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(vec![terrane_int_support::Int::from(0_i128)]);
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::List<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:10:15",
            "list of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(numbers.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_0 = &numbers.value.clone();
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        26 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:33-11:49 */)),
        26 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:33-11:49 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &numbers.value.clone();
        __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        27 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:56-11:72 */)),
        27 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:11:56-11:72 */) })
    );
    keys = {
        let __terrane_document_source = &String::from("left: 9");
        let __terrane_document_fallback = &terrane_collection_support::Map::<
            String,
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, terrane_int_support::Int
                >::new(String::from(""), terrane_int_support::Int::from(0_i128))
            ],
        );
        let __terrane_document_options = &default_yaml_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Map<String, terrane_int_support::Int>,
        >(
            terrane_document_support::parse_yaml_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
                terrane_limit(&__terrane_document_options.max_alias_nodes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:12:12",
            "map of string, int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(keys.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_2 = &keys.value.clone();
        __terrane_raised(__terrane_receiver_value_2.get_or_error(&String::from("left")),
        28 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:13:30-13:48 */) })
    );
    pair = {
        let __terrane_document_source = &String::from("[4,6]");
        let __terrane_document_fallback = &terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:14:12",
            "tuple of int",
            &TerraneDocumentShape {
                tuple_length: Some(2),
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(pair.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_3 = &pair.value.clone();
        __terrane_raised(__terrane_receiver_value_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        29 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:30-15:43 */)),
        29 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:30-15:43 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_4 = &pair.value.clone();
        __terrane_raised(__terrane_receiver_value_4
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        30 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:50-15:63 */)),
        30 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:15:50-15:63 */) })
    );
    wrong = {
        let __terrane_document_source = &String::from("[4]");
        let __terrane_document_fallback = &terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(1_i128),
                terrane_int_support::Int::from(2_i128)
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:16:13",
            "tuple of int",
            &TerraneDocumentShape {
                tuple_length: Some(2),
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(wrong.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_5 = &wrong.value.clone();
        __terrane_raised(__terrane_receiver_value_5
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        31 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:31-17:45 */)),
        31 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:31-17:45 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_6 = &wrong.value.clone();
        __terrane_raised(__terrane_receiver_value_6
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        32 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:52-17:66 */)),
        32 /* terrane-site: tests/integration/typed-serialization-destinations/case.trn:17:52-17:66 */) })
    );
    prototype = Some(String::from("fallback"));
    nulled = {
        let __terrane_document_source = &String::from("null");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:19:14",
            "string|none",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nulled.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &nulled
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    present = {
        let __terrane_document_source = &String::from("\"present\"");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:21:15",
            "string|none",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(present.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &present
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    invalid = {
        let __terrane_document_source = &String::from("false");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-destinations/case.trn:23:15",
            "string|none",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &invalid
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    encoded = {
        let __terrane_document_source = &terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(3_i128),
                terrane_int_support::Int::from(5_i128)
            ],
        );
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-destinations/case.trn:25:15",
            "list of int",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded.value
        .clone())
    );
    encoded_yaml = {
        let __terrane_document_source = &terrane_collection_support::Map::<
            String,
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, terrane_int_support::Int
                >::new(String::from("left"), terrane_int_support::Int::from(9_i128))
            ],
        );
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-destinations/case.trn:27:20",
            "map of string, int",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded_yaml.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded_yaml
        .value.clone())
    );
    encoded_none = {
        let __terrane_receiver_value_7 = &nulled.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_7;
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-destinations/case.trn:29:20",
            "string|none",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded_none.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded_none
        .value.clone())
    );
}
// Source: tests/integration/typed-serialization-generic/case.trn
// Namespace: typed-serialization-generic
#[derive(Clone)]
pub struct Holder<TerraneType54> {
    __terrane_phantom: std::marker::PhantomData<fn(TerraneType54)>,
    pub count: terrane_int_support::Int,
}
impl<TerraneType54> Holder<TerraneType54> {
    pub fn terrane_construct() -> Self {
        Self {
            __terrane_phantom: std::marker::PhantomData,
            count: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl<TerraneType54> TerraneDocumentDecode for Holder<TerraneType54> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "holder",
                source,
                "tests/integration/typed-serialization-generic/case.trn:4:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["count"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "holder",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-serialization-generic/case.trn:4:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("count");
            let field_path = __terrane_document_child_path(path, "count");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-generic/case.trn:5:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.count = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl<TerraneType54> TerraneDocumentEncode for Holder<TerraneType54> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("count")?;
        TerraneDocumentEncode::terrane_encode_document(&self.count, writer)?;
        writer.end_object()
    }
}
impl<TerraneType54> DocumentEncodableProtocol for Holder<TerraneType54>
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl<TerraneType54> From<Holder<TerraneType54>> for DocumentEncodable
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn from(value: Holder<TerraneType54>) -> Self {
        Self(Box::new(value))
    }
}
impl<TerraneType54> DocumentDecodableProtocol for Holder<TerraneType54>
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl<TerraneType54> From<Holder<TerraneType54>> for DocumentDecodable
where
    Holder<TerraneType54>: Clone + Send + Sync + 'static,
{
    fn from(value: Holder<TerraneType54>) -> Self {
        Self(Box::new(value))
    }
}
fn test_generic_terrane_typed_serialization_generic() {
    let encoded: TerraneDocumentDecodeOutcome<String>;
    let decoded: TerraneDocumentDecodeOutcome<Holder<String>>;
    encoded = {
        let __terrane_document_source = &Holder::<String>::terrane_construct();
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-generic/case.trn:7:15",
            "holder of string",
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&encoded.value.clone()));
    decoded = {
        let __terrane_document_source = &String::from("{\"count\":9}");
        let __terrane_document_fallback = &Holder::<String>::terrane_construct();
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Holder<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-generic/case.trn:9:15",
            "holder of string",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&decoded.value.clone().count));
}
// Source: tests/integration/typed-serialization-matrix/case.trn
// Namespace: typed-serialization-matrix
#[derive(Clone)]
pub struct Item {
    pub display: String,
    pub amount: terrane_int_support::Int,
}
impl Item {
    pub fn terrane_construct() -> Self {
        Self {
            display: String::from(""),
            amount: terrane_int_support::Int::from(0_i128),
        }
    }
}
impl TerraneDocumentDecode for Item {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "item",
                source,
                "tests/integration/typed-serialization-matrix/case.trn:8:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["displayName", "amount"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "item",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-serialization-matrix/case.trn:8:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("displayName");
            let field_path = __terrane_document_child_path(path, "displayName");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:9:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.display = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("amount");
            let field_path = __terrane_document_child_path(path, "amount");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:10:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.amount = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode for Item {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("displayName")?;
        TerraneDocumentEncode::terrane_encode_document(&self.display, writer)?;
        writer.key("amount")?;
        TerraneDocumentEncode::terrane_encode_document(&self.amount, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol for Item {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Item> for DocumentEncodable {
    fn from(value: Item) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol for Item {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Item> for DocumentDecodable {
    fn from(value: Item) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    pub name: String,
    pub item: Item,
    pub numbers: terrane_collection_support::List<terrane_int_support::Int>,
    pub names: terrane_collection_support::Map<String, String>,
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub optional: Option<String>,
}
impl TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from("default"),
            item: Item::terrane_construct(),
            numbers: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            names: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::< String, String
                    >::new(String::from(""), String::from(""))
                ],
            ),
            pair: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            optional: Some(String::from("kept")),
        }
    }
    pub fn validate_document(&self) -> Option<String> {
        if self.name.as_str() == "bad" {
            return Some(String::from("reserved"));
        }
        return None;
    }
}
impl TerraneDocumentDecode
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "record",
                source,
                "tests/integration/typed-serialization-matrix/case.trn:12:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "name",
            "item",
            "numbers",
            "names",
            "pair",
            "optional",
        ];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "record",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "tests/integration/typed-serialization-matrix/case.trn:12:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("name");
            let field_path = __terrane_document_child_path(path, "name");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:13:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("item");
            let field_path = __terrane_document_child_path(path, "item");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Item as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:14:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: None,
                    },
                ) {
                    Ok(decoded) => value.item = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("numbers");
            let field_path = __terrane_document_child_path(path, "numbers");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:15:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.numbers = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("names");
            let field_path = __terrane_document_child_path(path, "names");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Map<
                    String,
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:16:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.names = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("pair");
            let field_path = __terrane_document_child_path(path, "pair");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:17:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.pair = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("optional");
            let field_path = __terrane_document_child_path(path, "optional");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <Option<
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "tests/integration/typed-serialization-matrix/case.trn:18:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.optional = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() {
            if let Some(message) = value.validate_document() {
                diagnostics
                    .push(
                        __terrane_document_diagnostic(
                            path,
                            "record",
                            "map",
                            "validation",
                            message,
                            source,
                            "tests/integration/typed-serialization-matrix/case.trn:12:1",
                        ),
                    );
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl TerraneDocumentEncode
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_object()?;
        writer.key("name")?;
        TerraneDocumentEncode::terrane_encode_document(&self.name, writer)?;
        writer.key("item")?;
        TerraneDocumentEncode::terrane_encode_document(&self.item, writer)?;
        writer.key("numbers")?;
        TerraneDocumentEncode::terrane_encode_document(&self.numbers, writer)?;
        writer.key("names")?;
        TerraneDocumentEncode::terrane_encode_document(&self.names, writer)?;
        writer.key("pair")?;
        TerraneDocumentEncode::terrane_encode_document(&self.pair, writer)?;
        writer.key("optional")?;
        TerraneDocumentEncode::terrane_encode_document(&self.optional, writer)?;
        writer.end_object()
    }
}
impl DocumentEncodableProtocol
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
> for DocumentEncodable {
    fn from(
        value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentDecodableProtocol
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<
    TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
> for DocumentDecodable {
    fn from(
        value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentValidatableProtocol
for TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264 {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn validate_document(&self) -> Option<String> {
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::validate_document(
            &*self,
        )
    }
}
impl From<
    TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
> for DocumentValidatable {
    fn from(
        value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    ) -> Self {
        Self(Box::new(value))
    }
}
fn test_matrix() {
    let mut value: TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264;
    let mut child: Item;
    let json: TerraneDocumentDecodeOutcome<String>;
    let yaml: TerraneDocumentDecodeOutcome<String>;
    let decoded: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let from_yaml: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let defaults: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let invalid: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let mut diagnostic: TerraneDocumentDiagnostic;
    let validated: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    let nulled: TerraneDocumentDecodeOutcome<
        TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
    >;
    value = TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct();
    {
        let __terrane_value = String::from("ok");
        value.name = __terrane_value;
    }
    child = Item::terrane_construct();
    {
        let __terrane_value = String::from("nested");
        child.display = __terrane_value;
    }
    {
        let __terrane_value = terrane_int_support::Int::from(7_i128);
        child.amount = __terrane_value;
    }
    {
        let __terrane_value = child;
        value.item = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(2_i128),
                terrane_int_support::Int::from(4_i128)
            ],
        );
        value.numbers = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::Map::<
            String,
            String,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, String
                >::new(String::from("zone"), String::from("west"))
            ],
        );
        value.names = __terrane_value;
    }
    {
        let __terrane_value = terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(8_i128),
                terrane_int_support::Int::from(9_i128)
            ],
        );
        value.pair = __terrane_value;
    }
    json = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_json_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-matrix/case.trn:34:12",
            "record",
        )
    };
    yaml = {
        let __terrane_document_source = &value;
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "tests/integration/typed-serialization-matrix/case.trn:35:12",
            "record",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(json.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&json.value.clone())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&yaml.value.clone())
    );
    decoded = {
        let __terrane_receiver_value_0 = &json.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_0;
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:38:15",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(decoded
        .diagnostics.length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&decoded.value.clone().name),
        String::from("|"), terrane_scalar_support::scalar_text(&decoded.value.clone()
        .item.display), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_1 = &decoded.value.clone();
        __terrane_raised(__terrane_receiver_value_1.numbers
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        33 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:91-39:115 */)),
        33 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:91-39:115 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_2 = &decoded.value.clone();
        __terrane_raised(__terrane_receiver_value_2.names
        .get_or_error(&String::from("zone")), 34 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:122-39:149 */) }),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_3 = &decoded.value.clone();
        __terrane_raised(__terrane_receiver_value_3.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        35 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:156-39:177 */)),
        35 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:39:156-39:177 */) }),
        String::from("|"), { let mut __terrane_text = String::new(); match &decoded.value
        .clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    from_yaml = {
        let __terrane_receiver_value_4 = &yaml.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_4;
        let __terrane_document_options = &default_yaml_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_yaml_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
                terrane_limit(&__terrane_document_options.max_alias_nodes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:40:17",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(from_yaml.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&from_yaml.value.clone().name),
        String::from("|"), terrane_scalar_support::scalar_text(&from_yaml.value.clone()
        .item.amount)
    );
    defaults = {
        let __terrane_document_source = &String::from(
            "{\"item\":{\"displayName\":\"x\",\"amount\":1},\"unknown\":2}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = true;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:42:16",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(defaults.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&defaults.value.clone().name),
        String::from("|"), { let mut __terrane_text = String::new(); match &defaults
        .value.clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text },
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_5 = &defaults.value.clone();
        __terrane_raised(__terrane_receiver_value_5.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        36 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:43:90-43:112 */)),
        36 /* terrane-site: tests/integration/typed-serialization-matrix/case.trn:43:90-43:112 */) })
    );
    invalid = {
        let __terrane_document_source = &String::from(
            "{\"name\":\"bad\",\"item\":{\"displayName\":false,\"amount\":\"x\"},\"numbers\":[1,\"x\"],\"pair\":[1]}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:44:15",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(invalid
        .diagnostics.clone().length()))
    );
    let __terrane_iterable_0 = invalid.diagnostics.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from("|"), terrane_scalar_support::scalar_text(&diagnostic.reason
            .to_owned())
        );
    }
    validated = {
        let __terrane_document_source = &String::from(
            "{\"name\":\"bad\",\"optional\":null}",
        );
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:48:17",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(validated.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(validated
        .diagnostics.clone().length()))
    );
    nulled = {
        let __terrane_document_source = &String::from("{\"optional\":null}");
        let __terrane_document_options = &default_json_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || TerraneNsTypedSerializationMatrixRecordIdentity2F74797065642D73657269616C697A6174696F6E2D6D6174726978007265636F7264::terrane_construct(),
            __terrane_document_allow_unknown,
            "tests/integration/typed-serialization-matrix/case.trn:50:14",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nulled.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &nulled
        .value.clone().optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
}
// Source: <compiler-generated test registry>
// Namespace: core/testing/runner
fn __terrane_test_hex(value: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
fn __terrane_test_record(outcome: &str, error: &TerraneError) {
    let Ok(path) = std::env::var("TERRANE_TEST_RESULT") else {
        return;
    };
    let details = error.structured_details();
    let frames = error.source_frames();
    let mut record = format!(
        "1\n{}\n{}\n{}\n{}\n{}\n", outcome, __terrane_test_hex(&error.descriptor_name()),
        __terrane_test_hex(error.message()), details.len(), frames.len(),
    );
    for detail in details {
        record.push_str(&__terrane_test_hex(detail));
        record.push('\n');
    }
    for frame in frames {
        record.push_str(&__terrane_test_hex(&frame));
        record.push('\n');
    }
    let _ = std::fs::write(path, record);
}
fn main() {
    let selected = std::env::args().nth(1).unwrap_or_default();
    match selected.as_str() {
        "0" => {
            if let Err(error) = test_values_terrane_checked_coercion() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "1" => {
            if let Err(error) = test_values_terrane_explicit_numeric_coercions() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "2" => {
            if let Err(error) = test_values_terrane_integer_coercions() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "3" => {
            test_values_terrane_mixed_width_arithmetic();
        }
        "4" => {
            test_values_terrane_mixed_width_comparisons();
        }
        "5" => {
            if let Err(error) = test_values_terrane_numeric_destination_conversion() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "6" => {
            test_values_terrane_numeric_literals();
        }
        "7" => {
            test_boundaries_terrane_string_display_boundaries();
        }
        "8" => {
            if let Err(error) = test_composition() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "9" => {
            test_generic_terrane_string_display_generic();
        }
        "10" => {
            test_boundaries_terrane_typed_serialization_boundaries();
        }
        "11" => {
            test_custom();
        }
        "12" => {
            test_destinations();
        }
        "13" => {
            test_generic_terrane_typed_serialization_generic();
        }
        "14" => {
            test_matrix();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
