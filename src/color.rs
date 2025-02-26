use std::str::FromStr;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err("Invalid hex format");
        }
        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid hex value")?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid hex value")?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid hex value")?;
        Ok(Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        })
    }
}

impl Color {
    pub fn to_str(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }
}
