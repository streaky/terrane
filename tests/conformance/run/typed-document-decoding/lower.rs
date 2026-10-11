// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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
                    input.message.clone(), source, "case.trn:8:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "endpoint",
                source,
                "case.trn:8:1",
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
                                "case.trn:8:1",
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
                    "case.trn:9:5",
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
                    "case.trn:10:5",
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
        if self.service_name.as_str() == "reserved" {
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
                    "parse", input.message.clone(), source, "case.trn:12:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "service-config",
                source,
                "case.trn:12:1",
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
                                "case.trn:12:1",
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
                    "case.trn:13:5",
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
                    "case.trn:14:5",
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
                    "case.trn:15:5",
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
                    "case.trn:16:5",
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
                    "case.trn:17:5",
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
                    "case.trn:18:5",
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
                    "case.trn:19:5",
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
                            "case.trn:12:1",
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
                    input.message.clone(), source, "case.trn:26:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "waypoint",
                source,
                "case.trn:26:1",
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
                                "case.trn:26:1",
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
                    "case.trn:27:5",
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
                    "case.trn:28:5",
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
                    input.message.clone(), source, "case.trn:30:1")
                ],
            );
        }
        if terrane_document_support::document_kind(input) != "map" {
            return __terrane_document_type_error(
                input,
                path,
                "route",
                source,
                "case.trn:30:1",
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
                                "case.trn:30:1",
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
                    "case.trn:31:5",
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
                    "case.trn:32:5",
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
                    "case.trn:33:5",
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
    let decoded: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let malformed: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let mut diagnostic: TerraneDocumentDiagnostic;
    let invalid: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let permissive: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let yaml: TerraneDocumentDecodeOutcome<ServiceConfig>;
    let journey: TerraneDocumentDecodeOutcome<Route>;
    decoded = {
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
            "case.trn:36:15",
            "case.trn:36:15",
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
        0 /* terrane-site: case.trn:38:12-38:34 */)), 0 /* terrane-site: case.trn:38:12-38:34 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value.clone()
        .labels.get_or_error(&String::from("tier")), 1 /* terrane-site: case.trn:38:36-38:64 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(decoded.value.clone()
        .coordinates
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:38:66-38:94 */)), 2 /* terrane-site: case.trn:38:66-38:94 */)), terrane_scalar_support::scalar_text(&decoded.value
        .clone().ratio)
    );
    malformed = {
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
            "case.trn:40:17",
            "case.trn:40:17",
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
    let __terrane_iterable_0 = malformed.diagnostics.clone().clone();
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
            terrane_scalar_support::scalar_text(&String::from(":")),
            terrane_scalar_support::scalar_text(&diagnostic.reason.to_owned())
        );
    }
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(malformed
        .diagnostics.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:44:13-44:37 */)), 3 /* terrane-site: case.trn:44:13-44:37 */).source.clone().contains(&String::from("case.trn"))),
        terrane_scalar_support::scalar_text(&__terrane_raised(malformed.diagnostics
        .clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:44:69-44:93 */)), 4 /* terrane-site: case.trn:44:69-44:93 */).field_source.clone()
        .contains(&String::from("case.trn")))
    );
    invalid = {
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
            "case.trn:46:15",
            "case.trn:46:15",
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
        5 /* terrane-site: case.trn:47:28-47:50 */)), 5 /* terrane-site: case.trn:47:28-47:50 */).reason.to_owned()),
        terrane_scalar_support::scalar_text(&__terrane_raised(invalid.diagnostics.clone()
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:47:59-47:81 */)), 6 /* terrane-site: case.trn:47:59-47:81 */).message.clone())
    );
    permissive = {
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
            "case.trn:49:18",
            "case.trn:49:18",
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
    yaml = {
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
            "case.trn:52:12",
            "case.trn:52:12",
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
    journey = {
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
            "case.trn:55:15",
            "case.trn:55:15",
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
        7 /* terrane-site: case.trn:56:48-56:71 */)), 7 /* terrane-site: case.trn:56:48-56:71 */).y), terrane_scalar_support::scalar_text(&journey.value
        .clone().active)
    );
}
