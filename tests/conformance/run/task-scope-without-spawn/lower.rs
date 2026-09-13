// Generated deterministically by Terrane <version>.
// Runtime support: tasks_threaded.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-string-support
// Source: case.trn
// Namespace: task-scope-without-spawn
fn main() {
    let parent: TerraneTaskScope = TerraneTaskScope::new(Some(10 as u64));
    let child: TerraneTaskScope = parent.child_scope(5 as u64);
    child.child_scope(2 as u64);
}
