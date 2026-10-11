// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/process-exit
fn main() {
    let status: ExitStatus;
    status = make_exit_status(terrane_int_support::Int::from(7_i128));
    exit(status);
}
