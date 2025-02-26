use std::collections::HashMap;
use std::path::PathBuf;
use crate::palette::Palette;
use crate::sources::PaletteSource;
use crate::exporters::Exporter;
use crate::formatters::{GplFormatter, Formatter};
use crate::writers::{FileWriter, Writer};
use dirs;

pub struct InkscapeExporter {
    path: PathBuf,
}

impl Exporter for InkscapeExporter {
    fn new(source: &PaletteSource, _options: &HashMap<String, String>) -> Self {
        let config_dir = dirs::config_dir().expect("Could not find config directory");
        let path = config_dir.join("inkscape").join("palettes").join(format!("{}.gpl", source.id));
        InkscapeExporter { path }
    }

    fn export_palette(&self, palette: &Palette) -> Result<(), Box<dyn std::error::Error>> {
        let formatter = GplFormatter;
        let formatted_palette = formatter.format_palette(palette)?;
        let writer = FileWriter::new(self.path.to_str().unwrap());
        writer.write(&formatted_palette)?;
        Ok(())
    }
}
