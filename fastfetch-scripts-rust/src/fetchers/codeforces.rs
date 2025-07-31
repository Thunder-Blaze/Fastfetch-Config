use crate::{cache, config};
use anyhow::{anyhow, bail, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct CFApi {
    status: String,
    result: Vec<CFUser>,
}

#[derive(Deserialize)]
struct CFUser {
    rating: Option<i32>,
    #[serde(rename = "maxRating")]
    max_rating: Option<i32>,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("Codeforces")
        .ok_or_else(|| anyhow!("Missing Codeforces username. Run with --setup"))?;
    let key = format!("cf_{}", subparam);

    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = format!("https://codeforces.com/api/user.info?handles={}", user);
    let response: CFApi = reqwest::blocking::get(&url)?.json()?;
    if response.status != "OK" {
        bail!("Codeforces API returned non-OK status");
    }

    let user_info = &response.result[0];

    // Cache only if values exist
    if let Some(rating) = user_info.rating {
        cache::save_cache("cf_rating", &rating.to_string(), &user);
    }

    if let Some(max_rating) = user_info.max_rating {
        cache::save_cache("cf_maxrating", &max_rating.to_string(), &user);
    }

    match subparam {
        "rating" => Ok(user_info.rating.unwrap_or(0).to_string()),
        "maxrating" => Ok(user_info.max_rating.unwrap_or(0).to_string()),
        _ => bail!("Invalid subparam for Codeforces"),
    }
}