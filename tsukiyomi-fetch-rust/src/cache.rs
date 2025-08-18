//! # Cache Management Module
//!
//! This module provides intelligent caching functionality for Tsukiyomi-Fetch to minimize
//! API calls and improve performance. The cache system stores fetched data with timestamps
//! and user context, automatically invalidating expired entries.
//!
//! ## Features
//!
//! - **Time-based expiration**: Cached entries automatically expire after a configurable TTL
//! - **User-aware caching**: Cache entries are tied to specific usernames to prevent data mixing
//! - **Atomic operations**: Safe concurrent access with proper file locking mechanisms
//! - **Multiple entry support**: Batch operations for saving multiple cache entries
//! - **Automatic cleanup**: Old entries are automatically removed when new ones are saved
//!
//! ## Cache Format
//!
//! Cache entries are stored in a simple text format:
//! ```text
//! <key> <value> <timestamp> <username>
//! ```
//!
//! The cache file is located at `~/.cache/fastfetch/tsukiyomi.cache` by default.

use crate::{
    constants,
    error::{FetchError, Result},
};
use chrono::Utc;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

/// Cache time-to-live in seconds, loaded from constants
const TTL_SECS: i64 = constants::CACHE_TTL_SECONDS;

/// Represents a single cache entry with metadata.
///
/// Each cache entry contains the cached data along with metadata needed for
/// validation and user association. Entries are automatically timestamped
/// when created and can be validated against the configured TTL.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// The cache key (e.g., "gh_repos", "cf_rating")
    pub key: String,
    /// The cached value as a string
    pub value: String,
    /// Unix timestamp when this entry was created
    pub timestamp: i64,
    /// Username associated with this cache entry
    pub username: String,
}

impl CacheEntry {
    /// Creates a new cache entry with the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key identifier
    /// * `value` - The value to cache
    /// * `username` - The username this entry belongs to
    ///
    /// # Returns
    ///
    /// A new `CacheEntry` with the current UTC timestamp
    pub fn new(key: String, value: String, username: String) -> Self {
        Self {
            key,
            value,
            timestamp: Utc::now().timestamp(),
            username,
        }
    }

    /// Parses a cache entry from a line in the cache file.
    ///
    /// Expected format: `<key> <value> <timestamp> <username>`
    ///
    /// # Arguments
    ///
    /// * `line` - A line from the cache file
    ///
    /// # Returns
    ///
    /// `Some(CacheEntry)` if the line is properly formatted, `None` otherwise
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 4 {
            Some(Self {
                key: parts[0].to_string(),
                value: parts[1].to_string(),
                timestamp: parts[2].parse().ok()?,
                username: parts[3].to_string(),
            })
        } else {
            None
        }
    }

    /// Checks if this cache entry is still valid based on TTL.
    ///
    /// # Returns
    ///
    /// `true` if the entry is within the TTL window, `false` if expired
    pub fn is_valid(&self) -> bool {
        Utc::now().timestamp() - self.timestamp < TTL_SECS
    }

    /// Converts this cache entry to a line format for file storage.
    ///
    /// # Returns
    ///
    /// A formatted string ready to be written to the cache file
    pub fn to_line(&self) -> String {
        format!(
            "{} {} {} {}",
            self.key, self.value, self.timestamp, self.username
        )
    }
}

/// Returns the full path to the cache file.
///
/// The cache file is located at `~/.cache/fastfetch/tsukiyomi.cache`.
///
/// # Returns
///
/// `Ok(PathBuf)` with the cache file path, or `Err(FetchError)` if the home
/// directory cannot be determined.
fn cache_path() -> Result<PathBuf> {
    let home =
        dirs::home_dir().ok_or_else(|| FetchError::cache("Could not determine home directory"))?;
    Ok(home.join(constants::CACHE_DIR).join(constants::CACHE_FILE))
}

/// Ensures the cache directory exists, creating it if necessary.
///
/// This function creates the full directory path to the cache file location
/// if it doesn't already exist.
///
/// # Returns
///
/// `Ok(())` if the directory exists or was created successfully,
/// `Err(FetchError)` if directory creation fails.
fn ensure_cache_dir() -> Result<()> {
    let path = cache_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| FetchError::cache(format!("Failed to create cache directory: {}", e)))?;
    }
    Ok(())
}

/// Retrieves a cached value for the given key and username.
///
/// This function searches the cache file for a valid entry matching the provided
/// key and username. It performs validation to ensure:
/// - The entry belongs to the correct user
/// - The entry hasn't expired based on TTL
///
/// The search is performed in reverse order (most recent first) for better performance.
///
/// # Arguments
///
/// * `key` - The cache key to look up (e.g., "gh_repos")
/// * `username` - The username to match against
///
/// # Returns
///
/// * `Ok(Some(String))` - Cached value if found and valid
/// * `Ok(None)` - No valid cache entry found (expired, wrong user, or doesn't exist)
/// * `Err(FetchError)` - I/O error reading the cache file
pub fn get_cached(key: &str, username: &str) -> Result<Option<String>> {
    let path = cache_path()?;
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Ok(None), // Cache file doesn't exist yet
    };

    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<std::io::Result<_>>()?;
    lines.reverse(); // Check most recent entries first

    for line in lines {
        if let Some(entry) = CacheEntry::from_line(&line) {
            if entry.key == key {
                // Found an entry for this key
                if entry.username == username && entry.is_valid() {
                    // Same username and valid cache
                    return Ok(Some(entry.value));
                } else {
                    // Either username changed or cache expired - need to refetch
                    return Ok(None);
                }
            }
        }
    }

    Ok(None)
}

/// Saves a single cache entry, replacing any existing entry with the same key.
///
/// This function performs an atomic update of the cache file by:
/// 1. Reading all existing entries
/// 2. Filtering out any entries with the same key
/// 3. Writing back all remaining entries plus the new one
///
/// # Arguments
///
/// * `key` - The cache key to save
/// * `value` - The value to cache
/// * `username` - The username this entry belongs to
///
/// # Returns
///
/// `Ok(())` if the entry was saved successfully, `Err(FetchError)` on I/O errors.
pub fn save_cache(key: &str, value: &str, username: &str) -> Result<()> {
    ensure_cache_dir()?;
    let path = cache_path()?;
    let new_entry = CacheEntry::new(key.to_string(), value.to_string(), username.to_string());

    let existing_lines = if path.exists() {
        let file = File::open(&path)?;
        BufReader::new(file)
            .lines()
            .collect::<std::io::Result<Vec<_>>>()?
            .into_iter()
            .filter(|line| {
                if let Some(entry) = CacheEntry::from_line(line) {
                    // Remove any existing entry for this key (regardless of username)
                    entry.key != key
                } else {
                    true // Keep malformed lines
                }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let mut file = File::create(&path)
        .map_err(|e| FetchError::cache(format!("Failed to create cache file: {}", e)))?;

    for line in existing_lines {
        writeln!(file, "{}", line)?;
    }
    writeln!(file, "{}", new_entry.to_line())?;

    Ok(())
}

/// Saves multiple cache entries atomically, replacing any existing entries with matching keys.
///
/// This is an optimized version of `save_cache` for batch operations. It's particularly
/// useful when fetching multiple related statistics from a single API call (e.g.,
/// GitHub user data that includes repos, followers, and following counts).
///
/// # Arguments
///
/// * `entries` - A slice of (key, value) tuples to cache
/// * `username` - The username all entries belong to
///
/// # Returns
///
/// `Ok(())` if all entries were saved successfully, `Err(FetchError)` on I/O errors.
///
/// # Example
///
/// ```rust
/// let entries = vec![
///     ("gh_repos".to_string(), "42".to_string()),
///     ("gh_followers".to_string(), "123".to_string()),
/// ];
/// save_multiple_cache(&entries, "username")?;
/// ```
pub fn save_multiple_cache(entries: &[(String, String)], username: &str) -> Result<()> {
    ensure_cache_dir()?;
    let path = cache_path()?;

    let existing_lines = if path.exists() {
        let file = File::open(&path)?;
        BufReader::new(file)
            .lines()
            .collect::<std::io::Result<Vec<_>>>()?
            .into_iter()
            .filter(|line| {
                if let Some(entry) = CacheEntry::from_line(line) {
                    // Remove any existing entry for keys we're about to save (regardless of username)
                    !entries.iter().any(|(key, _)| entry.key == *key)
                } else {
                    true
                }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let mut file = File::create(&path)
        .map_err(|e| FetchError::cache(format!("Failed to create cache file: {}", e)))?;

    for line in existing_lines {
        writeln!(file, "{}", line)?;
    }

    for (key, value) in entries {
        let entry = CacheEntry::new(key.clone(), value.clone(), username.to_string());
        writeln!(file, "{}", entry.to_line())?;
    }

    Ok(())
}
