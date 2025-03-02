use crate::formatters::{Formatter, FormatterError};
use crate::palette::Palette;
use palette::Srgb;
use quick_xml::se::to_string;
use serde::Serialize;

#[derive(Serialize)]
struct ScribusColor {
    #[serde(rename = "NAME")]
    name: String,
    #[serde(rename = "RGB")]
    rgb: String,
}

#[derive(Serialize)]
#[serde(rename = "SCRIBUSCOLORS")]
struct ScribusColors {
    #[serde(rename = "COLOR")]
    colors: Vec<ScribusColor>,
}

pub struct ScribusFormatter;

impl Formatter for ScribusFormatter {
    fn format_palette(&self, palette: &Palette) -> Result<String, FormatterError> {
        let mut colors = Vec::new();

        for swatch in &palette.swatches {
            let (r, g, b) = Srgb::<u8>::from_format(swatch.color).into_components();
            let rgb = format!("#{:02x}{:02x}{:02x}", r, g, b);
            colors.push(ScribusColor {
                name: swatch.name.clone(),
                rgb,
            });
        }

        let scribus_colors = ScribusColors { colors };
        let xml_string =
            to_string(&scribus_colors).map_err(|e| FormatterError::FormatError(e.to_string()))?;

        Ok(xml_string)
    }
}
