pub trait StaticValue: 'static {
    fn value() -> &'static Self;
}

static VALUE: i64 = 7;

impl StaticValue for i64 {
    fn value() -> &'static Self {
        &VALUE
    }
}

pub fn borrowed_value<T: StaticValue>() -> &'static T {
    T::value()
}
