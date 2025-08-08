use crate::fetchers::{
    anilist, codechef, codeforces, github, instagram, leetcode, myanimelist, reddit, simkl,
};
use crate::{
    constants,
    error::{FetchError, Result},
};

// Helper function to safely fetch values, returning empty string on any error
fn safe_fetch<T>(result: crate::error::Result<T>) -> Option<T> {
    result.ok()
}

/// Parse color name to ANSI color code
fn parse_color_code(color: &str) -> &str {
    match color.to_lowercase().as_str() {
        "black" => "30",
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        _ => "36", // default to cyan
    }
}

/// Extract --color and --icon values from args
fn parse_args(args: &[String]) -> (String, Vec<String>) {
    let mut color = String::from("36"); // default to cyan
    let mut icons = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--color" => {
                if let Some(c) = args.get(i + 1) {
                    color = parse_color_code(c).to_string();
                    i += 1;
                }
            }
            "--icon" => {
                if let Some(icon) = args.get(i + 1) {
                    icons.push(icon.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    (color, icons)
}

pub fn run_wrapper(platform: &str, args: &[String]) -> Result<String> {
    let (color_code, icons) = parse_args(args);
    let platform_defaults = constants::get_platform_icons(platform);

    let icon = |i: usize, fallback: &str| {
        icons
            .get(i)
            .cloned()
            .or_else(|| platform_defaults.get(i).map(|s| s.to_string()))
            .unwrap_or_else(|| fallback.to_string())
    };
    let color = |s: &str| format!("\x1b[0;{}m{}\x1b[0m", color_code, s);

    match platform {
        "codeforces" => {
            match (
                safe_fetch(codeforces::fetch("rating")),
                safe_fetch(codeforces::fetch("maxrating")),
            ) {
                (Some(rating), Some(max)) => {
                    Ok(format!("{} {} {}", rating, color(&icon(0, "")), max))
                }
                _ => Ok(String::new()), // Silent failure on any error
            }
        }

        "codechef" => {
            match (
                safe_fetch(codechef::fetch("rating")),
                safe_fetch(codechef::fetch("maxrating")),
            ) {
                (Some(rating), Some(max)) => {
                    Ok(format!("{} {} {}", rating, color(&icon(0, "")), max))
                }
                _ => Ok(String::new()),
            }
        }

        "github" => {
            match (
                safe_fetch(github::fetch("repos")),
                safe_fetch(github::fetch("prs")),
                safe_fetch(github::fetch("stars")),
                safe_fetch(github::fetch("followers")),
            ) {
                (Some(repos), Some(prs), Some(stars), Some(followers)) => Ok(format!(
                    "{} {} {} {} {} {} {} {}",
                    color(&icon(0, "")),
                    repos,
                    color(&icon(1, "")),
                    prs,
                    color(&icon(2, "")),
                    stars,
                    color(&icon(3, "")),
                    followers
                )),
                _ => Ok(String::new()),
            }
        }

        "anilist" => {
            match (
                safe_fetch(anilist::fetch("anime_count")),
                safe_fetch(anilist::fetch("manga_count")),
                safe_fetch(anilist::fetch("episodes")),
                safe_fetch(anilist::fetch("chapters")),
            ) {
                (Some(anime), Some(manga), Some(episodes), Some(chapters)) => Ok(format!(
                    "{} {} {} {} {} {} {} {}",
                    color(&icon(0, "")),
                    anime,
                    color(&icon(1, "")),
                    episodes,
                    color(&icon(2, "󰂺")),
                    manga,
                    color(&icon(3, "")),
                    chapters
                )),
                _ => Ok(String::new()),
            }
        }

        "simkl" => {
            match (
                safe_fetch(simkl::fetch("movies", "completed")),
                safe_fetch(simkl::fetch("movies", "hours")),
            ) {
                (Some(movies), Some(hours)) => Ok(format!(
                    "{} {} {} {}h",
                    color(&icon(0, "")),
                    movies,
                    color(&icon(1, "")),
                    hours
                )),
                _ => Ok(String::new()),
            }
        }

        "myanimelist" => {
            match (
                safe_fetch(myanimelist::fetch("anime_total")),
                safe_fetch(myanimelist::fetch("manga_total")),
                safe_fetch(myanimelist::fetch("anime_episodes")),
                safe_fetch(myanimelist::fetch("manga_chapters")),
            ) {
                (Some(anime), Some(manga), Some(episodes), Some(chapters)) => Ok(format!(
                    "{} {} {} {} {} {} {} {}",
                    color(&icon(0, "")),
                    anime,
                    color(&icon(1, "")),
                    episodes,
                    color(&icon(2, "󰂺")),
                    manga,
                    color(&icon(3, "")),
                    chapters
                )),
                _ => Ok(String::new()),
            }
        }

        "leetcode" => match safe_fetch(leetcode::fetch("rank")) {
            Some(rank) => Ok(format!("{} {}", color(&icon(0, "󰆥")), rank)),
            _ => Ok(String::new()),
        },

        "instagram" => {
            match (
                safe_fetch(instagram::fetch("followers")),
                safe_fetch(instagram::fetch("following")),
            ) {
                (Some(followers), Some(following)) => Ok(format!(
                    "{} {} {} {}",
                    color(&icon(0, "")),
                    followers,
                    color(&icon(1, "")),
                    following
                )),
                _ => Ok(String::new()),
            }
        }

        "reddit" => {
            match (
                safe_fetch(reddit::fetch("link_karma")),
                safe_fetch(reddit::fetch("comment_karma")),
            ) {
                (Some(link_karma), Some(comment_karma)) => Ok(format!(
                    "{} {} {} {}",
                    color(&icon(0, "")),
                    link_karma,
                    color(&icon(1, "")),
                    comment_karma,
                )),
                _ => Ok(String::new()),
            }
        }

        _ => Err(FetchError::platform_not_found(platform)),
    }
}
