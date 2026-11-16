pub struct Panicker;

impl Panicker {
    pub fn new() -> Self {
        Self
    }

    pub fn fail(&self) {
        panic!("custom library instance panic");
    }

    pub async fn fail_async(&self) {
        panic!("custom library async panic");
    }
}

pub fn fail_free() {
    panic!("custom library free panic");
}
