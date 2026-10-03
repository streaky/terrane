#[macro_export]
macro_rules! message { ($value:literal) => { String::from($value) }; }
