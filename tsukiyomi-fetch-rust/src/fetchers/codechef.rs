//! # CodeChef Statistics Fetcher
//!
//! This module handles fetching user statistics from CodeChef by parsing HTML content.
//! CodeChef is a competitive programming platform similar to Codeforces.
//! Since CodeChef doesn't provide a public API, this module uses web scraping
//! with regex patterns to extract rating information from user profile pages.
//!
//! ## Supported Parameters
//!
//! - `rating`: Current contest rating
//! - `maxrating`: Maximum rating ever achieved
//!
//! ## Implementation
//!
//! Uses HTML parsing with regex to extract rating information from the user's
//! profile page. This approach is more fragile than API-based solutions but
//! necessary due to the lack of a public API.

use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use regex::Regex;

/// Extract rating and max rating from CodeChef HTML content using regex patterns
/// 
/// # Arguments
/// * `html` - HTML content from the CodeChef user profile page
/// 
/// # Returns
/// A tuple containing (current_rating, max_rating) as `Option<String>`
fn extract_ratings(html: &str) -> Result<(Option<String>, Option<String>)> {
    let rating_re = Regex::new(r#"<div class="rating-number">(\d+)"#)
        .map_err(|e| FetchError::parse(&format!("Regex error: {}", e)))?;
    let maxrating_re = Regex::new(r#"Highest Rating[^0-9]*?(\d+)"#)
        .map_err(|e| FetchError::parse(&format!("Regex error: {}", e)))?;

    let rating = rating_re
        .captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    let max_rating = maxrating_re
        .captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    Ok((rating, max_rating))
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("CodeChef")
        .ok_or_else(|| FetchError::config("Missing CodeChef username. Run with --setup"))?;
    let key = format!("cc_{}", subparam);

    if let Ok(Some(cached)) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = http::endpoints::CODECHEF_USER.url(&[&user]);
    let response_text = http::get_with_retry(&url, None)?;
    let (rating, maxrating) = extract_ratings(&response_text)?;

    if let Some(ref r) = rating {
        cache::save_cache("cc_rating", r, &user)?;
    }

    if let Some(ref m) = maxrating {
        cache::save_cache("cc_maxrating", m, &user)?;
    }

    let val = match subparam {
        "rating" => {
            rating.ok_or_else(|| FetchError::api("CodeChef", "Rating not found".to_string()))?
        }
        "maxrating" => maxrating
            .ok_or_else(|| FetchError::api("CodeChef", "Max rating not found".to_string()))?,
        _ => return Err(FetchError::invalid_param("CodeChef", subparam)),
    };

    Ok(val)
}
