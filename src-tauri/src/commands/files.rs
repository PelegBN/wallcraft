use std::path::PathBuf;

use base64::Engine;

use crate::error::AppError;

#[tauri::command]
pub async fn save_image_to_disk(
    source_path: String,
    destination_path: String,
) -> Result<String, AppError> {
    let source = PathBuf::from(&source_path);
    let dest = PathBuf::from(&destination_path);

    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    tokio::fs::copy(&source, &dest).await?;
    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn delete_temp_image(image_path: String) -> Result<(), AppError> {
    let path = PathBuf::from(&image_path);
    if path.exists() {
        tokio::fs::remove_file(&path).await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn read_image_base64(path: String) -> Result<String, AppError> {
    let bytes = tokio::fs::read(&path).await?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}
