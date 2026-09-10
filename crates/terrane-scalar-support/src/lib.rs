use std::fmt::Write as _;

use terrane_int_support::Int;

/// Produces canonical Terrane text for a core scalar value.
pub trait ScalarDisplay {
    fn write_scalar(&self, output: &mut String);

    #[must_use]
    fn scalar_text(&self) -> String {
        let mut output = String::new();
        self.write_scalar(&mut output);
        output
    }
}

/// Produces canonical Terrane text for a core scalar value.
#[must_use]
pub fn scalar_text(value: &impl ScalarDisplay) -> String {
    value.scalar_text()
}

impl<T: ScalarDisplay + ?Sized> ScalarDisplay for &T {
    fn write_scalar(&self, output: &mut String) {
        (*self).write_scalar(output);
    }
}

impl ScalarDisplay for bool {
    fn write_scalar(&self, output: &mut String) {
        output.push_str(if *self { "true" } else { "false" });
    }
}

impl ScalarDisplay for str {
    fn write_scalar(&self, output: &mut String) {
        output.push_str(self);
    }
}

impl ScalarDisplay for String {
    fn write_scalar(&self, output: &mut String) {
        output.push_str(self);
    }
}

impl ScalarDisplay for () {
    fn write_scalar(&self, output: &mut String) {
        output.push_str("none");
    }
}

macro_rules! integer_display {
    ($($type:ty),+ $(,)?) => {$(
        impl ScalarDisplay for $type {
            fn write_scalar(&self, output: &mut String) {
                write!(output, "{self}").expect("writing to a String cannot fail");
            }
        }
    )+};
}

integer_display!(Int, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

macro_rules! float_display {
    ($($type:ty),+ $(,)?) => {$(
        impl ScalarDisplay for $type {
            fn write_scalar(&self, output: &mut String) {
                if self.is_nan() {
                    output.push_str("nan");
                } else {
                    write!(output, "{self}").expect("writing to a String cannot fail");
                }
            }
        }
    )+};
}

float_display!(f32, f64);

/// Exact mantissa/exponent decomposition of one binary floating-point value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatDecomposition<T> {
    pub mantissa: T,
    pub exponent: i32,
}

/// Decomposes `value` as `mantissa * 2^exponent`.
///
/// Finite non-zero results have a mantissa whose magnitude is in `[0.5, 1)`.
/// Zero retains its sign. Infinities and NaNs are returned unchanged. Those
/// three edge categories use exponent zero.
#[must_use]
pub fn decompose_f32(value: f32) -> FloatDecomposition<f32> {
    if value == 0.0 || !value.is_finite() {
        return FloatDecomposition {
            mantissa: value,
            exponent: 0,
        };
    }
    if value.is_subnormal() {
        let decomposed = decompose_f32(value * 33_554_432.0);
        return FloatDecomposition {
            mantissa: decomposed.mantissa,
            exponent: decomposed.exponent - 25,
        };
    }
    let bits = value.to_bits();
    #[expect(
        clippy::cast_possible_wrap,
        reason = "the IEEE exponent mask is bounded to eight bits"
    )]
    let encoded_exponent = ((bits >> 23) & 0xff) as i32;
    FloatDecomposition {
        mantissa: f32::from_bits((bits & !(0xff << 23)) | (126_u32 << 23)),
        exponent: encoded_exponent - 126,
    }
}

/// Decomposes `value` as `mantissa * 2^exponent`.
///
/// Finite non-zero results have a mantissa whose magnitude is in `[0.5, 1)`.
/// Zero retains its sign. Infinities and NaNs are returned unchanged. Those
/// three edge categories use exponent zero.
#[must_use]
pub fn decompose_f64(value: f64) -> FloatDecomposition<f64> {
    if value == 0.0 || !value.is_finite() {
        return FloatDecomposition {
            mantissa: value,
            exponent: 0,
        };
    }
    if value.is_subnormal() {
        let decomposed = decompose_f64(value * 18_014_398_509_481_984.0);
        return FloatDecomposition {
            mantissa: decomposed.mantissa,
            exponent: decomposed.exponent - 54,
        };
    }
    let bits = value.to_bits();
    let encoded_exponent = ((bits >> 52) & 0x7ff) as i32;
    FloatDecomposition {
        mantissa: f64::from_bits((bits & !(0x7ff_u64 << 52)) | (1022_u64 << 52)),
        exponent: encoded_exponent - 1022,
    }
}

/// Scales a binary32 value by an exact integral power of two with one final
/// IEEE rounding at the destination precision.
#[must_use]
pub fn scale_binary_f32(mut value: f32, mut exponent: i32) -> f32 {
    if exponent > 127 {
        value *= f32::from_bits((254_u32) << 23);
        exponent -= 127;
        if exponent > 127 {
            value *= f32::from_bits((254_u32) << 23);
            exponent -= 127;
            if exponent > 127 {
                exponent = 127;
            }
        }
    } else if exponent < -126 {
        value *= f32::from_bits(1_u32 << 23) * f32::from_bits((127_u32 + 24) << 23);
        exponent += 126 - 24;
        if exponent < -126 {
            value *= f32::from_bits(1_u32 << 23) * f32::from_bits((127_u32 + 24) << 23);
            exponent += 126 - 24;
            if exponent < -126 {
                exponent = -126;
            }
        }
    }
    #[expect(
        clippy::cast_sign_loss,
        reason = "the reduction above clamps the encoded exponent to a non-negative range"
    )]
    let encoded_exponent = (127 + exponent) as u32;
    value * f32::from_bits(encoded_exponent << 23)
}

/// Scales a binary64 value by an exact integral power of two with one final
/// IEEE rounding at the destination precision.
#[must_use]
pub fn scale_binary_f64(mut value: f64, mut exponent: i32) -> f64 {
    if exponent > 1023 {
        value *= f64::from_bits((2046_u64) << 52);
        exponent -= 1023;
        if exponent > 1023 {
            value *= f64::from_bits((2046_u64) << 52);
            exponent -= 1023;
            if exponent > 1023 {
                exponent = 1023;
            }
        }
    } else if exponent < -1022 {
        value *= f64::from_bits(1_u64 << 52) * f64::from_bits((1023_u64 + 53) << 52);
        exponent += 1022 - 53;
        if exponent < -1022 {
            value *= f64::from_bits(1_u64 << 52) * f64::from_bits((1023_u64 + 53) << 52);
            exponent += 1022 - 53;
            if exponent < -1022 {
                exponent = -1022;
            }
        }
    }
    #[expect(
        clippy::cast_sign_loss,
        reason = "the reduction above clamps the encoded exponent to a non-negative range"
    )]
    let encoded_exponent = (1023 + exponent) as u64;
    value * f64::from_bits(encoded_exponent << 52)
}
