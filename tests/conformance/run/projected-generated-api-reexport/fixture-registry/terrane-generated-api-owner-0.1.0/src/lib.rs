// The public facade must recover these hidden definitions from a transitive owner.
#[doc(hidden)]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}
