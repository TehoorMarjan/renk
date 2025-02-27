use crate::formatters::Formatter;
use crate::palette::Palette;
use palette::Srgb;
use quick_xml::se::to_string;
use serde::Serialize;

#[derive(Serialize)]
struct SocColor {
    #[serde(rename = "draw:name")]
    name: String,
    #[serde(rename = "draw:color")]
    color: String,
}

#[derive(Serialize)]
#[serde(rename = "office:color-table")]
struct SocColors {
    #[serde(rename = "xmlns:office")]
    office: String,
    #[serde(rename = "xmlns:style")]
    style: String,
    #[serde(rename = "xmlns:text")]
    text: String,
    #[serde(rename = "xmlns:table")]
    table: String,
    #[serde(rename = "xmlns:draw")]
    draw: String,
    #[serde(rename = "xmlns:fo")]
    fo: String,
    #[serde(rename = "xmlns:xlink")]
    xlink: String,
    #[serde(rename = "xmlns:dc")]
    dc: String,
    #[serde(rename = "xmlns:meta")]
    meta: String,
    #[serde(rename = "xmlns:number")]
    number: String,
    #[serde(rename = "xmlns:svg")]
    svg: String,
    #[serde(rename = "xmlns:chart")]
    chart: String,
    #[serde(rename = "xmlns:dr3d")]
    dr3d: String,
    #[serde(rename = "xmlns:math")]
    math: String,
    #[serde(rename = "xmlns:form")]
    form: String,
    #[serde(rename = "xmlns:script")]
    script: String,
    #[serde(rename = "draw:color")]
    colors: Vec<SocColor>,
}

pub struct SocFormatter;

impl Formatter for SocFormatter {
    fn format_palette(&self, palette: &Palette) -> Result<String, Box<dyn std::error::Error>> {
        let mut colors = Vec::new();

        for swatch in &palette.swatches {
            let (r, g, b) = Srgb::<u8>::from_format(swatch.color).into_components();
            let color = format!("#{:02x}{:02x}{:02x}", r, g, b);
            colors.push(SocColor {
                name: swatch.name.clone(),
                color,
            });
        }

        let soc_colors = SocColors { 
            office: String::from("http://openoffice.org/2000/office"),
            style: String::from("http://openoffice.org/2000/style"),
            text: String::from("http://openoffice.org/2000/text"),
            table: String::from("http://openoffice.org/2000/table"),
            draw: String::from("http://openoffice.org/2000/drawing"),
            fo: String::from("http://www.w3.org/1999/XSL/Format"),
            xlink: String::from("http://www.w3.org/1999/xlink"),
            dc: String::from("http://purl.org/dc/elements/1.1/"),
            meta: String::from("http://openoffice.org/2000/meta"),
            number: String::from("http://openoffice.org/2000/datastyle"),
            svg: String::from("http://www.w3.org/2000/svg"),
            chart: String::from("http://openoffice.org/2000/chart"),
            dr3d: String::from("http://openoffice.org/2000/dr3d"),
            math: String::from("http://www.w3.org/1998/Math/MathML"),
            form: String::from("http://openoffice.org/2000/form"),
            script: String::from("http://openoffice.org/2000/script"),
            colors };
        let xml_string = to_string(&soc_colors)?;
        let xml_declaration = "<?xml version='1.0' encoding='UTF-8'?>\n";
        let full_xml_string = format!("{}{}", xml_declaration, xml_string);

        Ok(full_xml_string)
    }
}
