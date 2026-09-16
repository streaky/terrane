//! Reusable, explicitly accounted integration bridges for dependency shapes that Terrane's
//! generic projection and lowering cannot yet represent directly.
//!
//! Each adapter lives behind a feature so applications pay only for the dependency families they
//! use. [`registry`] centrally records every adapter limitation and its removal criterion.

#[doc(hidden)]
pub mod registry;

#[cfg(feature = "sqlx-sqlite")]
pub mod sqlx_sqlite;

#[cfg(feature = "axum-08")]
pub mod axum_08;

#[cfg(feature = "docs-rendering")]
pub mod docs_rendering;
