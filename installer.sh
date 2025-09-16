#!/bin/bash

FASTFETCH_DIR="/home/$USER/.config/fastfetch"

if ! [[ -d "$FASTFETCH_DIR" ]]; then
    echo "Creating Fastfetch directory..."
    mkdir -p "$FASTFETCH_DIR"
else
    echo "Fastfetch directory already exists."
    echo "Backing up existing Fastfetch configuration files..."
    a=$(date +%Y%m%d_%H%M%S)
    mkdir -p "$FASTFETCH_DIR/backup_$a"
    cp -r "$FASTFETCH_DIR/*" "${FASTFETCH_DIR}_backup_${a}/"
    echo "Backup completed."
fi

echo "Copying Fastfetch configuration files..."
cp -r ./fastfetch/* "$FASTFETCH_DIR/"

read -p "Do you want to download additional anime fastfetch images? (y/N): " choice
if [[ "$choice" == "y" || "$choice" == "Y" ]]; then
    echo "Downloading additional anime fastfetch images..."
    git clone https://github.com/thunder-blaze/FastfetchPngs.git /tmp/FastfetchPngs
    mkdir -p "$FASTFETCH_DIR/pngs"
    cp -rf /tmp/FastfetchPngs/* "$FASTFETCH_DIR/pngs/"
fi

echo "Fastfetch configuration files copied successfully."
echo "Initializing Fastfetch Script..."

if [[ -x "$FASTFETCH_DIR/tsukiyomi-fetch" ]]; then
    "$FASTFETCH_DIR/tsukiyomi-fetch" --setup
else
    echo "Error: tsukiyomi-fetch not found or not executable."
    exit 1
fi

if [[ $? -e 0 ]]; then
    echo "Fastfetch Script initialized successfully."
    if [[ -d "/tmp/FastfetchPngs" ]]; then
        echo "Cleaning up temporary files..."
        rm -rf /tmp/FastfetchPngs
    fi
else
    echo "Error: Failed to initialize Fastfetch Script."
    exit 1
fi
