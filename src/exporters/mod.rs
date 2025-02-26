use crate::palette::Palette;
use std::collections::HashMap;

pub trait Exporter {
    fn new(options: &HashMap<String, String>) -> Self where Self: Sized;
    fn export_palette(&self, palette: &Palette) -> Result<(), Box<dyn std::error::Error>>;
}

mod gpl;

pub fn create_exporter(destination: &str, options: &HashMap<String, String>) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>> {
    let exporter: Box<dyn Exporter> = match destination {
        "gpl" => Box::new(gpl::GplExporter::new(options)),
        //"gimp" => Box::new(crate::gimp::GimpExporter::new(options)),
        //"inkscape" => Box::new(crate::inkscape::InkscapeExporter::new(options)),
        unknown => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown exporter: ".to_string() + unknown))),
    };
    Ok(exporter)
}
