//! # Configuration Management Module
//!
//! This module handles persistent configuration storage for Tsukiyomi-Fetch.
//! It provides functionality to store and retrieve user credentials, API keys,
//! and platform-specific settings required for fetching statistics.
//!
//! ## Features
//!
//! - **Persistent storage**: Configuration is saved to `~/.config/fastfetch/tsukiyomi-fetch.conf`
//! - **Key-value format**: Simple `key=value` format for easy editing and parsing
//! - **Interactive setup**: Guided configuration process for all supported platforms
//! - **Secure handling**: Sensitive data like API keys are stored locally
//! - **Validation**: Input validation during the setup process
//!
//! ## Configuration Format
//!
//! The configuration file uses a simple key-value format:
//! ```text
//! GitHub=username
//! Codeforces=username
//! CodeChef=username
//! # Comments are supported
//! ```
//!
//! ## Supported Platforms
//!
//! The configuration system supports setup for all platforms that Tsukiyomi-Fetch
//! can fetch data from, including usernames, API keys, and other required credentials.

use crate::{
    constants,
    error::{FetchError, Result},
};
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};

/// Represents the application configuration with key-value storage.
///
/// This struct provides an interface for loading, modifying, and saving
/// configuration data. It stores all configuration values as strings
/// in a HashMap for flexible access.
#[derive(Debug, Clone)]
pub struct Config {
    /// Internal storage for configuration key-value pairs
    values: HashMap<String, String>,
}

impl Config {
    /// Loads configuration from the config file.
    ///
    /// If the config file doesn't exist, returns an empty configuration.
    /// The file is parsed line by line, expecting `key=value` format.
    /// Lines that don't match this format are silently ignored.
    ///
    /// # Returns
    ///
    /// `Ok(Config)` with loaded configuration, or `Err(FetchError)` if
    /// the file exists but cannot be read.
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        let mut values = HashMap::new();

        if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| FetchError::config(format!("Failed to read config file: {}", e)))?;

            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    values.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        Ok(Self { values })
    }

    /// Retrieves a configuration value by key.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key to look up
    ///
    /// # Returns
    ///
    /// `Some(&String)` if the key exists, `None` otherwise
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    /// Sets a configuration value.
    ///
    /// # Arguments
    ///
    /// * `key` - The configuration key to set
    /// * `value` - The value to associate with the key
    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    /// Saves the current configuration to the config file.
    ///
    /// This method creates the configuration directory if it doesn't exist,
    /// then writes all key-value pairs to the config file in `key=value` format.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the configuration was saved successfully,
    /// `Err(FetchError)` if directory creation or file writing fails.
    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        let dir = path
            .parent()
            .ok_or_else(|| FetchError::config("Invalid config path"))?;

        fs::create_dir_all(dir)
            .map_err(|e| FetchError::config(format!("Failed to create config directory: {}", e)))?;

        let mut file = fs::File::create(&path)
            .map_err(|e| FetchError::config(format!("Failed to create config file: {}", e)))?;

        for (key, value) in &self.values {
            writeln!(file, "{}={}", key, value)
                .map_err(|e| FetchError::config(format!("Failed to write config: {}", e)))?;
        }

        Ok(())
    }
}

/// Path to the config file
fn config_path() -> Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| FetchError::config("Could not determine home directory"))?;
    Ok(home
        .join(constants::CONFIG_DIR)
        .join(constants::CONFIG_FILE))
}

/// Get a config value by key (e.g. "GitHub" => "username")
pub fn get_config_value(key: &str) -> Option<String> {
    Config::load().ok()?.get(key).cloned()
}

/// Setup interactive config creation or update
pub fn setup() -> Result<()> {
    let platforms = [
        ("GitHub", "GitHub username"),
        ("Codeforces", "Codeforces handle"),
        ("CodeChef", "CodeChef username"),
        ("LeetCode", "LeetCode username"),
        ("AniList", "AniList username"),
        ("Simkl", "Simkl user ID"),
        ("MyAnimeList", "MyAnimeList username"),
        ("Instagram", "Instagram username"),
        ("Reddit", "Reddit username"),
        ("Steam", "Steam ID (numeric)"),
        ("Twitter", "Twitter username"),
        ("Discord", "Discord ID (numeric)"),
    ];

    let mut config = Config::load().unwrap_or_else(|_| Config {
        values: HashMap::new(),
    });

    println!("Setup tsukiyomi-fetch configuration");
    println!("Press Enter to skip any value or keep existing value.\n");

    for (platform, description) in &platforms {
        let current = config.get(*platform);

        if let Some(current_value) = current {
            print!("{} [current: {}]: ", description, current_value);
        } else {
            print!("{}: ", description);
        }

        io::stdout()
            .flush()
            .map_err(|e| FetchError::config(format!("Failed to flush stdout: {}", e)))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| FetchError::config(format!("Failed to read input: {}", e)))?;

        let value = input.trim();
        if !value.is_empty() {
            config.set(platform.to_string(), value.to_string());
        }
    }

    config.save()?;

    let path = config_path()?;
    println!("\nConfiguration saved to {}", path.display());
    println!("You can now run tsukiyomi-fetch with your configured platforms!");

    Ok(())
}

pub fn get_token(platform: &str, required: bool, key: bool) -> Result<Option<String>> {
    if let Some(token) = std::env::var(format!("{}", platform.to_uppercase())).ok() {
        return Ok(Some(token));
    } else if !required {
        return Ok(None);
    }
    if key {
        Err(FetchError::api_key(platform, "Required api key is missing"))
    } else {
        Err(FetchError::token(platform, "Required token is missing"))
    }
}
