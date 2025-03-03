use palette::Srgb;

use crate::formatters::{Formatter, FormatterError};
use crate::palette::Palette;

pub struct GplFormatter;

impl Formatter for GplFormatter {
    fn format_palette(&self, palette: &Palette) -> Result<String, FormatterError> {
        let mut gpl_string = String::new();
        gpl_string.push_str("GIMP Palette\n");
        gpl_string.push_str(&format!("Name: {}\n", palette.name));
        gpl_string.push_str("#\n");

        for swatch in &palette.swatches {
            let (r, g, b) = Srgb::<u8>::from_format(swatch.color).into_components();
            gpl_string.push_str(&format!("{:3} {:3} {:3} {}\n", r, g, b, swatch.name));
        }

        Ok(gpl_string)
    }
}
