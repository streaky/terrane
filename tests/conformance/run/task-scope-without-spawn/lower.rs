// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: task-scope-without-spawn
fn main() {
    let parent_deadline: Deadline;
    let parent: TerraneTaskScope;
    let child_deadline: Deadline;
    let child: TerraneTaskScope;
    let grandchild_deadline: Deadline;
    parent_deadline = Clock::terrane_static_deadline(
        __terrane_traced(
            Duration::terrane_static_milliseconds(
                terrane_int_support::Int::from(10_i128),
            ),
            0 /* terrane-site: case.trn:6:39-6:65 */,
        ),
    );
    parent = TerraneTaskScope::new(
        Some(parent_deadline.expires_at.elapsed_nanoseconds.clone()),
    );
    child_deadline = Clock::terrane_static_deadline(
        __terrane_traced(
            Duration::terrane_static_milliseconds(
                terrane_int_support::Int::from(5_i128),
            ),
            1 /* terrane-site: case.trn:8:38-8:63 */,
        ),
    );
    child = parent.child_scope(&child_deadline.expires_at.elapsed_nanoseconds);
    grandchild_deadline = Clock::terrane_static_deadline(
        __terrane_traced(
            Duration::terrane_static_milliseconds(
                terrane_int_support::Int::from(2_i128),
            ),
            2 /* terrane-site: case.trn:10:43-10:68 */,
        ),
    );
    child.child_scope(&grandchild_deadline.expires_at.elapsed_nanoseconds);
}
