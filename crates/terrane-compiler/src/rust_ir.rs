use std::fmt::Write as _;
use syn::fold::Fold as _;
use syn::parse::Parser as _;

use crate::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub version: &'static str,
    pub requires_platform_support: bool,
    pub runtime: Vec<GeneratedModule>,
    pub globals: Vec<Item>,
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedFile {
    pub path: String,
    pub contents: String,
    pub associations: Vec<SourceAssociation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RenderedProgram {
    version: &'static str,
    support: RenderedFragment,
    standalone: RenderedFragment,
    application: RenderedFragment,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenderedFragment {
    contents: String,
    associations: Vec<SourceAssociation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedModule {
    pub name: &'static str,
    pub items: Vec<Item>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceAssociation {
    pub generated_start: usize,
    pub generated_end: usize,
    pub source: Span,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModuleDestination {
    Support,
    Application,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub source_path: String,
    pub namespace: String,
    pub destination: ModuleDestination,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub source: Option<Span>,
    pub body: Block,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    parsed: syn::File,
}

struct CanonicalizeExpressions;

fn compact_borrow_tokens(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    tokens
        .into_iter()
        .map(|token| match token {
            proc_macro2::TokenTree::Group(group) => {
                let mut compact = proc_macro2::Group::new(
                    group.delimiter(),
                    compact_borrow_tokens(group.stream()),
                );
                compact.set_span(group.span());
                proc_macro2::TokenTree::Group(compact)
            }
            proc_macro2::TokenTree::Punct(punctuation) if punctuation.as_char() == '&' => {
                let mut compact = proc_macro2::Punct::new('&', proc_macro2::Spacing::Joint);
                compact.set_span(punctuation.span());
                proc_macro2::TokenTree::Punct(compact)
            }
            token => token,
        })
        .collect()
}

impl syn::fold::Fold for CanonicalizeExpressions {
    fn fold_expr(&mut self, expression: syn::Expr) -> syn::Expr {
        match syn::fold::fold_expr(self, expression) {
            syn::Expr::Paren(parenthesized) if parenthesized.attrs.is_empty() => {
                *parenthesized.expr
            }
            expression => expression,
        }
    }

    fn fold_macro(&mut self, expression_macro: syn::Macro) -> syn::Macro {
        let mut expression_macro = syn::fold::fold_macro(self, expression_macro);
        let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
        if let Ok(arguments) = parser.parse2(expression_macro.tokens.clone()) {
            let mut normalized = arguments.clone();
            for argument in &mut normalized {
                *argument = self.fold_expr(argument.clone());
            }
            expression_macro.tokens = if normalized == arguments {
                compact_borrow_tokens(expression_macro.tokens)
            } else {
                compact_borrow_tokens(quote::quote!(#normalized))
            };
        }
        expression_macro
    }
}

fn canonicalize_file(parsed: syn::File) -> syn::File {
    CanonicalizeExpressions.fold_file(parsed)
}

pub(crate) fn canonicalize_rust(rust: &str) -> Result<String, syn::Error> {
    let module_comment_marker = module_comment_marker(rust);
    let encoded = encode_terrane_module_comments(
        &encode_terrane_site_rows(&encode_terrane_comments(rust)),
        &module_comment_marker,
    );
    let parsed = syn::parse_file(&encoded)?;
    Ok(restore_terrane_metadata(
        &prettyplease::unparse(&canonicalize_file(parsed)),
        Some(&module_comment_marker),
    ))
}

fn encode_terrane_comments(rendered: &str) -> String {
    const MARKER: &str = " /* terrane-site: ";
    let mut encoded = String::with_capacity(rendered.len());
    let mut remaining = rendered;
    while let Some(comment_start) = remaining.find(MARKER) {
        let Some(comment_end) = remaining[comment_start + MARKER.len()..].find(" */") else {
            break;
        };
        let comment_end = comment_start + MARKER.len() + comment_end;
        let expression_end = remaining[..comment_start].trim_end().len();
        let expression_start = remaining[..expression_end]
            .char_indices()
            .rev()
            .find(|(_, character)| !character.is_ascii_digit())
            .map_or(0, |(index, character)| index + character.len_utf8());
        if expression_start == expression_end {
            encoded.push_str(&remaining[..comment_end + 3]);
            remaining = &remaining[comment_end + 3..];
            continue;
        }
        encoded.push_str(&remaining[..expression_start]);
        let expression = &remaining[expression_start..expression_end];
        let comment = &remaining[comment_start + MARKER.len()..comment_end];
        write!(encoded, "__terrane_comment!({expression}, {comment:?})")
            .expect("writing to a String cannot fail");
        remaining = &remaining[comment_end + 3..];
    }
    encoded.push_str(remaining);
    encoded
}

fn encode_terrane_site_rows(rendered: &str) -> String {
    const MARKER: &str = "/* terrane-site-row: ";
    let mut encoded = String::with_capacity(rendered.len());
    let mut remaining = rendered;
    while let Some(comment_start) = remaining.find(MARKER) {
        let Some(comment_end) = remaining[comment_start + MARKER.len()..].find(" */") else {
            break;
        };
        let comment_end = comment_start + MARKER.len() + comment_end;
        encoded.push_str(&remaining[..comment_start]);
        let comment = &remaining[comment_start + MARKER.len()..comment_end];
        write!(encoded, "__terrane_site_comment!({comment:?});")
            .expect("writing to a String cannot fail");
        remaining = &remaining[comment_end + 3..];
    }
    encoded.push_str(remaining);
    encoded
}

fn module_comment_marker(rendered: &str) -> String {
    let mut marker = "__terrane_generated_module_comment".to_owned();
    while rendered.contains(&marker) {
        marker.push('_');
    }
    marker
}

fn encode_terrane_module_comments(rendered: &str, marker: &str) -> String {
    let mut encoded = String::with_capacity(rendered.len());
    for line in rendered.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.starts_with("// Source: ") || content.starts_with("// Namespace: ") {
            writeln!(encoded, "{marker}!({content:?});").expect("writing to a String cannot fail");
        } else {
            encoded.push_str(line);
        }
    }
    encoded
}

fn restore_terrane_comments(rendered: &str) -> String {
    const MARKER: &str = "__terrane_comment!(";
    let mut restored = String::with_capacity(rendered.len());
    let mut remaining = rendered;
    while let Some(start) = remaining.find(MARKER) {
        restored.push_str(&remaining[..start]);
        let arguments_start = start + MARKER.len();
        let bytes = remaining.as_bytes();
        let mut depth = 1_usize;
        let mut in_string = false;
        let mut escaped = false;
        let mut separator = None;
        let mut end = None;
        for (offset, byte) in bytes[arguments_start..].iter().copied().enumerate() {
            if in_string {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    in_string = false;
                }
                continue;
            }
            match byte {
                b'"' => in_string = true,
                b'(' | b'[' | b'{' => depth += 1,
                b')' if depth == 1 => {
                    end = Some(arguments_start + offset);
                    break;
                }
                b')' | b']' | b'}' => depth -= 1,
                b',' if depth == 1 && separator.is_none() => {
                    separator = Some(arguments_start + offset);
                }
                _ => {}
            }
        }
        let (Some(separator), Some(end)) = (separator, end) else {
            restored.push_str(&remaining[start..]);
            return restored;
        };
        let expression = remaining[arguments_start..separator].trim_end();
        let encoded_comment = remaining[separator + 1..end]
            .trim()
            .strip_suffix(',')
            .unwrap_or_else(|| remaining[separator + 1..end].trim())
            .trim_end();
        let Ok(comment) = syn::parse_str::<syn::LitStr>(encoded_comment) else {
            restored.push_str(&remaining[start..=end]);
            remaining = &remaining[end + 1..];
            continue;
        };
        let comment = comment.value().replace("*/", "* /");
        write!(restored, "{expression} /* terrane-site: {comment} */")
            .expect("writing to a String cannot fail");
        remaining = &remaining[end + 1..];
    }
    restored.push_str(remaining);
    restored
}

fn encoded_literal_macro(
    rendered: &str,
    start: usize,
    marker: &str,
) -> Option<(usize, syn::LitStr)> {
    let argument_start = start + marker.len();
    let bytes = rendered.as_bytes();
    let mut depth = 1_usize;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, byte) in bytes[argument_start..].iter().copied().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'(' | b'[' | b'{' => depth += 1,
            b')' if depth == 1 => {
                let end = argument_start + offset;
                let literal = syn::parse_str(rendered[argument_start..end].trim()).ok()?;
                let consumed = end + 1 + usize::from(bytes.get(end + 1) == Some(&b';'));
                return Some((consumed, literal));
            }
            b']' | b'}' if depth == 1 => return None,
            b')' | b']' | b'}' => depth -= 1,
            _ => {}
        }
    }
    None
}

fn restore_terrane_site_rows(rendered: &str) -> String {
    const MARKER: &str = "__terrane_site_comment!(";
    let mut restored = String::with_capacity(rendered.len());
    let mut remaining = rendered;
    while let Some(start) = remaining.find(MARKER) {
        restored.push_str(&remaining[..start]);
        let Some((end, comment)) = encoded_literal_macro(remaining, start, MARKER) else {
            restored.push_str(&remaining[start..]);
            return restored;
        };
        write!(
            restored,
            "/* terrane-site-row: {} */",
            comment.value().replace("*/", "* /")
        )
        .expect("writing to a String cannot fail");
        remaining = &remaining[end..];
    }
    restored.push_str(remaining);
    restored
}

fn restore_terrane_module_comments(rendered: &str, marker: &str) -> String {
    let marker = format!("{marker}!(");
    let mut restored = String::with_capacity(rendered.len());
    let mut remaining = rendered;
    while let Some(start) = remaining.find(&marker) {
        restored.push_str(&remaining[..start]);
        let Some((end, comment)) = encoded_literal_macro(remaining, start, &marker) else {
            restored.push_str(&remaining[start..]);
            return restored;
        };
        restored.push_str(&comment.value());
        remaining = &remaining[end..];
    }
    restored.push_str(remaining);
    restored
}

fn restore_terrane_metadata(rendered: &str, module_comment_marker: Option<&str>) -> String {
    let restored = restore_terrane_site_rows(&restore_terrane_comments(rendered));
    let restored = match module_comment_marker {
        Some(marker) => restore_terrane_module_comments(&restored, marker),
        None => restored,
    };
    let mut normalized = String::with_capacity(restored.len());
    for line in restored.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.trim().is_empty() {
            if line.ends_with('\n') {
                normalized.push('\n');
            }
        } else {
            normalized.push_str(line);
        }
    }
    normalized
}

impl Block {
    fn from_rendered(rust: &str) -> Self {
        let parsed = syn::parse_file(rust).expect("lowered Rust item must parse");
        Self {
            parsed: canonicalize_file(parsed),
        }
    }

    fn render(&self, output: &mut String) {
        output.push_str(&restore_terrane_metadata(
            &prettyplease::unparse(&self.parsed),
            None,
        ));
    }
}

impl Item {
    #[must_use]
    pub fn generated(rust: &str) -> Self {
        Self {
            source: None,
            body: Block::from_rendered(rust),
        }
    }

    #[must_use]
    pub fn sourced(source: Span, rust: &str) -> Self {
        Self {
            source: Some(source),
            body: Block::from_rendered(rust),
        }
    }

    fn render(&self, output: &mut String) {
        self.body.render(output);
    }

    fn render_associated(&self, output: &mut String, associations: &mut Vec<SourceAssociation>) {
        let generated_start = output.len();
        self.render(output);
        if let Some(source) = self.source {
            associations.push(SourceAssociation {
                generated_start,
                generated_end: output.len(),
                source,
            });
        }
    }
}

impl RenderedProgram {
    pub(crate) fn standalone_file(&self, path: &str) -> RenderedFile {
        let mut contents = format!(
            "// Generated deterministically by Terrane {}.\n",
            self.version
        );
        let offset = contents.len();
        contents.push_str(&self.standalone.contents);
        RenderedFile {
            path: path.to_owned(),
            contents,
            associations: self
                .standalone
                .associations
                .iter()
                .map(|association| SourceAssociation {
                    generated_start: association.generated_start + offset,
                    generated_end: association.generated_end + offset,
                    source: association.source,
                })
                .collect(),
        }
    }

    pub(crate) fn files(&self, entrypoint: &std::path::Path) -> Result<Vec<RenderedFile>, String> {
        let Some(file_stem) = entrypoint.file_stem() else {
            return Err("generated Rust output path has no file name".to_owned());
        };
        let Some(stem) = file_stem.to_str() else {
            return Err("generated Rust output file name must be valid UTF-8".to_owned());
        };
        let support_name = format!("{stem}.support.rs");
        let support_path = entrypoint.with_file_name(&support_name);
        let Some(support_path) = support_path.to_str() else {
            return Err("generated Rust output path must be valid UTF-8".to_owned());
        };
        let Some(entrypoint) = entrypoint.to_str() else {
            return Err("generated Rust output path must be valid UTF-8".to_owned());
        };
        Ok(self.files_with_paths(entrypoint, support_path, &support_name))
    }

    fn files_with_paths(
        &self,
        entrypoint: &str,
        support_path: &str,
        support_name: &str,
    ) -> Vec<RenderedFile> {
        let mut application = format!(
            "// Generated deterministically by Terrane {}.\ninclude!(\"{support_name}\");\n",
            self.version
        );
        let application_offset = application.len();
        application.push_str(&self.application.contents);
        let application_associations = self
            .application
            .associations
            .iter()
            .map(|association| SourceAssociation {
                generated_start: association.generated_start + application_offset,
                generated_end: association.generated_end + application_offset,
                source: association.source,
            })
            .collect();
        vec![
            RenderedFile {
                path: support_path.to_owned(),
                contents: self.support.contents.clone(),
                associations: self.support.associations.clone(),
            },
            RenderedFile {
                path: entrypoint.to_owned(),
                contents: application,
                associations: application_associations,
            },
        ]
    }
}

impl Program {
    #[must_use]
    pub(crate) fn rendered(&self) -> RenderedProgram {
        fn render_modules<'a>(
            modules: impl IntoIterator<Item = &'a Module>,
            output: &mut String,
            associations: &mut Vec<SourceAssociation>,
        ) {
            for module in modules {
                if module.items.is_empty() {
                    continue;
                }
                write!(
                    output,
                    "// Source: {}\n// Namespace: {}\n",
                    module.source_path,
                    module.namespace.trim_start_matches('/')
                )
                .expect("writing to a String cannot fail");
                for item in &module.items {
                    item.render_associated(output, associations);
                }
            }
        }

        let mut support = String::new();
        let mut support_associations = Vec::new();
        for module in &self.runtime {
            for item in &module.items {
                item.render_associated(&mut support, &mut support_associations);
            }
        }
        for item in &self.globals {
            item.render_associated(&mut support, &mut support_associations);
        }
        let mut standalone = support.clone();
        let mut standalone_associations = support_associations.clone();
        render_modules(&self.modules, &mut standalone, &mut standalone_associations);

        let mut application = String::new();
        let mut application_associations = Vec::new();
        render_modules(
            self.modules
                .iter()
                .filter(|module| module.destination == ModuleDestination::Support),
            &mut support,
            &mut support_associations,
        );
        render_modules(
            self.modules
                .iter()
                .filter(|module| module.destination == ModuleDestination::Application),
            &mut application,
            &mut application_associations,
        );
        RenderedProgram {
            version: self.version,
            standalone: RenderedFragment {
                contents: standalone,
                associations: standalone_associations,
            },
            support: RenderedFragment {
                contents: support,
                associations: support_associations,
            },
            application: RenderedFragment {
                contents: application,
                associations: application_associations,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Block, canonicalize_rust, encode_terrane_comments, restore_terrane_comments,
        restore_terrane_metadata, restore_terrane_module_comments, restore_terrane_site_rows,
    };

    #[test]
    fn canonicalizes_expression_list_macro_arguments() {
        let block = Block::from_rendered(
            r#"fn main() {
                println!(
                    "{}{}",
                    scalar_text(&((outcome).completed)),
                    scalar_text(&((outcome).cancelled)),
                );
            }"#,
        );
        let mut rendered = String::new();
        block.render(&mut rendered);

        assert!(
            rendered.contains("scalar_text(&outcome.completed),"),
            "{rendered}"
        );
        assert!(
            rendered.contains("scalar_text(&outcome.cancelled),"),
            "{rendered}"
        );
        assert!(!rendered.contains("&(("), "{rendered}");
    }

    #[test]
    fn restores_generated_site_comments_after_canonicalization() {
        let block = Block::from_rendered(
            r#"fn main() {
                let value = raised(call(), __terrane_comment!(7, "src/main.trn:4:9-4:15"));
                let sites = [{
                    __terrane_site_comment!("site 7 /demo::main");
                    Site {
                        function: 0,
                        file: 0,
                        line: 4,
                        column: 9,
                        end_line: 4,
                        end_column: 15,
                    }
                }];
            }"#,
        );
        let mut rendered = String::new();
        block.render(&mut rendered);

        assert!(
            rendered.contains("7 /* terrane-site: src/main.trn:4:9-4:15 */"),
            "{rendered}"
        );
        assert!(
            rendered.contains("/* terrane-site-row: site 7 /demo::main */\n            Site {\n"),
            "{rendered}"
        );
        assert!(!rendered.contains("__terrane_comment"), "{rendered}");
        assert_eq!(canonicalize_rust(&rendered).unwrap(), rendered);
    }

    #[test]
    fn clears_whitespace_from_otherwise_blank_restored_lines() {
        assert_eq!(
            restore_terrane_metadata("first\n    \n\t\nlast\n   ", None),
            "first\n\n\nlast\n"
        );
    }

    #[test]
    fn encodes_site_comments_after_unicode_without_slicing_mid_character() {
        assert_eq!(
            encode_terrane_comments("é7 /* terrane-site: source */"),
            "é__terrane_comment!(7, \"source\")"
        );
    }

    #[test]
    fn module_comment_codec_does_not_capture_authored_marker_like_macros() {
        let rendered = "__terrane_generated_module_comment!(\"authored\");\n\
                        // Source: case.trn\n\
                        // Namespace: hello\n\
                        fn main() {}\n";
        let canonical = canonicalize_rust(rendered).unwrap();

        assert!(canonical.contains("__terrane_generated_module_comment!(\"authored\");"));
        assert!(canonical.contains("// Source: case.trn"));
        assert!(canonical.contains("// Namespace: hello"));
    }

    #[test]
    fn leaves_unclosed_comment_marker_unchanged() {
        assert_eq!(
            restore_terrane_comments("__terrane_comment!(7, \"site\""),
            "__terrane_comment!(7, \"site\""
        );
    }

    #[test]
    fn metadata_codecs_accept_delimiter_text_inside_encoded_literals() {
        assert_eq!(
            restore_terrane_module_comments(
                "__terrane_generated_module_comment!(\"// Source: odd);name.trn\");\n",
                "__terrane_generated_module_comment",
            ),
            "// Source: odd);name.trn\n"
        );
        assert_eq!(
            restore_terrane_site_rows("__terrane_site_comment!(\"odd);site\");\n"),
            "/* terrane-site-row: odd);site */\n"
        );
    }

    #[test]
    fn malformed_metadata_macro_delimiters_are_left_unchanged() {
        let rendered = "__terrane_site_comment!(]);\n";
        assert_eq!(restore_terrane_site_rows(rendered), rendered);
    }

    #[test]
    fn split_entrypoint_import_canonicalizes_without_moving_authored_comments() {
        let rendered = "include!(\"main.support.rs\");\n\
                        // Source: case.trn\n\
                        // Namespace: hello\n\
                        fn main() {}\n";
        assert_eq!(canonicalize_rust(rendered).unwrap(), rendered);
    }
}
