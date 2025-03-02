use crate::converters::{create_converter, ConverterError};
use crate::exporters::{create_exporter, ExporterError};
use crate::sources::PaletteSource;
use palette::Srgb;
use reqwest::blocking::get;
use reqwest::Error;
use std::collections::HashMap;
use std::fmt;

pub struct Swatch {
    pub name: String,
    pub color: Srgb<f32>,
}

pub struct Palette {
    pub name: String,
    pub swatches: Vec<Swatch>,
}

#[derive(Debug)]
pub enum ConvertError {
    DownloadError(reqwest::Error),
    ConversionError(ConverterError),
    ExportError(ExporterError),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::DownloadError(e) => write!(f, "Download error: {}", e),
            ConvertError::ConversionError(e) => write!(f, "Conversion error: {}", e),
            ConvertError::ExportError(e) => write!(f, "Export error: {}", e),
        }
    }
}

impl std::error::Error for ConvertError {}

fn download_palette(url: &str) -> Result<String, Error> {
    let response = get(url)?;
    let response_text = response.text()?;
    Ok(response_text)
}

pub fn convert(source: &PaletteSource, destination: &str) -> Result<(), ConvertError> {
    let response_text = download_palette(&source.url).map_err(ConvertError::DownloadError)?;
    let converter = create_converter(source).map_err(ConvertError::ConversionError)?;
    let swatches = converter
        .extract_palette(&response_text)
        .map_err(ConvertError::ConversionError)?;
    let palette = Palette {
        name: source.name.clone(),
        swatches,
    };

    let options = HashMap::new();
    let exporter =
        create_exporter(destination, source, &options).map_err(ConvertError::ExportError)?;
    exporter
        .export_palette(&palette)
        .map_err(ConvertError::ExportError)?;

    Ok(())
}
