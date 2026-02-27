#!/bin/bash

# Configuration
BINARY_PATH="./src-tauri/target/debug/photon-proxy"

echo "🚀 Building Photon Proxy (Debug)..."
bun run tauri build --debug

if [ -f "$BINARY_PATH" ]; then
    echo "🔓 Granting network capabilities to $BINARY_PATH..."
    sudo setcap 'cap_net_admin,cap_net_raw+ep' "$BINARY_PATH"
    echo "✅ Success! You can now run the app from your IDE or terminal without sudo."
    echo "💡 Note: You may need to run this script again if you make changes to the Rust code."
else
    echo "❌ Error: Binary not found at $BINARY_PATH"
    exit 1
fi
