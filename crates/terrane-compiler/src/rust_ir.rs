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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub source_path: String,
    pub namespace: String,
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
    let parsed = syn::parse_file(rust)?;
    Ok(prettyplease::unparse(&canonicalize_file(parsed)))
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
                b',' if depth == 1 => separator = Some(arguments_start + offset),
                _ => {}
            }
        }
        let (Some(separator), Some(end)) = (separator, end) else {
            restored.push_str(&remaining[start..]);
            return restored;
        };
        let expression = remaining[arguments_start..separator].trim_end();
        let encoded_comment = remaining[separator + 1..end].trim();
        let Ok(comment) = syn::parse_str::<syn::LitStr>(encoded_comment) else {
            restored.push_str(&remaining[start..=end]);
            remaining = &remaining[end + 1..];
            continue;
        };
        let comment = comment.value().replace("*/", "* /");
        write!(restored, "{expression} /* {comment} */")
            .expect("writing to a String cannot fail");
        remaining = &remaining[end + 1..];
    }
    restored.push_str(remaining);
    restored
}

impl Block {
    fn from_rendered(rust: &str) -> Self {
        let parsed = syn::parse_file(rust).expect("lowered Rust item must parse");
        Self {
            parsed: canonicalize_file(parsed),
        }
    }

    fn render(&self, output: &mut String) {
        output.push_str(&restore_terrane_comments(&prettyplease::unparse(&self.parsed)));
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

impl Program {
    #[must_use]
    pub fn render(&self) -> String {
        let files = self.render_files();
        let mut output = format!(
            "// Generated deterministically by Terrane {}.\n",
            self.version
        );
        for file in files.iter().filter(|file| file.path != "src/main.rs") {
            output.push_str(&file.contents);
        }
        output
    }
    #[must_use]
    pub fn render_files(&self) -> Vec<RenderedFile> {
        let mut files = Vec::new();
        let mut entrypoint = format!(
            "// Generated deterministically by Terrane {}.\n",
            self.version
        );
        for module in &self.runtime {
            if module.items.is_empty() {
                continue;
            }
            let mut contents = String::new();
            let mut associations = Vec::new();
            for item in &module.items {
                item.render_associated(&mut contents, &mut associations);
            }
            let path = format!("src/runtime/{}.rs", module.name);
            writeln!(entrypoint, "include!(\"runtime/{}.rs\");", module.name)
                .expect("writing to a String cannot fail");
            files.push(RenderedFile {
                path,
                contents,
                associations,
            });
        }
        if !self.globals.is_empty() {
            let mut contents = String::new();
            let mut associations = Vec::new();
            for item in &self.globals {
                item.render_associated(&mut contents, &mut associations);
            }
            files.push(RenderedFile {
                path: "src/authored/globals.rs".to_owned(),
                contents,
                associations,
            });
            entrypoint.push_str("include!(\"authored/globals.rs\");\n");
        }
        for module in &self.modules {
            if module.items.is_empty() {
                continue;
            }
            let mut contents = format!(
                "// Source: {}\n// Namespace: {}\n",
                module.source_path,
                module.namespace.trim_start_matches('/')
            );
            let mut associations = Vec::new();
            for item in &module.items {
                item.render_associated(&mut contents, &mut associations);
            }
            let path = format!("src/authored/{}.rs", module.source_path);
            writeln!(
                entrypoint,
                "include!(\"authored/{}.rs\");",
                module.source_path
            )
            .expect("writing to a String cannot fail");
            files.push(RenderedFile {
                path,
                contents,
                associations,
            });
        }
        files.push(RenderedFile {
            path: "src/main.rs".to_owned(),
            contents: entrypoint,
            associations: Vec::new(),
        });
        files
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, restore_terrane_comments};

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
                let sites = [Site {
                    function: __terrane_comment!(0, "site 7 /demo::main"),
                    file: 0,
                    line: 4,
                    column: 9,
                }];
            }"#,
        );
        let mut rendered = String::new();
        block.render(&mut rendered);

        assert!(
            rendered.contains("7 /* src/main.trn:4:9-4:15 */"),
            "{rendered}"
        );
        assert!(
            rendered.contains("function: 0 /* site 7 /demo::main */,"),
            "{rendered}"
        );
        assert!(!rendered.contains("__terrane_comment"), "{rendered}");
    }

    #[test]
    fn leaves_unclosed_comment_marker_unchanged() {
        assert_eq!(
            restore_terrane_comments("__terrane_comment!(7, \"site\""),
            "__terrane_comment!(7, \"site\""
        );
    }
}
