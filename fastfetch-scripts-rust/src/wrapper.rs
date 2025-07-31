use crate::fetchers::{
    anilist, codechef, codeforces, github, instagram, leetcode, myanimelist, simkl,
};
use anyhow::{Result, anyhow};
use std::result::Result::Ok;

pub fn run_wrapper(platform: &str) -> Result<String> {
    match platform {
        "codeforces" => {
            if let (Ok(rating), Ok(max)) =
                (codeforces::fetch("rating"), codeforces::fetch("maxrating"))
            {
                Ok(format!("{rating} \u{001b}[0;36m\u{001b}[0m {max}"))
            } else {
                Ok(String::new())
            }
        }

        "codechef" => {
            if let (Ok(rating), Ok(max)) = (codechef::fetch("rating"), codechef::fetch("maxrating"))
            {
                Ok(format!("{rating} \u{001b}[0;36m\u{001b}[0m {max}"))
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
                    "\u{001b}[0;36m\u{001b}[0m {repos} \
\u{001b}[0;36m\u{001b}[0m {prs} \
\u{001b}[0;36m\u{001b}[0m {stars} \
\u{001b}[0;36m\u{001b}[0m {followers}"
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
                    "\u{001b}[0;36m\u{001b}[0m  {anime} \
\u{001b}[0;36m\u{001b}[0m {episodes} \
\u{001b}[0;36m󰂺\u{001b}[0m {manga} \
\u{001b}[0;36m\u{001b}[0m {chapters}"
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
                    "\u{001b}[0;36m\u{001b}[0m {movies} \
\u{001b}[0;36m\u{001b}[0m {hours}h"
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
                    "\u{001b}[0;36m\u{001b}[0m {anime} \
\u{001b}[0;36m\u{001b}[0m {episodes} \
\u{001b}[0;36m󰂺\u{001b}[0m {manga} \
\u{001b}[0;36m\u{001b}[0m {chapters}"
                ))
            } else {
                Ok(String::new())
            }
        }

        "leetcode" => {
            if let Ok(rank) = leetcode::fetch("rank") {
                Ok(format!("\u{001b}[0;36m󰆥\u{001b}[0m {rank}"))
            } else {
                Ok(String::new())
            }
        }

        "instagram" => {
            if let (Ok(followers), Ok(following)) =
                (instagram::fetch("followers"), instagram::fetch("following"))
            {
                Ok(format!(
                    "\u{001b}[0;36m\u{001b}[0m {followers} \
\u{001b}[0;36m\u{001b}[0m  {following}"
                ))
            } else {
                Ok(String::new())
            }
        }

        _ => Err(anyhow!("Unknown platform: {platform}")),
    }
}
