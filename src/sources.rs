use reqwest::blocking::get;
use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PaletteSource {
    pub name: String,
    pub url: String,
    pub converter: String,
    pub options: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct RenkSources {
    pub sources: Vec<PaletteSource>,
}

pub fn fetch_sources(url: &str) -> Result<RenkSources, Error> {
    let response = get(url)?;
    let sources = response.json::<RenkSources>()?;
    Ok(sources)
}
