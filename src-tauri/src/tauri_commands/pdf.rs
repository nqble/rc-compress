//! PDF compression with the bundled Ghostscript (docs/ghostscript.md): rewrites a PDF with its pictures at a lower
//! resolution, keeping its text as text. Used by the background queue for PDFs from the right-click menu and the app
//! window (see `src/routes/webviews/tray/-queue.ts`); the levels match `src/utils/pdfLevels.ts`.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use tauri::{AppHandle, Manager};

use crate::core::domain::CollectedFile;

/// How small to make a PDF: the resolution of its pictures (Ghostscript's `-dPDFSETTINGS`)
#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PdfLevel {
    /// About 72 dpi: reading on screen, email
    Small,
    /// About 150 dpi: most documents
    Medium,
    /// About 300 dpi: printing
    High,
}

impl PdfLevel {
    fn ghostscript_setting(self) -> &'static str {
        match self {
            PdfLevel::Small => "/screen",
            PdfLevel::Medium => "/ebook",
            PdfLevel::High => "/printer",
        }
    }
}

/// Ghostscript's arguments to rewrite `input` into `output` at `level`. With `strip_metadata`, the
/// document information (title, author, dates, the program that made it) is emptied.
fn ghostscript_args(input: &Path, output: &Path, level: PdfLevel, strip_metadata: bool) -> Vec<String> {
    let mut args = vec![
        "-q".to_string(),
        // No access to files other than the ones named here
        "-dSAFER".to_string(),
        "-dBATCH".to_string(),
        "-dNOPAUSE".to_string(),
        "-sDEVICE=pdfwrite".to_string(),
        "-dCompatibilityLevel=1.6".to_string(),
        format!("-dPDFSETTINGS={}", level.ghostscript_setting()),
        format!("-sOutputFile={}", output.to_string_lossy()),
        input.to_string_lossy().to_string(),
    ];
    if strip_metadata {
        // Applied after the document: empties its information dictionary
        args.extend([
            "-c".to_string(),
            "[ /Title () /Author () /Subject () /Keywords () /Creator () /Producer () /DOCINFO pdfmark"
                .to_string(),
        ]);
    }
    args
}

/// Whether a file is a PDF (by its extension). Only the queue compresses PDFs: the main window hands
/// them over (`queue_pdfs`)
pub fn is_pdf(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

/// The PDFs among `files`, the rest left out
fn pdfs_only(files: Vec<CollectedFile>) -> Vec<CollectedFile> {
    files.into_iter().filter(|file| is_pdf(&file.path)).collect()
}

/// Queues the PDFs dropped on or opened in the main window, which knows only videos, images and
/// audio: they are compressed in the background with Quick Compress's PDF quality, like from the
/// right-click menu. Returns how many were queued (one already waiting is not queued again).
#[tauri::command]
pub fn queue_pdfs(app_handle: AppHandle, files: Vec<CollectedFile>) -> usize {
    let queue = crate::tauri_commands::drop_queue::add_files_to_queue;
    queue(&app_handle, pdfs_only(files), None, None).len()
}

/// Where a compressed PDF waits before it is saved: the app's cache, under a new name
fn output_path(app: &AppHandle) -> Result<PathBuf, String> {
    let cache = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&cache).map_err(|e| e.to_string())?;
    Ok(cache.join(format!("{}.pdf", nanoid::nanoid!())))
}

/// Compresses the PDF at `path` and returns where the copy is (in the app's cache) and its size
#[tauri::command]
pub async fn compress_pdf(
    app: AppHandle,
    path: String,
    level: PdfLevel,
    strip_metadata: bool,
) -> Result<(String, u64), String> {
    let output = output_path(&app)?;
    let mut command = crate::core::tool_bundle::command(&app, "ghostscript")?;
    command.args(ghostscript_args(Path::new(&path), &output, level, strip_metadata));
    // Ghostscript reads only the files it is given, never what is typed
    command.stdin(std::process::Stdio::null());
    let result = tauri::async_runtime::spawn_blocking(move || command.output())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        let _ = std::fs::remove_file(&output);
        return Err(String::from_utf8_lossy(&result.stderr).trim().to_string());
    }
    let size = std::fs::metadata(&output).map_err(|e| e.to_string())?.len();
    Ok((output.to_string_lossy().to_string(), size))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(level: PdfLevel, strip_metadata: bool) -> Vec<String> {
        ghostscript_args(Path::new("in.pdf"), Path::new("out.pdf"), level, strip_metadata)
    }

    #[test]
    fn each_level_picks_its_ghostscript_setting() {
        assert!(args(PdfLevel::Small, false).contains(&"-dPDFSETTINGS=/screen".to_string()));
        assert!(args(PdfLevel::Medium, false).contains(&"-dPDFSETTINGS=/ebook".to_string()));
        assert!(args(PdfLevel::High, false).contains(&"-dPDFSETTINGS=/printer".to_string()));
    }

    #[test]
    fn ghostscript_runs_in_safe_mode_and_writes_the_output_given() {
        let args = args(PdfLevel::Medium, false);
        assert!(args.contains(&"-dSAFER".to_string()));
        assert!(args.contains(&"-sOutputFile=out.pdf".to_string()));
        assert_eq!(args.last().unwrap(), "in.pdf");
    }

    #[test]
    fn pdfs_are_known_by_their_extension() {
        assert!(is_pdf("C:\\docs\\Report.PDF"));
        assert!(is_pdf("/home/me/a.pdf"));
        assert!(!is_pdf("/home/me/pdf"));
        assert!(!is_pdf("/home/me/a.pdf.zip"));
    }

    fn file(path: &str) -> CollectedFile {
        CollectedFile { path: path.to_string(), relative_dir: String::new() }
    }

    #[test]
    fn only_pdfs_are_handed_to_the_queue() {
        let files = vec![file("a.pdf"), file("b.mp4"), file("c.PDF"), file("d.png")];
        let paths: Vec<String> = pdfs_only(files).into_iter().map(|file| file.path).collect();
        assert_eq!(paths, ["a.pdf", "c.PDF"]);
    }

    #[test]
    fn metadata_is_emptied_only_when_asked() {
        assert!(!args(PdfLevel::Medium, false).iter().any(|arg| arg.contains("DOCINFO")));
        assert!(args(PdfLevel::High, true).last().unwrap().contains("/DOCINFO pdfmark"));
    }
}
