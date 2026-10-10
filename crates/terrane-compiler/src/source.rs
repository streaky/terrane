use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct Span {
    pub file: u32,
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[must_use]
    pub const fn new(file: u32, start: usize, end: usize) -> Self {
        Self { file, start, end }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SourceFile {
    id: u32,
    path: PathBuf,
    text: Arc<str>,
    #[serde(skip)]
    line_starts: Arc<[usize]>,
}
impl<'de> serde::Deserialize<'de> for SourceFile {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Stored {
            id: u32,
            path: PathBuf,
            text: String,
        }
        let stored = <Stored as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(stored.id, stored.path, stored.text))
    }
}

impl SourceFile {
    #[must_use]
    pub fn new(id: u32, path: PathBuf, text: String) -> Self {
        let line_starts = std::iter::once(0)
            .chain(text.match_indices('\n').map(|(offset, _)| offset + 1))
            .collect::<Vec<_>>();
        Self {
            id,
            path,
            text: text.into(),
            line_starts: line_starts.into(),
        }
    }
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
    #[must_use]
    pub fn line_column(&self, offset: usize) -> (usize, usize) {
        let mut offset = offset.min(self.text.len());
        while !self.text.is_char_boundary(offset) {
            offset -= 1;
        }
        let line_index = self
            .line_starts
            .partition_point(|start| *start <= offset)
            .saturating_sub(1);
        let line_start = self.line_starts[line_index];
        (
            line_index + 1,
            self.text[line_start..offset].chars().count() + 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_columns_bound_invalid_offsets_to_source_text() {
        let source = SourceFile::new(1, PathBuf::from("source.trn"), "a🙂\nnext".to_owned());
        assert_eq!(source.line_column(2), (1, 2));
        assert_eq!(source.line_column(usize::MAX), (2, 5));
    }

    #[test]
    fn cached_source_rebuilds_line_index_for_unicode_diagnostics() {
        let source = SourceFile::new(7, PathBuf::from("cached.trn"), "🙂\nαβ\nend".to_owned());
        let encoded = serde_json::to_vec(&source).unwrap();
        let restored: SourceFile = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(restored.line_column(7), (2, 2));
        assert_eq!(restored.line_column(10), (3, 1));
    }
}
