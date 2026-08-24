1|// Generated deterministically by Terrane <version>.
2|#[derive(Clone, Copy, Debug, Eq, PartialEq)]
3|enum TerraneErrorKind {
4|ArithmeticOverflow,
5|DivisionByZero,
6|IntegerConversionOverflow,
7|NegativeShiftCount,
8|CoercionError,
9|DecodeError,
10|IndexError,
11|MissingKey,
12|ResourceError,
13|SourceError,
14|}
15|impl TerraneErrorKind {
16|fn from_source_name(name: &str) -> Self {
17|match name {
18|".arithmetic-overflow" => Self::ArithmeticOverflow,
19|".division-by-zero" => Self::DivisionByZero,
20|".integer-conversion-overflow" => Self::IntegerConversionOverflow,
21|".negative-shift-count" => Self::NegativeShiftCount,
22|".coercion-error" => Self::CoercionError,
23|".decode-error" => Self::DecodeError,
24|".index-error" => Self::IndexError,
25|".missing-key" => Self::MissingKey,
26|".resource-error" => Self::ResourceError,
27|_ => Self::SourceError,
28|}
29|}
30|fn source_name(self) -> &'static str {
31|match self {
32|Self::ArithmeticOverflow => ".arithmetic-overflow",
33|Self::DivisionByZero => ".division-by-zero",
34|Self::IntegerConversionOverflow => ".integer-conversion-overflow",
35|Self::NegativeShiftCount => ".negative-shift-count",
36|Self::CoercionError => ".coercion-error",
37|Self::DecodeError => ".decode-error",
38|Self::IndexError => ".index-error",
39|Self::MissingKey => ".missing-key",
40|Self::ResourceError => ".resource-error",
41|Self::SourceError => ".error",
42|}
43|}
44|}
45|#[derive(Clone, Debug)]
46|struct TerraneError {
47|kind: TerraneErrorKind,
48|message: String,
49|cause: Option<Box<TerraneError>>,
50|context: Vec<&'static str>,
51|}
52|impl TerraneError {
53|fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
54|Self { kind, message: message.into(), cause: None, context: Vec::new() }
55|}
56|#[allow(dead_code)]
57|fn at(mut self, frame: &'static str) -> Self {
58|self.context.push(frame);
59|self
60|}
61|fn render(&self) -> String {
62|let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
63|if let Some(cause) = &self.cause {
64|rendered.push_str("\ncaused by: ");
65|rendered.push_str(&cause.render());
66|}
67|for frame in &self.context {
68|rendered.push_str("\nat ");
69|rendered.push_str(frame);
70|}
71|rendered
72|}
73|}
74|impl std::fmt::Display for TerraneError {
75|fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
76|formatter.write_str(&self.render())
77|}
78|}
79|impl From<terrane_int_support::ArithmeticError> for TerraneError {
80|fn from(error: terrane_int_support::ArithmeticError) -> Self {
81|Self::new(TerraneErrorKind::from_source_name(error.source_name()), error.to_string())
82|}
83|}
84|impl From<terrane_string_support::DecodeError> for TerraneError {
85|fn from(error: terrane_string_support::DecodeError) -> Self {
86|Self::new(TerraneErrorKind::DecodeError, error.to_string().trim_start_matches(".decode-error: "))
87|}
88|}
89|impl From<terrane_collection_support::IndexError> for TerraneError {
90|fn from(error: terrane_collection_support::IndexError) -> Self {
91|Self::new(TerraneErrorKind::IndexError, error.to_string())
92|}
93|}
94|impl From<terrane_collection_support::MissingKey> for TerraneError {
95|fn from(error: terrane_collection_support::MissingKey) -> Self {
96|Self::new(TerraneErrorKind::MissingKey, error.to_string())
97|}
98|}
99|impl From<terrane_collection_support::RangeStepError> for TerraneError {
100|fn from(error: terrane_collection_support::RangeStepError) -> Self {
101|Self::new(TerraneErrorKind::SourceError, error.to_string())
102|}
103|}
104|fn __terrane_uncaught(error: TerraneError) -> ! {
105|eprintln!("{}", error.render());
106|std::process::exit(1);
107|}
108|fn __terrane_generated_defect(message: &str) -> ! {
109|eprintln!("internal compiler defect: generated program reached an impossible completion: {message}");
110|std::process::exit(5);
111|}
112|#[allow(dead_code)]
113|enum TerraneCompletion<T> {
114|Normal,
115|Return(T),
116|Error(TerraneError),
117|Break,
118|Continue,
119|}
120|// Source: case.trn
121|// Namespace: typed-effect-reflection
122|fn fallible() -> Result<terrane_int_support::Int, TerraneError> {
123|    return Ok(terrane_int_support::Int::from(1_i128));
124|}
125|fn main() {
126|    let value: terrane_int_support::Int = (fallible()).unwrap_or_else(|error| __terrane_uncaught(error.at("/typed-effect-reflection::main (case.trn:8:15)")));
127|    println!("{}{}", terrane_scalar_support::scalar_text(&(value)), terrane_scalar_support::scalar_text(&("throws(error)".to_owned())));
128|}
129|
