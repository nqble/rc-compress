use std::path::PathBuf;

use crate::core::{
    domain::{
        ExifInfo, ImageBasicInfo, ImageBatchCompressionResult, ImageColorInfo,
        ImageCompressionConfig, ImageDimensions,
    },
    image,
    image_info::ImageInfo,
};
use crate::sys::fs::delete_stale_files;

#[tauri::command]
pub async fn get_image_dynamic_range(
    app: tauri::AppHandle,
    window: tauri::WebviewWindow,
    image_path: String,
) -> Result<crate::core::image_dynamic_range::ImageDynamicRangeInfo, String> {
    // Boxed: kept out of Tauri's IPC dispatch, which runs on the main thread (1 MiB of stack on
    // Windows); see `command_futures_stay_small`
    Box::pin(crate::core::image_dynamic_range::inspect_source(
        &app,
        &image_path,
        window.label(),
        &[],
    ))
    .await
    .map(|(info, _)| info)
}

#[tauri::command]
pub async fn compress_images_batch(
    app: tauri::AppHandle,
    batch_id: &str,
    images: Vec<ImageCompressionConfig>,
) -> Result<ImageBatchCompressionResult, String> {
    let mut image_compressor = image::ImageCompressor::new(&app)?;

    if let Ok(files) = delete_stale_files(
        image_compressor.get_asset_dir().as_str(),
        24 * 60 * 60 * 1000,
    )
    .await
    {
        log::debug!(
            "[main] Stale files deleted. Number of deleted files = {}",
            files.len()
        )
    };

    // Boxed, like `get_image_dynamic_range`
    Box::pin(image_compressor.compress_images_batch(batch_id, images))
        .await
        .map(|result| Ok(result))
        .unwrap_or_else(|err| Err(err))
}
#[tauri::command]
pub async fn convert_svg_to_png(
    app: tauri::AppHandle,
    image_path: &str,
    image_id: &str,
) -> Result<String, String> {
    let image_compressor = image::ImageCompressor::new(&app)?;

    let output_filename = format!("{}.png", image_id);
    let output_path: PathBuf = [
        image_compressor.get_asset_dir(),
        output_filename.to_string(),
    ]
    .iter()
    .collect();

    let output_path_str = &*output_path.to_string_lossy();

    let _ = image_compressor.convert_svg_to_raster_img(
        &image_path,
        output_path_str,
        "png",
        Some(1.0),
    )?;
    Ok(output_path_str.to_string())
}

#[tauri::command]
pub async fn get_image_basic_info(image_path: &str) -> Result<ImageBasicInfo, String> {
    ImageInfo::get_basic_info(image_path)
}

#[tauri::command]
pub async fn get_image_dimensions(
    app: tauri::AppHandle,
    image_path: &str,
) -> Result<ImageDimensions, String> {
    if crate::core::image_codecs::is_modern(&crate::core::image_codecs::extension(image_path)) {
        let path = Box::pin(crate::core::image_codecs::decoded_png(
            &app,
            image_path,
            "main",
            &[],
        ))
        .await?;
        return ImageInfo::get_dimensions(path.to_str().ok_or("Invalid cache path")?);
    }
    ImageInfo::get_dimensions(image_path)
}

#[tauri::command]
pub async fn get_image_color_info(
    app: tauri::AppHandle,
    image_path: &str,
) -> Result<ImageColorInfo, String> {
    if crate::core::image_codecs::is_modern(&crate::core::image_codecs::extension(image_path)) {
        let path = Box::pin(crate::core::image_codecs::decoded_png(
            &app,
            image_path,
            "main",
            &[],
        ))
        .await?;
        return ImageInfo::get_color_info(path.to_str().ok_or("Invalid cache path")?);
    }
    ImageInfo::get_color_info(image_path)
}

#[tauri::command]
pub async fn prepare_image_preview(
    app: tauri::AppHandle,
    image_path: &str,
) -> Result<String, String> {
    Box::pin(crate::core::image_codecs::decoded_png(
        &app,
        image_path,
        "main",
        &[],
    ))
        .await
        .map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn get_exif_info(image_path: &str) -> Result<ExifInfo, String> {
    ImageInfo::get_exif_info(image_path)
}


#[cfg(test)]
mod tests {
    use super::*;

    /// A command's future is built on the main thread before Tauri moves it to a worker. On
    /// Windows that thread has 1 MiB of stack, and image futures near 70 KB overflowed it (every
    /// image crashed the app with 0xC00000FD), so each image command must box its large inner work.
    const MAX_COMMAND_FUTURE_BYTES: usize = 16 * 1024;

    fn future_size2<A, B, F: std::future::Future>(_: fn(A, B) -> F) -> usize {
        std::mem::size_of::<F>()
    }

    fn future_size3<A, B, C, F: std::future::Future>(_: fn(A, B, C) -> F) -> usize {
        std::mem::size_of::<F>()
    }

    #[test]
    fn command_futures_stay_small() {
        type App = tauri::AppHandle;
        let sizes = [
            (
                "get_image_dynamic_range",
                future_size3::<App, tauri::WebviewWindow, String, _>(get_image_dynamic_range),
            ),
            (
                "compress_images_batch",
                future_size3::<App, &'static str, Vec<ImageCompressionConfig>, _>(
                    compress_images_batch,
                ),
            ),
            ("get_image_dimensions", future_size2::<App, &'static str, _>(get_image_dimensions)),
            ("get_image_color_info", future_size2::<App, &'static str, _>(get_image_color_info)),
            ("prepare_image_preview", future_size2::<App, &'static str, _>(prepare_image_preview)),
            (
                "get_image_dimension",
                future_size2::<App, &'static str, _>(crate::tauri_commands::fs::get_image_dimension),
            ),
        ];
        for (name, size) in sizes {
            assert!(size <= MAX_COMMAND_FUTURE_BYTES, "{name}'s future is {size} bytes");
        }
    }
}
