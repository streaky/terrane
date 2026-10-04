use crate::semantics::{FunctionContract, SemanticUnit};

/// Returns whether semantic definite-initialization analysis proved these fields for this constructor.
pub(super) fn constructor_proves_fields(
    unit: &SemanticUnit,
    constructor: &FunctionContract,
    fields: &std::collections::BTreeSet<&str>,
) -> bool {
    let Some(proof) = unit.required_init_proofs.get(&(
        constructor.span.file,
        constructor.span.start,
        constructor.span.end,
    )) else {
        return false;
    };
    fields
        .iter()
        .all(|field| proof.initialized_fields.contains(*field))
}
