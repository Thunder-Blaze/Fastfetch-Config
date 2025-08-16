# tsukiyomi-fetch

`tsukiyomi-fetch` is a performance-optimized Rust-powered stats fetcher with intelligent caching and asynchronous API calls.  
It is designed to integrate seamlessly with [Fastfetch](https://github.com/fastfetch-cli/fastfetch), providing wrappers for fetching and formatting platform statistics with minimal latency.

---

## Features

- Rust-powered backend for blazing fast execution  
- Intelligent caching system with 24-hour TTL  
- Asynchronous API calls for minimal latency  
- Wrapper commands for easy usage  
- Customizable output formatting (icon colors, glyphs, etc.)  
- Supports multiple platforms out of the box

---

## Installation

Add to your Cargo project:

```bash
cargo add tsukiyomi-fetch
```

Or install globally:

```bash
cargo install tsukiyomi-fetch
```

---

## Usage

### Wrapper Commands

Wrappers provide ready-to-use commands for fetching and formatting stats from supported platforms.

```bash
~/.config/fastfetch/tsukiyomi-fetch wrapper github
# Fetches and displays all GitHub stats
```

### Customization

You can customize wrapper output with flags:

```bash
# Change icon color (matches Fastfetch keycolor "green")
~/.config/fastfetch/tsukiyomi-fetch wrapper github --color green  

# Override default glyphs/icons
~/.config/fastfetch/tsukiyomi-fetch wrapper github --icons a --icons b  
```

---

## Supported Platforms

- GitHub  
- Discord  
- Codeforces  
- CodeChef  
- LeetCode  
- (more coming soon)

---

## License

This project is licensed under the MIT License.
