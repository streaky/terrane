//! Native representations of core-owned resources, independent of ecosystem packages.
use super::{
    ProjectedBoundaryCapabilities, ProjectedDependency, ProjectedItem, ProjectedKind, Projection,
};
use std::collections::BTreeMap;

pub(crate) const NAMESPACE: &str = "/core/networking/native";

pub(crate) fn rust_type(namespace: &str, name: &str) -> Option<&'static str> {
    if namespace != NAMESPACE {
        return None;
    }
    match name {
        "native-tcp-listener" => Some("tokio::net::TcpListener"),
        "native-tcp-stream" => Some("tokio::net::TcpStream"),
        "native-socket-address" => Some("std::net::SocketAddr"),
        "host-listener-transfer" => Some("std::io::Result<tokio::net::TcpListener>"),
        "host-stream-transfer" => Some("std::io::Result<tokio::net::TcpStream>"),
        _ => None,
    }
}

pub(crate) fn augment(projection: &mut Projection) {
    let items = [
        "native-tcp-listener",
        "native-tcp-stream",
        "native-socket-address",
        "host-listener-transfer",
        "host-stream-transfer",
    ]
    .into_iter()
    .map(|name| ProjectedItem {
        namespace: NAMESPACE.to_owned(),
        name: name.to_owned(),
        rust_path: projection
            .canonical_native_type(rust_type(NAMESPACE, name).expect("registered core native type"))
            .into_owned(),
        docs: None,
        kind: ProjectedKind::ForeignType {
            methods: Vec::new(),
            static_methods: Vec::new(),
            constants: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
            fields: Vec::new(),
            constructor: None,
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            displayable: name == "native-socket-address",
            cloneable: name == "native-socket-address",
            send: true,
            sync: true,
            generic_parameters: Vec::new(),
        },
    })
    .collect();
    projection.dependencies.push(ProjectedDependency {
        name: "core-networking-native".to_owned(),
        package: "core-networking-native".to_owned(),
        version: crate::VERSION.to_owned(),
        items,
        declined: Vec::new(),
        native_alias_identities: BTreeMap::new(),
        partial_declines: Vec::new(),
    });
    projection.content_hash = super::artifacts::projection_content_hash(projection)
        .expect("core native metadata has a JSON representation");
}

pub(crate) fn prepare_runtime(
    package: &crate::package::Package,
    projection: &mut Projection,
) -> Result<(), super::ProjectionError> {
    if package.rust_dependencies.is_empty() {
        return Ok(());
    }
    let workspace = package.root.join(".trn/dependencies");
    let versions = super::cargo_workspace::resolved_package_versions(&workspace)?;
    let default_runtime = crate::with_tokio_runtime(&[], &["net"])
        .into_iter()
        .next()
        .expect("runtime dependency");
    let version = match versions.get("tokio") {
        Some(versions) if versions.len() == 1 => {
            format!("={}", versions.first().expect("one version"))
        }
        Some(_) => {
            return Err(super::ProjectionError {
                message: "native core TCP interoperability requires one resolved Tokio owner"
                    .to_owned(),
            });
        }
        None => default_runtime.version,
    };
    let name = package
        .rust_dependencies
        .iter()
        .find(|dependency| dependency.package == "tokio")
        .map(|dependency| dependency.name.clone())
        .or_else(|| {
            projection
                .bound_dependencies
                .iter()
                .find(|dependency| dependency.package == "tokio")
                .map(|dependency| dependency.name.clone())
        })
        .unwrap_or_else(|| "__terrane_runtime_tokio".to_owned());
    if let Some(existing) = projection
        .bound_dependencies
        .iter_mut()
        .find(|dependency| dependency.name == name)
    {
        if !existing.features.iter().any(|feature| feature == "net") {
            existing.features.push("net".to_owned());
        }
    } else {
        projection
            .bound_dependencies
            .push(super::ProjectedBoundDependency {
                name: name.clone(),
                package: "tokio".to_owned(),
                version,
                features: vec!["net".to_owned()],
            });
    }
    for native in [
        "native-tcp-listener",
        "native-tcp-stream",
        "host-listener-transfer",
        "host-stream-transfer",
    ] {
        let rust = rust_type(NAMESPACE, native).expect("registered native transport");
        let owner = super::dependency_owners::rewrite_rust_bound_root(rust, "tokio", &name);
        projection
            .native_owner_aliases
            .insert(rust.to_owned(), owner);
    }
    super::cargo_workspace::write_workspace_with_bound_dependencies(
        &workspace,
        &package.rust_dependencies,
        &projection.bound_dependencies,
    )?;
    Ok(())
}
