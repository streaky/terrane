use super::super::prelude::*;
use crate::semantics::pipelines::{PipelineStage, pipeline_plan};

impl Emitter<'_> {
    pub(super) fn sequential_pipeline(&mut self, node: &SyntaxNode) -> String {
        let plan = pipeline_plan(self.unit, node, &self.unit.typed_bindings)
            .expect("validated pipeline contracts")
            .expect("pipeline types are resolved before lowering");
        let source_type = match &plan.source_type {
            ValueType::Reference(item)
            | ValueType::MutableReference(item)
            | ValueType::SharedReference(item) => item.value_type(),
            value_type => value_type.clone(),
        };
        let ValueType::List(source_item) = &source_type else {
            unreachable!("pipeline source is a list");
        };
        let (setup, limits) = self.pipeline_setup(node, &source_type, &plan.stages);
        let has_filter = plan
            .stages
            .iter()
            .any(|stage| matches!(stage, PipelineStage::Filter { .. }));
        let candidate_label = if has_filter {
            "'__terrane_pipeline_candidate: "
        } else {
            ""
        };
        let body = self.pipeline_body(node, &plan.stages, source_item.value_type());
        let exhausted = limits
            .iter()
            .map(|limit| format!("{limit} == 0"))
            .collect::<Vec<_>>()
            .join(" || ");
        let after = if limits.is_empty() {
            String::new()
        } else {
            format!("if {exhausted} {{ break; }} ")
        };
        let loop_body = format!(
            "for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {{ {candidate_label}{{ {body} }} {after} }}"
        );
        let loop_body = if limits.is_empty() {
            loop_body
        } else {
            format!("if !({exhausted}) {{ {loop_body} }}")
        };
        let ValueType::List(output_item) = plan.result_type else {
            unreachable!("collect produces a list");
        };
        let output_type = rust_value_type(self.package, output_item.value_type());
        let allocation = if has_filter {
            "Vec::new()".to_owned()
        } else {
            let mut capacity = "__terrane_pipeline_source.as_slice().len()".to_owned();
            for limit in &limits {
                write!(capacity, ".min({limit})").unwrap();
            }
            format!("Vec::with_capacity({capacity})")
        };
        format!(
            "{{ {setup}let mut __terrane_pipeline_output: Vec<{output_type}> = {allocation}; {loop_body} terrane_collection_support::List::new(__terrane_pipeline_output) }}"
        )
    }

    fn pipeline_setup(
        &mut self,
        node: &SyntaxNode,
        source_type: &ValueType,
        stages: &[PipelineStage],
    ) -> (String, Vec<String>) {
        let source = &node.children[0];
        let mut setup = String::new();
        if self.contains_guarded_read(source) || !self.read_borrow_is_stable_in(source, node) {
            let value = self.expression_as(source, source_type.clone());
            write!(setup, "let __terrane_pipeline_owner = {value}; let __terrane_pipeline_source = &__terrane_pipeline_owner; ").unwrap();
        } else {
            let (prelude, reference) = self.operand_reference_parts(source, false);
            write!(
                setup,
                "{prelude}let __terrane_pipeline_source = {reference}; "
            )
            .unwrap();
        }
        let mut limits = Vec::new();
        // Every callback operand remains live until traversal finishes, including
        // borrows established by earlier stages of the same pipeline.
        let mut borrowed_callbacks = BTreeSet::new();
        for stage in stages {
            match stage {
                PipelineStage::Filter { stage_node, .. }
                | PipelineStage::Map { stage_node, .. } => {
                    let argument = &node.children[*stage_node].children[1].children[0];
                    let (value, mutable) =
                        self.pipeline_callback(argument, node, &mut borrowed_callbacks);
                    let mutable = if mutable { "mut " } else { "" };
                    write!(
                        setup,
                        "let {mutable}__terrane_pipeline_callback_{stage_node} = {value}; "
                    )
                    .unwrap();
                }
                PipelineStage::Take { stage_node } => {
                    let argument = &node.children[*stage_node].children[1].children[0];
                    let value = self.pipeline_take(argument, *stage_node);
                    setup.push_str(&value);
                    limits.push(format!("__terrane_pipeline_remaining_{stage_node}"));
                }
            }
        }
        (setup, limits)
    }

    fn pipeline_take(&mut self, argument: &SyntaxNode, stage: usize) -> String {
        let mut argument = argument;
        while argument.kind == SyntaxKind::GroupExpression
            && let [inner] = argument.children.as_slice()
        {
            argument = inner;
        }
        let (prelude, count, zero) = if argument.kind == SyntaxKind::Name
            && self.value_type(argument) == Some(ValueType::Scalar(ScalarType::Int))
            && self.small_int_binding(argument).is_none()
        {
            let (prelude, reference) = self.operand_reference_parts(argument, false);
            (prelude, reference, "&terrane_int_support::Int::from(0_i64)")
        } else {
            (
                String::new(),
                self.expression_as(argument, ValueType::Scalar(ScalarType::Int)),
                "terrane_int_support::Int::from(0_i64)",
            )
        };
        // Finish this read (including any guard) before the next stage operand.
        format!(
            "let mut __terrane_pipeline_remaining_{stage} = {{ {prelude}let __terrane_pipeline_count_{stage} = {count}; if __terrane_pipeline_count_{stage} <= {zero} {{ 0 }} else {{ __terrane_pipeline_count_{stage}.as_usize().unwrap_or(usize::MAX) }} }}; "
        )
    }

    fn pipeline_callback(
        &mut self,
        argument: &SyntaxNode,
        scope: &SyntaxNode,
        borrowed_callbacks: &mut BTreeSet<crate::Span>,
    ) -> (String, bool) {
        let mut argument = argument;
        while argument.kind == SyntaxKind::GroupExpression
            && let [inner] = argument.children.as_slice()
        {
            argument = inner;
        }
        let direct = (argument.kind == SyntaxKind::Name)
            .then(|| self.contract_for_call(argument, false))
            .flatten()
            .filter(|contract| {
                contract.owner.is_none()
                    && !contract.is_anonymous
                    && contract.generic_parameters.is_empty()
                    && contract.captures.is_empty()
                    && contract.written_invocation_mode == InvocationMode::Shared
                    && self.package.units.iter().any(|unit| {
                        unit.source.id() == contract.span.file
                            && !unit.namespace.starts_with("/deps/")
                    })
            })
            .map(|contract| function_name(self.package, contract));
        if let Some(direct) = direct {
            return (direct, false);
        }
        let value_type = self.value_type(argument).expect("typed callback");
        if argument.kind == SyntaxKind::Name
            && let Some(binding) = self.local_typed_binding(argument)
            && self.flow_binding_type(binding) == value_type
            && !self.reference_backed(binding)
            && !self.contains_guarded_read(argument)
        {
            if let ValueType::Function(_, _, effects) = &value_type
                && effects.borrowed_environment
            {
                // The named carrier retains its planned last-use cleanup. Lend
                // it to the stage instead of moving it and later dropping it twice.
                let mutable = effects.modes.written == InvocationMode::Mutable;
                let (prelude, reference) = self.operand_reference_parts(argument, mutable);
                return (format!("{{ {prelude}{reference} }}"), false);
            }
            if !self.binding_value_is_reused(argument)
                && !self.non_consuming_capture_read(argument)
                && !self.binding_may_be_unassigned(binding)
                && !borrowed_callbacks.contains(&binding.span)
            {
                return (self.expression(argument), false);
            }
            if matches!(&value_type, ValueType::Function(_, _, effects)
                if effects.modes.written == InvocationMode::Shared)
                && self.read_borrow_is_stable_in(argument, scope)
            {
                borrowed_callbacks.insert(binding.span);
                let (prelude, reference) = self.operand_reference_parts(argument, false);
                return (format!("{{ {prelude}{reference} }}"), false);
            }
        }
        let mutable = matches!(&value_type, ValueType::Function(_, _, effects)
            if effects.borrowed_environment && effects.modes.written == InvocationMode::Mutable);
        (self.expression_as(argument, value_type), mutable)
    }

    fn pipeline_body(
        &self,
        node: &SyntaxNode,
        stages: &[PipelineStage],
        source_type: ValueType,
    ) -> String {
        let mut body = String::new();
        let mut item_type = source_type;
        let mut item = "__terrane_pipeline_item".to_owned();
        let mut borrowed = true;
        for stage in stages {
            match stage {
                PipelineStage::Take { stage_node } => {
                    write!(body, "__terrane_pipeline_remaining_{stage_node} -= 1; ").unwrap();
                }
                PipelineStage::Filter {
                    stage_node,
                    parameter,
                    effects,
                    ..
                }
                | PipelineStage::Map {
                    stage_node,
                    parameter,
                    effects,
                    ..
                } => {
                    let is_filter = matches!(stage, PipelineStage::Filter { .. });
                    let argument = Self::pipeline_item_argument(
                        &item,
                        self.pipeline_item_is_copy(&item_type),
                        parameter,
                        borrowed,
                        is_filter,
                    );
                    let callback = format!("__terrane_pipeline_callback_{stage_node}");
                    let call = match effects.modes.written {
                        InvocationMode::Shared => format!("{callback}({argument})"),
                        InvocationMode::Mutable => format!("{callback}.call(({argument},))"),
                        InvocationMode::Consuming => {
                            unreachable!("pipeline callbacks are repeatable")
                        }
                    };
                    let call = if effects.requires_throwing_abi() {
                        let site = self.error_site(&node.children[*stage_node]);
                        if self.try_completion {
                            format!("__terrane_traced_completion!({call}, {site})")
                        } else if self.propagate_errors {
                            format!("__terrane_traced_err({call}, {site})?")
                        } else {
                            format!("__terrane_traced({call}, {site})")
                        }
                    } else {
                        call
                    };
                    if is_filter {
                        write!(
                            body,
                            "if !({call}) {{ break '__terrane_pipeline_candidate; }} "
                        )
                        .unwrap();
                    } else if let PipelineStage::Map { result, .. } = stage {
                        item = format!("__terrane_pipeline_mapped_{stage_node}");
                        write!(body, "let {item} = {call}; ").unwrap();
                        item_type = result.value_type();
                        borrowed = false;
                    }
                }
            }
        }
        let output = if borrowed {
            if self.pipeline_item_is_copy(&item_type) {
                format!("*{item}")
            } else {
                format!("(*{item}).clone()")
            }
        } else {
            item
        };
        write!(body, "__terrane_pipeline_output.push({output}); ").unwrap();
        body
    }

    fn pipeline_item_is_copy(&self, value_type: &ValueType) -> bool {
        rust_value_is_copy(value_type) || self.package.native_value_is_copyable(value_type)
    }

    fn pipeline_item_argument(
        item: &str,
        copy: bool,
        expected: &ValueType,
        borrowed: bool,
        preserve: bool,
    ) -> String {
        if matches!(expected, ValueType::Reference(_)) {
            return if borrowed {
                item.to_owned()
            } else {
                format!("&{item}")
            };
        }
        if borrowed {
            if copy {
                format!("*{item}")
            } else {
                format!("(*{item}).clone()")
            }
        } else if preserve && !copy {
            format!("{item}.clone()")
        } else {
            item.to_owned()
        }
    }
}
