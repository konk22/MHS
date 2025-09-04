#!/bin/bash

# Script to create DMG file for macOS
# Usage: ./scripts/create-dmg.sh [target]

set -e

TARGET=${1:-"aarch64-apple-darwin"}
APP_NAME="Moonraker Host Scanner"
DMG_NAME="${APP_NAME// /_}_0.1.0_${TARGET}.dmg"
APP_PATH="src-tauri/target/${TARGET}/release/bundle/macos/${APP_NAME}.app"
DMG_PATH="src-tauri/target/${TARGET}/release/bundle/dmg/${DMG_NAME}"

echo "Creating DMG for target: ${TARGET}"
echo "App path: ${APP_PATH}"
echo "DMG path: ${DMG_PATH}"

# Check if app exists
if [ ! -d "${APP_PATH}" ]; then
    echo "Error: App bundle not found at ${APP_PATH}"
    echo "Please build the app first with: pnpm tauri build --target ${TARGET}"
    exit 1
fi

# Create DMG directory
mkdir -p "$(dirname "${DMG_PATH}")"

# Create DMG using hdiutil
echo "Creating DMG file..."
hdiutil create -volname "${APP_NAME}" -srcfolder "${APP_PATH}" -ov -format UDZO "${DMG_PATH}"

echo "DMG created successfully at: ${DMG_PATH}"
echo "Size: $(du -h "${DMG_PATH}" | cut -f1)"
