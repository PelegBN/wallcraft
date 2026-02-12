use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub previous_wallpaper: Mutex<Option<String>>,
    pub temp_dir: Mutex<Option<PathBuf>>,
}
