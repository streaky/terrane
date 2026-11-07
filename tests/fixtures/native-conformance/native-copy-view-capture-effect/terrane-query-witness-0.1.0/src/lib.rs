pub struct View<'a>(&'a str);
impl View<'_> {
    pub fn size(&self) -> usize {
        self.0.len()
    }
    pub fn into_size(self) -> usize {
        self.0.len()
    }
    pub fn chain(self) -> Self {
        self
    }
}
#[derive(Clone, Copy)]
pub struct CopyView<'a>(&'a str);
impl CopyView<'_> {
    pub fn size(self) -> usize {
        self.0.len()
    }
}
pub fn copy_identity(value: CopyView<'_>) -> CopyView<'_> {
    value
}
pub struct Spec<'a>(&'a str);
impl Clone for Spec<'_> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl Copy for Spec<'static> {}
impl Spec<'_> {
    pub fn size(self) -> usize {
        self.0.len()
    }
}
pub fn query(sql: &str) -> View<'_> {
    View(sql)
}
pub struct Holder {
    label: String,
}
impl Holder {
    pub fn new() -> Self {
        Self {
            label: "hello".to_owned(),
        }
    }
    pub fn view(&self) -> View<'_> {
        View(&self.label)
    }
    pub fn copy_view(&self) -> CopyView<'_> {
        CopyView(&self.label)
    }
    pub fn special_view(&self) -> Spec<'_> {
        Spec(&self.label)
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn label_len(&self) -> usize {
        self.label.len()
    }
    pub fn into_label_len(self) -> usize {
        self.label.len()
    }
    pub fn bump(&mut self) {
        self.label.push('!');
    }
}
#[derive(Clone, Copy)]
pub struct Token;
impl Token {
    pub fn new() -> Self {
        Self
    }
    pub fn value(self) -> u64 {
        7
    }
}
pub enum NE {
    A,
    B,
}
impl NE {
    pub fn code(self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 2,
        }
    }
}
#[derive(Clone, Copy)]
pub enum CE {
    A,
    B,
}
impl CE {
    pub fn code(self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 2,
        }
    }
}
#[derive(Clone)]
pub enum CloneEnum {
    A,
    B,
}
impl CloneEnum {
    pub fn code(self) -> u64 {
        match self {
            Self::A => 1,
            Self::B => 2,
        }
    }
}

#[derive(Clone)]
pub struct Cl;
impl Cl {
    pub fn new() -> Self {
        Self
    }
    pub fn value(&self) -> i64 {
        3
    }
    pub fn into_value(self) -> i64 {
        3
    }
}
