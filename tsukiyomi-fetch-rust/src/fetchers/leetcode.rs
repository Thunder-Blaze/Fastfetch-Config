//! # LeetCode Statistics Fetcher
//!
//! This module handles fetching user statistics from LeetCode using an external API service.
//! LeetCode is a platform for practicing coding interview questions and algorithmic problems.
//!
//! ## Supported Parameters
//!
//! - `ranking`: User's global ranking on LeetCode
//!
//! ## Implementation
//!
//! Uses the leetcode-stats-api.herokuapp.com service to retrieve user statistics
//! since LeetCode's official API requires authentication and has usage restrictions.

use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct LCUser {
    ranking: Option<u64>,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("LeetCode")
        .ok_or_else(|| FetchError::config("Missing LeetCode username. Run with --setup"))?;

    let key = format!("lc_{}", subparam);
    if let Ok(Some(cached)) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    match subparam {
        "rank" => {
            let url = http::endpoints::LEETCODE_USER.url(&[&user]);
            let response_text = http::get_with_retry(&url, None)?;
            let resp: LCUser = serde_json::from_str(&response_text)?;

            if let Some(ranking) = resp.ranking {
                let rank_str = ranking.to_string();
                cache::save_cache(&key, &rank_str, &user)?;
                Ok(rank_str)
            } else {
                Err(FetchError::api(
                    "LeetCode",
                    "Could not fetch LeetCode Rank".to_string(),
                ))
            }
        }
        _ => Err(FetchError::invalid_param("LeetCode", subparam)),
    }
}
