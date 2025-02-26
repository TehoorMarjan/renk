mod converters;
mod palette;
mod sources;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "renk")]
#[command(about = "A tool for grabing color palettes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    refresh: bool,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[command(subcommand)]
        list_command: ListCommands,
    },
    Get {
        destination: String,
        source: String,
    },
}

#[derive(Subcommand)]
enum ListCommands {
    #[clap(alias = "src")]
    Sources,
    #[clap(aliases = &["dest", "dst"])]
    Destinations,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { list_command } => match list_command {
            ListCommands::Sources => list_sources(cli.refresh),
            ListCommands::Destinations => list_destinations(),
        },
        Commands::Get {
            destination,
            source,
        } => get_palette(destination, source, cli.refresh),
    }
}

fn list_sources(refresh: bool) {
    match sources::load_sources(refresh) {
        Ok(sources) => {
            for source in sources.sources {
                println!("{}", source.name);
            }
        }
        Err(e) => eprintln!("Error fetching sources: {}", e),
    }
}

fn list_destinations() {
    // Implement the logic to list available destinations
    println!("Available destinations:");
    // Example destinations
    println!("inkscape");
    println!("gimp");
}

fn get_palette(destination: &str, source: &str, refresh: bool) {
    match sources::load_sources(refresh) {
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
