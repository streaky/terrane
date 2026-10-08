use super::super::prelude::*;

fn substitute_projected_generic(
    ty: &crate::rust_interop::projection::ProjectedType,
    substitutions: &BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
) -> crate::rust_interop::projection::ProjectedType {
    use crate::rust_interop::projection::ProjectedType;
    match ty {
        ProjectedType::Generic(name) => substitutions
            .get(name)
            .cloned()
            .unwrap_or_else(|| ty.clone()),
        ProjectedType::Optional(inner) => {
            ProjectedType::Optional(Box::new(substitute_projected_generic(inner, substitutions)))
        }
        ProjectedType::Sequence { rust_path, item } => ProjectedType::Sequence {
            rust_path: rust_path.clone(),
            item: Box::new(substitute_projected_generic(item, substitutions)),
        },
        ProjectedType::Set {
            rust_path,
            item,
            ordered,
        } => ProjectedType::Set {
            rust_path: rust_path.clone(),
            item: Box::new(substitute_projected_generic(item, substitutions)),
            ordered: *ordered,
        },
        ProjectedType::Mapping {
            rust_path,
            key,
            value,
            ordered,
        } => ProjectedType::Mapping {
            rust_path: rust_path.clone(),
            key: Box::new(substitute_projected_generic(key, substitutions)),
            value: Box::new(substitute_projected_generic(value, substitutions)),
            ordered: *ordered,
        },
        ProjectedType::Tuple(items) => ProjectedType::Tuple(
            items
                .iter()
                .map(|item| substitute_projected_generic(item, substitutions))
                .collect(),
        ),
        ProjectedType::AsyncIterationStep(item) => ProjectedType::AsyncIterationStep(Box::new(
            substitute_projected_generic(item, substitutions),
        )),
        _ => ty.clone(),
    }
}

pub(super) fn value_type_uses_parameter(value_type: &ValueType, name: &str) -> bool {
    match value_type {
        ValueType::TypeParameter(parameter) => parameter == name,
        ValueType::Optional(inner) => value_type_uses_parameter(inner, name),
        ValueType::Union(arms) => arms.iter().any(|arm| value_type_uses_parameter(arm, name)),
        ValueType::Object(identity) => identity
            .type_arguments
            .iter()
            .any(|argument| value_type_uses_parameter(argument, name)),
        ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::AsyncIterationStep(item)
        | ValueType::ChannelPair(item)
        | ValueType::ChannelSender(item)
        | ValueType::ChannelReceiver(item)
        | ValueType::ChannelSendOutcome(item)
        | ValueType::ChannelReceiveOutcome(item)
        | ValueType::DocumentDecodeOutcome(item)
        | ValueType::Task(item, _)
        | ValueType::ScopedTask(item, _)
        | ValueType::TaskOutcome(item)
        | ValueType::Reference(item)
        | ValueType::SharedReference(item)
        | ValueType::UnorderedSet(item) => value_type_uses_parameter(item.value_type_ref(), name),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_uses_parameter(key.value_type_ref(), name)
                || value_type_uses_parameter(value.value_type_ref(), name)
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            parameters
                .iter()
                .any(|parameter| value_type_uses_parameter(&parameter.value_type(), name))
                || value_type_uses_parameter(result.value_type_ref(), name)
        }
        _ => false,
    }
}

fn enum_phantom_parameters<'a>(
    descriptor: Option<&'a DescriptorContract>,
    contract: &SourceEnumContract,
) -> Vec<&'a GenericParameterContract> {
    descriptor
        .into_iter()
        .flat_map(|descriptor| &descriptor.generic_parameters)
        .filter(|parameter| {
            !contract
                .variants
                .iter()
                .flat_map(|variant| &variant.payload)
                .any(|field| value_type_uses_parameter(&field.value_type, &parameter.name))
        })
        .collect()
}

fn enum_marker_type(parameters: &[&GenericParameterContract]) -> Option<String> {
    (!parameters.is_empty()).then(|| {
        format!(
            "std::marker::PhantomData<fn({})>",
            parameters
                .iter()
                .map(|parameter| rust_type_parameter_name(&parameter.name))
                .collect::<Vec<_>>()
                .join(", ")
        )
    })
}

fn enum_marker_value(parameters: &[&GenericParameterContract]) -> Option<String> {
    (!parameters.is_empty()).then(|| "std::marker::PhantomData".to_owned())
}

fn enum_marker_pattern(parameters: &[&GenericParameterContract]) -> Option<&'static str> {
    (!parameters.is_empty()).then_some("_")
}

fn value_type_contains_nonclone_object(package: &SemanticPackage, value_type: &ValueType) -> bool {
    match value_type {
        ValueType::Optional(inner) => value_type_contains_nonclone_object(package, inner),
        ValueType::Object(identity) => {
            package
                .units
                .iter()
                .flat_map(|unit| &unit.descriptors)
                .find(|descriptor| descriptor.identity.base() == identity.base())
                .is_some_and(|descriptor| descriptor.resource_owning)
                || identity
                    .type_arguments
                    .iter()
                    .any(|argument| value_type_contains_nonclone_object(package, argument))
        }
        ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::AsyncIterationStep(item)
        | ValueType::ChannelPair(item)
        | ValueType::ChannelSender(item)
        | ValueType::ChannelReceiver(item)
        | ValueType::ChannelSendOutcome(item)
        | ValueType::ChannelReceiveOutcome(item)
        | ValueType::DocumentDecodeOutcome(item)
        | ValueType::Task(item, _)
        | ValueType::ScopedTask(item, _)
        | ValueType::TaskOutcome(item)
        | ValueType::Reference(item)
        | ValueType::SharedReference(item)
        | ValueType::UnorderedSet(item) => {
            value_type_contains_nonclone_object(package, item.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_contains_nonclone_object(package, key.value_type_ref())
                || value_type_contains_nonclone_object(package, value.value_type_ref())
        }
        _ => false,
    }
}

impl<'a> Emitter<'a> {
    pub(super) fn enum_designator(&self, node: &SyntaxNode) -> Option<&'a DescriptorContract> {
        let symbol = self
            .package
            .resolve_name_at(self.unit, node.span.start, self.text(node))?;
        self.unit.descriptors.iter().find(|descriptor| {
            descriptor.kind == ObjectKind::Enum
                && symbol.identity
                    == format!(
                        "{}::{}",
                        descriptor.identity.namespace, descriptor.identity.name
                    )
        })
    }
    pub(super) fn enum_declaration(&mut self, node: &SyntaxNode) {
        let Some(contract) = self
            .unit
            .source_enums
            .iter()
            .find(|contract| contract.span == node.span)
        else {
            return;
        };
        let descriptor = self.unit.descriptors.iter().find(|descriptor| {
            descriptor.identity.namespace == contract.identity.namespace
                && descriptor.identity.name == contract.identity.name
        });
        let parameters = descriptor.map_or(&[][..], |descriptor| {
            descriptor.generic_parameters.as_slice()
        });
        let (declarations, generic_use) = rust_generic_parameters(self.package, parameters);
        let phantom_parameters = enum_phantom_parameters(descriptor, contract);
        let marker_type = enum_marker_type(&phantom_parameters);
        let name = rust_object_type_name(self.package, &contract.identity);
        let cloneable = contract
            .variants
            .iter()
            .flat_map(|variant| &variant.payload)
            .all(|field| !value_type_contains_nonclone_object(self.package, &field.value_type));
        if cloneable && phantom_parameters.is_empty() {
            self.line("#[derive(Clone)]");
        }
        self.line(&format!("pub enum {name}{declarations} {{"));
        self.indent += 1;
        for variant in &contract.variants {
            let mut payload = variant
                .payload
                .iter()
                .map(|field| rust_value_type(self.package, field.value_type.clone()))
                .collect::<Vec<_>>();
            if let Some(marker) = &marker_type {
                payload.push(marker.clone());
            }
            let body = if payload.is_empty() {
                String::new()
            } else {
                format!("({})", payload.join(", "))
            };
            self.line(&format!("{}{},", rust_object_name(&variant.name), body));
        }
        self.indent -= 1;
        self.line("}");
        if cloneable && !phantom_parameters.is_empty() {
            self.line(&format!(
                "impl{declarations} Clone for {name}{generic_use} {{"
            ));
            self.indent += 1;
            self.line("fn clone(&self) -> Self {");
            self.indent += 1;
            self.line("match self {");
            self.indent += 1;
            for variant in &contract.variants {
                let variant_name = rust_object_name(&variant.name);
                let fields = (0..variant.payload.len())
                    .map(|i| format!("__field_{i}"))
                    .collect::<Vec<_>>();
                let mut patterns = fields.clone();
                if marker_type.is_some() {
                    patterns.push("_".to_owned());
                }
                let pat = if patterns.is_empty() {
                    variant_name.clone()
                } else {
                    format!("{variant_name}({})", patterns.join(", "))
                };
                let mut values = fields
                    .iter()
                    .map(|field| format!("{field}.clone()"))
                    .collect::<Vec<_>>();
                if marker_type.is_some() {
                    values.push("std::marker::PhantomData".to_owned());
                }
                let value = if values.is_empty() {
                    format!("Self::{variant_name}")
                } else {
                    format!("Self::{variant_name}({})", values.join(", "))
                };
                self.line(&format!("Self::{pat} => {value},"));
            }
            self.indent -= 1;
            self.line("}");
            self.indent -= 1;
            self.line("}");
            self.indent -= 1;
            self.line("}");
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "Native enum construction retains one recipe for argument evaluation, language conversion, and carrier adaptation"
    )]
    pub(super) fn source_enum_construction(
        &mut self,
        designator: &SyntaxNode,
        arguments: &SyntaxNode,
        result: &SyntaxNode,
    ) -> Option<String> {
        let SyntaxKind::StaticMemberExpression = designator.kind else {
            return None;
        };
        let [enum_name, variant_name] = designator.children.as_slice() else {
            return None;
        };
        let descriptor = self.enum_designator(enum_name)?;
        let contract = self.unit.source_enums.iter().find(|contract| {
            contract.identity.namespace == descriptor.identity.namespace
                && contract.identity.name == descriptor.identity.name
        });
        let Some(contract) = contract else {
            let item = self
                .package
                .projection
                .item(&descriptor.identity.namespace, &descriptor.identity.name)?;
            let crate::rust_interop::projection::ProjectedKind::Enum {
                variants,
                generic_parameters,
                ..
            } = &item.kind
            else {
                return None;
            };
            let selected_identity = match self.value_type(result) {
                Some(ValueType::Object(identity)) => identity,
                Some(ValueType::Optional(inner)) => match *inner {
                    ValueType::Object(identity) => identity,
                    _ => descriptor.identity.clone(),
                },
                _ => descriptor.identity.clone(),
            };
            let selected_types = generic_parameters
                .iter()
                .filter_map(|parameter| {
                    selected_identity
                        .native_arguments
                        .get(&parameter.name)
                        .and_then(|value_type| {
                            crate::semantics::destination_projected_type(self.package, value_type)
                                .ok()
                                .map(|projected| (parameter.name.clone(), projected))
                        })
                })
                .collect::<BTreeMap<_, _>>();
            let source_types = generic_parameters
                .iter()
                .zip(&selected_identity.type_arguments)
                .map(|(parameter, value_type)| (parameter.name.clone(), value_type.clone()))
                .collect::<BTreeMap<_, _>>();
            let variant = variants
                .iter()
                .find(|variant| variant.name == self.text(variant_name))?;
            if !variant.constructible || variant.unavailable_reason.is_some() {
                return None;
            }
            let mut setup = Vec::new();
            let mut values = BTreeMap::new();
            for (index, argument) in arguments.children.iter().enumerate() {
                let value = argument.children.last()?;
                let field = match variant.style {
                    crate::rust_interop::projection::ProjectedEnumVariantStyle::Tuple => {
                        if argument.children.len() != 1 {
                            return None;
                        }
                        variant.fields.get(index)?
                    }
                    crate::rust_interop::projection::ProjectedEnumVariantStyle::Struct => {
                        let name = argument.children.first()?;
                        if argument.children.len() != 2 {
                            return None;
                        }
                        variant
                            .fields
                            .iter()
                            .find(|field| field.name == self.text(name))?
                    }
                    crate::rust_interop::projection::ProjectedEnumVariantStyle::Unit => {
                        return None;
                    }
                };
                let input = format!("__terrane_native_enum_input_{index}");
                let source_type =
                    crate::semantics::projected_enum_payload_type(self.package, &field.ty).map(
                        |value_type| {
                            crate::semantics::substitute_value_type(&value_type, &source_types)
                        },
                    )?;
                setup.push(format!(
                    "let {input} = {};",
                    self.expression_as(value, source_type)
                ));
                let projected_type = substitute_projected_generic(&field.ty, &selected_types);
                let expression = if field.ty.contains_open_generic() {
                    let conversion = projected_argument_expression(&input, &projected_type);
                    self.fallible(
                        format!(
                            "(|| -> Result<_, crate::TerraneForeignError> {{ Ok({conversion}) }})()"
                        ),
                        result,
                    )
                } else {
                    match field.conversion {
                        crate::rust_interop::projection::ProjectedFieldConversion::Identity
                            if !projected_type.has_identity_representation() => {
                                let conversion = projected_argument_expression(&input, &projected_type);
                                self.fallible(
                                    format!("(|| -> Result<_, crate::TerraneForeignError> {{ Ok({conversion}) }})()"),
                                    result,
                                )
                            }
                        crate::rust_interop::projection::ProjectedFieldConversion::Identity => input,
                        crate::rust_interop::projection::ProjectedFieldConversion::OptionalOwned => format!("({input}).map(Into::into)"),
                        crate::rust_interop::projection::ProjectedFieldConversion::StringBorrow => format!("({input}).as_str()"),
                        crate::rust_interop::projection::ProjectedFieldConversion::OptionalStringBorrow
                        | crate::rust_interop::projection::ProjectedFieldConversion::OptionalSliceBorrow => format!("({input}).as_deref()"),
                        crate::rust_interop::projection::ProjectedFieldConversion::SliceBorrow => format!("({input}).as_slice()"),
                    }
                };
                let temporary = format!("__terrane_native_enum_payload_{index}");
                setup.push(format!("let {temporary} = {expression};"));
                values.insert(field.name.as_str(), temporary);
            }
            if arguments.children.len() != variant.fields.len() {
                return None;
            }
            let values = variant
                .fields
                .iter()
                .map(|field| values.get(field.name.as_str()).cloned())
                .collect::<Option<Vec<_>>>()?;
            let type_arguments = generic_parameters
                .iter()
                .map(|parameter| {
                    selected_types
                        .get(&parameter.name)
                        .map(crate::rust_interop::projection::ProjectedType::rust_type)
                })
                .collect::<Option<Vec<_>>>()?;
            let base_path = item
                .rust_path
                .split_once('<')
                .map_or(item.rust_path.as_str(), |(base, _)| base);
            let path = format!(
                "{}{}::{}",
                base_path,
                if type_arguments.is_empty() {
                    String::new()
                } else {
                    format!("::<{}>", type_arguments.join(", "))
                },
                variant.name,
            );
            let construction = match variant.style {
                crate::rust_interop::projection::ProjectedEnumVariantStyle::Unit => path,
                crate::rust_interop::projection::ProjectedEnumVariantStyle::Tuple => {
                    format!("{path}({})", values.join(", "))
                }
                crate::rust_interop::projection::ProjectedEnumVariantStyle::Struct => {
                    let fields = variant
                        .fields
                        .iter()
                        .zip(values)
                        .map(|(field, value)| format!("{}: {value}", field.rust_name))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{path} {{ {fields} }}")
                }
            };
            return Some(format!("{{ {} {construction} }}", setup.join(" ")));
        };
        self.source_enum_variant_construction(descriptor, contract, variant_name, arguments, result)
    }

    fn source_enum_variant_construction(
        &mut self,
        descriptor: &DescriptorContract,
        contract: &SourceEnumContract,
        variant_name: &SyntaxNode,
        arguments: &SyntaxNode,
        result: &SyntaxNode,
    ) -> Option<String> {
        let variant = contract
            .variants
            .iter()
            .find(|variant| variant.name == self.text(variant_name))?;
        let identity = match self.value_type(result) {
            Some(ValueType::Optional(inner)) => match *inner {
                ValueType::Object(identity) => identity,
                _ => descriptor.identity.clone(),
            },
            Some(ValueType::Object(identity)) => identity,
            _ => descriptor.identity.clone(),
        };
        let substitutions = descriptor
            .generic_parameters
            .iter()
            .zip(&identity.type_arguments)
            .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
            .collect();
        let mut values = BTreeMap::new();
        let mut setup = Vec::new();
        for (index, argument) in arguments.children.iter().enumerate() {
            let Some(name) = argument.children.first() else {
                continue;
            };
            let Some(value) = argument.children.last() else {
                continue;
            };
            let field = variant
                .payload
                .iter()
                .find(|field| field.name == self.text(name))?;
            let temporary = format!("__terrane_enum_payload_{index}");
            setup.push(format!(
                "let {temporary} = {};",
                self.expression_as(
                    value,
                    crate::semantics::substitute_value_type(&field.value_type, &substitutions)
                )
            ));
            values.insert(field.name.as_str(), temporary);
        }
        let mut values = variant
            .payload
            .iter()
            .map(|field| values.get(field.name.as_str()).cloned())
            .collect::<Option<Vec<_>>>()?;
        let phantom_parameters = enum_phantom_parameters(Some(descriptor), contract);
        if let Some(marker) = enum_marker_value(&phantom_parameters) {
            values.push(marker);
        }
        let variant_path = format!(
            "{}::{}",
            rust_source_type_application(self.package, &identity).replacen('<', "::<", 1),
            rust_object_name(&variant.name),
        );
        let construct = if values.is_empty() {
            variant_path
        } else {
            format!("{variant_path}({})", values.join(", "))
        };
        Some(if setup.is_empty() {
            construct
        } else {
            format!("{{ {} {construct} }}", setup.join(" "))
        })
    }

    pub(super) fn enum_match_statement(&mut self, node: &SyntaxNode) {
        let Some(scrutinee) = node.children.first() else {
            return;
        };
        let borrowed = scrutinee.kind == SyntaxKind::UnaryExpression
            && self.unary_operator(scrutinee).as_deref() == Some("ref");
        let consuming = scrutinee.kind == SyntaxKind::UnaryExpression
            && self.unary_operator(scrutinee).as_deref() == Some("move");
        let operand = if borrowed || consuming {
            scrutinee.children.last().unwrap_or(scrutinee)
        } else {
            scrutinee
        };
        let optional = matches!(self.value_type(operand), Some(ValueType::Optional(_)));
        let value_type = self.value_type(operand);
        let value = self.expression(operand);
        let value = if !borrowed
            && !consuming
            && operand.kind == SyntaxKind::Name
            && self
                .package
                .resolve_name_at(self.unit, operand.span.start, self.text(operand))
                .is_some_and(|symbol| symbol.kind == SymbolKind::Binding && !symbol.global)
            && value_type.as_ref().is_some_and(|value_type| {
                !crate::semantics::application_is_resource_owning(self.package, value_type)
            }) {
            format!("({value}).clone()")
        } else {
            value
        };
        if borrowed {
            self.line(&format!("match &({value}) {{"));
        } else {
            let matched = format!("__terrane_match_value_{}", node.span.start);
            self.line(&format!("let {matched} = {value};"));
            self.line(&format!("match {matched} {{"));
        }
        self.emit_enum_match_cases(node, operand, borrowed, optional, value_type.as_ref());
        self.indent -= 1;
        self.line("}");
    }
    #[expect(
        clippy::too_many_lines,
        reason = "Each case keeps its pattern, scoped payload conversion, and body under one emitted ownership boundary"
    )]
    fn emit_enum_match_cases(
        &mut self,
        node: &SyntaxNode,
        operand: &SyntaxNode,
        borrowed: bool,
        optional: bool,
        value_type: Option<&ValueType>,
    ) {
        let unit = self.unit;
        self.indent += 1;
        for case in node.children.iter().skip(1) {
            let Some(selector) = case.children.first() else {
                continue;
            };
            let payload = case
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::ParameterList);
            let bindings =
                payload
                    .into_iter()
                    .flat_map(|list| &list.children)
                    .map(|binding| {
                        let name = &unit.source.text()[binding.span.start..binding.span.end];
                        let identity = self
                            .unit
                            .flow_binding_ids
                            .get(&(binding.span.file, binding.span.start, binding.span.end))
                            .copied()
                            .unwrap_or(binding.span);
                        let used =
                            case.children
                                .iter()
                                .filter(|child| child.kind == SyntaxKind::Block)
                                .any(|body| match_binding_used(body, &self.unit.source, name))
                                || self.unit.flow_availability.keys().any(|key| {
                                    self.unit.flow_binding_ids.get(key) == Some(&identity)
                                });
                        if used { name } else { "_" }
                    })
                    .collect::<Vec<_>>();
            let mut native_payload_conversions = Vec::new();
            let mut none_pattern = false;
            let variant_pattern = if selector.kind == SyntaxKind::MatchCatchAll {
                "_".to_owned()
            } else if selector.kind == SyntaxKind::Name {
                let name = self.text(selector);
                none_pattern = name == "none";
                match name {
                    "none" => "None".to_owned(),
                    _ => continue,
                }
            } else if selector.kind == SyntaxKind::StaticMemberExpression {
                let [enum_name, variant] = selector.children.as_slice() else {
                    continue;
                };
                let Some(descriptor) = self
                    .enum_designator(enum_name)
                    .or_else(|| self.class_designator(enum_name))
                else {
                    continue;
                };
                let variant_name = self.text(variant);
                if let Some(contract) = self
                    .unit
                    .source_enums
                    .iter()
                    .find(|contract| contract.identity.base() == descriptor.identity.base())
                {
                    let enum_identity = match self.value_type(operand) {
                        Some(ValueType::Object(identity))
                            if identity.base() == contract.identity.base() =>
                        {
                            identity
                        }
                        Some(ValueType::Optional(inner)) => match *inner {
                            ValueType::Object(identity)
                                if identity.base() == contract.identity.base() =>
                            {
                                identity
                            }
                            _ => contract.identity.clone(),
                        },
                        _ => contract.identity.clone(),
                    };
                    let enum_type =
                        rust_source_type_application(self.package, &enum_identity.base());
                    let tag = format!("{enum_type}::{}", rust_object_name(variant_name));
                    let phantom_parameters = enum_phantom_parameters(
                        self.unit.descriptors.iter().find(|candidate| {
                            candidate.identity.base() == contract.identity.base()
                        }),
                        contract,
                    );
                    let mut payload_bindings = bindings
                        .iter()
                        .map(|name| {
                            if *name == "_" {
                                "_".to_owned()
                            } else {
                                rust_name(name)
                            }
                        })
                        .collect::<Vec<_>>();
                    if let Some(marker) = enum_marker_pattern(&phantom_parameters) {
                        payload_bindings.push(marker.to_owned());
                    }
                    if payload_bindings.is_empty() {
                        tag
                    } else {
                        format!("{tag}({})", payload_bindings.join(", "))
                    }
                } else if let Some(item) = self
                    .package
                    .projection
                    .item(&descriptor.identity.namespace, &descriptor.identity.name)
                {
                    if let crate::rust_interop::projection::ProjectedKind::Enum {
                        variants,
                        generic_parameters,
                        ..
                    } = &item.kind
                    {
                        let Some(variant) = variants
                            .iter()
                            .find(|candidate| candidate.name == variant_name)
                        else {
                            continue;
                        };
                        if borrowed {
                            let selected_identity = match &value_type {
                                Some(ValueType::Object(identity)) => Some(identity),
                                Some(ValueType::Optional(inner)) => match inner.as_ref() {
                                    ValueType::Object(identity) => Some(identity),
                                    _ => None,
                                },
                                _ => None,
                            };
                            let mut selected_types = BTreeMap::new();
                            if let Some(identity) = selected_identity {
                                for parameter in generic_parameters {
                                    if let Some(argument) =
                                        identity.native_arguments.get(&parameter.name)
                                        && let Ok(projected) =
                                            crate::semantics::destination_projected_type(
                                                self.package,
                                                argument,
                                            )
                                    {
                                        selected_types.insert(parameter.name.clone(), projected);
                                    }
                                }
                            }
                            for (field, binding) in variant.fields.iter().zip(&bindings) {
                                if *binding == "_" {
                                    continue;
                                }
                                let projected =
                                    substitute_projected_generic(&field.ty, &selected_types);
                                // Fixed-width payloads retain their native borrowed
                                // representation. Unbounded integers require a value conversion.
                                if matches!(
                                    projected,
                                    crate::rust_interop::projection::ProjectedType::Int
                                ) {
                                    let name = rust_name(binding);
                                    let converted = projected_result_expression(
                                        &format!("*{name}"),
                                        &projected,
                                    );
                                    native_payload_conversions
                                        .push(format!("let {name} = {converted};"));
                                }
                            }
                        }
                        let base_path = item
                            .rust_path
                            .split_once('<')
                            .map_or(item.rust_path.as_str(), |(base, _)| base);
                        let tag = format!("{base_path}::{}", variant.name);
                        match variant.style {
                            crate::rust_interop::projection::ProjectedEnumVariantStyle::Unit => tag,
                            crate::rust_interop::projection::ProjectedEnumVariantStyle::Tuple => {
                                format!(
                                    "{tag}({})",
                                    bindings
                                        .iter()
                                        .map(|name| if *name == "_" {
                                            "_".to_owned()
                                        } else {
                                            rust_name(name)
                                        })
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                )
                            }
                            crate::rust_interop::projection::ProjectedEnumVariantStyle::Struct => {
                                let fields = variant
                                    .fields
                                    .iter()
                                    .zip(&bindings)
                                    .map(|(field, binding)| {
                                        let name = if *binding == "_" {
                                            "_".to_owned()
                                        } else {
                                            rust_name(binding)
                                        };
                                        if name == field.rust_name {
                                            name
                                        } else {
                                            format!("{}: {name}", field.rust_name)
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                format!("{tag} {{ {fields} }}")
                            }
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let pattern = if none_pattern || variant_pattern == "_" {
                variant_pattern
            } else if optional {
                format!("Some({variant_pattern})")
            } else {
                variant_pattern
            };
            self.line(&format!("{pattern} => {{"));
            self.indent += 1;
            for conversion in native_payload_conversions {
                self.line(&conversion);
            }
            for (parameter, name) in payload
                .into_iter()
                .flat_map(|list| &list.children)
                .zip(&bindings)
            {
                if *name == "_" {
                    continue;
                }
                let Some(binding) = self
                    .unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == parameter.span)
                else {
                    continue;
                };
                let identity = self
                    .unit
                    .flow_binding_ids
                    .get(&(
                        parameter.span.file,
                        parameter.span.start,
                        parameter.span.end,
                    ))
                    .copied()
                    .unwrap_or(binding.span);
                let storage = self
                    .unit
                    .typed_bindings
                    .iter()
                    .find(|candidate| candidate.span == identity)
                    .unwrap_or(binding);
                if !self.active_function_bindings.contains(&storage.span) {
                    continue;
                }
                let mut value = rust_name(name);
                if let ValueType::Union(arms) = self.flow_binding_type(storage) {
                    let index = arms
                        .iter()
                        .position(|arm| arm == &binding.value_type)
                        .expect("pattern payload belongs to its flow carrier");
                    value = format!("{}::Arm{index}({value})", union_type_name(storage));
                }
                if self.binding_may_be_unassigned(storage) {
                    self.line(&format!(
                        "let _ = {}.insert({value});",
                        rust_binding_name(storage)
                    ));
                } else {
                    self.line(&format!("{} = {value};", rust_binding_name(storage)));
                }
            }
            if let Some(body) = case
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                self.block(body);
            }
            self.indent -= 1;
            self.line("},");
        }
    }
}

fn match_binding_used(node: &SyntaxNode, source: &crate::SourceFile, name: &str) -> bool {
    (node.kind == SyntaxKind::Name
        && source.text().get(node.span.start..node.span.end) == Some(name))
        || node
            .children
            .iter()
            .any(|child| match_binding_used(child, source, name))
}
