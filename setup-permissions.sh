# Configuration
BINARY_PATH="./src-tauri/target/debug/photon-proxy"

# Check if binary is running to avoid "Text file busy"
if pgrep -x "temp-svelte" > /dev/null || pgrep -x "photon-proxy" > /dev/null; then
    echo "⚠️  Alert: The application is currently running."
    echo "� Please close Photon Proxy (temp-svelte/photon-proxy) and any running 'sudo' processes before continuing."
    exit 1
fi

echo "�🚀 Building Photon Proxy (Debug)..."
bun run tauri build --debug

if [ -f "$BINARY_PATH" ]; then
    echo "🔓 Granting network capabilities to $BINARY_PATH..."
    sudo setcap 'cap_net_admin,cap_net_raw+ep' "$BINARY_PATH"
    echo "✅ Success! You can now run the app from your IDE or terminal without sudo."
else
    # Fallback for old name if build failed but binary exists
    ALT_PATH="./src-tauri/target/debug/temp-svelte"
    if [ -f "$ALT_PATH" ]; then
        echo "🔓 Granting network capabilities to $ALT_PATH..."
        sudo setcap 'cap_net_admin,cap_net_raw+ep' "$ALT_PATH"
        echo "✅ Success (using legacy name)! Consider restarting the script once the process is killed."
    else
        echo "❌ Error: Binary not found at $BINARY_PATH"
        exit 1
    fi
fi
