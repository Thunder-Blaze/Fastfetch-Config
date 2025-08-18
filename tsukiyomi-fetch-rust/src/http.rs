//! # HTTP Client and API Endpoint Management
//!
//! This module provides a centralized HTTP client and API endpoint definitions
//! for making requests to various platform APIs. It includes retry logic,
//! timeout handling, and structured endpoint management.
//!
//! ## Features
//!
//! - **Global HTTP client**: Singleton client with proper headers and timeouts
//! - **Retry mechanism**: Automatic retry with exponential backoff for failed requests
//! - **Endpoint templates**: Structured API endpoint definitions with parameter substitution
//! - **Error handling**: Comprehensive error handling for network operations
//! - **Rate limiting**: Built-in delays to respect API rate limits
//!
//! ## Design
//!
//! The module uses a lazy-initialized global HTTP client to avoid repeatedly
//! creating connections. API endpoints are defined as static constants with
//! template-based URL construction for dynamic parameters.

use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::{thread, time::Duration};

use crate::constants::{APP_USER_AGENT, HTTP_TIMEOUT_SECONDS, MAX_RETRIES};
use crate::error::{FetchError, Result};

/// Global HTTP client instance with default configuration
/// 
/// This client is initialized once and reused across all HTTP requests.
/// It includes proper user agent headers and timeout settings.
pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(APP_USER_AGENT));

    Client::builder()
        .timeout(Duration::from_secs(HTTP_TIMEOUT_SECONDS))
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client")
});

/// API endpoint definition with template-based URL construction
/// 
/// This struct represents an API endpoint with a base URL and a path template
/// that supports parameter substitution using `{0}`, `{1}`, etc. placeholders.
#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    /// Base URL of the API (e.g., `https://api.github.com`)
    pub base_url: &'static str,
    /// Path template with parameter placeholders (e.g., "/users/{0}")
    pub path_template: &'static str,
}

impl ApiEndpoint {
    /// Construct a complete URL by substituting parameters into the template
    /// 
    /// # Arguments
    /// * `params` - Array of string parameters to substitute into the template
    /// 
    /// # Returns
    /// Complete URL with all parameters substituted
    /// 
    /// # Example
    /// ```
    /// let endpoint = ApiEndpoint {
    ///     base_url: "https://api.github.com",
    ///     path_template: "/users/{0}/repos",
    /// };
    /// let url = endpoint.url(&["octocat"]);
    /// assert_eq!(url, "https://api.github.com/users/octocat/repos");
    /// ```
    pub fn url(&self, params: &[&str]) -> String {
        let mut url = format!("{}{}", self.base_url, self.path_template);
        for (i, param) in params.iter().enumerate() {
            url = url.replace(&format!("{{{}}}", i), param);
        }
        url
    }
}

/// API endpoint definitions for all supported platforms
/// 
/// This module contains static endpoint definitions for all platforms
/// that Tsukiyomi-Fetch can retrieve data from.
pub mod endpoints {
    use super::ApiEndpoint;

    /// GitHub user profile API endpoint
    pub const GITHUB_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.github.com",
        path_template: "/users/{0}",
    };

    /// GitHub star count API endpoint (using external service)
    pub const GITHUB_STARS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.github-star-counter.workers.dev",
        path_template: "/user/{0}",
    };

    /// GitHub pull requests API endpoint
    pub const GITHUB_PRS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.github.com",
        path_template: "/search/issues?q=author:{0}+type:pr",
    };

    /// Codeforces user profile API endpoint
    pub const CODEFORCES_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://codeforces.com",
        path_template: "/api/user.info?handles={0}",
    };

    /// CodeChef user profile API endpoint
    pub const CODECHEF_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://www.codechef.com",
        path_template: "/users/{0}",
    };

    /// LeetCode user statistics API endpoint (external service)
    pub const LEETCODE_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://leetcode-stats-api.herokuapp.com",
        path_template: "/{0}",
    };

    /// AniList GraphQL API endpoint
    pub const ANILIST_GRAPHQL: ApiEndpoint = ApiEndpoint {
        base_url: "https://graphql.anilist.co",
        path_template: "",
    };

    /// Simkl user statistics API endpoint
    pub const SIMKL_STATS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.simkl.com",
        path_template: "/users/{0}/stats",
    };

    /// MyAnimeList user statistics API endpoint (via Jikan)
    pub const MAL_STATS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.jikan.moe",
        path_template: "/v4/users/{0}/statistics",
    };

    /// Instagram user profile API endpoint
    pub const INSTAGRAM_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://i.instagram.com",
        path_template: "/api/v1/users/web_profile_info/?username={0}",
    };

    /// Reddit user profile API endpoint
    pub const REDDIT_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://www.reddit.com",
        path_template: "/user/{0}/about.json",
    };

    /// Steam user games library API endpoint
    pub const STEAM_STATS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.steampowered.com",
        path_template: "/IPlayerService/GetOwnedGames/v0001/?key={0}&steamid={1}&format=json&include_appinfo=1",
    };

    /// Twitter user ID lookup API endpoint
    pub const TWITTER_ID: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.twitter.com",
        path_template: "/2/users/by/username/{0}",
    };

    /// Twitter user profile API endpoint
    pub const TWITTER_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.twitter.com",
        path_template: "/2/users/{0}?user.fields=public_metrics",
    };

    /// Discord user status API endpoint (custom service)
    pub const DISCORD_STATUS: ApiEndpoint = ApiEndpoint {
        base_url: "https://tsukiyomi-bot.onrender.com",
        path_template: "/presences/{0}",
    };
}

/// Perform HTTP GET request with retry logic and exponential backoff
/// 
/// This function automatically retries failed requests up to `MAX_RETRIES` times
/// with exponential backoff. It handles both network errors and HTTP error status codes.
/// 
/// # Arguments
/// * `url` - The URL to make the GET request to
/// * `auth_token` - Optional bearer token for authentication
/// 
/// # Returns
/// * `Ok(String)` - Response body text on success
/// * `Err(FetchError)` - Network error, API error, or other failure
/// 
/// # Retry Logic
/// - Starts with 100ms delay between retries
/// - Doubles the delay on each retry (exponential backoff)
/// - Caps maximum delay at 5 seconds
/// - Retries on network errors and non-success HTTP status codes
pub fn get_with_retry(url: &str, auth_token: Option<&String>) -> Result<String> {
    let mut retry_count = 0;
    let mut delay = Duration::from_millis(100); // Start with 100ms delay

    loop {
        let mut request = HTTP_CLIENT.get(url);

        if let Some(token) = &auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        match request.send() {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(text) => return Ok(text),
                        Err(e) => {
                            if retry_count >= MAX_RETRIES {
                                return Err(FetchError::Network(e));
                            }
                        }
                    }
                } else {
                    let status = response.status();
                    if retry_count >= MAX_RETRIES {
                        return Err(FetchError::Api {
                            platform: "HTTP".to_string(),
                            message: format!("HTTP {} after {} retries", status, MAX_RETRIES),
                        });
                    }
                }
            }
            Err(e) => {
                if retry_count >= MAX_RETRIES {
                    return Err(FetchError::Network(e));
                }
            }
        }

        // Exponential backoff with jitter
        thread::sleep(delay);
        delay = Duration::from_millis((delay.as_millis() as u64 * 2).min(5000)); // Cap at 5 seconds
        retry_count += 1;
    }
}

/// Perform HTTP POST request with retry logic and exponential backoff
/// 
/// This function automatically retries failed POST requests up to `MAX_RETRIES` times
/// with exponential backoff. It's specifically designed for JSON API requests.
/// 
/// # Arguments
/// * `url` - The URL to make the POST request to
/// * `body` - JSON request body as a string
/// * `auth_token` - Optional bearer token for authentication
/// 
/// # Returns
/// * `Ok(String)` - Response body text on success
/// * `Err(FetchError)` - Network error, API error, or other failure
/// 
/// # Headers
/// - Sets `Content-Type: application/json`
/// - Adds `Authorization: Bearer <token>` if auth_token is provided
/// 
/// # Retry Logic
/// - Same exponential backoff strategy as `get_with_retry`
/// - Retries on network errors and non-success HTTP status codes
pub fn post_with_retry(url: &str, body: &str, auth_token: &Option<String>) -> Result<String> {
    let mut retry_count = 0;
    let mut delay = Duration::from_millis(100);

    loop {
        let mut request = HTTP_CLIENT.post(url).body(body.to_string());

        if let Some(token) = &auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        match request.header("Content-Type", "application/json").send() {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(text) => return Ok(text),
                        Err(e) => {
                            if retry_count >= MAX_RETRIES {
                                return Err(FetchError::Network(e));
                            }
                        }
                    }
                } else {
                    let status = response.status();
                    if retry_count >= MAX_RETRIES {
                        return Err(FetchError::Api {
                            platform: "HTTP".to_string(),
                            message: format!("HTTP {} after {} retries", status, MAX_RETRIES),
                        });
                    }
                }
            }
            Err(e) => {
                if retry_count >= MAX_RETRIES {
                    return Err(FetchError::Network(e));
                }
            }
        }

        thread::sleep(delay);
        delay = Duration::from_millis((delay.as_millis() as u64 * 2).min(5000));
        retry_count += 1;
    }
}
