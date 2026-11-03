fn __terrane_encode_document<T: TerraneDocumentEncode>(
    value: &T, max_depth: usize, max_bytes: usize, source: &str, expected: &str,
) -> TerraneDocumentDecodeOutcome<String> {
    let encoded = (|| {
        let mut writer = terrane_document_support::JsonWriter::new(max_depth, max_bytes)?;
        value.terrane_encode_document(&mut writer)?;
        writer.finish()
    })();
    match encoded {
        Ok(value) => TerraneDocumentDecodeOutcome {
            value, diagnostics: terrane_collection_support::List::new(Vec::new()),
        },
        Err(error) => TerraneDocumentDecodeOutcome {
            value: String::new(),
            diagnostics: terrane_collection_support::List::new(vec![
                __terrane_document_diagnostic(&"$", expected, "encoding", "writer",
                    error.to_string(), source, source),
            ]),
        },
    }
}

trait TerraneDocumentEncode {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError>;
}

impl<T: TerraneDocumentEncode + ?Sized> TerraneDocumentEncode for &T {
    fn terrane_encode_document(
        &self, writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        T::terrane_encode_document(*self, writer)
    }
}

impl TerraneDocumentEncode for String {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.string(self)
    }
}

impl TerraneDocumentEncode for () {
    fn terrane_encode_document(&self, writer: &mut terrane_document_support::JsonWriter) -> Result<(), terrane_document_support::DocumentError> {
        writer.null()
    }
}


impl TerraneDocumentEncode for bool {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.bool(*self)
    }
}

macro_rules! __terrane_document_encode_integer {
    ($($type:ty),+ $(,)?) => {$(
        impl TerraneDocumentEncode for $type {
            fn terrane_encode_document(
                &self,
                writer: &mut terrane_document_support::JsonWriter,
            ) -> Result<(), terrane_document_support::DocumentError> {
                writer.integer(&self.to_string())
            }
        }
    )+};
}

__terrane_document_encode_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, terrane_int_support::Int);

macro_rules! __terrane_document_encode_float {
    ($($type:ty),+ $(,)?) => {$(
        impl TerraneDocumentEncode for $type {
            fn terrane_encode_document(
                &self,
                writer: &mut terrane_document_support::JsonWriter,
            ) -> Result<(), terrane_document_support::DocumentError> {
                if !self.is_finite() {
                    return Err(terrane_document_support::DocumentError(
                        "non-finite floating-point value cannot be encoded as JSON".into(),
                    ));
                }
                let text = if self.abs() >= 1.0e16 || (*self != 0.0 && self.abs() < 1.0e-6) {
                    format!("{self:e}")
                } else {
                    self.to_string()
                };
                writer.decimal(&text)
            }
        }
    )+};
}

__terrane_document_encode_float!(f32, f64);

impl<T: TerraneDocumentEncode> TerraneDocumentEncode for Option<T> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        match self {
            Some(value) => value.terrane_encode_document(writer),
            None => writer.null(),
        }
    }
}

impl<T: TerraneDocumentEncode> TerraneDocumentEncode for terrane_collection_support::List<T> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_array()?;
        for value in self.as_slice() {
            value.terrane_encode_document(writer)?;
        }
        writer.end_array()
    }
}

impl<T: TerraneDocumentEncode> TerraneDocumentEncode for terrane_collection_support::Tuple<T> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_array()?;
        let mut index = 0;
        while let Some(value) = self.get(index) {
            value.terrane_encode_document(writer)?;
            index += 1;
        }
        writer.end_array()
    }
}

macro_rules! __terrane_document_encode_map {
    ($map:ident) => {
        impl<T: TerraneDocumentEncode> TerraneDocumentEncode for terrane_collection_support::$map<String, T> {
            fn terrane_encode_document(&self, writer: &mut terrane_document_support::JsonWriter) -> Result<(), terrane_document_support::DocumentError> {
                writer.begin_object()?;
                for (key, value) in self.as_entries() {
                    writer.key(key)?;
                    value.terrane_encode_document(writer)?;
                }
                writer.end_object()
            }
        }
    }
}
__terrane_document_encode_map!(Map);
__terrane_document_encode_map!(UnorderedMap);