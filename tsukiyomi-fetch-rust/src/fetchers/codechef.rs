use crate::{cache, config, http, error::{FetchError, Result}};
use regex::Regex;

/// Extract rating and max rating from CodeChef HTML.
fn extract_ratings(html: &str) -> Result<(Option<String>, Option<String>)> {
    let rating_re = Regex::new(r#"<div class="rating-number">(\d+)"#)
        .map_err(|e| FetchError::parse(&format!("Regex error: {}", e)))?;
    let maxrating_re = Regex::new(r#"Highest Rating[^0-9]*?(\d+)"#)
        .map_err(|e| FetchError::parse(&format!("Regex error: {}", e)))?;

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
        .ok_or_else(|| FetchError::config("Missing CodeChef username. Run with --setup"))?;
    let key = format!("cc_{}", subparam);

    if let Ok(Some(cached)) = cache::get_cached(&key, &user) {
        return Ok(cached);
    }

    let url = http::endpoints::CODECHEF_USER.url(&[&user]);
    let resp = http::get_with_retry(&url)?;
    let (rating, maxrating) = extract_ratings(&resp)?;

    if let Some(ref r) = rating {
        cache::save_cache("cc_rating", r, &user)?;
    }

    if let Some(ref m) = maxrating {
        cache::save_cache("cc_maxrating", m, &user)?;
    }

    let val = match subparam {
        "rating" => rating.ok_or_else(|| FetchError::api("CodeChef", "Rating not found".to_string()))?,
        "maxrating" => maxrating.ok_or_else(|| FetchError::api("CodeChef", "Max rating not found".to_string()))?,
        _ => return Err(FetchError::invalid_param("CodeChef", subparam)),
    };

    Ok(val)
}
