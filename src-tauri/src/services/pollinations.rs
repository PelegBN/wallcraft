use std::path::PathBuf;

use crate::error::AppError;

pub async fn generate(
    prompt: &str,
    target_width: u32,
    target_height: u32,
    output_path: &PathBuf,
) -> Result<(), AppError> {
    // Request at target resolution, capped at Pollinations' 2048 limit per axis
    let gen_width = target_width.min(2048);
    let gen_height = target_height.min(2048);
    let encoded_prompt = urlencoding::encode(prompt);
    let seed: u32 = rand::random();
    let url = format!(
        "https://image.pollinations.ai/prompt/{}?width={}&height={}&nologo=true&seed={}&model=flux-pro&enhance=true",
        encoded_prompt, gen_width, gen_height, seed
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::Generation(format!(
            "Pollinations returned status: {}",
            response.status()
        )));
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !content_type.starts_with("image/") {
        return Err(AppError::Generation(format!(
            "Pollinations returned non-image response: {}",
            content_type
        )));
    }

    let bytes = response.bytes().await?;
    tokio::fs::write(output_path, &bytes).await?;
    Ok(())
}
