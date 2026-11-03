// Breadcrumbs borrow their parent and document key. Successful decoding never renders a path.
enum TerraneDocumentPath<'a> {
    Root(&'a str),
    Field(&'a TerraneDocumentPath<'a>, &'a str),
    MapKey(&'a TerraneDocumentPath<'a>, &'a str),
    Index(&'a TerraneDocumentPath<'a>, usize),
}

impl std::fmt::Display for TerraneDocumentPath<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(path) => formatter.write_str(path),
            Self::Field(parent, key) => {
                if !key.is_empty()
                    && key.chars().all(|character| {
                        character.is_ascii_alphanumeric() || character == '_' || character == '-'
                    })
                {
                    write!(formatter, "{parent}.{key}")
                } else {
                    write!(formatter, "{parent}[{key:?}]")
                }
            }
            // Map entry paths retain the existing spelling, independently of class field paths.
            Self::MapKey(parent, key) => write!(formatter, "{parent}.{key}"),
            Self::Index(parent, index) => write!(formatter, "{parent}[{index}]"),
        }
    }
}

trait TerraneDocumentDecode: Sized {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>>;
}

fn __terrane_decode_document<T: Clone + TerraneDocumentDecode>(
    parsed: Result<terrane_document_support::Document, terrane_document_support::DataResult>,
    fallback: impl FnOnce() -> T,
    allow_unknown: bool,
    source: &str,
    expected: &str,
    tuple_length: Option<usize>,
) -> TerraneDocumentDecodeOutcome<T> {
    let path = TerraneDocumentPath::Root("$");
    let decoded = match parsed {
        Ok(mut input) => {
            if let Some(length) = tuple_length
                && matches!(&input, terrane_document_support::Document::List(items) if items.len() != length)
            {
                Err(vec![__terrane_document_diagnostic(&path, expected, "list", "tuple-length",
                    format!("expected exactly {length} tuple items"), source, source)])
            } else {
                T::terrane_decode_document(&mut input, &path, allow_unknown, source, source)
            }
        }
        Err(error) => Err(vec![__terrane_document_diagnostic(&path, expected, "invalid", "parse",
            error.message, source, source)]),
    };
    match decoded {
        Ok(value) => TerraneDocumentDecodeOutcome {
            value, diagnostics: terrane_collection_support::List::new(Vec::new()),
        },
        Err(diagnostics) => TerraneDocumentDecodeOutcome {
            value: fallback(), diagnostics: terrane_collection_support::List::new(diagnostics),
        },
    }
}
fn __terrane_document_kind(input: &terrane_document_support::Document) -> &'static str {
    use terrane_document_support::Document;
    match input {
        Document::None => "none",
        Document::Bool(_) => "bool",
        Document::Integer(_) => "integer",
        Document::Decimal(_) => "decimal",
        Document::String(_) => "string",
        Document::List(_) => "list",
        Document::Map(_) => "map",
    }
}

fn __terrane_document_child_path<'a>(
    path: &'a TerraneDocumentPath<'a>,
    key: &'a str,
) -> TerraneDocumentPath<'a> {
    __terrane_document_entry_path(path, key, true)
}

fn __terrane_document_entry_path<'a>(
    path: &'a TerraneDocumentPath<'a>, key: &'a str, class_field: bool,
) -> TerraneDocumentPath<'a> {
    if class_field {
        TerraneDocumentPath::Field(path, key)
    } else {
        TerraneDocumentPath::MapKey(path, key)
    }
}

fn __terrane_document_type_error<T>(
    input: &terrane_document_support::Document,
    path: &TerraneDocumentPath<'_>,
    expected: &str,
    source: &str,
    field_source: &str,
) -> Result<T, Vec<TerraneDocumentDiagnostic>> {
    Err(vec![__terrane_document_diagnostic(
        path,
        expected,
        __terrane_document_kind(input),
        "type-mismatch",
        format!("expected {expected}"),
        source,
        field_source,
    )])
}

impl TerraneDocumentDecode for String {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if let terrane_document_support::Document::String(value) = input {
            Ok(std::mem::take(value))
        } else {
            __terrane_document_type_error(input, path, "string", source, field_source)
        }
    }
}

impl TerraneDocumentDecode for () {
    fn terrane_decode_document(input: &mut terrane_document_support::Document, path: &TerraneDocumentPath<'_>, _: bool, source: &str, field_source: &str) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if matches!(input, terrane_document_support::Document::None) { Ok(()) }
        else { __terrane_document_type_error(input, path, "none", source, field_source) }
    }
}


impl TerraneDocumentDecode for bool {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if let terrane_document_support::Document::Bool(value) = input {
            Ok(*value)
        } else {
            __terrane_document_type_error(input, path, "bool", source, field_source)
        }
    }
}

macro_rules! __terrane_document_float {
    ($type:ty) => {
        impl TerraneDocumentDecode for $type {
            fn terrane_decode_document(
                input: &mut terrane_document_support::Document,
                path: &TerraneDocumentPath<'_>,
                _: bool,
                source: &str,
                field_source: &str,
            ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
                use terrane_document_support::Document;
                let text = match input {
                    Document::Integer(text) | Document::Decimal(text) => text,
                    _ => {
                        return __terrane_document_type_error(
                            input,
                            path,
                            stringify!($type),
                            source,
                            field_source,
                        );
                    }
                };
                text.parse::<$type>()
                    .ok()
                    .filter(|value| value.is_finite())
                    .ok_or_else(|| {
                        vec![__terrane_document_diagnostic(
                            path,
                            stringify!($type),
                            __terrane_document_kind(input),
                            "numeric-conversion",
                            "document number is outside the finite range of the destination float",
                            source,
                            field_source,
                        )]
                    })
            }
        }
    };
}

__terrane_document_float!(f32);
__terrane_document_float!(f64);

// Canonical document integers may use scientific notation beyond 21 decimal digits.
// Convert the exact coefficient and decimal scale, never through a binary float.
fn __terrane_document_integer_parts(text: &str) -> Option<(std::borrow::Cow<'_, str>, u32)> {
    let (coefficient, exponent) = text.split_once('e')?;
    let fraction_length = coefficient
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    let zeros = exponent
        .parse::<i64>()
        .ok()?
        .checked_sub(i64::try_from(fraction_length).ok()?)?;
    let coefficient = if fraction_length == 0 {
        std::borrow::Cow::Borrowed(coefficient)
    } else {
        std::borrow::Cow::Owned(coefficient.replace('.', ""))
    };
    let zeros = u32::try_from(zeros).ok()?;
    if coefficient.trim_start_matches('-').len().checked_add(usize::try_from(zeros).ok()?)?
        > terrane_document_support::MAXIMUM_NUMBER_DIGITS
    {
        return None;
    }
    Some((coefficient, zeros))
}

impl TerraneDocumentDecode for terrane_int_support::Int {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Integer(text) = input else {
            return __terrane_document_type_error(input, path, "integer", source, field_source);
        };
        let decoded = text
            .parse::<i128>()
            .ok()
            .map(terrane_int_support::Int::from)
            .or_else(|| {
                let radix = terrane_int_support::Int::from(10_i128);
                if let Some((coefficient, mut zeros)) = __terrane_document_integer_parts(text) {
                    let mut value = terrane_int_support::parse_radix(&coefficient, &radix).ok()?;
                    let mut factor = radix;
                    while zeros != 0 {
                        if zeros & 1 != 0 {
                            value = value * factor.clone();
                        }
                        zeros >>= 1;
                        if zeros != 0 {
                            factor = factor.clone() * factor;
                        }
                    }
                    Some(value)
                } else {
                    terrane_int_support::parse_radix(text, &radix).ok()
                }
            });
        decoded.ok_or_else(|| {
            vec![__terrane_document_diagnostic(
                path,
                "int",
                "integer",
                "numeric-conversion",
                "integer is outside the supported exact range",
                source,
                field_source,
            )]
        })
    }
}

macro_rules! __terrane_fixed_document_integer {
    ($($type:ty),+ $(,)?) => {
        $(impl TerraneDocumentDecode for $type {
            fn terrane_decode_document(
                input: &mut terrane_document_support::Document,
                path: &TerraneDocumentPath<'_>,
                _: bool,
                source: &str,
                field_source: &str,
            ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
                let terrane_document_support::Document::Integer(text) = input else {
                    return __terrane_document_type_error(input, path, stringify!($type), source, field_source);
                };
                let decoded = text.parse::<$type>().ok().or_else(|| {
                    let (coefficient, zeros) = __terrane_document_integer_parts(text)?;
                    let value = coefficient.parse::<$type>().ok()?;
                    let ten: $type = 10;
                    value.checked_mul(ten.checked_pow(zeros)?)
                });
                decoded.ok_or_else(|| vec![__terrane_document_diagnostic(
                    path, stringify!($type), "integer", "numeric-conversion", "integer is outside the destination range", source, field_source,
                )])
            }
        })+
    };
}

__terrane_fixed_document_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

impl<T: TerraneDocumentDecode> TerraneDocumentDecode for Option<T> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if matches!(input, terrane_document_support::Document::None) {
            Ok(None)
        } else {
            T::terrane_decode_document(input, path, allow_unknown, source, field_source).map(Some)
        }
    }
}

impl<T: TerraneDocumentDecode> TerraneDocumentDecode for terrane_collection_support::List<T> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::List(items) = input else {
            return __terrane_document_type_error(input, path, "list", source, field_source);
        };
        let mut values = Vec::with_capacity(items.len());
        let mut diagnostics = Vec::new();
        for (index, item) in items.iter_mut().enumerate() {
            let item_path = TerraneDocumentPath::Index(path, index);
            match T::terrane_decode_document(item, &item_path, allow_unknown, source, field_source)
            {
                Ok(value) => values.push(value),
                Err(mut errors) => diagnostics.append(&mut errors),
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
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let values = terrane_collection_support::List::<T>::terrane_decode_document(
            input,
            path,
            allow_unknown,
            source,
            field_source,
        )?;
        Ok(terrane_collection_support::Tuple::new(values.into_vec()))
    }
}

macro_rules! __terrane_document_decode_map {
    ($map:ident) => {
impl<T: Clone + TerraneDocumentDecode> TerraneDocumentDecode
    for terrane_collection_support::$map<String, T>
{
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(items) = input else {
            return __terrane_document_type_error(input, path, "map", source, field_source);
        };
        let mut entries = Vec::with_capacity(items.len());
        let mut diagnostics = Vec::new();
        for (key, mut item) in std::mem::take(items) {
            let item_path = __terrane_document_entry_path(path, &key, false);
            match T::terrane_decode_document(
                &mut item,
                &item_path,
                allow_unknown,
                source,
                field_source,
            ) {
                Ok(value) => entries.push(terrane_collection_support::Entry::new(key, value)),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        if diagnostics.is_empty() {
            Ok(terrane_collection_support::$map::new(entries))
        } else {
            Err(diagnostics)
        }
    }
}
    };
}
__terrane_document_decode_map!(Map);
__terrane_document_decode_map!(UnorderedMap);
