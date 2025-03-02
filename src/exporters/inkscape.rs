use crate::exporters::{Exporter, ExporterError};
use crate::formatters::{Formatter, GplFormatter};
use crate::palette::Palette;
use crate::sources::PaletteSource;
use crate::writers::{FileWriter, Writer};
use dirs;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct InkscapeExporter {
    path: PathBuf,
}

impl Exporter for InkscapeExporter {
    fn new(source: &PaletteSource, _options: &HashMap<String, String>) -> Self {
        let config_dir = dirs::config_dir().expect("Could not find config directory");
        let path = config_dir
            .join("inkscape")
            .join("palettes")
            .join(format!("{}.gpl", source.id));
        InkscapeExporter { path }
    }

    fn export_palette(&self, palette: &Palette) -> Result<(), ExporterError> {
        let formatter = GplFormatter;
        let formatted_palette = formatter
            .format_palette(palette)
            .map_err(|e| ExporterError::WriteError(e.to_string()))?;
        let path = self
            .path
            .to_str()
            .ok_or_else(|| ExporterError::WriteError("Unable to obtain path".to_string()))?;
        let writer = FileWriter::new(path);
        writer
            .write(&formatted_palette)
            .map_err(|e| ExporterError::WriteError(e.to_string()))?;
        Ok(())
    }
}
