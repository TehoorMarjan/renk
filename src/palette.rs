use crate::sources::PaletteSource;
use crate::color::Color;
use crate::converters::{Converter, create_converter};

use reqwest::blocking::get;
use reqwest::Error;

pub struct Swatch {
    pub name: String,
    pub color: Color,
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

    // Print all swatches contained in the returned palette
    for swatch in palette.swatches {
        println!("{}: {}", swatch.name, swatch.color.to_str());
    }

    // Implement the logic to save the palette to the destination format
    Ok(())
}
