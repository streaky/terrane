pub struct Descriptor<'a> {
    pub label: &'a str,
    pub note: Option<&'a str>,
    pub values: &'a [String],
    pub extra: Option<&'a [String]>,
    pub offset: u32,
}

pub fn summarize(descriptor: &Descriptor<'_>) -> String {
    format!(
        "{}:{}:{}",
        descriptor.label,
        descriptor.note.unwrap_or("none"),
        descriptor.values.iter().map(String::len).sum::<usize>()
            + descriptor
                .extra
                .unwrap_or_default()
                .iter()
                .map(String::len)
                .sum::<usize>()
            + descriptor.offset as usize,
    )
}

pub fn consume(descriptor: Descriptor<'_>) -> String {
    format!("consumed:{}", descriptor.offset)
}

pub struct Owned {
    pub label: String,
    pub count: u32,
    pub note: Option<String>,
    pub index: Option<i64>,
}

pub fn summarize_owned(value: &Owned) -> String {
    let note = value.note.as_deref().unwrap_or("none");
    match value.index {
        Some(index) => format!("{}:{}:{note}:{index}", value.label, value.count),
        None => format!("{}:{}:{note}:none", value.label, value.count),
    }
}

pub struct NativeSizes {
    pub count: usize,
    pub delta: isize,
    pub optional: Option<usize>,
    pub offsets: Vec<isize>,
}

pub fn summarize_sizes(value: &NativeSizes) -> String {
    format!("{}:{}:{:?}:{:?}", value.count, value.delta, value.optional, value.offsets)
}

pub struct BorrowedSizes<'a> {
    pub label: &'a str,
    pub count: usize,
    pub delta: Option<isize>,
}

pub fn summarize_borrowed_sizes(value: &BorrowedSizes<'_>) -> String {
    format!("{}:{}:{:?}", value.label, value.count, value.delta)
}
