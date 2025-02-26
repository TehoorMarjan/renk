use std::collections::HashMap;
use crate::palette::Palette;
use crate::sources::PaletteSource;

pub trait Converter {
    fn new(options: &HashMap<String, String>) -> Self where Self: Sized;
    fn extract_palette(&self, raw_data: &str) -> Result<Palette, Box<dyn std::error::Error>>;
}

mod text;
pub use text::TextConverter;

pub fn create_converter(source: &PaletteSource) -> Result<Box<dyn Converter>, Box<dyn std::error::Error>> {
    let converter: Box<dyn Converter> = match source.converter.as_str() {
        "text" => Box::new(TextConverter::new(&source.options)),
        unknown  => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown converter: ".to_string() + unknown))),
    };
    Ok(converter)
}