use std::path::PathBuf;

use crate::error::AppError;

pub async fn generate(
    prompt: &str,
    width: u32,
    height: u32,
    output_path: &PathBuf,
) -> Result<(), AppError> {
    let encoded_prompt = urlencoding::encode(prompt);
    let seed: u32 = rand::random();
    let url = format!(
        "https://image.pollinations.ai/prompt/{}?width={}&height={}&nologo=true&seed={}&model=flux&enhance=true",
        encoded_prompt, width, height, seed
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

    let bytes = response.bytes().await?;
    tokio::fs::write(output_path, &bytes).await?;
    Ok(())
}
