pub mod ops {
    pub trait Transform: 'static {
        fn transform<I, O>(&self, value: I) -> O
        where
            I: Into<i64>,
            O: From<i64>,
        {
            O::from(value.into())
        }
    }
}
