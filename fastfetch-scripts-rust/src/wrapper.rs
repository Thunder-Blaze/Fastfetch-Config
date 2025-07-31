use crate::fetchers::{anilist, codechef, codeforces, github, simkl, myanimelist, leetcode};
use anyhow::{anyhow, Ok, Result};

pub fn run_wrapper(platform: &str) -> Result<String> {
    match platform {
        "codeforces" => {
            let rating = codeforces::fetch("rating")?;
            let max = codeforces::fetch("maxrating")?;
            Ok(format!("{rating} \u{001b}[0;36m\u{001b}[0m {max}"))
        }

        "codechef" => {
            let rating = codechef::fetch("rating")?;
            let max = codechef::fetch("maxrating")?;
            Ok(format!("{rating} \u{001b}[0;36m\u{001b}[0m {max}"))
        }

        "github" => {
            let repos = github::fetch("repos")?;
            let prs = github::fetch("prs")?;
            let stars = github::fetch("stars")?;
            let followers = github::fetch("followers")?;
            Ok(format!(
                "\u{001b}[0;36m\u{001b}[0m {repos} \
\u{001b}[0;36m\u{001b}[0m {prs} \
\u{001b}[0;36m\u{001b}[0m {stars} \
\u{001b}[0;36m\u{001b}[0m {followers}"
            ))
        }

        "anilist" => {
            let anime = anilist::fetch("anime_count")?;
            let manga = anilist::fetch("manga_count")?;
            let episodes = anilist::fetch("episodes")?;
            let chapters = anilist::fetch("chapters")?;
            Ok(format!(
                "\u{001b}[0;36m\u{001b}[0m {anime} \
\u{001b}[0;36m\u{001b}[0m {episodes} \
\u{001b}[0;36m󰂺\u{001b}[0m {manga} \
\u{001b}[0;36m\u{001b}[0m {chapters}"
            ))
        }

        "simkl" => {
            let movies = simkl::fetch("movies", "completed")?;
            let hours = simkl::fetch("movies", "hours")?;
            Ok(format!(
                "\u{001b}[0;36m\u{001b}[0m {movies} \
\u{001b}[0;36m\u{001b}[0m {hours}h"
            ))
        }

        "myanimelist" => {
            let anime = myanimelist::fetch("anime_total")?;
            let manga = myanimelist::fetch("manga_total")?;
            let episodes = myanimelist::fetch("anime_episodes")?;
            let chapters = myanimelist::fetch("manga_chapters")?;
            Ok(format!(
                "\u{001b}[0;36m\u{001b}[0m {anime} \
\u{001b}[0;36m\u{001b}[0m {episodes} \
\u{001b}[0;36m󰂺\u{001b}[0m {manga} \
\u{001b}[0;36m\u{001b}[0m {chapters}"
            ))
        }

        "leetcode" => {
            let rank = leetcode::fetch("rank")?;
            Ok(format!(
                "\u{001b}[0;36m󰆥\u{001b}[0m {rank}"
            ))
        }

        "instagram" => {
            let followers = crate::fetchers::instagram::fetch("followers")?;
            let following = crate::fetchers::instagram::fetch("following")?;
            Ok(format!(
                "\u{001b}[0;36m\u{001b}[0m {followers} \
\u{001b}[0;36m\u{001b}[0m  {following}"
            ))
        }

        _ => Err(anyhow!("Unknown platform: {platform}")),
    }
}
