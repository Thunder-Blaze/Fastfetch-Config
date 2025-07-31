# ⚡ Fastfetch Config

Custom configuration and statistics script for [Fastfetch](https://github.com/fastfetch-cli/fastfetch), designed to display personalized system and online profile stats in a clean and minimal way.

<img src="assets/Preview3.webp" width="100%" alt="Fastfetch Preview"/>

---

## 🎯 Features

- 💻 Minimal and clean Fastfetch configuration with lots of high quality anime images.
- 📊 Modular `fastfetch-scripts.sh` script to show dynamic stats from:
  - ✅ Codeforces (current rating, maximum rating)
  - ✅ CodeChef (current rating, maximum rating)
  - ✅ GitHub (public repos, PRs, stars, forks, following, followers)
  - ✅ AniList (anime count, episodes watched, manga count, manga read)
  - ✅ Simkl (total watch hours, completed anime, anime watch hours, completed movies, movies watch hours, completed tv, tv watch hours)
  - ✅ MyAnimeLisrt (anime count, episodes watched, manga count, manga read)
  - ✅ LeetCode (current rank)
  - ✅ Instagram (followers, following)
- ⏱️ Caching system to avoid repeated API calls (1-day TTL).
- 📦 Easy Installation and Configuration.

---

## 📁 Directory Structure

```bash
Fastfetch-Config/
├── assets/
│   ├── Preview1.png            # Preview image (red theme)
│   └── Preview2.png            # Preview image (green theme)
├── fastfetch/
│   ├── pngs/                   # Folder Containing about 50 high quality images for fastfetch
│   ├── config.jsonc            # Main Fastfetch config
│   └── fastfetch-scripts       # Script to format, fetch and cache stats (written in Rust)
├── fastfetch-scripts-rust/     # Source code for the Rust script
│   ├── Cargo.toml              # Rust dependencies
│   └── src/
│       ├── fetchers/           # Fetchers for various stats
│       ├── config.rs           # Configuration handling for the Rust script
│       ├── main.rs             # Main entry point for the Rust script
│       ├── cache.rs            # Caching Functions for the Rust script
│       └── wrapper.rs          # Functions format stats for the Rust script
├── fastfetch-scripts-bash/     # Source code for the Bash scripts (Older version)
├── installer.sh            # Installer for Easy Installation
└── README.md               # Readme For the Project
```

---

## 🚀 Setup

### 1. Install Dependencies

- [Fastfetch](https://github.com/fastfetch-cli/fastfetch)
- `curl`, `jq`, `awk`, and `bash` (already available on most Linux distros)
```bash
# For Arch Users Like Me
sudo pacman -S fastfetch
```

### 2. Clone this repo and Install

```bash
git clone https://github.com/Thunder-Blaze/Fastfetch-Config /tmp/fastfetch
cd /tmp/fastfetch
chmod +x ./installer.sh
./installer.sh
```

### 3. Make script executable

```bash
# Just In Case
chmod +x ~/.config/fastfetch/fastfetch-scripts
```

### 4. Update your Fastfetch config

> You can modify the config yourself to make it fit according to your needs

---

## 🧠 Supported Stats

> These Commands are For the Rust Script `fastfetch-scripts`, you can also use the Bash Script, but it is not recommended as it is deprecated and will not receive any updates.

| Wrapping Commands         |
|---------------------------|
| wrapper anilist           |
| wrapper github            |
| wrapper codechef          |
| wrapper codeforces        |
| wrapper myanimelist       |
| wrapper simkl             |
| wrapper leetcode          |
| wrapper instagram         |


| Command                         | Description                     |
|---------------------------------|---------------------------------|
| `codeforces rating`             | Codeforces Current Rating       |
| `codeforces maxrating`          | Codeforces Max Rating           |
| `codechef rating`               | CodeChef Current Rating         |
| `codechef maxrating`            | CodeChef Max Rating             |
| `leetcode rating`               | LeetCode Current Rating         |
| `github repos`                  | GitHub public repo count        |
| `github followers`              | GitHub User Followers           |
| `github follwing`               | GitHub User Following           |
| `github prs`                    | Github Total PRs                |
| `github stars`                  | GitHub User Repository Stars    |
| `github forks`                  | Github User Repository Forks    |
| `anilist anime_count`           | AniList Anime Watched           |
| `anilist episodes`              | AniList Episodes Watched        |
| `anilist manga_count`           | AniList Manga Read              |
| `anilist chapters`              | AniList Chapters Read           |
| `simkl totalhours`              | Simkl Total Watch Hours         |
| `simkl movies completed`        | Simkl Movie Completed Count     |
| `simkl movies hours`            | Simkl Movie Watch Hours         |
| `simkl anime completed`         | Simkl Anime Completed Count     |
| `simkl anime hours`             | Simkl Anime Watch Hours         |
| `simkl tv completed`            | Simkl TV Series Completed Count |
| `simkl tv hours`                | Simkl TV Series Watch Hours     |
| `myanimelist anime_count`       | MyAnimeList Anime Watched       |
| `myanimelist anime_episodes`    | MyAnimeList Episodes Watched    |
| `myanimelist manga_total`       | MyAnimeList Manga Read          |
| `myanimelist manga_chapters`    | MyAnimeList Chapters Read       |
| `instagram followers`           | Instagram User Followers        |
| `instagram following`           | Instagram User Following        |
| `leetcode rank`                 | LeetCode Current Rank           |

---

## ⚙️ Configuration

### `~/.config/fastfetch/fastfetch_scripts.conf`

You can create a config file to store your usernames:

```bash
GitHub=Thunder-Blaze
AniList=ThunderBlaze
...
```

> Note - For Simkl UserID, visit `https://simkl.com/profile`, the URL will change to `https://simkl.com/XXXXXXX/`, this XXXXXXX is your Simkl User Id 

### Caching

- Cache stored in `~/.config/fastfetch/.cache`
- TTL: 24 * 60 * 60 seconds (1 day)
- Automatically invalidated when TTL expires or username changes

---

## 📸 Screenshots

<p align="center">
  <img src="assets/Preview1.webp" width="100%" alt="Fastfetch Preview 1"/>
</p>
<p align="center">
  <img src="assets/Preview2.webp" width="100%" alt="Fastfetch Preview 2"/>
</p>

---

## 🤝 Contributions

Contributions, improvements, or stat suggestions are welcome! Feel free to open issues or PRs.

Special Thanks to 
[Zero](https://github.com/GraveEaterMadison) (instagram fetcher)
[Insane](https://github.com/In-Saiyan) (codechef, codeforces, leetcode fetchers)
