//! Publication of the compiler-owned platform and signal support crates.

use std::fs;
use std::io;
use std::path::Path;

/// Write the platform and sibling signal support crates into a generated program's support root.
///
/// `directory` is the directory containing `terrane-platform-support/` and
/// `terrane-signal-support/`.
///
/// # Errors
///
/// Returns filesystem errors encountered while creating directories or publishing native sources.
pub fn write_platform_support(directory: &Path) -> io::Result<()> {
    let platform = directory.join("terrane-platform-support");
    let signal = directory.join("terrane-signal-support");
    fs::create_dir_all(platform.join("src"))?;
    fs::create_dir_all(signal.join("src"))?;
    write_if_changed(
        &platform.join("Cargo.toml"),
        crate::platform_support_manifest().as_bytes(),
    )?;
    for (name, bytes) in [
        (
            "lib.rs",
            include_bytes!("../../terrane-platform-support/src/lib.rs").as_slice(),
        ),
        (
            "observability.rs",
            include_bytes!("../../terrane-platform-support/src/observability.rs").as_slice(),
        ),
        (
            "networking_buffers.rs",
            include_bytes!("../../terrane-platform-support/src/networking_buffers.rs").as_slice(),
        ),
        (
            "networking_native.rs",
            include_bytes!("../../terrane-platform-support/src/networking_native.rs").as_slice(),
        ),
        (
            "networking_state.rs",
            include_bytes!("../../terrane-platform-support/src/networking_state.rs").as_slice(),
        ),
        (
            "networking_transport.rs",
            include_bytes!("../../terrane-platform-support/src/networking_transport.rs").as_slice(),
        ),
        (
            "signals.rs",
            include_bytes!("../../terrane-platform-support/src/signals.rs").as_slice(),
        ),
    ] {
        write_if_changed(&platform.join("src").join(name), bytes)?;
    }
    write_if_changed(
        &signal.join("Cargo.toml"),
        crate::signal_support_manifest().as_bytes(),
    )?;
    write_if_changed(
        &signal.join("src/lib.rs"),
        include_bytes!("../../terrane-signal-support/src/lib.rs"),
    )?;
    Ok(())
}
fn write_if_changed(path: &Path, content: &[u8]) -> io::Result<()> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content)
}
