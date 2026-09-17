pub mod ops {
    pub trait Transform: 'static {
        fn transform(&self, value: i64) -> i64;
    }
}
