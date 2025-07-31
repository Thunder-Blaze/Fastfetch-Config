use crate::{cache, config};
use anyhow::{anyhow, bail, Result};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
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

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("GitHub")
        .ok_or_else(|| anyhow!("Missing GitHub username. Run with --setup"))?;
    let key = format!("gh_{}", subparam);

    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let client = Client::new();

    match subparam {
        "repos" | "followers" | "following" => {
            let resp: GHUser = client
                .get(&format!("https://api.github.com/users/{}", user))
                .header(USER_AGENT, "fastfetch-rs")
                .send()?
                .json()?;

            let (repos, followers, following) = (
                Some(resp.public_repos.to_string()),
                Some(resp.followers.to_string()),
                Some(resp.following.to_string()),
            );

            if let Some(val) = &repos {
                cache::save_cache("gh_repos", val, &user);
            }
            if let Some(val) = &followers {
                cache::save_cache("gh_followers", val, &user);
            }
            if let Some(val) = &following {
                cache::save_cache("gh_following", val, &user);
            }

            let val = match subparam {
                "repos" => repos.unwrap(),
                "followers" => followers.unwrap(),
                "following" => following.unwrap(),
                _ => unreachable!(),
            };

            Ok(val)
        }

        "stars" | "forks" => {
            let resp: GHStars = client
                .get(&format!(
                    "https://api.github-star-counter.workers.dev/user/{}",
                    user
                ))
                .header(USER_AGENT, "fastfetch-rs")
                .send()?
                .json()?;

            let (stars, forks) = (Some(resp.stars.to_string()), Some(resp.forks.to_string()));

            if let Some(val) = &stars {
                cache::save_cache("gh_stars", val, &user);
            }
            if let Some(val) = &forks {
                cache::save_cache("gh_forks", val, &user);
            }

            let val = match subparam {
                "stars" => stars.unwrap(),
                "forks" => forks.unwrap(),
                _ => unreachable!(),
            };

            Ok(val)
        }

        "prs" => {
            let resp: serde_json::Value = client
                .get(&format!(
                    "https://api.github.com/search/issues?q=author:{}+type:pr",
                    user
                ))
                .header(USER_AGENT, "fastfetch-rs")
                .send()?
                .json()?;

            if let Some(prs) = resp["total_count"].as_u64().map(|v| v.to_string()) {
                cache::save_cache("gh_prs", &prs, &user);
                return Ok(prs);
            }

            bail!("Failed to extract PR count");
        }

        _ => bail!("Invalid GitHub subparam"),
    }
}
