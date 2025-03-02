use crate::palette::Palette;
use std::fmt;

pub trait Formatter {
    fn format_palette(&self, palette: &Palette) -> Result<String, FormatterError>;
}

#[derive(Debug)]
pub enum FormatterError {
    FormatError(String),
}

impl fmt::Display for FormatterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatterError::FormatError(e) => write!(f, "Format error: {}", e),
        }
    }
}

impl std::error::Error for FormatterError {}

mod gpl;
mod scribus;
mod soc;
pub use gpl::GplFormatter;
pub use scribus::ScribusFormatter;
pub use soc::SocFormatter;
