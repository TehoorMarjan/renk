use crate::palette::Palette;
use crate::formatters::Formatter;

pub struct GplFormatter;

impl Formatter for GplFormatter {
    fn format_palette(&self, palette: &Palette) -> Result<String, Box<dyn std::error::Error>> {
        // Implement the logic to format the palette as a GPL string
        Ok(String::new())
    }
}
