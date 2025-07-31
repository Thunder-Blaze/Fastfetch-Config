mod config;
mod cache;
mod fetchers {
    pub mod github;
    pub mod codeforces;
    pub mod codechef;
    pub mod leetcode;
    pub mod simkl;
    pub mod anilist;
    pub mod myanimelist;
    pub mod instagram;
}
mod wrapper;

use anyhow::Result;
use std::env;

fn print_help() {
    println!("Usage: fastfetch <platform> <subparam> [subsub]");
}

fn main() -> Result<()> {
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
    if args.len() < 3 { print_help(); return Ok(()); }
    if args.len() == 3 && args[1] == "wrapper" {
        let output = wrapper::run_wrapper(&args[2])?;
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
        },
        "anilist" => fetchers::anilist::fetch(sub)?,
        "myanimelist" => fetchers::myanimelist::fetch(sub)?,
        "instagram" => fetchers::instagram::fetch(sub)?,
        _ => { print_help(); return Ok(()); }
    };

    println!("{}", output);
    Ok(())
}
