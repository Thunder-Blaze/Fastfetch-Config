//! # Tsukiyomi-Fetch
//!
//! A fast, multi-platform statistics fetcher written in Rust.
//! 
//! Tsukiyomi-Fetch is a command-line tool that retrieves user statistics from various platforms
//! including GitHub, Codeforces, CodeChef, LeetCode, AniList, and many others. It features
//! intelligent caching, configurable output formatting, and a wrapper mode for integration
//! with system information tools like fastfetch.
//!
//! ## Features
//!
//! - **Multi-platform support**: Fetch statistics from 12+ different platforms
//! - **Intelligent caching**: Automatic caching with configurable TTL to reduce API calls
//! - **Wrapper mode**: Special formatting for integration with system fetch tools
//! - **Configuration management**: Interactive setup and persistent configuration storage
//! - **Error handling**: Comprehensive error types with detailed messages
//! - **Retry logic**: Automatic retry with exponential backoff for network requests
//!
//! ## Supported Platforms
//!
//! - GitHub (repositories, followers, stars, etc.)
//! - Codeforces (rating, max rating)
//! - CodeChef (rating, max rating)
//! - LeetCode (ranking)
//! - AniList (anime/manga statistics)
//! - MyAnimeList (anime/manga totals)
//! - Instagram (followers, following)
//! - Reddit (karma statistics)
//! - Steam (games, hours played)
//! - Twitter (followers)
//! - Discord (status)
//! - Simkl (movies, TV shows, anime)

mod cache;
mod config;
mod constants;
mod error;
mod http;
mod fetchers {
    pub mod anilist;
    pub mod codechef;
    pub mod codeforces;
    pub mod discord;
    pub mod github;
    pub mod instagram;
    pub mod leetcode;
    pub mod myanimelist;
    pub mod reddit;
    pub mod simkl;
    pub mod steam;
    pub mod twitter;
}
mod wrapper;

use dotenv::dotenv;
use error::Result;
use std::env;

/// Displays the help message with usage information, supported platforms, and examples.
///
/// This function prints comprehensive usage instructions including:
/// - Command syntax and options
/// - List of all supported platforms and their available metrics
/// - Wrapper mode options for integration with other tools
/// - Usage examples for common operations
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
    println!("    twitter      Twitter statistics (followers)");
    println!("    discord      Discord statistics (status)");
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

/// Main entry point for the Tsukiyomi-Fetch application.
///
/// This function handles command-line argument parsing and dispatches requests to the
/// appropriate fetcher modules. It supports several modes of operation:
///
/// - **Setup mode**: Interactive configuration (`--setup`)
/// - **Help mode**: Display usage information (`--help`, `-h`)
/// - **Wrapper mode**: Special formatting for fastfetch integration (`wrapper <platform>`)
/// - **Direct fetch mode**: Fetch specific metrics from platforms (`<platform> <metric>`)
///
/// The function loads environment variables, validates arguments, and routes requests
/// to the corresponding platform fetchers. All results are cached automatically to
/// reduce API calls and improve performance.
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or a `FetchError` if any operation fails.
///
/// # Examples
///
/// ```bash
/// # Setup configuration
/// tsukiyomi-fetch --setup
///
/// # Fetch GitHub repositories
/// tsukiyomi-fetch github repos
///
/// # Use wrapper mode with custom styling
/// tsukiyomi-fetch wrapper github --color cyan --icon 
/// ```
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
        "twitter" => fetchers::twitter::fetch(sub)?,
        "discord" => fetchers::discord::fetch(sub)?,
        _ => {
            print_help();
            return Ok(());
        }
    };

    println!("{}", output);
    Ok(())
}
