pub trait UnprojectableMember {
    fn raw(&self) -> *const u8;
}
