// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use clap::Parser;

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
    long_about = "This tool lists photo files in a specified directory. It supports raw files and compressed files.",
    after_help = "Examples:\n  pfl ./photos          List photo files in the 'photos' directory\n  pfl ./photos list.txt   List photo files and save to 'list.txt'"
)]
struct Cli {
    /// The directory to list photo files from
    directory: std::path::PathBuf,
    /// Optional path to save the list
    path: Option<std::path::PathBuf>,
}

fn main() {
    println!("Hello, world!");
}
