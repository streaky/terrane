//! Native implementation of Terrane's text regex operations.

use std::{ops::Range, sync::Arc};
use terrane_string_support::TextRange;

#[derive(Clone)]
pub struct Pattern(Arc<fancy_regex::Regex>);

pub type Outcome<T> = Result<T, fancy_regex::Error>;

const BACKTRACK_LIMIT: usize = 1_000_000;
const MAX_PROGRAM_SIZE: usize = 100_000;

#[derive(Clone)]
pub struct Captures(Arc<CaptureData>);

struct CaptureData {
    source: Arc<str>,
    pattern: Pattern,
    locations: Vec<Option<Range<usize>>>,
}

/// Compiles one Unicode text pattern, delegating regular portions to automata.
///
/// # Errors
///
/// Returns an error for invalid or unsupported syntax or an oversized compiled pattern.
pub fn compile(pattern: &str, case_insensitive: bool) -> Outcome<Pattern> {
    fancy_regex::RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .backtrack_limit(BACKTRACK_LIMIT)
        .max_prog_size(MAX_PROGRAM_SIZE)
        .delegate_dfa_skip_cache_capacity_check(false)
        .build()
        .map(|regex| Pattern(Arc::new(regex)))
}

#[must_use]
pub fn result_error<T>(result: &Outcome<T>) -> Option<String> {
    result.as_ref().err().map(ToString::to_string)
}

/// Extracts a successful compile or search outcome.
///
/// # Panics
///
/// The Terrane wrapper must throw a typed error before calling this on a failure.
#[must_use]
pub fn result_value<T>(result: Outcome<T>) -> T {
    result.expect("Terrane checked the regex outcome")
}

/// # Errors
///
/// Returns an execution error if the engine cannot finish the search.
pub fn matches(pattern: &Pattern, text: &str) -> Outcome<bool> {
    pattern.0.is_match(text)
}

/// # Errors
///
/// Returns an execution error if the engine cannot finish the search.
pub fn first_match(pattern: &Pattern, text: &str) -> Outcome<Option<TextRange>> {
    Ok(pattern.0.find(text)?.and_then(|matched| {
        TextRange::from_shared(Arc::from(text), matched.start(), matched.end())
    }))
}

/// # Errors
///
/// Returns an execution error if the engine cannot finish the search.
pub fn captures(pattern: &Pattern, text: &str) -> Outcome<Option<Captures>> {
    Ok(pattern.0.captures(text)?.map(|found| {
        Captures(Arc::new(CaptureData {
            source: Arc::from(text),
            pattern: pattern.clone(),
            locations: found
                .iter()
                .map(|matched| matched.map(|matched| matched.range()))
                .collect(),
        }))
    }))
}

#[must_use]
pub fn capture(captures: &Captures, index: Option<usize>) -> Option<TextRange> {
    let range = captures.0.locations.get(index?)?.as_ref()?;
    TextRange::from_shared(Arc::clone(&captures.0.source), range.start, range.end)
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
