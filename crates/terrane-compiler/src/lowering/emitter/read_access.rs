use super::super::prelude::*;

impl Emitter<'_> {
    pub(super) fn read_snapshot(&mut self, operand: &SyntaxNode) -> String {
        let (setup, reference) = self.operand_reference_parts(operand, false);
        let name = self.fresh_read_binding_name("snapshot");
        // A statement boundary ends lock temporaries even when no setup was
        // needed (notably the shared-async `this` handle).
        format!("{{ {setup}let {name} = (*({reference})).clone(); {name} }}")
    }

    pub(super) fn borrowed_string_literal(&self, expression: &SyntaxNode) -> Option<String> {
        let mut literal = expression;
        while literal.kind == SyntaxKind::GroupExpression
            && let [inner] = literal.children.as_slice()
        {
            literal = inner;
        }
        (literal.kind == SyntaxKind::Literal
            && self.value_type(literal) == Some(ValueType::Scalar(ScalarType::String)))
        .then(|| crate::lowering::helpers::native_macro_literal(self.text(literal)))
    }

    pub(super) fn fresh_read_binding_name(&mut self, stem: &str) -> String {
        let index = self.read_place_counter;
        self.read_place_counter += 1;
        format!("__terrane_{stem}_{index}")
    }

    pub(super) fn read_index_needs_snapshot(
        &self,
        receiver: &SyntaxNode,
        index: &SyntaxNode,
    ) -> bool {
        if self.receiver_needs_live_guard(receiver) {
            !simple_read(index) || self.contains_guarded_read(index)
        } else {
            !self.read_borrow_is_stable_in(receiver, index)
        }
    }

    /// Observation can lend an existing place until the selected scope ends,
    /// including across suspension when its lender remains valid. Shared storage
    /// needs bounded guards; mutation within the scope needs a value snapshot.
    pub(super) fn read_borrow_is_stable_in(
        &self,
        operand: &SyntaxNode,
        scope: &SyntaxNode,
    ) -> bool {
        if self.receiver_needs_live_guard(operand) {
            return false;
        }
        let mut root = operand;
        while matches!(
            root.kind,
            SyntaxKind::GroupExpression
                | SyntaxKind::MemberExpression
                | SyntaxKind::IndexExpression
        ) && let Some(inner) = root.children.first()
        {
            root = inner;
        }
        if root.kind != SyntaxKind::Name {
            return true;
        }
        let Some(identity) = self.local_binding_identity(root) else {
            return false;
        };
        if scope.kind == SyntaxKind::ForStatement
            && scope.children.first().is_some_and(|target| {
                target
                    .children
                    .iter()
                    .any(|name| self.local_binding_identity(name) == Some(identity))
            })
        {
            return false;
        }
        !crate::semantics::binding_span_is_mutated_in(
            self.package,
            self.unit,
            scope,
            identity,
            true,
            ClosureWrites::Include,
        )
    }

    /// Immediate traversal observes the source directly instead of constructing
    /// a collection solely to iterate it. Item values retain their source ABI.
    pub(super) fn text_iteration_source(
        &mut self,
        collection: &SyntaxNode,
        scope: &SyntaxNode,
    ) -> Option<(String, String, crate::semantics::TextUnit)> {
        let mut collection = collection;
        while collection.kind == SyntaxKind::GroupExpression
            && let [inner] = collection.children.as_slice()
        {
            collection = inner;
        }
        let (source, unit) = match self.receiver_value_type(collection)? {
            ValueType::Scalar(ScalarType::String) => {
                (collection, crate::semantics::TextUnit::Graphemes)
            }
            ValueType::Scalar(ScalarType::Bytes) => (collection, crate::semantics::TextUnit::Bytes),
            ValueType::StringView(unit) if collection.kind == SyntaxKind::MemberExpression => {
                (collection.children.first()?, unit)
            }
            _ => return None,
        };
        let (setup, source) = if self.read_borrow_is_stable_in(source, scope) {
            self.operand_reference_parts(source, false)
        } else {
            // Mutation during traversal needs a snapshot. Do not retain a
            // shared guard across user code or a suspension.
            (String::new(), self.read_snapshot(source))
        };
        Some((setup, source, unit))
    }

    pub(super) fn contains_guarded_read(&self, operand: &SyntaxNode) -> bool {
        self.receiver_needs_live_guard(operand)
            || operand
                .children
                .iter()
                .any(|child| self.contains_guarded_read(child))
    }

    fn has_guarded_index_read(&self, operand: &SyntaxNode) -> bool {
        operand.kind == SyntaxKind::IndexExpression
            && operand
                .children
                .iter()
                .skip(1)
                .any(|index| self.contains_guarded_read(index))
            || operand
                .children
                .iter()
                .any(|child| self.has_guarded_index_read(child))
    }

    fn can_coalesce_read(&self, operand: &SyntaxNode) -> bool {
        simple_read(operand)
            && !self.has_guarded_index_read(operand)
            && (!self.contains_guarded_read(operand)
                || matches!(
                    self.receiver_value_type(operand),
                    Some(
                        ValueType::Scalar(ScalarType::String | ScalarType::Bytes)
                            | ValueType::List(_)
                            | ValueType::Object(_)
                    )
                ))
    }

    pub(super) fn prepare_read_guards<'node>(
        &mut self,
        mut operands: impl Iterator<Item = &'node SyntaxNode> + Clone,
        setup: &mut String,
    ) {
        if !operands
            .clone()
            .all(|operand| self.can_coalesce_read(operand))
        {
            return;
        }
        let mut roots = BTreeMap::<crate::Span, (&SyntaxNode, usize)>::new();
        for operand in &mut operands {
            let root = read_root(operand);
            if root.kind == SyntaxKind::Name
                && self.receiver_needs_live_guard(root)
                && let Some(identity) = self.local_binding_identity(root)
            {
                roots.entry(identity).or_insert((root, 0)).1 += 1;
            }
        }
        // Different bindings may own handles to the same storage. Do not
        // acquire multiple guards without a storage-distinctness proof.
        if roots.len() != 1 {
            return;
        }
        for (identity, (root, uses)) in roots {
            if uses < 2 {
                continue;
            }
            let (owner_setup, place) = self.borrowed_receiver_parts(root);
            let name = format!(
                "__terrane_native_read_guard_{}",
                self.borrowed_receiver_overrides.len()
            );
            write!(setup, "{owner_setup}let {name} = &*({place}); ")
                .expect("writing to a String cannot fail");
            self.borrowed_receiver_overrides.push((identity, name));
        }
    }

    pub(super) fn has_read_guard(&self, operand: &SyntaxNode) -> bool {
        self.local_binding_identity(read_root(operand))
            .is_some_and(|identity| {
                self.borrowed_receiver_overrides
                    .iter()
                    .any(|(candidate, _)| *candidate == identity)
            })
    }

    pub(super) fn native_read_needs_snapshot(
        &self,
        operand: &SyntaxNode,
        arguments: &SyntaxNode,
        guarded_receiver: bool,
        retained: bool,
    ) -> bool {
        if !self.receiver_needs_live_guard(operand) {
            return !self.read_borrow_is_stable_in(operand, arguments);
        }
        let coalesced = self.has_read_guard(operand);
        retained
            || guarded_receiver
            || !arguments
                .children
                .iter()
                .all(|argument| simple_read(argument.children.last().unwrap_or(argument)))
            || !coalesced
                && arguments
                    .children
                    .iter()
                    .filter(|argument| {
                        self.contains_guarded_read(argument.children.last().unwrap_or(argument))
                    })
                    .count()
                    > 1
    }

    pub(super) fn staged_read_snapshot(
        &mut self,
        operand: &SyntaxNode,
        setup: &mut String,
        index: &mut usize,
    ) -> String {
        let value = self.read_snapshot(operand);
        let name = format!("__terrane_native_snapshot_{index}");
        *index += 1;
        write!(setup, "let {name} = {value}; ").expect("writing to a String cannot fail");
        format!("&{name}")
    }
    pub(super) fn staged_native_value_argument(
        &mut self,
        operand: &SyntaxNode,
        value_type: ValueType,
        setup: &mut String,
        index: &mut usize,
    ) -> String {
        let value = self.expression_as(operand, value_type);
        let name = format!("__terrane_native_value_{index}");
        *index += 1;
        write!(setup, "let {name} = {value}; ").expect("writing to a String cannot fail");
        format!("&{name}")
    }

    pub(super) fn staged_native_future_receiver(
        receiver: impl std::fmt::Display,
        setup: &mut String,
    ) -> String {
        let name = "__terrane_native_future_receiver";
        write!(setup, "let {name} = {receiver}; ").expect("writing to a String cannot fail");
        name.to_owned()
    }

    fn iterable_constructor(collection_type: Option<ValueType>, source: &str) -> String {
        match collection_type {
            Some(
                ValueType::Scalar(ScalarType::Bytes)
                | ValueType::StringView(crate::semantics::TextUnit::Bytes),
            ) => format!("terrane_collection_support::bytes_iterator(&{source})"),
            Some(ValueType::StringView(
                crate::semantics::TextUnit::Scalars | crate::semantics::TextUnit::Graphemes,
            )) => format!("terrane_collection_support::Iterator::new({source})"),
            Some(ValueType::StringList | ValueType::TextRangeList) => {
                format!("terrane_collection_support::slice_iterator(&{source})")
            }
            Some(ValueType::Iterator(_)) => format!("&mut {source}"),
            Some(ValueType::Reference(item))
                if matches!(
                    item.value_type(),
                    ValueType::List(_) | ValueType::Tuple(_, _)
                ) =>
            {
                format!("{source}.terrane_borrowing_iterator()")
            }
            Some(
                ValueType::List(_)
                | ValueType::Map(_, _)
                | ValueType::Set(_)
                | ValueType::Tuple(_, _)
                | ValueType::Range
                | ValueType::UnorderedMap(_, _)
                | ValueType::UnorderedSet(_),
            ) => format!("terrane_collection_support::Iterable::terrane_iterator(&{source})"),
            Some(ValueType::Object(_)) => format!("{source}.iterator()"),
            _ => format!("terrane_collection_support::string_iterator(&{source})"),
        }
    }
    pub(super) fn index_read_receiver_parts(
        &mut self,
        receiver: &SyntaxNode,
        index: &SyntaxNode,
    ) -> (String, String) {
        let consuming = matches!(
            self.receiver_value_type(receiver),
            Some(ValueType::List(item))
                if matches!(item.value_type_ref(), ValueType::InvocationScopedNative { .. })
        );
        if self.assignment_target || consuming {
            (String::new(), self.receiver_expression(receiver))
        } else if !self.read_index_needs_snapshot(receiver, index) {
            self.borrowed_receiver_parts(receiver)
        } else {
            (String::new(), self.read_snapshot(receiver))
        }
    }

    pub(super) fn for_iterable_source(
        &mut self,
        collection: &SyntaxNode,
        collection_type: Option<&ValueType>,
        text_iteration: Option<(String, String, crate::semantics::TextUnit)>,
        loop_index: usize,
    ) -> (String, Option<crate::semantics::TextUnit>) {
        let (expression, unit) = if let Some((setup, source, unit)) = text_iteration {
            if !setup.is_empty() {
                self.line(&setup);
            }
            (source, Some(unit))
        } else {
            (
                match collection_type {
                    Some(value_type) => self.expression_as(collection, value_type.clone()),
                    None => self.expression(collection),
                },
                None,
            )
        };
        if matches!(collection_type, Some(ValueType::Iterator(_)))
            && collection.kind == SyntaxKind::Name
        {
            (expression, unit)
        } else {
            let name = format!("__terrane_iterable_{loop_index}");
            let mutable = if matches!(collection_type, Some(ValueType::Iterator(_))) {
                "mut "
            } else {
                ""
            };
            self.line(&format!("let {mutable}{name} = {expression};"));
            (name, unit)
        }
    }

    pub(super) fn for_iterator_constructor(
        &self,
        collection: &SyntaxNode,
        source: &str,
        collection_type: Option<ValueType>,
        native: bool,
        text_unit: Option<crate::semantics::TextUnit>,
    ) -> String {
        if let Some(unit) = text_unit {
            match unit {
                crate::semantics::TextUnit::Bytes
                    if matches!(
                        self.receiver_value_type(collection),
                        Some(ValueType::Scalar(ScalarType::Bytes))
                    ) =>
                {
                    format!("{source}.iter().copied()")
                }
                crate::semantics::TextUnit::Bytes => format!("{source}.bytes()"),
                crate::semantics::TextUnit::Scalars => {
                    format!("{source}.chars().map(|value| value.to_string())")
                }
                crate::semantics::TextUnit::Graphemes => {
                    format!("terrane_string_support::graphemes(&{source})")
                }
            }
        } else if native {
            format!("{source}.iter()")
        } else {
            Self::iterable_constructor(collection_type, source)
        }
    }
}

fn read_root(mut operand: &SyntaxNode) -> &SyntaxNode {
    while matches!(
        operand.kind,
        SyntaxKind::GroupExpression | SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
    ) && let Some(inner) = operand.children.first()
    {
        operand = inner;
    }
    operand
}

fn simple_read(operand: &SyntaxNode) -> bool {
    matches!(
        operand.kind,
        SyntaxKind::Name
            | SyntaxKind::Literal
            | SyntaxKind::GroupExpression
            | SyntaxKind::MemberExpression
            | SyntaxKind::IndexExpression
    ) && operand.children.iter().all(simple_read)
}
