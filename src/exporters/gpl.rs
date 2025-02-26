use std::collections::HashMap;
use crate::palette::Palette;
use crate::exporters::Exporter;
use crate::formatters::{GplFormatter, Formatter};
use crate::writers::{StdoutWriter, Writer};

pub struct GplExporter;

impl Exporter for GplExporter {
    fn new(_options: &HashMap<String, String>) -> Self {
        GplExporter {}
    }

    fn export_palette(&self, palette: &Palette) -> Result<(), Box<dyn std::error::Error>> {
        let formatter = GplFormatter;
        let formatted_palette = formatter.format_palette(palette)?;
        let writer = StdoutWriter;
        writer.write(&formatted_palette)?;
        Ok(())
    }
}
