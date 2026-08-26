// Generated deterministically by Terrane <version>.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerraneErrorKind {
    ArithmeticOverflow,
    DivisionByZero,
    IntegerConversionOverflow,
    NegativeShiftCount,
    CoercionError,
    DecodeError,
    IndexError,
    MissingKey,
    ResourceError,
    SourceError,
}
impl TerraneErrorKind {
    fn from_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string().trim_start_matches(".decode-error: "),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
}
fn __terrane_uncaught(error: TerraneError) -> ! {
    eprintln!("{}", error.render());
    std::process::exit(1);
}
fn __terrane_generated_defect(message: &str) -> ! {
    eprintln!(
        "internal compiler defect: generated program reached an impossible completion: {message}"
    );
    std::process::exit(5);
}
#[allow(dead_code)]
enum TerraneCompletion<T> {
    Normal,
    Return(T),
    Error(TerraneError),
    Break,
    Continue,
}
// Source: case.trn
// Namespace: conformance/lexical-paths
fn main() {
    let relative: Path = Path::terrane_construct(
        String::from("alpha/./beta/../gamma/../../delta"),
    );
    let rooted: Path = Path::terrane_construct(
        String::from("/alpha/../../beta/file.tar.gz"),
    );
    let base: Path = Path::terrane_construct(String::from("work/root"));
    let child: Path = Path::terrane_construct(String::from("../next"));
    let relative_normal: Path = normalise_path(relative.clone());
    let rooted_normal: Path = normalise_path(rooted.clone());
    let relative_text: String = relative_normal.text.clone();
    let rooted_text: String = rooted_normal.text.clone();
    let rooted_name: String = path_name(rooted_normal.clone());
    let rooted_stem: String = path_stem(rooted_normal.clone());
    let rooted_extension: String = path_extension(rooted_normal.clone());
    let rooted_parent: Path = path_parent(rooted_normal.clone());
    let rooted_parent_text: String = rooted_parent.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&relative_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_text));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_name));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_stem));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_extension));
    println!("{}", terrane_scalar_support::scalar_text(&rooted_parent_text));
    let resolved: Path = join_path(base.clone(), child.clone());
    let resolved_text: String = resolved.text.clone();
    println!("{}", terrane_scalar_support::scalar_text(&resolved_text));
}
// Source: standard/paths.trn
// Namespace: standard/paths
#[derive(Clone)]
pub struct Path {
    pub text: String,
}
impl Path {
    pub fn terrane_construct(input: String) -> Self {
        let mut value = Self { text: String::from("") };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: String) {
        self.text = input;
    }
}
pub fn path_components(subject: Path) -> terrane_collection_support::List<String> {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let mut result: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        let part: String = parts
            .get(
                terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::path-components (paths.trn:17:16)"),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::path-components (paths.trn:17:16)"),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/paths::path-components (paths.trn:17:16)"),
            ));
        if part != String::from("") {
            result.append(part);
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result.clone();
}
pub fn path_is_absolute(subject: Path) -> bool {
    return subject.text.starts_with(&String::from("/"));
}
pub fn normalise_path(subject: Path) -> Path {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let absolute: bool = path_is_absolute(subject.clone());
    let mut kept: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    let mut part_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while part_index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        let part: String = parts
            .get(
                terrane_collection_support::index_from_int(&part_index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::normalise-path (paths.trn:33:16)"),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(&part_index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::normalise-path (paths.trn:33:16)"),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/paths::normalise-path (paths.trn:33:16)"),
            ));
        if part != String::from("") && part != String::from(".") {
            if part == String::from("..") {
                if count.clone() > terrane_int_support::Int::from(0_i128)
                    && kept
                        .get_or_error(
                            terrane_collection_support::index_from_int(
                                    &(count.clone() - terrane_int_support::Int::from(1_i128)),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/paths::normalise-path (paths.trn:36:34)"),
                                )),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/paths::normalise-path (paths.trn:36:34)"),
                        )) != String::from("..")
                {
                    count = count.clone() - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute {
                        if count.clone()
                            < terrane_int_support::Int::from(
                                terrane_int_support::Int::from(kept.length()),
                            )
                        {
                            kept.set(
                                    terrane_collection_support::index_from_int(&count.clone())
                                        .unwrap_or_else(|error| __terrane_uncaught(
                                            TerraneError::from(error)
                                                .at("/standard/paths::normalise-path (paths.trn:41:29)"),
                                        )),
                                    part,
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/paths::normalise-path (paths.trn:41:29)"),
                                ));
                        } else {
                            kept.append(part);
                        }
                        count = count.clone() + terrane_int_support::Int::from(1_i128);
                    }
                }
            } else {
                if count.clone()
                    < terrane_int_support::Int::from(
                        terrane_int_support::Int::from(kept.length()),
                    )
                {
                    kept.set(
                            terrane_collection_support::index_from_int(&count.clone())
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/paths::normalise-path (paths.trn:47:21)"),
                                )),
                            part,
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/paths::normalise-path (paths.trn:47:21)"),
                        ));
                } else {
                    kept.append(part);
                }
                count = count.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
        part_index = part_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < count.clone() {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&kept
            .get_or_error(terrane_collection_support::index_from_int(&index.clone())
            .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::normalise-path (paths.trn:57:33)")))).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::normalise-path (paths.trn:57:33)"))))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    if result == String::from("") && absolute {
        result = String::from("/");
    }
    return Path::terrane_construct(result);
}
pub fn path_name(subject: Path) -> String {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(
        normal.clone(),
    );
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return String::from("");
    }
    return parts
        .get_or_error(
            terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(
                        terrane_int_support::Int::from(parts.length()),
                    ) - terrane_int_support::Int::from(1_i128)),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/paths::path-name (paths.trn:70:12)"),
                )),
        )
        .unwrap_or_else(|error| __terrane_uncaught(
            TerraneError::from(error).at("/standard/paths::path-name (paths.trn:70:12)"),
        ));
}
pub fn path_parent(subject: Path) -> Path {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(
        normal.clone(),
    );
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return normal.clone();
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
            - terrane_int_support::Int::from(1_i128)
    {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&parts
            .get_or_error(terrane_collection_support::index_from_int(&index.clone())
            .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-parent (paths.trn:82:33)")))).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-parent (paths.trn:82:33)"))))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let absolute: bool = path_is_absolute(normal.clone());
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    return Path::terrane_construct(result);
}
pub fn path_stem(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return current;
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(pieces.len() as i128)
            - terrane_int_support::Int::from(1_i128)
    {
        if index.clone() > terrane_int_support::Int::from(0_i128) {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("."))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&pieces
            .get(terrane_collection_support::index_from_int(&index.clone())
            .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-stem (paths.trn:99:33)")))).cloned()
            .ok_or(terrane_collection_support::IndexError { index :
            terrane_collection_support::index_from_int(&index.clone()).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-stem (paths.trn:99:33)"))) }).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-stem (paths.trn:99:33)"))))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result;
}
pub fn path_extension(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return String::from("");
    }
    return pieces
        .get(
            terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(pieces.len() as i128)
                        - terrane_int_support::Int::from(1_i128)),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/paths::path-extension (paths.trn:108:12)"),
                )),
        )
        .cloned()
        .ok_or(terrane_collection_support::IndexError {
            index: terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(pieces.len() as i128)
                        - terrane_int_support::Int::from(1_i128)),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/paths::path-extension (paths.trn:108:12)"),
                )),
        })
        .unwrap_or_else(|error| __terrane_uncaught(
            TerraneError::from(error)
                .at("/standard/paths::path-extension (paths.trn:108:12)"),
        ));
}
pub fn join_path(base: Path, child: Path) -> Path {
    let absolute: bool = path_is_absolute(child.clone());
    if absolute {
        return normalise_path(child.clone());
    }
    let mut joined: String = base.text.clone();
    if joined != String::from("") && !joined.ends_with(&String::from("/")) {
        joined = format!(
            "{}{}", terrane_scalar_support::scalar_text(&joined),
            terrane_scalar_support::scalar_text(&String::from("/"))
        );
    }
    joined = format!(
        "{}{}", terrane_scalar_support::scalar_text(&joined),
        terrane_scalar_support::scalar_text(&child.text)
    );
    let combined: Path = Path::terrane_construct(joined);
    return normalise_path(combined.clone());
}
