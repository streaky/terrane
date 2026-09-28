pub trait Value {
    type Output;

    fn value(&self) -> Self::Output;
}

struct Hidden(u32);
impl Value for Hidden {
    type Output = u32;

    fn value(&self) -> Self::Output {
        self.0
    }
}

pub struct Wrapper<T: Value> {
    inner: T,
}

impl<T: Value<Output = u32>> Wrapper<T> {
    pub fn value(&self) -> u32 {
        self.inner.value()
    }
}

pub fn make(value: u32) -> Wrapper<impl Value<Output = u32>> {
    Wrapper {
        inner: Hidden(value),
    }
}

pub fn consume<T: Value<Output = u32>>(wrapper: Wrapper<T>) -> u32 {
    wrapper.value()
}

pub trait Service<T> {
    fn serve(self) -> i64;
}

pub struct SelectedLayer<F, T> {
    function: F,
    marker: std::marker::PhantomData<T>,
}

pub fn choose<F, T>(function: F) -> SelectedLayer<F, T> {
    SelectedLayer {
        function,
        marker: std::marker::PhantomData,
    }
}

impl<F, T> Service<T> for SelectedLayer<F, T>
where
    F: Fn(T) -> i64,
    T: Default,
{
    fn serve(self) -> i64 {
        (self.function)(T::default())
    }
}

pub fn serve<L, T>(layer: L) -> i64
where
    L: Service<T> + 'static,
{
    layer.serve()
}

