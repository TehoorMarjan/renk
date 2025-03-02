use crate::palette::Palette;
use crate::sources::PaletteSource;
use std::collections::HashMap;
use std::fmt;

pub trait Exporter {
    fn new(source: &PaletteSource, options: &HashMap<String, String>) -> Self
    where
        Self: Sized;
    fn export_palette(&self, palette: &Palette) -> Result<(), ExporterError>;
}

#[derive(Debug)]
pub enum ExporterError {
    InvalidInput(String),
    WriteError(String),
    FormattingError(String),
}

impl fmt::Display for ExporterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExporterError::InvalidInput(e) => write!(f, "Invalid input: {}", e),
            ExporterError::WriteError(e) => write!(f, "Write error: {}", e),
            ExporterError::FormattingError(e) => write!(f, "Formatting error: {}", e),
        }
    }
}

impl std::error::Error for ExporterError {}

mod gpl;
mod inkscape;
mod scribus;
mod scribusstd;
mod soc;

pub fn create_exporter(
    destination: &str,
    source: &PaletteSource,
    options: &HashMap<String, String>,
) -> Result<Box<dyn Exporter>, ExporterError> {
    let exporter: Box<dyn Exporter> = match destination {
        "gpl" => Box::new(gpl::GplExporter::new(source, options)),
        "inkscape" => Box::new(inkscape::InkscapeExporter::new(source, options)),
        "scribus" => Box::new(scribus::ScribusExporter::new(source, options)),
        "scribusstd" => Box::new(scribusstd::ScribusStdExporter::new(source, options)),
        "soc" => Box::new(soc::SocExporter::new(source, options)),
        unknown => {
            return Err(ExporterError::InvalidInput(
                "Unknown exporter: ".to_string() + unknown,
            ))
        }
    };
    Ok(exporter)
}

pub fn list_exporters() -> Vec<&'static str> {
    vec!["gpl", "inkscape", "scribus", "scribusstd", "soc"]
}
