mod sources;
#[path = "../gen/config.rs"]
mod config;

use crate::sources::{fetch_sources, PaletteSource};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: renk <command> [args]");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "list" => list_sources(),
        "get" => {
            if args.len() < 4 {
                eprintln!("Usage: renk get <destination> <source>");
                return;
            }
            let destination = &args[2];
            let source = &args[3];
            get_palette(destination, source);
        }
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn list_sources() {
    let config_url = config::CONFIG_URL.to_string();
    match fetch_sources(&config_url) {
        Ok(sources) => {
            for source in sources.sources {
                println!("Name: {}, URL: {}, Converter: {}", source.name, source.url, source.converter);
            }
        }
        Err(e) => eprintln!("Error fetching sources: {}", e),
    }
}

fn get_palette(destination: &str, source: &str) {
    // Implement the logic to get the palette and convert it to the destination format
    println!("Getting palette from {} to {}", source, destination);
}
