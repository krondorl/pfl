// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use clap::{CommandFactory, Parser};
use pfl::{commands::print_info, commands::process_scan_results, search_format::SearchFormat};

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
    after_help = "Examples:\n  pfl ./photos          List photo files in the 'photos' directory\n  pfl ./photos list.txt   List photo files and save to 'list.txt'\n  pfl --format raw ./photos list.txt   List raw photo files and save to 'list.txt'"
)]
struct Cli {
    /// The directory to list photo files from
    directory: std::path::PathBuf,
    /// Specify the search format
    #[arg(short, long, default_value = "all", value_parser = ["all", "raw", "compressed"])]
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

    println!("Searching photo files in directory: {:?}", args.directory);
    println!();

    let mut print_to_screen = true;

    if args.path.is_some() {
        print_to_screen = false;
    }

    if let Err(e) =
        process_scan_results(&args.directory, &search_format, print_to_screen, args.path)
    {
        eprintln!("Error: {}", e);
    }
}
