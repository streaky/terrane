use std::collections::BTreeSet;

use syn::parse::Parser as _;
use syn::visit::Visit as _;

/// Syntactic free value uses, not Rust name resolution or macro expansion.
/// Opaque macro/verbatim tokens and unparseable fragments retain conservative matching.
pub(crate) fn rust_free_value_identifiers(rust: &str) -> BTreeSet<String> {
    let body = rust.split_once('\n').map_or(rust, |(header, body)| {
        if matches!(header.trim(), "rust" | "unsafe rust") {
            body
        } else {
            rust
        }
    });
    let Ok(statements) = syn::Block::parse_within.parse_str(body) else {
        return super::rust_syntactic_identifiers(body);
    };
    let mut values = FreeValues::default();
    values.block(&statements);
    values.free
}

/// Explicit mutable borrows of free Rust values, respecting Rust lexical shadowing.
/// Method-call mutability still belongs to Rust; callers must spell `&mut` when
/// lending a source value mutably through an opaque Rust block.
pub(crate) fn rust_mutably_borrowed_value_identifiers(rust: &str) -> BTreeSet<String> {
    let body = rust.split_once('\n').map_or(rust, |(header, body)| {
        if matches!(header.trim(), "rust" | "unsafe rust") {
            body
        } else {
            rust
        }
    });
    let Ok(statements) = syn::Block::parse_within.parse_str(body) else {
        return BTreeSet::new();
    };
    let mut values = FreeValues::default();
    values.block(&statements);
    values.mutably_borrowed
}

#[derive(Default)]
struct FreeValues {
    scopes: Vec<BTreeSet<String>>,
    free: BTreeSet<String>,
    mutably_borrowed: BTreeSet<String>,
}

impl FreeValues {
    fn record(&mut self, name: String) {
        if !self.scopes.iter().rev().any(|scope| scope.contains(&name)) {
            self.free.insert(name);
        }
    }

    fn bind_pattern(&mut self, pattern: &syn::Pat) {
        struct Bindings<'a>(&'a mut BTreeSet<String>);
        impl<'ast> syn::visit::Visit<'ast> for Bindings<'_> {
            fn visit_pat_ident(&mut self, binding: &'ast syn::PatIdent) {
                self.0.insert(binding.ident.to_string());
                syn::visit::visit_pat_ident(self, binding);
            }
        }
        Bindings(self.scopes.last_mut().expect("lexical Rust scope")).visit_pat(pattern);
    }

    fn bind_use(&mut self, tree: &syn::UseTree) {
        match tree {
            syn::UseTree::Name(name) => {
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert(name.ident.to_string());
            }
            syn::UseTree::Rename(rename) => {
                self.scopes
                    .last_mut()
                    .unwrap()
                    .insert(rename.rename.to_string());
            }
            syn::UseTree::Path(path) => self.bind_use(&path.tree),
            syn::UseTree::Group(group) => {
                for tree in &group.items {
                    self.bind_use(tree);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    fn block(&mut self, statements: &[syn::Stmt]) {
        self.scopes.push(BTreeSet::new());
        // Rust items are in scope throughout their block, unlike let bindings.
        for statement in statements {
            if let syn::Stmt::Item(item) = statement {
                let name = match item {
                    syn::Item::Fn(item) => Some(&item.sig.ident),
                    syn::Item::Const(item) => Some(&item.ident),
                    syn::Item::Static(item) => Some(&item.ident),
                    syn::Item::Struct(item) => Some(&item.ident),
                    _ => None,
                };
                if let Some(name) = name {
                    self.scopes.last_mut().unwrap().insert(name.to_string());
                }
                if let syn::Item::Use(item) = item {
                    self.bind_use(&item.tree);
                }
            }
        }
        for statement in statements {
            if let syn::Stmt::Local(local) = statement {
                // A new pattern does not shadow its initializer or let-else branch.
                if let Some(init) = &local.init {
                    self.visit_expr(&init.expr);
                    if let Some((_, diverge)) = &init.diverge {
                        self.visit_expr(diverge);
                    }
                }
                self.bind_pattern(&local.pat);
            } else {
                self.visit_stmt(statement);
            }
        }
        self.scopes.pop();
    }

    fn function_body(&mut self, signature: &syn::Signature, body: &syn::Block) {
        self.scopes.push(BTreeSet::new());
        for input in &signature.inputs {
            match input {
                syn::FnArg::Typed(parameter) => self.bind_pattern(&parameter.pat),
                syn::FnArg::Receiver(_) => {
                    self.scopes.last_mut().unwrap().insert("self".to_owned());
                }
            }
        }
        self.visit_block(body);
        self.scopes.pop();
    }

    fn condition(&mut self, expression: &syn::Expr) {
        match expression {
            syn::Expr::Let(test) => {
                self.visit_expr(&test.expr);
                self.bind_pattern(&test.pat);
            }
            syn::Expr::Binary(binary) if matches!(binary.op, syn::BinOp::And(_)) => {
                self.condition(&binary.left);
                self.condition(&binary.right);
            }
            _ => self.visit_expr(expression),
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for FreeValues {
    fn visit_expr_reference(&mut self, expression: &'ast syn::ExprReference) {
        fn root(expression: &syn::Expr) -> Option<&syn::Ident> {
            match expression {
                syn::Expr::Path(path)
                    if path.qself.is_none()
                        && path.path.leading_colon.is_none()
                        && path.path.segments.len() == 1 =>
                {
                    Some(&path.path.segments[0].ident)
                }
                syn::Expr::Paren(value) => root(&value.expr),
                syn::Expr::Group(value) => root(&value.expr),
                syn::Expr::Field(value) => root(&value.base),
                syn::Expr::Index(value) => root(&value.expr),
                syn::Expr::Unary(value) if matches!(value.op, syn::UnOp::Deref(_)) => {
                    root(&value.expr)
                }
                _ => None,
            }
        }
        if expression.mutability.is_some()
            && let Some(name) = root(&expression.expr)
        {
            let name = name.to_string();
            if !self.scopes.iter().rev().any(|scope| scope.contains(&name)) {
                self.mutably_borrowed.insert(name);
            }
        }
        syn::visit::visit_expr_reference(self, expression);
    }

    fn visit_expr_path(&mut self, expression: &'ast syn::ExprPath) {
        if expression.qself.is_none()
            && expression.path.leading_colon.is_none()
            && expression.path.segments.len() == 1
        {
            self.record(expression.path.segments[0].ident.to_string());
        }
        // Const generic arguments can themselves read values.
        syn::visit::visit_expr_path(self, expression);
    }

    fn visit_block(&mut self, block: &'ast syn::Block) {
        self.block(&block.stmts);
    }

    fn visit_expr_closure(&mut self, closure: &'ast syn::ExprClosure) {
        self.scopes.push(BTreeSet::new());
        for pattern in &closure.inputs {
            self.bind_pattern(pattern);
        }
        self.visit_expr(&closure.body);
        self.scopes.pop();
    }

    fn visit_item_fn(&mut self, function: &'ast syn::ItemFn) {
        self.function_body(&function.sig, &function.block);
    }

    fn visit_impl_item_fn(&mut self, function: &'ast syn::ImplItemFn) {
        self.function_body(&function.sig, &function.block);
    }

    fn visit_trait_item_fn(&mut self, function: &'ast syn::TraitItemFn) {
        if let Some(body) = &function.default {
            self.function_body(&function.sig, body);
        }
    }

    fn visit_expr_if(&mut self, expression: &'ast syn::ExprIf) {
        self.scopes.push(BTreeSet::new());
        self.condition(&expression.cond);
        self.visit_block(&expression.then_branch);
        self.scopes.pop();
        if let Some((_, branch)) = &expression.else_branch {
            self.visit_expr(branch);
        }
    }

    fn visit_expr_while(&mut self, expression: &'ast syn::ExprWhile) {
        self.scopes.push(BTreeSet::new());
        self.condition(&expression.cond);
        self.visit_block(&expression.body);
        self.scopes.pop();
    }

    fn visit_expr_for_loop(&mut self, expression: &'ast syn::ExprForLoop) {
        self.visit_expr(&expression.expr);
        self.scopes.push(BTreeSet::new());
        self.bind_pattern(&expression.pat);
        self.visit_block(&expression.body);
        self.scopes.pop();
    }

    fn visit_expr_match(&mut self, expression: &'ast syn::ExprMatch) {
        self.visit_expr(&expression.expr);
        for arm in &expression.arms {
            self.scopes.push(BTreeSet::new());
            self.bind_pattern(&arm.pat);
            if let Some((_, guard)) = &arm.guard {
                self.condition(guard);
            }
            self.visit_expr(&arm.body);
            self.scopes.pop();
        }
    }

    fn visit_macro(&mut self, invocation: &'ast syn::Macro) {
        for name in super::rust_syntactic_identifiers(&invocation.tokens.to_string()) {
            self.record(name);
        }
    }

    fn visit_expr(&mut self, expression: &'ast syn::Expr) {
        if let syn::Expr::Verbatim(tokens) = expression {
            for name in super::rust_syntactic_identifiers(&tokens.to_string()) {
                self.record(name);
            }
        } else {
            syn::visit::visit_expr(self, expression);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rust_free_value_identifiers;

    fn expect_reads(fragment: &str, expected: &[&str]) {
        assert_eq!(
            rust_free_value_identifiers(fragment),
            expected.iter().map(|name| (*name).to_owned()).collect()
        );
    }

    #[test]
    fn paths_members_and_shadow_initializers() {
        expect_reads(
            "{ let value = value; let local = 1; std::cmp::max(local, object.field.method()); value }",
            &["value", "object"],
        );
        expect_reads("rust\n { let value = 41; value + 1 }", &[]);
        expect_reads("unsafe rust\n { let value = 41; value + 1 }", &[]);
        expect_reads(
            "{ let value = 1; { let inner = value; inner } value }; value",
            &["value"],
        );
    }

    #[test]
    fn patterns_conditions_and_closures_keep_their_scopes() {
        expect_reads(
            "for (key, value) in pairs { consume(key, value, external); } match input { Some(inner) => inner, None => fallback }",
            &["pairs", "consume", "external", "input", "fallback"],
        );
        expect_reads(
            "items.iter().map(|item| item.field + outer)",
            &["items", "outer"],
        );
        expect_reads(
            "if let Some(value) = input && value > limit { value } else { value }",
            &["input", "limit", "value"],
        );
        expect_reads(
            "while let Some(value) = input { consume(value); } value",
            &["input", "consume", "value"],
        );
        expect_reads(
            "let Some(value) = input else { return value; }; value",
            &["input", "value"],
        );
    }

    #[test]
    fn items_and_opaque_macros_preserve_value_uses() {
        expect_reads(
            "{ use std::cmp::max; fn local(value: i32) -> i32 { value } local(max(1, 2)) }",
            &[],
        );
        expect_reads(
            "struct Local; impl Local { fn method(&self, value: i32) -> i32 { let _ = self; value } } trait Trait { fn method(&self, value: i32) -> i32 { let _ = self; value } }",
            &[],
        );
        expect_reads(
            "{ let local = 1; unknown!(local, free.field); }",
            &["free", "field"],
        );
        expect_reads("unknown!(free)", &["free"]);
        expect_reads("let incomplete =", &["let", "incomplete"]);
    }
}
