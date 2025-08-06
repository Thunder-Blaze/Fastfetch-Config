use crate::{
    cache, config,
    error::{FetchError, Result},
    http,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct GHUser {
    public_repos: u32,
    followers: u32,
    following: u32,
}

#[derive(Deserialize)]
struct GHStars {
    stars: u32,
    forks: u32,
}

const SUPPORTED_PARAMS: &[&str] = &["repos", "followers", "following", "stars", "forks", "prs"];

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
            let response_text = http::get_with_retry(&url)?;
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
            let response_text = http::get_with_retry(&url)?;
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
            let response_text = http::get_with_retry(&url)?;
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
