use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Monitor detection failed: {0}")]
    MonitorDetection(String),
    #[error("Image generation failed: {0}")]
    Generation(String),
    #[error("Upscaling failed: {0}")]
    Upscale(String),
    #[error("Wallpaper operation failed: {0}")]
    Wallpaper(String),
    #[error("File operation failed: {0}")]
    FileOp(String),
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
