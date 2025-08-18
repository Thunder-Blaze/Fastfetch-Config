//! # GitHub Statistics Fetcher
//!
//! This module handles fetching user statistics from the GitHub API.
//! It retrieves various metrics including repository counts, follower/following counts,
//! total stars received, and pull request counts.
//!
//! ## Supported Parameters
//!
//! - `repos`: Number of public repositories
//! - `followers`: Number of followers
//! - `following`: Number of users being followed
//! - `stars`: Total stars received across all repositories
//! - `forks`: Total forks received across all repositories
//! - `prs`: Number of pull requests created by the user
//!
//! ## Configuration
//!
//! Requires a GitHub username to be set in the configuration file.
//! The GitHub API has rate limits for unauthenticated requests,
//! but basic user information is typically available without authentication.
//!
//! ## API Endpoints
//!
//! - User profile: `https://api.github.com/users/{username}`
//! - Star count: `https://api.github-star-counter.workers.dev/user/{username}`
//! - Pull requests: `https://api.github.com/search/issues?q=author:{username}+type:pr`

use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

/// GitHub user profile response structure
#[derive(Deserialize)]
struct GHUser {
    /// Number of public repositories
    public_repos: u32,
    /// Number of followers
    followers: u32,
    /// Number of users being followed
    following: u32,
}

/// GitHub star count response structure (from external service)
#[derive(Deserialize)]
struct GHStars {
    /// Total stars received across all repositories
    stars: u32,
    /// Total forks received across all repositories
    forks: u32,
}

/// List of supported GitHub statistics parameters
const SUPPORTED_PARAMS: &[&str] = &["repos", "followers", "following", "stars", "forks", "prs"];

/// Fetch GitHub user statistics for the specified parameter
/// 
/// This function retrieves various GitHub statistics based on the requested parameter.
/// It uses intelligent caching to avoid hitting API rate limits and provides
/// comprehensive error handling for network and API issues.
/// 
/// # Arguments
/// * `subparam` - The specific statistic to fetch (repos, followers, following, stars, forks, prs)
/// 
/// # Returns
/// * `Ok(String)` - The requested statistic value as a string
/// * `Err(FetchError)` - Configuration error, network error, or unsupported parameter
/// 
/// # Errors
/// * `InvalidParam` - If the requested parameter is not supported
/// * `Config` - If GitHub username is not configured
/// * `Network` - If API requests fail
/// * `Api` - If GitHub API returns an error response
pub fn fetch(subparam: &str) -> Result<String> {
    if !SUPPORTED_PARAMS.contains(&subparam) {
        return Err(FetchError::invalid_param("GitHub", subparam));
    }

    let username = config::get_config_value("GitHub")
        .ok_or_else(|| FetchError::config("Missing GitHub username. Run with --setup"))?;

    let cache_key = format!("gh_{}", subparam);

    if let Ok(Some(cached)) = cache::get_cached(&cache_key, &username) {
        return Ok(cached);
    }

    match subparam {
        "repos" | "followers" | "following" => {
            let url = http::endpoints::GITHUB_USER.url(&[&username]);
            let response_text = http::get_with_retry(&url, None)?;
            let resp: GHUser = serde_json::from_str(&response_text)?;

            let data = vec![
                ("gh_repos".to_string(), resp.public_repos.to_string()),
                ("gh_followers".to_string(), resp.followers.to_string()),
                ("gh_following".to_string(), resp.following.to_string()),
            ];

            cache::save_multiple_cache(&data, &username)?;

            let value = match subparam {
                "repos" => resp.public_repos.to_string(),
                "followers" => resp.followers.to_string(),
                "following" => resp.following.to_string(),
                _ => unreachable!(),
            };

            Ok(value)
        }

        "stars" | "forks" => {
            let url = http::endpoints::GITHUB_STARS.url(&[&username]);
            let response_text = http::get_with_retry(&url, None)?;
            let resp: GHStars = serde_json::from_str(&response_text)?;

            let data = vec![
                ("gh_stars".to_string(), resp.stars.to_string()),
                ("gh_forks".to_string(), resp.forks.to_string()),
            ];

            cache::save_multiple_cache(&data, &username)?;

            let value = match subparam {
                "stars" => resp.stars.to_string(),
                "forks" => resp.forks.to_string(),
                _ => unreachable!(),
            };

            Ok(value)
        }

        "prs" => {
            let url = http::endpoints::GITHUB_PRS.url(&[&username]);
            let response_text = http::get_with_retry(&url, None)?;
            let resp: serde_json::Value = serde_json::from_str(&response_text)?;

            let prs = resp["total_count"]
                .as_u64()
                .ok_or_else(|| FetchError::parse("Failed to extract PR count"))?
                .to_string();

            cache::save_cache("gh_prs", &prs, &username)?;
            Ok(prs)
        }

        _ => unreachable!(), // Already validated above
    }
}
