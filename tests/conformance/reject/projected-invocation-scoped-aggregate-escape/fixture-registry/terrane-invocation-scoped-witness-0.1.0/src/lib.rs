pub struct ScopedView<'view> {
    pub marker: std::marker::PhantomData<&'view ()>,
}

#[derive(Clone)]
pub struct BorrowedLabel<'view> {
    value: &'view str,
}

impl<'view> From<BorrowedLabel<'view>> for ScopedView<'view> {
    fn from(label: BorrowedLabel<'view>) -> Self {
        let _ = label.value;
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

pub fn borrowed_label(value: &str) -> BorrowedLabel<'_> {
    BorrowedLabel { value }
}
