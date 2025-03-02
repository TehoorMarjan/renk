use crate::{palette::Swatch, sources::PaletteSource};
use std::collections::HashMap;
use std::fmt;

pub trait Converter {
    fn new(options: &HashMap<String, String>) -> Self
    where
        Self: Sized;
    fn extract_palette(&self, raw_data: &str) -> Result<Vec<Swatch>, ConverterError>;
}

#[derive(Debug)]
pub enum ConverterError {
    InvalidRegex(String),
    MissingCapture(String),
    ParseError(String),
}

impl fmt::Display for ConverterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConverterError::InvalidRegex(e) => write!(f, "Invalid regex: {}", e),
            ConverterError::MissingCapture(e) => write!(f, "Missing capture: {}", e),
            ConverterError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for ConverterError {}

mod tailwind;
mod text;
pub use tailwind::TailwindConverter;
pub use text::TextConverter;

pub fn create_converter(source: &PaletteSource) -> Result<Box<dyn Converter>, ConverterError> {
    let converter: Box<dyn Converter> = match source.converter.as_str() {
        "text" => Box::new(TextConverter::new(&source.options)),
        "tailwind" => Box::new(TailwindConverter::new(&source.options)),
        unknown => {
            return Err(ConverterError::InvalidRegex(
                "Unknown converter: ".to_string() + unknown,
            ))
        }
    };
    Ok(converter)
}
