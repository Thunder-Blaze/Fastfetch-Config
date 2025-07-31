use crate::{cache, config};
use anyhow::{anyhow, bail, Result};
use reqwest::blocking::Client;
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
        .ok_or_else(|| anyhow!("Missing Instagram username. Run with --setup"))?;

    let key = format!("ig_{}", subparam);
    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = format!(
        "https://i.instagram.com/api/v1/users/web_profile_info/?username={}",
        user
    );

    let client = Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .header("x-ig-app-id", "936619743392459")
        .send()?
        .error_for_status()?
        .json::<InstagramData>()?;

    let followers = resp.data.user.edge_followed_by.count.to_string();
    let following = resp.data.user.edge_follow.count.to_string();

    cache::save_cache("ig_followers", &followers, &user);
    cache::save_cache("ig_following", &following, &user);

    let val = match subparam {
        "followers" => followers,
        "following" => following,
        _ => bail!("Invalid subparam for Instagram"),
    };

    Ok(val)
}
