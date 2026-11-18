use super::Emitter;
use crate::{
    ScalarType,
    lowering::helpers::{function_name, rust_object_type_name, rust_value_type, union_type_name},
    semantics::{FunctionContract, ValueType, interpolatable},
    syntax::{SyntaxKind, SyntaxNode},
};

impl Emitter<'_> {
    pub(super) fn borrowed_key_name(&self, mut node: &SyntaxNode) -> Option<&str> {
        while node.kind == SyntaxKind::GroupExpression
            && let [inner] = node.children.as_slice()
        {
            node = inner;
        }
        let binding = self.local_typed_binding(node)?;
        (Some(binding.span) == self.borrowed_interpolation_key)
            .then(|| self.binding_storage_name(binding))
    }

    pub(super) fn write_interpolation(
        &mut self,
        node: &SyntaxNode,
        key: Option<&SyntaxNode>,
        output: &str,
    ) -> String {
        let key = key.map_or("", |key| self.text(key)).to_owned();
        let ty = self
            .value_type(node)
            .expect("validated interpolation operand");
        let value = self.display_receiver(node);
        let write = self.write_interpolation_value(node, &ty, &value, &key, output);
        if self.reference_uses_shared_storage(node) {
            // Writers are statements: keep the upgraded owner local without
            // wrapping the statement in a value-producing expression.
            format!(
                "{{ let __terrane_owner = {}.upgrade().expect(\"reference expired\"); {write} }}",
                self.expression(node)
            )
        } else {
            write
        }
    }

    fn write_interpolation_value(
        &mut self,
        node: &SyntaxNode,
        ty: &ValueType,
        value: &str,
        key: &str,
        output: &str,
    ) -> String {
        match ty {
            ValueType::Reference(inner)
            | ValueType::MutableReference(inner)
            | ValueType::SharedReference(inner) => self.write_interpolation_value(
                node,
                inner.value_type_ref(),
                &format!("*({value})"),
                key,
                output,
            ),
            ValueType::Optional(inner) => {
                let present =
                    self.write_interpolation_value(node, inner, "__terrane_present", key, output);
                format!(
                    "match &({value}) {{ Some(__terrane_present) => {{ {present} }}, None => {output}.push_str(\"none\"), }}"
                )
            }
            ValueType::Union(reaching) => {
                let binding = self
                    .union_binding(node)
                    .expect("union interpolation has canonical storage");
                let name = union_type_name(&binding);
                let arms = self.union_arms(&binding);
                let cases = arms.iter().enumerate().map(|(index, arm)| {
                    if !reaching.contains(arm) {
                        return format!("{name}::Arm{index}(_) => unreachable!(\"flow-proven storage refinement\")");
                    }
                    let write = self.write_interpolation_value(node, arm, "__terrane_arm", key, output);
                    format!("{name}::Arm{index}(__terrane_arm) => {{ {write} }}")
                }).collect::<Vec<_>>().join(", ");
                format!("match &({value}) {{ {cases} }}")
            }
            ValueType::Scalar(_) => format!(
                "terrane_scalar_support::ScalarDisplay::write_scalar(&({value}), &mut {output});"
            ),
            _ => {
                let method = interpolatable::method(self.unit, ty, node.span.start)
                    .expect("validated interpolation protocol");
                let throws = method.throws;
                if matches!(ty, ValueType::TypeParameter(_))
                    || matches!(ty, ValueType::Object(owner) if interpolatable::is_protocol(owner))
                {
                    let protocol = format!(
                        "{}Protocol",
                        rust_object_type_name(self.package, interpolatable::identity())
                    );
                    let receiver_type = rust_value_type(self.package, ty.clone());
                    let call = format!(
                        "<{receiver_type} as {protocol}>::__terrane_write_interpolation(&({value}), {key:?}, &mut {output})"
                    );
                    let call = if throws {
                        self.display_call_error(&call, node)
                    } else {
                        call
                    };
                    format!("{call};")
                } else {
                    let method_name = function_name(self.package, method);
                    let argument = if interpolatable::borrows_key(self.unit, method) {
                        format!("{key:?}")
                    } else {
                        format!("String::from({key:?})")
                    };
                    let call = format!("({value}).{method_name}({argument})");
                    let call = if throws {
                        self.display_call_error(&call, node)
                    } else {
                        call
                    };
                    format!(
                        "{{ let __terrane_rendered = {call}; if {output}.capacity() == 0 {{ {output} = __terrane_rendered; }} else {{ {output}.push_str(&__terrane_rendered); }} }}"
                    )
                }
            }
        }
    }

    pub(super) fn emit_interpolation_writer_default(&mut self, methods: &[FunctionContract]) {
        let method = methods
            .iter()
            .find(|method| method.name == "interpolate")
            .expect("core interpolation method");
        let name = function_name(self.package, method);
        let (result, propagate, finish) = if method.throws {
            (" -> Result<(), TerraneError>", "?", "Ok(())")
        } else {
            ("", "", "")
        };
        self.line(&format!("fn __terrane_write_interpolation(&self, key: &str, output: &mut String){result} {{ let text = self.{name}(key){propagate}; if output.capacity() == 0 {{ *output = text; }} else {{ output.push_str(&text); }} {finish} }}"));
    }

    pub(super) fn emit_scalar_interpolation_implementations(
        &mut self,
        protocol: &str,
        methods: &[FunctionContract],
    ) {
        let method = methods
            .iter()
            .find(|method| method.name == "interpolate")
            .expect("core interpolation method");
        let name = function_name(self.package, method);
        let (result, text, writer_result, finish) = if method.throws {
            (
                "Result<String, TerraneError>",
                "Ok(terrane_scalar_support::scalar_text(self))",
                " -> Result<(), TerraneError>",
                "Ok(())",
            )
        } else {
            (
                "String",
                "terrane_scalar_support::scalar_text(self)",
                "",
                "",
            )
        };
        let facade = protocol
            .strip_suffix("Protocol")
            .expect("source protocol suffix");
        for scalar in ScalarType::ALL
            .into_iter()
            .filter(|scalar| *scalar != ScalarType::Bytes)
        {
            let ty = rust_value_type(self.package, ValueType::Scalar(scalar));
            self.line(&format!("impl {protocol} for {ty} {{"));
            self.indent += 1;
            self.line(&format!(
                "fn clone_box(&self) -> Box<dyn {protocol}> {{ Box::new(self.clone()) }}"
            ));
            self.line(&format!(
                "fn separate_box(&self) -> Box<dyn {protocol}> {{ Box::new(self.clone()) }}"
            ));
            self.line(&format!(
                "fn {name}(&self, _key: &str) -> {result} {{ {text} }}"
            ));
            self.line(&format!("fn __terrane_write_interpolation(&self, _key: &str, output: &mut String){writer_result} {{ terrane_scalar_support::ScalarDisplay::write_scalar(self, output); {finish} }}"));
            self.indent -= 1;
            self.line("}");
            self.line(&format!("impl From<{ty}> for {facade} {{ fn from(value: {ty}) -> Self {{ Self(Box::new(value)) }} }}"));
        }
        // Erased values remain usable as generic protocol arguments. Forwarding
        // clone_box also lets Option<T> erase its clone without a T: Clone bound.
        for (ty, receiver) in [
            ("Box<dyn ".to_owned() + protocol + ">", "**self"),
            (facade.to_owned(), "*self.0"),
        ] {
            self.line(&format!("impl {protocol} for {ty} {{"));
            self.indent += 1;
            self.line(&format!(
                "fn clone_box(&self) -> Box<dyn {protocol}> {{ ({receiver}).clone_box() }}"
            ));
            self.line(&format!(
                "fn separate_box(&self) -> Box<dyn {protocol}> {{ ({receiver}).separate_box() }}"
            ));
            self.line(&format!(
                "fn {name}(&self, key: &str) -> {result} {{ ({receiver}).{name}(key) }}"
            ));
            self.line(&format!("fn __terrane_write_interpolation(&self, key: &str, output: &mut String){writer_result} {{ ({receiver}).__terrane_write_interpolation(key, output) }}"));
            self.indent -= 1;
            self.line("}");
        }
        let none = if method.throws {
            "Ok(terrane_scalar_support::scalar_text(&()))"
        } else {
            "terrane_scalar_support::scalar_text(&())"
        };
        self.line(&format!("impl<T: {protocol}> {protocol} for Option<T> {{"));
        self.indent += 1;
        self.line(&format!("fn clone_box(&self) -> Box<dyn {protocol}> {{ Box::new(self.as_ref().map(|value| value.clone_box())) }}"));
        self.line(&format!("fn separate_box(&self) -> Box<dyn {protocol}> {{ Box::new(self.as_ref().map(|value| value.separate_box())) }}"));
        self.line(&format!("fn {name}(&self, key: &str) -> {result} {{ match self {{ Some(value) => value.{name}(key), None => {none} }} }}"));
        self.line(&format!("fn __terrane_write_interpolation(&self, key: &str, output: &mut String){writer_result} {{ match self {{ Some(value) => value.__terrane_write_interpolation(key, output), None => {{ terrane_scalar_support::ScalarDisplay::write_scalar(&(), output); {finish} }} }} }}"));
        self.indent -= 1;
        self.line("}");
        self.line(&format!("impl<T: {protocol} + 'static> From<Option<T>> for {facade} {{ fn from(value: Option<T>) -> Self {{ Self(Box::new(value)) }} }}"));
    }
}
