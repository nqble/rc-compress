//! Which files rc-compress can make smaller, by their extension: the ones the right-click menu queues. Anything else
//! (a `.txt`, a `.zip`) is skipped without a trace instead of failing in the queue.
//!
//! The list must match the app's `inputExtensions` (`src/types/compression.ts`) plus PDFs;
//! `scripts/test-compressible.mjs` checks that. It is kept here, not only sent by the windows, because a launch from
//! the menu queues its files before any window has loaded.

use std::path::Path;

/// Videos, images, audio (`inputExtensions` in `compression.ts`; `gif` is listed once) and PDFs, lower case
pub const COMPRESSIBLE_EXTENSIONS: [&str; 36] = [
    // Video
    "mp4", "mov", "mkv", "webm", "avi", "ts", "gif", "m4v", "mts", "m2ts",
    // Image
    "png", "jpg", "jpeg", "webp", "svg", "avif", "jxl", "heic", "heif", "bmp", "tiff", "tif",
    // Audio
    "mp3", "m4a", "wav", "flac", "ogg", "opus", "aac", "m4b", "aiff", "aif", "wma", "oga", "mp2",
    // PDF
    "pdf",
];

/// Whether rc-compress can compress this file, by its extension (any case)
pub fn is_compressible(path: &str) -> bool {
    let Some(extension) = Path::new(path).extension().and_then(|extension| extension.to_str()) else {
        return false;
    };
    let extension = extension.to_lowercase();
    COMPRESSIBLE_EXTENSIONS.contains(&extension.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_and_pdfs_are_compressible() {
        for path in ["a.MP4", "b.jpg", "c.heic", "d.wav", "e.mp2", "C:\\docs\\f.PDF"] {
            assert!(is_compressible(path), "{path}");
        }
    }

    #[test]
    fn other_files_are_not() {
        for path in ["notes.txt", "photos.zip", "setup.exe", "no-extension", "archive.pdf.zip"] {
            assert!(!is_compressible(path), "{path}");
        }
    }
}
