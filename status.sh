#!/bin/bash
# status.sh — Quick health check for caveman-rs project

set -e

echo "=== Rust Binary ==="
if [ -f ~/.local/bin/caveman-rs ]; then
    echo "Binary: Installed at ~/.local/bin/caveman-rs"
    version=$(/home/hermes-pi/.local/bin/caveman-rs --version 2>/dev/null || echo "Unknown")
    echo "Version: $version"
    echo "Size: $(du -h ~/.local/bin/caveman-rs | cut -f1)"
else
    echo "ERROR: Binary not found at ~/.local/bin/caveman-rs"
fi

echo ""
echo "=== Python Plugin ==="
if [ -d ~/.hermes/plugins/caveman-compression ]; then
    echo "Plugin: Installed"
    echo "Files:"
    ls -la ~/.hermes/plugins/caveman-compression/
else
    echo "ERROR: Plugin directory not found"
fi

echo ""
echo "=== Git Status ==="
cd ~/caveman-rs || exit
git status --short
echo "Last commit: $(git log --oneline -1 2>/dev/null || echo 'No commits')"

echo ""
echo "=== Quick Test ==="
echo "Running caveman-rs on test input..."
echo '{"content": "The quick brown fox jumps over the lazy dog"}' | ~/.local/bin/caveman-rs
