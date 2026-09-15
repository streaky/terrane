//! Reusable, explicitly accounted integration bridges for dependency shapes that Terrane's
//! generic projection and lowering cannot yet represent directly.
//!
//! Each adapter lives behind a feature so applications pay only for the dependency families they
//! use. Adapter limitations and removal criteria are recorded in
//! `terrane_compiler::integration_adapters`.

#[cfg(feature = "sqlx-sqlite")]
pub mod sqlx_sqlite;
