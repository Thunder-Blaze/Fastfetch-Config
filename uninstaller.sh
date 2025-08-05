#!/bin/bash

# Uninstaller script for Tsukiyomi-Fetch

CONFIG_DIR="$HOME/.config/fastfetch"
CONFIG_FILE="$CONFIG_DIR/tsukiyomi-fetch.conf"
EXEC_FILE="$CONFIG_DIR/tsukiyomi-fetch"
CACHE_DIR="$HOME/.cache/fastfetch"
CACHE_FILE="$CONFIG_DIR/tsukiyomi.cache"

echo "Uninstalling Tsukiyomi-Fetch..."

# Remove config directory
if [ -d "$CONFIG_DIR" ]; then
    if [ -f "${CONFIG_FILE}" ]; then
        echo "Removing tsukiyomi config..."
        rm -f "$CONFIG_FILE"
    fi
fi

# Remove script/executable
if [ -f "$EXEC_FILE" ]; then
    sudo rm -f "$EXEC_FILE"
    echo "Removed executable: $EXEC_FILE"
fi

# Remove cache
if [ -f "$CACHE_FILE" ]; then
    rm -f "$CACHE_FILE"
    echo "Removed cache file: $CACHE_FILE"
fi

echo "Tsukiyomi-Fetch has been uninstalled."
echo "Modify your Fastfetch configuration files to not use Tsukiyomi-Fetch."