pub(crate) struct BundledSource {
    pub namespace: &'static str,
    pub path: &'static str,
    pub text: &'static str,
}

const SOURCES: &[BundledSource] = &[
    BundledSource {
        namespace: "/standard/streams",
        path: "standard/streams.trn",
        text: include_str!("standard/streams.trn"),
    },
    BundledSource {
        namespace: "/standard/paths",
        path: "standard/paths.trn",
        text: include_str!("standard/paths.trn"),
    },
    BundledSource {
        namespace: "/standard/filesystem",
        path: "standard/filesystem.trn",
        text: include_str!("standard/filesystem.trn"),
    },
];

pub(crate) fn source(namespace: &str) -> Option<&'static BundledSource> {
    SOURCES.iter().find(|source| source.namespace == namespace)
}
