use crate::{cache, config};
use anyhow::{Result, anyhow, bail};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct LCUser {
    ranking: Option<u64>,
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("LeetCode")
        .ok_or_else(|| anyhow!("Missing LeetCode username. Run with --setup"))?;

    let key = format!("lc_{}", subparam);
    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    match subparam {
        "rank" => {
            let client = Client::new();
            let resp = client
                .get(&format!(
                    "https://leetcode-stats-api.herokuapp.com/{}",
                    user
                ))
                .send()?
                .json::<LCUser>()?;

            if let Some(ranking) = resp.ranking {
                let rank_str = ranking.to_string();
                cache::save_cache(&key, &rank_str, &user);
                Ok(rank_str)
            } else {
                bail!("Could not fetch LeetCode Rank");
            }
        }
        _ => bail!("Invalid LeetCode subparam"),
    }
}
