// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use crate::extensions::is_photo_file;
use crate::search_format;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use walkdir::WalkDir;

pub fn print_info(cmd: clap::Command) {
    let about = cmd.get_about().map(|s| s.to_string()).unwrap_or_default();
    println!(
        "{} {} — {}",
        cmd.get_name(),
        cmd.get_version().unwrap_or(""),
        about
    );
    println!();
}

pub fn scan_dir(root: &Path) -> Result<Vec<PathBuf>> {
    let cancelled = Arc::new(AtomicBool::new(false));

    // Ctrl-C handler
    {
        let cancelled = Arc::clone(&cancelled);
        ctrlc::set_handler(move || {
            cancelled.store(true, Ordering::SeqCst);
        })?;
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {pos} files scanned").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let results: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| {
            // Stop early if Ctrl-C pressed
            if cancelled.load(Ordering::Relaxed) {
                return None;
            }

            let entry = match entry {
                Ok(e) => e,
                Err(_) => {
                    pb.inc(1);
                    return None;
                }
            };

            let path = entry.path();
            pb.inc(1);

            if !path.is_file() {
                return None;
            }

            match is_photo_file(path, &search_format::SearchFormat::All) {
                Ok(true) => {
                    pb.println(path.display().to_string());
                    Some(path.to_path_buf())
                }
                _ => None,
            }
        })
        .collect();

    if cancelled.load(Ordering::Relaxed) {
        pb.finish_with_message("Scan cancelled");
    } else {
        pb.finish_with_message("Scan complete");
    }

    Ok(results)
}
