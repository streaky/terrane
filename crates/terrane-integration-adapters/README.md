# Terrane Integration Adapters

This crate provides temporary, feature-gated bridges between Terrane applications and
external crates whose APIs Terrane's generic projection and lowering cannot yet express
directly. These adapters complement the projected upstream API with only the operations
needed to cross each known integration gap.

Keeping the adapters in one crate makes their dependencies explicit, allows applications
to enable only the integrations they use, and prevents package-specific workarounds from
leaking into the compiler. The central registry records the limitation behind each adapter,
the dependency versions it supports, and the condition under which the adapter can be
removed.

Adapters are therefore intentionally temporary. As Terrane gains the corresponding generic
compiler support, each workaround and its registry entry should be deleted in favour of the
directly projected dependency API.
