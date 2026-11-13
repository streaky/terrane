pub use std::option::Option as Maybe;
pub use std::result::Result as Outcome;
pub use std::char::from_u32 as character;
pub use std::str::from_utf8 as text;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub use self::Either as Result;

pub fn pick(left: bool) -> Result<i64, String> {
    if left {
        Either::Left(1)
    } else {
        Either::Right("right".to_owned())
    }
}

pub fn describe(value: &Either<i64, String>) -> String {
    match value {
        Either::Left(value) => format!("left {value}"),
        Either::Right(value) => format!("right {value}"),
    }
}

pub fn parse(input: &str) -> Outcome<i64, std::io::Error> {
    input.parse().map_err(std::io::Error::other)
}

pub fn maybe(present: bool) -> Maybe<i64> {
    present.then_some(41)
}

