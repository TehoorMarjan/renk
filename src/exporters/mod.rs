use crate::palette::Palette;
use crate::sources::PaletteSource;
use std::collections::HashMap;

pub trait Exporter {
    fn new(source: &PaletteSource, options: &HashMap<String, String>) -> Self where Self: Sized;
    fn export_palette(&self, palette: &Palette) -> Result<(), Box<dyn std::error::Error>>;
}

mod gpl;
mod inkscape;

pub fn create_exporter(destination: &str, source: &PaletteSource, options: &HashMap<String, String>) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>> {
    let exporter: Box<dyn Exporter> = match destination {
        "gpl" => Box::new(gpl::GplExporter::new(source, options)),
        "inkscape" => Box::new(inkscape::InkscapeExporter::new(source, options)),
        unknown => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown exporter: ".to_string() + unknown))),
    };
    Ok(exporter)
}
