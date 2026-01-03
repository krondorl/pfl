// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use clap::{Parser, ValueEnum};

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
    #[arg(
        short, 
        long, 
        value_enum, 
        default_value_t = SearchFormat::All
    )]
    format: SearchFormat,
    /// Optional path to save the list
    path: Option<std::path::PathBuf>,
}

// Derived ValueEnum allows clap to treat these variants as valid CLI options
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SearchFormat {
    All,
    Raw,
    Compressed,
}

fn main() {
    println!("Hello, world!");
}
