use dirs;
use reqwest::blocking::get;
use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;

#[path = "../gen/config.rs"]
mod config;

#[derive(Deserialize, Debug)]
pub struct PaletteSource {
    pub id: String,
    pub name: String,
    pub url: String,
    pub converter: String,
    pub options: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct RenkSources {
    pub sources: Vec<PaletteSource>,
}

fn download_sources() -> Result<String, Error> {
    eprintln!("Refreshing sources file");
    let url = config::CONFIG_URL.to_string();
    let response = get(url)?.error_for_status()?;
    let response_text = response.text()?;
    Ok(response_text)
}

fn get_sources_path() -> Result<std::path::PathBuf, std::io::Error> {
    let mut sources_path = dirs::cache_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not get cache directory",
        )
    })?;
    sources_path.push("renk");
    sources_path.push("sources.json");
    Ok(sources_path)
}

fn write_sources_file(response_text: &str) -> Result<(), std::io::Error> {
    let sources_path = get_sources_path()?;
    let sources_dir = sources_path.parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not get parent directory",
        )
    })?;
    std::fs::create_dir_all(&sources_dir)?;
    let mut sources_file = std::fs::File::create(sources_path)?;
    sources_file.write_all(response_text.as_bytes())?;
    Ok(())
}

fn load_from_file() -> Result<RenkSources, std::io::Error> {
    let sources_path = get_sources_path()?;
    let sources_file = std::fs::File::open(sources_path)?;
    let sources: RenkSources = serde_json::from_reader(sources_file)?;
    Ok(sources)
}

fn sources_file_up_to_date() -> Result<bool, std::io::Error> {
    let sources_path = get_sources_path()?;
    let modified = sources_path.metadata()?.modified()?;
    let now = std::time::SystemTime::now();
    match now.duration_since(modified) {
        Ok(diff) => Ok(diff.as_secs() <= 3600),
        Err(_) => Ok(false), // Treat the file as out of date if there's a clock error
    }
}

pub fn load_sources(refresh: bool) -> Result<RenkSources, std::io::Error> {
    if !refresh {
        let up_to_date: bool =
            sources_file_up_to_date().or_else(|_| Ok::<bool, std::io::Error>(false))?;
        if up_to_date {
            return load_from_file();
        }
    }
    match download_sources() {
        Ok(response_text) => {
            if let Err(e) = write_sources_file(&response_text) {
                eprintln!("WARN: Could not cache sources file: {}", e);
            }
            let sources = serde_json::from_str::<RenkSources>(&response_text)?;
            return Ok(sources);
        }
        Err(e) => {
            eprintln!("WARN: Could not fetch sources: {}", e);
            return load_from_file();
        }
    }
}
