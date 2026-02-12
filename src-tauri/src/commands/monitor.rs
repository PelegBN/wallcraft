use tauri::WebviewWindow;

use crate::error::AppError;
use crate::models::monitor::{MonitorInfo, MonitorLayout};

#[tauri::command]
pub async fn get_monitors(window: WebviewWindow) -> Result<MonitorLayout, AppError> {
    let available = window
        .available_monitors()
        .map_err(|e| AppError::MonitorDetection(e.to_string()))?;

    let primary = window
        .primary_monitor()
        .map_err(|e| AppError::MonitorDetection(e.to_string()))?;

    let primary_name = primary.and_then(|p| p.name().map(|n| n.to_string()));

    let mut monitors: Vec<MonitorInfo> = Vec::new();
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for monitor in &available {
        let name = monitor.name().cloned().unwrap_or_default();
        let size = monitor.size();
        let pos = monitor.position();
        let scale = monitor.scale_factor();

        let is_primary = primary_name.as_deref() == Some(&name);

        let info = MonitorInfo {
            name: name.clone(),
            width: size.width,
            height: size.height,
            x: pos.x,
            y: pos.y,
            scale_factor: scale,
            is_primary,
        };

        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x + size.width as i32);
        max_y = max_y.max(pos.y + size.height as i32);

        monitors.push(info);
    }

    Ok(MonitorLayout {
        monitors,
        total_width: (max_x - min_x) as u32,
        total_height: (max_y - min_y) as u32,
    })
}
