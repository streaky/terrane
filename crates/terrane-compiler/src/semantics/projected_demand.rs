use super::prelude::*;

pub(super) type ProjectedOwner = (String, String);

pub(super) fn imported_projected_types(
    projection: &crate::rust_interop::projection::Projection,
    imports: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, ProjectedOwner> {
    imports
        .iter()
        .flat_map(|(namespace, names)| {
            names.iter().filter_map(|name| {
                projection
                    .projected_owner_for_import(namespace, name)
                    .map(|owner| (name.clone(), owner))
            })
        })
        .collect()
}

fn projected_name<'a>(source: &'a SourceFile, node: &SyntaxNode) -> Option<&'a str> {
    if node.kind == SyntaxKind::Name {
        return Some(&source.text()[node.span.start..node.span.end]);
    }
    node.children
        .iter()
        .find_map(|child| projected_name(source, child))
}

fn projected_expression_owners(
    source: &SourceFile,
    node: &SyntaxNode,
    imported: &BTreeMap<String, ProjectedOwner>,
    bindings: &BTreeMap<String, BTreeSet<ProjectedOwner>>,
    projection: &crate::rust_interop::projection::Projection,
    source_results: &SourceResults<'_>,
) -> BTreeSet<ProjectedOwner> {
    if node.kind == SyntaxKind::Name {
        let name = &source.text()[node.span.start..node.span.end];
        return bindings
            .get(name)
            .cloned()
            .or_else(|| {
                source_results
                    .owner_for_name(source, name, imported, projection)
                    .map(|owner| BTreeSet::from([owner]))
            })
            .unwrap_or_default();
    }
    if matches!(
        node.kind,
        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
    ) && let (Some(receiver), Some(member)) = (node.children.first(), node.children.get(1))
    {
        let receiver_owners = projected_expression_owners(
            source,
            receiver,
            imported,
            bindings,
            projection,
            source_results,
        );
        let member = &source.text()[member.span.start..member.span.end];
        let result = receiver_owners
            .into_iter()
            .filter_map(|(namespace, owner)| {
                projection
                    .projected_member_scoped_family_owner(&namespace, &owner, member)
                    .or_else(|| {
                        projection.projected_member_result_owner(&namespace, &owner, member)
                    })
                    .map(|owner| BTreeSet::from([owner]))
                    .or_else(|| {
                        let selector =
                            projection.projected_payload_selector(&namespace, &owner, member)?;
                        bindings
                            .get(&format!("@constructor:{namespace}::{owner}:{selector}"))
                            .cloned()
                    })
            })
            .flatten()
            .collect::<BTreeSet<_>>();
        return if result.is_empty() {
            bindings.get(member).cloned().unwrap_or_default()
        } else {
            result
        };
    }
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
    {
        if let Some(owners) =
            source_results.call_result_owners(source, node, imported, bindings, projection)
        {
            return owners;
        }
        return projected_expression_owners(
            source,
            callee,
            imported,
            bindings,
            projection,
            source_results,
        );
    }
    if node.kind == SyntaxKind::ArgumentList || node.kind == SyntaxKind::Argument {
        return BTreeSet::new();
    }
    node.children
        .iter()
        .flat_map(|child| {
            projected_expression_owners(
                source,
                child,
                imported,
                bindings,
                projection,
                source_results,
            )
        })
        .collect()
}

pub(super) fn collect_projected_binding_owners(
    source: &SourceFile,
    node: &SyntaxNode,
    imported: &BTreeMap<String, ProjectedOwner>,
    bindings: &mut BTreeMap<String, BTreeSet<ProjectedOwner>>,
    projection: &crate::rust_interop::projection::Projection,
    source_results: &SourceResults<'_>,
) {
    if matches!(
        node.kind,
        SyntaxKind::Binding | SyntaxKind::Assignment | SyntaxKind::Parameter
    ) && let Some(name) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
    {
        let owners = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::TypeExpression)
            .and_then(|ty| projected_name(source, ty))
            .and_then(|ty| source_results.owner_for_name(source, ty, imported, projection))
            .map(|owner| BTreeSet::from([owner]))
            .or_else(|| {
                node.children.last().map(|value| {
                    projected_expression_owners(
                        source,
                        value,
                        imported,
                        bindings,
                        projection,
                        source_results,
                    )
                })
            })
            .unwrap_or_default();
        if !owners.is_empty() {
            bindings
                .entry(source.text()[name.span.start..name.span.end].to_owned())
                .or_default()
                .extend(owners);
        }
        if let Some(initializer) = node.children.last()
            && initializer.kind == SyntaxKind::CallExpression
            && let Some(callee) = initializer.children.first()
            && callee.kind == SyntaxKind::ConstructionExpression
            && let Some(owner_name) = callee
                .children
                .first()
                .and_then(|class| projected_name(source, class))
            && let Some((namespace, owner)) = imported.get(owner_name)
            && let Some(constructor) = projection.projected_constructor(namespace, owner)
            && let Some(arguments) = initializer.children.get(1)
        {
            let mut positional = 0;
            for argument in &arguments.children {
                let named = argument
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
                let parameter = if let Some(name) = named {
                    constructor.parameters.iter().find(|parameter| {
                        parameter.name == source.text()[name.span.start..name.span.end]
                    })
                } else {
                    let parameter = constructor.parameters.get(positional);
                    positional += 1;
                    parameter
                };
                if let Some(selector) = parameter.and_then(|parameter| {
                    crate::rust_interop::projection::Projection::payload_selector(&parameter.ty)
                }) && let Some(value) = argument.children.last()
                {
                    let owners = projected_expression_owners(
                        source,
                        value,
                        imported,
                        bindings,
                        projection,
                        source_results,
                    );
                    bindings
                        .entry(format!("@constructor:{namespace}::{owner}:{selector}"))
                        .or_default()
                        .extend(owners);
                }
            }
        }
    }
    for child in &node.children {
        collect_projected_binding_owners(
            source,
            child,
            imported,
            bindings,
            projection,
            source_results,
        );
    }
}
pub(super) fn collect_demanded_projected_members(
    source: &SourceFile,
    node: &SyntaxNode,
    imported: &BTreeMap<String, ProjectedOwner>,
    bindings: &BTreeMap<String, BTreeSet<ProjectedOwner>>,
    demanded: &mut BTreeMap<ProjectedOwner, BTreeSet<String>>,
    demand_sites: &mut crate::rust_interop::projection::ProjectionDemandSites,
    source_results: &SourceResults<'_>,
) {
    let projection = source_results.projection;
    if matches!(
        node.kind,
        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
    ) && let (Some(receiver), Some(member)) = (node.children.first(), node.children.get(1))
    {
        let owners = projected_expression_owners(
            source,
            receiver,
            imported,
            bindings,
            projection,
            source_results,
        );
        let member_name = source.text()[member.span.start..member.span.end].to_owned();
        let (line, column) = source.line_column(member.span.start);
        let site = format!("{}:{line}:{column}", source.path().display());
        for owner in owners {
            demanded
                .entry(owner.clone())
                .or_default()
                .insert(member_name.clone());
            demand_sites
                .entry((owner.0, owner.1, Some(member_name.clone())))
                .or_default()
                .insert(site.clone());
        }
    }
    for child in &node.children {
        collect_demanded_projected_members(
            source,
            child,
            imported,
            bindings,
            demanded,
            demand_sites,
            source_results,
        );
    }
}

pub(super) fn collect_associated_projected_member_demands(
    source: &SourceFile,
    node: &SyntaxNode,
    imported: &BTreeMap<String, ProjectedOwner>,
    demanded: &mut BTreeMap<ProjectedOwner, BTreeSet<String>>,
    demand_sites: &mut crate::rust_interop::projection::ProjectionDemandSites,
    projection: &crate::rust_interop::projection::Projection,
    source_results: &SourceResults<'_>,
) {
    if node.kind == SyntaxKind::ClassDeclaration {
        let associated = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
            .into_iter()
            .flat_map(|block| &block.children)
            .filter(|child| child.kind == SyntaxKind::AssociatedTypeDeclaration)
            .filter_map(|association| {
                let name = association
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)?;
                let ty = association
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::TypeExpression)?;
                let owner = imported.get(projected_name(source, ty)?)?;
                Some((
                    source.text()[name.span.start..name.span.end].to_owned(),
                    owner.clone(),
                ))
            })
            .collect::<BTreeMap<_, _>>();
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            for method in block
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::FunctionDeclaration)
            {
                let mut scoped_imports = imported.clone();
                scoped_imports.extend(
                    associated
                        .iter()
                        .map(|(alias, owner)| (alias.clone(), owner.clone())),
                );
                let mut scoped_bindings = BTreeMap::new();
                collect_projected_binding_owners(
                    source,
                    method,
                    &scoped_imports,
                    &mut scoped_bindings,
                    projection,
                    source_results,
                );
                collect_demanded_projected_members(
                    source,
                    method,
                    &scoped_imports,
                    &scoped_bindings,
                    demanded,
                    demand_sites,
                    source_results,
                );
            }
        }
        return;
    }
    for child in &node.children {
        collect_associated_projected_member_demands(
            source,
            child,
            imported,
            demanded,
            demand_sites,
            projection,
            source_results,
        );
    }
}

struct SourceSignature<'a> {
    unit: &'a SemanticUnit,
    contract: FunctionContract,
}

struct SourceCallable<'a> {
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    signature: std::cell::OnceCell<Option<SourceSignature<'a>>>,
    scope: Option<Span>,
}

// Namespace → declaring class (empty for free functions) → callable name.
type SourceCallables<'a> =
    BTreeMap<&'a str, BTreeMap<&'a str, BTreeMap<&'a str, Vec<SourceCallable<'a>>>>>;

/// Header-only contracts supply demand before projected bodies are available.
/// Default expressions and function bodies are checked by the normal analysis.
pub(super) struct SourceResults<'a> {
    units: &'a [SemanticUnit],
    projection: &'a crate::rust_interop::projection::Projection,
    unit_indices: BTreeMap<u32, usize>,
    imports: Vec<Vec<Import>>,
    classes: BTreeSet<(&'a str, &'a str)>,
    callables: SourceCallables<'a>,
}

impl<'a> SourceResults<'a> {
    pub(super) fn new(
        units: &'a [SemanticUnit],
        projection: &'a crate::rust_interop::projection::Projection,
    ) -> Result<Self, SemanticFailure> {
        let mut results = Self {
            units,
            projection,
            unit_indices: units
                .iter()
                .enumerate()
                .map(|(i, u)| (u.source.id(), i))
                .collect(),
            imports: units
                .iter()
                .map(imports_in_tree)
                .collect::<Result<_, _>>()?,
            classes: BTreeSet::new(),
            callables: BTreeMap::new(),
        };
        for (index, unit) in units.iter().enumerate() {
            results.collect(index, &unit.tree.root, None, None);
        }
        Ok(results)
    }

    fn collect(
        &mut self,
        index: usize,
        node: &'a SyntaxNode,
        owner: Option<&'a str>,
        scope: Option<Span>,
    ) {
        let unit = &self.units[index];
        let name = node
            .children
            .iter()
            .find(|n| n.kind == SyntaxKind::Name)
            .map(|n| node_text(&unit.source, n));
        let owner = if node.kind == SyntaxKind::ClassDeclaration {
            if let Some(name) = name {
                self.classes.insert((&unit.namespace, name));
            }
            name
        } else {
            owner
        };
        let mut child_scope = scope;
        if is_function_node(node) {
            if let Some(name) = name {
                self.callables
                    .entry(&unit.namespace)
                    .or_default()
                    .entry(owner.unwrap_or(""))
                    .or_default()
                    .entry(name)
                    .or_default()
                    .push(SourceCallable {
                        unit,
                        node,
                        signature: std::cell::OnceCell::new(),
                        scope,
                    });
            }
            child_scope = Some(node.span);
        }
        for child in &node.children {
            self.collect(index, child, owner, child_scope);
        }
    }

    pub(super) fn imports(&self, source: &SourceFile) -> &[Import] {
        self.unit_indices
            .get(&source.id())
            .map_or(&[], |index| &self.imports[*index])
    }

    fn owner_for_name(
        &self,
        source: &SourceFile,
        name: &str,
        imported: &BTreeMap<String, ProjectedOwner>,
        projection: &crate::rust_interop::projection::Projection,
    ) -> Option<ProjectedOwner> {
        let unit = &self.units[*self.unit_indices.get(&source.id())?];
        if self.classes.contains(&(unit.namespace.as_str(), name)) {
            return Some((unit.namespace.clone(), name.to_owned()));
        }
        if let Some(import) = self
            .imports(source)
            .iter()
            .find(|import| import.alias == name)
        {
            return projection
                .projected_owner_for_import(&import.target, &import.object)
                .or_else(|| {
                    self.classes
                        .contains(&(import.target.as_str(), import.object.as_str()))
                        .then(|| (import.target.clone(), import.object.clone()))
                });
        }
        imported.get(name).cloned()
    }

    fn callable(
        &self,
        namespace: &str,
        owner: Option<&str>,
        name: &str,
        site: Span,
    ) -> Option<&SourceCallable<'a>> {
        self.callables
            .get(namespace)?
            .get(owner.unwrap_or(""))?
            .get(name)?
            .iter()
            .filter(|callable| {
                callable.scope.is_none_or(|scope| {
                    scope.file == site.file && scope.start <= site.start && scope.end >= site.end
                })
            })
            .max_by_key(|callable| callable.scope.map_or(0, |scope| scope.start + 1))
    }

    fn call_result_owners(
        &self,
        source: &SourceFile,
        call: &SyntaxNode,
        imported: &BTreeMap<String, ProjectedOwner>,
        bindings: &BTreeMap<String, BTreeSet<ProjectedOwner>>,
        projection: &crate::rust_interop::projection::Projection,
    ) -> Option<BTreeSet<ProjectedOwner>> {
        let [callee, arguments] = call.children.as_slice() else {
            return None;
        };
        let mut base = callee;
        while matches!(
            base.kind,
            SyntaxKind::GroupExpression | SyntaxKind::AppliedType | SyntaxKind::TypeExpression
        ) {
            base = base.children.first()?;
        }
        if base.kind == SyntaxKind::Name {
            let name = node_text(source, base);
            let unit = &self.units[*self.unit_indices.get(&source.id())?];
            let import = self
                .imports(source)
                .iter()
                .find(|import| import.alias == name);
            let (namespace, name) = import.map_or((unit.namespace.as_str(), name), |i| {
                (i.target.as_str(), i.object.as_str())
            });
            let callable = self.callable(namespace, None, name, call.span)?;
            return Some(self.result_owners(callable, source, arguments, imported, bindings));
        }
        if matches!(
            base.kind,
            SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
        ) {
            let [receiver, member, ..] = base.children.as_slice() else {
                return None;
            };
            let owners =
                projected_expression_owners(source, receiver, imported, bindings, projection, self);
            let mut result = BTreeSet::new();
            let mut found = false;
            for (namespace, owner) in owners {
                if let Some(callable) = self.callable(
                    &namespace,
                    Some(&owner),
                    node_text(source, member),
                    call.span,
                ) {
                    found = true;
                    result.extend(
                        self.result_owners(callable, source, arguments, imported, bindings),
                    );
                }
            }
            return found.then_some(result);
        }
        None
    }

    fn result_owners(
        &self,
        callable: &SourceCallable<'_>,
        source: &SourceFile,
        arguments: &SyntaxNode,
        imported: &BTreeMap<String, ProjectedOwner>,
        bindings: &BTreeMap<String, BTreeSet<ProjectedOwner>>,
    ) -> BTreeSet<ProjectedOwner> {
        let Some(callable) = callable
            .signature
            .get_or_init(|| {
                super::contracts::analyze_function_signature(
                    callable.unit,
                    callable.node,
                    &BTreeMap::new(),
                    false,
                )
                .ok()
                .map(|contract| SourceSignature {
                    unit: callable.unit,
                    contract,
                })
            })
            .as_ref()
        else {
            return BTreeSet::new();
        };
        let Some(result) = callable.contract.return_type.as_ref() else {
            return BTreeSet::new();
        };
        self.type_owners(callable, result, source, arguments, imported, bindings)
    }

    fn type_owners(
        &self,
        callable: &SourceSignature<'_>,
        ty: &ValueType,
        source: &SourceFile,
        arguments: &SyntaxNode,
        imported: &BTreeMap<String, ProjectedOwner>,
        bindings: &BTreeMap<String, BTreeSet<ProjectedOwner>>,
    ) -> BTreeSet<ProjectedOwner> {
        let projection = self.projection;
        match ty {
            ValueType::Reference(element)
            | ValueType::MutableReference(element)
            | ValueType::SharedReference(element)
            | ValueType::Task(element, _)
            | ValueType::ScopedTask(element, _) => self.type_owners(
                callable,
                element.value_type_ref(),
                source,
                arguments,
                imported,
                bindings,
            ),
            ValueType::Optional(value) => {
                self.type_owners(callable, value, source, arguments, imported, bindings)
            }
            ValueType::Union(values) => values
                .iter()
                .flat_map(|value| {
                    self.type_owners(callable, value, source, arguments, imported, bindings)
                })
                .collect(),
            ValueType::Object(_) | ValueType::TypeParameter(_) => {
                let name = match ty {
                    ValueType::Object(owner) => owner.name.as_str(),
                    ValueType::TypeParameter(name) => name.as_str(),
                    _ => unreachable!(),
                };
                if callable
                    .contract
                    .generic_parameters
                    .iter()
                    .any(|parameter| parameter.name == name)
                {
                    let caller = &self.units[self.unit_indices[&source.id()]];
                    return callable
                        .contract
                        .parameters
                        .iter()
                        .enumerate()
                        .filter(|(_, parameter)| parameter.value_type.as_ref() == Some(ty))
                        .filter_map(|(index, parameter)| {
                            super::ownership::argument_for_parameter_name(
                                caller,
                                arguments,
                                &parameter.name,
                                index,
                            )
                        })
                        .flat_map(|argument| {
                            projected_expression_owners(
                                source, argument, imported, bindings, projection, self,
                            )
                        })
                        .collect();
                }
                self.owner_for_name(&callable.unit.source, name, imported, projection)
                    .into_iter()
                    .collect()
            }
            _ => BTreeSet::new(),
        }
    }
}
