use crate::{cache, config};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use reqwest::blocking::Client;
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
    let user = config::get_config_value("MyAnimeList").ok_or_else(|| anyhow!("No MAL username"))?;
    let key = format!("mal_{}", subparam);

    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let client = Client::new();
    let resp: MALStats = client
        .get(&format!("https://api.jikan.moe/v4/users/{}/statistics", user))
        .send()?
        .json()?;

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
        cache::save_cache(&format!("mal_{}", k), v, &user);
    }

    // Return only requested
    if let Some(val) = cache_map.get(subparam) {
        Ok(val.clone())
    } else {
        Err(anyhow!("Unknown MAL subparam: {subparam}"))
    }
}