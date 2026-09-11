pub trait AssociatedValue {
    type Value;
    fn value(&self) -> Self::Value;
}
