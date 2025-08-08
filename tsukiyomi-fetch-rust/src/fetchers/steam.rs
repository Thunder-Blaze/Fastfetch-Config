use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Game {
    playtime_forever: u64,
}

#[derive(Debug, Deserialize)]
struct SteamResponse {
    response: GamesList,
}

#[derive(Debug, Deserialize)]
struct GamesList {
    games: Vec<Game>,
}

const SUPPORTED_PARAMS: &[&str] = &["total_games", "total_hours"];

pub fn fetch(subparam: &str) -> Result<String> {
    if !SUPPORTED_PARAMS.contains(&subparam) {
        return Err(FetchError::invalid_param("Steam", subparam));
    }

    let steam_id = config::get_config_value("Steam")
        .ok_or_else(|| FetchError::config("Missing Steam ID. Run with --setup"))?;

    let api_key = match config::get_token("Steam", true, true) {
        Ok(api_key) => api_key.unwrap(),
        Err(e) => Err(e)?,
    };

    let cache_key = format!("steam_{}", subparam);

    if let Ok(Some(cached)) = cache::get_cached(&cache_key, &steam_id) {
        return Ok(cached);
    }

    let url = http::endpoints::STEAM_STATS.url(&[&api_key, &steam_id]);
    let response_text = http::get_with_retry(&url, None)?;
    let response: SteamResponse = serde_json::from_str(&response_text)?;

    let games = response.response.games;

    let total_games = games.len();
    let total_hours: f64 =
        games.iter().map(|g| g.playtime_forever as u64).sum::<u64>() as f64 / 60.0;

    // Cache both values
    let entries = vec![
        ("steam_total_games".to_string(), total_games.to_string()),
        (
            "steam_total_hours".to_string(),
            format!("{:.2}", total_hours),
        ),
    ];

    cache::save_multiple_cache(&entries, &steam_id)?;

    let value = match subparam {
        "total_games" => total_games.to_string(),
        "total_hours" => format!("{:.2}", total_hours),
        _ => unreachable!(), // Already validated above
    };

    Ok(value)
}
