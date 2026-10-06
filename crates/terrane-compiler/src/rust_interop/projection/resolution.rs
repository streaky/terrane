//! Owns the end-to-end Rust dependency resolution transaction.
/// Resolves every declared Rust package and derives the shared Terrane projection.
///
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use super::{
    CargoExecution, CargoToolchain, DeclinedItem, ProjectedDependency, ProjectedKind, Projection,
    ProjectionCacheIdentity, ProjectionError, ProjectionResolution, ProjectionSource,
    PublishedProjection, ResolutionEvent, ResolutionOutcome, ResolutionSource, ResolutionStatus,
    RustDependency, aliases, apply_namespace_overlays, borrowed_graph, cache_identity,
    canonicalize_projected_type_names, containment, decline_unnameable_bound_owners,
    decline_unrepresentable_error_types, enforce_transitive_reachability,
    external_reexport_rustdocs, fetch_remote_projection, foreign_impls, generate_rustdoc, history,
    namespace_overlays_from_metadata, normalize_projected_items, persist_dependency_lock,
    prefer_alias, project_external_provided_trait_methods, project_rustdoc,
    projected_bound_dependencies, projected_interface_impl_question, projection_content_hash,
    provider_fragment_public_paths, recursive_owner_dependencies, remove_legacy_projection_cache,
    resolve_cross_dependency_boundary_conversions, resolved_dependency_metadata,
    rewrite_projected_owner_root, run_cargo, rustdoc_public_paths, seed_dependency_lock,
    write_cache_atomically, write_workspace, write_workspace_with_bound_dependencies,
};
/// # Errors
/// Returns a projection error when Cargo resolution, rustdoc generation, cache input reading, or
/// projection of the resolved metadata fails.
///
/// # Panics
/// Panics only when internally derived reexport-provider indices no longer address the declared
/// and projected dependency vectors built from the same resolved graph.
#[expect(
    clippy::too_many_lines,
    reason = "one transactional resolution path owns fetch, exact cache, artifact, and local fallback"
)]
pub fn resolve(
    root: &Path,
    dependencies: &[RustDependency],
    demands: Option<&BTreeSet<(String, String)>>,
) -> Result<Projection, ProjectionError> {
    let sandbox = containment();
    if dependencies.is_empty() {
        let mut projection = Projection {
            native_owner_aliases: BTreeMap::default(),
            cache_identity: String::from("no-rust-dependencies"),
            content_hash: String::new(),
            source: ProjectionSource::Local,
            dependencies: Vec::new(),
            probes: Vec::new(),
            bound_dependencies: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution {
                outcome: ResolutionOutcome::NoDependencies,
                events: Vec::new(),
            },
            removed: Vec::new(),
            containment: sandbox,
        };
        projection.content_hash = projection_content_hash(&projection)?;
        return Ok(projection);
    }
    // The declared manifest and resolved lock are hashed into the projection identity before any
    // review-visible bound-owner edges are injected. Once such edges exist, Cargo cannot accept
    // that deliberate manifest rewrite under `--locked`; offline resolution plus exact pins and
    // the projection content hash preserve the already-resolved graph without network drift.
    // Path-dependency source and package-metadata contents are deliberately outside this identity:
    // changing either without changing the consumer manifest or lock requires clearing the
    // projection cache. Namespace-overlay metadata follows that existing invalidation boundary so
    // warm cache hits remain metadata-free.
    let workspace = root.join(".trn/dependencies");
    seed_dependency_lock(root, &workspace)?;
    write_workspace(&workspace, dependencies)?;
    if workspace.join("Cargo.lock").exists() {
        run_cargo(
            &workspace,
            &["fetch", "--offline"],
            CargoToolchain::Default,
            CargoExecution::Host,
        )?;
    } else {
        run_cargo(
            &workspace,
            &["fetch"],
            CargoToolchain::Default,
            CargoExecution::Host,
        )?;
    }
    persist_dependency_lock(root, &workspace)?;
    let (identity, target) = cache_identity(root, &workspace, dependencies, demands, sandbox)?;
    let cache_path = workspace.join("projection.json");
    if let Ok(bytes) = fs::read(&cache_path) {
        let matches_identity = serde_json::from_slice::<ProjectionCacheIdentity<'_>>(&bytes)
            .is_ok_and(|header| header.cache_identity == identity);
        if matches_identity && let Ok(mut cached) = serde_json::from_slice::<Projection>(&bytes) {
            let actual_hash = projection_content_hash(&cached)?;
            if cached.content_hash != actual_hash {
                return Err(ProjectionError {
                    message: format!(
                        "cached dependency projection `{}` failed its content hash: expected `{}`, computed `{actual_hash}`",
                        cache_path.display(),
                        cached.content_hash
                    ),
                });
            }
            cached.containment = sandbox;
            cached.resolution = ProjectionResolution {
                outcome: ResolutionOutcome::ExactCache,
                events: vec![ResolutionEvent {
                    source: ResolutionSource::ExactCache,
                    status: ResolutionStatus::Hit,
                    reason: format!("matched projection identity `{identity}`"),
                }],
            };
            write_workspace_with_bound_dependencies(
                &workspace,
                dependencies,
                &cached.bound_dependencies,
            )?;
            persist_dependency_lock(root, &workspace)?;
            history::apply_projection_history(root, &mut cached)?;
            remove_legacy_projection_cache(&workspace)?;
            return Ok(cached);
        }
    }

    let mut resolution_events = vec![ResolutionEvent {
        source: ResolutionSource::BundledArtifact,
        status: ResolutionStatus::Skipped,
        reason: "bundled projection distribution is deliberately deferred until Terrane has a release artifact channel".to_owned(),
    }];
    match fetch_remote_projection(&identity, &target, dependencies, sandbox)? {
        PublishedProjection::Hit(mut projection) => {
            resolution_events.push(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Hit,
                reason: format!(
                    "verified identity and content hash `{}`",
                    projection.content_hash
                ),
            });
            projection.resolution = ProjectionResolution {
                outcome: ResolutionOutcome::PublishedArtifact,
                events: resolution_events,
            };
            write_workspace_with_bound_dependencies(
                &workspace,
                dependencies,
                &projection.bound_dependencies,
            )?;
            persist_dependency_lock(root, &workspace)?;
            let bytes =
                serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
                    message: format!("cannot serialize published dependency projection: {error}"),
                })?;
            write_cache_atomically(&cache_path, &[&bytes])?;
            history::apply_projection_history(root, &mut projection)?;
            remove_legacy_projection_cache(&workspace)?;
            projection
                .probes
                .sort_by(|left, right| left.question.cmp(&right.question));
            return Ok(projection);
        }
        PublishedProjection::Event(event) => resolution_events.push(event),
    }

    let metadata = resolved_dependency_metadata(&workspace)?;
    let overlays = namespace_overlays_from_metadata(&metadata, dependencies)?;

    let mut rustdocs = Vec::new();
    for dependency in dependencies {
        let package_spec = dependency.version.strip_prefix('=').map_or_else(
            || dependency.package.clone(),
            |version| format!("{}@{version}", dependency.package),
        );
        let crate_name = dependency.package.replace('-', "_");
        let document = generate_rustdoc(
            &workspace,
            &package_spec,
            &crate_name,
            &dependency.package,
            sandbox,
            false,
        )?;
        let public_paths = rustdoc_public_paths(&document);
        rustdocs.push((dependency, document, public_paths));
    }
    let mut declared_public_paths = BTreeMap::new();
    for (_, document, public_paths) in &rustdocs {
        for (id, public_path) in public_paths {
            let Some(summary) = document
                .paths
                .get(id)
                .filter(|summary| summary.crate_id == 0)
            else {
                continue;
            };
            declared_public_paths
                .entry(summary.path.join("::"))
                .and_modify(|current| prefer_alias(current, public_path))
                .or_insert_with(|| public_path.clone());
        }
    }
    let canonical_public_paths = vec![declared_public_paths.clone(); dependencies.len()];
    let mut projected = rustdocs
        .iter()
        .enumerate()
        .map(|(dependency_index, (dependency, document, public_paths))| {
            project_rustdoc(
                dependency,
                document,
                public_paths,
                &canonical_public_paths[dependency_index],
                true,
            )
        })
        .collect::<Vec<_>>();
    let mut reexport_declines = (0..dependencies.len())
        .map(|_| Vec::new())
        .collect::<Vec<_>>();
    let reexport_rustdocs = external_reexport_rustdocs(
        &workspace,
        &rustdocs,
        &projected,
        &metadata,
        &target,
        sandbox,
        demands,
        &mut reexport_declines,
    )?;
    for (dependency, mut declines) in projected.iter_mut().zip(reexport_declines) {
        dependency.declined.append(&mut declines);
        normalize_projected_items(&mut dependency.items, &mut dependency.declined);
    }
    for reexport in &reexport_rustdocs {
        for provider in &reexport.providers {
            for (facade_path, canonical_path) in &provider.rust_path_aliases {
                rewrite_projected_owner_root(
                    std::slice::from_mut(
                        projected
                            .get_mut(provider.dependency_index)
                            .expect("reexport provider index came from projected dependencies"),
                    ),
                    facade_path,
                    canonical_path,
                );
            }
            let fragment_public_paths =
                provider_fragment_public_paths(&declared_public_paths, provider);
            let mut fragment = project_rustdoc(
                dependencies
                    .get(provider.dependency_index)
                    .expect("reexport provider index came from declared dependencies"),
                &reexport.document,
                &provider.public_paths,
                &fragment_public_paths,
                false,
            );
            for (facade_path, canonical_path) in &provider.rust_path_aliases {
                rewrite_projected_owner_root(
                    std::slice::from_mut(&mut fragment),
                    facade_path,
                    canonical_path,
                );
            }
            let dependency = projected
                .get_mut(provider.dependency_index)
                .expect("reexport provider index came from projected dependencies");
            dependency.items.append(&mut fragment.items);
            dependency.declined.append(&mut fragment.declined);
            normalize_projected_items(&mut dependency.items, &mut dependency.declined);
        }
    }
    project_external_provided_trait_methods(&mut projected, &rustdocs, &canonical_public_paths);
    let native_owner_aliases = foreign_impls::project_foreign_owner_impls(
        &mut projected,
        &rustdocs,
        &canonical_public_paths,
        &reexport_rustdocs,
    );
    aliases::project_closed_alias_members(
        &mut projected,
        &rustdocs,
        &reexport_rustdocs,
        &declared_public_paths,
    );
    for dependency in &mut projected {
        borrowed_graph::add_optional_owners(&mut dependency.items);
    }
    apply_namespace_overlays(&mut projected, &overlays)?;
    resolve_cross_dependency_boundary_conversions(&mut projected);
    for dependency in dependencies {
        rewrite_projected_owner_root(
            &mut projected,
            &dependency.package.replace('-', "_"),
            &dependency.name.replace('-', "_"),
        );
    }
    let mut bound_dependencies =
        recursive_owner_dependencies(&projected, dependencies, &workspace, false)?;
    for dependency in &bound_dependencies {
        rewrite_projected_owner_root(
            &mut projected,
            &dependency.package.replace('-', "_"),
            &dependency.name,
        );
    }
    enforce_transitive_reachability(
        &mut projected,
        dependencies,
        &bound_dependencies,
        &workspace,
        false,
    )?;
    canonicalize_projected_type_names(&mut projected);
    bound_dependencies.sort_by(|left, right| left.name.cmp(&right.name));
    enforce_transitive_reachability(
        &mut projected,
        dependencies,
        &bound_dependencies,
        &workspace,
        true,
    )?;
    decline_unrepresentable_error_types(&mut projected);
    let auto_trait_questions = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| {
            matches!(
                item.kind,
                ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
            )
        })
        .flat_map(|item| {
            ["core::marker::Send", "core::marker::Sync"]
                .into_iter()
                .map(|rust_bound| crate::rust_interop::BoundQuestion {
                    rust_type: item.rust_path.clone(),
                    rust_bound: rust_bound.to_owned(),
                    inferred_parameters: Vec::new(),
                })
        })
        .collect::<Vec<_>>();
    if !auto_trait_questions.is_empty() {
        let report = crate::rust_interop::ProjectionOracle::new(&workspace, &identity, sandbox)
            .prove_bounds(&auto_trait_questions)?;
        for evidence in report.evidence {
            let satisfied = evidence.answer == crate::rust_interop::ProbeAnswer::Yes;
            for item in projected
                .iter_mut()
                .flat_map(|dependency| &mut dependency.items)
                .filter(|item| item.rust_path == evidence.question.rust_type)
            {
                match &mut item.kind {
                    ProjectedKind::ForeignType { send, sync, .. }
                    | ProjectedKind::Enum { send, sync, .. } => {
                        match evidence.question.rust_bound.as_str() {
                            "core::marker::Send" => *send = satisfied,
                            "core::marker::Sync" => *sync = satisfied,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    let impl_questions = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match &item.kind {
            ProjectedKind::Interface(interface) => {
                Some(projected_interface_impl_question(item, interface))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if !impl_questions.is_empty() {
        let report = crate::rust_interop::ProjectionOracle::new(&workspace, &identity, sandbox)
            .prove_impls(&impl_questions)?;
        decline_unproven_projected_interfaces(&mut projected, &report.evidence);
    }
    resolution_events.push(ResolutionEvent {
        source: ResolutionSource::LocalRustdoc,
        status: ResolutionStatus::Generated,
        reason: "no reusable exact artifact was available; generated with the pinned local rustdoc toolchain".to_owned(),
    });
    decline_unnameable_bound_owners(
        &mut projected,
        dependencies,
        &bound_dependencies,
        &workspace,
    )?;
    decline_functions_with_missing_generic_interfaces(&mut projected);
    for dependency in
        projected_bound_dependencies(&projected, dependencies, &bound_dependencies, &workspace)?
    {
        if !bound_dependencies
            .iter()
            .any(|existing| existing.name == dependency.name)
        {
            bound_dependencies.push(dependency);
        }
    }
    bound_dependencies.sort_by(|left, right| left.name.cmp(&right.name));
    let mut projection = Projection {
        native_owner_aliases,
        cache_identity: identity,
        content_hash: String::new(),
        source: ProjectionSource::Local,
        dependencies: projected,
        bound_dependencies,
        containment: sandbox,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution {
            outcome: ResolutionOutcome::LocalRustdoc,
            events: resolution_events,
        },
        removed: Vec::new(),
    };
    write_workspace_with_bound_dependencies(
        &workspace,
        dependencies,
        &projection.bound_dependencies,
    )?;
    persist_dependency_lock(root, &workspace)?;
    projection.content_hash = projection_content_hash(&projection)?;
    let bytes = serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
        message: format!("cannot serialize dependency projection: {error}"),
    })?;
    write_cache_atomically(&cache_path, &[&bytes])?;
    history::apply_projection_history(root, &mut projection)?;
    remove_legacy_projection_cache(&workspace)?;
    Ok(projection)
}
fn decline_unproven_projected_interfaces(
    projected: &mut [ProjectedDependency],
    evidence: &[crate::rust_interop::ImplProbeEvidence],
) {
    for evidence in evidence
        .iter()
        .filter(|evidence| evidence.answer != crate::rust_interop::ProbeAnswer::Yes)
    {
        for dependency in &mut *projected {
            let Some(index) = dependency
                .items
                .iter()
                .position(|item| item.rust_path == evidence.question.label)
            else {
                continue;
            };
            let item = dependency.items.remove(index);
            dependency.declined.push(DeclinedItem {
                rust_path: item.rust_path,
                reason:
                    "trait implementation signature is not representable against the resolved dependency"
                        .to_owned(),
            });
            break;
        }
    }
}
fn decline_functions_with_missing_generic_interfaces(projected: &mut [ProjectedDependency]) {
    let projected_interfaces = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| matches!(item.kind, ProjectedKind::Interface(_)))
        .map(|item| item.rust_path.clone())
        .collect::<BTreeSet<_>>();
    for dependency in projected {
        let mut retained = Vec::with_capacity(dependency.items.len());
        for item in std::mem::take(&mut dependency.items) {
            let declined_bound = match &item.kind {
                ProjectedKind::Function(function) => function
                    .parameters
                    .iter()
                    .filter_map(|parameter| parameter.generic_interface.as_ref())
                    .find(|bound| !projected_interfaces.contains(*bound))
                    .cloned(),
                _ => None,
            };
            if let Some(bound) = declined_bound {
                dependency.declined.push(DeclinedItem {
                    rust_path: item.rust_path,
                    reason: format!("generic input references declined interface `{bound}`"),
                });
            } else {
                retained.push(item);
            }
        }
        dependency.items = retained;
    }
}
#[cfg(test)]
mod tests;
