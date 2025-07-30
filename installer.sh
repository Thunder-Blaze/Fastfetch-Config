FASTFETCH_DIR="/home/$USER/.config/fastfetch"
echo "$FASTFETCH_DIR"
if ! [[ -d "$FASTFETCH_DIR" ]]; then
    echo "Creating Fastfetch directory..."
    mkdir -p "$FASTFETCH_DIR"
else
    echo "Fastfetch directory already exists."
    echo "Backing up existing Fastfetch configuration files..."
    a=$(date +%Y%m%d_%H%M%S)
    mkdir -p "$FASTFETCH_DIR/backup_$a"
    cp -r "$FASTFETCH_DIR/*" "$FASTFETCH_DIR/backup_$a/"
    echo "Backup completed."
fi

echo "Copying Fastfetch configuration files..."
cp -r ./fastfetch/* "$FASTFETCH_DIR/"

echo "Fastfetch configuration files copied successfully."
echo "Initializing Fastfetch Script..."

exec "$FASTFETCH_DIR/fastfetch-scripts.sh --setup"
