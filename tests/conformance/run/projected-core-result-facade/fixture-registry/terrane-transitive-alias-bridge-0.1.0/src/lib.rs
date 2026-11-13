pub use public::NewFailure;

pub fn opaque_transitive(ok: bool) -> hidden::Result<i64> {
    if ok { Ok(8) } else { Err(hidden::NewFailure) }
}

pub fn public_transitive(ok: bool) -> public::Result<i64> {
    if ok { Ok(9) } else { Err(NewFailure) }
}

pub struct TransitiveReader;

impl TransitiveReader {
    pub fn opaque_static(ok: bool) -> hidden::Result<i64> {
        opaque_transitive(ok)
    }

    pub fn typed(&self, ok: bool) -> public::Result<i64> {
        public_transitive(ok)
    }

    pub async fn opaque_async(&self, ok: bool) -> hidden::Result<i64> {
        opaque_transitive(ok)
    }
}

pub fn bridge_reader() -> TransitiveReader { TransitiveReader }
