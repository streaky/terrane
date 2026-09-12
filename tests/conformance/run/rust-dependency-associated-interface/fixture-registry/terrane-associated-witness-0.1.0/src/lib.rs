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

#[derive(Clone)]
pub struct Nested;

impl Nested {
    pub fn value(&self) -> i64 {
        0
    }
}

pub fn nested_total<T>(value: T) -> i64
where
    T: Sequence<Item = Nested>,
{
    value.first().map(|item| item.value()).unwrap_or_default()
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

pub fn list_total<T>(value: T) -> i64
where
    T: Sequence<Item = Vec<i64>>,
{
    value.first().map(|item| item.len() as i64).unwrap_or_default() + value.length()
}
