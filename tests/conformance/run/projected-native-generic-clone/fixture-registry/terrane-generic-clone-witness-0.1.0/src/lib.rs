#[derive(Clone)]
pub struct CloneBox<T> {
    pub value: T,
}

pub fn make<T>(value: T) -> CloneBox<T> {
    CloneBox { value }
}

impl<T: Copy> CloneBox<T> {
    pub fn read<U>(&self, _input: U) -> Read<'_, T> {
        Read { source: self }
    }
}

pub struct Read<'a, T> {
    source: &'a CloneBox<T>,
}

impl<T: Copy> Future for Read<'_, T> {
    type Output = Result<T, std::io::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(Ok(self.source.value))
    }
}
