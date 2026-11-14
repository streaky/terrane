use icu_casemap::CaseMapper;
use icu_locale_core::langid;
use icu_normalizer::{ComposingNormalizerBorrowed, DecomposingNormalizerBorrowed};
use icu_properties::{
    CodePointSetData,
    props::{Lowercase, Uppercase, WhiteSpace},
};
use icu_segmenter::{GraphemeClusterSegmenter, WordSegmenter, options::WordBreakInvariantOptions};
use std::sync::Arc;
use writeable::Writeable;

/// Unicode Character Database version shared by text segmentation, normalization, and folding.
pub const UNICODE_DATA_VERSION: (u64, u64, u64) = (18, 0, 0);

/// Returns the number of user-perceived characters using Unicode extended
/// grapheme-cluster boundaries.
#[must_use]
pub fn length(value: &str) -> usize {
    grapheme_segmenter()
        .segment_str(value)
        .count()
        .saturating_sub(1)
}

/// Iterates over owned user-perceived characters.
pub fn graphemes(value: &str) -> impl Iterator<Item = String> + '_ {
    grapheme_slices(value).map(str::to_owned)
}

fn grapheme_slices(value: &str) -> impl Iterator<Item = &str> + '_ {
    let segmenter = grapheme_segmenter();
    let mut boundaries = segmenter.segment_str(value);
    let mut start = boundaries.next().unwrap_or(0);
    std::iter::from_fn(move || {
        let end = boundaries.next()?;
        let grapheme = &value[start..end];
        start = end;
        Some(grapheme)
    })
}

fn grapheme_segmenter() -> icu_segmenter::GraphemeClusterSegmenterBorrowed<'static> {
    GraphemeClusterSegmenter::new()
}

fn word_segmenter() -> icu_segmenter::WordSegmenterBorrowed<'static> {
    WordSegmenter::new_for_non_complex_scripts(WordBreakInvariantOptions::default())
}

/// Returns the number of Unicode scalar values.
#[must_use]
pub fn scalar_length(value: &str) -> usize {
    value.chars().count()
}

/// Returns the number of bytes in the UTF-8 encoding.
#[must_use]
pub const fn byte_length(value: &str) -> usize {
    value.len()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Utf32Le,
    Utf32Be,
}
impl Encoding {
    #[must_use]
    pub const fn source_name(self) -> &'static str {
        match self {
            Self::Utf8 => "utf8",
            Self::Utf16Le => "utf16-le",
            Self::Utf16Be => "utf16-be",
            Self::Utf32Le => "utf32-le",
            Self::Utf32Be => "utf32-be",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeError {
    pub encoding: Encoding,
    pub byte_offset: usize,
}
impl std::fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "invalid {} sequence at byte offset {}",
            self.encoding.source_name(),
            self.byte_offset
        )
    }
}

impl std::error::Error for DecodeError {}

#[cfg(test)]
mod tests {
    use super::{
        DecodeError, Encoding, case_fold, graphemes, length, lower, lower_first, normalise, trim,
        trim_end, trim_start, upper, upper_first, upper_words,
    };

    #[test]
    fn decode_error_display_is_message_only() {
        let error = DecodeError {
            encoding: Encoding::Utf8,
            byte_offset: 3,
        };
        assert_eq!(error.to_string(), "invalid utf8 sequence at byte offset 3");
    }

    #[test]
    fn ascii_case_mapping_matches_ascii_tables() {
        let value: String = (0u8..=127).map(char::from).collect();
        assert_eq!(upper(&value), value.to_ascii_uppercase());
        assert_eq!(lower(&value), value.to_ascii_lowercase());
        assert_eq!(case_fold(&value), value.to_ascii_lowercase());
        assert_eq!(upper_first("  hello world Σ"), "  Hello world Σ");
        assert_eq!(lower_first("  HELLO Σ"), "  hELLO Σ");
    }

    #[test]
    fn ascii_whitespace_trimming_includes_vertical_tab() {
        assert_eq!(trim("\t\n\u{b}\u{c}\r text \t\u{b}"), "text");
        assert_eq!(trim_start("\u{b} text", None), "text");
        assert_eq!(trim_end("text \u{b}", None), "text");
        assert_eq!(trim("\u{85}\u{a0}text\u{a0}"), "text");
        assert_eq!(trim("\u{200b}text\u{200b}"), "\u{200b}text\u{200b}");
    }

    #[test]
    fn unicode_case_mapping_keeps_context_and_cased_selection() {
        assert_eq!(lower("ΟΣ"), "ος");
        assert_eq!(lower_first("ΣΑ"), "σΑ");
        assert_eq!(upper_first("ǆemal"), "Ǆemal");
        assert_eq!(upper_first("ßeta"), "SSeta");
        assert_eq!(lower("İ"), "i\u{307}");
        assert_eq!(upper_words("they're here"), "They're Here");
        assert_eq!(
            upper_words("they're here, word-two 123abc"),
            "They're Here, Word-Two 123Abc"
        );
    }

    #[test]
    fn unicode18_data_changes_and_text_algorithms() {
        let new_combining_mark = "A\u{1ae0}";
        assert_eq!(length(new_combining_mark), 1);
        assert_eq!(
            graphemes(new_combining_mark).collect::<Vec<_>>(),
            vec![new_combining_mark.to_owned()]
        );
        let new_compat_ligature = "\u{1df95}";
        assert_eq!(case_fold(new_compat_ligature), "ss");
        assert_eq!(normalise(new_compat_ligature, "nfc"), new_compat_ligature);
        assert_eq!(normalise(new_compat_ligature, "nfkc"), "ß");
        assert_eq!(normalise(new_compat_ligature, "nfkd"), "ß");
        assert_eq!(case_fold("Straße"), "strasse");
        assert_eq!(upper("ß"), "SS");
        assert_eq!(lower("İ"), "i\u{307}");
        assert_eq!(lower("ΟΣ"), "ος");
        assert_eq!(upper_first("ßeta"), "SSeta");
        assert_eq!(lower_first("HELLO"), "hELLO");
        assert_eq!(upper_words("they're here"), "They're Here");
        assert_eq!(trim("\u{a0}text\u{a0}"), "text");
        assert_eq!(trim("\u{200b}text\u{200b}"), "\u{200b}text\u{200b}");
    }
}

#[cfg(test)]
mod replace_tests {
    use super::{replace, replace_broadcast, replace_many};

    #[test]
    fn scalar_replace_preserves_grapheme_empty_pattern_behavior() {
        assert_eq!(replace("e\u{301}x", "", "-"), "-e\u{301}-x-");
    }

    #[test]
    fn multi_replace_uses_original_input_and_list_priority() {
        assert_eq!(
            replace_many("aaa", &["aa".into(), "a".into()], &["X".into(), "Y".into()]).unwrap(),
            "XY"
        );
        assert_eq!(
            replace_many(
                "&<",
                &["&".into(), "<".into()],
                &["&amp;".into(), "&lt;".into()],
            )
            .unwrap(),
            "&amp;&lt;"
        );
        assert_eq!(
            replace_many(
                "&<",
                &["<".into(), "&".into()],
                &["&lt;".into(), "&amp;".into()],
            )
            .unwrap(),
            "&amp;&lt;"
        );
        assert_eq!(
            replace_many(
                "ab",
                &["a".into(), "ab".into()],
                &["first".into(), "second".into()],
            )
            .unwrap(),
            "firstb"
        );
        assert_eq!(
            replace_many("abc", &["b".into()], &["abc".into()]).unwrap(),
            "aabcc"
        );
        assert_eq!(
            replace_broadcast("one two", &["one".into(), "two".into()], "x"),
            "x x"
        );
        assert_eq!(
            replace_many(
                "ab",
                &[String::new(), "a".into()],
                &["-".into(), "X".into()]
            )
            .unwrap(),
            "-a-b-"
        );
        assert_eq!(
            replace_many(
                "ab",
                &["a".into(), String::new()],
                &["X".into(), "-".into()]
            )
            .unwrap(),
            "X-b-"
        );
    }

    #[test]
    fn empty_list_and_empty_pattern_are_defined() {
        assert_eq!(replace_many("text", &[], &[]).unwrap(), "text");
        assert_eq!(
            replace_many("e\u{301}x", &[String::new()], &["-".into()]).unwrap(),
            "-e\u{301}-x-"
        );
        assert_eq!(
            replace_many("x", &["a".into()], &[]).unwrap_err(),
            super::ReplaceError {
                searches: 1,
                replacements: 0,
            }
        );
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextRange {
    source: Arc<str>,
    start: usize,
    end: usize,
}

impl TextRange {
    #[must_use]
    pub fn text(&self) -> &str {
        &self.source[self.start..self.end]
    }

    #[must_use]
    pub fn byte_start(&self) -> usize {
        self.start
    }

    #[must_use]
    pub fn byte_end(&self) -> usize {
        self.end
    }

    #[must_use]
    pub fn scalar_start(&self) -> usize {
        self.source[..self.start].chars().count()
    }

    #[must_use]
    pub fn scalar_end(&self) -> usize {
        self.source[..self.end].chars().count()
    }

    #[must_use]
    pub fn grapheme_start(&self) -> usize {
        grapheme_segmenter()
            .segment_str(&self.source[..self.start])
            .count()
            .saturating_sub(1)
    }

    #[must_use]
    pub fn grapheme_end(&self) -> usize {
        grapheme_segmenter()
            .segment_str(&self.source[..self.end])
            .count()
            .saturating_sub(1)
    }
}

#[must_use]
pub fn encode(value: &str, encoding: Encoding) -> Vec<u8> {
    match encoding {
        Encoding::Utf8 => value.as_bytes().to_vec(),
        Encoding::Utf16Le | Encoding::Utf16Be => value
            .encode_utf16()
            .flat_map(|unit| match encoding {
                Encoding::Utf16Le => unit.to_le_bytes(),
                Encoding::Utf16Be => unit.to_be_bytes(),
                _ => unreachable!(),
            })
            .collect(),
        Encoding::Utf32Le | Encoding::Utf32Be => value
            .chars()
            .flat_map(|scalar| match encoding {
                Encoding::Utf32Le => u32::from(scalar).to_le_bytes(),
                Encoding::Utf32Be => u32::from(scalar).to_be_bytes(),
                _ => unreachable!(),
            })
            .collect(),
    }
}

/// Decodes bytes with the selected canonical encoding.
///
/// # Errors
///
/// Returns the encoding and byte offset of malformed input.
pub fn decode(value: &[u8], encoding: Encoding) -> Result<String, DecodeError> {
    match encoding {
        Encoding::Utf8 => std::str::from_utf8(value)
            .map(str::to_owned)
            .map_err(|error| DecodeError {
                encoding,
                byte_offset: error.valid_up_to(),
            }),
        Encoding::Utf16Le | Encoding::Utf16Be => {
            let (chunks, remainder) = value.as_chunks::<2>();
            if !remainder.is_empty() {
                return Err(DecodeError {
                    encoding,
                    byte_offset: value.len() - 1,
                });
            }
            let units = chunks.iter().map(|chunk| match encoding {
                Encoding::Utf16Le => u16::from_le_bytes([chunk[0], chunk[1]]),
                Encoding::Utf16Be => u16::from_be_bytes([chunk[0], chunk[1]]),
                _ => unreachable!(),
            });
            let mut result = String::new();
            for (index, scalar) in char::decode_utf16(units).enumerate() {
                match scalar {
                    Ok(scalar) => result.push(scalar),
                    Err(_) => {
                        return Err(DecodeError {
                            encoding,
                            byte_offset: index * 2,
                        });
                    }
                }
            }
            Ok(result)
        }
        Encoding::Utf32Le | Encoding::Utf32Be => {
            let (chunks, remainder) = value.as_chunks::<4>();
            if !remainder.is_empty() {
                return Err(DecodeError {
                    encoding,
                    byte_offset: value.len() - remainder.len(),
                });
            }
            let mut result = String::new();
            for (index, chunk) in chunks.iter().enumerate() {
                let scalar = match encoding {
                    Encoding::Utf32Le => {
                        u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                    }
                    Encoding::Utf32Be => {
                        u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                    }
                    _ => unreachable!(),
                };
                let Some(scalar) = char::from_u32(scalar) else {
                    return Err(DecodeError {
                        encoding,
                        byte_offset: index * 4,
                    });
                };
                result.push(scalar);
            }
            Ok(result)
        }
    }
}

#[must_use]
pub fn trim(value: &str) -> String {
    value.trim_matches(is_whitespace).to_owned()
}

#[must_use]
pub fn trim_start(value: &str, pattern: Option<&str>) -> String {
    pattern.map_or_else(
        || value.trim_start_matches(is_whitespace).to_owned(),
        |pattern| value.strip_prefix(pattern).unwrap_or(value).to_owned(),
    )
}

#[must_use]
pub fn trim_end(value: &str, pattern: Option<&str>) -> String {
    pattern.map_or_else(
        || value.trim_end_matches(is_whitespace).to_owned(),
        |pattern| value.strip_suffix(pattern).unwrap_or(value).to_owned(),
    )
}

fn is_whitespace(character: char) -> bool {
    if character.is_ascii() {
        matches!(character, '\t'..='\r' | ' ')
    } else {
        CodePointSetData::new::<WhiteSpace>().contains(character)
    }
}

#[must_use]
pub fn find(value: &str, pattern: &str) -> Option<TextRange> {
    let source = Arc::<str>::from(value);
    source.find(pattern).map(|start| TextRange {
        source,
        start,
        end: start + pattern.len(),
    })
}

#[must_use]
pub fn find_all(value: &str, pattern: &str) -> Vec<TextRange> {
    let source = Arc::<str>::from(value);
    if pattern.is_empty() {
        return grapheme_segmenter()
            .segment_str(&source)
            .map(|start| TextRange {
                source: Arc::clone(&source),
                start,
                end: start,
            })
            .collect();
    }
    source
        .match_indices(pattern)
        .map(|(start, matched)| TextRange {
            source: Arc::clone(&source),
            start,
            end: start + matched.len(),
        })
        .collect()
}

#[must_use]
pub fn upper(value: &str) -> String {
    if value.is_ascii() {
        value.to_ascii_uppercase()
    } else {
        CaseMapper::new()
            .uppercase_to_string(value, &langid!("und"))
            .into_owned()
    }
}

#[must_use]
pub fn lower(value: &str) -> String {
    if value.is_ascii() {
        value.to_ascii_lowercase()
    } else {
        CaseMapper::new()
            .lowercase_to_string(value, &langid!("und"))
            .into_owned()
    }
}

#[must_use]
pub fn upper_first(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    write_first_cased(value, &mut result, true);
    result
}

#[must_use]
pub fn upper_words(value: &str) -> String {
    let segmenter = word_segmenter();
    let mut boundaries = segmenter.segment_str(value);
    let mut start = boundaries.next().unwrap_or(0);
    let mut result = String::with_capacity(value.len());
    for end in boundaries {
        write_first_cased(&value[start..end], &mut result, true);
        start = end;
    }
    result
}

#[must_use]
pub fn lower_first(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    write_first_cased(value, &mut result, false);
    result
}

fn write_first_cased(value: &str, result: &mut String, uppercase: bool) {
    let Some((index, character)) = value.char_indices().find(|(_, character)| {
        if character.is_ascii() {
            character.is_ascii_alphabetic()
        } else {
            let lowercase = CodePointSetData::new::<Lowercase>();
            let uppercase_set = CodePointSetData::new::<Uppercase>();
            lowercase.contains(*character) || uppercase_set.contains(*character)
        }
    }) else {
        result.push_str(value);
        return;
    };
    let end = index + character.len_utf8();
    result.push_str(&value[..index]);
    if character.is_ascii() {
        result.push(if uppercase {
            character.to_ascii_uppercase()
        } else {
            character.to_ascii_lowercase()
        });
    } else {
        let case_mapper = CaseMapper::new();
        if uppercase {
            case_mapper
                .uppercase(&value[index..end], &langid!("und"))
                .write_to(result)
                .expect("writing case mapping to String is infallible");
        } else {
            case_mapper
                .lowercase(&value[index..end], &langid!("und"))
                .write_to(result)
                .expect("writing case mapping to String is infallible");
        }
    }
    result.push_str(&value[end..]);
}

#[must_use]
pub fn case_fold(value: &str) -> String {
    if value.is_ascii() {
        value.to_ascii_lowercase()
    } else {
        CaseMapper::new().fold_string(value).into_owned()
    }
}

#[must_use]
pub fn normalise(value: &str, form: &str) -> String {
    match form {
        "nfc" => ComposingNormalizerBorrowed::new_nfc()
            .normalize(value)
            .into_owned(),
        "nfd" => DecomposingNormalizerBorrowed::new_nfd()
            .normalize(value)
            .into_owned(),
        "nfkc" => ComposingNormalizerBorrowed::new_nfkc()
            .normalize(value)
            .into_owned(),
        "nfkd" => DecomposingNormalizerBorrowed::new_nfkd()
            .normalize(value)
            .into_owned(),
        _ => unreachable!("compiler validates normalization forms"),
    }
}

#[must_use]
pub fn split(value: &str, pattern: &str) -> Vec<String> {
    if pattern.is_empty() {
        return grapheme_slices(value).map(str::to_owned).collect();
    }
    value.split(pattern).map(str::to_owned).collect()
}

#[must_use]
pub fn replace(value: &str, pattern: &str, replacement: &str) -> String {
    if pattern.is_empty() {
        let mut result =
            String::with_capacity(value.len() + replacement.len() * (length(value) + 1));
        result.push_str(replacement);
        for grapheme in grapheme_slices(value) {
            result.push_str(grapheme);
            result.push_str(replacement);
        }
        return result;
    }
    value.replace(pattern, replacement)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaceError {
    pub searches: usize,
    pub replacements: usize,
}

impl std::fmt::Display for ReplaceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "replace received {} searches and {} replacements",
            self.searches, self.replacements
        )
    }
}

impl std::error::Error for ReplaceError {}

/// Replaces original-input matches with list-priority, left-to-right semantics.
///
/// Empty patterns compete in list order at original grapheme boundaries. An
/// empty winner inserts its replacement and advances one original scalar
/// unchanged. Matching never examines replacement text.
///
/// # Errors
///
/// Returns [`ReplaceError`] when the two lists have different lengths.
pub fn replace_many(
    value: &str,
    patterns: &[String],
    replacements: &[String],
) -> Result<String, ReplaceError> {
    if patterns.len() != replacements.len() {
        return Err(ReplaceError {
            searches: patterns.len(),
            replacements: replacements.len(),
        });
    }
    Ok(replace_patterns(value, patterns, |index| {
        replacements[index].as_str()
    }))
}

/// Replaces each listed search with the same replacement string.
#[must_use]
pub fn replace_broadcast(value: &str, patterns: &[String], replacement: &str) -> String {
    replace_patterns(value, patterns, |_| replacement)
}

fn replace_patterns<'a>(
    value: &str,
    patterns: &[String],
    replacement: impl Fn(usize) -> &'a str,
) -> String {
    if patterns.is_empty() {
        return value.to_owned();
    }
    // Nonempty patterns match at scalar boundaries; only empty patterns need
    // grapheme segmentation. No replacement text enters this scan.
    let segmenter = patterns
        .iter()
        .any(String::is_empty)
        .then(grapheme_segmenter);
    let mut boundaries = segmenter
        .as_ref()
        .map(|segmenter| segmenter.segment_str(value));
    let mut boundary = boundaries.as_mut().and_then(Iterator::next);
    let mut output = String::with_capacity(value.len());
    let mut cursor = 0;
    let mut unchanged = 0;
    loop {
        while boundary.is_some_and(|boundary| boundary < cursor) {
            boundary = boundaries.as_mut().and_then(Iterator::next);
        }
        let remaining = &value[cursor..];
        if let Some((index, pattern)) = patterns.iter().enumerate().find(|(_, pattern)| {
            if pattern.is_empty() {
                boundary == Some(cursor)
            } else {
                remaining.starts_with(pattern.as_str())
            }
        }) {
            output.push_str(&value[unchanged..cursor]);
            output.push_str(replacement(index));
            unchanged = cursor;
            if !pattern.is_empty() {
                cursor += pattern.len();
                unchanged = cursor;
                continue;
            }
            // An empty match wins this position but cannot consume input.
            // Copy one original scalar to advance without trying another
            // pattern at the same position or skipping inside-grapheme matches.
            if let Some(character) = remaining.chars().next() {
                let end = cursor + character.len_utf8();
                output.push_str(&value[cursor..end]);
                cursor = end;
                unchanged = cursor;
                continue;
            }
            break;
        }
        if let Some(character) = remaining.chars().next() {
            cursor += character.len_utf8();
        } else {
            break;
        }
    }
    output.push_str(&value[unchanged..]);
    output
}
