//! # Instagram Statistics Fetcher
//!
//! This module handles fetching user statistics from Instagram by parsing
//! the web profile information. It retrieves follower and following counts
//! from publicly available profile data.
//!
//! ## Supported Parameters
//!
//! - `followers`: Number of followers
//! - `following`: Number of accounts being followed
//!
//! ## Implementation
//!
//! Uses Instagram's web profile API endpoint to retrieve basic user statistics.
//! This approach works for public profiles without requiring authentication.

use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct InstagramData {
    data: UserData,
}

#[derive(Deserialize)]
struct UserData {
    user: InstagramUser,
}

#[derive(Deserialize)]
struct InstagramUser {
    edge_followed_by: CountWrapper, // followers
    edge_follow: CountWrapper,      // following
}

#[derive(Deserialize)]
struct CountWrapper {
    count: u32,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("Instagram")
        .ok_or_else(|| FetchError::config("Missing Instagram username. Run with --setup"))?;

    let key = format!("ig_{}", subparam);
    if let Ok(Some(cached)) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = http::endpoints::INSTAGRAM_USER.url(&[&user]);

    // Instagram requires specific headers
    let response_text = http::HTTP_CLIENT
        .get(&url)
        .header("x-ig-app-id", "936619743392459")
        .send()?
        .error_for_status()
        .map_err(|e| FetchError::api("Instagram", e.to_string()))?
        .text()?;

    let resp: InstagramData = serde_json::from_str(&response_text)?;

    let followers = resp.data.user.edge_followed_by.count.to_string();
    let following = resp.data.user.edge_follow.count.to_string();

    cache::save_cache("ig_followers", &followers, &user)?;
    cache::save_cache("ig_following", &following, &user)?;

    let val = match subparam {
        "followers" => followers,
        "following" => following,
        _ => return Err(FetchError::invalid_param("Instagram", subparam)),
    };

    Ok(val)
}
