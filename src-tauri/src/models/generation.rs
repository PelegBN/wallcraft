use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiProvider {
    Pollinations,
    OpenAi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub categories: Vec<String>,
    pub custom_prompt: Option<String>,
    pub width: u32,
    pub height: u32,
    pub provider: AiProvider,
    pub target_width: u32,
    pub target_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub image_path: String,
    pub original_width: u32,
    pub original_height: u32,
    pub final_width: u32,
    pub final_height: u32,
    pub was_upscaled: bool,
}
