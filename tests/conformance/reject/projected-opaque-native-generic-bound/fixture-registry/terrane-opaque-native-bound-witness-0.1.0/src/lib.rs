pub struct CopyOnly<T: Copy> {
    value: T,
}

impl<T: Copy> CopyOnly<T> {
    pub fn is_present(&self) -> bool {
        let _ = &self.value;
        true
    }
}
