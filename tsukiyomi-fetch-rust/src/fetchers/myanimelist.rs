use crate::{cache, config, http, error::{FetchError, Result}};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default)]
struct MALStats {
    data: UserStats,
}

#[derive(Deserialize, Default)]
struct UserStats {
    anime: AnimeStats,
    manga: MangaStats,
}

#[derive(Deserialize, Default)]
struct AnimeStats {
    days_watched: f32,
    episodes_watched: u32,
    total_entries: u32,
    completed: u32,
}

#[derive(Deserialize, Default)]
struct MangaStats {
    days_read: f32,
    chapters_read: u32,
    volumes_read: u32,
    total_entries: u32,
    completed: u32,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("MyAnimeList")
        .ok_or_else(|| FetchError::config("Missing MyAnimeList username. Run with --setup"))?;
    let key = format!("mal_{}", subparam);

    if let Ok(Some(cached)) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = http::endpoints::MAL_STATS.url(&[&user]);
    let response_text = http::get_with_retry(&url)?;
    let resp: MALStats = serde_json::from_str(&response_text)?;

    let mut cache_map: HashMap<&str, String> = HashMap::new();

    let resp = resp.data;

    // Anime stats
    cache_map.insert("anime_days", format!("{}", resp.anime.days_watched));
    cache_map.insert("anime_episodes", resp.anime.episodes_watched.to_string());
    cache_map.insert("anime_completed", resp.anime.completed.to_string());
    cache_map.insert("anime_total", resp.anime.total_entries.to_string());

    // Manga stats
    cache_map.insert("manga_days", format!("{}", resp.manga.days_read));
    cache_map.insert("manga_chapters", resp.manga.chapters_read.to_string());
    cache_map.insert("manga_volumes", resp.manga.volumes_read.to_string());
    cache_map.insert("manga_completed", resp.manga.completed.to_string());
    cache_map.insert("manga_total", resp.manga.total_entries.to_string());

    // Save all
    for (k, v) in &cache_map {
        cache::save_cache(&format!("mal_{}", k), v, &user)?;
    }

    // Return only requested
    if let Some(val) = cache_map.get(subparam) {
        Ok(val.clone())
    } else {
        Err(FetchError::invalid_param("MyAnimeList", subparam))
    }
}
