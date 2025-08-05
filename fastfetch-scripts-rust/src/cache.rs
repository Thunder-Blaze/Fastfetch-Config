use chrono::Utc;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

const TTL_SECS: i64 = 60 * 60 * 24; // 1 day

macro_rules! define_cache_enum {
    (
        $( $variant:ident => $key:expr ),*
        $(,)?
    ) => {
        #[derive(Debug)]
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

fn cache_path() -> PathBuf {
    dirs::home_dir().unwrap().join(".cache/fastfetch/tsukiyomi.cache")
}

pub fn get_cached(key: &str, username: &str) -> Option<String> {
    let file = File::open(cache_path()).ok()?;
    let reader = BufReader::new(file);
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

pub fn save_cache(key: &str, value: &str, username: &str) {
    let path = cache_path();
    let timestamp = Utc::now().timestamp();
    let new_line = format!("{} {} {} {}", key, value, timestamp, username);

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

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    let mut file = File::create(&path).expect("Failed to open cache file for writing");
    for line in lines {
        writeln!(file, "{}", line).expect("Failed to write to cache file");
    }
    writeln!(file, "{}", new_line).expect("Failed to write new cache entry");
}
