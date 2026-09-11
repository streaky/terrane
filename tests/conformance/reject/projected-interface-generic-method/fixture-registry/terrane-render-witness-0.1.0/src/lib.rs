pub trait GenericMethod {
    fn map<T>(&self, value: T) -> T;
}
