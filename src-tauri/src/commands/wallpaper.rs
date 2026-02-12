use crate::error::AppError;
use crate::services::wallpaper_manager;
use crate::state::AppState;

#[tauri::command]
pub async fn get_current_wallpaper(
    state: tauri::State<'_, AppState>,
) -> Result<String, AppError> {
    let current = wallpaper_manager::get_current()?;
    *state.previous_wallpaper.lock().unwrap() = Some(current.clone());
    Ok(current)
}

#[tauri::command]
pub async fn set_wallpaper(path: String, mode: Option<String>) -> Result<(), AppError> {
    wallpaper_manager::set_from_path(&path)?;

    if let Some(mode_str) = mode {
        let wm = match mode_str.as_str() {
            "span" => wallpaper::Mode::Span,
            "fit" => wallpaper::Mode::Fit,
            "center" => wallpaper::Mode::Center,
            _ => wallpaper::Mode::Crop,
        };
        wallpaper_manager::set_mode(wm)?;
    }

    Ok(())
}

#[tauri::command]
pub async fn restore_wallpaper(state: tauri::State<'_, AppState>) -> Result<(), AppError> {
    let prev = state.previous_wallpaper.lock().unwrap().clone();
    match prev {
        Some(path) => {
            wallpaper_manager::set_from_path(&path)?;
            Ok(())
        }
        None => Err(AppError::Wallpaper("No previous wallpaper saved".into())),
    }
}
