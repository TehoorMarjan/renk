use crate::exporters::Exporter;
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

    fn export_palette(&self, palette: &Palette) -> Result<(), Box<dyn std::error::Error>> {
        let formatter = SocFormatter;
        let formatted_palette = formatter.format_palette(palette)?;
        let writer = StdoutWriter;
        writer.write(&formatted_palette)?;
        Ok(())
    }
}
