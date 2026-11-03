//! Serde's data model directly into an owned exact document. No JSON Value,
//! text serialization/reparse, or binary64 widening of f32/i128 is involved.
use super::NativeJsonError;
use crate::{Document, MAXIMUM_JSON_DEPTH};
use serde::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use std::collections::BTreeMap;

const MAXIMUM_OUTPUT_BYTES: usize = 16_777_216;
pub(super) fn to_document<T: Serialize + ?Sized>(value: &T) -> Result<Document, NativeJsonError> {
    let mut remaining = MAXIMUM_OUTPUT_BYTES;
    value.serialize(Encoder {
        depth: 0,
        remaining: &mut remaining,
    })
}
struct Encoder<'a> {
    depth: usize,
    remaining: &'a mut usize,
}
impl Encoder<'_> {
    fn charge(&mut self, bytes: usize) -> Result<(), NativeJsonError> {
        if self.depth > MAXIMUM_JSON_DEPTH {
            return Err(NativeJsonError("native document depth exceeded".into()));
        }
        *self.remaining = self
            .remaining
            .checked_sub(bytes)
            .ok_or_else(|| NativeJsonError("native document output byte limit exceeded".into()))?;
        Ok(())
    }
    fn string_cost(value: &str) -> usize {
        2 + value
            .bytes()
            .map(|byte| match byte {
                b'"' | b'\\' | b'\n' | b'\r' | b'\t' | 8 | 12 => 2,
                0..=31 => 6,
                _ => 1,
            })
            .sum::<usize>()
    }
    fn number(mut self, text: &str) -> Result<Document, NativeJsonError> {
        let document = crate::classify_number(text).map_err(NativeJsonError)?;
        let length = match &document {
            Document::Integer(value) | Document::Decimal(value) => value.len(),
            _ => unreachable!(),
        };
        self.charge(length)?;
        Ok(document)
    }
    fn tagged(&mut self, tag: &str) -> Result<(), NativeJsonError> {
        self.charge(3 + Self::string_cost(tag))?;
        self.depth += 1;
        Ok(())
    }
}
macro_rules! integer {
    ($name:ident,$ty:ty) => {
        fn $name(self, value: $ty) -> Result<Document, NativeJsonError> {
            self.number(&value.to_string())
        }
    };
}
impl<'a> ser::Serializer for Encoder<'a> {
    type Ok = Document;
    type Error = NativeJsonError;
    type SerializeSeq = Sequence<'a>;
    type SerializeTuple = Sequence<'a>;
    type SerializeTupleStruct = Sequence<'a>;
    type SerializeTupleVariant = Sequence<'a>;
    type SerializeMap = Mapping<'a>;
    type SerializeStruct = Mapping<'a>;
    type SerializeStructVariant = Mapping<'a>;
    fn serialize_bool(mut self, value: bool) -> Result<Document, Self::Error> {
        self.charge(if value { 4 } else { 5 })?;
        Ok(Document::Bool(value))
    }
    integer!(serialize_i8, i8);
    integer!(serialize_i16, i16);
    integer!(serialize_i32, i32);
    integer!(serialize_i64, i64);
    integer!(serialize_i128, i128);
    integer!(serialize_u8, u8);
    integer!(serialize_u16, u16);
    integer!(serialize_u32, u32);
    integer!(serialize_u64, u64);
    integer!(serialize_u128, u128);
    fn serialize_f32(self, value: f32) -> Result<Document, Self::Error> {
        if !value.is_finite() {
            return Err(NativeJsonError("non-finite native floating value".into()));
        }
        let text = if value.abs() >= 1.0e16 || (value != 0.0 && value.abs() < 1.0e-6) {
            format!("{value:e}")
        } else {
            value.to_string()
        };
        self.number(&text)
    }
    fn serialize_f64(self, value: f64) -> Result<Document, Self::Error> {
        if !value.is_finite() {
            return Err(NativeJsonError("non-finite native floating value".into()));
        }
        let text = if value.abs() >= 1.0e16 || (value != 0.0 && value.abs() < 1.0e-6) {
            format!("{value:e}")
        } else {
            value.to_string()
        };
        self.number(&text)
    }
    fn serialize_char(self, value: char) -> Result<Document, Self::Error> {
        self.collect_str(&value)
    }
    fn serialize_str(mut self, value: &str) -> Result<Document, Self::Error> {
        self.charge(Self::string_cost(value))?;
        Ok(Document::String(value.into()))
    }
    fn serialize_bytes(self, values: &[u8]) -> Result<Document, Self::Error> {
        let mut sequence = self.serialize_seq(Some(values.len()))?;
        for value in values {
            SerializeSeq::serialize_element(&mut sequence, value)?;
        }
        SerializeSeq::end(sequence)
    }
    fn serialize_none(self) -> Result<Document, Self::Error> {
        self.serialize_unit()
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Document, Self::Error> {
        value.serialize(self)
    }
    fn serialize_unit(mut self) -> Result<Document, Self::Error> {
        self.charge(4)?;
        Ok(Document::None)
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Document, Self::Error> {
        self.serialize_unit()
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Document, Self::Error> {
        self.serialize_str(variant)
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Document, Self::Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        mut self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Document, Self::Error> {
        self.tagged(variant)?;
        Ok(Document::Map(BTreeMap::from([(
            variant.into(),
            value.serialize(self)?,
        )])))
    }
    fn serialize_seq(mut self, _: Option<usize>) -> Result<Sequence<'a>, Self::Error> {
        self.charge(2)?;
        Ok(Sequence {
            encoder: self,
            items: Vec::new(),
            tag: None,
        })
    }
    fn serialize_tuple(self, length: usize) -> Result<Sequence<'a>, Self::Error> {
        self.serialize_seq(Some(length))
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        length: usize,
    ) -> Result<Sequence<'a>, Self::Error> {
        self.serialize_seq(Some(length))
    }
    fn serialize_tuple_variant(
        mut self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Sequence<'a>, Self::Error> {
        self.tagged(variant)?;
        let mut sequence = self.serialize_seq(Some(length))?;
        sequence.tag = Some(variant);
        Ok(sequence)
    }
    fn serialize_map(mut self, _: Option<usize>) -> Result<Mapping<'a>, Self::Error> {
        self.charge(2)?;
        Ok(Mapping {
            encoder: self,
            items: BTreeMap::new(),
            pending: None,
            tag: None,
        })
    }
    fn serialize_struct(self, _: &'static str, length: usize) -> Result<Mapping<'a>, Self::Error> {
        self.serialize_map(Some(length))
    }
    fn serialize_struct_variant(
        mut self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Mapping<'a>, Self::Error> {
        self.tagged(variant)?;
        let mut mapping = self.serialize_map(Some(length))?;
        mapping.tag = Some(variant);
        Ok(mapping)
    }
    fn collect_str<T: std::fmt::Display + ?Sized>(
        mut self,
        value: &T,
    ) -> Result<Document, Self::Error> {
        self.charge(2)?;
        let mut output = DisplayString {
            encoder: &mut self,
            text: String::new(),
            error: None,
        };
        if std::fmt::write(&mut output, format_args!("{value}")).is_err() {
            return Err(output
                .error
                .unwrap_or_else(|| NativeJsonError("native Display formatting failed".into())));
        }
        Ok(Document::String(output.text))
    }
}
struct DisplayString<'a, 'b> {
    encoder: &'a mut Encoder<'b>,
    text: String,
    error: Option<NativeJsonError>,
}
impl std::fmt::Write for DisplayString<'_, '_> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        if let Err(error) = self.encoder.charge(Encoder::string_cost(value) - 2) {
            self.error = Some(error);
            return Err(std::fmt::Error);
        }
        self.text.push_str(value);
        Ok(())
    }
}
pub(super) struct Sequence<'a> {
    encoder: Encoder<'a>,
    items: Vec<Document>,
    tag: Option<&'static str>,
}
impl Sequence<'_> {
    fn push<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), NativeJsonError> {
        if !self.items.is_empty() {
            self.encoder.charge(1)?;
        }
        self.items.push(value.serialize(Encoder {
            depth: self.encoder.depth + 1,
            remaining: self.encoder.remaining,
        })?);
        Ok(())
    }
    fn finish(self) -> Document {
        wrap(self.tag, Document::List(self.items))
    }
}
fn wrap(tag: Option<&'static str>, value: Document) -> Document {
    match tag {
        None => value,
        Some(tag) => Document::Map(BTreeMap::from([(tag.into(), value)])),
    }
}
macro_rules! sequence {
    ($trait:ident,$method:ident) => {
        impl $trait for Sequence<'_> {
            type Ok = Document;
            type Error = NativeJsonError;
            fn $method<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
                self.push(value)
            }
            fn end(self) -> Result<Document, Self::Error> {
                Ok(self.finish())
            }
        }
    };
}
sequence!(SerializeSeq, serialize_element);
sequence!(SerializeTuple, serialize_element);
sequence!(SerializeTupleStruct, serialize_field);
sequence!(SerializeTupleVariant, serialize_field);
pub(super) struct Mapping<'a> {
    encoder: Encoder<'a>,
    items: BTreeMap<String, Document>,
    pending: Option<String>,
    tag: Option<&'static str>,
}
impl Mapping<'_> {
    fn key(&mut self, key: String) -> Result<(), NativeJsonError> {
        if self.pending.is_some() || self.items.contains_key(&key) {
            return Err(NativeJsonError(
                "duplicate native document key or missing value".into(),
            ));
        }
        if !self.items.is_empty() {
            self.encoder.charge(1)?;
        }
        self.encoder.charge(1)?;
        self.pending = Some(key);
        Ok(())
    }
    fn value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), NativeJsonError> {
        let key = self
            .pending
            .take()
            .ok_or_else(|| NativeJsonError("native map value without key".into()))?;
        let value = value.serialize(Encoder {
            depth: self.encoder.depth + 1,
            remaining: self.encoder.remaining,
        })?;
        self.items.insert(key, value);
        Ok(())
    }
    fn field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), NativeJsonError> {
        self.encoder.charge(Encoder::string_cost(key))?;
        self.key(key.into())?;
        self.value(value)
    }
    fn finish(self) -> Result<Document, NativeJsonError> {
        if self.pending.is_some() {
            return Err(NativeJsonError("native map key without value".into()));
        }
        Ok(wrap(self.tag, Document::Map(self.items)))
    }
}
impl SerializeMap for Mapping<'_> {
    type Ok = Document;
    type Error = NativeJsonError;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> {
        let key = key.serialize(Encoder {
            depth: self.encoder.depth + 1,
            remaining: self.encoder.remaining,
        })?;
        let Document::String(key) = key else {
            return Err(NativeJsonError(
                "native document maps require string keys".into(),
            ));
        };
        self.key(key)
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.value(value)
    }
    fn end(self) -> Result<Document, Self::Error> {
        self.finish()
    }
}
macro_rules! structure {
    ($trait:ident) => {
        impl $trait for Mapping<'_> {
            type Ok = Document;
            type Error = NativeJsonError;
            fn serialize_field<T: Serialize + ?Sized>(
                &mut self,
                key: &'static str,
                value: &T,
            ) -> Result<(), Self::Error> {
                self.field(key, value)
            }
            fn end(self) -> Result<Document, Self::Error> {
                self.finish()
            }
        }
    };
}
structure!(SerializeStruct);
structure!(SerializeStructVariant);
