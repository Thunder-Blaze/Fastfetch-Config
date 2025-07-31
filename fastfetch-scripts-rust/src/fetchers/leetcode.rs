use crate::{cache, config};
use anyhow::{anyhow, bail, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct LCUser {
    rating: Option<f64>,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("LeetCode")
        .ok_or_else(|| anyhow!("Missing LeetCode username. Run with --setup"))?;
    print!("{}", user);

    let key = format!("lc_{}", subparam);
    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    match subparam {
        "rating" => {
            let client = Client::new();
            let resp = client
                .get(&format!(
                    "https://leetcode-stats-api.herokuapp.com/{}",
                    user
                ))
                .send()?
                .json::<LCUser>()?;

            if let Some(rating) = resp.rating {
                let rating_str = (rating.round() as i32).to_string();
                cache::save_cache(&key, &rating_str, &user);
                Ok(rating_str)
            } else {
                bail!("Could not fetch LeetCode rating");
            }
        }
        _ => bail!("Invalid LeetCode subparam"),
    }
}
