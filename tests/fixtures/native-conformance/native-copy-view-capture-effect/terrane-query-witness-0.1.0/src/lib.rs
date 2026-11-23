pub struct View<'a>(&'a str);
impl View<'_> {
    pub fn size(&self) -> usize {
        self.0.len()
    }
    pub fn into_size(self) -> usize {
        self.0.len()
    }
    pub fn chain(self) -> Self {
        self
    }
    pub fn reborrow(&self) -> View<'_> {
        View(self.0)
    }
}
#[derive(Clone, Copy)]
pub struct CopyView<'a>(&'a str);
impl CopyView<'_> {
    pub fn size(self) -> usize {
        self.0.len()
    }
}
pub fn copy_identity(value: CopyView<'_>) -> CopyView<'_> {
    value
}
pub struct Spec<'a>(&'a str);
impl Clone for Spec<'_> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl Copy for Spec<'static> {}
impl Spec<'_> {
    pub fn size(self) -> usize {
        self.0.len()
    }
}
pub fn query(sql: &str) -> View<'_> {
    View(sql)
}
pub struct Holder {
    label: String,
}
impl Holder {
    pub fn new() -> Self {
        Self {
            label: "hello".to_owned(),
        }
    }
    pub fn view(&self) -> View<'_> {
        View(&self.label)
    }
    pub fn copy_view(&self) -> CopyView<'_> {
        CopyView(&self.label)
    }
    pub fn special_view(&self) -> Spec<'_> {
        Spec(&self.label)
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn label_len(&self) -> usize {
        self.label.len()
    }
    pub fn into_label_len(self) -> usize {
        self.label.len()
    }
    pub fn bump(&mut self) {
        self.label.push('!');
    }
}
#[derive(Clone, Copy)]
pub struct Token;
impl Token {
    pub fn new() -> Self {
        Self
    }
    pub fn value(self) -> u64 {
        7
    }
}
static COPY_PROBE_CLONES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[derive(Copy)]
pub struct CopyProbe(i64);
impl Clone for CopyProbe {
    fn clone(&self) -> Self {
        COPY_PROBE_CLONES.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        *self
    }
}
impl CopyProbe {
    pub fn new(value: i64) -> Self { Self(value) }
    pub fn value(&self) -> i64 { self.0 }
    pub fn reset_clones() { COPY_PROBE_CLONES.store(0, std::sync::atomic::Ordering::SeqCst); }
    pub fn clones() -> usize { COPY_PROBE_CLONES.load(std::sync::atomic::Ordering::SeqCst) }
}

pub use std::net::IpAddr as Address;
pub fn sample_address() -> Address { [127, 0, 0, 1].into() }
pub fn same_address(left: &Address, right: &Address) -> bool { std::ptr::eq(left, right) }
pub enum NE {
    A,
    B,
}
impl NE {
    pub fn code(self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 2,
        }
    }
}
#[derive(Clone, Copy)]
pub enum CE {
    A,
    B,
}
impl CE {
    pub fn code(self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 2,
        }
    }
}
#[derive(Clone)]
pub enum CloneEnum {
    A,
    B,
}
impl CloneEnum {
    pub fn code(self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 2,
        }
    }
}

#[derive(Clone)]
pub struct Cl;
impl Cl {
    pub fn new() -> Self {
        Self
    }
    pub fn value(&self) -> i64 {
        3
    }
    pub fn into_value(self) -> i64 {
        3
    }
}
#[derive(Default)]
pub struct Layer<T>(T);
// Each layer adds several JSON containers; 32 exceed the default decoder limit.
pub type Deep = Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<Layer<u8>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;
trait Depth {
    fn depth(&self) -> i64;
}
impl Depth for u8 {
    fn depth(&self) -> i64 {
        i64::from(*self)
    }
}
impl<T: Depth> Depth for Layer<T> {
    fn depth(&self) -> i64 {
        self.0.depth() + 1
    }
}
pub fn make_deep() -> Deep {
    Deep::default()
}
pub fn read_deep(value: &Deep) -> i64 {
    value.depth()
}

// Like a request extension holding an original URI, a public tuple field
// contains an owner whose method lends text from the owner.
pub struct OriginalTarget(pub RequestTarget);

pub struct RequestTarget(TargetParts);

pub struct TargetParts(String);

static TARGET_CLONES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Clone for RequestTarget {
    fn clone(&self) -> Self {
        TARGET_CLONES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(TargetParts(self.0.text().to_owned()))
    }
}

impl OriginalTarget {
    pub fn new(path: &str) -> Self {
        Self(RequestTarget(TargetParts(path.to_owned())))
    }

    pub fn clones() -> usize {
        TARGET_CLONES.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl RequestTarget {
    pub fn path(&self) -> &str {
        self.0.text()
    }

    pub fn parts(&self) -> Option<&TargetParts> {
        (!self.0.text().is_empty()).then_some(&self.0)
    }
}

impl TargetParts {
    pub fn text(&self) -> &str {
        &self.0
    }

    pub fn checked_text(&self) -> std::io::Result<&str> {
        if self.0.is_ascii() {
            Ok(&self.0)
        } else {
            Err(std::io::Error::other("not ASCII"))
        }
    }
}

pub struct ResponseCode(i64);

impl ResponseCode {
    pub fn new(code: i64) -> Self {
        Self(code)
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        (self.0 == 200).then_some("OK")
    }
}

pub struct ReasonPhrase(String);

impl ReasonPhrase {
    pub fn new(text: &str) -> Self {
        Self(text.to_owned())
    }

    pub fn get(&self) -> Option<&str> {
        (!self.0.is_empty()).then_some(self.0.as_str())
    }
}

pub fn choose_reason(code: i64, reason: &str) -> std::io::Result<Option<&str>> {
    match code {
        200 => Ok(Some(reason)),
        299 => Ok(None),
        _ => Err(std::io::Error::other("not a response code")),
    }
}

pub struct MetadataText(String);

static METADATA_COPIES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Clone for MetadataText {
    fn clone(&self) -> Self {
        METADATA_COPIES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(self.0.clone())
    }
}

impl MetadataText {
    pub fn contents(&self) -> &str {
        &self.0
    }

    pub fn searched_contents(&self) -> &str {
        assert_ne!(self.0, "too late", "search did not short-circuit");
        self.contents()
    }

    pub fn copies() -> usize {
        METADATA_COPIES.load(std::sync::atomic::Ordering::Relaxed)
    }
}

pub trait Describe {
    fn raw_label(&self) -> &MetadataText;
    fn type_name(&self) -> &str;
}

pub struct TypedRecord(MetadataText);

impl TypedRecord {
    pub fn new(label: &str) -> Self {
        Self(MetadataText(label.to_owned()))
    }

    pub fn find(&self, label: &str) -> Option<&MetadataText> {
        if label == "panic" {
            std::panic::resume_unwind(Box::new("native lookup failed"));
        }
        (self.0.contents() == label).then_some(&self.0)
    }

    pub fn metadata(&self, owned: bool) -> std::borrow::Cow<'_, MetadataText> {
        if owned {
            std::borrow::Cow::Owned(MetadataText("transient label".to_owned()))
        } else {
            std::borrow::Cow::Borrowed(&self.0)
        }
    }
}

impl Describe for TypedRecord {
    fn raw_label(&self) -> &MetadataText {
        &self.0
    }

    fn type_name(&self) -> &str {
        self.0.contents()
    }
}

#[derive(Clone)]
pub struct MetadataTable<T = MetadataText>(T);

impl MetadataTable {
    pub fn snapshot_text(&self) -> &str {
        self.0.contents()
    }
}

pub struct MetadataRequest(MetadataTable);

impl MetadataRequest {
    pub fn new(label: &str) -> Self {
        Self(MetadataTable(MetadataText(label.to_owned())))
    }

    pub fn metadata(&self) -> &MetadataTable {
        &self.0
    }
}

pub struct Octets(Vec<u8>);

impl Octets {
    pub fn new() -> Self {
        Self(vec![0xf1, 0, b'a'])
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn select(&self, index: usize) -> std::io::Result<Option<&[u8]>> {
        if index > 3 {
            Err(std::io::Error::other("invalid offset"))
        } else {
            Ok(self.0.get(index..).filter(|tail| !tail.is_empty()))
        }
    }
}

pub struct Records([MetadataText; 3]);

impl Records {
    pub fn new(with_match: bool) -> Self {
        let labels = if with_match {
            ["", "first", "too late"]
        } else {
            ["", "", ""]
        };
        Self(labels.map(|label| MetadataText(label.to_owned())))
    }

    pub fn values(&self) -> RecordValues<'_, MetadataText> {
        RecordValues(&self.0)
    }
}

pub struct RecordValues<'a, T>(&'a [T]);

impl<T> RecordValues<'_, T> {
    pub fn iter(&self) -> RecordIter<'_, T> {
        RecordIter(self.0.iter())
    }
}

pub struct RecordIter<'a, T>(std::slice::Iter<'a, T>);

impl<'a, T> Iterator for RecordIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
