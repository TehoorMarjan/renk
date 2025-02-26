use crate::sources::PaletteSource;
use crate::converters::create_converter;
use crate::exporters::create_exporter;
use palette::Srgb;
use std::collections::HashMap;

use reqwest::blocking::get;
use reqwest::Error;

pub struct Swatch {
    pub name: String,
    pub color: Srgb<f32>,
}

pub struct Palette {
    pub swatches: Vec<Swatch>,
}

fn download_palette(url: &str) -> Result<String, Error> {
    let response = get(url)?;
    let response_text = response.text()?;
    Ok(response_text)
}

pub fn convert(source: &PaletteSource, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response_text = download_palette(&source.url)?;
    let converter = create_converter(source)?;
    let palette = converter.extract_palette(&response_text)?;

    // Implement the logic to save the palette to the destination format
    let options = HashMap::new();
    let exporter = create_exporter(destination, &options)?;
    exporter.export_palette(&palette)?;

    Ok(())
}
