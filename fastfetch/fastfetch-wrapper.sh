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
        a=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh anilist anime_count)
        b=$(/home/$USER/.config/fastfetch/fastfetch-scripts.sh anilist manga_count)
        echo "\u001b[0;36m\u001b[0m $a \u001b[0;36m󰂺\u001b[0m $b"
        ;;
    *)
        echo "Unknown platform: $PLATFORM"
        exit 1
        ;;
esac
