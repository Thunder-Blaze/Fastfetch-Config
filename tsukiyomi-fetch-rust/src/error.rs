//! # Error Handling Module
//!
//! This module defines comprehensive error types for Tsukiyomi-Fetch using the `thiserror` crate.
//! It provides structured error handling for all operations including network requests,
//! configuration management, caching, API interactions, and data parsing.
//!
//! ## Error Categories
//!
//! - **Configuration errors**: Issues with reading/writing config files
//! - **Network errors**: HTTP request failures, timeouts, connectivity issues
//! - **Cache errors**: Problems with cache file operations
//! - **API errors**: Platform-specific API response errors
//! - **Authentication errors**: Token and API key validation failures
//! - **Parsing errors**: JSON deserialization and data format issues
//! - **Platform errors**: Unknown or unsupported platform requests
//!
//! ## Design Philosophy
//!
//! The error system is designed to provide:
//! - **Clear error messages**: Descriptive error text for debugging
//! - **Platform context**: Which platform caused the error when applicable
//! - **Automatic conversion**: From standard library and third-party errors
//! - **Convenient constructors**: Helper methods for common error patterns

use thiserror::Error;

/// Comprehensive error type for all Tsukiyomi-Fetch operations
/// 
/// This enum covers all possible error conditions that can occur during
/// the execution of fetch operations, configuration management, and data processing.
#[derive(Error, Debug)]
pub enum FetchError {
    /// Configuration file or setup related errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Network connectivity and HTTP request errors
    /// Automatically converts from `reqwest::Error`
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Cache file operations and data persistence errors
    #[error("Cache error: {message}")]
    Cache { message: String },

    /// Platform API response errors with context
    #[error("API error: {platform} returned: {message}")]
    Api { platform: String, message: String },

    /// Authentication token validation errors
    #[error("Token error: {platform} returned: {message}")]
    Token { platform: String, message: String },

    /// API key validation and authentication errors
    #[error("Api Key error: {platform} returned: {message}")]
    ApiKey { platform: String, message: String },

    /// Data parsing and format validation errors
    #[error("Parsing error: {message}")]
    Parse { message: String },

    /// File system I/O errors
    /// Automatically converts from `std::io::Error`
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    /// Automatically converts from `serde_json::Error`
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Regular expression compilation and matching errors
    /// Automatically converts from `regex::Error`
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// Invalid parameter values for platform requests
    #[error("Invalid parameter: {param} for platform {platform}")]
    InvalidParam { platform: String, param: String },

    /// Requests for unsupported or unknown platforms
    #[error("Platform not found: {platform}")]
    PlatformNotFound { platform: String },

    /// Legacy error handling for backward compatibility
    /// Automatically converts from `anyhow::Error`
    #[error("Legacy error: {0}")]
    Legacy(#[from] anyhow::Error),
}

/// Type alias for Results using FetchError
/// 
/// This provides a convenient shorthand for functions that return
/// either a success value of type T or a FetchError.
pub type Result<T> = std::result::Result<T, FetchError>;

impl FetchError {
    /// Create a configuration error with a custom message
    /// 
    /// # Arguments
    /// * `message` - Description of the configuration issue
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a cache error with a custom message
    /// 
    /// # Arguments
    /// * `message` - Description of the cache operation issue
    pub fn cache(message: impl Into<String>) -> Self {
        Self::Cache {
            message: message.into(),
        }
    }

    /// Create an API error with platform context
    /// 
    /// # Arguments
    /// * `platform` - Name of the platform that caused the error
    /// * `message` - Description of the API response issue
    pub fn api(platform: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Api {
            platform: platform.into(),
            message: message.into(),
        }
    }

    /// Create a token authentication error with platform context
    /// 
    /// # Arguments
    /// * `platform` - Name of the platform with token issues
    /// * `message` - Description of the token validation problem
    pub fn token(platform: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Token {
            platform: platform.into(),
            message: message.into(),
        }
    }

    /// Create an API key authentication error with platform context
    /// 
    /// # Arguments
    /// * `platform` - Name of the platform with API key issues
    /// * `message` - Description of the API key validation problem
    pub fn api_key(platform: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ApiKey {
            platform: platform.into(),
            message: message.into(),
        }
    }

    /// Create a parsing error with a custom message
    /// 
    /// # Arguments
    /// * `message` - Description of the data parsing issue
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse {
            message: message.into(),
        }
    }

    /// Create an invalid parameter error with platform and parameter context
    /// 
    /// # Arguments
    /// * `platform` - Name of the platform receiving invalid parameters
    /// * `param` - Name of the invalid parameter
    pub fn invalid_param(platform: impl Into<String>, param: impl Into<String>) -> Self {
        Self::InvalidParam {
            platform: platform.into(),
            param: param.into(),
        }
    }

    /// Create a platform not found error
    /// 
    /// # Arguments
    /// * `platform` - Name of the unsupported platform
    pub fn platform_not_found(platform: impl Into<String>) -> Self {
        Self::PlatformNotFound {
            platform: platform.into(),
        }
    }
}
