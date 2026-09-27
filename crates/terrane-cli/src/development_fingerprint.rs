use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::UNIX_EPOCH;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const PATHS: &[&str] = &[
    ":(glob)crates/**/*.rs",
    ":(glob)**/Cargo.toml",
    "Cargo.lock",
];

pub fn dirty_fingerprint(root: &Path) -> Option<u64> {
    let mut status_args = vec!["diff", "--name-status", "-z", "--no-ext-diff", "HEAD", "--"];
    status_args.extend(PATHS);
    let status = git_output(root, &status_args)?;

    let mut changed_args = vec!["diff", "--name-only", "-z", "--no-ext-diff", "HEAD", "--"];
    changed_args.extend(PATHS);
    let changed = git_output(root, &changed_args)?;
    let mut untracked_args = vec!["ls-files", "-z", "--others", "--exclude-standard", "--"];
    untracked_args.extend(PATHS);
    let untracked = git_output(root, &untracked_args)?;

    let mut paths = changed
        .split(|byte| *byte == 0)
        .chain(untracked.split(|byte| *byte == 0))
        .filter(|path| !path.is_empty())
        .collect::<Vec<_>>();
    paths.sort_unstable();
    paths.dedup();

    let mut hash = FNV_OFFSET;
    hash_bytes(&mut hash, &status);
    for path in paths {
        hash_bytes(&mut hash, path);
        hash_bytes(&mut hash, &[0]);
        if let Ok(metadata) = fs::metadata(root.join(String::from_utf8_lossy(path).as_ref())) {
            let modified = metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok()?;
            hash_bytes(&mut hash, &modified.as_nanos().to_le_bytes());
        }
        hash_bytes(&mut hash, &[0]);
    }
    Some(hash)
}

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

pub fn git_output(root: &Path, arguments: &[&str]) -> Option<Vec<u8>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
        .ok()?;
    output.status.success().then_some(output.stdout)
}
