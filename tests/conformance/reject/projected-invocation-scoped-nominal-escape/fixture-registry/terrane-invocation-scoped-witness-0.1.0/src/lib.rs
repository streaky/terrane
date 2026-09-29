pub struct ScopedView<'view> {
    pub marker: std::marker::PhantomData<&'view ()>,
}

pub struct BorrowedLabel<'view> {
    value: &'view str,
}

impl<'view> Into<ScopedView<'view>> for BorrowedLabel<'view> {
    fn into(self) -> ScopedView<'view> {
        let _ = self.value;
        ScopedView {
            marker: std::marker::PhantomData,
        }
    }
}

pub fn borrowed_label(value: &str) -> BorrowedLabel<'_> {
    BorrowedLabel { value }
}
