//! Accounting for temporary ecosystem integration adapters.
//!
//! Entries in this registry are deliberately data, not hooks in generic projection or lowering.
//! A bridged gap points at an ordinary publishable Rust crate that Terrane projects through the
//! normal dependency path. An unbridged gap records where that path still declines a useful shape.
//! Removing an entry requires satisfying its removal criterion with generic regression coverage.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterStatus {
    Unbridged,
    Package {
        name: &'static str,
        version: &'static str,
    },
}

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

pub const INTEGRATION_ADAPTERS: &[IntegrationAdapter] = &[
    IntegrationAdapter {
        id: "sqlx-sqlite-connection-traits",
        tracking_key: "projection/sqlx-connection-trait-members",
        dependency: "sqlx-sqlite",
        supported_versions: "0.8.x with bundled SQLite",
        limitation: "SqliteConnection projects as its upstream identity, but trait-provided connect and close operations do not yet project as callable members",
        status: AdapterStatus::Package {
            name: "terrane-integration-adapters",
            version: "0.1.x with feature `sqlx-sqlite`",
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
            version: "0.1.x with feature `sqlx-sqlite`",
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
            version: "0.1.x with feature `sqlx-sqlite`",
        },
        removal_criterion: "generic projected trait calls infer bytes row results and string column indices for direct Row::try_get use",
    },
    IntegrationAdapter {
        id: "axum-native-handler-callables",
        tracking_key: "projection/native-callback-wrapper-shape",
        dependency: "axum",
        supported_versions: "0.8.x",
        limitation: "a Terrane-callable projected wrapper is not yet lowered to the ecosystem-native async callable shape required by Handler<T, S>",
        status: AdapterStatus::Unbridged,
        removal_criterion: "generic projected-callback lowering proves and emits the native callable shape while preserving Terrane panic and throwable contracts",
    },
    IntegrationAdapter {
        id: "axum-closed-type-aliases",
        tracking_key: "projection/closed-alias-canonical-identity",
        dependency: "axum and axum-core",
        supported_versions: "0.8.x and 0.5.x",
        limitation: "closed aliases such as Response and equivalent MethodRouter<()> paths do not consistently receive one canonical projected identity",
        status: AdapterStatus::Unbridged,
        removal_criterion: "generic closed-alias projection canonicalizes direct, defaulted, and specialized paths to the same semantic identity",
    },
    IntegrationAdapter {
        id: "axum-serve-associated-inference",
        tracking_key: "projection/associated-bound-result-inference",
        dependency: "axum",
        supported_versions: "0.8.x",
        limitation: "serve result parameters selected through service associated bounds cannot be closed from the router argument and awaited through IntoFuture",
        status: AdapterStatus::Unbridged,
        removal_criterion: "generic associated-bound inference and projected IntoFuture lowering compile a direct serve call without naming its internal service body type",
    },
    IntegrationAdapter {
        id: "axum-websocket-message-shapes",
        tracking_key: "projection/websocket-enum-stream-sink",
        dependency: "axum",
        supported_versions: "0.8.x with ws",
        limitation: "WebSocket receive/send and Message enum payload shapes lack an end-to-end projected source contract",
        status: AdapterStatus::Unbridged,
        removal_criterion: "generic enum, optional/result, and async member projection supports a bounded Terrane-authored echo session",
    },
];

#[must_use]
pub fn adapter_for_package(package: &str) -> Option<&'static IntegrationAdapter> {
    INTEGRATION_ADAPTERS.iter().find(
        |entry| matches!(entry.status, AdapterStatus::Package { name, .. } if name == package),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{AdapterStatus, INTEGRATION_ADAPTERS, adapter_for_package};

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
    fn shipped_adapter_packages_are_accounted_for() {
        let entry = adapter_for_package("terrane-integration-adapters")
            .expect("shipped integration adapter crate must be registered");
        assert!(matches!(entry.status, AdapterStatus::Package { .. }));
        assert_eq!(entry.dependency, "sqlx-sqlite");
    }
}
