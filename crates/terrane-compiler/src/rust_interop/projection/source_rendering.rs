use super::*;

pub(super) fn render_callable_declaration(
    output: &mut String,
    item: &ProjectedItem,
    aliases: &BTreeMap<String, String>,
) {
    match &item.kind {
        ProjectedKind::Function(function) => {
            render_function(output, function, true, 0, aliases, None);
        }
        ProjectedKind::Macro(function) => {
            writeln!(
                output,
                "function {}; arguments {} ...\n",
                function.name,
                ProjectedType::Generic("__MacroArgument".to_owned()).terrane_name()
            )
            .expect("writing to a string cannot fail");
        }
        _ => {}
    }
}

pub(super) fn projected_item_functions(item: &ProjectedItem) -> Vec<&ProjectedFunction> {
    match &item.kind {
        ProjectedKind::Function(function) | ProjectedKind::Macro(function) => vec![function],
        ProjectedKind::ForeignType {
            methods,
            static_methods,
            ..
        }
        | ProjectedKind::Enum {
            methods,
            static_methods,
            ..
        } => methods.iter().chain(static_methods).collect(),
        ProjectedKind::Interface(interface) => interface
            .methods
            .iter()
            .map(|method| &method.function)
            .collect(),
    }
}
pub(super) fn propagate_partial_contract_requirements(unavailable: &mut [UnavailableProjection]) {
    let unavailable_paths = unavailable
        .iter()
        .map(|entry| entry.rust_path.clone())
        .collect::<BTreeSet<_>>();
    let mut required_paths = unavailable
        .iter()
        .filter(|entry| !entry.required_by.is_empty())
        .map(|entry| entry.rust_path.clone())
        .collect::<BTreeSet<_>>();
    loop {
        let inherited = unavailable
            .iter()
            .filter(|entry| required_paths.contains(&entry.rust_path))
            .flat_map(|entry| entry.references.intersection(&unavailable_paths).cloned())
            .collect::<BTreeSet<_>>();
        let previous_len = required_paths.len();
        required_paths.extend(inherited);
        if required_paths.len() == previous_len {
            break;
        }
    }
    let required_by_contracts = unavailable
        .iter()
        .filter(|target| required_paths.contains(&target.rust_path))
        .map(|target| {
            let contracts = unavailable
                .iter()
                .filter(|source| {
                    required_paths.contains(&source.rust_path)
                        && source.references.contains(&target.rust_path)
                })
                .map(|source| source.rust_path.clone())
                .collect();
            (target.rust_path.clone(), contracts)
        })
        .collect::<BTreeMap<_, _>>();

    for target in unavailable {
        target.required_by_contracts = required_by_contracts
            .get(&target.rust_path)
            .cloned()
            .unwrap_or_default();
    }
}
pub(super) fn unavailable_member_map(
    unavailable: &[UnavailableProjection],
) -> UnavailableMemberMap<'_> {
    let mut members = UnavailableMemberMap::new();
    for entry in unavailable.iter().filter(|entry| entry.member.is_some()) {
        members
            .entry((&entry.namespace, &entry.name))
            .or_default()
            .push(entry);
    }
    for entries in members.values_mut() {
        entries.sort_by(|left, right| {
            left.member
                .cmp(&right.member)
                .then_with(|| left.rust_path.cmp(&right.rust_path))
                .then_with(|| left.reason.cmp(&right.reason))
        });
    }
    members
}

pub(super) fn append_required_nominal_units(
    sources: &mut Vec<(String, String)>,
    projection: &Projection,
    unavailable: &[UnavailableProjection],
) {
    let mut candidates = BTreeMap::<(&str, &str), Vec<&UnavailableProjection>>::new();
    for entry in unavailable.iter().filter(|entry| entry.is_required()) {
        if matches!(entry.partial, Some(PartialProjection::NominalType { .. })) {
            candidates
                .entry((&entry.namespace, &entry.name))
                .or_default()
                .push(entry);
        }
    }
    for ((namespace, name), entries) in candidates {
        if entries.len() != 1
            || projection
                .dependencies
                .iter()
                .flat_map(|dependency| &dependency.items)
                .any(|item| item.namespace == namespace && item.name == name)
        {
            continue;
        }
        let entry = entries[0];
        let Some(PartialProjection::NominalType {
            declaration,
            native_kind,
            generic_parameters,
        }) = &entry.partial
        else {
            continue;
        };
        if native_kind == "trait" && !generic_parameters.is_empty() {
            continue;
        }
        let rendered_generics = if generic_parameters.is_empty() {
            "none".to_owned()
        } else {
            generic_parameters.join(", ")
        };
        let addition = format!(
            "# Partial native {native_kind}: {}\n\
             # Native generic parameters retained as a residual obligation: {rendered_generics}\n\
             # This nominal class is not registered for lowering until its residual obligations are satisfied.\n\
             {declaration}\n",
            entry.rust_path
        );
        if let Some((_, source)) = sources
            .iter_mut()
            .find(|(source_namespace, _)| source_namespace == namespace)
        {
            source.push('\n');
            source.push_str(&addition);
        } else {
            sources.push((
                namespace.to_owned(),
                format!(
                    "namespace {}\n\n{addition}",
                    namespace.trim_start_matches('/')
                ),
            ));
        }
    }
    sources.sort_by(|left, right| left.0.cmp(&right.0));
}

pub(super) fn render_required_projected_declarations(
    output: &mut String,
    projection: &Projection,
    unavailable: &[UnavailableProjection],
) {
    let mut required_projected = BTreeMap::<&str, (&ProjectedItem, BTreeSet<&str>)>::new();
    for source in unavailable.iter().filter(|entry| entry.is_required()) {
        for reference in &source.references {
            for item in projection
                .dependencies
                .iter()
                .flat_map(|dependency| &dependency.items)
                .filter(|item| item.rust_path == *reference)
            {
                required_projected
                    .entry(&item.rust_path)
                    .or_insert_with(|| (item, BTreeSet::new()))
                    .1
                    .insert(&source.rust_path);
            }
        }
    }
    output.push_str("#\n# Required admitted projected declarations\n");
    if required_projected.is_empty() {
        output.push_str("# None\n");
        return;
    }
    for (rust_path, (item, contracts)) in required_projected {
        writeln!(
            output,
            "#\n# Native path: {rust_path}\n\
             # Terrane namespace: {}\n\
             # Projected declaration: {}\n\
             # Required by projection contracts:",
            item.namespace, item.name
        )
        .expect("writing to a string cannot fail");
        for contract in contracts {
            writeln!(output, "# - {contract}").expect("writing to a string cannot fail");
        }
    }
}

pub(super) fn render_unavailable_projection(
    output: &mut String,
    unavailable: &UnavailableProjection,
) {
    let status = if unavailable.is_required() {
        "required"
    } else {
        "unused"
    };
    writeln!(output, "#\n# Native path: {}", unavailable.rust_path)
        .expect("writing to a string cannot fail");
    writeln!(output, "# Terrane namespace: {}", unavailable.namespace)
        .expect("writing to a string cannot fail");
    writeln!(
        output,
        "# Unavailable declaration: {}{}",
        unavailable.name,
        unavailable
            .member
            .as_ref()
            .map_or_else(String::new, |member| format!(".{member}"))
    )
    .expect("writing to a string cannot fail");
    writeln!(output, "# Projection status: unavailable ({status})")
        .expect("writing to a string cannot fail");
    writeln!(output, "# Projection gap: {}", unavailable.reason)
        .expect("writing to a string cannot fail");
    if unavailable.is_required() {
        match &unavailable.partial {
            Some(PartialProjection::Function {
                signature,
                generic_constraints,
                callback_shapes,
            }) => {
                output.push_str("# Generated partial contract: function\n");
                writeln!(output, "# Function shape: {signature}")
                    .expect("writing to a string cannot fail");
                for constraint in generic_constraints {
                    writeln!(output, "# Generic constraint: {constraint}")
                        .expect("writing to a string cannot fail");
                }
                for callback in callback_shapes {
                    writeln!(
                        output,
                        "# Callback parameter `{}`: {}",
                        callback.parameter, callback.contract
                    )
                    .expect("writing to a string cannot fail");
                    for method in &callback.methods {
                        writeln!(output, "#   Required callback method: {method}")
                            .expect("writing to a string cannot fail");
                    }
                }
            }
            Some(PartialProjection::NominalType {
                declaration,
                native_kind,
                generic_parameters,
            }) => {
                if native_kind == "trait" && !generic_parameters.is_empty() {
                    output.push_str(
                        "# Generated partial contract: parameterized native trait template\n",
                    );
                    writeln!(
                        output,
                        "# Native trait template parameters: {}",
                        generic_parameters.join(", ")
                    )
                    .expect("writing to a string cannot fail");
                    output.push_str("# Terrane nominal declaration: none\n");
                } else {
                    writeln!(output, "# Generated partial contract: {native_kind}")
                        .expect("writing to a string cannot fail");
                    writeln!(output, "# Terrane nominal declaration: {declaration}")
                        .expect("writing to a string cannot fail");
                    if !generic_parameters.is_empty() {
                        writeln!(
                            output,
                            "# Residual native generic parameters: {}",
                            generic_parameters.join(", ")
                        )
                        .expect("writing to a string cannot fail");
                    }
                }
            }
            Some(PartialProjection::Namespace) => {
                output.push_str(
                    "# Generated partial contract: namespace container; no value declaration\n",
                );
            }
            None => {}
        }
    }
    render_demand_sites(output, Some(&unavailable.required_by));
    if !unavailable.required_by_contracts.is_empty() {
        output.push_str("# Required by projection contracts:\n");
        for contract in &unavailable.required_by_contracts {
            writeln!(output, "# - {contract}").expect("writing to a string cannot fail");
        }
    }
}

pub(super) fn render_demand_sites(output: &mut String, sites: Option<&BTreeSet<String>>) {
    let Some(sites) = sites.filter(|sites| !sites.is_empty()) else {
        output.push_str("# Required by: none\n");
        return;
    };
    output.push_str("# Required by:\n");
    for site in sites {
        writeln!(output, "# - {site}").expect("writing to a string cannot fail");
    }
}

fn projected_member_is_demanded(
    namespace: &str,
    owner: &str,
    member: &str,
    demanded: &ProjectedMemberDemands,
) -> bool {
    demanded
        .get(&(namespace.to_owned(), owner.to_owned()))
        .is_some_and(|members| members.contains(member))
}

pub(super) fn expanded_source_imports(
    all_items: &[&ProjectedItem],
    imports: &BTreeMap<String, BTreeSet<String>>,
    demanded_members: &ProjectedMemberDemands,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut expanded = imports.clone();
    loop {
        let previous_count = expanded.values().map(BTreeSet::len).sum::<usize>();
        let selected = all_items
            .iter()
            .copied()
            .filter(|item| {
                expanded
                    .get(&item.namespace)
                    .is_some_and(|names| names.contains(&item.name))
            })
            .collect::<Vec<_>>();
        for (rust_path, name) in collect_source_foreign(all_items, &selected, demanded_members) {
            if let Some(item) = projected_item_for_foreign(all_items, &rust_path, &name) {
                expanded
                    .entry(item.namespace.clone())
                    .or_default()
                    .insert(item.name.clone());
            }
        }
        if expanded.values().map(BTreeSet::len).sum::<usize>() == previous_count {
            return expanded;
        }
    }
}

pub(super) fn projected_source_dependencies(
    ordered_foreign: &[(&String, &String)],
    all_items: &[&ProjectedItem],
    namespace: &str,
) -> BTreeSet<String> {
    ordered_foreign
        .iter()
        .filter_map(|(rust_path, name)| {
            projected_item_for_foreign(all_items, rust_path, name)
                .filter(|item| item.namespace != namespace)
                .map(|item| item.namespace.clone())
        })
        .collect()
}

pub(super) fn projected_item_for_foreign<'a>(
    all_items: &'a [&ProjectedItem],
    rust_path: &str,
    name: &str,
) -> Option<&'a ProjectedItem> {
    if let Some(exact) = all_items
        .iter()
        .copied()
        .find(|item| item.rust_path == rust_path)
    {
        return Some(exact);
    }
    let mut candidates = all_items
        .iter()
        .copied()
        .filter(|item| item.name == name)
        .filter_map(|item| {
            let parent = item.rust_path.rsplit_once("::")?.0;
            rust_path
                .starts_with(&format!("{parent}::"))
                .then_some((parent.len(), item))
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(prefix_len, _)| std::cmp::Reverse(*prefix_len));
    let (best_len, best) = candidates.first().copied()?;
    (candidates
        .get(1)
        .is_none_or(|(next_len, _)| *next_len < best_len))
    .then_some(best)
}

#[expect(
    clippy::too_many_lines,
    reason = "source dependency closure handles each projected declaration shape together"
)]
pub(super) fn collect_source_foreign(
    all_items: &[&ProjectedItem],
    selected: &[&ProjectedItem],
    demanded_members: &ProjectedMemberDemands,
) -> BTreeMap<String, String> {
    let mut foreign = BTreeMap::<String, String>::new();
    for item in selected {
        match &item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                collect_foreign_function(function, &mut foreign);
            }
            ProjectedKind::ForeignType {
                fields,
                methods,
                static_methods,
                ..
            } => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for field in fields {
                    collect_foreign_type(&field.ty, &mut foreign);
                }
                for method in methods.iter().chain(static_methods).filter(|method| {
                    projected_member_is_demanded(
                        &item.namespace,
                        &item.name,
                        &method.name,
                        demanded_members,
                    )
                }) {
                    collect_foreign_function(method, &mut foreign);
                }
            }
            ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for method in methods.iter().chain(static_methods).filter(|method| {
                    projected_member_is_demanded(
                        &item.namespace,
                        &item.name,
                        &method.name,
                        demanded_members,
                    )
                }) {
                    collect_foreign_function(method, &mut foreign);
                }
            }
            ProjectedKind::Interface(interface) => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for supertrait in &interface.supertraits {
                    foreign.insert(supertrait.rust_path.clone(), supertrait.name.clone());
                }
                for method in &interface.methods {
                    collect_foreign_function(&method.function, &mut foreign);
                }
            }
        }
    }
    loop {
        let previous_len = foreign.len();
        let referenced = foreign.keys().cloned().collect::<Vec<_>>();
        for rust_path in referenced {
            let Some(item) = all_items
                .iter()
                .copied()
                .find(|item| item.rust_path == rust_path)
            else {
                continue;
            };
            if let ProjectedKind::Interface(interface) = &item.kind {
                for supertrait in &interface.supertraits {
                    foreign.insert(supertrait.rust_path.clone(), supertrait.name.clone());
                }
                for method in &interface.methods {
                    collect_foreign_function(&method.function, &mut foreign);
                }
                continue;
            }
            if let ProjectedKind::ForeignType { fields, .. } = &item.kind {
                for field in fields {
                    collect_foreign_type(&field.ty, &mut foreign);
                }
            }
            let methods = match &item.kind {
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => Some((methods, static_methods)),
                _ => None,
            };
            let Some((methods, static_methods)) = methods else {
                continue;
            };
            for method in methods.iter().chain(static_methods).filter(|method| {
                projected_member_is_demanded(
                    &item.namespace,
                    &item.name,
                    &method.name,
                    demanded_members,
                )
            }) {
                collect_foreign_function(method, &mut foreign);
            }
        }
        if foreign.len() == previous_len {
            for item in all_items {
                if let Some(name) = foreign.get_mut(&item.rust_path) {
                    name.clone_from(&item.name);
                }
            }
            return foreign;
        }
    }
}

pub(super) fn foreign_aliases(foreign: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let preferred = foreign
        .iter()
        .map(|(rust_path, name)| {
            (
                rust_path.as_str(),
                preferred_foreign_name(rust_path, name).replace('_', "-"),
            )
        })
        .collect::<Vec<_>>();
    let counts = preferred
        .iter()
        .fold(BTreeMap::new(), |mut counts, (_, name)| {
            *counts.entry(name.clone()).or_insert(0_usize) += 1;
            counts
        });
    let qualified = preferred
        .iter()
        .map(|(rust_path, name)| {
            (counts[name] != 1)
                .then(|| readable_generic_foreign_name(rust_path, name))
                .flatten()
        })
        .collect::<Vec<_>>();
    let qualified_counts = qualified
        .iter()
        .flatten()
        .fold(BTreeMap::new(), |mut counts, name| {
            *counts.entry(name.clone()).or_insert(0_usize) += 1;
            counts
        });
    preferred
        .into_iter()
        .zip(qualified)
        .map(|((rust_path, name), qualified)| {
            let alias = if counts[&name] == 1 {
                name
            } else if let Some(qualified) =
                qualified.filter(|candidate| qualified_counts[candidate] == 1)
            {
                qualified
            } else {
                let digest = format!("{:x}", Sha256::digest(rust_path.as_bytes()));
                format!("{name}-hash-h{}", &digest[..12])
            };
            (rust_path.to_owned(), alias)
        })
        .collect()
}

fn readable_generic_foreign_name(rust_path: &str, name: &str) -> Option<String> {
    let (_, arguments) = rust_path.split_once('<')?;
    let arguments = arguments.strip_suffix('>')?;
    let mut readable = String::with_capacity(name.len() + 4 + arguments.len());
    readable.push_str(name);
    readable.push_str("-of-");
    let mut separator = false;
    for character in arguments.chars() {
        if character.is_ascii_alphanumeric() {
            if separator && !readable.ends_with('-') {
                readable.push('-');
            }
            separator = false;
            readable.push(character);
        } else {
            separator = true;
        }
    }
    (!readable.ends_with("-of-")).then_some(readable)
}

fn preferred_foreign_name(rust_path: &str, projected_name: &str) -> String {
    let constructor = rust_path
        .split_once('<')
        .map_or(rust_path, |(constructor, _)| constructor);
    let short = constructor.rsplit("::").next().unwrap_or(constructor);
    let generated = instantiated_type_name(short, rust_path);
    if generated == projected_name {
        short.to_owned()
    } else {
        projected_name.to_owned()
    }
}

pub(super) fn collect_foreign_function(
    function: &ProjectedFunction,
    foreign: &mut BTreeMap<String, String>,
) {
    for ty in function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
    {
        collect_foreign_type(ty, foreign);
    }
}

fn collect_foreign_type(ty: &ProjectedType, foreign: &mut BTreeMap<String, String>) {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            name,
            arguments,
            ..
        } => {
            foreign.insert(rust_path.clone(), name.clone());
            for argument in arguments {
                collect_foreign_type(argument, foreign);
            }
        }
        ProjectedType::BoxedInterface {
            trait_path,
            name,
            associated_type,
            ..
        } => {
            foreign.insert(trait_path.clone(), name.clone());
            if let Some(associated) = associated_type {
                collect_foreign_type(&associated.ty, foreign);
            }
        }

        ProjectedType::InvocationScoped { owned, name, .. } => {
            if name == "borrowed-option" {
                foreign.insert("std::option::Option".to_owned(), name.clone());
            }
            collect_foreign_type(owned, foreign);
        }
        ProjectedType::Optional(inner)
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. } => collect_foreign_type(inner, foreign),
        ProjectedType::Mapping { key, value, .. } => {
            collect_foreign_type(key, foreign);
            collect_foreign_type(value, foreign);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                collect_foreign_type(item, foreign);
            }
        }
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            for parameter in parameters {
                collect_foreign_type(parameter, foreign);
            }
            collect_foreign_type(result, foreign);
        }
        _ => {}
    }
}
pub(super) fn render_foreign_declaration(
    output: &mut String,
    namespace: &str,
    name: &str,
    projected_item: Option<&ProjectedItem>,
    aliases: &BTreeMap<String, String>,
    demanded_members: &ProjectedMemberDemands,
    unavailable_members: Option<&UnavailableMemberMap<'_>>,
) {
    if let Some(item) = projected_item
        && item.namespace != namespace
    {
        write!(output, "from {} import {}", item.namespace, item.name)
            .expect("writing to a string cannot fail");
        if name != item.name {
            write!(output, " as {name}").expect("writing to a string cannot fail");
        }
        output.push('\n');
        return;
    }
    match projected_item.map(|item| &item.kind) {
        Some(ProjectedKind::Interface(interface)) => {
            writeln!(
                output,
                "{}interface {name}",
                if interface.is_unsafe { "unsafe " } else { "" }
            )
            .expect("writing to a string cannot fail");
            for method in &interface.methods {
                render_interface_method(output, method, aliases);
            }
        }
        projected_kind => {
            writeln!(output, "class {name}").expect("writing to a string cannot fail");
            if let Some(
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    constants,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    constants,
                    ..
                },
            ) = projected_kind
            {
                if let ProjectedKind::ForeignType {
                    fields,
                    constructor,
                    ..
                } = projected_kind.expect("matched projected kind")
                {
                    for field in fields {
                        writeln!(
                            output,
                            "    {} {}",
                            field.name,
                            projected_type_name(&field.ty, aliases)
                        )
                        .expect("writing to a string cannot fail");
                    }
                    if let Some(constructor) = constructor {
                        let mut declaration = constructor.clone();
                        declaration.result = ProjectedType::None;
                        render_function(output, &declaration, false, 4, aliases, None);
                    }
                }
                for constant in constants {
                    render_projected_constant(output, constant, aliases);
                }
                for method in methods.iter().filter(|method| {
                    projected_member_is_demanded(namespace, name, &method.name, demanded_members)
                }) {
                    render_function(output, method, false, 4, aliases, None);
                }
                for method in static_methods.iter().filter(|method| {
                    projected_member_is_demanded(namespace, name, &method.name, demanded_members)
                }) {
                    render_function(output, method, false, 4, aliases, Some(name));
                }
            }
        }
    }
    if unavailable_members.is_some()
        && let Some(item) = projected_item.filter(|item| item.namespace == namespace)
    {
        render_omitted_admitted_members(output, item, demanded_members);
    }
    render_unavailable_members_for_item(output, namespace, projected_item, unavailable_members);
    output.push('\n');
}

fn render_projected_constant(
    output: &mut String,
    constant: &ProjectedConstant,
    aliases: &BTreeMap<String, String>,
) {
    write!(
        output,
        "    constant {} {}",
        constant.name,
        projected_type_name(&constant.ty, aliases)
    )
    .expect("writing to a string cannot fail");
    if let Some(value) = &constant.terrane_value {
        write!(output, " = {value}").expect("writing to a string cannot fail");
    }
    output.push('\n');
}

fn render_unavailable_members_for_item(
    output: &mut String,
    namespace: &str,
    item: Option<&ProjectedItem>,
    unavailable_members: Option<&UnavailableMemberMap<'_>>,
) {
    let Some(item) = item.filter(|item| item.namespace == namespace) else {
        return;
    };
    if let Some(unavailable) =
        unavailable_members.and_then(|members| members.get(&(namespace, item.name.as_str())))
    {
        render_unavailable_members(output, unavailable);
    }
}

pub(super) fn inventory_member_syntax_gap(function: &ProjectedFunction) -> Option<String> {
    let mut text = "namespace projection-inventory\n\nclass Inventory\n".to_owned();
    render_function(&mut text, function, false, 4, &BTreeMap::new(), None);
    let source = crate::SourceFile::new(0, PathBuf::from("terrane-projection.generated.trn"), text);
    let lexed = match crate::lexer::lex(&source) {
        Ok(lexed) => lexed,
        Err(diagnostics) => {
            return diagnostics
                .first()
                .map(|diagnostic| diagnostic.message.clone());
        }
    };
    crate::parser::parse(&source, lexed)
        .diagnostics
        .first()
        .map(|diagnostic| diagnostic.message.clone())
}

fn render_omitted_admitted_members(
    output: &mut String,
    item: &ProjectedItem,
    demanded_members: &ProjectedMemberDemands,
) {
    let functions = match &item.kind {
        ProjectedKind::ForeignType {
            methods,
            static_methods,
            ..
        }
        | ProjectedKind::Enum {
            methods,
            static_methods,
            ..
        } => methods.iter().chain(static_methods),
        _ => return,
    };
    let omitted = functions
        .filter(|function| {
            !projected_member_is_demanded(
                &item.namespace,
                &item.name,
                &function.name,
                demanded_members,
            )
        })
        .collect::<Vec<_>>();
    if omitted.is_empty() {
        return;
    }
    output.push_str("  #\n  # Admitted native members without syntax-valid generated source\n");
    for function in omitted {
        writeln!(output, "  #\n  # {}", function.name).expect("writing to a string cannot fail");
        writeln!(
            output,
            "  # Native path: {}::{}",
            item.rust_path, function.name
        )
        .expect("writing to a string cannot fail");
        let reason = inventory_member_syntax_gap(function)
            .unwrap_or_else(|| "not selected for generated inventory source".to_owned());
        writeln!(output, "  # Inventory omission: {reason}")
            .expect("writing to a string cannot fail");
    }
}

fn render_unavailable_members(output: &mut String, unavailable: &[&UnavailableProjection]) {
    output.push_str("  #\n  # Unavailable native members retained for structure\n");
    for entry in unavailable {
        let member = entry.member.as_deref().unwrap_or(&entry.name);
        writeln!(output, "  #\n  # {member}").expect("writing to a string cannot fail");
        writeln!(output, "  # Native path: {}", entry.rust_path)
            .expect("writing to a string cannot fail");
        if let Some(PartialProjection::Function { signature, .. }) = &entry.partial {
            writeln!(output, "  # Native signature: {signature}")
                .expect("writing to a string cannot fail");
        }
        writeln!(output, "  # Projection gap: {}", entry.reason)
            .expect("writing to a string cannot fail");
    }
}

fn render_interface_method(
    output: &mut String,
    method: &ProjectedInterfaceMethod,
    foreign_aliases: &BTreeMap<String, String>,
) {
    if let Some(docs) = &method.docs {
        for line in docs.lines() {
            writeln!(output, "  ## {}", line.trim()).expect("writing to a string cannot fail");
        }
    }
    let function = &method.function;
    let mode = match function.receiver {
        Some(Receiver::MutableBorrow) => "mutable ",
        Some(Receiver::Move) => "consuming ",
        _ => "",
    };
    let asynchronous = if function.is_async { "async " } else { "" };
    write!(output, "  {mode}{asynchronous}function {}", function.name)
        .expect("writing to a string cannot fail");
    if function.result != ProjectedType::None {
        write!(
            output,
            " {}",
            projected_type_name(&function.result, foreign_aliases)
        )
        .expect("writing to a string cannot fail");
    }
    if function.error.is_some() {
        output.push_str(" throws dependency-error");
    }
    output.push(';');
    if !function.parameters.is_empty() {
        output.push(' ');
        for (index, parameter) in function.parameters.iter().enumerate() {
            if index > 0 {
                output.push_str(", ");
            }
            write!(
                output,
                "{} {}",
                parameter.name,
                projected_type_name(&parameter.ty, foreign_aliases)
            )
            .expect("writing to a string cannot fail");
        }
    }
    output.push('\n');
}

pub(super) fn foreign_function_dependency_count(
    function: &ProjectedFunction,
    owner: &str,
) -> usize {
    function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
        .filter(|ty| foreign_type_name(ty).is_some_and(|name| name != owner))
        .count()
}

fn foreign_type_name(ty: &ProjectedType) -> Option<&str> {
    match ty {
        ProjectedType::Foreign { name, .. } => Some(name),
        ProjectedType::Optional(inner)
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. } => foreign_type_name(inner),
        ProjectedType::Mapping { key, value, .. } => {
            foreign_type_name(key).or_else(|| foreign_type_name(value))
        }
        ProjectedType::Tuple(items) => items.iter().find_map(foreign_type_name),
        _ => None,
    }
}

pub(super) fn render_function(
    output: &mut String,
    function: &ProjectedFunction,
    public: bool,
    indent: usize,
    foreign_aliases: &BTreeMap<String, String>,
    static_owner: Option<&str>,
) {
    let prefix = " ".repeat(indent);
    let visibility = if public { "public " } else { "" };
    let unsafe_ = if function.is_unsafe { "unsafe " } else { "" };
    let static_ = if static_owner.is_some() {
        "static "
    } else {
        ""
    };
    let asynchronous = if function.is_async { "async " } else { "" };
    write!(
        output,
        "{prefix}{visibility}{unsafe_}{static_}{asynchronous}function {}",
        function.name
    )
    .expect("writing to a string cannot fail");
    if function.result != ProjectedType::None
        && (function.destination_result.is_none()
            || matches!(function.result, ProjectedType::InvocationScoped { .. })
            || function.receiver.is_some())
        && !(function.chain_role == Some(ChainRole::Root)
            && matches!(
                function.result,
                ProjectedType::Opaque {
                    anonymous_chain: true,
                    ..
                }
            ))
    {
        write!(
            output,
            " {}",
            projected_type_name(&function.result, foreign_aliases)
        )
        .expect("writing to a string cannot fail");
    }
    if function.error.is_some() {
        output.push_str(" throws dependency-error");
    }
    output.push(';');
    if !function.parameters.is_empty() {
        output.push(' ');
        for (index, parameter) in function.parameters.iter().enumerate() {
            if index != 0 {
                output.push_str(", ");
            }
            write!(
                output,
                "{} {}{}",
                parameter.name,
                if parameter.borrowed
                    && matches!(function.result, ProjectedType::InvocationScoped { .. })
                {
                    "ref "
                } else {
                    ""
                },
                projected_parameter_type_name(&parameter.ty, foreign_aliases)
            )
            .expect("writing to a string cannot fail");
        }
    }
    output.push('\n');
}

fn projected_parameter_type_name(ty: &ProjectedType, aliases: &BTreeMap<String, String>) -> String {
    match ty {
        ProjectedType::Optional(inner) => {
            format!("{}|none", projected_parameter_type_name(inner, aliases))
        }
        ProjectedType::Sequence { .. }
        | ProjectedType::Mapping { .. }
        | ProjectedType::Set { .. }
        | ProjectedType::Tuple(_)
        | ProjectedType::AsyncIterationStep(_)
        | ProjectedType::BoxedInterface {
            associated_type: Some(_),
            ..
        } => {
            format!("({})", projected_type_name(ty, aliases))
        }
        ProjectedType::InvocationScoped {
            owned,
            expression_scoped: true,
            ..
        } if !matches!(owned.as_ref(), ProjectedType::Optional(_)) => {
            projected_parameter_type_name(owned, aliases)
        }
        _ => projected_type_name(ty, aliases),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "projected source type rendering exhaustively covers one closed type model"
)]
fn projected_type_name(ty: &ProjectedType, foreign_aliases: &BTreeMap<String, String>) -> String {
    match ty {
        ProjectedType::Foreign {
            rust_path, name, ..
        } => foreign_aliases
            .get(rust_path)
            .cloned()
            .unwrap_or_else(|| name.clone()),
        ProjectedType::InvocationScoped {
            owned,
            name,
            expression_scoped: true,
            ..
        } => {
            if matches!(owned.as_ref(), ProjectedType::Optional(_)) {
                name.clone()
            } else {
                projected_type_name(owned, foreign_aliases)
            }
        }
        ProjectedType::InvocationScoped { .. } => "host-invocation-scoped-native".to_owned(),
        ProjectedType::BoxedInterface {
            trait_path,
            name,
            associated_type,
            ..
        } => {
            let base = foreign_aliases
                .get(trait_path)
                .cloned()
                .unwrap_or_else(|| name.clone());
            associated_type.as_ref().map_or(base.clone(), |associated| {
                format!(
                    "{base} of {}",
                    projected_type_name(&associated.ty, foreign_aliases)
                )
            })
        }
        ProjectedType::Optional(inner) => {
            format!("{}|none", projected_type_name(inner, foreign_aliases))
        }
        ProjectedType::AsyncIterationStep(inner) => {
            format!(
                "async-iteration-step of {}",
                projected_type_name(inner, foreign_aliases)
            )
        }
        ProjectedType::Sequence { item, .. } => {
            format!("list of {}", projected_type_name(item, foreign_aliases))
        }
        ProjectedType::Mapping {
            key,
            value,
            ordered,
            ..
        } => format!(
            "{}map of {}, {}",
            if *ordered { "" } else { "unordered-" },
            projected_type_name(key, foreign_aliases),
            projected_type_name(value, foreign_aliases)
        ),
        ProjectedType::Set { item, ordered, .. } => format!(
            "{}set of {}",
            if *ordered { "" } else { "unordered-" },
            projected_type_name(item, foreign_aliases)
        ),
        ProjectedType::Tuple(items) => format!(
            "tuple of {}",
            projected_type_name(&items[0], foreign_aliases)
        ),
        ProjectedType::Callback {
            parameters,
            parameter_borrows,
            result,
            invocation_mode,
            is_async,
            ..
        } => {
            let mut rendered = format!(
                "{}{}function",
                invocation_mode.source_prefix(),
                if *is_async { "async " } else { "" }
            );
            if !parameters.is_empty() {
                write!(
                    rendered,
                    " from {}",
                    parameters
                        .iter()
                        .enumerate()
                        .map(|(index, parameter)| format!(
                            "{}{}",
                            if parameter_borrows.get(index) == Some(&true) {
                                "ref "
                            } else {
                                ""
                            },
                            projected_type_name(parameter, foreign_aliases)
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .expect("writing to a string cannot fail");
            }
            write!(
                rendered,
                " to {}",
                projected_type_name(result, foreign_aliases)
            )
            .expect("writing to a string cannot fail");
            rendered
        }
        _ => ty.terrane_name(),
    }
}
