use crate::exporters::{Exporter, ExporterError};
use crate::formatters::{Formatter, SocFormatter};
use crate::palette::Palette;
use crate::sources::PaletteSource;
use crate::writers::{StdoutWriter, Writer};
use std::collections::HashMap;

pub struct SocExporter;

impl Exporter for SocExporter {
    fn new(_source: &PaletteSource, _options: &HashMap<String, String>) -> Self {
        SocExporter {}
    }

    fn export_palette(&self, palette: &Palette) -> Result<(), ExporterError> {
        let formatter = SocFormatter;
        let formatted_palette = formatter
            .format_palette(palette)
            .map_err(|e| ExporterError::FormattingError(e.to_string()))?;
        let writer = StdoutWriter;
        writer
            .write(&formatted_palette)
            .map_err(|e| ExporterError::WriteError(e.to_string()))?;
        Ok(())
    }
}
