mod commands;
mod error;
mod models;
mod services;
mod state;

use models::settings::AppSettings;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running WallCraft");
}
