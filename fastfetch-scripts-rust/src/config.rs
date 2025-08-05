use std::collections::HashMap;
use std::fs::{self};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

/// Path to the config file
fn config_path() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join(".config/fastfetch/tsukiyomi-fetch.conf"))
}

/// Get a config value by key (e.g. "GitHub" => "username")
pub fn get_config_value(key: &str) -> Option<String> {
    let path = config_path()?;
    for line in fs::read_to_string(path).ok()?.lines() {
        if let Some(v) = line.strip_prefix(&format!("{}=", key)) {
            return Some(v.to_string());
        }
    }
    None
}

/// Setup interactive config creation or update
pub fn setup() -> io::Result<()> {
    let keys = [
        "GitHub",
        "Codeforces",
        "CodeChef",
        "LeetCode",
        "AniList",
        "Simkl",
        "MyAnimeList",
        "Instagram",
    ];
    let path = config_path().expect("Could not determine home directory");
    let dir = path.parent().unwrap();

    fs::create_dir_all(dir)?;

    // Load existing config
    let mut current: HashMap<String, String> = HashMap::new();
    if path.exists() {
        for line in BufReader::new(fs::File::open(&path)?).lines().flatten() {
            if let Some((k, v)) = line.split_once('=') {
                current.insert(k.to_string(), v.to_string());
            }
        }
    }

    // Prompt user
    println!("Setup fastfetch config. Press Enter to skip any value.");
    for key in &keys {
        print!("{}: ", key);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim();
        if !value.is_empty() {
            current.insert(key.to_string(), value.to_string());
        }
    }

    // Rewrite config file
    let mut file = fs::File::create(&path)?;
    for (k, v) in current {
        writeln!(file, "{}={}", k, v)?;
    }

    println!("Config saved to {}", path.display());
    Ok(())
}
