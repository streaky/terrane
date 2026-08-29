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
    BundledSource {
        namespace: "/standard/process",
        path: "standard/process.trn",
        text: include_str!("standard/process.trn"),
    },
    BundledSource {
        namespace: "/standard/documents",
        path: "standard/documents.trn",
        text: include_str!("standard/documents.trn"),
    },
    BundledSource {
        namespace: "/standard/json",
        path: "standard/json.trn",
        text: include_str!("standard/json.trn"),
    },
    BundledSource {
        namespace: "/standard/yaml",
        path: "standard/yaml.trn",
        text: include_str!("standard/yaml.trn"),
    },
    BundledSource {
        namespace: "/standard/urls",
        path: "standard/urls.trn",
        text: include_str!("standard/urls.trn"),
    },
    BundledSource {
        namespace: "/standard/random",
        path: "standard/random.trn",
        text: include_str!("standard/random.trn"),
    },
    BundledSource {
        namespace: "/standard/codecs",
        path: "standard/codecs.trn",
        text: include_str!("standard/codecs.trn"),
    },
    BundledSource {
        namespace: "/standard/compression",
        path: "standard/compression.trn",
        text: include_str!("standard/compression.trn"),
    },
    BundledSource {
        namespace: "/standard/uuid",
        path: "standard/uuid.trn",
        text: include_str!("standard/uuid.trn"),
    },
    BundledSource {
        namespace: "/standard/networking",
        path: "standard/networking.trn",
        text: include_str!("standard/networking.trn"),
    },
    BundledSource {
        namespace: "/standard/tls",
        path: "standard/tls.trn",
        text: include_str!("standard/tls.trn"),
    },
    BundledSource {
        namespace: "/standard/concurrency",
        path: "standard/concurrency.trn",
        text: include_str!("standard/concurrency.trn"),
    },
];

pub(crate) fn source(namespace: &str) -> Option<&'static BundledSource> {
    SOURCES.iter().find(|source| source.namespace == namespace)
}
