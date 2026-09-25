pub struct PlainOwner {
    pub label: String,
}

impl PlainOwner {
    pub fn standard() -> Self {
        Self {
            label: "plain".to_owned(),
        }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }
}

pub struct DefaultOwner<T = String> {
    pub label: T,
}

impl DefaultOwner<String> {
    pub const CODE: u32 = 40 + 2;

    pub fn standard() -> Self {
        Self {
            label: "default".to_owned(),
        }
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }
}
