use crate::{cache, config, http, error::{FetchError, Result}};
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
    let user = config::get_config_value("AniList")
        .ok_or_else(|| FetchError::config("Missing AniList username. Run with --setup"))?;
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

    let response_text = http::post_with_retry(&http::endpoints::ANILIST_GRAPHQL.base_url, &payload.to_string())?;
    let resp: AniListResponse = serde_json::from_str(&response_text)?;

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
        None => Err(FetchError::invalid_param("AniList", subparam)),
    }
}
