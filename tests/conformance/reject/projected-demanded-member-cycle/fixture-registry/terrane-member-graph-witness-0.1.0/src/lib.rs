pub mod left {
    #[derive(Clone)]
    pub struct Left;

    impl Left {
        pub fn new() -> Self {
            Self
        }

        pub fn value(&self) -> u32 {
            41
        }

        pub fn poison(&self) -> crate::right::Right {
            crate::right::Right
        }
    }
}

pub mod right {
    #[derive(Clone)]
    pub struct Right;

    impl Right {
        pub fn poison(&self) -> crate::left::Left {
            crate::left::Left
        }
    }
}
