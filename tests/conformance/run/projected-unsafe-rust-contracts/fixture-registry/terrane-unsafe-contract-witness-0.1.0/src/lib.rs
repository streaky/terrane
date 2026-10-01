use std::fmt;

pub unsafe fn unchecked_add(left: i64, right: i64) -> i64 {
    left + right
}


pub struct ScopedView<'view> {
    value: i64,
    marker: std::marker::PhantomData<&'view str>,
}

impl fmt::Display for ScopedView<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

pub fn borrowed_view(value: &str) -> ScopedView<'_> {
    ScopedView {
        value: value.len() as i64,
        marker: std::marker::PhantomData,
    }
}

pub unsafe trait UnsafeView<'view> {
    fn view(&self, state: &'view String) -> ScopedView<'view>;
}

unsafe impl<'view, F> UnsafeView<'view> for F
where
    F: Fn(&'view String) -> ScopedView<'view>,
{
    fn view(&self, state: &'view String) -> ScopedView<'view> {
        self(state)
    }
}

pub fn render_unsafe(
    state: &String,
    view: impl for<'view> UnsafeView<'view>,
) -> i64 {
    view.view(state).value
}
