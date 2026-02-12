use std::path::PathBuf;

use crate::error::AppError;

pub async fn generate(
    prompt: &str,
    api_key: &str,
    width: u32,
    height: u32,
    output_path: &PathBuf,
) -> Result<(), AppError> {
    let size = nearest_dalle_size(width, height);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "dall-e-3",
            "prompt": prompt,
            "n": 1,
            "size": size,
            "response_format": "url"
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Generation(format!(
            "OpenAI returned status {}: {}",
            status, body
        )));
    }

    let body: serde_json::Value = response.json().await?;
    let image_url = body["data"][0]["url"]
        .as_str()
        .ok_or_else(|| AppError::Generation("No image URL in OpenAI response".into()))?;

    let img_response = client.get(image_url).send().await?;
    let bytes = img_response.bytes().await?;
    tokio::fs::write(output_path, &bytes).await?;
    Ok(())
}

fn nearest_dalle_size(w: u32, h: u32) -> String {
    let ratio = w as f64 / h as f64;
    if ratio > 1.4 {
        "1792x1024".to_string()
    } else if ratio < 0.7 {
        "1024x1792".to_string()
    } else {
        "1024x1024".to_string()
    }
}
