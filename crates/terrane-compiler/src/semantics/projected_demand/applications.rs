//! Header-only application matching: retain each actual type argument through
//! aliases, declared fields, and factory returns without forwarding outer owners.
use super::super::prelude::*;
use super::{ProjectedOwner, SourceResults, find_preceding_binding};

#[derive(Clone)]
struct HeaderType {
    name: String,
    owner: Option<ProjectedOwner>,
    arguments: Vec<HeaderType>,
}

impl SourceResults<'_> {
    fn header_type(
        &self,
        source: &SourceFile,
        node: &SyntaxNode,
        parameters: &BTreeMap<String, HeaderType>,
        imported: &BTreeMap<String, ProjectedOwner>,
    ) -> Option<HeaderType> {
        if node.kind == SyntaxKind::Name {
            let name = node_text(source, node);
            return parameters.get(name).cloned().or_else(|| {
                Some(HeaderType {
                    name: name.to_owned(),
                    owner: self.owner_for_name(source, name, imported, self.projection),
                    arguments: Vec::new(),
                })
            });
        }
        if node.kind == SyntaxKind::AppliedType {
            let mut head =
                self.header_type(source, node.children.first()?, parameters, imported)?;
            head.arguments = node
                .children
                .iter()
                .skip(1)
                .map(|argument| self.header_type(source, argument, parameters, imported))
                .collect::<Option<_>>()?;
            return Some(head);
        }
        if node.children.len() == 1 {
            return self.header_type(source, &node.children[0], parameters, imported);
        }
        None
    }

    fn expression_header(
        &self,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        imported: &BTreeMap<String, ProjectedOwner>,
        visited: &mut BTreeSet<Span>,
    ) -> Option<HeaderType> {
        if !visited.insert(node.span) {
            return None;
        }
        let result = self.expression_header_inner(unit, node, imported, visited);
        visited.remove(&node.span);
        result
    }

    #[expect(
        clippy::too_many_lines,
        reason = "header-only inference keeps binding, field and callable substitutions in one syntax dispatch"
    )]
    fn expression_header_inner(
        &self,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        imported: &BTreeMap<String, ProjectedOwner>,
        visited: &mut BTreeSet<Span>,
    ) -> Option<HeaderType> {
        if matches!(
            node.kind,
            SyntaxKind::Argument | SyntaxKind::GroupExpression | SyntaxKind::UnaryExpression
        ) {
            return self.expression_header(unit, node.children.last()?, imported, visited);
        }
        if node.kind == SyntaxKind::Name {
            let name = node_text(&unit.source, node);
            if name == "this" {
                let owner = self.implicit_receiver_owner(&unit.source, node)?;
                return Some(HeaderType {
                    name: owner.1.clone(),
                    owner: Some(owner),
                    arguments: Vec::new(),
                });
            }
            let mut declaration = None;
            find_preceding_binding(
                &unit.source,
                &unit.tree.root,
                name,
                node.span.start,
                &mut declaration,
            );
            let declaration = declaration?;
            if let Some(ty) = declaration
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::TypeExpression)
            {
                return self.header_type(&unit.source, ty, &BTreeMap::new(), imported);
            }
            return self.expression_header(unit, declaration.children.last()?, imported, visited);
        }
        if node.kind == SyntaxKind::MemberExpression {
            let [receiver, member] = node.children.as_slice() else {
                return None;
            };
            let receiver = self.expression_header(unit, receiver, imported, visited)?;
            let owner = receiver.owner.as_ref()?;
            for declaring_unit in self.units.iter().filter(|unit| unit.namespace == owner.0) {
                let Some(class) = declaring_unit.tree.root.children.iter().find(|class| {
                    matches!(
                        class.kind,
                        SyntaxKind::ClassDeclaration | SyntaxKind::TraitDeclaration
                    ) && declaration_name(class, &declaring_unit.source).as_deref()
                        == Some(owner.1.as_str())
                }) else {
                    continue;
                };
                let parameters = class
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::TypeParameterList)
                    .into_iter()
                    .flat_map(|list| &list.children)
                    .filter_map(|parameter| {
                        parameter
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::Name)
                    })
                    .map(|name| node_text(&declaring_unit.source, name).to_owned())
                    .zip(receiver.arguments.iter().cloned())
                    .collect();
                let field = class
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)?
                    .children
                    .iter()
                    .find(|field| {
                        field.kind == SyntaxKind::Binding
                            && declaration_name(field, &declaring_unit.source).as_deref()
                                == Some(node_text(&unit.source, member))
                    })?;
                let ty = field
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::TypeExpression)?;
                return self.header_type(&declaring_unit.source, ty, &parameters, imported);
            }
            return None;
        }
        if node.kind == SyntaxKind::CallExpression {
            let [callee, arguments] = node.children.as_slice() else {
                return None;
            };
            let mut base = callee;
            while matches!(
                base.kind,
                SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
            ) {
                base = base.children.first()?;
            }
            let callable = if base.kind == SyntaxKind::Name {
                let name = node_text(&unit.source, base);
                let import = self
                    .imports(&unit.source)
                    .iter()
                    .find(|import| import.alias == name);
                let (namespace, name) = import.map_or((unit.namespace.as_str(), name), |import| {
                    (import.target.as_str(), import.object.as_str())
                });
                self.callable(namespace, None, name, node.span)?
            } else if matches!(
                base.kind,
                SyntaxKind::StaticMemberExpression | SyntaxKind::MemberExpression
            ) {
                let [receiver, member] = base.children.as_slice() else {
                    return None;
                };
                let owner = if base.kind == SyntaxKind::StaticMemberExpression {
                    self.owner_for_name(
                        &unit.source,
                        node_text(&unit.source, receiver),
                        imported,
                        self.projection,
                    )?
                } else {
                    self.expression_header(unit, receiver, imported, visited)?
                        .owner?
                };
                let Some(callable) = self.callable(
                    &owner.0,
                    Some(&owner.1),
                    node_text(&unit.source, member),
                    node.span,
                ) else {
                    let function = self.projection.method(
                        &owner.0,
                        &owner.1,
                        node_text(&unit.source, member),
                        base.kind == SyntaxKind::StaticMemberExpression,
                    )?;
                    let crate::rust_interop::projection::ProjectedType::Foreign { name, .. } =
                        &function.result
                    else {
                        return None;
                    };
                    let result_owner =
                        self.projection.projected_owner_for_import(&owner.0, name)?;
                    return Some(HeaderType {
                        name: result_owner.1.clone(),
                        owner: Some(result_owner),
                        arguments: Vec::new(),
                    });
                };
                callable
            } else {
                return None;
            };
            let generic_names = callable
                .node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::TypeParameterList)
                .into_iter()
                .flat_map(|parameters| &parameters.children)
                .filter_map(|parameter| {
                    parameter
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)
                })
                .map(|name| node_text(&callable.unit.source, name).to_owned())
                .collect::<BTreeSet<_>>();
            let mut substitutions = BTreeMap::new();
            if let Some(parameters) = callable
                .node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::ParameterList)
            {
                for (index, parameter) in parameters.children.iter().enumerate() {
                    let Some(name) = parameter
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)
                    else {
                        continue;
                    };
                    let Some(formal) = parameter
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::TypeExpression)
                    else {
                        continue;
                    };
                    let Some(argument) = super::super::ownership::argument_for_parameter_name(
                        unit,
                        arguments,
                        node_text(&callable.unit.source, name),
                        index,
                    ) else {
                        continue;
                    };
                    if let Some(actual) = self.expression_header(unit, argument, imported, visited)
                    {
                        bind_header(
                            &callable.unit.source,
                            formal,
                            &actual,
                            &generic_names,
                            &mut substitutions,
                        );
                    }
                }
            }
            let result = callable
                .node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::TypeExpression)?;
            return self.header_type(&callable.unit.source, result, &substitutions, imported);
        }
        // Object construction has a parsed type child, not an inferred body type.
        node.children.iter().find_map(|child| {
            (child.kind == SyntaxKind::AppliedType)
                .then(|| self.header_type(&unit.source, child, &BTreeMap::new(), imported))
                .flatten()
        })
    }

    pub(super) fn substituted_application_owner(
        &self,
        caller: &SemanticUnit,
        argument: &SyntaxNode,
        formal_source: &SourceFile,
        formal: &SyntaxNode,
        generic: &str,
        imported: &BTreeMap<String, ProjectedOwner>,
    ) -> Option<ProjectedOwner> {
        let actual = self.expression_header(caller, argument, imported, &mut BTreeSet::new())?;
        let expected = self.header_type(
            formal_source,
            formal.children.first()?,
            &BTreeMap::new(),
            imported,
        )?;
        if actual.owner != expected.owner
            || (actual.owner.is_none() && actual.name != expected.name)
        {
            return None;
        }
        let mut substitutions = BTreeMap::new();
        bind_header(
            formal_source,
            formal,
            &actual,
            &BTreeSet::from([generic.to_owned()]),
            &mut substitutions,
        );
        substitutions.remove(generic)?.owner
    }
}

fn bind_header(
    source: &SourceFile,
    formal: &SyntaxNode,
    actual: &HeaderType,
    generics: &BTreeSet<String>,
    substitutions: &mut BTreeMap<String, HeaderType>,
) {
    if formal.kind == SyntaxKind::Name && generics.contains(node_text(source, formal)) {
        substitutions.insert(node_text(source, formal).to_owned(), actual.clone());
    } else if formal.kind == SyntaxKind::AppliedType {
        for (formal, actual) in formal.children.iter().skip(1).zip(&actual.arguments) {
            bind_header(source, formal, actual, generics, substitutions);
        }
    } else if formal.children.len() == 1 {
        bind_header(source, &formal.children[0], actual, generics, substitutions);
    }
}
