use crate::{cache, config};
use anyhow::{anyhow, bail, Result};
use regex::Regex;

/// Extract rating and max rating from CodeChef HTML.
fn extract_ratings(html: &str) -> Result<(Option<String>, Option<String>)> {
    let rating_re = Regex::new(r#"<div class="rating-number">(\d+)"#)?;
    let maxrating_re = Regex::new(r#"Highest Rating[^0-9]*?(\d+)"#)?;

    let rating = rating_re
        .captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    let max_rating = maxrating_re
        .captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    Ok((rating, max_rating))
}

pub fn fetch(subparam: &str) -> Result<String> {
    let user = config::get_config_value("CodeChef")
        .ok_or_else(|| anyhow!("Missing CodeChef username. Run with --setup"))?;
    let key = format!("cc_{}", subparam);

    if let Some(cached) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = format!("https://www.codechef.com/users/{}", user);
    let resp = reqwest::blocking::get(&url)?.text()?;
    let (rating, maxrating) = extract_ratings(&resp)?;

    if let Some(ref r) = rating {
        cache::save_cache("cc_rating", r, &user);
    }

    if let Some(ref m) = maxrating {
        cache::save_cache("cc_maxrating", m, &user);
    }

    let val = match subparam {
        "rating" => rating.ok_or_else(|| anyhow!("Rating not found"))?,
        "maxrating" => maxrating.ok_or_else(|| anyhow!("Max rating not found"))?,
        _ => bail!("Invalid subparam for CodeChef"),
    };

    Ok(val)
}
