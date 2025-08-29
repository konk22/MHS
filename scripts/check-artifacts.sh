#!/bin/bash

echo "=== Artifacts Check Script ==="

# Check if artifacts directory exists
if [ ! -d "artifacts" ]; then
    echo "❌ artifacts directory not found"
    exit 1
fi

# Check if macos-arm directory exists
if [ ! -d "artifacts/macos-arm" ]; then
    echo "❌ artifacts/macos-arm directory not found"
    exit 1
fi

echo "✅ artifacts/macos-arm directory exists"

# Check for files
echo ""
echo "=== File Check ==="

DMG_FILES=$(find artifacts/macos-arm -name "*.dmg" 2>/dev/null)
TAR_FILES=$(find artifacts/macos-arm -name "*.tar.gz" 2>/dev/null)
APP_FILES=$(find artifacts/macos-arm -name "*.app" -type d 2>/dev/null)

echo "DMG files found:"
if [ -n "$DMG_FILES" ]; then
    echo "$DMG_FILES" | while read file; do
        echo "  ✅ $(basename "$file") ($(du -h "$file" | cut -f1))"
    done
else
    echo "  ❌ No DMG files found"
fi

echo ""
echo "TAR.GZ files found:"
if [ -n "$TAR_FILES" ]; then
    echo "$TAR_FILES" | while read file; do
        echo "  ✅ $(basename "$file") ($(du -h "$file" | cut -f1))"
    done
else
    echo "  ❌ No TAR.GZ files found"
fi

echo ""
echo "APP files found:"
if [ -n "$APP_FILES" ]; then
    echo "$APP_FILES" | while read file; do
        echo "  ✅ $(basename "$file") ($(du -h "$file" | cut -f1))"
    done
else
    echo "  ❌ No APP files found"
fi

echo ""
echo "=== Summary ==="
DMG_COUNT=$(echo "$DMG_FILES" | wc -l)
TAR_COUNT=$(echo "$TAR_FILES" | wc -l)
APP_COUNT=$(echo "$APP_FILES" | wc -l)

echo "Total files: $((DMG_COUNT + TAR_COUNT + APP_COUNT))"
echo "DMG: $DMG_COUNT"
echo "TAR.GZ: $TAR_COUNT"
echo "APP: $APP_COUNT"

if [ $DMG_COUNT -gt 0 ] || [ $TAR_COUNT -gt 0 ]; then
    echo "✅ Ready for release!"
    exit 0
else
    echo "❌ No release files found!"
    exit 1
fi
