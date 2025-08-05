use chrono::Utc;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};
use crate::{constants, error::{FetchError, Result}};

const TTL_SECS: i64 = constants::CACHE_TTL_SECONDS;

#[derive(Debug, Clone)]
pub struct CacheKey {
    pub platform: &'static str,
    pub metric: &'static str,
}

impl CacheKey {
    pub fn new(platform: &'static str, metric: &'static str) -> Self {
        Self { platform, metric }
    }
    
    pub fn as_string(&self) -> String {
        format!("{}_{}", self.platform, self.metric)
    }
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub key: String,
    pub value: String,
    pub timestamp: i64,
    pub username: String,
}

impl CacheEntry {
    pub fn new(key: String, value: String, username: String) -> Self {
        Self {
            key,
            value,
            timestamp: Utc::now().timestamp(),
            username,
        }
    }
    
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
    
    pub fn is_valid(&self) -> bool {
        Utc::now().timestamp() - self.timestamp < TTL_SECS
    }
    
    pub fn to_line(&self) -> String {
        format!("{} {} {} {}", self.key, self.value, self.timestamp, self.username)
    }
}

macro_rules! define_cache_enum {
    (
        $( $variant:ident => $key:expr ),*
        $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum SaveCache {
            $(
                $variant(String),
            )*
        }

        impl SaveCache {
            pub fn key(&self) -> &'static str {
                match self {
                    $(
                        SaveCache::$variant(_) => $key,
                    )*
                }
            }

            pub fn value(&self) -> &str {
                match self {
                    $(
                        SaveCache::$variant(v) => v,
                    )*
                }
            }

            pub fn from_parts(key: &str, value: &str) -> Option<Self> {
                let val = value.to_string();
                match key {
                    $(
                        $key => Some(SaveCache::$variant(val)),
                    )*
                    _ => None,
                }
            }
        }
    };
}

define_cache_enum! {
    CfRating => "CfRating",
    CfMaxRating => "CfMaxRating",
    CcRating => "CcRating",
    CcMaxRating => "CcMaxRating",
    LeetCodeRating => "LeetCodeRating",
    LeetCodeRank => "LeetCodeRank",
    GitHubRepos => "GitHubRepos",
    GitHubFollowers => "GitHubFollowers",
    GitHubFollowing => "GitHubFollowing",
    GitHubPRs => "GitHubPRs",
    GitHubStars => "GitHubStars",
    GitHubForks => "GitHubForks",
    AniListAnimeCount => "AniListAnimeCount",
    AniListEpisodes => "AniListEpisodes",
    AniListMangaCount => "AniListMangaCount",
    AniListChapters => "AniListChapters",
    SimklTotalHours => "SimklTotalHours",
    SimklMoviesCompleted => "SimklMoviesCompleted",
    SimklMoviesHours => "SimklMoviesHours",
    SimklAnimeCompleted => "SimklAnimeCompleted",
    SimklAnimeHours => "SimklAnimeHours",
    SimklTVCompleted => "SimklTVCompleted",
    SimklTVHours => "SimklTVHours",
    MALAnimeCount => "MALAnimeCount",
    MALAniEpisodes => "MALAniEpisodes",
    MALMangaTotal => "MALMangaTotal",
    MALMangaChapters => "MALMangaChapters",
    InstagramFollowers => "InstagramFollowers",
    InstagramFollowing => "InstagramFollowing",
}

fn cache_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| FetchError::cache("Could not determine home directory"))?;
    Ok(home.join(constants::CACHE_DIR).join(constants::CACHE_FILE))
}

fn ensure_cache_dir() -> Result<()> {
    let path = cache_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| FetchError::cache(format!("Failed to create cache directory: {}", e)))?;
    }
    Ok(())
}

pub fn get_cached(key: &str, username: &str) -> Result<Option<String>> {
    let path = cache_path()?;
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Ok(None), // Cache file doesn't exist yet
    };
    
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<std::io::Result<_>>()?;
    lines.reverse();

    for line in lines {
        if let Some(entry) = CacheEntry::from_line(&line) {
            if entry.key == key && entry.username == username {
                if entry.is_valid() {
                    return Ok(Some(entry.value));
                } else {
                    break; // Found expired entry, stop looking
                }
            }
        }
    }
    
    Ok(None)
}

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
                    !(entry.key == key && entry.username == username)
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
                    !entries.iter().any(|(key, _)| entry.key == *key && entry.username == username)
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
