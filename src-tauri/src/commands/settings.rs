use std::fs;
use std::path::PathBuf;

use crate::error::AppError;
use crate::models::settings::AppSettings;

fn settings_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("wallcraft");
    let _ = fs::create_dir_all(&config_dir);
    config_dir.join("settings.json")
}

#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, AppError> {
    let path = settings_path();
    if path.exists() {
        let content = fs::read_to_string(&path)
            .map_err(|e| AppError::FileOp(format!("Failed to read settings: {}", e)))?;
        let settings: AppSettings = serde_json::from_str(&content)
            .unwrap_or_default();
        Ok(settings)
    } else {
        Ok(AppSettings::default())
    }
}

#[tauri::command]
pub async fn save_settings(settings: AppSettings) -> Result<(), AppError> {
    let path = settings_path();
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| AppError::FileOp(format!("Failed to serialize settings: {}", e)))?;
    fs::write(&path, content)
        .map_err(|e| AppError::FileOp(format!("Failed to write settings: {}", e)))?;
    Ok(())
}
