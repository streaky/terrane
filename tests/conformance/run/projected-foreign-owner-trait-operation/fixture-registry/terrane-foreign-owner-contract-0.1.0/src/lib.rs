use terrane_foreign_owner_value::Value;

pub type Response<T = i64> = T;

pub trait Doubled {
    fn doubled(&self) -> Response;
}

impl Doubled for Value {
    fn doubled(&self) -> Response {
        self.amount * 2
    }
}
