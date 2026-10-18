#[derive(Debug)]
pub struct Error;
impl std::fmt::Display for Error {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        output.write_str("empty text")
    }
}
impl std::error::Error for Error {}

pub mod api {
    pub struct Document {
        text: String,
    }
    impl Document {
        pub fn from_text(text: String) -> Result<Self, crate::Error> {
            if text.is_empty() {
                Err(crate::Error)
            } else {
                Ok(Self { text })
            }
        }
        pub fn save_to_bytes(&mut self) -> Result<Vec<u8>, crate::Error> {
            Ok(self.text.as_bytes().to_vec())
        }
        // Unselected editor operations form the original eager-member dependency cycle.
        pub fn edit(self) -> crate::editor::Editor {
            crate::editor::Editor(self)
        }
    }
}

pub mod editor {
    pub struct Editor(pub(crate) crate::api::Document);
    impl Editor {
        pub fn finish(self) -> crate::api::Document {
            self.0
        }
    }
}
