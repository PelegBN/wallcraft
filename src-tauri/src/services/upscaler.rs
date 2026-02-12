use std::path::PathBuf;

use tauri::{AppHandle, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

use crate::error::AppError;

pub async fn upscale_image(
    app: &AppHandle,
    input_path: &PathBuf,
    output_path: &PathBuf,
    scale: u32,
) -> Result<(), AppError> {
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| AppError::Upscale(format!("Failed to get resource dir: {}", e)))?;
    let model_dir = resource_dir.join("binaries").join("models");

    let sidecar = app
        .shell()
        .sidecar("realesrgan-ncnn-vulkan")
        .map_err(|e| AppError::Upscale(format!("Failed to find upscaler binary: {}", e)))?
        .args([
            "-i",
            &input_path.to_string_lossy(),
            "-o",
            &output_path.to_string_lossy(),
            "-s",
            &scale.to_string(),
            "-n",
            "realesrgan-x4plus",
            "-m",
            &model_dir.to_string_lossy(),
            "-f",
            "png",
        ]);

    let (mut rx, _child) = sidecar
        .spawn()
        .map_err(|e| AppError::Upscale(format!("Failed to spawn upscaler: {}", e)))?;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Terminated(payload) => {
                if payload.code.unwrap_or(-1) != 0 {
                    return Err(AppError::Upscale(format!(
                        "Upscaler exited with code: {:?}",
                        payload.code
                    )));
                }
                break;
            }
            CommandEvent::Error(err) => {
                return Err(AppError::Upscale(format!("Upscaler error: {}", err)));
            }
            _ => {}
        }
    }

    Ok(())
}