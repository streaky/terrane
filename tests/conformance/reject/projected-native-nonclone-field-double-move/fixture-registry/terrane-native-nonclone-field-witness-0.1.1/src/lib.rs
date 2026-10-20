pub struct NonClone;
pub struct Payload(pub Vec<NonClone>, pub Vec<NonClone>);

pub fn make_value() -> Payload {
    let values = || (0..42).map(|_| NonClone).collect();
    Payload(values(), values())
}
