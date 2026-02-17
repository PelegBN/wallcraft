use crate::error::AppError;

pub fn get_current() -> Result<String, AppError> {
    #[cfg(target_os = "linux")]
    {
        if is_gnome() {
            if let Ok(path) = get_gnome_dark_wallpaper() {
                if !path.is_empty() {
                    return Ok(path);
                }
            }
        }
    }

    wallpaper::get().map_err(|e| AppError::Wallpaper(format!("Failed to get wallpaper: {}", e)))
}

pub fn set_from_path(path: &str) -> Result<(), AppError> {
    wallpaper::set_from_path(path)
        .map_err(|e| AppError::Wallpaper(format!("Failed to set wallpaper: {}", e)))?;

    #[cfg(target_os = "linux")]
    {
        if is_gnome() {
            if let Err(e) = set_gnome_dark_wallpaper(path) {
                log::warn!("Failed to set GNOME dark wallpaper: {}", e);
            }
        }
    }

    Ok(())
}

pub fn set_mode(mode: wallpaper::Mode) -> Result<(), AppError> {
    wallpaper::set_mode(mode)
        .map_err(|e| AppError::Wallpaper(format!("Failed to set wallpaper mode: {}", e)))
}

#[cfg(target_os = "linux")]
fn is_gnome() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP")
        .map(|d| {
            let lower = d.to_lowercase();
            lower.contains("gnome") || lower.contains("unity") || lower.contains("budgie")
        })
        .unwrap_or(false)
}

#[cfg(target_os = "linux")]
fn get_gnome_dark_wallpaper() -> Result<String, AppError> {
    let output = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.background", "picture-uri-dark"])
        .output()
        .map_err(|e| AppError::Wallpaper(format!("Failed to run gsettings: {}", e)))?;

    let uri = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let cleaned = uri.trim_matches('\'').trim_matches('"').to_string();
    let path = cleaned
        .strip_prefix("file://")
        .unwrap_or(&cleaned)
        .to_string();
    Ok(path)
}

#[cfg(target_os = "linux")]
fn set_gnome_dark_wallpaper(path: &str) -> Result<(), AppError> {
    let uri = if path.starts_with("file://") {
        path.to_string()
    } else {
        format!("file://{}", path)
    };

    let output = std::process::Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &uri,
        ])
        .output()
        .map_err(|e| AppError::Wallpaper(format!("Failed to set dark wallpaper: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("gsettings picture-uri-dark failed: {}", stderr);
    }

    Ok(())
}
