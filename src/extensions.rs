// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use anyhow::{Context, Result};
use std::path::Path;

use crate::search_format::SearchFormat;

/// All supported RAW photo extensions (lowercase, no dots)
pub const RAW_EXTENSIONS: &[&str] = &[
    "cr2", "cr3", // Canon
    "arw", // Sony
    "nef", "nrw", // Nikon
    "raf", // Fujifilm
    "rw2", // Panasonic
];

/// All supported compressed photo extensions (lowercase, no dots)
pub const COMPRESSED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "heif", "heic"];

/// Combined list (useful for simple scanning)
pub const ALL_PHOTO_EXTENSIONS: &[&str] = &[
    "cr2", "cr3", "arw", "nef", "nrw", "raf", "rw2", // RAW
    "jpg", "jpeg", "heif", "heic", // Compressed
];

pub fn is_photo_file(path: &Path, search_format: &SearchFormat) -> Result<bool> {
    let ext = match path.extension() {
        Some(ext) => ext.to_str().context("File extension is not valid UTF-8")?,
        None => return Ok(false),
    };

    match search_format {
        SearchFormat::Raw => Ok(RAW_EXTENSIONS.iter().any(|e| ext.eq_ignore_ascii_case(e))),
        SearchFormat::Compressed => Ok(COMPRESSED_EXTENSIONS
            .iter()
            .any(|e| ext.eq_ignore_ascii_case(e))),
        SearchFormat::All => Ok(ALL_PHOTO_EXTENSIONS
            .iter()
            .any(|e| ext.eq_ignore_ascii_case(e))),
    }
}
