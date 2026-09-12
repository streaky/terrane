#[derive(Clone)]
pub struct SyncForeign(std::cell::Cell<()>);

pub fn sync_foreign() -> SyncForeign {
    SyncForeign(std::cell::Cell::new(()))
}

pub trait Worker: Sync {
    fn value(&self) -> i64;
}
