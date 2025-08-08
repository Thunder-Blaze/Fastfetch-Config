use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Cache error: {message}")]
    Cache { message: String },

    #[error("API error: {platform} returned: {message}")]
    Api { platform: String, message: String },

    #[error("Token error: {platform} returned: {message}")]
    Token { platform: String, message: String },

    #[error("Parsing error: {message}")]
    Parse { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Invalid parameter: {param} for platform {platform}")]
    InvalidParam { platform: String, param: String },

    #[error("Platform not found: {platform}")]
    PlatformNotFound { platform: String },

    #[error("Legacy error: {0}")]
    Legacy(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, FetchError>;

impl FetchError {
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    pub fn cache(message: impl Into<String>) -> Self {
        Self::Cache {
            message: message.into(),
        }
    }

    pub fn api(platform: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Api {
            platform: platform.into(),
            message: message.into(),
        }
    }

    pub fn token(platform: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Token {
            platform: platform.into(),
            message: message.into(),
        }
    }

    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse {
            message: message.into(),
        }
    }

    pub fn invalid_param(platform: impl Into<String>, param: impl Into<String>) -> Self {
        Self::InvalidParam {
            platform: platform.into(),
            param: param.into(),
        }
    }

    pub fn platform_not_found(platform: impl Into<String>) -> Self {
        Self::PlatformNotFound {
            platform: platform.into(),
        }
    }
}
