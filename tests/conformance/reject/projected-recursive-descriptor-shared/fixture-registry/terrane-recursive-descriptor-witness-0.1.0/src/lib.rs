use std::{collections::HashMap, ffi::OsStr, path::PathBuf, time::Duration};

pub struct Descriptor<'a> {
    pub window_size: Option<(u32, u32)>,
    pub nested: Option<(Option<u32>, Option<u32>)>,
    pub path: PathBuf,
    pub timeout: Duration,
    pub labels: Option<HashMap<String, String>>,
    pub args: Vec<&'a OsStr>,
    pub names: Vec<&'a str>,
    pub note: Option<&'a str>,
    pub strings: &'a [String],
}
fn summary(value: &Descriptor<'_>) -> String {
    format!("{}:{}:{}:{}:{}:{}:{}:{}:{}", value.window_size.map(|(w, h)| format!("{w}x{h}")).unwrap_or_else(|| "none".into()), value.nested.map(|(a,b)| format!("{},{}", a.unwrap_or(0), b.unwrap_or(0))).unwrap_or_else(|| "outer-none".into()), value.path.display(), value.timeout.as_millis(), value.labels.as_ref().map_or(0, HashMap::len), value.args.iter().map(|x| x.to_string_lossy()).collect::<Vec<_>>().join(","), value.names.join(","), value.note.unwrap_or("none"), value.strings.join(","))
}
pub fn path_value() -> PathBuf { PathBuf::from("/tmp/example") }
pub fn timeout_value() -> Duration { Duration::from_millis(25) }
pub fn inspect(value: &Descriptor<'_>) -> String { summary(value) }
pub fn inspect_shared(value: &Descriptor<'_>) -> String { summary(value) }
pub fn consume(value: Descriptor<'_>) -> String { summary(&value) }
pub fn fail_early(_: &Descriptor<'_>) -> Result<String, std::io::Error> { Err(std::io::Error::other("early")) }
pub fn mutate(value: &mut Descriptor<'_>) { value.window_size = Some((1, 2)); }
pub fn borrow_name<'a>(value: &'a Descriptor<'_>) -> &'a str { value.names[0] }
impl Descriptor<'_> {
    pub fn inspect_method(&self) -> String { summary(self) }
    pub fn consume_method(self) -> String { summary(&self) }
}
pub struct Factory;
impl Factory {
    pub fn inspect_static(value: &Descriptor<'_>) -> String { summary(value) }
    pub fn consume_static(value: Descriptor<'_>) -> String { summary(&value) }
}

static TICKET_DROPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub struct Ticket;
impl Drop for Ticket {
    fn drop(&mut self) { TICKET_DROPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
}
pub struct UniqueDescriptor<'a> { pub ticket: Ticket, pub text: &'a str }
pub fn ticket() -> Ticket { Ticket }
pub fn take_unique(value: UniqueDescriptor<'_>) -> String { value.text.to_owned() }
pub fn inspect_unique(value: &UniqueDescriptor<'_>) -> String { value.text.to_owned() }
pub fn ticket_drops() -> usize { TICKET_DROPS.load(std::sync::atomic::Ordering::SeqCst) }
pub fn fail_unique(_value: UniqueDescriptor<'_>) -> Result<(), std::io::Error> { Err(std::io::Error::other("unique failure")) }
