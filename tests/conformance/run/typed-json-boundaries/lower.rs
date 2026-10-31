// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: typed-json-boundaries
#[derive(Clone)]
pub struct PolicyError {
    pub message: String,
}
impl PolicyError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("policy-error"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn unknown_policy() -> Result<bool, TerraneError> {
    return Err({
        let value = PolicyError::terrane_construct();
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
            0 /* terrane-site: case.trn:13:5-13:33 */,
        )
    });
}
#[derive(Clone)]
pub struct Numbers {
    pub signed: i128,
    pub unsigned: u128,
    pub adaptive: terrane_int_support::Int,
    pub integral: i64,
    pub real: f64,
    pub narrow: f32,
    pub optional: Option<i64>,
}
impl Numbers {
    pub fn terrane_construct() -> Self {
        Self {
            signed: 0,
            unsigned: 0,
            adaptive: terrane_int_support::Int::from(0_i128),
            integral: 0,
            real: 0.0_f64,
            narrow: 0.0_f32,
            optional: None,
        }
    }
}
impl TerraneDocumentDecode for Numbers {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "numbers",
                source,
                "case.trn:15:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "signed",
            "unsigned",
            "adaptive",
            "integral",
            "real",
            "narrow",
            "optional",
        ];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "numbers",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:15:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("signed");
            let field_path = __terrane_document_child_path(path, "signed");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <i128 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:16:5",
                ) {
                    Ok(decoded) => value.signed = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("unsigned");
            let field_path = __terrane_document_child_path(path, "unsigned");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <u128 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:17:5",
                ) {
                    Ok(decoded) => value.unsigned = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("adaptive");
            let field_path = __terrane_document_child_path(path, "adaptive");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_int_support::Int as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:18:5",
                ) {
                    Ok(decoded) => value.adaptive = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("integral");
            let field_path = __terrane_document_child_path(path, "integral");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <i64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:19:5",
                ) {
                    Ok(decoded) => value.integral = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("real");
            let field_path = __terrane_document_child_path(path, "real");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:20:5",
                ) {
                    Ok(decoded) => value.real = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("narrow");
            let field_path = __terrane_document_child_path(path, "narrow");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <f32 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:21:5",
                ) {
                    Ok(decoded) => value.narrow = decoded,
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
                    i64,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:22:5",
                ) {
                    Ok(decoded) => value.optional = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Numbers {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Numbers> for DocumentDecodable {
    fn from(value: Numbers) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Item {
    pub amount: i64,
}
impl Item {
    pub fn terrane_construct() -> Self {
        Self { amount: 0 }
    }
}
impl TerraneDocumentDecode for Item {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "item",
                source,
                "case.trn:24:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["amount"];
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
                                "case.trn:24:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("amount");
            let field_path = __terrane_document_child_path(path, "amount");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <i64 as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:25:5",
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
pub struct Envelope {
    pub children: terrane_collection_support::List<Item>,
    pub keyed: terrane_collection_support::Map<String, Item>,
    pub pair: terrane_collection_support::Tuple<i64>,
}
impl Envelope {
    pub fn terrane_construct() -> Self {
        Self {
            children: terrane_collection_support::List::<Item>::new(Vec::new()),
            keyed: terrane_collection_support::Map::<String, Item>::new(Vec::new()),
            pair: terrane_collection_support::Tuple::<i64>::new(vec![0, 0]),
        }
    }
}
impl TerraneDocumentDecode for Envelope {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Map(input_map) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "envelope",
                source,
                "case.trn:27:1",
            );
        };
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["odd.key", "keyed", "pair"];
        if !allow_unknown {
            for key in input_map.keys() {
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                &__terrane_document_child_path(path, key),
                                "envelope",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:27:1",
                            ),
                        );
                }
            }
        }
        {
            let field = input_map.get_mut("odd.key");
            let field_path = __terrane_document_child_path(path, "odd.key");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::List<
                    Item,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:28:5",
                ) {
                    Ok(decoded) => value.children = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = input_map.get_mut("keyed");
            let field_path = __terrane_document_child_path(path, "keyed");
            if field.is_none() {} else {
                let field = field.expect("field presence checked");
                match <terrane_collection_support::Map<
                    String,
                    Item,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:29:5",
                ) {
                    Ok(decoded) => value.keyed = decoded,
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
                    i64,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:30:5",
                ) {
                    Ok(decoded) => value.pair = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Envelope {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Envelope> for DocumentDecodable {
    fn from(value: Envelope) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let options: JsonOptions;
    let good: TerraneDocumentDecodeOutcome<Numbers>;
    let bad: TerraneDocumentDecodeOutcome<Numbers>;
    let mut diagnostic: TerraneDocumentDiagnostic;
    let nested: TerraneDocumentDecodeOutcome<Envelope>;
    let duplicate: TerraneDocumentDecodeOutcome<Numbers>;
    let trailing: TerraneDocumentDecodeOutcome<Numbers>;
    let too_deep: TerraneDocumentDecodeOutcome<Envelope>;
    let too_large: TerraneDocumentDecodeOutcome<Numbers>;
    let source: bool;
    let text: String;
    let rejected: TerraneDocumentDecodeOutcome<Numbers>;
    let unknown: TerraneDocumentDecodeOutcome<Numbers>;
    let mut decoded: Option<TerraneDocumentDecodeOutcome<Numbers>> = None;
    let mut error: Option<TerraneError> = None;
    options = default_json_options();
    good = {
        let __terrane_document_source = &String::from(
            "{\"signed\":-170141183460469231731687303715884105728,\"unsigned\":340282366920938463463374607431768211455,\"adaptive\":1e40,\"integral\":1.2e2,\"real\":1e308,\"narrow\":3.4e38,\"optional\":null}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:34:12",
                    "case.trn:34:12",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Numbers::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Numbers::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "numbers", "invalid", "parse", error.message,
                            "case.trn:34:12", "case.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().signed));
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().unsigned));
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().adaptive));
    println!("{}", terrane_scalar_support::scalar_text(&good.value.clone().integral));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(good.value.clone().real > 1.0 &&good
        .value.clone().narrow > 0.0_f32))
    );
    println!(
        "{}", { let mut __terrane_text = String::new(); match &good.value.clone()
        .optional { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    bad = {
        let __terrane_document_source = &String::from(
            "{\"signed\":-170141183460469231731687303715884105729,\"unsigned\":340282366920938463463374607431768211456,\"adaptive\":1.5,\"real\":1e309,\"narrow\":3.5e38,\"optional\":false}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:41:11",
                    "case.trn:41:11",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Numbers::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Numbers::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "numbers", "invalid", "parse", error.message,
                            "case.trn:41:11", "case.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(bad
        .diagnostics.clone().length()))
    );
    let __terrane_iterable_0 = bad.diagnostics.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from("|"), terrane_scalar_support::scalar_text(&diagnostic
            .actual_kind.clone()), String::from("|"),
            terrane_scalar_support::scalar_text(&diagnostic.reason.to_owned())
        );
    }
    nested = {
        let __terrane_document_source = &String::from(
            "{\"odd.key\":[{\"amount\":\"no\"}],\"keyed\":{\"a.b\":{\"amount\":false}},\"pair\":[\"bad\"],\"spare\":true}",
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Envelope as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:45:14",
                    "case.trn:45:14",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Envelope::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Envelope::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "envelope", "invalid", "parse", error.message,
                            "case.trn:45:14", "case.trn:27:1")
                        ],
                    ),
                }
            }
        }
    };
    let __terrane_iterable_1 = nested.diagnostics.clone();
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        diagnostic = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &diagnostic;
        println!(
            "{}{}{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            String::from("|"), terrane_scalar_support::scalar_text(&diagnostic
            .actual_kind.clone()), String::from("|"),
            terrane_scalar_support::scalar_text(&diagnostic.reason.to_owned())
        );
    }
    duplicate = {
        let __terrane_document_source = &String::from("{\"integral\":1,\"integral\":2}");
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:48:17",
                    "case.trn:48:17",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Numbers::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Numbers::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "numbers", "invalid", "parse", error.message,
                            "case.trn:48:17", "case.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    trailing = {
        let __terrane_document_source = &String::from("{\"integral\":1} true");
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:49:16",
                    "case.trn:49:16",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Numbers::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Numbers::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "numbers", "invalid", "parse", error.message,
                            "case.trn:49:16", "case.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    too_deep = {
        let __terrane_document_source = &String::from("{\"odd.key\":[{\"amount\":1}]}");
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(128_i128),
        );
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Envelope as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:50:16",
                    "case.trn:50:16",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Envelope::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Envelope::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "envelope", "invalid", "parse", error.message,
                            "case.trn:50:16", "case.trn:27:1")
                        ],
                    ),
                }
            }
        }
    };
    too_large = {
        let __terrane_document_source = &String::from("{\"integral\":12345}");
        let __terrane_document_options = &JsonOptions::terrane_construct(
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(8_i128),
        );
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:51:17",
                    "case.trn:51:17",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Numbers::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Numbers::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "numbers", "invalid", "parse", error.message,
                            "case.trn:51:17", "case.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(duplicate.diagnostics.length() != 0
        &&{ let __terrane_receiver_value_0 = &duplicate.diagnostics.clone();
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        1 /* terrane-site: case.trn:52:33-52:57 */)), 1 /* terrane-site: case.trn:52:33-52:57 */) } .reason.to_owned().as_str() == "parse" &&trailing
        .diagnostics.length() != 0 &&{ let __terrane_receiver_value_1 = &trailing
        .diagnostics.clone(); __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:52:100-52:123 */)), 2 /* terrane-site: case.trn:52:100-52:123 */) } .reason.to_owned().as_str() == "parse" &&too_deep
        .diagnostics.length() != 0 &&{ let __terrane_receiver_value_2 = &too_deep
        .diagnostics.clone(); __terrane_raised(__terrane_receiver_value_2
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: case.trn:52:166-52:189 */)), 3 /* terrane-site: case.trn:52:166-52:189 */) } .reason.to_owned().as_str() == "parse" &&too_large
        .diagnostics.length() != 0 &&{ let __terrane_receiver_value_3 = &too_large
        .diagnostics.clone(); __terrane_raised(__terrane_receiver_value_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:52:233-52:257 */)), 4 /* terrane-site: case.trn:52:233-52:257 */) } .reason.to_owned().as_str() == "parse"))
    );
    source = true;
    text = String::from("{\"integral\":7,\"future\":{\"anything\":[1,2,3]}}");
    rejected = {
        let __terrane_document_source = &text.clone();
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:55:16",
                    "case.trn:55:16",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Numbers::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Numbers::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "numbers", "invalid", "parse", error.message,
                            "case.trn:55:16", "case.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    unknown = {
        let __terrane_document_source = &text;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = source;
        let __terrane_document_path = TerraneDocumentPath::Root("$");
        match terrane_document_support::parse_json_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
        ) {
            Ok(mut input) => {
                match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                    &mut input,
                    &__terrane_document_path,
                    __terrane_document_allow_unknown,
                    "case.trn:56:15",
                    "case.trn:56:15",
                ) {
                    Ok(value) => {
                        TerraneDocumentDecodeOutcome {
                            value,
                            diagnostics: terrane_collection_support::List::new(
                                Vec::new(),
                            ),
                        }
                    }
                    Err(diagnostics) => {
                        TerraneDocumentDecodeOutcome {
                            value: Numbers::terrane_construct(),
                            diagnostics: terrane_collection_support::List::new(
                                diagnostics,
                            ),
                        }
                    }
                }
            }
            Err(error) => {
                TerraneDocumentDecodeOutcome {
                    value: Numbers::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(
                        vec![
                            __terrane_document_diagnostic(&__terrane_document_path,
                            "numbers", "invalid", "parse", error.message,
                            "case.trn:56:15", "case.trn:15:1")
                        ],
                    ),
                }
            }
        }
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(rejected.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&unknown.value.clone().integral)
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = decoded
                .insert({
                    let __terrane_document_source = &String::from("not json");
                    let __terrane_document_options = &options;
                    let __terrane_document_allow_unknown = __terrane_traced_completion!(
                        unknown_policy(), 5 /* terrane-site: case.trn:59:69-59:84 */
                    );
                    let __terrane_document_path = TerraneDocumentPath::Root("$");
                    match terrane_document_support::parse_json_document(
                        __terrane_document_source,
                        terrane_limit(&__terrane_document_options.max_depth),
                        terrane_limit(&__terrane_document_options.max_bytes),
                    ) {
                        Ok(mut input) => {
                            match <Numbers as TerraneDocumentDecode>::terrane_decode_document(
                                &mut input,
                                &__terrane_document_path,
                                __terrane_document_allow_unknown,
                                "case.trn:59:19",
                                "case.trn:59:19",
                            ) {
                                Ok(value) => {
                                    TerraneDocumentDecodeOutcome {
                                        value,
                                        diagnostics: terrane_collection_support::List::new(
                                            Vec::new(),
                                        ),
                                    }
                                }
                                Err(diagnostics) => {
                                    TerraneDocumentDecodeOutcome {
                                        value: Numbers::terrane_construct(),
                                        diagnostics: terrane_collection_support::List::new(
                                            diagnostics,
                                        ),
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            TerraneDocumentDecodeOutcome {
                                value: Numbers::terrane_construct(),
                                diagnostics: terrane_collection_support::List::new(
                                    vec![
                                        __terrane_document_diagnostic(&__terrane_document_path,
                                        "numbers", "invalid", "parse", error.message,
                                        "case.trn:59:19", "case.trn:15:1")
                                    ],
                                ),
                            }
                        }
                    }
                });
            println!(
                "{}", terrane_scalar_support::scalar_text(&{ let
                __terrane_receiver_value_4 = &decoded.as_ref()
                .expect("flow-proven available binding").clone().diagnostics.clone();
                __terrane_raised_completion!(__terrane_receiver_value_4
                .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                6 /* terrane-site: case.trn:60:16-60:38 */)), 6 /* terrane-site: case.trn:60:16-60:38 */) } .reason.to_owned())
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
                    let _ = error.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&error.as_ref()
                        .expect("flow-proven available binding").clone().message()
                        .to_owned())
                    );
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
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
