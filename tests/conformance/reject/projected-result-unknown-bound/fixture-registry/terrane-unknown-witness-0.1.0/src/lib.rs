// The library satisfies the bound for i64, but an external probe cannot spell
// the private module path. This is indeterminate accessibility, not E0271 No.
mod private {
    pub struct Hidden;
}

pub trait Value {
    type Item;
}

impl Value for i64 {
    type Item = private::Hidden;
}

pub fn value<T: Value<Item = private::Hidden> + Default>() -> T {
    T::default()
}
