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
            + descriptor.extra.unwrap_or_default().iter().map(String::len).sum::<usize>()
            + descriptor.offset as usize,
    )
}

pub fn consume(descriptor: Descriptor<'_>) -> String {
    format!("consumed:{}", descriptor.offset)
}

pub struct Owned {
    pub label: String,
    pub count: u32,
}

pub fn summarize_owned(value: &Owned) -> String {
    format!("{}:{}", value.label, value.count)
}
