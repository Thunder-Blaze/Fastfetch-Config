use crate::fetchers::{
    anilist, codechef, codeforces, github, instagram, leetcode, myanimelist, simkl,
};
use anyhow::{anyhow, Result};

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

    let icon = |i: usize, default: &str| {
        icons.get(i).cloned().unwrap_or_else(|| default.to_string())
    };
    let color = |s: &str| format!("\x1b[0;{}m{}\x1b[0m", color_code, s);

    match platform {
        "codeforces" => {
            if let (Ok(rating), Ok(max)) =
                (codeforces::fetch("rating"), codeforces::fetch("maxrating"))
            {
                Ok(format!(
                    "{} {} {}",
                    rating,
                    color(&icon(0, "")),
                    max
                ))
            } else {
                Ok(String::new())
            }
        }

        "codechef" => {
            if let (Ok(rating), Ok(max)) = (codechef::fetch("rating"), codechef::fetch("maxrating"))
            {
                Ok(format!(
                    "{} {} {}",
                    rating,
                    color(&icon(0, "")),
                    max
                ))
            } else {
                Ok(String::new())
            }
        }

        "github" => {
            if let (Ok(repos), Ok(prs), Ok(stars), Ok(followers)) = (
                github::fetch("repos"),
                github::fetch("prs"),
                github::fetch("stars"),
                github::fetch("followers"),
            ) {
                Ok(format!(
                    "{} {} {} {} {} {} {} {}",
                    color(&icon(0, "")),
                    repos,
                    color(&icon(1, "")),
                    prs,
                    color(&icon(2, "")),
                    stars,
                    color(&icon(3, "")),
                    followers
                ))
            } else {
                Ok(String::new())
            }
        }

        "anilist" => {
            if let (Ok(anime), Ok(manga), Ok(episodes), Ok(chapters)) = (
                anilist::fetch("anime_count"),
                anilist::fetch("manga_count"),
                anilist::fetch("episodes"),
                anilist::fetch("chapters"),
            ) {
                Ok(format!(
                    "{} {} {} {} {} {} {} {}",
                    color(&icon(0, "")),
                    anime,
                    color(&icon(1, "")),
                    episodes,
                    color(&icon(2, "󰂺")),
                    manga,
                    color(&icon(3, "")),
                    chapters
                ))
            } else {
                Ok(String::new())
            }
        }

        "simkl" => {
            if let (Ok(movies), Ok(hours)) = (
                simkl::fetch("movies", "completed"),
                simkl::fetch("movies", "hours"),
            ) {
                Ok(format!(
                    "{} {} {} {}h",
                    color(&icon(0, "")),
                    movies,
                    color(&icon(1, "")),
                    hours
                ))
            } else {
                Ok(String::new())
            }
        }

        "myanimelist" => {
            if let (Ok(anime), Ok(manga), Ok(episodes), Ok(chapters)) = (
                myanimelist::fetch("anime_total"),
                myanimelist::fetch("manga_total"),
                myanimelist::fetch("anime_episodes"),
                myanimelist::fetch("manga_chapters"),
            ) {
                Ok(format!(
                    "{} {} {} {} {} {} {} {}",
                    color(&icon(0, "")),
                    anime,
                    color(&icon(1, "")),
                    episodes,
                    color(&icon(2, "󰂺")),
                    manga,
                    color(&icon(3, "")),
                    chapters
                ))
            } else {
                Ok(String::new())
            }
        }

        "leetcode" => {
            if let Ok(rank) = leetcode::fetch("rank") {
                Ok(format!("{} {}", color(&icon(0, "󰆥")), rank))
            } else {
                Ok(String::new())
            }
        }

        "instagram" => {
            if let (Ok(followers), Ok(following)) =
                (instagram::fetch("followers"), instagram::fetch("following"))
            {
                Ok(format!(
                    "{} {} {} {}",
                    color(&icon(0, "")),
                    followers,
                    color(&icon(1, "")),
                    following
                ))
            } else {
                Ok(String::new())
            }
        }

        _ => Err(anyhow!("Unknown platform: {platform}")),
    }
}
