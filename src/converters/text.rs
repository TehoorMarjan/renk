use std::str::FromStr;
use std::collections::HashMap;
use crate::palette::{Palette, Swatch};
use palette::Srgb;
use crate::converters::Converter;
use regex::Regex;

pub struct TextConverter {
    regex: Regex,
}

impl Converter for TextConverter {
    fn new(options: &HashMap<String, String>) -> Self {
        let regex_str = options.get("regex").expect("Missing regex option");
        let regex = Regex::new(regex_str).expect("Invalid regex pattern");
        TextConverter { regex }
    }

    fn extract_palette(&self, raw_data: &str) -> Result<Palette, Box<dyn std::error::Error>> {
        let mut swatches = Vec::new();

        for cap in self.regex.captures_iter(raw_data) {
            let name = cap.name("name").ok_or("Missing name capture")?.as_str().to_string();
            let value = cap.name("value").ok_or("Missing value capture")?.as_str();
            let color: Srgb<f32> = Srgb::from_str(value)?.into_format();
            swatches.push(Swatch { name, color });
        }

        Ok(Palette { swatches })
    }
}
