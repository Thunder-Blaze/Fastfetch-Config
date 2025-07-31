use chrono::Utc;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

const TTL_SECS: i64 = 60 * 60 * 24; // 1 day

fn cache_path() -> PathBuf {
    dirs::home_dir().unwrap().join(".config/fastfetch/.cache")
}

pub fn get_cached(key: &str, username: &str) -> Option<String> {
    let file = File::open(cache_path()).ok()?;
    let reader = BufReader::new(file);

    // Collect all lines into a Vec and reverse it
    let mut lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    lines.reverse();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 4 && parts[0] == key && parts[3] == username {
            let ts: i64 = parts[2].parse().ok()?;
            if Utc::now().timestamp() - ts < TTL_SECS {
                return Some(parts[1].to_string());
            }
            break;
        }
    }
    None
}

pub fn save_cache(key: &str, val: &str, username: &str) {
    let path = cache_path();
    let timestamp = Utc::now().timestamp();

    let new_line = format!("{} {} {} {}", key, val, timestamp, username);

    // Read all lines except the one we're replacing
    let lines = if let Ok(file) = File::open(&path) {
        BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .filter(|line| {
                let parts: Vec<_> = line.split_whitespace().collect();
                !(parts.len() == 4 && parts[0] == key && parts[3] == username)
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };

    // Write back all lines + the new entry
    let mut file = File::create(&path).expect("Failed to open cache file for writing");
    for line in lines {
        writeln!(file, "{}", line).expect("Failed to write to cache file");
    }
    writeln!(file, "{}", new_line).expect("Failed to write new cache entry");
}
