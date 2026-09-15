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
        id: "sqlx-sqlite-connection-traits",
        tracking_key: "projection/sqlx-connection-trait-members",
        dependency: "sqlx-sqlite",
        supported_versions: "0.8.x with bundled SQLite",
        limitation: "SqliteConnection projects as its upstream identity, but trait-provided connect and close operations do not yet project as callable members",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "sqlx-sqlite",
        },
        removal_criterion: "generic trait-member projection lets Terrane construct and explicitly close the directly projected SqliteConnection",
    },
    IntegrationAdapter {
        id: "sqlx-sqlite-query-lifetimes",
        tracking_key: "projection/sqlx-query-lifetime-chain",
        dependency: "sqlx",
        supported_versions: "0.8.x with SQLite and Tokio runtime",
        limitation: "Query<'q, DB, A> and its borrowed argument state cannot cross a free-standing Terrane value boundary",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "sqlx-sqlite",
        },
        removal_criterion: "generic chain-only lowering keeps SQLx query, bind, and execute intermediates inside one generated Rust expression",
    },
    IntegrationAdapter {
        id: "sqlx-sqlite-row-extraction",
        tracking_key: "projection/sqlx-row-generic-extraction",
        dependency: "sqlx-core",
        supported_versions: "0.8.x",
        limitation: "Row::try_get<T, I> requires generic result and column-index selection that the projected call cannot yet close from an application destination",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "sqlx-sqlite",
        },
        removal_criterion: "generic projected trait calls infer bytes row results and string column indices for direct Row::try_get use",
    },
    IntegrationAdapter {
        id: "axum-closed-response-alias",
        tracking_key: "projection/closed-alias-canonical-identity",
        dependency: "axum and axum-core",
        supported_versions: "0.8.x and 0.5.x",
        limitation: "Axum's closed Response alias cannot yet be used as a projected Terrane handler result",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "axum-08",
        },
        removal_criterion: "generic closed-alias projection lets a Terrane WebSocket upgrade handler return Axum's Response alias directly",
    },
    IntegrationAdapter {
        id: "axum-serve-into-future",
        tracking_key: "projection/associated-bound-result-inference",
        dependency: "axum",
        supported_versions: "0.8.x",
        limitation: "serve closes its service bounds but returns a value whose projected IntoFuture implementation is not awaitable",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "axum-08",
        },
        removal_criterion: "generic projected IntoFuture lowering compiles and awaits direct axum::serve with a Router",
    },
    IntegrationAdapter {
        id: "axum-websocket-message-shapes",
        tracking_key: "projection/websocket-enum-stream-sink",
        dependency: "axum",
        supported_versions: "0.8.x with ws",
        limitation: "WebSocket::recv nests optional and fallible results, while payload-bearing Message variants have no projected Terrane constructors",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "axum-08",
        },
        removal_criterion: "generic enum constructors and optional/result async member projection support the same bounded Terrane-authored echo session directly",
    },
    IntegrationAdapter {
        id: "tokio-listener-static-methods",
        tracking_key: "projection/projected-static-wrapper-dependencies",
        dependency: "tokio",
        supported_versions: "1.53.x with net",
        limitation: "TcpListener::bind is input-selected, but projected static member calls do not yet select its concrete string instantiation",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x",
            feature: "axum-08",
        },
        removal_criterion: "generic projected static-call specialization infers the bind argument and emits direct TcpListener::bind",
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
