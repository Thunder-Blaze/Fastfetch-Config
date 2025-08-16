use crate::{
    config,
    error::{FetchError, Result},
    http,
};

const SUPPORTED_PARAMS: &[&str] = &["status"];

pub fn fetch(subparam: &str) -> Result<String> {
    if !SUPPORTED_PARAMS.contains(&subparam) {
        return Err(FetchError::invalid_param("Discord", subparam));
    }

    let id = config::get_config_value("Discord")
        .ok_or_else(|| FetchError::config("Missing Discord ID. Run with --setup"))?;

    let url = http::endpoints::DISCORD_STATUS.url(&[&id]);
    println!("Fetching Discord status for ID: {}, {}", id, url);
    let response_text = http::get_with_retry(&url, None)?;

    if response_text.is_empty() {
        return Err(FetchError::api("Discord", "No status found for the given ID".to_string()));
    }

    let value = match subparam {
        "status" => response_text,
        _ => unreachable!(), // Already validated above
    };

    Ok(value)
}
