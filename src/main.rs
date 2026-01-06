// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use clap::{CommandFactory, Parser};
use pfl::{commands::print_info, search_format::SearchFormat};

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " — ",
    env!("CARGO_PKG_DESCRIPTION"),
    "\nPhoto file lister"
);

/// List photo files in a directory
#[derive(Parser)]
#[command(version = VERSION_INFO)]
#[command(
    long_about = "This tool lists photo files in a specified directory.\nIt supports raw files and compressed files.\nRaw files include formats from Canon, Sony, Nikon, Fujifilm and Panasonic.",
    after_help = "Examples:\n  pfl ./photos          List photo files in the 'photos' directory\n  pfl ./photos list.txt   List photo files and save to 'list.txt'"
)]
struct Cli {
    /// The directory to list photo files from
    directory: std::path::PathBuf,
    /// Specify the search format
    #[arg(short, long, default_value = "all")]
    format: String,
    /// Optional path to save the list
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::parse();
    let cmd = Cli::command();

    print_info(cmd);

    let mut search_format = SearchFormat::All;

    match args.format.as_str() {
        "all" => println!("Searching all files..."),
        "raw" => {
            println!("Searching raw files...");
            search_format = SearchFormat::Raw;
        }
        "compressed" => {
            println!("Searching compressed files...");
            search_format = SearchFormat::Compressed;
        }
        _ => println!("Unknown format, defaulting to all files..."),
    }

    println!("Listing photo files in directory: {:?}", args.directory);
    println!();

    let _photo_files = match pfl::commands::scan_dir(&args.directory, &search_format) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
            return;
        }
    };
}
