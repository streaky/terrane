use std::sync::Arc;
pub use protocol::{Failure, Request as Command, SharedReply as AliasedReply};

#[derive(Clone, Debug)]
pub struct Alpha { pub value: i64 }
#[derive(Clone, Debug)]
pub struct AlphaReply { pub value: i64 }
impl Command for Alpha {
    type Response = AlphaReply;
    fn execute(self) -> Result<Self::Response, Failure> {
        if self.value < 0 { return Err(Failure); }
        Ok(AlphaReply { value: self.value + 1 })
    }
}

#[derive(Clone, Debug)]
pub struct Beta { pub text: String }
impl Command for Beta {
    type Response = AliasedReply;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok(AliasedReply { label: self.text })
    }
}

#[derive(Clone, Debug)]
pub struct Maybe { pub value: i64 }
impl Command for Maybe {
    type Response = Option<Vec<AlphaReply>>;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok((self.value >= 0).then(|| vec![AlphaReply { value: self.value }]))
    }
}

#[derive(Clone, Debug)]
pub struct Echo<T> { pub value: T }
#[derive(Clone, Debug)]
pub struct EchoReply<T> { pub value: T }
impl<T> Command for Echo<T> {
    type Response = EchoReply<T>;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok(EchoReply { value: self.value })
    }
}

pub trait Alternate {
    type Response;
    fn alternate(self) -> Self::Response;
}
impl Alternate for Alpha {
    type Response = AliasedReply;
    fn alternate(self) -> Self::Response {
        AliasedReply { label: format!("alternate:{}", self.value) }
    }
}

#[derive(Clone, Debug)]
pub struct Executor;
impl Executor {
    pub fn new() -> Self { Self }
    pub fn call<C: Command>(&self, command: C) -> Result<C::Response, Failure> {
        command.execute()
    }
    pub fn send<C: Command>(command: C) -> Result<C::Response, Failure> {
        command.execute()
    }
}
pub type SharedExecutor = Arc<Executor>;
pub fn shared_executor() -> SharedExecutor { Arc::new(Executor) }
pub fn execute<C: Command>(command: C) -> Result<C::Response, Failure> { command.execute() }
pub fn alternate<C: Alternate>(command: C) -> C::Response { command.alternate() }

#[derive(Debug)]
pub struct Unique { pub text: String }
#[derive(Debug)]
pub struct UniqueReply { pub text: String }
impl Command for Unique {
    type Response = UniqueReply;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok(UniqueReply { text: self.text })
    }
}

impl Command for &Alpha {
    type Response = AliasedReply;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok(AliasedReply { label: format!("borrowed:{}", self.value) })
    }
}

#[derive(Clone, Debug)]
pub struct Tagged { pub value: i64 }
#[derive(Clone, Debug)]
pub struct TaggedReply { pub r#type: i64 }
impl Command for Tagged {
    type Response = TaggedReply;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok(TaggedReply { r#type: self.value })
    }
}
