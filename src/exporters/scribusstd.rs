use crate::exporters::{Exporter, ExporterError};
use crate::formatters::{Formatter, ScribusFormatter};
use crate::palette::Palette;
use crate::sources::PaletteSource;
use crate::writers::{StdoutWriter, Writer};
use std::collections::HashMap;

pub struct ScribusStdExporter;

impl Exporter for ScribusStdExporter {
    fn new(_source: &PaletteSource, _options: &HashMap<String, String>) -> Self {
        ScribusStdExporter {}
    }

    fn export_palette(&self, palette: &Palette) -> Result<(), ExporterError> {
        let formatter = ScribusFormatter;
        let formatted_palette = formatter
            .format_palette(palette)
            .map_err(|e| ExporterError::WriteError(e.to_string()))?;
        let writer = StdoutWriter;
        writer
            .write(&formatted_palette)
            .map_err(|e| ExporterError::WriteError(e.to_string()))?;
        Ok(())
    }
}
