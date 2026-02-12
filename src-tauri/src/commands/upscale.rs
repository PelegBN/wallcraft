use std::path::PathBuf;

use tauri::{AppHandle, Emitter};

use crate::error::AppError;
use crate::services::upscaler;

#[tauri::command]
pub async fn upscale_image(
    app: AppHandle,
    input_path: String,
    scale: u32,
) -> Result<String, AppError> {
    let _ = app.emit("generation-progress", "upscaling");

    let input = PathBuf::from(&input_path);
    let stem = input
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let output = input.with_file_name(format!("{}_upscaled.png", stem));

    upscaler::upscale_image(&app, &input, &output, scale).await?;

    let _ = app.emit("generation-progress", "complete");

    Ok(output.to_string_lossy().to_string())
}