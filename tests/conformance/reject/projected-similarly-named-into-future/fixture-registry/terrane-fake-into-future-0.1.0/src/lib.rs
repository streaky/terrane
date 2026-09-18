pub trait IntoFuture {
    type Output;

    fn into_future(self) -> Self::Output;
}

pub struct Immediate(u32);

impl IntoFuture for Immediate {
    type Output = u32;

    fn into_future(self) -> Self::Output {
        self.0
    }
}

pub fn immediate(value: u32) -> Immediate {
    Immediate(value)
}
