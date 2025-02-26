use crate::palette::Palette;

pub trait Formatter {
    fn format_palette(&self, palette: &Palette) -> Result<String, Box<dyn std::error::Error>>;
}

mod gpl;
pub use gpl::GplFormatter;
