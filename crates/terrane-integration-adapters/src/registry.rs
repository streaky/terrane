//! Central accounting for temporary ecosystem integration gaps.
//!
//! Entries are deliberately data, not hooks in generic projection or lowering. A bridged gap
//! points at an ordinary feature-gated module in this crate. An unbridged gap records where the
//! normal dependency projection path still declines a useful shape. Removing an entry requires
//! satisfying its removal criterion with generic regression coverage.

/// Whether a recorded gap is still unbridged or supplied by an adapter package feature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterStatus {
    Unbridged,
    Package {
        name: &'static str,
        version: &'static str,
        feature: &'static str,
    },
}

/// One independently removable ecosystem integration gap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegrationAdapter {
    pub id: &'static str,
    pub tracking_key: &'static str,
    pub dependency: &'static str,
    pub supported_versions: &'static str,
    pub limitation: &'static str,
    pub status: AdapterStatus,
    pub removal_criterion: &'static str,
}

/// Every bridged and known unbridged ecosystem integration gap.
pub const INTEGRATION_ADAPTERS: &[IntegrationAdapter] = &[
    IntegrationAdapter {
        id: "sqlx-sqlite-query-lifetimes",
        tracking_key: "collections/consume-non-clone-foreign-items",
        dependency: "sqlx",
        supported_versions: "0.8.x with SQLite and Tokio runtime",
        limitation: "`query_bytes` remains only because Terrane cannot yet consume each non-Clone row from a persistent list returned by direct `fetch_all`; query, bind, execute, fetch, row access, typed errors, and connection lifecycle are direct",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "sqlx-sqlite",
        },
        removal_criterion: "S1 consuming collection or stream iteration lets Terrane process every non-Clone row returned by fetch-all without adapter-owned conversion",
    },
    IntegrationAdapter {
        id: "projected-mutual-namespace-cycle",
        tracking_key: "projection/mutually-referential-namespace-sources",
        dependency: "generic Rust dependencies",
        supported_versions: "all projected Rustdoc graphs",
        limitation: "demanded methods whose signatures make generated Terrane dependency namespaces mutually import one another are declined because projected source units are currently ordered as an acyclic graph",
        status: AdapterStatus::Unbridged,
        removal_criterion: "projected dependency declarations use semantic descriptors or another representation that resolves mutually referential namespaces without cyclic generated-source ordering",
    },
    IntegrationAdapter {
        id: "godot-generated-api-projection",
        tracking_key: "projection/godot-generated-reexport-surface",
        dependency: "godot",
        supported_versions: "0.5.x",
        limitation: "godot's macro-generated and reexported engine API currently produces no directly projectable Terrane members",
        status: AdapterStatus::Unbridged,
        removal_criterion: "generic dependency projection exposes representative godot builtin and class types without a package-specific projector",
    },
    IntegrationAdapter {
        id: "godot-gdextension-registration",
        tracking_key: "native-interop/rust-proc-macro-extension-entry",
        dependency: "godot",
        supported_versions: "0.5.x",
        limitation: "GDExtension entrypoint and GodotClass registration require Rust attribute and derive macros that Terrane source cannot declare",
        status: AdapterStatus::Unbridged,
        removal_criterion: "a generic native-extension contract can generate a cdylib entrypoint and registered host class without a maintained Rust module",
    },
];

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{AdapterStatus, INTEGRATION_ADAPTERS};

    #[test]
    fn adapter_ids_and_tracking_keys_are_unique() {
        let mut ids = BTreeSet::new();
        let mut tracking_keys = BTreeSet::new();
        for entry in INTEGRATION_ADAPTERS {
            assert!(ids.insert(entry.id), "duplicate adapter id {}", entry.id);
            assert!(
                tracking_keys.insert(entry.tracking_key),
                "duplicate adapter tracking key {}",
                entry.tracking_key
            );
            assert!(!entry.removal_criterion.is_empty());
        }
    }

    #[test]
    fn shipped_records_name_the_owning_package_and_feature() {
        for entry in INTEGRATION_ADAPTERS {
            let AdapterStatus::Package {
                name,
                version,
                feature,
            } = entry.status
            else {
                continue;
            };
            assert_eq!(name, env!("CARGO_PKG_NAME"));
            assert!(
                !version.is_empty(),
                "adapter {} has no package version",
                entry.id
            );
            assert!(
                !feature.is_empty(),
                "adapter {} has no package feature",
                entry.id
            );
        }
    }
}
