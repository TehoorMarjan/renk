use crate::exporters::{Exporter, ExporterError};
use crate::formatters::{Formatter, ScribusFormatter};
use crate::palette::Palette;
use crate::sources::PaletteSource;
use crate::writers::{FileWriter, Writer};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ScribusExporter {
    output_path: PathBuf,
}

impl Exporter for ScribusExporter {
    fn new(source: &PaletteSource, _options: &HashMap<String, String>) -> Self {
        let output_path = PathBuf::from(format!("{}.xml", source.id));
        ScribusExporter { output_path }
    }

    fn export_palette(&self, palette: &Palette) -> Result<(), ExporterError> {
        let formatter = ScribusFormatter;
        let formatted_palette = formatter
            .format_palette(palette)
            .map_err(|e| ExporterError::FormattingError(e.to_string()))?;
        let path = self
            .output_path
            .to_str()
            .ok_or_else(|| ExporterError::WriteError("Unable to obtain path".to_string()))?;
        let writer = FileWriter::new(path);
        writer
            .write(&formatted_palette)
            .map_err(|e| ExporterError::WriteError(e.to_string()))?;
        Ok(())
    }
}
