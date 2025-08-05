use crate::{constants, error::{FetchError, Result}};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    values: HashMap<String, String>,
}

impl Config {
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
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }
    
    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        let dir = path.parent()
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
    
    pub fn validate_platform(&self, platform: &str) -> Result<String> {
        let username = self.get(platform)
            .ok_or_else(|| FetchError::config(format!("Missing {} username. Run with --setup", platform)))?;
        
        if username.trim().is_empty() {
            return Err(FetchError::config(format!("{} username is empty. Run with --setup", platform)));
        }
        
        Ok(username.clone())
    }
}

/// Path to the config file
fn config_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| FetchError::config("Could not determine home directory"))?;
    Ok(home.join(constants::CONFIG_DIR).join(constants::CONFIG_FILE))
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
    ];
    
    let mut config = Config::load().unwrap_or_else(|_| Config { values: HashMap::new() });

    println!("Setup tsukiyomi-fetch configuration");
    println!("Press Enter to skip any value or keep existing value.\n");
    
    for (platform, description) in &platforms {
        let current = config.get(*platform);
        
        if let Some(current_value) = current {
            print!("{} [current: {}]: ", description, current_value);
        } else {
            print!("{}: ", description);
        }
        
        io::stdout().flush()
            .map_err(|e| FetchError::config(format!("Failed to flush stdout: {}", e)))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
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
