#!/bin/bash

TTL=$(( 60 * 60 * 24 ))  # Cache TTL: 1 day

CONFIG_DIR="$HOME/.config/fastfetch"
mkdir -p "$CONFIG_DIR"
CONFIG="$CONFIG_DIR/fastfetch-scripts.conf"
CACHE_FILE="$CONFIG_DIR/.cache"
touch "$CONFIG"
touch "$CACHE_FILE"

save_cache() {
    local key="$1"
    local val="$2"
    local username="$3"
    echo "$key $val $(date +%s) $username" >> "$CACHE_FILE"
}

get_cached() {
    local key="$1"
    local current_user="$2"
    local now=$(date +%s)
    local entry
    entry=$(grep "^$key " "$CACHE_FILE" | tail -n1)

    if [[ -n "$entry" ]]; then
        local value=$(echo "$entry" | awk '{print $2}')
        local timestamp=$(echo "$entry" | awk '{print $3}')
        local stored_user=$(echo "$entry" | awk '{print $4}')
        if [[ "$stored_user" == "$current_user" && $((now - timestamp)) -lt $TTL ]]; then
            echo "$value"
            return 0
        fi
    fi
    return 1
}

get_config_value() {
    grep "^$1=" "$CONFIG" | cut -d '=' -f2
}

handle_error() {
    echo "error"
    exit 1
}

print_help() {
    cat <<EOF
Usage:
  $0 <platform> <subparam>

Platforms and Subparams:

  codeforces
    rating        → Current rating
    maxrating     → Highest rating

  codechef
    rating        → Current rating
    maxrating     → Highest rating

  leetcode
    rating        → Current contest rating

  github
    repos         → Public repositories
    followers     → Followers
    following     → Following
    prs           → Pull requests authored
    stars         → Starred repositories

  anilist
    anime_count   → Anime watched
    episodes      → Episodes watched
    manga_count   → Manga read
    chapters      → Chapters read

Other Options:
  --setup         → Configure your usernames
  --help          → Show this help

Examples:
  $0 codeforces rating
  $0 github stars
  $0 anilist chapters
EOF
    exit 0
}

[[ "$1" == "--help" || "$#" -eq 0 ]] && print_help

if [[ "$1" == "--setup" ]]; then
    read -p "Codeforces Username: " cf
    read -p "CodeChef Username: " cc
    read -p "LeetCode Username: " lc
    read -p "GitHub Username: " gh
    read -p "AniList Username: " al
    {
        echo "Codeforces=$cf"
        echo "CodeChef=$cc"
        echo "LeetCode=$lc"
        echo "GitHub=$gh"
        echo "AniList=$al"
    } > "$CONFIG"
    echo "Setup complete!"
    exit 0
fi

platform="$1"
subparam="$2"

case "$platform" in
    codeforces)
        user=$(get_config_value "Codeforces")
        [[ -z "$user" ]] && echo "Missing Codeforces username. Run with --setup" && exit 1
        key="cf_${subparam}"
        if cached=$(get_cached "$key" "$user"); then echo "$cached"; exit 0; fi
        resp=$(curl -sf "https://codeforces.com/api/user.info?handles=$user") || handle_error
        [[ $(echo "$resp" | jq -r .status) != "OK" ]] && handle_error
        case "$subparam" in
            rating) val=$(jq -r '.result[0].rating // empty' <<< "$resp") ;;
            maxrating) val=$(jq -r '.result[0].maxRating // empty' <<< "$resp") ;;
            *) print_help ;;
        esac
        [[ -n "$val" ]] && save_cache "$key" "$val" "$user" && echo "$val" || handle_error
        ;;

    codechef)
        user=$(get_config_value "CodeChef")
        [[ -z "$user" ]] && echo "Missing CodeChef username. Run with --setup" && exit 1
        key="cc_${subparam}"
        if cached=$(get_cached "$key" "$user"); then echo "$cached"; exit 0; fi
        resp=$(curl -sf "https://www.codechef.com/users/$user") || handle_error
        case "$subparam" in
            rating)
                val=$(grep -oP '<div class="rating-number">(\d+)' <<< "$resp" | grep -oP '\d+')
                ;;
            maxrating)
                val=$(grep -oP 'Highest Rating [^0-9]*\K\d+' <<< "$resp")
                ;;
            *) print_help ;;
        esac
        [[ -n "$val" ]] && save_cache "$key" "$val" "$user" && echo "$val" || handle_error
        ;;

    leetcode)
        user=$(get_config_value "LeetCode")
        [[ -z "$user" ]] && echo "Missing LeetCode username. Run with --setup" && exit 1
        key="lc_${subparam}"
        if cached=$(get_cached "$key" "$user"); then echo "$cached"; exit 0; fi
        case "$subparam" in
            rating)
                val=$(curl -sf "https://leetcode-stats-api.herokuapp.com/$user" | jq -r '.rating // 0') || handle_error
                ;;
            *) print_help ;;
        esac
        [[ -n "$val" ]] && save_cache "$key" "$val" "$user" && echo "$val" || handle_error
        ;;

    github)
        user=$(get_config_value "GitHub")
        [[ -z "$user" ]] && echo "Missing GitHub username. Run with --setup" && exit 1
        key="gh_${subparam}"
        if cached=$(get_cached "$key" "$user"); then echo "$cached"; exit 0; fi

        case "$subparam" in
            repos|followers|following)
                if [[ "$subparam" == "repos" ]]; then
                    subparam="public_repos"
                fi
                val=$(curl -sf "https://api.github.com/users/$user" | jq -r ".${subparam} // 0") || handle_error
                ;;
            prs)
                val=$(curl -sf "https://api.github.com/search/issues?q=author:$user+type:pr" | jq -r '.total_count // 0') || handle_error
                ;;
            stars|forks)
                val=$(curl -sf "https://api.github-star-counter.workers.dev/user/$user" | jq -r ".${subparam} // 0") || handle_error
                ;;
            *) print_help ;;
        esac
        [[ -n "$val" ]] && save_cache "$key" "$val" "$user" && echo "$val" || handle_error
        ;;

    anilist)
        user=$(get_config_value "AniList")
        [[ -z "$user" ]] && echo "Missing AniList username. Run with --setup" && exit 1
        key="al_${subparam}"
        if cached=$(get_cached "$key" "$user"); then echo "$cached"; exit 0; fi

        read -r -d '' query <<EOF
{
  "query": "query (\$name: String) { User(name: \$name) { statistics { anime { count episodesWatched } manga { count chaptersRead } } } }",
  "variables": { "name": "$user" }
}
EOF

        resp=$(curl -sf -X POST -H "Content-Type: application/json" -d "$query" https://graphql.anilist.co) || handle_error

        case "$subparam" in
            anime_count) val=$(jq -r '.data.User.statistics.anime.count // 0' <<< "$resp") ;;
            episodes)    val=$(jq -r '.data.User.statistics.anime.episodesWatched // 0' <<< "$resp") ;;
            manga_count) val=$(jq -r '.data.User.statistics.manga.count // 0' <<< "$resp") ;;
            chapters)    val=$(jq -r '.data.User.statistics.manga.chaptersRead // 0' <<< "$resp") ;;
            *) print_help ;;
        esac
        [[ -n "$val" ]] && save_cache "$key" "$val" "$user" && echo "$val" || handle_error
        ;;

    *) print_help ;;
esac
