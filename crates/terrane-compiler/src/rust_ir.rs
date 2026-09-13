use proc_macro2::{TokenStream, TokenTree};
use std::fmt::Write as _;
use syn::fold::Fold as _;
use syn::parse::Parser as _;

use crate::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub version: &'static str,
    pub requires_platform_support: bool,
    pub requires_async_runtime: bool,
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
    runtime_source_files: Vec<&'static str>,
    support: RenderedFragment,
    standalone: RenderedFragment,
    application: RenderedFragment,
    review: RenderedFragment,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenderedFragment {
    contents: String,
    associations: Vec<SourceAssociation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedModule {
    pub name: &'static str,
    pub source_files: Vec<&'static str>,
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
enum BlockBody {
    Parsed(syn::File),
    Raw(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    body: BlockBody,
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
        let after_comment = &remaining[comment_end + 3..];
        let expression_start = after_comment
            .char_indices()
            .find_map(|(index, character)| (!character.is_whitespace()).then_some(index))
            .unwrap_or(after_comment.len());
        if after_comment.as_bytes().get(expression_start) != Some(&b'{') {
            encoded.push_str(&remaining[..comment_end + 3]);
            remaining = after_comment;
            continue;
        }
        let mut depth = 0_usize;
        let mut expression_end = None;
        for (offset, byte) in after_comment.as_bytes()[expression_start..]
            .iter()
            .copied()
            .enumerate()
        {
            match byte {
                b'{' => depth += 1,
                b'}' if depth == 1 => {
                    expression_end = Some(expression_start + offset);
                    break;
                }
                b'}' => depth -= 1,
                _ => {}
            }
        }
        let Some(expression_end) = expression_end else {
            encoded.push_str(&remaining[..comment_end + 3]);
            remaining = after_comment;
            continue;
        };
        let expression = after_comment[expression_start + 1..expression_end].trim();
        if !expression.starts_with("Site {") {
            encoded.push_str(&remaining[..comment_end + 3]);
            remaining = after_comment;
            continue;
        }
        encoded.push_str(&remaining[..comment_start]);
        let comment = &remaining[comment_start + MARKER.len()..comment_end];
        write!(
            encoded,
            "__terrane_site_row!({:?}; {expression:?})",
            comment.replace("*/", "* /")
        )
        .expect("writing to a String cannot fail");
        remaining = &after_comment[expression_end + 1..];
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
    const MARKER: &str = "__terrane_site_row!(";
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
                b')' | b']' | b'}' if depth > 1 => depth -= 1,
                b';' if depth == 1 && separator.is_none() => {
                    separator = Some(arguments_start + offset);
                }
                _ => {}
            }
        }
        let (Some(separator), Some(end)) = (separator, end) else {
            restored.push_str(&remaining[start..]);
            return restored;
        };
        let Ok(comment) =
            syn::parse_str::<syn::LitStr>(remaining[arguments_start..separator].trim())
        else {
            restored.push_str(&remaining[start..=end]);
            remaining = &remaining[end + 1..];
            continue;
        };
        let Ok(expression) = syn::parse_str::<syn::LitStr>(remaining[separator + 1..end].trim())
        else {
            restored.push_str(&remaining[start..=end]);
            remaining = &remaining[end + 1..];
            continue;
        };
        let expression = expression.value();
        if !expression.starts_with("Site {") {
            restored.push_str(&remaining[start..=end]);
            remaining = &remaining[end + 1..];
            continue;
        }
        let indentation = remaining[..start]
            .rsplit_once('\n')
            .map_or("", |(_, indentation)| indentation);
        write!(
            restored,
            "/* terrane-site-row: {} */\n{indentation}{{ {expression} }}",
            comment.value()
        )
        .expect("writing to a String cannot fail");
        remaining = &remaining[end + 1..];
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
        let body = syn::parse_file(rust).map_or_else(
            |_| BlockBody::Raw(rust.to_owned()),
            |parsed| BlockBody::Parsed(canonicalize_file(parsed)),
        );
        Self { body }
    }

    fn render(&self, output: &mut String) {
        match &self.body {
            BlockBody::Parsed(parsed) => output.push_str(&restore_terrane_metadata(
                &prettyplease::unparse(parsed),
                None,
            )),
            BlockBody::Raw(raw) => output.push_str(raw),
        }
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

const VENDORED_SUPPORT_CRATES: [(&str, &str); 7] = [
    ("terrane_int_support", "terrane-int-support"),
    ("terrane_collection_support", "terrane-collection-support"),
    ("terrane_scalar_support", "terrane-scalar-support"),
    ("terrane_string_support", "terrane-string-support"),
    ("terrane_document_support", "terrane-document-support"),
    ("terrane_stream_abi", "terrane-stream-abi"),
    ("terrane_platform_support", "terrane-platform-support"),
];

fn token_stream_contains_ident(tokens: TokenStream, expected: &str) -> bool {
    tokens.into_iter().any(|token| match token {
        TokenTree::Group(group) => token_stream_contains_ident(group.stream(), expected),
        TokenTree::Ident(ident) => ident == expected,
        TokenTree::Punct(_) | TokenTree::Literal(_) => false,
    })
}

fn push_manifest_line(output: &mut String, label: &str, values: &[&str]) {
    write!(output, "// {label}:").expect("writing to a String cannot fail");
    if !values.is_empty() {
        write!(output, " {}", values.join(", ")).expect("writing to a String cannot fail");
    }
    output.push('\n');
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
    pub(crate) fn review_file(&self) -> String {
        let runtime_support = self.runtime_source_files().collect::<Vec<_>>();
        let tokens = self
            .standalone
            .contents
            .parse::<TokenStream>()
            .expect("rendered Rust must contain valid tokens");
        let vendored_support = VENDORED_SUPPORT_CRATES
            .iter()
            .filter_map(|(rust_name, package_name)| {
                token_stream_contains_ident(tokens.clone(), rust_name).then_some(*package_name)
            })
            .collect::<Vec<_>>();
        let mut output = format!(
            "// Generated deterministically by Terrane {}.\n",
            self.version
        );
        push_manifest_line(&mut output, "Runtime support", &runtime_support);
        push_manifest_line(&mut output, "Vendored support crates", &vendored_support);
        output.push_str(&self.review.contents);
        output
    }

    fn runtime_source_files(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.runtime_source_files.iter().copied()
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

        let mut runtime = String::new();
        let mut runtime_associations = Vec::new();
        for module in &self.runtime {
            for item in &module.items {
                item.render_associated(&mut runtime, &mut runtime_associations);
            }
        }
        let mut program = String::new();
        let mut program_associations = Vec::new();
        for item in &self.globals {
            item.render_associated(&mut program, &mut program_associations);
        }
        render_modules(&self.modules, &mut program, &mut program_associations);
        let mut review = String::new();
        let mut review_associations = Vec::new();
        for module in self
            .runtime
            .iter()
            .filter(|module| module.source_files.is_empty())
        {
            for item in &module.items {
                item.render_associated(&mut review, &mut review_associations);
            }
        }
        let review_prefix_len = review.len();
        review.push_str(&program);
        review_associations.extend(program_associations.iter().map(|association| {
            SourceAssociation {
                generated_start: association.generated_start + review_prefix_len,
                generated_end: association.generated_end + review_prefix_len,
                source: association.source,
            }
        }));
        let mut standalone = runtime.clone();
        standalone.push_str(&program);
        let mut standalone_associations = runtime_associations.clone();
        standalone_associations.extend(program_associations.iter().map(|association| {
            SourceAssociation {
                generated_start: association.generated_start + runtime.len(),
                generated_end: association.generated_end + runtime.len(),
                source: association.source,
            }
        }));

        let mut support = runtime;
        let mut support_associations = runtime_associations;
        for item in &self.globals {
            item.render_associated(&mut support, &mut support_associations);
        }
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
            runtime_source_files: self
                .runtime
                .iter()
                .flat_map(|module| module.source_files.iter().copied())
                .collect(),
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
            review: RenderedFragment {
                contents: review,
                associations: review_associations,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Block, GeneratedModule, Item, Module, ModuleDestination, Program, canonicalize_rust,
        encode_terrane_comments, restore_terrane_comments, restore_terrane_metadata,
        restore_terrane_module_comments, restore_terrane_site_rows,
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
    fn preserves_unparsable_generated_text_for_validation() {
        let rust = "fn broken( {";
        let block = Block::from_rendered(rust);
        let mut rendered = String::new();
        block.render(&mut rendered);

        assert_eq!(rendered, rust);
        assert!(canonicalize_rust(&rendered).is_err());
    }

    #[test]
    fn restores_generated_site_comments_after_canonicalization() {
        let block = Block::from_rendered(
            r#"fn main() {
                let value = raised(call(), __terrane_comment!(7, "src/main.trn:4:9-4:15"));
                let sites = [__terrane_site_row!("site 7 /demo::main"; "Site { function: 0, file: 0, line: 4, column: 9, end_line: 4, end_column: 15 }")];
            }"#,
        );
        let mut rendered = String::new();
        block.render(&mut rendered);

        assert!(
            rendered.contains("7 /* terrane-site: src/main.trn:4:9-4:15 */"),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "/* terrane-site-row: site 7 /demo::main */\n        { Site { function: 0, file: 0, line: 4, column: 9, end_line: 4, end_column: 15 } }"
            ),
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
            restore_terrane_site_rows(
                "__terrane_site_row!(\"odd);site\"; \"Site { function: 0 }\")\n"
            ),
            "/* terrane-site-row: odd);site */\n{ Site { function: 0 } }\n"
        );
    }

    #[test]
    fn malformed_metadata_macro_delimiters_are_left_unchanged() {
        let rendered = "__terrane_site_row!(]);\n";
        assert_eq!(restore_terrane_site_rows(rendered), rendered);
    }

    #[test]
    fn review_rendering_replaces_runtime_contents_with_a_manifest() {
        let program = |runtime_body, sites_body| Program {
            version: "test",
            requires_platform_support: false,
            requires_async_runtime: true,
            runtime: vec![
                GeneratedModule {
                    name: "async",
                    source_files: vec!["async.rs"],
                    items: vec![Item::generated(runtime_body)],
                },
                GeneratedModule {
                    name: "sites",
                    source_files: Vec::new(),
                    items: vec![Item::generated(sites_body)],
                },
            ],
            globals: vec![Item::generated(
                "static VALUE: terrane_int_support::Int = terrane_int_support::Int::ZERO;",
            )],
            modules: vec![Module {
                source_path: "case.trn".to_owned(),
                namespace: "/case".to_owned(),
                destination: ModuleDestination::Application,
                items: vec![Item::generated("fn main() {}")],
            }],
        };

        let sites = "static SITES: &[u32] = &[7];\n\
                     static LABEL: &str = \"terrane_string_support\";";
        let first = program("fn runtime_first() {}", sites).rendered();
        let second = program("fn runtime_second() {}", sites).rendered();
        let changed_sites = program(
            "fn runtime_first() {}",
            "static SITES: &[u32] = &[8];\n\
             static LABEL: &str = \"terrane_string_support\";",
        )
        .rendered();
        assert_ne!(
            first.standalone_file("<stdout>").contents,
            second.standalone_file("<stdout>").contents
        );
        assert_eq!(first.review_file(), second.review_file());
        assert_ne!(first.review_file(), changed_sites.review_file());
        assert_eq!(
            first.review_file(),
            "// Generated deterministically by Terrane test.\n\
             // Runtime support: async.rs\n\
             // Vendored support crates: terrane-int-support\n\
             static SITES: &[u32] = &[7];\n\
             static LABEL: &str = \"terrane_string_support\";\n\
             static VALUE: terrane_int_support::Int = terrane_int_support::Int::ZERO;\n\
             // Source: case.trn\n\
             // Namespace: case\n\
             fn main() {}\n"
        );
    }

    #[test]
    fn empty_review_manifests_have_no_trailing_spaces() {
        let rendered = Program {
            version: "test",
            requires_platform_support: false,
            requires_async_runtime: false,
            runtime: Vec::new(),
            globals: Vec::new(),
            modules: Vec::new(),
        }
        .rendered()
        .review_file();

        assert_eq!(
            rendered,
            "// Generated deterministically by Terrane test.\n\
             // Runtime support:\n\
             // Vendored support crates:\n"
        );
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
