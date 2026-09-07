// Compiler-generated glue is required here because Rust must materialize Terrane's statically known
// destination type. Mapping policy remains in class metadata and in the emitted per-class decoder.
#[allow(
    dead_code,
    reason = "each diagnostic member is materialized only when Terrane source observes it"
)]
#[derive(Clone)]
struct TerraneDocumentDiagnostic {
    path: String,
    expected: String,
    actual_kind: String,
    reason: &'static str,
    message: String,
    source: String,
    field_source: String,
}

#[derive(Clone)]
struct TerraneDocumentDecodeOutcome<T: Clone> {
    value: T,
    diagnostics: terrane_collection_support::List<TerraneDocumentDiagnostic>,
}

trait TerraneDocumentDecode: Sized {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>>;
}

fn __terrane_document_diagnostic(
    path: impl Into<String>,
    expected: impl Into<String>,
    actual_kind: impl Into<String>,
    reason: &'static str,
    message: impl Into<String>,
    source: &str,
    field_source: &str,
) -> TerraneDocumentDiagnostic {
    TerraneDocumentDiagnostic {
        path: path.into(),
        expected: expected.into(),
        actual_kind: actual_kind.into(),
        reason,
        message: message.into(),
        source: source.to_owned(),
        field_source: field_source.to_owned(),
    }
}

fn __terrane_document_child_path(path: &str, key: &str) -> String {
    if !key.is_empty()
        && key
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-')
    {
        format!("{path}.{key}")
    } else {
        format!("{path}[{key:?}]")
    }
}

fn __terrane_document_type_error<T>(
    input: &terrane_document_support::DataResult,
    path: &str,
    expected: &str,
    source: &str,
    field_source: &str,
) -> Result<T, Vec<TerraneDocumentDiagnostic>> {
    Err(vec![__terrane_document_diagnostic(
        path,
        expected,
        terrane_document_support::document_kind(input),
        "type-mismatch",
        format!("expected {expected}"),
        source,
        field_source,
    )])
}

impl TerraneDocumentDecode for String {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        _allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) == "string" {
            Ok(terrane_document_support::document_text(input))
        } else {
            __terrane_document_type_error(input, path, "string", source, field_source)
        }
    }
}

impl TerraneDocumentDecode for bool {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        _allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) == "bool" {
            Ok(terrane_document_support::document_text(input) == "true")
        } else {
            __terrane_document_type_error(input, path, "bool", source, field_source)
        }
    }
}

fn __terrane_document_float_is_exact(
    input: &terrane_document_support::DataResult,
    mantissa: u128,
    binary_exponent: i32,
    negative: bool,
) -> bool {
    let kind = terrane_document_support::document_kind(input);
    let (coefficient, decimal_exponent) = if kind == "integer" {
        (terrane_document_support::document_text(input), 0_i64)
    } else if kind == "decimal" {
        (
            terrane_document_support::document_coefficient(input),
            terrane_document_support::document_exponent(input),
        )
    } else {
        return false;
    };
    let Ok(coefficient) = coefficient.parse::<i128>() else {
        return false;
    };
    if (coefficient < 0) != negative {
        return coefficient == 0 && mantissa == 0;
    }
    let mut decimal_numerator = coefficient.unsigned_abs();
    let mut decimal_denominator = 1_u128;
    if decimal_exponent >= 0 {
        let Ok(exponent) = u32::try_from(decimal_exponent) else {
            return false;
        };
        let Some(power) = 10_u128.checked_pow(exponent) else {
            return false;
        };
        let Some(value) = decimal_numerator.checked_mul(power) else {
            return false;
        };
        decimal_numerator = value;
    } else {
        let Ok(exponent) = u32::try_from(decimal_exponent.unsigned_abs()) else {
            return false;
        };
        let Some(power) = 10_u128.checked_pow(exponent) else {
            return false;
        };
        decimal_denominator = power;
    }
    let (binary_numerator, binary_denominator) = if binary_exponent >= 0 {
        let Ok(exponent) = u32::try_from(binary_exponent) else {
            return false;
        };
        let Some(value) = mantissa.checked_shl(exponent) else {
            return false;
        };
        (value, 1_u128)
    } else {
        let Some(denominator) = 1_u128.checked_shl(binary_exponent.unsigned_abs()) else {
            return false;
        };
        (mantissa, denominator)
    };
    decimal_numerator.checked_mul(binary_denominator)
        == binary_numerator.checked_mul(decimal_denominator)
}

macro_rules! __terrane_document_float {
    ($type:ty, $bits:ty, $mantissa_bits:expr, $bias:expr) => {
        impl TerraneDocumentDecode for $type {
            fn terrane_decode_document(
                input: &terrane_document_support::DataResult,
                path: &str,
                _allow_unknown: bool,
                source: &str,
                field_source: &str,
            ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
                let kind = terrane_document_support::document_kind(input);
                if kind != "integer" && kind != "decimal" {
                    return __terrane_document_type_error(
                        input,
                        path,
                        stringify!($type),
                        source,
                        field_source,
                    );
                }
                let text = terrane_document_support::document_text(input);
                let parsed = text.parse::<$type>().ok();
                let exact = parsed.filter(|value| {
                    if !value.is_finite() {
                        return false;
                    }
                    let bits: $bits = value.to_bits();
                    let sign = (bits >> (<$bits>::BITS - 1)) != 0;
                    let exponent_mask: $bits =
                        ((1 as $bits) << (<$bits>::BITS - 1 - $mantissa_bits)) - 1;
                    let exponent =
                        ((bits >> $mantissa_bits) & exponent_mask) as i32;
                    let fraction_mask: $bits = ((1 as $bits) << $mantissa_bits) - 1;
                    let fraction = bits & fraction_mask;
                    let (mantissa, binary_exponent) = if exponent == 0 {
                        (
                            fraction as u128,
                            1 - $bias - $mantissa_bits as i32,
                        )
                    } else {
                        (
                            (((1 as $bits) << $mantissa_bits) | fraction) as u128,
                            exponent - $bias - $mantissa_bits as i32,
                        )
                    };
                    __terrane_document_float_is_exact(
                        input,
                        mantissa,
                        binary_exponent,
                        sign,
                    )
                });
                exact.ok_or_else(|| {
                    vec![__terrane_document_diagnostic(
                        path,
                        stringify!($type),
                        kind,
                        "numeric-conversion",
                        "document number is not exactly representable by the destination float",
                        source,
                        field_source,
                    )]
                })
            }
        }
    };
}

__terrane_document_float!(f32, u32, 23, 127);
__terrane_document_float!(f64, u64, 52, 1023);

impl TerraneDocumentDecode for terrane_int_support::Int {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        _allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) != "integer" {
            return __terrane_document_type_error(input, path, "integer", source, field_source);
        }
        let text = terrane_document_support::document_text(input);
        terrane_int_support::parse_radix(&text, &terrane_int_support::Int::from(10_i128)).map_err(
            |_| {
                vec![__terrane_document_diagnostic(
                    path,
                    "int",
                    "integer",
                    "numeric-conversion",
                    "integer is outside the supported exact range",
                    source,
                    field_source,
                )]
            },
        )
    }
}

macro_rules! __terrane_fixed_document_integer {
    ($($type:ty),+ $(,)?) => {
        $(
            impl TerraneDocumentDecode for $type {
                fn terrane_decode_document(
                    input: &terrane_document_support::DataResult,
                    path: &str,
                    _allow_unknown: bool,
                    source: &str,
                    field_source: &str,
                ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
                    if terrane_document_support::document_kind(input) != "integer" {
                        return __terrane_document_type_error(
                            input,
                            path,
                            stringify!($type),
                            source,
                            field_source,
                        );
                    }
                    terrane_document_support::document_text(input).parse::<$type>().map_err(|_| {
                        vec![__terrane_document_diagnostic(
                            path,
                            stringify!($type),
                            "integer",
                            "numeric-conversion",
                            "integer is outside the destination range",
                            source,
                            field_source,
                        )]
                    })
                }
            }
        )+
    };
}

__terrane_fixed_document_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

impl<T: TerraneDocumentDecode> TerraneDocumentDecode for Option<T> {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) == "none" {
            Ok(None)
        } else {
            T::terrane_decode_document(input, path, allow_unknown, source, field_source).map(Some)
        }
    }
}

impl<T: TerraneDocumentDecode> TerraneDocumentDecode for terrane_collection_support::List<T> {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) != "list" {
            return __terrane_document_type_error(input, path, "list", source, field_source);
        }
        let mut values = Vec::new();
        let mut diagnostics = Vec::new();
        for index in 0..terrane_document_support::document_length(input) {
            let item = terrane_document_support::document_item(input, index);
            let item_path = format!("{path}[{index}]");
            match T::terrane_decode_document(
                &item,
                &item_path,
                allow_unknown,
                source,
                field_source,
            ) {
                Ok(value) => values.push(value),
                Err(mut item_diagnostics) => diagnostics.append(&mut item_diagnostics),
            }
        }
        if diagnostics.is_empty() {
            Ok(terrane_collection_support::List::new(values))
        } else {
            Err(diagnostics)
        }
    }
}

impl<T: Clone + TerraneDocumentDecode> TerraneDocumentDecode
    for terrane_collection_support::Tuple<T>
{
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let list = terrane_collection_support::List::<T>::terrane_decode_document(
            input,
            path,
            allow_unknown,
            source,
            field_source,
        )?;
        Ok(terrane_collection_support::Tuple::new(list.into_vec()))
    }
}

impl<T: Clone + TerraneDocumentDecode> TerraneDocumentDecode
    for terrane_collection_support::Map<String, T>
{
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(input, path, "map", source, field_source);
        }
        let mut entries = Vec::new();
        let mut diagnostics = Vec::new();
        for index in 0..terrane_document_support::document_length(input) {
            let key = terrane_document_support::document_key(input, index);
            let item = terrane_document_support::document_field(input, &key);
            let item_path = format!("{path}.{}", key);
            match T::terrane_decode_document(
                &item,
                &item_path,
                allow_unknown,
                source,
                field_source,
            ) {
                Ok(value) => entries.push(terrane_collection_support::Entry::new(key, value)),
                Err(mut item_diagnostics) => diagnostics.append(&mut item_diagnostics),
            }
        }
        if diagnostics.is_empty() {
            Ok(terrane_collection_support::Map::new(entries))
        } else {
            Err(diagnostics)
        }
    }
}
