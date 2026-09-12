pub trait BorrowingBase {
    fn borrowed(&self) -> &str;
}

pub trait InvalidChild: BorrowingBase {
    fn value(&self) -> i64;
}
