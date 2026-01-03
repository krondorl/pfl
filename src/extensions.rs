// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use anyhow::{Context, Result};
use std::path::Path;

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
    // RAW
    "cr2", "cr3", "arw", "nef", "nrw", "raf", "rw2", // Compressed
    "jpg", "jpeg", "heif", "heic",
];

pub fn is_photo_file(path: &Path) -> Result<bool> {
    let ext = match path.extension() {
        Some(ext) => ext.to_str().context("File extension is not valid UTF-8")?,
        None => return Ok(false),
    };

    // ASCII-only case-insensitive compare without allocation
    Ok(ALL_PHOTO_EXTENSIONS
        .iter()
        .any(|e| ext.eq_ignore_ascii_case(e)))
}
