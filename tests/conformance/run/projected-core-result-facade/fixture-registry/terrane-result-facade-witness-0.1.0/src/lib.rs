pub use std::option::Option as Maybe;
pub use std::result::Result as Outcome;
pub use std::char::from_u32 as character;
pub use std::str::from_utf8 as text;
pub use terrane_outcome_alias_witness::Failure;

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

pub fn external(ok: bool) -> terrane_outcome_alias_witness::Result<i64> {
    if ok { Ok(2) } else { Err(Failure) }
}

pub fn renamed(ok: bool) -> terrane_outcome_alias_witness::Verdict<i64> {
    external(ok)
}

pub fn external_maybe(present: bool) -> terrane_outcome_alias_witness::Maybe<i64> {
    present.then_some(3)
}

pub fn nested(present: bool, ok: bool) -> Option<terrane_outcome_alias_witness::Result<i64>> {
    present.then(|| external(ok))
}

pub struct Reader;

impl Reader {
    pub fn open(ok: bool) -> terrane_outcome_alias_witness::Result<i64> {
        external(ok)
    }

    pub fn read(&self, ok: bool) -> terrane_outcome_alias_witness::Verdict<i64> {
        external(ok)
    }

    pub async fn asynchronous(&self, ok: bool) -> terrane_outcome_alias_witness::Result<i64> {
        external(ok)
    }
}

pub fn reader() -> Reader { Reader }

pub fn opaque(ok: bool) -> terrane_outcome_alias_witness::OpaqueResult<i64> {
    if ok { Ok(4) } else { Err(terrane_outcome_alias_witness::OpaqueFailure) }
}

pub fn duration() -> std::time::Duration {
    std::time::Duration::from_secs(4)
}

impl terrane_outcome_alias_witness::Index<Reader> for &str {
    fn index(&self, _row: &Reader) -> terrane_outcome_alias_witness::Result<i64> {
        Ok(self.len() as i64)
    }
}

