mod commands;
mod error;
mod models;
mod services;
mod state;

use tauri::Manager;

use models::settings::AppSettings;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Set window icon for taskbar on Linux
            #[cfg(target_os = "linux")]
            {
                use std::path::PathBuf;
                // Install .desktop file and icon at runtime for dev mode
                if let Some(home) = dirs::home_dir() {
                    let icon_src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("icons/icon.png");
                    let icon_dest = home.join(".local/share/icons/hicolor/512x512/apps/wallcraft.png");
                    let icon_128 = home.join(".local/share/icons/hicolor/128x128/apps/wallcraft.png");
                    let icon_src_128 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("icons/128x128.png");
                    let desktop_dest = home.join(".local/share/applications/wallcraft.desktop");

                    // Create dirs and copy icon
                    let _ = std::fs::create_dir_all(icon_dest.parent().unwrap());
                    let _ = std::fs::create_dir_all(icon_128.parent().unwrap());
                    let _ = std::fs::copy(&icon_src, &icon_dest);
                    let _ = std::fs::copy(&icon_src_128, &icon_128);

                    // Write desktop entry (only if it doesn't already exist)
                    if !desktop_dest.exists() {
                        let exe_path = std::env::current_exe().unwrap_or_default();
                        let desktop_content = format!(
                            "[Desktop Entry]\nName=WallCraft\nComment=AI-powered desktop wallpaper generator\nExec={}\nIcon={}\nType=Application\nCategories=Utility;\nStartupWMClass=wallcraft\n",
                            exe_path.display(),
                            icon_128.display()
                        );
                        let _ = std::fs::write(&desktop_dest, desktop_content);
                    }

                    // Update icon cache and desktop database
                    let _ = std::process::Command::new("gtk-update-icon-cache")
                        .args(["-f", "-t"])
                        .arg(home.join(".local/share/icons/hicolor"))
                        .output();
                    let _ = std::process::Command::new("update-desktop-database")
                        .arg(home.join(".local/share/applications"))
                        .output();
                }
            }

            // Set window icon via Tauri API
            if let Some(window) = app.get_webview_window("main") {
                let png_bytes = include_bytes!("../icons/128x128.png");
                match image::load_from_memory(png_bytes) {
                    Ok(img) => {
                        let rgba = img.to_rgba8();
                        let (w, h) = rgba.dimensions();
                        let icon = tauri::image::Image::new_owned(
                            rgba.into_raw(),
                            w,
                            h,
                        );
                        eprintln!("Setting window icon: {w}x{h}");
                        if let Err(e) = window.set_icon(icon) {
                            eprintln!("Failed to set window icon: {e}");
                        }
                    }
                    Err(e) => eprintln!("Failed to decode icon PNG: {e}"),
                }
            }
            Ok(())
        })
        .manage(AppState::default())
        .manage(AppSettings::default())
        .invoke_handler(tauri::generate_handler![
            commands::monitor::get_monitors,
            commands::generation::generate_image,
            commands::wallpaper::get_current_wallpaper,
            commands::wallpaper::set_wallpaper,
            commands::wallpaper::restore_wallpaper,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::upscale::upscale_image,
            commands::files::save_image_to_disk,
            commands::files::delete_temp_image,
            commands::files::read_image_base64,
        ])
        .run(tauri::generate_context!())
        .expect("error while running WallCraft");
}
