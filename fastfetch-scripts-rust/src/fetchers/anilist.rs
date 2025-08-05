use crate::{cache, config};
use anyhow::{Result, anyhow};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct AniListResponse {
    data: AniListData,
}

#[derive(Deserialize)]
struct AniListData {
    #[serde(rename = "User")]
    user: AniListUser,
}

#[derive(Deserialize)]
struct AniListUser {
    statistics: AniStats,
}

#[derive(Deserialize)]
struct AniStats {
    anime: AnimeStats,
    manga: MangaStats,
}

#[derive(Deserialize)]
struct AnimeStats {
    count: u32,
    #[serde(rename = "episodesWatched")]
    episodes_watched: u32,
}

#[derive(Deserialize)]
struct MangaStats {
    count: u32,
    #[serde(rename = "chaptersRead")]
    chapters_read: u32,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("AniList").ok_or_else(|| anyhow!("No AniList username"))?;
    let key = format!("al_{}", subparam);

    if let Ok(Some(cached)) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let query = r#"
        query ($name: String) {
            User(name: $name) {
                statistics {
                    anime {
                        count
                        episodesWatched
                    }
                    manga {
                        count
                        chaptersRead
                    }
                }
            }
        }
    "#;

    let payload = serde_json::json!({
        "query": query,
        "variables": { "name": user }
    });

    let client = Client::new();
    let resp: AniListResponse = client
        .post("https://graphql.anilist.co")
        .json(&payload)
        .send()?
        .json()?;

    let anime_count = resp.data.user.statistics.anime.count.to_string();
    let episodes = resp.data.user.statistics.anime.episodes_watched.to_string();
    let manga_count = resp.data.user.statistics.manga.count.to_string();
    let chapters = resp.data.user.statistics.manga.chapters_read.to_string();

    let mut cache_map: HashMap<&str, String> = HashMap::new();
    cache_map.insert("anime_count", anime_count.clone());
    cache_map.insert("episodes", episodes.clone());
    cache_map.insert("manga_count", manga_count.clone());
    cache_map.insert("chapters", chapters.clone());

    for (k, v) in &cache_map {
        cache::save_cache(&format!("al_{}", k), v, &user)?;
    }

    match cache_map.get(subparam) {
        Some(val) => Ok(val.clone()),
        None => Err(anyhow!("Unknown AniList subparam: {subparam}")),
    }
}
