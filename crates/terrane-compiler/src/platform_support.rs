//! Publication of the compiler-owned platform and signal support crates.

use std::fs;
use std::io;
use std::path::Path;

/// Write the platform support crate and, when requested, its sibling signal support crate.
///
/// # Errors
///
/// Returns filesystem errors encountered while creating directories or publishing native sources.
pub fn write_platform_support(directory: &Path, uses_process_signals: bool) -> io::Result<()> {
    let platform = directory.join("terrane-platform-support");
    fs::create_dir_all(platform.join("src"))?;
    write_if_changed(
        &platform.join("Cargo.toml"),
        crate::platform_support_manifest(uses_process_signals).as_bytes(),
    )?;
    for (name, bytes) in [
        (
            "lib.rs",
            include_bytes!("../../terrane-platform-support/src/lib.rs").as_slice(),
        ),
        (
            "child_process.rs",
            include_bytes!("../../terrane-platform-support/src/child_process.rs").as_slice(),
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
            "temporary.rs",
            include_bytes!("../../terrane-platform-support/src/temporary.rs").as_slice(),
        ),
    ] {
        write_if_changed(&platform.join("src").join(name), bytes)?;
    }
    if uses_process_signals {
        write_if_changed(
            &platform.join("src/signals.rs"),
            include_bytes!("../../terrane-platform-support/src/signals.rs"),
        )?;
        let signal = directory.join("terrane-signal-support");
        fs::create_dir_all(signal.join("src"))?;
        write_if_changed(
            &signal.join("Cargo.toml"),
            crate::signal_support_manifest().as_bytes(),
        )?;
        write_if_changed(
            &signal.join("src/lib.rs"),
            include_bytes!("../../terrane-signal-support/src/lib.rs"),
        )?;
    } else {
        match fs::remove_file(platform.join("src/signals.rs")) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn write_if_changed(path: &Path, content: &[u8]) -> io::Result<()> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content)
}
