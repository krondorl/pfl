// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

use pfl::{
    extensions::{ALL_PHOTO_EXTENSIONS, COMPRESSED_EXTENSIONS, RAW_EXTENSIONS, is_photo_file},
    search_format,
};
use std::path::Path;

#[test]
fn test_raw_extensions_recognized() {
    let search_format = search_format::SearchFormat::Raw;
    // Test that all RAW format extensions are correctly identified
    let raw_files = vec![
        "photo.CR2",
        "photo.cr3",
        "image.ARW",
        "shot.nef",
        "pic.NRW",
        "camera.raf",
        "snap.RW2",
    ];

    for file in raw_files {
        let path = Path::new(file);
        assert!(
            is_photo_file(path, &search_format).unwrap(),
            "Failed to recognize RAW file: {}",
            file
        );
    }

    // Verify RAW_EXTENSIONS constant contains expected formats
    assert_eq!(RAW_EXTENSIONS.len(), 7);
    assert!(RAW_EXTENSIONS.contains(&"cr2"));
    assert!(RAW_EXTENSIONS.contains(&"nef"));
}

#[test]
fn test_compressed_extensions_recognized() {
    let search_format = search_format::SearchFormat::Compressed;
    // Test that all compressed format extensions are correctly identified
    let compressed_files = vec!["photo.jpg", "image.JPEG", "pic.heif", "snap.HEIC"];

    for file in compressed_files {
        let path = Path::new(file);
        assert!(
            is_photo_file(path, &search_format).unwrap(),
            "Failed to recognize compressed file: {}",
            file
        );
    }

    // Verify COMPRESSED_EXTENSIONS constant contains expected formats
    assert_eq!(COMPRESSED_EXTENSIONS.len(), 4);
    assert!(COMPRESSED_EXTENSIONS.contains(&"jpg"));
    assert!(COMPRESSED_EXTENSIONS.contains(&"heic"));
}

#[test]
fn test_non_photo_files_rejected() {
    let search_format = search_format::SearchFormat::All;
    // Test that non-photo files are correctly rejected
    let non_photo_files = vec![
        "document.pdf",
        "video.mp4",
        "archive.zip",
        "text.txt",
        "code.rs",
        "image.png", // PNG not in supported list
        "image.gif", // GIF not in supported list
        "file.raw",  // Generic 'raw' not supported
        "noextension",
    ];

    for file in non_photo_files {
        let path = Path::new(file);
        assert!(
            !is_photo_file(path, &search_format).unwrap(),
            "Incorrectly recognized non-photo file as photo: {}",
            file
        );
    }
}

#[test]
fn test_case_insensitivity_and_edge_cases() {
    let search_format = search_format::SearchFormat::All;
    // Test case insensitivity with various capitalizations
    let mixed_case_files = vec![
        ("photo.JPG", true),
        ("image.Cr2", true),
        ("pic.NeF", true),
        ("file.HEIC", true),
        ("test.Jpeg", true),
    ];

    for (file, expected) in mixed_case_files {
        let path = Path::new(file);
        assert_eq!(
            is_photo_file(path, &search_format).unwrap(),
            expected,
            "Case insensitivity failed for: {}",
            file
        );
    }

    // Test edge cases
    let path_with_dots = Path::new("my.photo.file.jpg");
    assert!(is_photo_file(path_with_dots, &search_format).unwrap());

    let path_no_extension = Path::new("photo");
    assert!(!is_photo_file(path_no_extension, &search_format).unwrap());

    // Verify ALL_PHOTO_EXTENSIONS combines both lists
    assert_eq!(
        ALL_PHOTO_EXTENSIONS.len(),
        RAW_EXTENSIONS.len() + COMPRESSED_EXTENSIONS.len()
    );
}
