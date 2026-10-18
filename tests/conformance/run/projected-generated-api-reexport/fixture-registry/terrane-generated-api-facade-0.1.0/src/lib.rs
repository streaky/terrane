// Exercise both external module and glob reexports, not direct local definitions.
pub use terrane_generated_api_owner::generated::builtin;

pub mod classes {
    pub use terrane_generated_api_owner::generated::classes::*;
}
