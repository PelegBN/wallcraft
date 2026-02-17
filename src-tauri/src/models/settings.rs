use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ai_provider: String,
    pub openai_api_key: Option<String>,
    pub save_directory: Option<String>,
    pub upscale_enabled: bool,
    pub upscale_factor: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ai_provider: "pollinations".to_string(),
            openai_api_key: None,
            save_directory: None,
            upscale_enabled: true,
            upscale_factor: 4,
        }
    }
}
