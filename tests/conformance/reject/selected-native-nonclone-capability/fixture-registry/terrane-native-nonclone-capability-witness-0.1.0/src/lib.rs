pub struct NonClone;
pub struct Opaque<T>(pub T);

pub fn make_value() -> Opaque<NonClone> {
    Opaque(NonClone)
}

pub fn consume_value(_: Opaque<NonClone>) {}
