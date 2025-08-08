use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct RedditApi {
    data: Option<RedditUser>,
}

#[derive(Deserialize)]
struct RedditUser {
    link_karma: u64,
    comment_karma: u64,
}

const SUPPORTED_PARAMS: &[&str] = &["link_karma", "comment_karma"];

pub fn fetch(subparam: &str) -> Result<String> {
    if !SUPPORTED_PARAMS.contains(&subparam) {
        return Err(FetchError::invalid_param("Reddit", subparam));
    }

    let username = config::get_config_value("Reddit")
        .ok_or_else(|| FetchError::config("Missing Reddit username. Run with --setup"))?;

    let cache_key = format!("reddit_{}", subparam);
    if let Ok(Some(cached)) = cache::get_cached(&cache_key, &username) {
        return Ok(cached);
    }

    let url = http::endpoints::REDDIT_USER.url(&[&username]);
    let response_text = http::get_with_retry(&url, None)?;
    let response: RedditApi = serde_json::from_str(&response_text)
        .map_err(|e| FetchError::parse(&format!("Failed to parse JSON: {}", e)))?;

    let data: RedditUser = match response.data {
        Some(data) => data,
        None => return Err(FetchError::api("Reddit", "No user data found")),
    };

    let link_karma = data.link_karma.to_string();

    let comment_karma = data.comment_karma.to_string();

    let entries = vec![
        ("reddit_link_karma".to_string(), link_karma.clone()),
        ("reddit_comment_karma".to_string(), comment_karma.clone()),
    ];

    cache::save_multiple_cache(&entries, &username)?;

    let value = match subparam {
        "link_karma" => link_karma,
        "comment_karma" => comment_karma,
        _ => unreachable!(),
    };

    Ok(value)
}
