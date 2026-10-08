use std::fmt::Write as _;

use crate::{
    lowering::{
        dependencies::{projected_callback_input_expression, projected_callback_output_expression},
        emitter::Emitter,
        helpers::{effective_object_methods, rust_name, rust_object_type_name},
    },
    semantics::{DescriptorContract, InvocationMode, ObjectIdentity, ObjectKind},
};

impl Emitter<'_> {
    #[expect(
        clippy::too_many_lines,
        reason = "foreign interface method signatures and boundary conversions are emitted together"
    )]
    pub(super) fn projected_interface_implementation(
        &mut self,
        object: &DescriptorContract,
        interface_identity: &ObjectIdentity,
    ) {
        let Some(projected_item) = self
            .package
            .projection
            .item(&interface_identity.namespace, &interface_identity.name)
        else {
            return;
        };
        let crate::rust_interop::projection::ProjectedKind::Interface(interface) =
            &projected_item.kind
        else {
            return;
        };
        let trait_path = projected_item.rust_path.clone();
        let mut interface = interface.clone();
        let associated_application = interface_identity
            .application
            .as_deref()
            .map(|application| {
                crate::semantics::destination_projected_type(self.package, application)
                    .expect("validated projected associated type")
            });
        if let Some(application) = &associated_application {
            for method in &mut interface.methods {
                for parameter in &mut method.function.parameters {
                    parameter.ty.bind_associated(application);
                }
                method.function.result.bind_associated(application);
            }
        }
        let generic_bounds = interface.associated_type.as_ref().map(|associated| {
            let bounds = associated.bounds.join(" + ");
            if bounds.is_empty() {
                "'static".to_owned()
            } else {
                format!("'static + {bounds}")
            }
        });
        let generic_declaration =
            if object.kind == ObjectKind::Interface && interface_identity.application.is_none() {
                generic_bounds.as_ref().map_or_else(String::new, |bounds| {
                    format!("<TerraneAssociated: {bounds}>")
                })
            } else {
                String::new()
            };
        let mut class_type = rust_object_type_name(self.package, &object.identity);
        if !generic_declaration.is_empty() {
            class_type.push_str("<TerraneAssociated>");
        }
        let implementation_paths = if object.kind == ObjectKind::Interface {
            interface
                .supertraits
                .iter()
                .map(|supertrait| supertrait.rust_path.clone())
                .chain(std::iter::once(trait_path))
                .collect::<Vec<_>>()
        } else {
            vec![trait_path]
        };
        for implementation_path in implementation_paths {
            self.line(&format!(
                "impl{generic_declaration} {implementation_path} for {class_type} {{"
            ));
            self.indent += 1;
            if let Some(associated) = &interface.associated_type
                && associated
                    .rust_path
                    .rsplit_once("::")
                    .map(|(owner, _)| owner)
                    == Some(implementation_path.as_str())
            {
                let application = associated_application.as_ref().map_or_else(
                    || "TerraneAssociated".to_owned(),
                    crate::rust_interop::projection::ProjectedType::rust_type,
                );
                self.line(&format!("type {} = {application};", associated.name));
            }
            for projected in interface
                .methods
                .iter()
                .filter(|method| {
                    method.owner_rust_path.as_deref() == Some(implementation_path.as_str())
                })
                .cloned()
            {
                if object.kind == ObjectKind::Interface && projected.provided {
                    continue;
                }
                let method = projected.function;
                let Some(implementation) = effective_object_methods(self.unit, object)
                    .into_iter()
                    .find(|candidate| candidate.name == method.name && !candidate.is_static)
                else {
                    continue;
                };
                self.line_start();
                if method.is_async {
                    self.output.push_str("async ");
                }
                let receiver = match (method.receiver, implementation.written_invocation_mode) {
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Mutable,
                    ) => "mut self",
                    (Some(crate::rust_interop::projection::Receiver::Move), _) => "self",
                    (Some(crate::rust_interop::projection::Receiver::MutableBorrow), _) => {
                        "&mut self"
                    }
                    _ => "&self",
                };
                write!(self.output, "fn {}({receiver}", rust_name(&method.name))
                    .expect("writing to a string cannot fail");
                for parameter in &method.parameters {
                    let mut ty = parameter.ty.rust_type();
                    if parameter.borrowed {
                        ty = format!(
                            "&{}{}",
                            if parameter.mutable_borrow { "mut " } else { "" },
                            ty
                        );
                    }
                    write!(self.output, ", {}: {ty}", rust_name(&parameter.name))
                        .expect("writing to a string cannot fail");
                }
                self.output.push(')');
                let rust_result = method.result.rust_type();
                if method.result != crate::rust_interop::projection::ProjectedType::None {
                    write!(self.output, " -> {rust_result}")
                        .expect("writing to a string cannot fail");
                }
                self.output.push_str(" {\n");
                self.indent += 1;
                let arguments = method
                    .parameters
                    .iter()
                    .map(|parameter| {
                        projected_callback_input_expression(
                            &rust_name(&parameter.name),
                            &parameter.ty,
                            &parameter.ty.rust_type(),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let receiver = match (method.receiver, implementation.written_invocation_mode) {
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Shared,
                    ) => "&self",
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Mutable,
                    ) => "&mut self",
                    (
                        Some(crate::rust_interop::projection::Receiver::Move),
                        InvocationMode::Consuming,
                    ) => "self",
                    (_, InvocationMode::Shared) => "&*self",
                    (_, InvocationMode::Mutable) => "&mut *self",
                    (_, InvocationMode::Consuming) => {
                        unreachable!("validated projected interface mode compatibility")
                    }
                };
                let await_ = if method.is_async { ".await" } else { "" };
                let terrane_call = format!(
                    "<{class_type}>::{}({receiver}, {arguments}){await_}",
                    rust_name(&method.name)
                );
                let terrane_call = if method.is_async && interface.send {
                    match method.receiver {
                        Some(crate::rust_interop::projection::Receiver::Borrow) => format!(
                            "{{ let __terrane_receiver = self.clone(); __terrane_projected_async_entry(async move {{ <{class_type}>::{}(&__terrane_receiver, {arguments}).await }}).await }}",
                            rust_name(&method.name)
                        ),
                        Some(crate::rust_interop::projection::Receiver::MutableBorrow) => format!(
                            "{{ let mut __terrane_receiver = self.clone(); let (__terrane_output, __terrane_receiver) = __terrane_projected_async_entry(async move {{ let __terrane_output = <{class_type}>::{}(&mut __terrane_receiver, {arguments}).await; (__terrane_output, __terrane_receiver) }}).await; *self = __terrane_receiver; __terrane_output }}",
                            rust_name(&method.name)
                        ),
                        Some(crate::rust_interop::projection::Receiver::Move) => format!(
                            "__terrane_projected_async_entry(async move {{ {terrane_call} }}).await"
                        ),
                        None => terrane_call,
                    }
                } else {
                    terrane_call
                };
                let converted =
                    projected_callback_output_expression("__terrane_value", &method.result);
                if method.is_async {
                    self.line(&format!(
                    "let __terrane_boundary: Result<{rust_result}, crate::TerraneForeignError> = async {{ let __terrane_value = {terrane_call}; Ok({converted}) }}.await;"
                ));
                } else {
                    self.line(&format!(
                    "let __terrane_boundary: Result<{rust_result}, crate::TerraneForeignError> = (|| {{ let __terrane_value = {terrane_call}; Ok({converted}) }})();"
                ));
                }
                self.line(
                    "__terrane_boundary.unwrap_or_else(|error| panic!(\"{}\", error.render()))",
                );
                self.indent -= 1;
                self.line("}");
            }
            self.indent -= 1;
            self.line("}");
        }
    }
}
