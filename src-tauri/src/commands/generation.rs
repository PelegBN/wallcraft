use std::path::PathBuf;

use tauri::{AppHandle, Emitter};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::generation::{AiProvider, GenerationRequest, GenerationResult};
use crate::models::settings::AppSettings;
use crate::services::{openai, pollinations};
use crate::state::AppState;

#[tauri::command]
pub async fn generate_image(
    app: AppHandle,
    request: GenerationRequest,
    state: tauri::State<'_, AppState>,
    settings: tauri::State<'_, AppSettings>,
) -> Result<GenerationResult, AppError> {
    let _ = app.emit("generation-progress", "starting");

    let prompt = build_prompt(&request.categories, request.custom_prompt.as_deref());

    let (gen_width, gen_height) = match request.provider {
        AiProvider::Pollinations => (request.width.min(2048), request.height.min(2048)),
        AiProvider::OpenAi => (1024, 1024),
    };

    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("wallcraft");
    tokio::fs::create_dir_all(&cache_dir).await?;

    let filename = format!("{}.png", Uuid::new_v4());
    let output_path = cache_dir.join(&filename);

    let _ = app.emit("generation-progress", "generating");

    match request.provider {
        AiProvider::Pollinations => {
            pollinations::generate(&prompt, gen_width, gen_height, &output_path).await?;
        }
        AiProvider::OpenAi => {
            let api_key = settings
                .openai_api_key
                .as_deref()
                .ok_or_else(|| AppError::Generation("OpenAI API key not configured".into()))?;
            openai::generate(&prompt, api_key, gen_width, gen_height, &output_path).await?;
        }
    }

    let _ = app.emit("generation-progress", "complete");

    *state.temp_dir.lock().unwrap() = Some(cache_dir);

    Ok(GenerationResult {
        image_path: output_path.to_string_lossy().to_string(),
        original_width: gen_width,
        original_height: gen_height,
        final_width: gen_width,
        final_height: gen_height,
        was_upscaled: false,
    })
}

fn build_prompt(categories: &[String], custom: Option<&str>) -> String {
    let mut parts: Vec<String> = Vec::new();

    if !categories.is_empty() {
        parts.push(format!(
            "A stunning desktop wallpaper featuring: {}",
            categories.join(", ")
        ));
    }

    if let Some(custom_text) = custom {
        if !custom_text.trim().is_empty() {
            parts.push(custom_text.to_string());
        }
    }

    let quality_suffix = "ultra high resolution, photorealistic, sharp details, professional quality, 8K UHD desktop wallpaper, wide aspect ratio, vibrant colors, stunning composition";

    if parts.is_empty() {
        format!("A beautiful abstract desktop wallpaper, {}", quality_suffix)
    } else {
        format!("{}, {}", parts.join(". "), quality_suffix)
    }
}
