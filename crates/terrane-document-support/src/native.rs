//! Explicit JSON interoperability. Exact JSON output is format-specific; native
//! floating destinations deliberately round once at their requested precision.
use crate::{Document, MAXIMUM_JSON_DEPTH, MAXIMUM_NUMBER_DIGITS, write_json_bounded};
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::{borrow::Cow, fmt};

#[path = "native_serialize.rs"]
mod serialize;

impl serde::ser::Error for NativeJsonError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

/// Native Serde data into a bounded exact tree, without an intermediate JSON
/// string or Value. Nonfinite floats and non-string object keys are errors.
///
/// # Errors
/// Rejects unsupported map keys, duplicate entries, nonfinite floats, custom
/// serialization failures, and exceeded depth/output budgets.
pub fn to_document<T: Serialize + ?Sized>(value: &T) -> Result<Document, NativeJsonError> {
    serialize::to_document(value)
}

pub fn serialize_document<T: Serialize + ?Sized>(value: &T) -> crate::DataResult {
    match to_document(value) {
        Ok(value) => crate::DataResult::success(value),
        Err(error) => {
            crate::DataResult::failure(error.to_string(), "$", "serializable native document")
        }
    }
}

/// Explicit native JSON-value adaptation, including feature-private exact
/// numbers. Unlike generic Serde, this adapter knows the source format.
#[must_use]
pub fn serialize_json_value(value: &serde_json::Value) -> crate::DataResult {
    match json_value_to_document(value) {
        Ok(value) => crate::DataResult::success(value),
        Err(error) => crate::DataResult::failure(error.to_string(), "$", "native JSON document"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeJsonError(pub String);
impl fmt::Display for NativeJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for NativeJsonError {}
impl de::Error for NativeJsonError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

/// Borrowed exact-number JSON serialization, without Serde's private number map.
/// Other formats should use their own explicit adapter, not this JSON contract.
pub struct JsonDocument<'a>(pub &'a Document);
impl Serialize for JsonDocument<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let text = write_json_bounded(self.0, MAXIMUM_JSON_DEPTH, 16_777_216)
            .map_err(serde::ser::Error::custom)?;
        let raw =
            serde_json::value::RawValue::from_string(text).map_err(serde::ser::Error::custom)?;
        raw.serialize(serializer)
    }
}

/// Deserialize directly from the borrowed document tree. No JSON string or
/// intermediate native Value is built. Borrowed strings retain the document loan.
///
/// # Errors
/// Rejects shape/range mismatches, excessive depth and destination-specific failures.
pub fn from_document<'de, T: Deserialize<'de>>(value: &'de Document) -> Result<T, NativeJsonError> {
    T::deserialize(DocumentDeserializer { value, depth: 0 })
}

#[derive(Clone, Copy)]
struct DocumentDeserializer<'de> {
    value: &'de Document,
    depth: usize,
}
impl<'de> DocumentDeserializer<'de> {
    fn child(self, value: &'de Document) -> Result<Self, NativeJsonError> {
        if self.depth >= MAXIMUM_JSON_DEPTH {
            return Err(NativeJsonError("native document depth exceeded".into()));
        }
        Ok(Self {
            value,
            depth: self.depth + 1,
        })
    }
    fn number(self) -> Result<&'de str, NativeJsonError> {
        match self.value {
            Document::Integer(value) | Document::Decimal(value) => Ok(value),
            _ => Err(NativeJsonError("expected numeric document value".into())),
        }
    }
    fn integral(self) -> Result<Cow<'de, str>, NativeJsonError> {
        let text = self.number()?;
        let normalized = crate::normalize_number(text).map_err(NativeJsonError)?;
        if !matches!(
            crate::classify_number(text).map_err(NativeJsonError)?,
            Document::Integer(_)
        ) {
            return Err(NativeJsonError("expected integral document value".into()));
        }
        let Some((mantissa, exponent)) = normalized.split_once('e') else {
            return Ok(Cow::Owned(normalized));
        };
        let exponent: i32 = exponent
            .parse()
            .map_err(|_| NativeJsonError("invalid numeric exponent".into()))?;
        let fractional = mantissa
            .split_once('.')
            .map_or(0, |(_, fraction)| fraction.len());
        let zeros = i64::from(exponent) - i64::try_from(fractional).unwrap_or(i64::MAX);
        let zeros =
            usize::try_from(zeros).map_err(|_| NativeJsonError("nonintegral number".into()))?;
        let digits = mantissa
            .len()
            .saturating_sub(usize::from(mantissa.contains('.')));
        if digits.saturating_add(zeros) > MAXIMUM_NUMBER_DIGITS {
            return Err(NativeJsonError(
                "native integer expansion exceeds digit limit".into(),
            ));
        }
        let mut result = String::with_capacity(digits + zeros);
        result.extend(mantissa.chars().filter(|character| *character != '.'));
        result.extend(std::iter::repeat_n('0', zeros));
        Ok(Cow::Owned(result))
    }
}

macro_rules! native_integer {
    ($method:ident, $visit:ident, $ty:ty) => {
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
            let number = self.integral()?.parse::<$ty>().map_err(|_| {
                NativeJsonError(concat!("number is outside ", stringify!($ty)).into())
            })?;
            visitor.$visit(number)
        }
    };
}
impl<'de> Deserializer<'de> for DocumentDeserializer<'de> {
    type Error = NativeJsonError;
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            Document::None => visitor.visit_unit(),
            Document::Bool(value) => visitor.visit_bool(*value),
            Document::String(value) => visitor.visit_borrowed_str(value),
            Document::Integer(_) => {
                let text = self.integral()?;
                if let Ok(value) = text.parse::<i64>() {
                    visitor.visit_i64(value)
                } else if let Ok(value) = text.parse::<u64>() {
                    visitor.visit_u64(value)
                } else if let Ok(value) = text.parse::<i128>() {
                    visitor.visit_i128(value)
                } else if let Ok(value) = text.parse::<u128>() {
                    visitor.visit_u128(value)
                } else {
                    Err(NativeJsonError(
                        "integer exceeds native Serde integer range".into(),
                    ))
                }
            }
            Document::Decimal(_) => self.deserialize_f64(visitor),
            Document::List(values) => checked_sequence(self, values, visitor),
            Document::Map(values) => checked_map(self, values, visitor),
        }
    }
    native_integer!(deserialize_i8, visit_i8, i8);
    native_integer!(deserialize_i16, visit_i16, i16);
    native_integer!(deserialize_i32, visit_i32, i32);
    native_integer!(deserialize_i64, visit_i64, i64);
    native_integer!(deserialize_i128, visit_i128, i128);
    native_integer!(deserialize_u8, visit_u8, u8);
    native_integer!(deserialize_u16, visit_u16, u16);
    native_integer!(deserialize_u32, visit_u32, u32);
    native_integer!(deserialize_u64, visit_u64, u64);
    native_integer!(deserialize_u128, visit_u128, u128);
    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let value = self
            .number()?
            .parse::<f32>()
            .map_err(|_| NativeJsonError("invalid f32".into()))?;
        if !value.is_finite() {
            return Err(NativeJsonError("number exceeds finite f32 range".into()));
        }
        visitor.visit_f32(value)
    }
    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let value = self
            .number()?
            .parse::<f64>()
            .map_err(|_| NativeJsonError("invalid f64".into()))?;
        if !value.is_finite() {
            return Err(NativeJsonError("number exceeds finite f64 range".into()));
        }
        visitor.visit_f64(value)
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if matches!(self.value, Document::None) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match self.value {
            Document::String(value) => visitor.visit_enum(value.as_str().into_deserializer()),
            Document::Map(values) if values.len() == 1 => {
                let (key, value) = values.iter().next().expect("one enum entry");
                visitor.visit_enum(Variant {
                    key,
                    value: self.child(value)?,
                })
            }
            _ => Err(NativeJsonError(
                "expected externally tagged native enum".into(),
            )),
        }
    }
    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let Document::String(value) = self.value {
            visitor.visit_borrowed_bytes(value.as_bytes())
        } else {
            self.deserialize_any(visitor)
        }
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }
    serde::forward_to_deserialize_any! { bool char str string byte_buf unit unit_struct seq tuple tuple_struct map struct identifier }
}
struct Elements<'de> {
    owner: DocumentDeserializer<'de>,
    values: std::slice::Iter<'de, Document>,
}
impl<'de> SeqAccess<'de> for Elements<'de> {
    type Error = NativeJsonError;
    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        self.values
            .next()
            .map(|value| seed.deserialize(self.owner.child(value)?))
            .transpose()
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.values.len())
    }
}

fn checked_sequence<'de, V: Visitor<'de>>(
    owner: DocumentDeserializer<'de>,
    values: &'de [Document],
    visitor: V,
) -> Result<V::Value, NativeJsonError> {
    let mut access = Elements {
        owner,
        values: values.iter(),
    };
    let result = visitor.visit_seq(&mut access)?;
    if access.values.next().is_some() {
        return Err(NativeJsonError(
            "native sequence has unconsumed elements".into(),
        ));
    }
    Ok(result)
}

fn checked_map<'de, V: Visitor<'de>>(
    owner: DocumentDeserializer<'de>,
    values: &'de std::collections::BTreeMap<String, Document>,
    visitor: V,
) -> Result<V::Value, NativeJsonError> {
    let mut access = Fields {
        owner,
        values: values.iter(),
        pending: None,
    };
    let result = visitor.visit_map(&mut access)?;
    if access.pending.is_some() || access.values.next().is_some() {
        return Err(NativeJsonError("native map has unconsumed entries".into()));
    }
    Ok(result)
}
struct Fields<'de> {
    owner: DocumentDeserializer<'de>,
    values: std::collections::btree_map::Iter<'de, String, Document>,
    pending: Option<&'de Document>,
}
impl<'de> MapAccess<'de> for Fields<'de> {
    type Error = NativeJsonError;
    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let Some((key, value)) = self.values.next() else {
            return Ok(None);
        };
        self.pending = Some(value);
        seed.deserialize(KeyDeserializer(key)).map(Some)
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let value = self
            .pending
            .take()
            .ok_or_else(|| NativeJsonError("native map value requested without key".into()))?;
        seed.deserialize(self.owner.child(value)?)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.values.len())
    }
}
struct KeyDeserializer<'de>(&'de str);
impl<'de> Deserializer<'de> for KeyDeserializer<'de> {
    type Error = NativeJsonError;
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_borrowed_str(self.0)
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf
        option unit unit_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}

struct Variant<'de> {
    key: &'de str,
    value: DocumentDeserializer<'de>,
}
impl<'de> EnumAccess<'de> for Variant<'de> {
    type Error = NativeJsonError;
    type Variant = Self;
    fn variant_seed<V: DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self), Self::Error> {
        Ok((seed.deserialize(self.key.into_deserializer())?, self))
    }
}
impl<'de> VariantAccess<'de> for Variant<'de> {
    type Error = NativeJsonError;
    fn unit_variant(self) -> Result<(), Self::Error> {
        Deserialize::deserialize(self.value)
    }
    fn newtype_variant_seed<T: DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        seed.deserialize(self.value)
    }
    fn tuple_variant<V: Visitor<'de>>(
        self,
        length: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.value.deserialize_tuple(length, visitor)
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.value.deserialize_struct("", fields, visitor)
    }
}

/// Deliberate conversion to the native JSON value model. Its number feature
/// profile determines representability; decimals must remain mathematically exact.
///
/// # Errors
/// Rejects numbers that the active native JSON representation cannot preserve exactly.
pub fn document_to_json_value(value: &Document) -> Result<serde_json::Value, NativeJsonError> {
    convert_document_to_json(value, 0)
}
fn convert_document_to_json(
    value: &Document,
    depth: usize,
) -> Result<serde_json::Value, NativeJsonError> {
    if depth > MAXIMUM_JSON_DEPTH {
        return Err(NativeJsonError("native JSON depth exceeded".into()));
    }
    Ok(match value {
        Document::None => serde_json::Value::Null,
        Document::Bool(value) => serde_json::Value::Bool(*value),
        Document::String(value) => serde_json::Value::String(value.clone()),
        Document::Integer(text) | Document::Decimal(text) => {
            let native = text
                .parse::<serde_json::Number>()
                .map_err(|error| NativeJsonError(error.to_string()))?;
            let original = crate::normalize_number(text).map_err(NativeJsonError)?;
            let converted =
                crate::normalize_number(&native.to_string()).map_err(NativeJsonError)?;
            if original != converted {
                return Err(NativeJsonError("number cannot be represented exactly by native JSON Value; use JsonDocument for exact JSON output".into()));
            }
            serde_json::Value::Number(native)
        }
        Document::List(values) => serde_json::Value::Array(
            values
                .iter()
                .map(|value| convert_document_to_json(value, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
        Document::Map(values) => serde_json::Value::Object(
            values
                .iter()
                .map(|(key, value)| Ok((key.clone(), convert_document_to_json(value, depth + 1)?)))
                .collect::<Result<_, NativeJsonError>>()?,
        ),
    })
}
/// # Errors
/// Rejects excessive nesting and numbers outside the supported exact-number limits.
pub fn json_value_to_document(value: &serde_json::Value) -> Result<Document, NativeJsonError> {
    fn convert(value: &serde_json::Value, depth: usize) -> Result<Document, NativeJsonError> {
        if depth > MAXIMUM_JSON_DEPTH {
            return Err(NativeJsonError("native JSON depth exceeded".into()));
        }
        Ok(match value {
            serde_json::Value::Null => Document::None,
            serde_json::Value::Bool(value) => Document::Bool(*value),
            serde_json::Value::String(value) => Document::String(value.clone()),
            serde_json::Value::Number(value) => {
                crate::classify_number(&value.to_string()).map_err(NativeJsonError)?
            }
            serde_json::Value::Array(values) => Document::List(
                values
                    .iter()
                    .map(|value| convert(value, depth + 1))
                    .collect::<Result<_, _>>()?,
            ),
            serde_json::Value::Object(values) => Document::Map(
                values
                    .iter()
                    .map(|(key, value)| Ok((key.clone(), convert(value, depth + 1)?)))
                    .collect::<Result<_, NativeJsonError>>()?,
            ),
        })
    }
    convert(value, 0)
}
