#[derive(Debug)]
pub struct HiddenError;

pub fn fail() -> Result<(), HiddenError> {
    Err(HiddenError)
}
