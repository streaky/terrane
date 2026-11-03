//! Stateful callable erasure must bind borrowed inputs at each invocation, not
//! in a stored Arguments tuple. Shapes are authority-only; payload/result types
//! remain generic, so this also works inside generic source functions.
use crate::semantics::{CallableParameterType, InvocationMode, SemanticPackage, ValueType};

pub(super) fn has_borrowed_inputs(parameters: &[CallableParameterType]) -> bool {
    parameters.iter().any(|parameter| {
        matches!(
            parameter.value_type_ref(),
            ValueType::Reference(_) | ValueType::MutableReference(_)
        )
    })
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct Shape {
    inputs: Vec<u8>,
    mutable: bool,
    borrowed_environment: bool,
}

impl Shape {
    pub(super) fn new(
        parameters: &[CallableParameterType],
        mode: InvocationMode,
        borrowed_environment: bool,
    ) -> Option<Self> {
        if mode == InvocationMode::Shared || !has_borrowed_inputs(parameters) {
            return None;
        }
        let inputs = parameters
            .iter()
            .map(|parameter| match parameter.value_type_ref() {
                ValueType::Reference(_) => 1,
                ValueType::MutableReference(_) => 2,
                _ => 0,
            })
            .collect::<Vec<_>>();
        Some(Self {
            inputs,
            mutable: mode == InvocationMode::Mutable,
            borrowed_environment,
        })
    }

    pub(super) fn name(&self) -> String {
        let mode = if self.mutable { "Mutable" } else { "Consuming" };
        let environment = if self.borrowed_environment {
            "Borrowed"
        } else {
            "Owned"
        };
        let authority = self
            .inputs
            .iter()
            .map(|kind| match kind {
                0 => 'V',
                1 => 'R',
                2 => 'M',
                _ => unreachable!(),
            })
            .collect::<String>();
        format!("Terrane{environment}{mode}Inputs{authority}")
    }

    pub(super) fn rust_type(
        &self,
        package: &SemanticPackage,
        parameters: &[CallableParameterType],
        output: &str,
    ) -> String {
        let arguments = parameters
            .iter()
            .map(|parameter| match parameter.value_type_ref() {
                ValueType::Reference(inner) | ValueType::MutableReference(inner) => {
                    super::helpers::rust_reference_element_type(package, inner)
                }
                _ => super::helpers::rust_callable_parameter_type(package, parameter),
            })
            .chain(std::iter::once(output.to_owned()))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{}<{}{arguments}>",
            self.name(),
            if self.borrowed_environment {
                "'_, "
            } else {
                ""
            }
        )
    }

    pub(super) fn runtime(&self) -> String {
        let types = (0..self.inputs.len())
            .map(|index| format!("A{index}"))
            .chain(std::iter::once("Output".to_owned()))
            .collect::<Vec<_>>()
            .join(", ");
        let arguments = self
            .inputs
            .iter()
            .enumerate()
            .map(|(index, kind)| {
                format!(
                    "{}A{index}",
                    match kind {
                        0 => "",
                        1 => "&",
                        2 => "&mut ",
                        _ => unreachable!(),
                    }
                )
            })
            .collect::<Vec<_>>();
        let tuple = match arguments.as_slice() {
            [argument] => format!("({argument},)"),
            _ => format!("({})", arguments.join(", ")),
        };
        let name = self.name();
        if !self.borrowed_environment && self.mutable {
            return super::mutable_callable::runtime(&name, &types, &tuple);
        }
        let lifetime = if self.borrowed_environment {
            "'env, "
        } else {
            ""
        };
        let bounds = if self.borrowed_environment {
            "+ 'env"
        } else {
            "+ Send + 'static"
        };
        let invocation = if self.mutable { "FnMut" } else { "FnOnce" };
        let receiver = if self.mutable { "&mut self" } else { "self" };
        format!(
            r"
        pub struct {name}<{lifetime}{types}> {{
            body: Box<dyn {invocation}({tuple}) -> Output {bounds}>,
        }}
        impl<{lifetime}{types}> {name}<{lifetime}{types}> {{
            fn new<Function>(function: Function) -> Self
            where Function: {invocation}({tuple}) -> Output {bounds} {{
                Self {{ body: Box::new(function) }}
            }}
            fn call({receiver}, arguments: {tuple}) -> Output {{ (self.body)(arguments) }}
        }}
        "
        )
    }
}
