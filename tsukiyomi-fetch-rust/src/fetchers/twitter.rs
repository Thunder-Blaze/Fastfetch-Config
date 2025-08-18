//! # Twitter Statistics Fetcher
//!
//! This module handles fetching user statistics from Twitter using the Twitter API v2.
//! It retrieves follower counts and other public metrics from user profiles.
//!
//! ## Supported Parameters
//!
//! - `followers`: Number of followers
//!
//! ## Configuration
//!
//! Requires a Twitter API Bearer token to be configured due to Twitter's
//! API access restrictions. The token can be obtained from the Twitter Developer Portal.
//!
//! ## Implementation
//!
//! Uses Twitter API v2 endpoints with bearer token authentication to retrieve
//! user metrics and public profile information.

use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TwitterUserResponse {
    data: TwitterUser,
}

#[derive(Debug, Deserialize)]
struct TwitterUser {
    id: String,
}

#[derive(Debug, Deserialize)]
struct TwitterUserStatsResponse {
    data: TwitterUserStats,
}

#[derive(Debug, Deserialize)]
struct TwitterUserStats {
    public_metrics: PublicMetrics,
}

#[derive(Debug, Deserialize)]
struct PublicMetrics {
    followers_count: u64,
}

const SUPPORTED_PARAMS: &[&str] = &["followers"];

pub fn fetch(subparam: &str) -> Result<String> {
    if !SUPPORTED_PARAMS.contains(&subparam) {
        return Err(FetchError::invalid_param("Twitter", subparam));
    }

    let username = config::get_config_value("Twitter")
        .ok_or_else(|| FetchError::config("Missing Twitter username. Run with --setup"))?;

    let bearer_token = match config::get_token("Twitter", true, true) {
        Ok(token) => token.unwrap(),
        Err(e) => Err(e)?,
    };

    let cache_key = format!("twitter_{}", subparam);
    if let Ok(Some(cached)) = cache::get_cached(&cache_key, &username) {
        return Ok(cached);
    }

    // Get user ID
    let user_url = http::endpoints::TWITTER_ID.url(&[&username]);
    let user_response_text = http::get_with_retry(&user_url, Some(&bearer_token))?;
    let user_response: TwitterUserResponse = serde_json::from_str(&user_response_text)?;

    let user_id = user_response.data.id;

    // Get followers count
    let stats_url = http::endpoints::TWITTER_USER.url(&[&user_id]);
    let stats_response_text = http::get_with_retry(&stats_url, Some(&bearer_token))?;
    let stats_response: TwitterUserStatsResponse = serde_json::from_str(&stats_response_text)?;

    let followers_count = stats_response.data.public_metrics.followers_count;

    // Cache the result
    let value = followers_count.to_string();
    let entries = vec![(cache_key, value.clone())];
    cache::save_multiple_cache(&entries, &username)?;

    Ok(value)
}
