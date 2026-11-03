// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
#[derive(Clone)]
pub struct Record {
    pub pair: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub rows: terrane_collection_support::List<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >,
}
impl Record {
    pub fn terrane_construct() -> Self {
        Self {
            pair: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(
                vec![
                    terrane_int_support::Int::from(0_i128),
                    terrane_int_support::Int::from(0_i128)
                ],
            ),
            rows: terrane_collection_support::List::<
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >::new(
                vec![
                    terrane_collection_support::Tuple::< terrane_int_support::Int
                    >::new(vec![terrane_int_support::Int::from(0_i128),
                    terrane_int_support::Int::from(0_i128)])
                ],
            ),
        }
    }
}
impl TerraneDocumentDecode for Record {
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
                "case.trn:5:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["pair", "rows"];
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
                                "case.trn:5:1",
                            ),
                        );
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
                    "case.trn:6:5",
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
            let field = input_map.get_mut("rows");
            let field_path = __terrane_document_child_path(path, "rows");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    terrane_collection_support::Tuple<terrane_int_support::Int>,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:7:5",
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: Some(
                                    &TerraneDocumentShape {
                                        tuple_length: None,
                                        inner: None,
                                    },
                                ),
                            },
                        ),
                    },
                ) {
                    Ok(decoded) => value.rows = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Record {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Record> for DocumentDecodable {
    fn from(value: Record) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let options: JsonOptions;
    let correct: TerraneDocumentDecodeOutcome<Record>;
    let wrong: TerraneDocumentDecodeOutcome<Record>;
    let nested: TerraneDocumentDecodeOutcome<
        terrane_collection_support::List<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >,
    >;
    let keyed: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Map<
            String,
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >,
    >;
    let accepted: TerraneDocumentDecodeOutcome<
        terrane_collection_support::List<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >,
    >;
    options = default_json_options();
    correct = {
        let __terrane_document_source = &String::from(
            "{\"pair\":[3,4],\"rows\":[[5,6]]}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:10:15",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(correct.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_0 = &correct.value.clone();
        __terrane_raised(__terrane_receiver_value_0.pair
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:11:33-11:54 */)), 0 /* terrane-site: case.trn:11:33-11:54 */) }), String::from("|"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_1 = &correct
        .value.clone(); let __terrane_index_2 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:11:61-11:82 */); let __terrane_index_place_3 =
        __terrane_raised(__terrane_receiver_value_1.rows.get(__terrane_index_2)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
        1 /* terrane-site: case.trn:11:61-11:82 */);
        __terrane_raised(__terrane_index_place_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        2 /* terrane-site: case.trn:11:61-11:85 */)), 2 /* terrane-site: case.trn:11:61-11:85 */) })
    );
    wrong = {
        let __terrane_document_source = &String::from(
            "{\"pair\":[1],\"rows\":[[1,2,3]]}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Record,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || Record::terrane_construct(),
            __terrane_document_allow_unknown,
            "case.trn:12:13",
            "record",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(wrong.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(wrong.value
        .clone().pair.length())), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_receiver_value_4 = &wrong.value.clone();
        __terrane_raised(__terrane_receiver_value_4.rows
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: case.trn:14:61-14:80 */)), 3 /* terrane-site: case.trn:14:61-14:80 */) } .length()))
    );
    nested = {
        let __terrane_document_source = &String::from("[[1,2,3]]");
        let __terrane_document_fallback = &terrane_collection_support::List::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >::new(
            vec![
                terrane_collection_support::Tuple::< terrane_int_support::Int
                >::new(vec![terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)])
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::List<
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:15:14",
            "list of tuple of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nested.diagnostics.length() !=
        0)), String::from("|"),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from({ let
        __terrane_receiver_value_5 = &nested.value.clone();
        __terrane_raised(__terrane_receiver_value_5
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:17:32-17:47 */)), 4 /* terrane-site: case.trn:17:32-17:47 */) } .length()))
    );
    keyed = {
        let __terrane_document_source = &String::from("{\"a\":[1]}");
        let __terrane_document_fallback = &terrane_collection_support::Map::<
            String,
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >::new(
            vec![
                terrane_collection_support::Entry::< String,
                terrane_collection_support::Tuple < terrane_int_support::Int
                >>::new(String::from(""), terrane_collection_support::Tuple::<
                terrane_int_support::Int
                >::new(vec![terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)]))
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Map<
                String,
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:18:13",
            "map of string, tuple of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: Some(2),
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(keyed.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_6 = &keyed.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_6
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:19:31-19:51 */)), 5 /* terrane-site: case.trn:19:31-19:51 */) } .path.clone()), String::from("|"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_7 = &keyed
        .diagnostics.clone(); __terrane_raised(__terrane_receiver_value_7
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:19:63-19:83 */)), 6 /* terrane-site: case.trn:19:63-19:83 */) } .reason.to_owned())
    );
    accepted = {
        let __terrane_document_source = &String::from("[[4,6]]");
        let __terrane_document_fallback = &terrane_collection_support::List::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >::new(
            vec![
                terrane_collection_support::Tuple::< terrane_int_support::Int
                >::new(vec![terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)])
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::List<
                terrane_collection_support::Tuple<terrane_int_support::Int>,
            >,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:20:16",
            "list of tuple of int",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: Some(
                    &TerraneDocumentShape {
                        tuple_length: None,
                        inner: Some(
                            &TerraneDocumentShape {
                                tuple_length: None,
                                inner: None,
                            },
                        ),
                    },
                ),
            },
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(accepted.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_8 = &accepted.value.clone(); let __terrane_index_9 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:21:34-21:51 */); let __terrane_index_place_10 =
        __terrane_raised(__terrane_receiver_value_8.get(__terrane_index_9).ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_9)),
        7 /* terrane-site: case.trn:21:34-21:51 */);
        __terrane_raised(__terrane_index_place_10
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        8 /* terrane-site: case.trn:21:34-21:54 */)), 8 /* terrane-site: case.trn:21:34-21:54 */) })
    );
}
