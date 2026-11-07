//! Native loans use reaching values and backwards liveness over execution edges,
//! not source offsets. Rebinding kills the old value; aliases and closures retain
//! its origin only while a reachable use needs them. Finally blocks are routed
//! through each continuation, including loop exits.
use super::super::prelude::*;
use super::binding_identity_at;

#[derive(Clone, Copy)]
struct Exits {
    normal: usize,
    returned: usize,
    thrown: usize,
    broken: usize,
    continued: usize,
}

#[derive(Default)]
struct Step {
    span: Option<Span>,
    reads: BTreeSet<Span>,
    definition: Option<Span>,
    sources: BTreeSet<Span>,
    next: Vec<usize>,
    previous: Vec<usize>,
    capture_region: Option<Span>,
}

type StorageAliases = BTreeMap<Span, BTreeSet<Span>>;

pub(super) struct Flow {
    steps: Vec<Step>,
    entry: usize,
    live_after: Vec<BTreeSet<Span>>,
    aliases: Vec<Option<StorageAliases>>,
    clause_entries: BTreeMap<Span, Vec<usize>>,
}

struct Builder<'a> {
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    steps: Vec<Step>,
    clause_entries: BTreeMap<Span, Vec<usize>>,
}

impl Builder<'_> {
    fn identity(&self, node: &SyntaxNode) -> Option<Span> {
        binding_identity_at(self.unit, node).or_else(|| {
            self.package
                .resolve_name_at(
                    self.unit,
                    node.span.start,
                    node_text(&self.unit.source, node),
                )
                .and_then(|symbol| symbol.declaration_span)
        })
    }

    fn reads(&self, node: &SyntaxNode, reads: &mut BTreeSet<Span>) {
        if node.kind == SyntaxKind::Name {
            reads.extend(self.identity(node));
        }
        for (index, child) in node.children.iter().enumerate() {
            if super::super::scope_flow::is_flow_value_child(node, index, child) {
                self.reads(child, reads);
            }
        }
    }

    fn step(&mut self, step: Step) -> usize {
        let index = self.steps.len();
        self.steps.push(step);
        index
    }

    fn sequence(&mut self, nodes: &[SyntaxNode], mut exits: Exits) -> usize {
        for node in nodes.iter().rev() {
            exits.normal = self.build(node, exits);
        }
        exits.normal
    }

    fn condition(&mut self, condition: &SyntaxNode, yes: usize, no: usize, exits: Exits) -> usize {
        let next = match super::super::scope_flow::constant_boolean(self.unit, condition) {
            Some(true) => vec![yes],
            Some(false) => vec![no],
            None => vec![yes, no],
        };
        let branch = self.step(Step {
            next,
            ..Step::default()
        });
        self.build(
            condition,
            Exits {
                normal: branch,
                ..exits
            },
        )
    }

    fn if_statement(&mut self, node: &SyntaxNode, exits: Exits) -> usize {
        let Some(condition) = node.children.first() else {
            return exits.normal;
        };
        let Some(body) = node.children.get(1) else {
            return exits.normal;
        };
        let mut remaining = exits.normal;
        for clause in node.children.iter().skip(2).rev() {
            let Some(body) = clause
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            else {
                continue;
            };
            let yes = self.build(body, exits);
            remaining = if let Some(condition) = clause
                .children
                .iter()
                .find(|child| child.kind != SyntaxKind::Block)
            {
                self.condition(condition, yes, remaining, exits)
            } else {
                yes
            };
        }
        let yes = self.build(body, exits);
        self.condition(condition, yes, remaining, exits)
    }

    fn carries_loan(&self, value: &SyntaxNode) -> bool {
        infer_value_type(self.unit, value, &self.unit.typed_bindings)
            .ok()
            .flatten()
            .is_some_and(|ty| {
                super::field_has_native_loan(self.package, &ty)
                    || matches!(
                        ty,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::Function(..)
                            | ValueType::AsyncFunction(..)
                    )
            })
    }
    fn loan_sources(&self, node: &SyntaxNode, sources: &mut BTreeSet<Span>) {
        if node.kind == SyntaxKind::Name && self.carries_loan(node) {
            sources.extend(self.identity(node));
        }
        for (index, child) in node.children.iter().enumerate() {
            if super::super::scope_flow::is_flow_value_child(node, index, child) {
                self.loan_sources(child, sources);
            }
        }
    }
    fn loop_statement(&mut self, node: &SyntaxNode, exits: Exits) -> usize {
        if node.kind == SyntaxKind::ForStatement
            && let [initial, condition, update, body] = node.children.as_slice()
        {
            let gate = self.step(Step::default());
            let condition_entry = self.build(
                condition,
                Exits {
                    normal: gate,
                    ..exits
                },
            );
            let update_entry = self.build(
                update,
                Exits {
                    normal: condition_entry,
                    ..exits
                },
            );
            let body_entry = self.build(
                body,
                Exits {
                    normal: update_entry,
                    continued: update_entry,
                    broken: exits.normal,
                    ..exits
                },
            );
            self.steps[gate].next =
                match super::super::scope_flow::constant_boolean(self.unit, condition) {
                    Some(true) => vec![body_entry],
                    Some(false) => vec![exits.normal],
                    None => vec![body_entry, exits.normal],
                };
            return self.build(
                initial,
                Exits {
                    normal: condition_entry,
                    ..exits
                },
            );
        }
        let gate = self.step(Step::default());
        let body = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block);
        let condition = (node.kind == SyntaxKind::WhileStatement)
            .then(|| {
                node.children
                    .iter()
                    .find(|child| child.kind != SyntaxKind::Block)
            })
            .flatten();
        let condition_entry = condition.map_or(gate, |condition| {
            self.build(
                condition,
                Exits {
                    normal: gate,
                    ..exits
                },
            )
        });
        let body_entry = body.map_or(condition_entry, |body| {
            self.build(
                body,
                Exits {
                    normal: condition_entry,
                    broken: exits.normal,
                    continued: condition_entry,
                    ..exits
                },
            )
        });
        self.steps[gate].next = match condition
            .and_then(|condition| super::super::scope_flow::constant_boolean(self.unit, condition))
        {
            Some(true) => vec![body_entry],
            Some(false) => vec![exits.normal],
            None => vec![body_entry, exits.normal],
        };
        if node.kind == SyntaxKind::ForStatement
            && let Some(collection) = node
                .children
                .iter()
                .rev()
                .find(|child| child.kind != SyntaxKind::Block)
        {
            return self.build(
                collection,
                Exits {
                    normal: gate,
                    ..exits
                },
            );
        }
        condition_entry
    }

    fn try_statement(&mut self, node: &SyntaxNode, exits: Exits) -> usize {
        let finally = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::FinallyClause);
        let routed = if let Some(finally) = finally {
            let mut continuations = BTreeMap::new();
            let mut route = |target| {
                *continuations.entry(target).or_insert_with(|| {
                    self.build(
                        finally,
                        Exits {
                            normal: target,
                            ..exits
                        },
                    )
                })
            };
            Exits {
                normal: route(exits.normal),
                returned: route(exits.returned),
                thrown: route(exits.thrown),
                broken: route(exits.broken),
                continued: route(exits.continued),
            }
        } else {
            exits
        };
        let mut catches = node
            .children
            .iter()
            .filter(|child| child.kind == SyntaxKind::CatchClause)
            .map(|catch| self.build(catch, routed))
            .collect::<Vec<_>>();
        catches.push(routed.thrown);
        let thrown = self.step(Step {
            next: catches,
            ..Step::default()
        });
        let body = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
            .map_or(routed.normal, |body| {
                self.build(body, Exits { thrown, ..routed })
            });
        // Lowering captures the body and catches in one closure. Its access
        // obligations cover even a statically non-throwing catch.
        let mut reads = BTreeSet::new();
        let mut capture_region = node.span;
        capture_region.end = node.span.start;
        for child in &node.children {
            if child.kind != SyntaxKind::FinallyClause {
                self.reads(child, &mut reads);
                capture_region.end = capture_region.end.max(child.span.end);
            }
        }
        self.step(Step {
            span: Some(node.span),
            reads,
            next: vec![body, thrown],
            capture_region: Some(capture_region),
            ..Step::default()
        })
    }

    fn switch_statement(&mut self, node: &SyntaxNode, exits: Exits) -> usize {
        let selector = node.switch_selector();
        let cases = node.switch_cases().collect::<Vec<_>>();
        let mut remaining = exits.normal;
        for case in cases.into_iter().rev() {
            let [header, body] = case.children.as_slice() else {
                continue;
            };
            let body = self.build(body, exits);
            if header.kind == SyntaxKind::MatchCatchAll {
                remaining = body;
            } else {
                for predicate in header.children.iter().rev() {
                    remaining = if selector.is_none() {
                        self.condition(predicate, body, remaining, exits)
                    } else {
                        let gate = self.step(Step {
                            next: vec![body, remaining],
                            ..Step::default()
                        });
                        self.build(
                            predicate,
                            Exits {
                                normal: gate,
                                ..exits
                            },
                        )
                    };
                }
            }
        }
        selector.map_or(remaining, |selector| {
            self.build(
                selector,
                Exits {
                    normal: remaining,
                    ..exits
                },
            )
        })
    }

    #[expect(
        clippy::too_many_lines,
        reason = "CFG node construction keeps syntax dispatch and value events together"
    )]
    fn build(&mut self, node: &SyntaxNode, exits: Exits) -> usize {
        if node.kind == SyntaxKind::BinaryExpression
            && let [left, right] = node.children.as_slice()
        {
            let operator = self.unit.source.text()[left.span.end..right.span.start].trim();
            if matches!(operator, "and" | "or") {
                let rhs = self.build(right, exits);
                let (yes, no) = if operator == "and" {
                    (rhs, exits.normal)
                } else {
                    (exits.normal, rhs)
                };
                return self.condition(left, yes, no, exits);
            }
        }
        match node.kind {
            SyntaxKind::IfStatement => return self.if_statement(node, exits),
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement => {
                return self.loop_statement(node, exits);
            }
            SyntaxKind::TryStatement => return self.try_statement(node, exits),
            SyntaxKind::SwitchStatement => return self.switch_statement(node, exits),
            SyntaxKind::MatchStatement | SyntaxKind::SelectStatement => {
                let mut entries = Vec::new();
                for case in &node.children {
                    if matches!(
                        case.kind,
                        SyntaxKind::MatchCase | SyntaxKind::SelectCase | SyntaxKind::ElseClause
                    ) {
                        entries.push(self.build(case, exits));
                    }
                }
                let gate = self.step(Step {
                    next: entries,
                    ..Step::default()
                });
                let selector = match node.kind {
                    SyntaxKind::MatchStatement => node.children.first(),
                    _ => None,
                };
                return if let Some(selector) = selector {
                    self.build(
                        selector,
                        Exits {
                            normal: gate,
                            ..exits
                        },
                    )
                } else {
                    gate
                };
            }
            SyntaxKind::CatchClause | SyntaxKind::FinallyClause => {
                let entry = self.sequence(&node.children, exits);
                self.clause_entries
                    .entry(node.span)
                    .or_default()
                    .push(entry);
                return entry;
            }
            SyntaxKind::Block | SyntaxKind::ElseClause => {
                return self.sequence(&node.children, exits);
            }
            _ => {}
        }
        let destination = match node.kind {
            SyntaxKind::ReturnStatement => exits.returned,
            SyntaxKind::ThrowStatement => exits.thrown,
            SyntaxKind::BreakStatement => exits.broken,
            SyntaxKind::ContinueStatement => exits.continued,
            _ => exits.normal,
        };
        let mut event = Step {
            span: Some(node.span),
            next: vec![destination],
            ..Step::default()
        };
        if node.kind == SyntaxKind::Name {
            event.reads.extend(self.identity(node));
        }
        // Borrowed inputs stay live through the call, not just their name reads.
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
        {
            if callee.kind == SyntaxKind::MemberExpression
                && let Some(receiver) = callee.children.first()
                && self.carries_loan(receiver)
            {
                self.reads(receiver, &mut event.reads);
            }
            for argument in &arguments.children {
                if self.carries_loan(argument) {
                    self.reads(argument, &mut event.reads);
                }
            }
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && let Some(target) = node
                .children
                .first()
                .filter(|target| target.kind == SyntaxKind::Name)
        {
            event.definition = self.identity(target).or_else(|| {
                declared_bindings_at_node(self.unit, node)
                    .next()
                    .map(|binding| binding.span)
            });
            if let Some(value) = node.children.last()
                && self.carries_loan(value)
            {
                self.loan_sources(value, &mut event.sources);
            }
        }
        if matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            let mut captures = BTreeSet::new();
            self.reads(node, &mut captures);
            captures.retain(|span| {
                span.file != node.span.file
                    || span.start < node.span.start
                    || span.end > node.span.end
            });
            event.reads.extend(captures.iter().copied());
            event.sources = captures;
            event.definition = Some(node.span);
            return self.step(event);
        }
        if node.kind == SyntaxKind::CallExpression
            && node
                .children
                .first()
                .and_then(|callee| function_contract_for_call(self.package, self.unit, callee))
                .is_some_and(|contract| contract.throws)
        {
            event.next.push(exits.thrown);
        }
        let mut next = self.step(event);
        for (index, child) in node.children.iter().enumerate().rev() {
            if super::super::scope_flow::is_flow_value_child(node, index, child) {
                next = self.build(
                    child,
                    Exits {
                        normal: next,
                        ..exits
                    },
                );
            }
        }
        next
    }
}

impl Flow {
    pub(super) fn new(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        function: &SyntaxNode,
    ) -> Self {
        let mut builder = Builder {
            package,
            unit,
            steps: vec![Step::default()],
            clause_entries: BTreeMap::new(),
        };
        let exits = Exits {
            normal: 0,
            returned: 0,
            thrown: 0,
            broken: 0,
            continued: 0,
        };
        let entry = function
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
            .map_or(0, |body| builder.build(body, exits));
        for index in 0..builder.steps.len() {
            for offset in 0..builder.steps[index].next.len() {
                let successor = builder.steps[index].next[offset];
                builder.steps[successor].previous.push(index);
            }
        }
        let mut flow = Self {
            live_after: vec![BTreeSet::new(); builder.steps.len()],
            steps: builder.steps,
            entry,
            aliases: Vec::new(),
            clause_entries: builder.clause_entries,
        };
        let mut live_before = vec![BTreeSet::new(); flow.steps.len()];
        loop {
            let mut changed = false;
            for (index, step) in flow.steps.iter().enumerate() {
                let mut live = BTreeSet::new();
                for &next in &step.next {
                    live.extend(live_before[next].iter().copied());
                }
                if live != flow.live_after[index] {
                    flow.live_after[index].clone_from(&live);
                    changed = true;
                }
                if let Some(definition) = step.definition {
                    live.remove(&definition);
                }
                live.extend(step.reads.iter().copied());
                if live != live_before[index] {
                    live_before[index] = live;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        flow.aliases = flow.storage_aliases();
        flow
    }

    fn storage_aliases(&self) -> Vec<Option<StorageAliases>> {
        let mut aliases = vec![None::<StorageAliases>; self.steps.len()];
        loop {
            let mut changed = false;
            for index in (0..self.steps.len()).rev() {
                let step = &self.steps[index];
                let mut incoming = (index == self.entry).then(StorageAliases::new);
                for &previous in &step.previous {
                    if let Some(state) = &aliases[previous] {
                        let merged = incoming.get_or_insert_with(StorageAliases::new);
                        for (binding, sources) in state {
                            merged.entry(*binding).or_default().extend(sources);
                        }
                    }
                }
                let Some(mut state) = incoming else {
                    continue;
                };
                if let Some(definition) = step.definition {
                    let mut sources = step.sources.clone();
                    for source in &step.sources {
                        if let Some(inherited) = state.get(source) {
                            sources.extend(inherited);
                        }
                    }
                    sources.remove(&definition);
                    state.remove(&definition);
                    if !sources.is_empty() {
                        state.insert(definition, sources);
                    }
                }
                if aliases[index].as_ref() != Some(&state) {
                    aliases[index] = Some(state);
                    changed = true;
                }
            }
            if !changed {
                return aliases;
            }
        }
    }

    pub(super) fn reads_after_without_rewrite(&self, statement: Span, binding: Span) -> bool {
        let mut pending = self
            .steps
            .iter()
            .filter(|step| {
                step.capture_region
                    .is_some_and(|region| region.start == statement.start)
            })
            .flat_map(|step| step.next.iter().map(|next| (*next, false)))
            .collect::<Vec<_>>();
        let mut visited = vec![[false; 2]; self.steps.len()];
        while let Some((index, mut outside)) = pending.pop() {
            let step = &self.steps[index];
            outside |= step.span.is_some_and(|span| {
                span.file == statement.file
                    && (span.start < statement.start || span.end > statement.end)
            });
            if std::mem::replace(&mut visited[index][usize::from(outside)], true) {
                continue;
            }
            if outside {
                if step.definition == Some(binding) {
                    continue;
                }
                if step.reads.contains(&binding) {
                    return true;
                }
            }
            pending.extend(step.next.iter().map(|next| (*next, outside)));
        }
        false
    }

    pub(super) fn reads_in_clause_without_rewrite(&self, clause: Span, binding: Span) -> bool {
        let Some(entries) = self.clause_entries.get(&clause) else {
            return false;
        };
        let mut pending = entries.clone();
        let mut visited = vec![false; self.steps.len()];
        while let Some(index) = pending.pop() {
            if std::mem::replace(&mut visited[index], true) {
                continue;
            }
            let step = &self.steps[index];
            if step.span.is_some_and(|span| {
                span.file != clause.file || span.start < clause.start || span.end > clause.end
            }) {
                continue;
            }
            if step.reads.contains(&binding) {
                return true;
            }
            if step.definition != Some(binding) {
                pending.extend(step.next.iter().copied());
            }
        }
        false
    }

    pub(super) fn live_origins(&self, origin: Span) -> BTreeSet<Span> {
        let mut reaching = vec![None::<BTreeSet<Span>>; self.steps.len()];
        loop {
            let mut changed = false;
            for index in (0..self.steps.len()).rev() {
                let step = &self.steps[index];
                let mut values = (index == self.entry).then(BTreeSet::new);
                for &previous in &step.previous {
                    if let Some(incoming) = &reaching[previous] {
                        values
                            .get_or_insert_with(BTreeSet::new)
                            .extend(incoming.iter().copied());
                    }
                }
                let Some(mut values) = values else {
                    continue;
                };
                if let Some(definition) = step.definition {
                    let carries = step.span.is_some_and(|span| {
                        span.file == origin.file
                            && span.start <= origin.start
                            && origin.end <= span.end
                    }) || step.sources.iter().any(|source| values.contains(source));
                    values.remove(&definition);
                    if carries {
                        values.insert(definition);
                    }
                }
                // Rust storage regions join at control-flow merges even when an
                // alias and a new lender were introduced on opposite branches.
                // Use the current alias state so a dominating rewrite still
                // kills dependencies of the replaced value.
                if let Some(aliases) = &self.aliases[index] {
                    loop {
                        let mut joined = false;
                        for (alias, sources) in aliases {
                            if !sources.is_disjoint(&values) {
                                joined |= values.insert(*alias);
                            }
                        }
                        if !joined {
                            break;
                        }
                    }
                }
                if reaching[index].as_ref() != Some(&values) {
                    reaching[index] = Some(values);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut live = BTreeSet::new();
        let mut captures = Vec::new();
        // A loan produced into hoisted storage inside a try also borrows the
        // closure's captures, even though it does not reach the entry step.
        // Keep its lenders stable throughout the body/catch capture boundary.
        captures.extend(self.steps.iter().filter_map(|step| {
            step.capture_region.filter(|region| {
                region.file == origin.file
                    && region.start <= origin.start
                    && origin.end <= region.end
            })
        }));
        for (index, step) in self.steps.iter().enumerate() {
            let Some(values) = &reaching[index] else {
                continue;
            };
            if !values.is_disjoint(&self.live_after[index]) || !values.is_disjoint(&step.reads) {
                live.extend(step.span);
                captures.extend(step.capture_region);
            }
        }
        for step in &self.steps {
            if let Some(span) = step.span
                && captures.iter().any(|region| {
                    region.file == span.file && region.start <= span.start && span.end <= region.end
                })
            {
                live.insert(span);
            }
        }
        live
    }
}
