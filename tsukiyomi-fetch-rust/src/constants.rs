//! # Application Constants
//!
//! This file ontains all the constant values used throughout Tsukiyomi-Fetch.
//! These constants define configuration paths, HTTP settings, cache behavior,
//! and platform-specific icons for the wrapper mode.
//!
//! ## Organization
//!
//! Constants are organized into logical groups:
//! - **Application metadata**: User agent strings and version information
//! - **Cache settings**: TTL values and file paths for caching
//! - **Configuration**: Default directories and file names for config storage
//! - **HTTP settings**: Timeout values and retry limits for network requests
//! - **Platform icons**: Unicode icons for each supported platform in wrapper mode

/// User agent string sent with HTTP requests to identify the application
pub const APP_USER_AGENT: &str = concat!("tsukiyomi-fetch/", "0.1.0");

/// Cache time-to-live in seconds (24 hours)
/// Controls how long cached data remains valid before requiring refresh
pub const CACHE_TTL_SECONDS: i64 = 60 * 60 * 24; // 1 day

/// Directory path for cache storage, relative to user's home directory
pub const CACHE_DIR: &str = ".cache/fastfetch";

/// Filename for the cache file that stores fetched data
pub const CACHE_FILE: &str = "tsukiyomi.cache";

/// Directory path for configuration storage, relative to user's home directory
pub const CONFIG_DIR: &str = ".config/fastfetch";

/// Filename for the configuration file that stores user credentials
pub const CONFIG_FILE: &str = "tsukiyomi-fetch.conf";

/// HTTP request timeout in seconds
/// Requests that take longer than this will be cancelled
pub const HTTP_TIMEOUT_SECONDS: u64 = 30;

/// Maximum number of retry attempts for failed HTTP requests
pub const MAX_RETRIES: usize = 3;

/// Platform-specific default icons for wrapper mode
pub const CODEFORCES_ICONS: &[&str] = &[""];
pub const CODECHEF_ICONS: &[&str] = &[""];
pub const GITHUB_ICONS: &[&str] = &["", "", "", ""];
pub const ANILIST_ICONS: &[&str] = &["󰑈", "󰈈", "󰂺", "󰈈"];
pub const SIMKL_ICONS: &[&str] = &["", "󰈈"];
pub const MYANIMELIST_ICONS: &[&str] = &["", "󰈈", "󰂺", "󰈈"];
pub const LEETCODE_ICONS: &[&str] = &["󰆥"];
pub const INSTAGRAM_ICONS: &[&str] = &["", ""];
pub const REDDIT_ICONS: &[&str] = &["", ""];
pub const STEAM_ICONS: &[&str] = &["󰊖", "󰥔"];
pub const TWITTER_ICONS: &[&str] = &[""];
pub const DISCORD_ICONS: &[&str] = &["", "", "", "", ""];

// Helper function to get platform-specific default icons
pub fn get_platform_icons(platform: &str) -> &'static [&'static str] {
    match platform {
        "codeforces" => CODEFORCES_ICONS,
        "codechef" => CODECHEF_ICONS,
        "github" => GITHUB_ICONS,
        "anilist" => ANILIST_ICONS,
        "simkl" => SIMKL_ICONS,
        "myanimelist" => MYANIMELIST_ICONS,
        "leetcode" => LEETCODE_ICONS,
        "instagram" => INSTAGRAM_ICONS,
        "reddit" => REDDIT_ICONS,
        "steam" => STEAM_ICONS,
        "twitter" => TWITTER_ICONS,
        "discord" => DISCORD_ICONS,
        _ => &[], // Empty array for unknown platforms
    }
}
