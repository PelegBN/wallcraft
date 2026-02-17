use std::path::PathBuf;

use tauri::{AppHandle, Emitter};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::generation::{AiProvider, GenerationRequest, GenerationResult};
use crate::models::settings::AppSettings;
use crate::services::{openai, pollinations, svg_generator};
use crate::state::AppState;

#[tauri::command]
pub async fn generate_image(
    app: AppHandle,
    request: GenerationRequest,
    state: tauri::State<'_, AppState>,
    settings: tauri::State<'_, AppSettings>,
) -> Result<GenerationResult, AppError> {
    let _ = app.emit("generation-progress", "starting");

    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("wallcraft");
    tokio::fs::create_dir_all(&cache_dir).await?;

    let filename = format!("{}.png", Uuid::new_v4());
    let output_path = cache_dir.join(&filename);

    let _ = app.emit("generation-progress", "generating");

    let use_svg = !request.styles.is_empty();

    let (final_width, final_height) = if use_svg {
        // Vector art mode: generate SVG → rasterize to PNG at exact target resolution
        svg_generator::generate(
            &request.styles,
            &request.color_schemes,
            request.custom_prompt.as_deref(),
            request.target_width,
            request.target_height,
            &output_path,
        )?;
        (request.target_width, request.target_height)
    } else {
        // Direct prompt mode: use AI image generation API
        let prompt = build_prompt(request.custom_prompt.as_deref());

        let (gen_width, gen_height) = match request.provider {
            AiProvider::Pollinations => {
                pollinations::generate(
                    &prompt,
                    request.target_width,
                    request.target_height,
                    &output_path,
                )
                .await?;
                read_image_dimensions(&output_path)?
            }
            AiProvider::OpenAi => {
                let api_key = settings
                    .openai_api_key
                    .as_deref()
                    .ok_or_else(|| {
                        AppError::Generation("OpenAI API key not configured".into())
                    })?;
                openai::generate(&prompt, api_key, 1024, 1024, &output_path).await?;
                (1024, 1024)
            }
        };

        // Resize if the API returned smaller than target
        if gen_width < request.target_width || gen_height < request.target_height {
            resize_image(&output_path, request.target_width, request.target_height)?;
            (request.target_width, request.target_height)
        } else {
            (gen_width, gen_height)
        }
    };

    let _ = app.emit("generation-progress", "complete");

    *state.temp_dir.lock().unwrap() = Some(cache_dir);

    Ok(GenerationResult {
        image_path: output_path.to_string_lossy().to_string(),
        original_width: final_width,
        original_height: final_height,
        final_width,
        final_height,
        was_upscaled: false,
    })
}

fn read_image_dimensions(path: &PathBuf) -> Result<(u32, u32), AppError> {
    let reader = image::ImageReader::open(path)
        .map_err(|e| AppError::Generation(format!("Failed to open generated image: {}", e)))?
        .with_guessed_format()
        .map_err(|e| AppError::Generation(format!("Failed to guess image format: {}", e)))?;
    let img = reader
        .decode()
        .map_err(|e| AppError::Generation(format!("Failed to decode generated image: {}", e)))?;
    Ok((img.width(), img.height()))
}

fn resize_image(path: &PathBuf, target_width: u32, target_height: u32) -> Result<(), AppError> {
    let reader = image::ImageReader::open(path)
        .map_err(|e| AppError::Generation(format!("Failed to open image for resize: {}", e)))?
        .with_guessed_format()
        .map_err(|e| AppError::Generation(format!("Failed to guess format for resize: {}", e)))?;
    let img = reader
        .decode()
        .map_err(|e| AppError::Generation(format!("Failed to decode image for resize: {}", e)))?;
    let resized =
        img.resize_exact(target_width, target_height, image::imageops::FilterType::Lanczos3);
    resized
        .save(path)
        .map_err(|e| AppError::Generation(format!("Failed to save resized image: {}", e)))?;
    Ok(())
}

/// Build prompt for direct prompt mode (AI generation, not vector art)
fn build_prompt(custom: Option<&str>) -> String {
    let quality_suffix = "masterpiece, best quality, ultra-detailed, sharp focus, \
        desktop wallpaper, no text, no watermark, no logo, no signature";

    match custom {
        Some(text) if !text.trim().is_empty() => {
            format!("{}, {}", text.trim(), quality_suffix)
        }
        _ => format!("A beautiful abstract desktop wallpaper, {}", quality_suffix),
    }
}
