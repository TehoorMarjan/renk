use crate::converters::{Converter, ConverterError};
use crate::palette::Swatch;
use palette::{FromColor, Oklch, Srgb};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub struct TailwindConverter {
    regex: Regex,
}

impl Converter for TailwindConverter {
    fn new(_options: &HashMap<String, String>) -> Self {
        let regex_str = r"--color-(?P<name>.*?):\s+((?P<value>#[[:xdigit:]]+)|(oklch\((?P<value_l>\d*\.?\d+)\s+(?P<value_c>\d*\.?\d+)\s+(?P<value_h>\d*\.?\d+)\)));";
        let regex = Regex::new(regex_str).expect("Invalid regex pattern");
        TailwindConverter { regex }
    }

    fn extract_palette(&self, raw_data: &str) -> Result<Vec<Swatch>, ConverterError> {
        let mut swatches = Vec::new();

        for cap in self.regex.captures_iter(raw_data) {
            let name = cap
                .name("name")
                .ok_or_else(|| ConverterError::MissingCapture("name".to_string()))?
                .as_str()
                .to_string();
            if let Some(value) = cap.name("value") {
                let color: Srgb<f32> = Srgb::from_str(value.as_str())
                    .map_err(|e| ConverterError::ParseError(e.to_string()))?
                    .into_format();
                swatches.push(Swatch { name, color });
            } else {
                let value_l = cap
                    .name("value_l")
                    .ok_or_else(|| ConverterError::MissingCapture("value_l".to_string()))?
                    .as_str()
                    .parse::<f32>()
                    .map_err(|e| ConverterError::ParseError(e.to_string()))?;
                let value_c = cap
                    .name("value_c")
                    .ok_or_else(|| ConverterError::MissingCapture("value_c".to_string()))?
                    .as_str()
                    .parse::<f32>()
                    .map_err(|e| ConverterError::ParseError(e.to_string()))?;
                let value_h = cap
                    .name("value_h")
                    .ok_or_else(|| ConverterError::MissingCapture("value_h".to_string()))?
                    .as_str()
                    .parse::<f32>()
                    .map_err(|e| ConverterError::ParseError(e.to_string()))?;
                let oklch_color = Oklch::new(value_l, value_c, value_h);
                let color: Srgb<f32> = Srgb::from_color(oklch_color);
                swatches.push(Swatch { name, color });
            }
        }

        Ok(swatches)
    }
}
