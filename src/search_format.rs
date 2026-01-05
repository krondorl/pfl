use clap::ValueEnum;

// Derived ValueEnum allows clap to treat these variants as valid CLI options
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SearchFormat {
    All,
    Raw,
    Compressed,
}
