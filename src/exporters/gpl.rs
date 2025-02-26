use crate::exporters::Exporter;
use crate::formatters::{Formatter, GplFormatter};
use crate::palette::Palette;
use crate::sources::PaletteSource;
use crate::writers::{StdoutWriter, Writer};
use std::collections::HashMap;

pub struct GplExporter;

impl Exporter for GplExporter {
    fn new(_source: &PaletteSource, _options: &HashMap<String, String>) -> Self {
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
