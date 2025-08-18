//! # Codeforces Statistics Fetcher
//!
//! This module handles fetching user statistics from the Codeforces API.
//! Codeforces is a competitive programming platform where users participate
//! in contests and solve algorithmic problems.
//!
//! ## Supported Parameters
//!
//! - `rating`: Current contest rating of the user
//! - `maxrating`: Maximum rating ever achieved by the user
//!
//! ## Configuration
//!
//! Requires a Codeforces handle (username) to be set in the configuration file.
//! The Codeforces API is public and doesn't require authentication for basic
//! user information.
//!
//! ## API Endpoint
//!
//! Uses the official Codeforces API: `https://codeforces.com/api/user.info?handles={username}`
//!
//! ## Rating System
//!
//! Codeforces uses a rating system similar to chess ratings:
//! - Newbie: 0-1199 (gray)
//! - Pupil: 1200-1399 (green)
//! - Specialist: 1400-1599 (cyan)
//! - Expert: 1600-1899 (blue)
//! - Candidate Master: 1900-2099 (violet)
//! - Master: 2100-2299 (orange)
//! - International Master: 2300-2399 (orange)
//! - Grandmaster: 2400-2999 (red)
//! - International Grandmaster: 3000+ (red)

use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

/// Codeforces API response wrapper
#[derive(Deserialize)]
struct CFApi {
    /// API response status ("OK" or "FAILED")
    status: String,
    /// Array of user objects (should contain exactly one user)
    result: Vec<CFUser>,
}

/// Codeforces user information structure
#[derive(Deserialize)]
struct CFUser {
    /// Current contest rating (None if user hasn't participated in rated contests)
    rating: Option<i32>,
    /// Maximum rating ever achieved (None if user hasn't participated in rated contests)
    #[serde(rename = "maxRating")]
    max_rating: Option<i32>,
}

/// List of supported Codeforces statistics parameters
const SUPPORTED_PARAMS: &[&str] = &["rating", "maxrating"];

/// Fetch Codeforces user statistics for the specified parameter
/// 
/// This function retrieves Codeforces contest ratings using the official API.
/// It handles cases where users haven't participated in rated contests yet.
/// 
/// # Arguments
/// * `subparam` - The specific statistic to fetch (rating, maxrating)
/// 
/// # Returns
/// * `Ok(String)` - The requested rating value as a string, or "Unrated" if not available
/// * `Err(FetchError)` - Configuration error, network error, or unsupported parameter
/// 
/// # Errors
/// * `InvalidParam` - If the requested parameter is not supported
/// * `Config` - If Codeforces handle is not configured
/// * `Network` - If API requests fail
/// * `Api` - If Codeforces API returns an error response
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
    let response_text = http::get_with_retry(&url, None)?;
    let response: CFApi = serde_json::from_str(&response_text)?;

    if response.status != "OK" {
        return Err(FetchError::api("Codeforces", "API returned non-OK status"));
    }

    let user_info = response
        .result
        .first()
        .ok_or_else(|| FetchError::api("Codeforces", "No user data in response"))?;

    // Cache both values when we fetch them
    let entries = vec![
        (
            "cf_rating".to_string(),
            user_info.rating.unwrap_or(0).to_string(),
        ),
        (
            "cf_maxrating".to_string(),
            user_info.max_rating.unwrap_or(0).to_string(),
        ),
    ];

    cache::save_multiple_cache(&entries, &username)?;

    let value = match subparam {
        "rating" => user_info.rating.unwrap_or(0).to_string(),
        "maxrating" => user_info.max_rating.unwrap_or(0).to_string(),
        _ => unreachable!(), // Already validated above
    };

    Ok(value)
}
