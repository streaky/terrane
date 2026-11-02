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

impl<'loan> Command for &'loan Alpha {
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

#[derive(Clone, Debug)]
pub struct Tree {
    pub value: i64,
    pub children: Option<Vec<Tree>>,
}

#[derive(Clone, Debug)]
pub struct TreeRequest {
    pub value: i64,
}

impl Command for TreeRequest {
    type Response = Tree;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok(Tree { value: self.value, children: None })
    }
}

pub fn child_count(tree: &Tree) -> usize {
    tree.children.as_ref().map_or(0, Vec::len)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode { First, Last }

pub fn mode(present: bool) -> Option<Mode> {
    present.then_some(Mode::First)
}

#[derive(Clone, Debug)]
pub struct NumericFields {
    pub quality: Option<u32>,
    pub ratio: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Batch<T> { pub value: T }
impl<T> Command for Batch<Vec<T>> {
    type Response = T;
    fn execute(self) -> Result<Self::Response, Failure> {
        self.value.into_iter().next().ok_or(Failure)
    }
}
pub fn batch() -> Batch<Vec<String>> {
    Batch { value: vec!["nested batch".to_owned()] }
}

#[derive(Clone, Debug)]
pub struct Marked;
impl protocol::Marker for Marked {}
pub fn marked() -> Marked { Marked }

#[derive(Clone, Debug)]
pub struct Payload;
pub trait Reply { type Output; }
impl Reply for Payload { type Output = bool; }
impl Reply for &'static Payload { type Output = AlphaReply; }
#[derive(Clone, Debug)]
pub struct BorrowedOutput;
impl Command for BorrowedOutput {
    type Response = <&'static Payload as Reply>::Output;
    fn execute(self) -> Result<Self::Response, Failure> {
        Ok(AlphaReply { value: 92 })
    }
}
pub fn borrowed_output() -> BorrowedOutput { BorrowedOutput }

impl<T> Command for Batch<std::collections::BTreeSet<T>> {
    type Response = usize;
    fn execute(self) -> Result<Self::Response, Failure> { Ok(self.value.len()) }
}
pub fn set_batch() -> Batch<std::collections::BTreeSet<String>> {
    Batch { value: std::collections::BTreeSet::from(["member".to_owned()]) }
}
impl<T> Command for Batch<std::collections::BTreeMap<String, T>> {
    type Response = T;
    fn execute(self) -> Result<Self::Response, Failure> {
        self.value.into_values().next().ok_or(Failure)
    }
}
pub fn mapped_batch() -> Batch<std::collections::BTreeMap<String, AlphaReply>> {
    Batch { value: std::collections::BTreeMap::from([("member".to_owned(), AlphaReply { value: 94 })]) }
}
