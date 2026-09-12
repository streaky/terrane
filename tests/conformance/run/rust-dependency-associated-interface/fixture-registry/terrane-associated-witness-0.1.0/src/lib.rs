pub trait Sequence: Send + Sync {
    type Item: Clone + Send + Sync + 'static;

    fn first(&self) -> Option<Self::Item>;
    fn length(&self) -> i64;
}

pub trait NamedSequence: Sequence {
    fn label(&self) -> String;
}

pub fn sequence_total<T>(value: T) -> i64
where
    T: Sequence<Item = i64>,
{
    value.first().unwrap_or_default() + value.length()
}

pub fn named_total<T>(value: T) -> i64
where
    T: NamedSequence<Item = i64>,
{
    value.first().unwrap_or_default() + value.length() + value.label().len() as i64
}

pub fn erased_total(value: Box<dyn Sequence<Item = i64>>) -> i64 {
    value.first().unwrap_or_default() + value.length()
}
