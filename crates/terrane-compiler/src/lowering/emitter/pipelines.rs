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
        format!(
            "{{ {setup}let mut __terrane_pipeline_output: Vec<{output_type}> = Vec::new(); {loop_body} terrane_collection_support::List::new(__terrane_pipeline_output) }}"
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
        for stage in stages {
            match stage {
                PipelineStage::Filter { stage_node, .. }
                | PipelineStage::Map { stage_node, .. } => {
                    let argument = &node.children[*stage_node].children[1].children[0];
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
                    let value = direct.unwrap_or_else(|| {
                        self.expression_as(
                            argument,
                            self.value_type(argument).expect("typed callback"),
                        )
                    });
                    write!(
                        setup,
                        "let __terrane_pipeline_callback_{stage_node} = {value}; "
                    )
                    .unwrap();
                }
                PipelineStage::Take { stage_node } => {
                    let argument = &node.children[*stage_node].children[1].children[0];
                    let value = self.expression_as(argument, ValueType::Scalar(ScalarType::Int));
                    write!(setup, "let __terrane_pipeline_count_{stage_node} = {value}; let mut __terrane_pipeline_remaining_{stage_node} = if __terrane_pipeline_count_{stage_node} <= terrane_int_support::Int::from(0_i64) {{ 0 }} else {{ __terrane_pipeline_count_{stage_node}.as_usize().unwrap_or(usize::MAX) }}; ").unwrap();
                    limits.push(format!("__terrane_pipeline_remaining_{stage_node}"));
                }
            }
        }
        (setup, limits)
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
                        &item, &item_type, parameter, borrowed, is_filter,
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
            if rust_value_is_copy(&item_type) {
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

    fn pipeline_item_argument(
        item: &str,
        actual: &ValueType,
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
            if rust_value_is_copy(actual) {
                format!("*{item}")
            } else {
                format!("(*{item}).clone()")
            }
        } else if preserve && !rust_value_is_copy(actual) {
            format!("{item}.clone()")
        } else {
            item.to_owned()
        }
    }
}
