use crate::{cache, config, http, error::{FetchError, Result}};
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
            let response_text = http::get_with_retry(&url)?;
            let resp: LCUser = serde_json::from_str(&response_text)?;

            if let Some(ranking) = resp.ranking {
                let rank_str = ranking.to_string();
                cache::save_cache(&key, &rank_str, &user)?;
                Ok(rank_str)
            } else {
                Err(FetchError::api("LeetCode", "Could not fetch LeetCode Rank".to_string()))
            }
        }
        _ => Err(FetchError::invalid_param("LeetCode", subparam)),
    }
}
