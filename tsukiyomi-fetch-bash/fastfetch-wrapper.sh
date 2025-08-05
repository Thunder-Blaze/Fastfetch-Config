#!/bin/bash

if [[ -z "$1" ]]; then
    echo "Usage: $0 <platform>"
    exit 1
fi

PLATFORM="$1"

case "$PLATFORM" in
    "codeforces")
        a=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh codeforces rating)
        b=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh codeforces maxrating)
        echo "$a \u001b[0;36m\u001b[0m $b"
        ;;
    "codechef")
        a=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh codechef rating)
        b=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh codechef maxrating)
        echo "$a \u001b[0;36m\u001b[0m $b"
        ;;
    "github")
        repos=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh github repos)
        prs=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh github prs)
        stars=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh github stars)
        followers=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh github followers)
        echo "\u001b[0;36m\u001b[0m $repos \u001b[0;36m\u001b[0m $prs \u001b[0;36m\u001b[0m $stars \u001b[0;36m\u001b[0m $followers"
        ;;
    "anilist")
        anime=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh anilist anime_count)
        manga=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh anilist manga_count)
        episodes=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh anilist episodes)
        chapters=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh anilist chapters)
        echo "\u001b[0;36m\u001b[0m $anime \u001b[0;36m\u001b[0m $episodes \u001b[0;36m󰂺\u001b[0m $manga \u001b[0;36m\u001b[0m $chapters"
        ;;
    "simkl")
        simkl_movies_hours=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh simkl movies hours)
        simkl_movies=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh simkl movies completed)
        echo "\u001b[0;36m\u001b[0m $simkl_movies \u001b[0;36m\u001b[0m ${simkl_movies_hours}h"
        ;;
    *)
        echo "Unknown platform: $PLATFORM"
        exit 1
        ;;
esac
