use super::prelude::*;

fn collect_unsafe_rust_spans(node: &SyntaxNode, spans: &mut Vec<Span>) {
    if node.kind == SyntaxKind::UnsafeRustBlock {
        spans.push(node.span);
    }
    for child in &node.children {
        collect_unsafe_rust_spans(child, spans);
    }
}

type ProjectedOwner = (String, String);

fn imported_projected_types(
    projection: &crate::projection::Projection,
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
    projection: &crate::projection::Projection,
) -> BTreeSet<ProjectedOwner> {
    if node.kind == SyntaxKind::Name {
        let name = &source.text()[node.span.start..node.span.end];
        return bindings
            .get(name)
            .cloned()
            .or_else(|| {
                imported
                    .get(name)
                    .cloned()
                    .map(|owner| BTreeSet::from([owner]))
            })
            .unwrap_or_default();
    }
    if matches!(
        node.kind,
        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
    ) && let (Some(receiver), Some(member)) = (node.children.first(), node.children.get(1))
    {
        let receiver_owners =
            projected_expression_owners(source, receiver, imported, bindings, projection);
        let member = &source.text()[member.span.start..member.span.end];
        let result = receiver_owners
            .into_iter()
            .filter_map(|(namespace, owner)| {
                projection.projected_member_result_owner(&namespace, &owner, member)
            })
            .collect::<BTreeSet<_>>();
        return if result.is_empty() {
            bindings.get(member).cloned().unwrap_or_default()
        } else {
            result
        };
    }
    node.children
        .iter()
        .flat_map(|child| {
            projected_expression_owners(source, child, imported, bindings, projection)
        })
        .collect()
}

fn collect_projected_binding_owners(
    source: &SourceFile,
    node: &SyntaxNode,
    imported: &BTreeMap<String, ProjectedOwner>,
    bindings: &mut BTreeMap<String, BTreeSet<ProjectedOwner>>,
    projection: &crate::projection::Projection,
) {
    if matches!(
        node.kind,
        SyntaxKind::Binding | SyntaxKind::Assignment | SyntaxKind::Parameter
    ) && let Some(name) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
    {
        let owner = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::TypeExpression)
            .and_then(|ty| projected_name(source, ty))
            .and_then(|ty| imported.get(ty).cloned())
            .or_else(|| {
                node.children.last().and_then(|value| {
                    projected_expression_owners(source, value, imported, bindings, projection)
                        .into_iter()
                        .next()
                })
            });
        if let Some(owner) = owner {
            bindings
                .entry(source.text()[name.span.start..name.span.end].to_owned())
                .or_default()
                .insert(owner);
        }
    }
    for child in &node.children {
        collect_projected_binding_owners(source, child, imported, bindings, projection);
    }
}
fn collect_name_demand_sites(
    source: &SourceFile,
    node: &SyntaxNode,
    alias: &str,
    sites: &mut BTreeSet<String>,
) {
    if matches!(
        node.kind,
        SyntaxKind::ImportDeclaration
            | SyntaxKind::ObjectImport
            | SyntaxKind::ImportSelection
            | SyntaxKind::ImportAlias
            | SyntaxKind::NamespaceDeclaration
    ) {
        return;
    }
    if node.kind == SyntaxKind::Name && &source.text()[node.span.start..node.span.end] == alias {
        let (line, column) = source.line_column(node.span.start);
        sites.insert(format!("{}:{line}:{column}", source.path().display()));
    }
    for (index, child) in node.children.iter().enumerate() {
        if node.kind.child_field(index, child.kind) != "name" {
            collect_name_demand_sites(source, child, alias, sites);
        }
    }
}

fn first_name_demand_span(source: &SourceFile, node: &SyntaxNode, alias: &str) -> Option<Span> {
    if matches!(
        node.kind,
        SyntaxKind::ImportDeclaration
            | SyntaxKind::ObjectImport
            | SyntaxKind::ImportSelection
            | SyntaxKind::ImportAlias
            | SyntaxKind::NamespaceDeclaration
    ) {
        return None;
    }
    if node.kind == SyntaxKind::Name && &source.text()[node.span.start..node.span.end] == alias {
        return Some(node.span);
    }
    node.children
        .iter()
        .enumerate()
        .filter(|(index, child)| node.kind.child_field(*index, child.kind) != "name")
        .find_map(|(_, child)| first_name_demand_span(source, child, alias))
}

fn name_demand_span(units: &[SemanticUnit], source_id: u32, alias: &str) -> Option<Span> {
    let unit = units.iter().find(|unit| unit.source.id() == source_id)?;
    first_name_demand_span(&unit.source, &unit.tree.root, alias)
}

fn name_demand_sites(units: &[SemanticUnit], source_id: u32, alias: &str) -> BTreeSet<String> {
    let mut sites = BTreeSet::new();
    if let Some(unit) = units.iter().find(|unit| unit.source.id() == source_id) {
        collect_name_demand_sites(&unit.source, &unit.tree.root, alias, &mut sites);
    }
    sites
}

fn collect_demanded_projected_members(
    source: &SourceFile,
    node: &SyntaxNode,
    imported: &BTreeMap<String, ProjectedOwner>,
    bindings: &BTreeMap<String, BTreeSet<ProjectedOwner>>,
    demanded: &mut BTreeMap<ProjectedOwner, BTreeSet<String>>,
    demand_sites: &mut crate::projection::ProjectionDemandSites,
    projection: &crate::projection::Projection,
) {
    if matches!(
        node.kind,
        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
    ) && let (Some(receiver), Some(member)) = (node.children.first(), node.children.get(1))
    {
        let owners = projected_expression_owners(source, receiver, imported, bindings, projection);
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
            projection,
        );
    }
}

pub(super) fn parse_unit(
    source: &SourceFile,
    source_path: String,
    expected_namespace: Option<&str>,
    prelude: bool,
    bundled: bool,
    role: crate::SourceRole,
) -> Result<SemanticUnit, SemanticFailure> {
    let lexed = lexer::lex(source).map_err(|diagnostics| SemanticFailure {
        source: source.clone(),
        diagnostics,
    })?;
    let parsed = parser::parse(source, lexed);
    if !parsed.diagnostics.is_empty() {
        return Err(SemanticFailure {
            source: source.clone(),
            diagnostics: parsed.diagnostics,
        });
    }
    let namespace = declared_namespace(
        source,
        &parsed.tree,
        expected_namespace.is_none() && source.text().starts_with("#!"),
    )
    .map_err(|diagnostic| SemanticFailure {
        source: source.clone(),
        diagnostics: vec![diagnostic],
    })?;
    if let Some(expected) = expected_namespace
        && namespace != expected
    {
        let span = parsed
            .tree
            .root
            .children
            .iter()
            .find(|node| node.kind == SyntaxKind::NamespaceDeclaration)
            .map_or(Span::new(source.id(), 0, source.text().len()), |node| {
                node.span
            });
        let diagnostic = Diagnostic::error(
            "S2020",
            format!(
                "declared namespace `{namespace}` does not match `{expected}` required by its source directory"
            ),
            span,
        )
        .with_help(format!("declare `namespace {}`", expected.trim_start_matches('/')));
        return Err(SemanticFailure {
            source: source.clone(),
            diagnostics: vec![diagnostic],
        });
    }
    let enclosing_function_spans = index_enclosing_function_spans(&parsed.tree.root);
    let mut unsafe_rust_spans = Vec::new();
    collect_unsafe_rust_spans(&parsed.tree.root, &mut unsafe_rust_spans);
    Ok(SemanticUnit {
        source: source.clone(),
        source_path,
        tree: parsed.tree,
        namespace,
        prelude,
        bundled,
        role,
        scopes: Vec::new(),
        typed_bindings: Vec::new(),
        functions: Vec::new(),
        reference_provenance: BTreeMap::new(),
        reference_return_lenders: BTreeMap::new(),
        descriptors: Vec::new(),
        builtin_descriptors: builtin_descriptor_contracts(),
        comparable_foreign_objects: BTreeSet::new(),
        nonclone_foreign_objects: BTreeSet::new(),
        projected_interfaces_requiring_application: BTreeSet::new(),
        function_aliases: BTreeMap::new(),
        function_contracts_by_span: BTreeMap::new(),
        descriptor_aliases: BTreeMap::new(),
        projected_removals: Vec::new(),
        projected_destination_functions: BTreeSet::new(),
        projected_call_specializations: BTreeMap::new(),
        enclosing_function_spans,
        unsafe_rust_spans,
        unreachable_spans: Vec::new(),
        evaluation_steps: Vec::new(),
        selections: Vec::new(),
    })
}

fn parse_authored_units(package: &Package) -> Result<Vec<SemanticUnit>, SemanticFailure> {
    package
        .units
        .iter()
        .map(|unit| {
            parse_unit(
                &unit.source,
                unit.relative_path_text(),
                unit.expected_namespace.as_deref(),
                unit.prelude,
                false,
                unit.role,
            )
        })
        .collect()
}
fn validate_generated_projection_units(
    package: &Package,
    inventory: &str,
) -> Result<(), SemanticFailure> {
    let units = crate::projection::generated_projection_units(inventory).map_err(|message| {
        failure(
            &package.units[0].source,
            "S2028",
            format!("cannot materialize generated projection source units: {message}"),
            Span::new(package.units[0].source.id(), 0, 0),
        )
    })?;
    let mut source_id = package
        .units
        .iter()
        .map(|unit| unit.source.id())
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    for unit in units {
        let source = SourceFile::new(
            source_id,
            package
                .root
                .join(crate::projection::GENERATED_PROJECTION_FILE),
            unit.source,
        );
        parse_unit(
            &source,
            format!(
                "{}#{}",
                crate::projection::GENERATED_PROJECTION_FILE,
                unit.namespace
            ),
            Some(&unit.namespace),
            package.prelude,
            false,
            crate::package::SourceRole::Production,
        )
        .map_err(|failure| {
            let diagnostics = failure
                .diagnostics
                .into_iter()
                .map(|diagnostic| {
                    let detail = diagnostic.primary.map_or_else(String::new, |span| {
                        let (line, column) = source.line_column(span.start);
                        let source_line = source
                            .text()
                            .lines()
                            .nth(line.saturating_sub(1))
                            .unwrap_or_default();
                        format!(" at {line}:{column}: `{source_line}`")
                    });
                    Diagnostic::error(
                        "S2028",
                        format!(
                            "generated projection source unit `{}` is invalid{detail}: {}",
                            unit.namespace, diagnostic.message
                        ),
                        Span::new(package.units[0].source.id(), 0, 0),
                    )
                })
                .collect();
            SemanticFailure {
                source: package.units[0].source.clone(),
                diagnostics,
            }
        })?;
        source_id = source_id.saturating_add(1);
    }
    Ok(())
}

fn persist_projection_inventory(
    package: &Package,
    projection: &crate::projection::Projection,
    demand_sites: &crate::projection::ProjectionDemandSites,
) -> Result<(), SemanticFailure> {
    let inventory = projection.documented_inventory(demand_sites);
    validate_generated_projection_units(package, &inventory)?;
    let path = package
        .root
        .join(crate::projection::GENERATED_PROJECTION_FILE);
    if std::fs::read_to_string(&path).ok().as_deref() == Some(&inventory) {
        return Ok(());
    }
    std::fs::write(&path, inventory).map_err(|error| {
        failure(
            &package.units[0].source,
            "S2028",
            format!(
                "cannot write complete dependency projection inventory `{}`: {error}",
                path.display()
            ),
            Span::new(package.units[0].source.id(), 0, 0),
        )
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "unit projection applies removals and destination metadata atomically"
)]
fn augment_units_with_projection(
    package: &Package,
    projection: &crate::projection::Projection,
    mut units: Vec<SemanticUnit>,
    persist_inventory: bool,
) -> Result<Vec<SemanticUnit>, SemanticFailure> {
    let mut loaded = units
        .iter()
        .map(|unit| unit.namespace.clone())
        .collect::<BTreeSet<_>>();
    let mut next_source_id = package.next_source_id();
    let mut demand_sites = crate::projection::ProjectionDemandSites::new();
    let mut dependency_imports = BTreeMap::<String, BTreeSet<String>>::new();
    let mut imported_aliases = BTreeMap::<String, ProjectedOwner>::new();
    for unit in &units {
        for import in imports_in_tree(unit)?
            .into_iter()
            .filter(|import| import.target.starts_with("/deps/"))
        {
            if let Some(details) = projection.item_ambiguity(&import.target, &import.object) {
                return Err(failure(
                    &import.source,
                    "S2056",
                    format!(
                        "Rust dependency member `{}` in `{}` is ambiguous: {details}",
                        import.object, import.target
                    ),
                    import.span,
                ));
            }
            if let Some(owner) =
                projection.projected_owner_for_import(&import.target, &import.object)
            {
                imported_aliases.insert(import.alias.clone(), owner);
            }
            demand_sites
                .entry((import.target.clone(), import.object.clone(), None))
                .or_default()
                .extend(name_demand_sites(&units, import.source.id(), &import.alias));
            dependency_imports
                .entry(import.target)
                .or_default()
                .insert(import.object);
        }
    }
    let mut imported_types = imported_projected_types(projection, &dependency_imports);
    imported_types.extend(imported_aliases);
    let mut binding_owners = BTreeMap::<String, BTreeSet<ProjectedOwner>>::new();
    for unit in &units {
        collect_projected_binding_owners(
            &unit.source,
            &unit.tree.root,
            &imported_types,
            &mut binding_owners,
            projection,
        );
    }
    let mut demanded_members = BTreeMap::<ProjectedOwner, BTreeSet<String>>::new();
    for unit in &units {
        collect_demanded_projected_members(
            &unit.source,
            &unit.tree.root,
            &imported_types,
            &binding_owners,
            &mut demanded_members,
            &mut demand_sites,
            projection,
        );
    }
    if persist_inventory {
        persist_projection_inventory(package, projection, &demand_sites)?;
    }
    let projected_sources = projection
        .source_for_imports_with_members(&dependency_imports, &demanded_members)
        .map_err(|message| SemanticFailure {
            source: package.units[0].source.clone(),
            diagnostics: vec![Diagnostic::error(
                "S2028",
                message,
                Span::new(package.units[0].source.id(), 0, 0),
            )],
        })?;
    for (namespace, text) in projected_sources {
        if !loaded.insert(namespace.clone()) {
            continue;
        }
        let path = format!(
            "<terrane>/projected/{}.trn",
            namespace.trim_start_matches('/')
        );
        let source = SourceFile::new(next_source_id, path.clone().into(), text);
        next_source_id = next_source_id.saturating_add(1);
        units.push(parse_unit(
            &source,
            path,
            Some(&namespace),
            package.prelude,
            true,
            crate::SourceRole::Bundled,
        )?);
    }
    let mut index = 0;
    while index < units.len() {
        let targets = imports_in_tree(&units[index])?
            .into_iter()
            .map(|import| import.target)
            .collect::<BTreeSet<_>>();
        for target in targets {
            let Some(bundled) = crate::bundled::source(&target) else {
                continue;
            };
            if !loaded.insert(target) {
                continue;
            }
            let source = SourceFile::new(
                next_source_id,
                std::path::PathBuf::from(format!("<terrane>/{}", bundled.path)),
                bundled.text.to_owned(),
            );
            next_source_id = next_source_id.saturating_add(1);
            units.push(parse_unit(
                &source,
                bundled.path.to_owned(),
                Some(bundled.namespace),
                package.prelude,
                true,
                crate::SourceRole::Bundled,
            )?);
        }
        index += 1;
    }
    let mut destination_functions = BTreeSet::new();
    for dependency in &projection.dependencies {
        for item in &dependency.items {
            match &item.kind {
                crate::projection::ProjectedKind::Function(function)
                    if function.destination_result.is_some() =>
                {
                    destination_functions.insert(format!("{}::{}", item.namespace, item.name));
                }
                crate::projection::ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                } => {
                    for method in methods.iter().chain(static_methods) {
                        if method.destination_result.is_some() {
                            destination_functions.insert(format!(
                                "{}::{}.{}",
                                item.namespace, item.name, method.name
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    for unit in &mut units {
        unit.projected_removals.clone_from(&projection.removed);
        unit.projected_destination_functions
            .clone_from(&destination_functions);
    }
    Ok(units)
}
pub(super) fn apply_projected_method_contracts(
    units: &mut [SemanticUnit],
    projection: &crate::projection::Projection,
) {
    for unit in units {
        if !unit.namespace.starts_with("/deps/") {
            continue;
        }
        for contract in &mut unit.functions {
            let Some(owner) = contract.owner.as_deref() else {
                continue;
            };
            let type_name = unit
                .descriptors
                .iter()
                .find(|object| object.identity.name == owner)
                .map_or(owner, |object| object.name.as_str());
            let projected_interface_method =
                projection.interface_method(&unit.namespace, type_name, &contract.name);
            let Some(method) = projected_interface_method
                .map(|method| &method.function)
                .or_else(|| {
                    projection.method(
                        &unit.namespace,
                        type_name,
                        &contract.name,
                        contract.is_static,
                    )
                })
            else {
                continue;
            };
            contract.throws = method.error.is_some();
            let invocation_mode = match method.receiver {
                Some(crate::projection::Receiver::Move) => InvocationMode::Consuming,
                Some(crate::projection::Receiver::MutableBorrow) => InvocationMode::Mutable,
                _ => InvocationMode::Shared,
            };
            contract.written_invocation_mode = invocation_mode;
            contract.exact_invocation_mode = invocation_mode;
            contract.projected_provided =
                projected_interface_method.is_some_and(|method| method.provided);
        }
    }
}
fn validate_projected_static_declines(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::StaticMemberExpression
            && let [receiver, member] = node.children.as_slice()
            && let Some(owner) = package.resolve_name_at(
                unit,
                receiver.span.start,
                node_text(&unit.source, receiver),
            )
            && owner.kind == SymbolKind::Class
            && owner.namespace.starts_with("/deps/")
        {
            let member_name = node_text(&unit.source, member);
            if let Some(reason) = package.projection.declined_method_reason(
                &owner.namespace,
                &owner.name,
                member_name,
            ) {
                return Err(failure(
                    &unit.source,
                    "T0105",
                    format!(
                        "Rust dependency static member `{member_name}` on class `{}` is not projected: {reason}",
                        node_text(&unit.source, receiver),
                    ),
                    member.span,
                ));
            }
        }
        for child in &node.children {
            visit(package, unit, child)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit(package, unit, &unit.tree.root)?;
    }
    Ok(())
}

fn dependency_demands_from_units(
    units: &[SemanticUnit],
) -> Result<BTreeSet<(String, String)>, SemanticFailure> {
    Ok(units
        .iter()
        .map(imports_in_tree)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .filter(|import| import.target.starts_with("/deps/"))
        .map(|import| (import.target, import.object))
        .collect())
}

/// Parses authored package units and returns the exact dependency imports that can demand
/// generated or externally reexported projection fragments.
///
/// # Errors
/// Returns the first source-oriented lexer, parser, or import discovery failure.
pub fn dependency_projection_demands(
    package: &Package,
) -> Result<BTreeSet<(String, String)>, SemanticFailure> {
    dependency_demands_from_units(&parse_authored_units(package)?)
}

fn dependency_projection(
    package: &Package,
    _demands: &BTreeSet<(String, String)>,
) -> Result<crate::projection::Projection, SemanticFailure> {
    crate::projection::resolve(&package.root, &package.rust_dependencies, None).map_err(|error| {
        failure(
            &package.units[0].source,
            "S2028",
            error.message,
            Span::new(package.units[0].source.id(), 0, 0),
        )
    })
}

/// Builds the complete namespace tree, then resolves declarations and imports.
///
/// Semantic phases fail at the first diagnostic in deterministic package and source
/// order. Unlike independently discoverable manifest errors, later semantic errors can
/// depend on declarations or imports that an earlier error prevented from assembling.
///
/// # Errors
/// Returns the first source-oriented lexer, parser, namespace, scope, or import failure.
pub fn analyze(package: &Package) -> Result<SemanticPackage, SemanticFailure> {
    let units = parse_authored_units(package)?;
    let demands = dependency_demands_from_units(&units)?;
    let projection = dependency_projection(package, &demands)?;
    analyze_parsed_with_projection(package, projection, units, true)
}

#[cfg(test)]
pub(super) fn analyze_with_projection(
    package: &Package,
    projection: crate::projection::Projection,
) -> Result<SemanticPackage, SemanticFailure> {
    let units = parse_authored_units(package)?;
    analyze_parsed_with_projection(package, projection, units, false)
}

#[expect(
    clippy::too_many_lines,
    reason = "semantic phase orchestration remains linear and order-sensitive"
)]
fn analyze_parsed_with_projection(
    package: &Package,
    projection: crate::projection::Projection,
    units: Vec<SemanticUnit>,
    persist_inventory: bool,
) -> Result<SemanticPackage, SemanticFailure> {
    let mut units = augment_units_with_projection(package, &projection, units, persist_inventory)?;
    let nonclone_foreign_objects = projection
        .dependencies
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match &item.kind {
            crate::projection::ProjectedKind::ForeignType {
                cloneable: false, ..
            } => Some(ObjectIdentity::new(&item.namespace, &item.name)),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    for unit in &mut units {
        unit.comparable_foreign_objects = projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| {
                matches!(
                    item.kind,
                    crate::projection::ProjectedKind::Enum {
                        data_carrying: false,
                        comparable: true,
                        ..
                    }
                )
            })
            .map(|item| ObjectIdentity::new(&item.namespace, &item.name))
            .collect();
        unit.nonclone_foreign_objects
            .clone_from(&nonclone_foreign_objects);
        unit.projected_interfaces_requiring_application = projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter_map(|item| match &item.kind {
                crate::projection::ProjectedKind::Interface(interface)
                    if interface.associated_type.is_some() =>
                {
                    Some(ObjectIdentity::new(&item.namespace, &item.name))
                }
                _ => None,
            })
            .collect();
    }
    validate_compiler_owned_names(&units)?;

    let mut namespaces = bootstrap_namespaces();
    for unit in &units {
        let bundled = unit.bundled;
        if !bundled
            && (unit.namespace == "/core"
                || unit.namespace.starts_with("/core/")
                || unit.namespace == "/deps"
                || unit.namespace.starts_with("/deps/")
                || crate::bundled::source(&unit.namespace).is_some())
        {
            let span = unit
                .tree
                .root
                .children
                .iter()
                .find(|node| node.kind == SyntaxKind::NamespaceDeclaration)
                .map_or(Span::new(unit.source.id(), 0, 0), |node| node.span);
            return Err(failure(
                &unit.source,
                "S2017",
                format!(
                    "cannot declare into compiler-owned namespace `{}`",
                    unit.namespace
                ),
                span,
            ));
        }
        namespaces.entry(unit.namespace.clone()).or_default();
    }
    let production_namespaces = units
        .iter()
        .filter(|unit| unit.role == crate::SourceRole::Production)
        .map(|unit| unit.namespace.as_str())
        .collect::<BTreeSet<_>>();
    if let Some(unit) = units.iter().find(|unit| {
        unit.role == crate::SourceRole::IntegrationTest
            && production_namespaces.contains(unit.namespace.as_str())
    }) {
        let span = unit
            .tree
            .root
            .children
            .iter()
            .find(|node| node.kind == SyntaxKind::NamespaceDeclaration)
            .map_or(Span::new(unit.source.id(), 0, 0), |node| node.span);
        return Err(failure(
            &unit.source,
            "S2054",
            format!(
                "integration test cannot declare production namespace `{}`",
                unit.namespace
            ),
            span,
        ));
    }

    let mut imports = Vec::new();
    let mut globals = BTreeMap::<String, Symbol>::new();
    for unit in &units {
        collect_unit(unit, &mut namespaces, &mut globals, &mut imports)?;
    }
    let discovered_imports = units
        .iter()
        .map(imports_in_tree)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    if package.purpose == crate::PackagePurpose::Production
        && let Some(import) = discovered_imports.iter().find(|import| {
            !import.bundled
                && (import.target == "/core/testing" || import.target.starts_with("/core/testing/"))
        })
    {
        return Err(failure(
            &import.source,
            "S2053",
            format!(
                "test-only namespace `{}` is unavailable to production sources",
                import.target
            ),
            import.span,
        ));
    }
    for import in discovered_imports.iter().filter(|import| !import.bundled) {
        for capability in namespace_capabilities(&import.target) {
            if !package.profile.allows(capability) {
                return Err(failure(
                    &import.source,
                    "S2032",
                    format!(
                        "profile `{}` forbids capability `{capability}` required by `{}` imported by `{}`",
                        package.profile.name, import.target, import.namespace
                    ),
                    import.span,
                ));
            }
        }
    }
    let test_only_namespaces = units
        .iter()
        .filter(|unit| {
            matches!(
                unit.role,
                crate::SourceRole::UnitTest
                    | crate::SourceRole::IntegrationTest
                    | crate::SourceRole::EndToEndTest
            )
        })
        .map(|unit| unit.namespace.as_str())
        .filter(|namespace| !production_namespaces.contains(namespace))
        .collect::<BTreeSet<_>>();
    for import in &discovered_imports {
        let role = units
            .iter()
            .find(|unit| unit.source.id() == import.source.id())
            .map_or(crate::SourceRole::Bundled, |unit| unit.role);
        if role == crate::SourceRole::Production
            && test_only_namespaces.contains(import.target.as_str())
        {
            return Err(failure(
                &import.source,
                "S2055",
                format!(
                    "production source cannot import test-only namespace `{}`",
                    import.target
                ),
                import.span,
            ));
        }
        if role == crate::SourceRole::EndToEndTest
            && production_namespaces.contains(import.target.as_str())
        {
            return Err(failure(
                &import.source,
                "S2054",
                format!(
                    "end-to-end test must drive the application artifact instead of importing `{}`",
                    import.target
                ),
                import.span,
            ));
        }
    }
    let mut unused_unavailable_imports = BTreeSet::new();
    for import in &discovered_imports {
        let Some(dependency) = import
            .target
            .strip_prefix("/deps/")
            .and_then(|path| path.split('/').next())
        else {
            continue;
        };
        if !package
            .rust_dependencies
            .iter()
            .any(|declared| declared.name.replace('_', "-") == dependency)
        {
            return Err(failure(
                &import.source,
                "S2027",
                format!("Rust dependency `{dependency}` is not declared in `package.toml`"),
                import.span,
            ));
        }
        if projection.item(&import.target, &import.object).is_none() {
            let Some(demand_span) = name_demand_span(&units, import.source.id(), &import.alias)
            else {
                unused_unavailable_imports.insert((
                    import.source.id(),
                    import.span.start,
                    import.target.clone(),
                    import.object.clone(),
                ));
                continue;
            };
            if let Some(details) = projection.item_ambiguity(&import.target, &import.object) {
                return Err(failure(
                    &import.source,
                    "S2056",
                    format!(
                        "Rust dependency member `{}` in `{}` is ambiguous: {details}",
                        import.object, import.target
                    ),
                    demand_span,
                ));
            }
            if let Some(removed) = projection
                .removed
                .iter()
                .find(|removed| removed.namespace == import.target && removed.name == import.object)
            {
                return Err(failure(
                    &import.source,
                    "S2031",
                    format!(
                        "Rust dependency member `{}` in `{}` was removed when the projected dependency changed from version `{}` to `{}`",
                        import.object,
                        import.target,
                        removed.previous_version,
                        removed.current_version
                    ),
                    demand_span,
                ));
            }
            let reason = projection.dependencies.iter().find_map(|dependency| {
                dependency.declined.iter().find_map(|declined| {
                    (crate::projection::namespace_for_rust_path(dependency, &declined.rust_path)
                        == import.target
                        && declined.rust_path.rsplit("::").next() == Some(import.object.as_str()))
                    .then_some(declined.reason.as_str())
                })
            });
            let message = reason.map_or_else(
                || {
                    format!(
                        "Rust dependency projection has no member `{}` in `{}`; see `{}` for the complete projection inventory and current demand",
                        import.object,
                        import.target,
                        crate::projection::GENERATED_PROJECTION_FILE,
                    )
                },
                |reason| {
                    format!(
                        "Rust dependency member `{}` in `{}` is not projected: {reason}; see `{}` for the complete projection inventory and current demand",
                        import.object,
                        import.target,
                        crate::projection::GENERATED_PROJECTION_FILE,
                    )
                },
            );
            return Err(failure(&import.source, "S2029", message, demand_span));
        }
    }
    imports.retain(|import| {
        !unused_unavailable_imports.contains(&(
            import.source.id(),
            import.span.start,
            import.target.clone(),
            import.object.clone(),
        ))
    });
    let prelude_namespaces = units
        .iter()
        .filter(|unit| unit.prelude)
        .map(|unit| unit.namespace.clone())
        .collect::<BTreeSet<_>>();
    let prelude_bindings = if prelude_namespaces.is_empty() {
        BTreeMap::new()
    } else {
        bootstrap_prelude()
    };
    let mut import_warnings = resolve_imports(
        imports,
        &mut namespaces,
        &globals,
        &prelude_bindings,
        &prelude_namespaces,
    )?;
    for unit in &mut units {
        unit.scopes = collect_lexical_scopes(unit, &namespaces, &globals, &prelude_bindings)?;
        import_warnings.extend(
            unit.scopes
                .iter()
                .flat_map(|scope| scope.import_warnings.iter().cloned()),
        );
    }
    apply_projected_method_contracts(&mut units, &projection);
    let descriptor_constructs = bootstrap_descriptor_constructs();

    let mut semantic = SemanticPackage {
        identity: package.identity.clone(),
        prelude: package.prelude,
        reflection: package.reflection,
        executor: package.executor,
        artifact: package.artifact,
        execution_strategy: crate::execution::ExecutionStrategy::from_profile(package.executor),
        execution_requirements: crate::execution::ExecutionRequirements::default(),
        profile: package.profile.clone(),
        root: package.root.clone(),
        namespaces,
        globals,
        prelude_bindings,
        descriptor_constructs,
        units,
        projection,
        binding_events: BTreeMap::new(),
        referenced_functions: BTreeSet::new(),
        import_warnings,
        bootstrap_version: BOOTSTRAP_VERSION,
    };
    validate_initializer_dependencies(&semantic)?;
    validate_references(&semantic)?;
    validate_projected_static_declines(&semantic)?;
    analyze_types(&mut semantic)?;
    if semantic.execution_strategy == crate::execution::ExecutionStrategy::Local {
        for unit in &semantic.units {
            if let Some(destructor) = unit
                .functions
                .iter()
                .find(|contract| contract.name == "destruct" && contract.is_async)
            {
                return Err(failure(
                    &unit.source,
                    "T0137",
                    "awaited destructors require the threaded executor",
                    destructor.span,
                ));
            }
        }
    }
    validate_shared_ownership_cycles(&semantic)?;
    validate_error_clauses(&semantic)?;
    validate_moves(&semantic)?;
    analyze_reference_provenance(&mut semantic)?;
    validate_referenced_replacements(&semantic)?;
    infer_throwing_effects(&mut semantic)?;
    apply_projected_method_contracts(&mut semantic.units, &semantic.projection);
    refresh_typed_bindings_after_effect_inference(&mut semantic)?;
    analyze_selections(&mut semantic)?;
    validate_class_field_initializers(&semantic)?;
    validate_constant_reassignment(&semantic)?;
    validate_global_definite_assignment(&semantic)?;
    record_binding_mutability(&mut semantic);
    validate_calls(&semantic)?;
    validate_discarded_temporary_mutations(&semantic)?;
    validate_definite_assignment(&semantic)?;
    record_binding_events(&mut semantic);
    infer_task_transferability(&mut semantic);
    validate_projected_callback_arguments(&semantic)?;
    validate_suspension_ownership(&semantic)?;
    validate_task_consumption(&semantic)?;
    validate_task_transferability(&semantic)?;
    let unreachable_units = validate_control_flow(&semantic)?;
    for (unit, unreachable_spans) in semantic.units.iter_mut().zip(unreachable_units) {
        unit.unreachable_spans = unreachable_spans;
        unit.evaluation_steps = collect_evaluation_steps(&unit.source, &unit.tree.root);
    }
    record_function_references(&mut semantic);
    Ok(semantic)
}

pub(super) fn object_implements_identity(object: &DescriptorContract, target: &str) -> bool {
    object
        .interfaces
        .iter()
        .any(|interface| interface.qualified() == target)
}

pub(super) fn identity_implements(package: &SemanticPackage, identity: &str, target: &str) -> bool {
    package.units.iter().any(|unit| {
        unit.descriptors.iter().any(|object| {
            package
                .namespaces
                .values()
                .flat_map(|namespace| namespace.symbols.values())
                .any(|symbol| {
                    symbol.identity == identity && symbol.declaration_span == Some(object.span)
                })
                && object_implements_identity(object, target)
        })
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "error validation keeps throw, catch, and finally rules in one ordered traversal"
)]
pub(super) fn validate_error_clauses(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    #[expect(
        clippy::too_many_lines,
        reason = "the recursive visitor validates the complete structured-error boundary"
    )]
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        in_catch: bool,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::ThrowStatement {
            if node.children.is_empty() {
                if !in_catch {
                    return Err(failure(
                        &unit.source,
                        "T0020",
                        "bare `throw` is only valid inside a catch clause",
                        node.span,
                    ));
                }
            } else {
                let thrown = &node.children[0];
                let symbol = package.resolve_name_at(
                    unit,
                    thrown.span.start,
                    node_text(&unit.source, thrown.children.first().unwrap_or(thrown)),
                );
                let value_type = infer_value_type(unit, thrown, &unit.typed_bindings)?;
                let standard = symbol.is_some_and(|symbol| symbol.kind == SymbolKind::ErrorObject)
                    || matches!(
                        &value_type,
                        Some(ValueType::Object(identity))
                            if identity == &ObjectIdentity::new("/core/errors", "throwable")
                    );
                let object_name = match &value_type {
                    Some(ValueType::Descriptor(name)) => Some(name.as_str()),
                    Some(ValueType::Object(identity)) => Some(identity.name.as_str()),
                    _ if thrown.kind == SyntaxKind::CallExpression => thrown
                        .children
                        .first()
                        .filter(|callee| callee.kind == SyntaxKind::Name)
                        .map(|callee| node_text(&unit.source, callee)),
                    _ => None,
                };
                let user_throwable = object_name
                    .and_then(|name| {
                        package.resolve_name_at(
                            unit,
                            thrown.span.start,
                            name.rsplit_once("::").map_or(name, |(_, local)| local),
                        )
                    })
                    .is_some_and(|symbol| {
                        identity_implements(package, &symbol.identity, "/core/errors::throwable")
                    });
                if !standard && !user_throwable {
                    return Err(failure(
                        &unit.source,
                        "T0021",
                        "thrown values must implement `throwable`",
                        thrown.span,
                    ));
                }
            }
        }
        if node.kind == SyntaxKind::TryStatement {
            let mut caught = BTreeSet::new();
            let mut catches_all = false;
            for clause in node
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
            {
                let Some(descriptor) = clause
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Name)
                else {
                    if catches_all {
                        return Err(failure(
                            &unit.source,
                            "T0022",
                            "catch-all clause is unreachable",
                            clause.span,
                        ));
                    }
                    catches_all = true;
                    continue;
                };
                let name = node_text(&unit.source, descriptor);
                let symbol = package.resolve_name_at(unit, descriptor.span.start, name);
                let valid = symbol.is_some_and(|symbol| {
                    symbol.kind == SymbolKind::ErrorObject
                        || (symbol.kind == SymbolKind::Interface
                            && symbol.identity == "/core/errors::throwable")
                        || (symbol.kind == SymbolKind::Class
                            && package
                                .projection
                                .is_projected_error_type(&symbol.namespace, &symbol.name))
                        || (matches!(symbol.kind, SymbolKind::Class | SymbolKind::TypeDescriptor)
                            && identity_implements(
                                package,
                                &symbol.identity,
                                "/core/errors::throwable",
                            ))
                });
                if !valid {
                    return Err(failure(
                        &unit.source,
                        "T0021",
                        format!("`{name}` is not a throwable descriptor"),
                        descriptor.span,
                    ));
                }
                let identity = &symbol.expect("validated error symbol").identity;
                if catches_all || !caught.insert(identity.clone()) {
                    return Err(failure(
                        &unit.source,
                        "T0022",
                        format!("catch clause for `{name}` is unreachable"),
                        clause.span,
                    ));
                }
                catches_all = identity == "/core/errors::throwable";
            }
        }
        for child in &node.children {
            let child_in_catch = in_catch || node.kind == SyntaxKind::CatchClause;
            visit(package, unit, child, child_in_catch)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit(package, unit, &unit.tree.root, false)?;
    }
    Ok(())
}

pub(super) fn populate_namespace_function_contracts(package: &mut SemanticPackage) {
    let namespaces = package
        .units
        .iter()
        .map(|unit| unit.namespace.clone())
        .collect::<Vec<_>>();
    let functions = package
        .units
        .iter()
        .map(|unit| unit.functions.clone())
        .collect::<Vec<_>>();
    for (unit, namespace) in package.units.iter_mut().zip(&namespaces) {
        unit.functions = namespaces
            .iter()
            .zip(&functions)
            .filter(|(candidate, _)| *candidate == namespace)
            .flat_map(|(_, functions)| functions.iter().cloned())
            .collect();
    }
}

pub(super) fn populate_object_aliases(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| unit.descriptors.iter())
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        let mut aliases = package
            .namespaces
            .get(&unit.namespace)
            .into_iter()
            .flat_map(|namespace| &namespace.symbols)
            .chain(
                unit.scopes
                    .iter()
                    .flat_map(|scope| &scope.symbols)
                    .flat_map(|(name, symbols)| symbols.iter().map(move |symbol| (name, symbol))),
            )
            .filter_map(|(visible_name, symbol)| {
                let span = symbol.declaration_span?;
                matches!(
                    symbol.kind,
                    SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                )
                .then(|| contracts.get(&(span.file, span.start, span.end)))
                .flatten()
                .cloned()
                .map(|mut contract| {
                    contract.name.clone_from(visible_name);
                    contract
                })
            })
            .collect::<Vec<_>>();
        aliases.retain(|alias| {
            !unit
                .descriptors
                .iter()
                .any(|contract| contract.name == alias.name)
        });
        unit.descriptors.extend(aliases);
    }
}

pub(super) fn populate_function_aliases(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| unit.functions.iter())
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        let mut aliases = BTreeMap::new();
        let mut contracts_by_span = BTreeMap::new();
        for namespace_name in namespace_chain(&unit.namespace) {
            let Some(namespace) = package.namespaces.get(&namespace_name) else {
                continue;
            };
            for (visible_name, symbol) in &namespace.symbols {
                let Some(span) = symbol.declaration_span else {
                    continue;
                };
                if symbol.kind != SymbolKind::Function || !visible_from(symbol, &unit.namespace) {
                    continue;
                }
                let key = (span.file, span.start, span.end);
                if let Some(contract) = contracts.get(&key) {
                    aliases
                        .entry(visible_name.clone())
                        .or_insert_with(|| contract.clone());
                    contracts_by_span
                        .entry(key)
                        .or_insert_with(|| contract.clone());
                }
            }
        }
        for symbol in unit
            .scopes
            .iter()
            .flat_map(|scope| scope.symbols.values())
            .flatten()
            .filter(|symbol| symbol.kind == SymbolKind::Function)
        {
            let Some(span) = symbol.declaration_span else {
                continue;
            };
            let key = (span.file, span.start, span.end);
            if let Some(contract) = contracts.get(&key) {
                contracts_by_span
                    .entry(key)
                    .or_insert_with(|| contract.clone());
            }
        }
        unit.function_aliases = aliases;
        unit.function_contracts_by_span = contracts_by_span;
    }
}

pub(super) fn resolved_function_contract<'a>(
    unit: &'a SemanticUnit,
    name: &str,
    offset: usize,
) -> Option<&'a FunctionContract> {
    lexical_scope_chain(unit, offset)
        .find_map(|scope| {
            let symbol = scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol.kind == SymbolKind::Function
                    && symbol.binding_span.is_none_or(|span| span.end <= offset)
            })?;
            let span = symbol.declaration_span?;
            unit.function_contracts_by_span
                .get(&(span.file, span.start, span.end))
        })
        .or_else(|| unit.function_aliases.get(name))
}

fn enqueue_value_type_object_dependencies(value_type: &ValueType, queue: &mut Vec<ObjectIdentity>) {
    match value_type {
        ValueType::Object(identity) => queue.push(identity.clone()),
        ValueType::Optional(inner) => enqueue_value_type_object_dependencies(inner, queue),
        ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::AsyncIterationStep(item)
        | ValueType::ChannelPair(item)
        | ValueType::ChannelSender(item)
        | ValueType::ChannelReceiver(item)
        | ValueType::ChannelSendOutcome(item)
        | ValueType::ChannelReceiveOutcome(item)
        | ValueType::DocumentDecodeOutcome(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::UnorderedSet(item)
        | ValueType::Task(item, _)
        | ValueType::ScopedTask(item, _)
        | ValueType::TaskOutcome(item)
        | ValueType::Reference(item)
        | ValueType::SharedReference(item) => {
            enqueue_value_type_object_dependencies(item.value_type_ref(), queue);
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            enqueue_value_type_object_dependencies(key.value_type_ref(), queue);
            enqueue_value_type_object_dependencies(value.value_type_ref(), queue);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                enqueue_value_type_object_dependencies(parameter.value_type_ref(), queue);
            }
            enqueue_value_type_object_dependencies(result.value_type_ref(), queue);
        }
        _ => {}
    }
}

fn enqueue_function_contract_object_dependencies(
    contract: &FunctionContract,
    queue: &mut Vec<ObjectIdentity>,
) {
    if let Some(return_type) = &contract.return_type {
        enqueue_value_type_object_dependencies(return_type, queue);
    }
    for parameter in &contract.parameters {
        if let Some(value_type) = &parameter.value_type {
            enqueue_value_type_object_dependencies(value_type, queue);
        }
    }
}

pub(super) fn populate_function_type_dependencies(package: &mut SemanticPackage) {
    let objects = package
        .units
        .iter()
        .flat_map(|unit| {
            unit.descriptors
                .iter()
                .map(move |object| (unit.namespace.as_str(), object))
        })
        .fold(
            BTreeMap::<ObjectIdentity, DescriptorContract>::new(),
            |mut objects, (namespace, object)| {
                let canonical =
                    object.name == object.identity.name && namespace == object.identity.namespace;
                objects
                    .entry(object.identity.clone())
                    .and_modify(|existing| {
                        if canonical {
                            existing.clone_from(object);
                        }
                    })
                    .or_insert_with(|| object.clone());
                objects
            },
        );
    let methods = package
        .units
        .iter()
        .flat_map(|unit| unit.functions.iter())
        .filter_map(|method| {
            method
                .owner
                .as_ref()
                .map(|owner| ((method.span.file, owner.clone()), method.clone()))
        })
        .fold(
            BTreeMap::<(u32, String), Vec<FunctionContract>>::new(),
            |mut methods, (key, method)| {
                methods.entry(key).or_default().push(method);
                methods
            },
        );
    for unit in &mut package.units {
        let mut queue = Vec::new();
        for contract in unit
            .function_aliases
            .values()
            .chain(unit.function_contracts_by_span.values())
        {
            enqueue_function_contract_object_dependencies(contract, &mut queue);
        }
        queue.extend(
            unit.descriptors
                .iter()
                .filter(|object| {
                    object.name != object.identity.name
                        || object.identity.namespace != unit.namespace
                })
                .map(|object| object.identity.clone()),
        );
        let mut visited = BTreeSet::new();
        while let Some(key) = queue.pop() {
            if !visited.insert(key.clone()) {
                continue;
            }
            let Some(object) = objects.get(&key) else {
                continue;
            };
            for field in &object.fields {
                enqueue_value_type_object_dependencies(&field.value_type, &mut queue);
            }
            let object_methods = methods
                .get(&(object.span.file, object.name.clone()))
                .cloned()
                .unwrap_or_default();
            for method in &object_methods {
                enqueue_function_contract_object_dependencies(method, &mut queue);
            }
            if !unit
                .descriptors
                .iter()
                .any(|candidate| candidate.name == object.name)
            {
                unit.descriptors.push(object.clone());
            }
            for method in object_methods {
                if !unit
                    .functions
                    .iter()
                    .any(|candidate| candidate.span == method.span && candidate.name == method.name)
                {
                    unit.functions.push(method);
                }
            }
        }
    }
}

impl SemanticPackage {
    #[must_use]
    pub fn symbol(&self, namespace: &str, name: &str) -> Option<&Symbol> {
        self.namespaces.get(namespace)?.symbols.get(name)
    }

    #[must_use]
    pub(crate) fn function_is_referenced(&self, declaration: Span) -> bool {
        self.referenced_functions.contains(&span_key(declaration))
    }

    pub(crate) fn mark_functions_referenced(
        &mut self,
        declarations: impl IntoIterator<Item = Span>,
    ) {
        self.referenced_functions
            .extend(declarations.into_iter().map(span_key));
        super::bindings::synchronize_execution_requirements(self);
    }

    fn resolve_name_with_prelude(
        &self,
        namespace: &str,
        name: &str,
        prelude: bool,
    ) -> Option<&Symbol> {
        namespace_chain(namespace)
            .find_map(|path| {
                self.symbol(&path, name).filter(|symbol| {
                    visible_from(symbol, namespace)
                        && (symbol.kind != SymbolKind::Binding
                            || symbol.constant
                            || symbol.global
                            || symbol.namespace == namespace)
                })
            })
            .or_else(|| {
                self.globals
                    .get(name)
                    .filter(|symbol| visible_from(symbol, namespace))
            })
            .or_else(|| self.symbol("/core/types", name))
            .or_else(|| prelude.then(|| self.prelude_bindings.get(name)).flatten())
    }

    #[must_use]
    pub fn resolve_name(&self, namespace: &str, name: &str) -> Option<&Symbol> {
        self.resolve_name_with_prelude(namespace, name, self.prelude)
    }

    #[must_use]
    pub fn resolve_name_at<'a>(
        &'a self,
        unit: &'a SemanticUnit,
        offset: usize,
        name: &str,
    ) -> Option<&'a Symbol> {
        let mut scopes = lexical_scope_chain(unit, offset).peekable();
        let inside_lexical_scope = scopes.peek().is_some();
        scopes
            .find_map(|scope| {
                scope
                    .symbols
                    .get(name)?
                    .iter()
                    .rev()
                    .find(|symbol| symbol.binding_span.is_none_or(|span| span.end <= offset))
            })
            .or_else(|| {
                self.resolve_name_with_prelude(&unit.namespace, name, unit.prelude)
                    .filter(|symbol| !inside_lexical_scope || symbol.available_in_function_body())
            })
    }

    #[must_use]
    pub fn lexical_replaced_binding_span(
        &self,
        unit: &SemanticUnit,
        span: Span,
        name: &str,
    ) -> Option<Span> {
        let current = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.name == name && binding.span == span)?;
        let current_scope = lexical_scope_index_at(unit, current.span.start);
        lexical_scope_chain(unit, span.start).find_map(|scope| {
            let symbols = scope.symbols.get(name)?;
            symbols
                .iter()
                .filter_map(|symbol| symbol.declaration_span)
                .filter(|prior| {
                    prior.start < span.start
                        && lexical_scope_index_at(unit, prior.start) == current_scope
                })
                .max_by_key(|prior| prior.start)
        })
    }

    #[must_use]
    pub fn is_lexical_replacement(&self, unit: &SemanticUnit, span: Span, name: &str) -> bool {
        self.lexical_replaced_binding_span(unit, span, name)
            .is_some()
    }
}
