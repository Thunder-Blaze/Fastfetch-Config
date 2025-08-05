use crate::{cache, config, http, error::{FetchError, Result}};
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct MediaStats {
    #[serde(rename = "total_mins")]
    total_mins: Option<u32>,
    completed: Option<CompletedCount>,
}

#[derive(Deserialize, Default)]
struct CompletedCount {
    count: Option<u32>,
}

#[derive(Deserialize, Default)]
struct SimklStats {
    anime: Option<MediaStats>,
    tv: Option<MediaStats>,
    movies: Option<MediaStats>,
    #[serde(rename = "total_mins")]
    total_mins: Option<u32>,
}

pub fn fetch(media_type: &str, stat_type: &str) -> Result<String> {
    let user = config::get_config_value("Simkl")
        .ok_or_else(|| FetchError::config("Missing Simkl user ID. Run with --setup"))?;

    let cache_key = format!("sk_{}_{}", media_type, stat_type);

    // If *any* stat is cached, assume all are
    if cache::get_cached("sk_anime_hours", &user).ok().flatten().is_some() {
        return cache::get_cached(&cache_key, &user)?
            .ok_or_else(|| FetchError::cache("Cached value not found for Simkl"));
    }

    // Fetch all stats in one go
    let url = http::endpoints::SIMKL_STATS.url(&[&user]);
    let response_text = http::get_with_retry(&url)?;
    let resp: SimklStats = serde_json::from_str(&response_text)?;

    let extract_stats = |media: Option<MediaStats>| -> (String, String) {
        let media = media.unwrap_or_default();

        let hours = media.total_mins.map(|m| m / 60).unwrap_or(0).to_string();

        let completed = media
            .completed
            .unwrap_or_default()
            .count
            .unwrap_or(0)
            .to_string();

        (hours, completed)
    };

    let (anime_hours, anime_completed) = extract_stats(resp.anime);
    let (tv_hours, tv_completed) = extract_stats(resp.tv);
    let (movies_hours, movies_completed) = extract_stats(resp.movies);
    let total_hours = resp.total_mins.unwrap_or(0) / 60;

    // Save all values to cache
    let cache_entries = vec![
        ("sk_anime_hours".to_string(), anime_hours),
        ("sk_anime_completed".to_string(), anime_completed),
        ("sk_tv_hours".to_string(), tv_hours),
        ("sk_tv_completed".to_string(), tv_completed),
        ("sk_movies_hours".to_string(), movies_hours),
        ("sk_movies_completed".to_string(), movies_completed),
        ("sk_totalhours".to_string(), total_hours.to_string()),
    ];
    
    cache::save_multiple_cache(&cache_entries, &user)?;

    // Retrieve only the one user asked for
    cache::get_cached(&cache_key, &user)?
        .ok_or_else(|| FetchError::cache(&format!("Missing Simkl stat '{}'", cache_key)))
}
