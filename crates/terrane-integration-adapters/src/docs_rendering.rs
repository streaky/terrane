//! Narrow documentation-rendering operations whose upstream shapes are not yet projectable.

use pdf_oxide::api::Pdf;

/// Render HTML into PDF bytes without exposing `pdf_oxide`'s editor graph.
///
/// # Errors
/// Returns the upstream rendering or serialization error as display text.
pub fn html_to_pdf_bytes(html: String) -> Result<Vec<u8>, String> {
    let mut pdf = Pdf::from_html(&html).map_err(|error| error.to_string())?;
    pdf.save_to_bytes().map_err(|error| error.to_string())
}
