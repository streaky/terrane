//! Rust interoperability subsystem.
//!
//! This module is the discoverable owner of compiler behavior that derives, proves, specializes,
//! and lowers Rust-facing contracts. General Terrane parsing, value semantics, control flow,
//! ownership, diagnostics, and generic Rust emission remain in their language-wide modules.
//!
//! # Phase map
//!
//! 1. [`projection`] consumes pinned Cargo/Rustdoc input and produces canonical projected metadata.
//! 2. Terrane semantic analysis consumes that metadata and records validated projected-call plans.
//! 3. `terrane-rust-analysis` proves native obligations requested by projection and semantics.
//! 4. The lowering dependency planner in `lowering.rs` consumes semantic plans and emits statically
//!    typed Rust wrappers and calls; it does not rediscover projection choices. Its source lives in
//!    this subsystem but is mounted as a private child of the lowering module so it can share
//!    emitter internals without exposing them across compiler phases.
//! 5. [`census`] compares the projected surface with native discovery as diagnostic evidence.
//!
//! Shared Rustdoc indexing, exact probes, and Cargo process policy come directly from
//! `terrane-rust-analysis`; no compiler compatibility facade duplicates them. Generic Rust
//! emission remains under `lowering/`; only projected dependency/wrapper planning is owned here.
//! # Value lifetimes and terminal paths
//!
//! | Value/terminal path | Status | Evidence |
//! | --- | --- | --- |
//! | Ordinary owned projected values | Implemented | `projected-owned-opaque-result` |
//! | Expression-scoped native chains | Implemented | Axum/HTTP package conformance |
//! | Owned callback result converted by a higher-ranked native blanket recipe | Implemented | `projected-invocation-scoped-callback` |
//! | Exact invocation-scoped lifetime-bearing callback result | Implemented | `projected-invocation-scoped-callback`, `projected-invocation-scoped-nominal-escape` |
//! | Invocation-scoped native operation graphs | Planned (work unit 4C) | package-independent graph fixture |
//! | Real projected Iced view cutover | Planned (work units 4D–4E) | `packages/iced-ui` |
//!
//! Projection metadata is canonical. Semantic analysis owns Terrane-region, ownership, and
//! control-flow validity. Lowering owns Rust spelling and emission only. These ownership boundaries
//! deliberately prevent a second callback-recipe, generic-substitution, obligation, or scoped-value
//! model from appearing in another compiler phase.
pub mod census;
pub mod projection;

pub(crate) use terrane_rust_analysis::configure_projection_cargo_command;
pub use terrane_rust_analysis::{
    BoundQuestion, CallProbeEvidence, CallProbeReport, CallQuestion, ImplProbeEvidence,
    ImplProbeReport, ImplQuestion, ProbeAnswer, ProbeEvidence, ProbeReport, ProjectionOracle,
    configure_cargo_command,
};
