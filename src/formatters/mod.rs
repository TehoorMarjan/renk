use crate::palette::Palette;

pub trait Formatter {
    fn format_palette(&self, palette: &Palette) -> Result<String, Box<dyn std::error::Error>>;
}

mod gpl;
mod scribus;
mod soc;
pub use gpl::GplFormatter;
pub use scribus::ScribusFormatter;
pub use soc::SocFormatter;
