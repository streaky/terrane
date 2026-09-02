// Rust justification: ABI boundary to bounded document parser sizes.

pub fn terrane_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(0)
}
