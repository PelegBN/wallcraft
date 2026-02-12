use crate::error::AppError;

pub fn get_current() -> Result<String, AppError> {
    wallpaper::get().map_err(|e| AppError::Wallpaper(format!("Failed to get wallpaper: {}", e)))
}

pub fn set_from_path(path: &str) -> Result<(), AppError> {
    wallpaper::set_from_path(path)
        .map_err(|e| AppError::Wallpaper(format!("Failed to set wallpaper: {}", e)))
}

pub fn set_mode(mode: wallpaper::Mode) -> Result<(), AppError> {
    wallpaper::set_mode(mode)
        .map_err(|e| AppError::Wallpaper(format!("Failed to set wallpaper mode: {}", e)))
}
