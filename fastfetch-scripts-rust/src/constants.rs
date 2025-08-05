// Application constants
pub const APP_NAME: &str = "tsukiyomi-fetch";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_USER_AGENT: &str = concat!("tsukiyomi-fetch/", "0.1.0");

// Cache settings
pub const CACHE_TTL_SECONDS: i64 = 60 * 60 * 24; // 1 day
pub const CACHE_DIR: &str = ".cache/fastfetch";
pub const CACHE_FILE: &str = "tsukiyomi.cache";

// Config settings
pub const CONFIG_DIR: &str = ".config/fastfetch";
pub const CONFIG_FILE: &str = "tsukiyomi-fetch.conf";

// HTTP settings
pub const HTTP_TIMEOUT_SECONDS: u64 = 30;
pub const MAX_RETRIES: usize = 3;

// Supported platforms and their metrics
pub const GITHUB_METRICS: &[&str] = &["repos", "followers", "following", "stars", "forks", "prs"];
pub const CODEFORCES_METRICS: &[&str] = &["rating", "maxrating"];
pub const CODECHEF_METRICS: &[&str] = &["rating", "maxrating"];
pub const LEETCODE_METRICS: &[&str] = &["rank"];
pub const ANILIST_METRICS: &[&str] = &["anime_count", "manga_count", "episodes", "chapters"];
pub const SIMKL_METRICS: &[&str] = &["movies", "tv", "anime", "totalhours"];
pub const MAL_METRICS: &[&str] = &["anime_total", "manga_total", "anime_episodes", "manga_chapters"];
pub const INSTAGRAM_METRICS: &[&str] = &["followers", "following"];

// Color codes for terminal output
pub const COLOR_CODES: &[(&str, &str)] = &[
    ("black", "30"),
    ("red", "31"),
    ("green", "32"),
    ("yellow", "33"),
    ("blue", "34"),
    ("magenta", "35"),
    ("cyan", "36"),
    ("white", "37"),
];

// Platform-specific default icons for wrapper mode
pub const CODEFORCES_ICONS: &[&str] = &[""];
pub const CODECHEF_ICONS: &[&str] = &[""];
pub const GITHUB_ICONS: &[&str] = &["", "", "", ""];
pub const ANILIST_ICONS: &[&str] = &["", "", "󰂺", ""];
pub const SIMKL_ICONS: &[&str] = &["", ""];
pub const MYANIMELIST_ICONS: &[&str] = &["", "", "󰂺", ""];
pub const LEETCODE_ICONS: &[&str] = &["󰆥"];
pub const INSTAGRAM_ICONS: &[&str] = &["", ""];

// Legacy - kept for backward compatibility
pub const DEFAULT_ICONS: &[&str] = &["", "", "", "", "󰂺", "", "", ""];

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
        _ => &[], // Empty array for unknown platforms
    }
}
