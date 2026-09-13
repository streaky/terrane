// Generated deterministically by Terrane <version>.
// Runtime support: platform_data_base.rs, platform_documents.rs, platform_json.rs, platform_yaml.rs, typed_documents.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-document-support
// Source: case.trn
// Namespace: typed-document-decoding
#[derive(Clone)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
}
impl Endpoint {
    pub fn terrane_construct() -> Self {
        Self {
            host: String::from(""),
            port: 443,
        }
    }
}
impl TerraneDocumentDecode for Endpoint {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "endpoint", "invalid", "parse",
                    input.message.clone(), source, "case.trn:9:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "endpoint",
                source,
                "case.trn:9:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["host", "port"];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "endpoint",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:9:1",
                            ),
                        );
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "host");
            let field_path = __terrane_document_child_path(path, "host");
            if field.failed {} else {
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:10:5",
                ) {
                    Ok(decoded) => value.host = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "port");
            let field_path = __terrane_document_child_path(path, "port");
            if field.failed {} else {
                match <u16 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:11:5",
                ) {
                    Ok(decoded) => value.port = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Endpoint {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Endpoint> for DocumentDecodable {
    fn from(value: Endpoint) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct ServiceConfig {
    pub service_name: String,
    pub endpoint: Endpoint,
    pub note: Option<String>,
    pub ports: terrane_collection_support::List<u16>,
    pub labels: terrane_collection_support::Map<String, String>,
    pub coordinates: terrane_collection_support::Tuple<terrane_int_support::Int>,
    pub ratio: f64,
}
impl ServiceConfig {
    pub fn terrane_construct() -> Self {
        Self {
            service_name: String::from(""),
            endpoint: Endpoint::terrane_construct(),
            note: None,
            ports: terrane_collection_support::List::<u16>::new(vec![0]),
            labels: terrane_collection_support::Map::<
                String,
                String,
            >::new(
                vec![
                    terrane_collection_support::Entry::< String, String
                    >::new(String::from(""), String::from(""))
                ],
            ),
            coordinates: terrane_collection_support::Tuple::<
                terrane_int_support::Int,
            >::new(vec![terrane_int_support::Int::from(0_i128)]),
            ratio: 0.1,
        }
    }
    pub fn validate_document(&self) -> Option<String> {
        if self.service_name == String::from("reserved") {
            return Some(String::from("service name is reserved"));
        }
        return None;
    }
}
impl TerraneDocumentDecode for ServiceConfig {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "service-config", "invalid",
                    "parse", input.message.clone(), source, "case.trn:13:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "service-config",
                source,
                "case.trn:13:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &[
            "serviceName",
            "endpoint",
            "note",
            "ports",
            "labels",
            "coordinates",
            "ratio",
        ];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "service-config",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:13:1",
                            ),
                        );
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "serviceName");
            let field_path = __terrane_document_child_path(path, "serviceName");
            if field.failed {} else {
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:14:5",
                ) {
                    Ok(decoded) => value.service_name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "endpoint");
            let field_path = __terrane_document_child_path(path, "endpoint");
            if field.failed {} else {
                match <Endpoint as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:15:5",
                ) {
                    Ok(decoded) => value.endpoint = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "note");
            let field_path = __terrane_document_child_path(path, "note");
            if field.failed {} else {
                match <Option<
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:16:5",
                ) {
                    Ok(decoded) => value.note = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "ports");
            let field_path = __terrane_document_child_path(path, "ports");
            if field.failed {} else {
                match <terrane_collection_support::List<
                    u16,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:17:5",
                ) {
                    Ok(decoded) => value.ports = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "labels");
            let field_path = __terrane_document_child_path(path, "labels");
            if field.failed {} else {
                match <terrane_collection_support::Map<
                    String,
                    String,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:18:5",
                ) {
                    Ok(decoded) => value.labels = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "coordinates");
            let field_path = __terrane_document_child_path(path, "coordinates");
            if field.failed {} else {
                match <terrane_collection_support::Tuple<
                    terrane_int_support::Int,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:19:5",
                ) {
                    Ok(decoded) => value.coordinates = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "ratio");
            let field_path = __terrane_document_child_path(path, "ratio");
            if field.failed {} else {
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:20:5",
                ) {
                    Ok(decoded) => value.ratio = decoded,
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
                            "service-config",
                            "map",
                            "validation",
                            message,
                            source,
                            "case.trn:13:1",
                        ),
                    );
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for ServiceConfig {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<ServiceConfig> for DocumentDecodable {
    fn from(value: ServiceConfig) -> Self {
        Self(Box::new(value))
    }
}
impl DocumentValidatableProtocol for ServiceConfig {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol> {
        Box::new(self.clone())
    }
    fn validate_document(&self) -> Option<String> {
        ServiceConfig::validate_document(&*self)
    }
}
impl From<ServiceConfig> for DocumentValidatable {
    fn from(value: ServiceConfig) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
}
impl Waypoint {
    pub fn terrane_construct() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl TerraneDocumentDecode for Waypoint {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "waypoint", "invalid", "parse",
                    input.message.clone(), source, "case.trn:27:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "waypoint",
                source,
                "case.trn:27:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["x", "y"];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "waypoint",
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
            let field = terrane_document_support::document_field(input, "x");
            let field_path = __terrane_document_child_path(path, "x");
            if field.failed {} else {
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:28:5",
                ) {
                    Ok(decoded) => value.x = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "y");
            let field_path = __terrane_document_child_path(path, "y");
            if field.failed {} else {
                match <f64 as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:29:5",
                ) {
                    Ok(decoded) => value.y = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Waypoint {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Waypoint> for DocumentDecodable {
    fn from(value: Waypoint) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct Route {
    pub name: String,
    pub points: terrane_collection_support::List<Waypoint>,
    pub active: bool,
}
impl Route {
    pub fn terrane_construct() -> Self {
        Self {
            name: String::from(""),
            points: terrane_collection_support::List::<
                Waypoint,
            >::new(vec![Waypoint::terrane_construct()]),
            active: false,
        }
    }
}
impl TerraneDocumentDecode for Route {
    fn terrane_decode_document(
        input: &terrane_document_support::DataResult,
        path: &str,
        allow_unknown: bool,
        source: &str,
        _field_source: &str,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if input.failed {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "route", "invalid", "parse",
                    input.message.clone(), source, "case.trn:31:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "route",
                source,
                "case.trn:31:1",
            );
        }
        let mut value = Self::terrane_construct();
        let mut diagnostics = Vec::new();
        let declared_fields: &[&str] = &["name", "points", "active"];
        if !allow_unknown {
            for index in 0..terrane_document_support::document_length(input) {
                let key = terrane_document_support::document_key(input, index);
                if !declared_fields.contains(&key.as_str()) {
                    diagnostics
                        .push(
                            __terrane_document_diagnostic(
                                __terrane_document_child_path(path, &key),
                                "route",
                                "present",
                                "unknown-field",
                                format!("unknown field `{key}`"),
                                source,
                                "case.trn:31:1",
                            ),
                        );
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "name");
            let field_path = __terrane_document_child_path(path, "name");
            if field.failed {} else {
                match <String as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:32:5",
                ) {
                    Ok(decoded) => value.name = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "points");
            let field_path = __terrane_document_child_path(path, "points");
            if field.failed {} else {
                match <terrane_collection_support::List<
                    Waypoint,
                > as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:33:5",
                ) {
                    Ok(decoded) => value.points = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        {
            let field = terrane_document_support::document_field(input, "active");
            let field_path = __terrane_document_child_path(path, "active");
            if field.failed {} else {
                match <bool as TerraneDocumentDecode>::terrane_decode_document(
                    &field,
                    &field_path,
                    allow_unknown,
                    source,
                    "case.trn:34:5",
                ) {
                    Ok(decoded) => value.active = decoded,
                    Err(mut field_diagnostics) => {
                        diagnostics.append(&mut field_diagnostics)
                    }
                }
            }
        }
        if diagnostics.is_empty() { Ok(value) } else { Err(diagnostics) }
    }
}
impl DocumentDecodableProtocol for Route {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol> {
        Box::new(self.clone())
    }
}
impl From<Route> for DocumentDecodable {
    fn from(value: Route) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    let decoded: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":\"api\",\"endpoint\":{\"host\":\"localhost\",\"port\":8443},\"ports\":[80,443],\"labels\":{\"tier\":\"edge\"},\"coordinates\":[4,9],\"ratio\":0.1}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:37:15",
            "case.trn:37:15",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(decoded.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&decoded.value.clone().service_name),
        terrane_scalar_support::scalar_text(&decoded.value.clone().endpoint.host),
        terrane_scalar_support::scalar_text(&decoded.value.clone().endpoint.port)
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value
        .clone().ports
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        0 /* terrane-site: case.trn:39:12-39:34 */)), 0 /* terrane-site: case.trn:39:12-39:34 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value.clone()
        .labels.get_or_error(&String::from("tier")), 1 /* terrane-site: case.trn:39:36-39:64 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value.clone()
        .coordinates
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:39:66-39:94 */)), 2 /* terrane-site: case.trn:39:66-39:94 */)), terrane_scalar_support::scalar_text(&decoded.value
        .clone().ratio)
    );
    let malformed: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":3,\"endpoint\":{\"host\":false},\"ports\":[1,70000],\"ratio\":0.1,\"extra\":true}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:41:17",
            "case.trn:41:17",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(malformed.diagnostics.length() !=
        0)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(malformed
        .diagnostics.clone().length()))
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &malformed.diagnostics.clone(),
    );
    loop {
        let diagnostic = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&diagnostic.path.clone()),
            terrane_scalar_support::scalar_text(&String::from(":")),
            terrane_scalar_support::scalar_text(&diagnostic.reason.to_owned())
        );
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(malformed
        .diagnostics.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:45:13-45:37 */)), 3 /* terrane-site: case.trn:45:13-45:37 */).source.clone().contains(&String::from("case.trn"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(malformed.diagnostics
        .clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:45:69-45:93 */)), 4 /* terrane-site: case.trn:45:69-45:93 */).field_source.clone()
        .contains(&String::from("case.trn")))
    );
    let invalid: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":\"reserved\",\"endpoint\":{\"host\":\"ok\"}}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:47:15",
            "case.trn:47:15",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&__terrane_raised(invalid.diagnostics
        .clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:48:28-48:50 */)), 5 /* terrane-site: case.trn:48:28-48:50 */).reason.to_owned()),
        terrane_scalar_support::scalar_text(&__terrane_raised(invalid.diagnostics.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:48:59-48:81 */)), 6 /* terrane-site: case.trn:48:59-48:81 */).message.clone())
    );
    let permissive: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from(
            "{\"serviceName\":\"extra\",\"endpoint\":{\"host\":\"ok\"},\"extra\":true}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            true,
            "case.trn:50:18",
            "case.trn:50:18",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(permissive.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&permissive.value.clone().endpoint.port)
    );
    let yaml: TerraneDocumentDecodeOutcome<ServiceConfig> = {
        let source = String::from("serviceName: yaml\nendpoint:\n  host: local");
        let options = default_yaml_options();
        let input = terrane_document_support::parse_yaml(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
            terrane_limit(&options.max_alias_nodes),
        );
        match <ServiceConfig as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:53:12",
            "case.trn:53:12",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: ServiceConfig::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(yaml.diagnostics.length() != 0)),
        terrane_scalar_support::scalar_text(&yaml.value.clone().service_name),
        terrane_scalar_support::scalar_text(&yaml.value.clone().endpoint.host)
    );
    let journey: TerraneDocumentDecodeOutcome<Route> = {
        let source = String::from(
            "{\"name\":\"coast\",\"points\":[{\"x\":1.25,\"y\":2.75},{\"x\":3.5,\"y\":4.5}],\"active\":true}",
        );
        let options = default_json_options();
        let input = terrane_document_support::parse_json(
            &source,
            terrane_limit(&options.max_depth),
            terrane_limit(&options.max_bytes),
        );
        match <Route as TerraneDocumentDecode>::terrane_decode_document(
            &input,
            "$",
            false,
            "case.trn:56:15",
            "case.trn:56:15",
        ) {
            Ok(value) => {
                TerraneDocumentDecodeOutcome {
                    value,
                    diagnostics: terrane_collection_support::List::new(Vec::new()),
                }
            }
            Err(diagnostics) => {
                TerraneDocumentDecodeOutcome {
                    value: Route::terrane_construct(),
                    diagnostics: terrane_collection_support::List::new(diagnostics),
                }
            }
        }
    };
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(journey.diagnostics.length() !=
        0)), terrane_scalar_support::scalar_text(&journey.value.clone().name),
        terrane_scalar_support::scalar_text(&__terrane_raised(journey.value.clone()
        .points
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:57:48-57:71 */)), 7 /* terrane-site: case.trn:57:48-57:71 */).y), terrane_scalar_support::scalar_text(&journey.value
        .clone().active)
    );
}
// Source: core/documents.trn
// Namespace: core/documents
#[derive(Clone)]
pub struct DocumentInteger {
    pub text: String,
}
impl DocumentInteger {
    pub fn terrane_construct(text: String) -> Self {
        let mut value = Self { text: String::from("0") };
        value.construct(text);
        value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
}
#[derive(Clone)]
pub struct DocumentDecimal {
    pub coefficient: String,
    pub exponent: terrane_int_support::Int,
    pub text: String,
}
impl DocumentDecimal {
    pub fn terrane_construct(
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) -> Self {
        let mut value = Self {
            coefficient: String::from("0"),
            exponent: terrane_int_support::Int::from(0_i128),
            text: String::from("0"),
        };
        value.construct(coefficient, exponent, text);
        value
    }
    pub fn construct(
        &mut self,
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) {
        self.coefficient = coefficient;
        self.exponent = exponent.clone();
        self.text = text;
    }
}
pub trait SerializableProtocol {
    fn clone_box(&self) -> Box<dyn SerializableProtocol>;
    fn separate_box(&self) -> Box<dyn SerializableProtocol>;
    fn to_document(&self) -> DocumentValue;
}
impl Clone for Box<dyn SerializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Serializable(Box<dyn SerializableProtocol>);
impl Clone for Serializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Serializable {
    pub fn to_document(&self) -> DocumentValue {
        self.0.to_document()
    }
}
pub trait DeserializableProtocol {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol>;
    fn separate_box(&self) -> Box<dyn DeserializableProtocol>;
    fn from_document(&self, value: DocumentValue) -> DocumentResult;
}
impl Clone for Box<dyn DeserializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Deserializable(Box<dyn DeserializableProtocol>);
impl Clone for Deserializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Deserializable {
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        self.0.from_document(value)
    }
}
pub trait DocumentDecodableProtocol {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol>;
}
impl Clone for Box<dyn DocumentDecodableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[allow(
    dead_code,
    reason = "marker interface storage is materialized only when a value is erased to that marker"
)]
pub struct DocumentDecodable(Box<dyn DocumentDecodableProtocol>);
impl Clone for DocumentDecodable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentDecodable {}
pub trait DocumentValidatableProtocol {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn validate_document(&self) -> Option<String>;
}
impl Clone for Box<dyn DocumentValidatableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct DocumentValidatable(Box<dyn DocumentValidatableProtocol>);
impl Clone for DocumentValidatable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentValidatable {
    pub fn validate_document(&self) -> Option<String> {
        self.0.validate_document()
    }
}
#[derive(Clone)]
pub struct DocumentValue {
    pub raw: terrane_document_support::DataResult,
    pub encoded: String,
    pub kind: String,
    pub scalar: String,
    pub integer: DocumentInteger,
    pub decimal: DocumentDecimal,
}
impl DocumentValue {
    pub fn terrane_construct(raw: terrane_document_support::DataResult) -> Self {
        let mut value = Self {
            raw: terrane_empty_document(),
            encoded: String::from(""),
            kind: String::from("invalid"),
            scalar: String::from(""),
            integer: DocumentInteger::terrane_construct(String::from("0")),
            decimal: DocumentDecimal::terrane_construct(
                String::from("0"),
                terrane_int_support::Int::from(0_i128),
                String::from("0"),
            ),
        };
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: terrane_document_support::DataResult) {
        self.kind = terrane_document_kind(&raw);
        self.scalar = terrane_document_text(&raw);
        self.encoded = terrane_data_encoded(&raw);
        if self.kind == String::from("integer") {
            self.integer = DocumentInteger::terrane_construct(self.scalar.clone());
        }
        if self.kind == String::from("decimal") {
            self.decimal = DocumentDecimal::terrane_construct(
                terrane_document_coefficient(&raw),
                terrane_document_exponent(&raw),
                self.scalar.clone(),
            );
        }
        self.raw = raw;
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_document_length(&self.raw);
    }
    pub fn to_document(&self) -> DocumentValue {
        return self.clone();
    }
    pub fn item(&self, index: terrane_int_support::Int) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_item(
            &self.raw,
            index.clone(),
        );
        return make_document_result(raw);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.raw, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_field(
            &self.raw,
            name,
        );
        return make_document_result(raw);
    }
}
impl SerializableProtocol for DocumentValue {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        DocumentValue::to_document(&*self)
    }
}
impl From<DocumentValue> for Serializable {
    fn from(value: DocumentValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct DocumentResult {
    pub failed: bool,
    pub message: String,
    pub path: String,
    pub expected: String,
    pub value: DocumentValue,
}
impl DocumentResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            path: String::from("$"),
            expected: String::from(""),
            value: DocumentValue::terrane_construct(terrane_empty_document()),
        };
        value.construct(failed, message, path, expected, raw);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) {
        self.failed = failed;
        self.message = message;
        self.path = path;
        self.expected = expected;
        self.value = DocumentValue::terrane_construct(raw);
    }
}
#[derive(Clone)]
pub struct DocumentMapping {
    pub descriptor_name: String,
    pub expected_kind: String,
    pub field_names: terrane_collection_support::List<String>,
    pub optional_fields: terrane_collection_support::List<String>,
    pub default_fields: terrane_collection_support::List<String>,
    pub default_values: terrane_collection_support::List<String>,
    pub allow_unknown: bool,
}
impl DocumentMapping {
    pub fn terrane_construct(
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) -> Self {
        let mut value = Self {
            descriptor_name: String::from("document-value"),
            expected_kind: String::from("map"),
            field_names: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            optional_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            allow_unknown: false,
        };
        value.construct(descriptor_name, expected_kind, allow_unknown);
        value
    }
    pub fn construct(
        &mut self,
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) {
        self.descriptor_name = descriptor_name;
        self.expected_kind = expected_kind;
        self.allow_unknown = allow_unknown;
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return decode_document(value.clone(), self.clone());
    }
}
impl DeserializableProtocol for DocumentMapping {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn from_document(&self, value: DocumentValue) -> DocumentResult {
        DocumentMapping::from_document(&*self, value)
    }
}
impl From<DocumentMapping> for Deserializable {
    fn from(value: DocumentMapping) -> Self {
        Self(Box::new(value))
    }
}
pub fn serialize_document(value: Serializable) -> DocumentValue {
    return value.to_document();
}
pub fn deserialize_document(
    value: DocumentValue,
    destination: Deserializable,
) -> DocumentResult {
    return destination.from_document(value.clone());
}
pub fn make_document_result(
    raw: terrane_document_support::DataResult,
) -> DocumentResult {
    return DocumentResult::terrane_construct(
        terrane_data_failed(&raw),
        terrane_data_message(&raw),
        terrane_data_path(&raw),
        terrane_data_expected(&raw),
        raw,
    );
}
pub fn make_document_none() -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_none());
}
pub fn make_document_bool(value: bool) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_bool(value));
}
pub fn make_document_string(value: String) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_string(value));
}
pub fn make_document_integer(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_integer(value));
}
pub fn make_document_decimal(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_decimal(value));
}
#[derive(Clone)]
pub struct DocumentMapEntries {
    pub raw: terrane_document_support::DataResult,
}
impl DocumentMapEntries {
    pub fn terrane_construct() -> Self {
        let mut value = Self {
            raw: terrane_make_document_map(),
        };
        value.construct();
        value
    }
    pub fn construct(&mut self) {
        self.raw = terrane_make_document_map();
    }
    pub fn append(&mut self, key: String, value: DocumentValue) {
        self.raw = terrane_document_map_insert(&self.raw, key, &value.raw);
    }
}
pub fn append_document_map_entry(
    mut entries: DocumentMapEntries,
    key: String,
    value: DocumentValue,
) -> DocumentMapEntries {
    entries.append(key, value.clone());
    return entries.clone();
}
pub fn make_document_list(
    values: terrane_collection_support::List<DocumentValue>,
) -> DocumentResult {
    let mut raw: terrane_document_support::DataResult = terrane_make_document_list();
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(values.length()))
    {
        raw = terrane_document_list_append(
            &raw,
            &__terrane_raised(
                    values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                8 /* terrane-site: core/documents.trn:140:47-140:60 */,
                            ),
                        ),
                    8 /* terrane-site: core/documents.trn:140:47-140:60 */,
                )
                .raw,
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return make_document_result(raw);
}
pub fn make_document_map(entries: DocumentMapEntries) -> DocumentResult {
    return make_document_result(entries.raw);
}
pub fn mapping_required_fields(
    mapping: DocumentMapping,
) -> terrane_collection_support::List<String> {
    let fields: terrane_collection_support::List<String> = mapping.field_names;
    let optional_fields: terrane_collection_support::List<String> = mapping
        .optional_fields;
    let default_fields: terrane_collection_support::List<String> = mapping
        .default_fields;
    let mut required: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = required.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(fields.length()),
            )
        {
            let field: String = __terrane_raised(
                fields
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            9 /* terrane-site: core/documents.trn:153:17-153:30 */,
                        ),
                    ),
                9 /* terrane-site: core/documents.trn:153:17-153:30 */,
            );
            let mut optional: bool = false;
            let mut optional_index: terrane_int_support::Int = terrane_int_support::Int::from(
                0_i128,
            );
            while optional_index.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(optional_fields.length()),
                )
            {
                if __terrane_raised(
                    optional_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &optional_index.clone(),
                                ),
                                10 /* terrane-site: core/documents.trn:157:16-157:47 */,
                            ),
                        ),
                    10 /* terrane-site: core/documents.trn:157:16-157:47 */,
                ) == field
                {
                    optional = true;
                }
                optional_index = optional_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            let mut defaulted: bool = false;
            let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
                0_i128,
            );
            while default_index.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(default_fields.length()),
                )
            {
                if __terrane_raised(
                    default_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                ),
                                11 /* terrane-site: core/documents.trn:163:16-163:45 */,
                            ),
                        ),
                    11 /* terrane-site: core/documents.trn:163:16-163:45 */,
                ) == field
                {
                    defaulted = true;
                }
                default_index = default_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            if field != String::from("") && !optional && !defaulted {
                __terrane_list_append_0.push(field);
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return required.clone();
}
pub fn decode_document(
    value: DocumentValue,
    mapping: DocumentMapping,
) -> DocumentResult {
    let required: terrane_collection_support::List<String> = mapping_required_fields(
        mapping.clone(),
    );
    let mut declared_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut field_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    {
        let __terrane_list_append_1 = declared_fields.make_unique();
        while field_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.field_names.length()),
            )
        {
            if __terrane_raised(
                mapping
                    .field_names
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &field_index.clone(),
                            ),
                            12 /* terrane-site: core/documents.trn:176:12-176:44 */,
                        ),
                    ),
                12 /* terrane-site: core/documents.trn:176:12-176:44 */,
            ) != String::from("")
            {
                __terrane_list_append_1
                    .push(
                        __terrane_raised(
                            mapping
                                .field_names
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &field_index.clone(),
                                        ),
                                        13 /* terrane-site: core/documents.trn:177:37-177:69 */,
                                    ),
                                ),
                            13 /* terrane-site: core/documents.trn:177:37-177:69 */,
                        ),
                    );
            }
            field_index = field_index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut default_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_values: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while default_index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(mapping.default_fields.length()),
        )
        && default_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.default_values.length()),
            )
    {
        if __terrane_raised(
            mapping
                .default_fields
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &default_index.clone(),
                        ),
                        14 /* terrane-site: core/documents.trn:183:12-183:49 */,
                    ),
                ),
            14 /* terrane-site: core/documents.trn:183:12-183:49 */,
        ) != String::from("")
        {
            default_fields
                .append(
                    __terrane_raised(
                        mapping
                            .default_fields
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    15 /* terrane-site: core/documents.trn:184:36-184:73 */,
                                ),
                            ),
                        15 /* terrane-site: core/documents.trn:184:36-184:73 */,
                    ),
                );
            default_values
                .append(
                    __terrane_raised(
                        mapping
                            .default_values
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    16 /* terrane-site: core/documents.trn:185:36-185:73 */,
                                ),
                            ),
                        16 /* terrane-site: core/documents.trn:185:36-185:73 */,
                    ),
                );
        }
        default_index = default_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let raw: terrane_document_support::DataResult = terrane_validate_mapping(
        &value.raw,
        mapping.expected_kind,
        required,
        declared_fields,
        default_fields,
        default_values,
        mapping.allow_unknown,
    );
    let mut result: DocumentResult = make_document_result(raw);
    if result.failed {
        result.expected = mapping.descriptor_name.clone();
    }
    return result.clone();
}
// Source: core/json.trn
// Namespace: core/documents/json
#[derive(Clone)]
pub struct JsonOptions {
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
}
impl JsonOptions {
    pub fn terrane_construct(
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            max_depth: terrane_int_support::Int::from(256_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
        };
        value.construct(max_depth, max_bytes);
        value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) {
        self.max_depth = max_depth.clone();
        self.max_bytes = max_bytes.clone();
    }
}
pub fn default_json_options() -> JsonOptions {
    return JsonOptions::terrane_construct(
        terrane_int_support::Int::from(256_i128),
        terrane_int_support::Int::from(16777216_i128),
    );
}
pub fn parse_json(input: String, options: JsonOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_json(value: DocumentValue, options: JsonOptions) -> DocumentResult {
    let _ = &options;
    let raw: terrane_document_support::DataResult = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn canonical_json(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn decode_json(
    input: String,
    mapping: Deserializable,
    options: JsonOptions,
) -> DocumentResult {
    let parsed: DocumentResult = parse_json(input, options.clone());
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value, mapping.clone());
}
pub fn encode_json(value: Serializable, options: JsonOptions) -> DocumentResult {
    return stringify_json(serialize_document(value.clone()), options.clone());
}
// Source: core/yaml.trn
// Namespace: core/documents/yaml
#[derive(Clone)]
pub struct YamlOptions {
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
    pub max_alias_nodes: terrane_int_support::Int,
}
impl YamlOptions {
    pub fn terrane_construct(
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_alias_nodes: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            max_depth: terrane_int_support::Int::from(128_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
            max_alias_nodes: terrane_int_support::Int::from(65536_i128),
        };
        value.construct(max_depth, max_bytes, max_alias_nodes);
        value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_alias_nodes: terrane_int_support::Int,
    ) {
        self.max_depth = max_depth.clone();
        self.max_bytes = max_bytes.clone();
        self.max_alias_nodes = max_alias_nodes.clone();
    }
}
pub fn default_yaml_options() -> YamlOptions {
    return YamlOptions::terrane_construct(
        terrane_int_support::Int::from(128_i128),
        terrane_int_support::Int::from(16777216_i128),
        terrane_int_support::Int::from(65536_i128),
    );
}
pub fn make_yaml_options(
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
    max_alias_nodes: terrane_int_support::Int,
) -> YamlOptions {
    return YamlOptions::terrane_construct(
        max_depth.clone(),
        max_bytes.clone(),
        max_alias_nodes.clone(),
    );
}
pub fn parse_yaml(input: String, options: YamlOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_yaml_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
        options.max_alias_nodes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_yaml(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn decode_yaml(
    input: String,
    mapping: Deserializable,
    options: YamlOptions,
) -> DocumentResult {
    let parsed: DocumentResult = parse_yaml(input, options.clone());
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value, mapping.clone());
}
pub fn encode_yaml(value: Serializable) -> DocumentResult {
    return stringify_yaml(serialize_document(value.clone()));
}
