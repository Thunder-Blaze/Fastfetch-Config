use crate::{config, cache};
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct SimklStats {
    anime: SimklSection,
    tv: SimklSection,
    movies: SimklSection,
}

#[derive(Deserialize)]
struct SimklSection {
    hours: u32,
    completed: u32,
}

#[derive(Deserialize)]
struct SimklResponse {
    stats: SimklStats,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let key = format!("simkl_{}", subparam);
    let user = config::get_config_value("Simkl").ok_or_else(|| anyhow!("No Simkl username"))?;
    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = format!("https://api.simkl.com/users/{}/stats", user);
    let resp: SimklStats = reqwest::blocking::get(&url)?.json()?;

    let val = match subparam {
        "anime_hours" => resp.anime.hours.to_string(),
        "anime_completed" => resp.anime.completed.to_string(),
        "tv_hours" => resp.tv.hours.to_string(),
        "tv_completed" => resp.tv.completed.to_string(),
        "movie_hours" => resp.movies.hours.to_string(),
        "movie_completed" => resp.movies.completed.to_string(),
        _ => anyhow::bail!("bad subparam"),
    };

    cache::save_cache(&key, &val, &user);
    Ok(val)
}

