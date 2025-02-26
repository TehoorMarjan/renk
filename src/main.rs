mod sources;
mod palette;
mod color;
mod converters;

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
    match sources::load_sources() {
        Ok(sources) => {
            for source in sources.sources {
                println!("{}", source.name);
            }
        }
        Err(e) => eprintln!("Error fetching sources: {}", e),
    }
}

fn get_palette(destination: &str, source: &str) {
    match sources::load_sources() {
        Err(e) => eprintln!("Error fetching sources: {}", e),
        Ok(sources) => {
            for source_description in sources.sources {
                if source_description.name == source {
                    if let Err(e) = palette::convert(&source_description, &destination) {
                        eprintln!("Error converting swatch: {}", e);
                    }
                    return;
                }
            }
            // If we reach this point, the source was not found
            eprintln!("Invalid source: {}", source);
        }
    }
}