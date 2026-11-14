use terrane_error_owner_witness::ExternalError;

pub fn fail() -> Result<(), ExternalError> {
    Err(ExternalError)
}
