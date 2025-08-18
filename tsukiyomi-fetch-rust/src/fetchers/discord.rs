//! # Discord Status Fetcher
//!
//! This module handles fetching Discord user presence and activity information.
//! It connects to a custom Discord bot service to retrieve real-time user status,
//! activities, and presence information.
//!
//! ## Supported Parameters
//!
//! - `status`: Current online status (online, idle, dnd, offline)
//! - `activity`: Current activity or game being played
//! - `listening`: Current music/media being listened to
//! - `watching`: Current media being watched
//! - `custom`: Custom status message
//!
//! ## Implementation
//!
//! Uses a custom Discord bot service hosted at tsukiyomi-bot.onrender.com
//! to retrieve presence information via Discord's gateway API.

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
    let response_text = http::get_with_retry(&url, None)?;

    if response_text.is_empty() {
        return Err(FetchError::api(
            "Discord",
            "No status found for the given ID".to_string(),
        ));
    }

    let value = match subparam {
        "status" => response_text,
        _ => unreachable!(), // Already validated above
    };

    Ok(value)
}
