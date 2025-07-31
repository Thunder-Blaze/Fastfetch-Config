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
    stars         → User repository stars
    forks         → User repository forks

  anilist
    anime_count   → Anime watched
    episodes      → Episodes watched
    manga_count   → Manga read
    chapters      → Chapters read

  simkl
    anime         → Anime stats
      hours        → Total hours watched
      completed    → Completed anime count
    tv           → TV stats
      hours        → Total hours watched
      completed    → Completed TV count
    movies       → Movies stats
      hours        → Total hours watched
      completed    → Completed movies count
    totalhours   → Total hours across all media

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
    read -p "Simkl User ID: " sk
    {
        echo "Codeforces=$cf"
        echo "CodeChef=$cc"
        echo "LeetCode=$lc"
        echo "GitHub=$gh"
        echo "AniList=$al"
        echo "Simkl=$sk"
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
        rating=$(jq -r '.result[0].rating // empty' <<< "$resp")
        maxrating=$(jq -r '.result[0].maxRating // empty' <<< "$resp")
        [[ -n "$rating" ]] && save_cache "cf_rating" "$rating" "$user"
        [[ -n "$maxrating" ]] && save_cache "cf_maxrating" "$maxrating" "$user"

        case "$subparam" in
            rating) echo "$rating" ;;
            maxrating) echo "$maxrating" ;;
            *) print_help ;;
        esac
        ;;

    codechef)
        user=$(get_config_value "CodeChef")
        [[ -z "$user" ]] && echo "Missing CodeChef username. Run with --setup" && exit 1
        key="cc_${subparam}"
        if cached=$(get_cached "$key" "$user"); then echo "$cached"; exit 0; fi
        resp=$(curl -sf "https://www.codechef.com/users/$user") || handle_error
        rating=$(grep -oP '<div class="rating-number">(\d+)' <<< "$resp" | grep -oP '\d+')
        maxrating=$(grep -oP 'Highest Rating [^0-9]*\K\d+' <<< "$resp")

        [[ -n "$rating" ]] && save_cache "cc_rating" "$rating" "$user"
        [[ -n "$maxrating" ]] && save_cache "cc_maxrating" "$maxrating" "$user"

        case "$subparam" in
            rating) echo "$rating" ;;
            maxrating) echo "$maxrating" ;;
            *) print_help ;;
        esac
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
                resp=$(curl -sf "https://api.github.com/users/$user") || handle_error
                repos=$(jq -r '.public_repos // 0' <<< "$resp")
                followers=$(jq -r '.followers // 0' <<< "$resp")
                following=$(jq -r '.following // 0' <<< "$resp")

                save_cache "gh_repos" "$repos" "$user"
                save_cache "gh_followers" "$followers" "$user"
                save_cache "gh_following" "$following" "$user"

                case "$subparam" in
                    repos) echo "$repos" ;;
                    followers) echo "$followers" ;;
                    following) echo "$following" ;;
                esac
                ;;

            stars|forks)
                resp=$(curl -sf "https://api.github-star-counter.workers.dev/user/$user") || handle_error
                stars=$(jq -r '.stars // 0' <<< "$resp")
                forks=$(jq -r '.forks // 0' <<< "$resp")

                save_cache "gh_stars" "$stars" "$user"
                save_cache "gh_forks" "$forks" "$user"

                case "$subparam" in
                    stars) echo "$stars" ;;
                    forks) echo "$forks" ;;
                esac
                ;;

            prs)
                val=$(curl -sf "https://api.github.com/search/issues?q=author:$user+type:pr" | jq -r '.total_count // 0') || handle_error
                save_cache "gh_prs" "$val" "$user"
                echo "$val"
                ;;

            *) print_help ;;
        esac
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

        anime_count=$(jq -r '.data.User.statistics.anime.count // 0' <<< "$resp")
        episodes=$(jq -r '.data.User.statistics.anime.episodesWatched // 0' <<< "$resp")
        manga_count=$(jq -r '.data.User.statistics.manga.count // 0' <<< "$resp")
        chapters=$(jq -r '.data.User.statistics.manga.chaptersRead // 0' <<< "$resp")

        save_cache "al_anime_count" "$anime_count" "$user"
        save_cache "al_episodes" "$episodes" "$user"
        save_cache "al_manga_count" "$manga_count" "$user"
        save_cache "al_chapters" "$chapters" "$user"

        case "$subparam" in
            anime_count) echo "$anime_count" ;;
            episodes) echo "$episodes" ;;
            manga_count) echo "$manga_count" ;;
            chapters) echo "$chapters" ;;
            *) print_help ;;
        esac
        ;;
    
    simkl)
        user=$(get_config_value "Simkl")
        [[ -z "$user" ]] && echo "Missing Simkl user ID. Run with --setup" && exit 1

        # Check if *any* simkl stat is already cached (we assume all are cached together)
        probe_key="sk_anime_hours"
        if cached=$(get_cached "$probe_key" "$user"); then
            case "$subparam" in
                anime|tv|movies)
                    [[ "$3" == "hours" ]] && get_cached "sk_${subparam}_hours" "$user" && exit 0
                    [[ "$3" == "completed" ]] && get_cached "sk_${subparam}_completed" "$user" && exit 0
                    ;;
                totalhours)
                    get_cached "sk_totalhours" "$user" && exit 0
                    ;;
                *) print_help ;;
            esac
        fi

        # Fetch all stats at once
        resp=$(curl -sf "https://api.simkl.com/users/${user}/stats") || handle_error

        # Extract and cache all stats
        for type in anime tv movies; do
            mins=$(jq -r ".${type}.total_mins // 0" <<< "$resp")
            hours=$((mins / 60))
            completed=$(jq -r ".${type}.completed.count // 0" <<< "$resp")
            save_cache "sk_${type}_hours" "$hours" "$user"
            save_cache "sk_${type}_completed" "$completed" "$user"
        done
        total_mins=$(jq -r '.total_mins' <<< "$resp")
        total_hours=$((total_mins / 60))
        save_cache "sk_totalhours" "$total_hours" "$user"

        # Output requested stat
        case "$subparam" in
            anime|tv|movies)
                [[ "$3" == "hours" ]] && echo "${!subparam}_hours" && get_cached "sk_${subparam}_hours" "$user" && exit 0
                [[ "$3" == "completed" ]] && get_cached "sk_${subparam}_completed" "$user" && exit 0
                ;;
            totalhours)
                echo "$total_hours"
                ;;
            *) print_help ;;
        esac
        ;;
    
    *) print_help ;;
esac
