pub(crate) struct BundledSource {
    pub namespace: &'static str,
    pub path: &'static str,
    pub text: &'static str,
}

const SOURCES: &[BundledSource] = &[
    BundledSource {
        namespace: "/core/streams",
        path: "core/streams.trn",
        text: include_str!("core/streams.trn"),
    },
    BundledSource {
        namespace: "/core/filesystem/paths",
        path: "core/paths.trn",
        text: include_str!("core/paths.trn"),
    },
    BundledSource {
        namespace: "/core/filesystem",
        path: "core/filesystem.trn",
        text: include_str!("core/filesystem.trn"),
    },
    BundledSource {
        namespace: "/core/process",
        path: "core/process.trn",
        text: include_str!("core/process.trn"),
    },
    BundledSource {
        namespace: "/core/documents",
        path: "core/documents.trn",
        text: include_str!("core/documents.trn"),
    },
    BundledSource {
        namespace: "/core/documents/json",
        path: "core/json.trn",
        text: include_str!("core/json.trn"),
    },
    BundledSource {
        namespace: "/core/documents/yaml",
        path: "core/yaml.trn",
        text: include_str!("core/yaml.trn"),
    },
    BundledSource {
        namespace: "/core/urls",
        path: "core/urls.trn",
        text: include_str!("core/urls.trn"),
    },
    BundledSource {
        namespace: "/core/random",
        path: "core/random.trn",
        text: include_str!("core/random.trn"),
    },
    BundledSource {
        namespace: "/core/codecs",
        path: "core/codecs.trn",
        text: include_str!("core/codecs.trn"),
    },
    BundledSource {
        namespace: "/core/compression",
        path: "core/compression.trn",
        text: include_str!("core/compression.trn"),
    },
    BundledSource {
        namespace: "/core/random/uuid",
        path: "core/uuid.trn",
        text: include_str!("core/uuid.trn"),
    },
    BundledSource {
        namespace: "/core/networking",
        path: "core/networking.trn",
        text: include_str!("core/networking.trn"),
    },
    BundledSource {
        namespace: "/core/networking/tls",
        path: "core/tls.trn",
        text: include_str!("core/tls.trn"),
    },
    BundledSource {
        namespace: "/core/concurrency",
        path: "core/concurrency.trn",
        text: include_str!("core/concurrency.trn"),
    },
];

pub(crate) fn source(namespace: &str) -> Option<&'static BundledSource> {
    SOURCES.iter().find(|source| source.namespace == namespace)
}
