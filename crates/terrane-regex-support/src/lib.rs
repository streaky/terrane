//! Native implementation of Terrane's text regex operations.

use std::sync::Arc;
use terrane_string_support::TextRange;

#[derive(Clone)]
pub struct Pattern(Arc<regex::Regex>);

pub type Compilation = Result<Pattern, regex::Error>;

#[derive(Clone)]
pub struct Captures(Arc<CaptureData>);

struct CaptureData {
    source: Arc<str>,
    pattern: Pattern,
    locations: regex::CaptureLocations,
}

/// Compiles one Unicode text pattern with the engine's default size limit.
///
/// # Errors
///
/// Returns an error for invalid or unsupported syntax or an oversized compiled pattern.
pub fn compile(pattern: &str, case_insensitive: bool) -> Compilation {
    regex::RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .build()
        .map(|regex| Pattern(Arc::new(regex)))
}

#[must_use]
pub fn compile_error(result: &Compilation) -> Option<String> {
    result.as_ref().err().map(ToString::to_string)
}

/// Extracts the successfully compiled pattern.
///
/// # Panics
///
/// The Terrane wrapper must throw a typed error before calling this on a failure.
#[must_use]
pub fn compiled_pattern(result: Compilation) -> Pattern {
    result.expect("Terrane checked regex compilation")
}

#[must_use]
pub fn matches(pattern: &Pattern, text: &str) -> bool {
    pattern.0.is_match(text)
}

#[must_use]
pub fn first_match(pattern: &Pattern, text: &str) -> Option<TextRange> {
    let matched = pattern.0.find(text)?;
    TextRange::from_shared(Arc::from(text), matched.start(), matched.end())
}

#[must_use]
pub fn captures(pattern: &Pattern, text: &str) -> Option<Captures> {
    let mut locations = pattern.0.capture_locations();
    pattern.0.captures_read(&mut locations, text)?;
    Some(Captures(Arc::new(CaptureData {
        source: Arc::from(text),
        pattern: pattern.clone(),
        locations,
    })))
}

#[must_use]
pub fn capture(captures: &Captures, index: Option<usize>) -> Option<TextRange> {
    let (start, end) = captures.0.locations.get(index?)?;
    TextRange::from_shared(Arc::clone(&captures.0.source), start, end)
}

#[must_use]
pub fn named_capture(captures: &Captures, name: &str) -> Option<TextRange> {
    let index = captures
        .0
        .pattern
        .0
        .capture_names()
        .position(|candidate| candidate == Some(name))?;
    capture(captures, Some(index))
}
