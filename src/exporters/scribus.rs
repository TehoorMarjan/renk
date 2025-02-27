use crate::exporters::Exporter;
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

    fn export_palette(&self, palette: &Palette) -> Result<(), Box<dyn std::error::Error>> {
        let formatter = ScribusFormatter;
        let formatted_palette = formatter.format_palette(palette)?;
        let writer = FileWriter::new(self.output_path.to_str().unwrap());
        writer.write(&formatted_palette)?;
        Ok(())
    }
}
