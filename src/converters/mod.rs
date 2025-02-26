use std::collections::HashMap;
use crate::{palette::Swatch, sources::PaletteSource};

pub trait Converter {
    fn new(options: &HashMap<String, String>) -> Self where Self: Sized;
    fn extract_palette(&self, raw_data: &str) -> Result<Vec<Swatch>, Box<dyn std::error::Error>>;
}

mod text;
mod tailwind;
pub use text::TextConverter;
pub use tailwind::TailwindConverter;

pub fn create_converter(source: &PaletteSource) -> Result<Box<dyn Converter>, Box<dyn std::error::Error>> {
    let converter: Box<dyn Converter> = match source.converter.as_str() {
        "text" => Box::new(TextConverter::new(&source.options)),
        "tailwind" => Box::new(TailwindConverter::new(&source.options)),
        unknown  => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown converter: ".to_string() + unknown))),
    };
    Ok(converter)
}