use crate::{cache, config, http, error::{FetchError, Result}};
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

const SUPPORTED_PARAMS: &[&str] = &["rating", "maxrating"];

pub fn fetch(subparam: &str) -> Result<String> {
    if !SUPPORTED_PARAMS.contains(&subparam) {
        return Err(FetchError::invalid_param("Codeforces", subparam));
    }

    let username = config::get_config_value("Codeforces")
        .ok_or_else(|| FetchError::config("Missing Codeforces handle. Run with --setup"))?;
    
    let cache_key = format!("cf_{}", subparam);

    if let Ok(Some(cached)) = cache::get_cached(&cache_key, &username) {
        return Ok(cached);
    }

    let url = http::endpoints::CODEFORCES_USER.url(&[&username]);
    let response: CFApi = http::HTTP_CLIENT
        .get(&url)
        .send()?
        .error_for_status()
        .map_err(|e| FetchError::api("Codeforces", e.to_string()))?
        .json()?;

    if response.status != "OK" {
        return Err(FetchError::api("Codeforces", "API returned non-OK status"));
    }

    let user_info = response.result.first()
        .ok_or_else(|| FetchError::api("Codeforces", "No user data in response"))?;

    // Cache both values when we fetch them
    let entries = vec![
        ("cf_rating".to_string(), user_info.rating.unwrap_or(0).to_string()),
        ("cf_maxrating".to_string(), user_info.max_rating.unwrap_or(0).to_string()),
    ];
    
    cache::save_multiple_cache(&entries, &username)?;

    let value = match subparam {
        "rating" => user_info.rating.unwrap_or(0).to_string(),
        "maxrating" => user_info.max_rating.unwrap_or(0).to_string(),
        _ => unreachable!(), // Already validated above
    };

    Ok(value)
}
