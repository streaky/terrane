pub struct NonClone;
pub struct Payload(pub Vec<NonClone>);

pub fn make_value() -> Payload {
    Payload((0..42).map(|_| NonClone).collect())
}
