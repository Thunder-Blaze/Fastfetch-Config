use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::{thread, time::Duration};

use crate::constants::{APP_USER_AGENT, HTTP_TIMEOUT_SECONDS, MAX_RETRIES};
use crate::error::{FetchError, Result};

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(APP_USER_AGENT));

    Client::builder()
        .timeout(Duration::from_secs(HTTP_TIMEOUT_SECONDS))
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client")
});

#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    pub base_url: &'static str,
    pub path_template: &'static str,
}

impl ApiEndpoint {
    pub fn url(&self, params: &[&str]) -> String {
        let mut url = format!("{}{}", self.base_url, self.path_template);
        for (i, param) in params.iter().enumerate() {
            url = url.replace(&format!("{{{}}}", i), param);
        }
        url
    }
}

pub mod endpoints {
    use super::ApiEndpoint;

    pub const GITHUB_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.github.com",
        path_template: "/users/{0}",
    };

    pub const GITHUB_STARS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.github-star-counter.workers.dev",
        path_template: "/user/{0}",
    };

    pub const GITHUB_PRS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.github.com",
        path_template: "/search/issues?q=author:{0}+type:pr",
    };

    pub const CODEFORCES_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://codeforces.com",
        path_template: "/api/user.info?handles={0}",
    };

    pub const CODECHEF_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://www.codechef.com",
        path_template: "/users/{0}",
    };

    pub const LEETCODE_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://leetcode-stats-api.herokuapp.com",
        path_template: "/{0}",
    };

    pub const ANILIST_GRAPHQL: ApiEndpoint = ApiEndpoint {
        base_url: "https://graphql.anilist.co",
        path_template: "",
    };

    pub const SIMKL_STATS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.simkl.com",
        path_template: "/users/{0}/stats",
    };

    pub const MAL_STATS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.jikan.moe",
        path_template: "/v4/users/{0}/statistics",
    };

    pub const INSTAGRAM_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://i.instagram.com",
        path_template: "/api/v1/users/web_profile_info/?username={0}",
    };

    pub const REDDIT_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://www.reddit.com",
        path_template: "/user/{0}/about.json",
    };

    pub const STEAM_STATS: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.steampowered.com",
        path_template:
            "/IPlayerService/GetOwnedGames/v0001/?key={0}&steamid={1}&format=json&include_appinfo=1",
    };

    pub const TWITTER_ID: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.twitter.com",
        path_template: "/2/users/by/username/{0}",
    };

    pub const TWITTER_USER: ApiEndpoint = ApiEndpoint {
        base_url: "https://api.twitter.com",
        path_template: "/2/users/{0}?user.fields=public_metrics",
    };

    pub const DISCORD_STATUS: ApiEndpoint = ApiEndpoint {
        base_url: "https://tsukiyomi-bot.onrender.com",
        path_template: "/presences/{0}",
    };
}

/// Perform HTTP GET request with retry logic and exponential backoff
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
