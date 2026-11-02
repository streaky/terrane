use std::fmt;
#[derive(Debug)]
pub struct Failure;
impl fmt::Display for Failure { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str("failure") } }
impl std::error::Error for Failure {}
#[derive(Clone, Debug)]
pub struct Input { pub value: i64 }
pub trait Command { type Output; fn execute(self) -> Result<Self::Output, Failure>; }
impl<'loan: 'static> Command for &'loan Input {
    type Output = i64;
    fn execute(self) -> Result<Self::Output, Failure> { Ok(self.value) }
}
pub fn execute<C: Command>(input: C) -> Result<C::Output, Failure> { input.execute() }
