mod cache;
mod config;
mod constants;
mod error;
mod http;
mod fetchers {
    pub mod anilist;
    pub mod codechef;
    pub mod codeforces;
    pub mod github;
    pub mod instagram;
    pub mod leetcode;
    pub mod myanimelist;
    pub mod reddit;
    pub mod simkl;
    pub mod steam;
}
mod wrapper;

use dotenv::dotenv;
use error::Result;
use std::env;

fn print_help() {
    println!("tsukiyomi-fetch - A fast statistics fetcher for various platforms");
    println!();
    println!("USAGE:");
    println!("    tsukiyomi-fetch <platform> <metric> [options]");
    println!("    tsukiyomi-fetch wrapper <platform> [--color <color>] [--icon <icon>...]");
    println!("    tsukiyomi-fetch --setup");
    println!("    tsukiyomi-fetch --help");
    println!();
    println!("PLATFORMS:");
    println!("    github       GitHub statistics (repos, followers, following, stars, forks, prs)");
    println!("    codeforces   Codeforces ratings (rating, maxrating)");
    println!("    codechef     CodeChef ratings (rating, maxrating)");
    println!("    leetcode     LeetCode statistics (rank)");
    println!("    anilist      AniList statistics (anime_count, manga_count, episodes, chapters)");
    println!("    simkl        Simkl statistics (movies, tv, anime with completed/hours)");
    println!("    myanimelist  MyAnimeList statistics (anime_total, manga_total, etc.)");
    println!("    instagram    Instagram statistics (followers, following)");
    println!("    reddit       Reddit statistics (link karma, comment karma)");
    println!("    steam        Steam statistics (total_games, total_hours)");
    println!();
    println!("OPTIONS:");
    println!("    --setup      Interactive configuration setup");
    println!("    --help, -h   Show this help message");
    println!();
    println!("WRAPPER OPTIONS:");
    println!("    --color      Set color (black, red, green, yellow, blue, magenta, cyan, white)");
    println!("    --icon       Set custom icons (can be used multiple times)");
    println!();
    println!("EXAMPLES:");
    println!("    tsukiyomi-fetch github repos");
    println!("    tsukiyomi-fetch wrapper github --color cyan --icon  --icon ");
    println!("    tsukiyomi-fetch --setup");
}

fn main() -> Result<()> {
    dotenv().ok();
    let args: Vec<_> = env::args().collect();
    if let Some(a) = args.get(1) {
        if a == "--help" || a == "-h" {
            print_help();
            return Ok(());
        }
        if a == "--setup" {
            config::setup()?;
            println!("Configuration setup complete. Please run again without --setup.");
            return Ok(());
        }
    }
    if args.len() < 3 {
        print_help();
        return Ok(());
    }
    if args.len() >= 3 && args[1] == "wrapper" {
        let Some(platform) = args.get(2) else {
            print_help();
            return Ok(());
        };

        let extra_args = args[3..].to_vec(); // collect remaining args
        let output = wrapper::run_wrapper(platform, &extra_args)?;
        println!("{output}");
        return Ok(());
    }
    let platform = &args[1];
    let sub = &args[2];
    let sub2 = args.get(3).map(|s| s.as_str());

    let output = match platform.as_str() {
        "github" => fetchers::github::fetch(sub)?,
        "codeforces" => fetchers::codeforces::fetch(sub)?,
        "codechef" => fetchers::codechef::fetch(sub)?,
        "leetcode" => fetchers::leetcode::fetch(sub)?,
        "simkl" => {
            let stat_type = sub2.unwrap_or("completed");
            fetchers::simkl::fetch(sub, &stat_type)?
        }
        "anilist" => fetchers::anilist::fetch(sub)?,
        "myanimelist" => fetchers::myanimelist::fetch(sub)?,
        "instagram" => fetchers::instagram::fetch(sub)?,
        "reddit" => fetchers::reddit::fetch(sub)?,
        "steam" => fetchers::steam::fetch(sub)?,
        _ => {
            print_help();
            return Ok(());
        }
    };

    println!("{}", output);
    Ok(())
}
