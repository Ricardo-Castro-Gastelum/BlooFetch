#!/bin/bash
set -e

BLOOFETCH_VERSION="v0.2.0"
REPO="Ricardo-Castro-Gastelum/bloofetch"

detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case $os in
        linux)
            case $arch in
                x86_64) echo "bloofetch-linux-amd64" ;;
                aarch64|arm64) echo "bloofetch-linux-arm64" ;;
                *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        darwin)
            case $arch in
                x86_64) echo "bloofetch-macos-amd64" ;;
                arm64) echo "bloofetch-macos-arm64" ;;
                *) echo "Unsupported architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        *)
            echo "Unsupported OS: $os" >&2
            echo "For other platforms, download from: https://github.com/$REPO/releases" >&2
            exit 1
            ;;
    esac
}

main() {
    local binary=$(detect_platform)
    local url="https://github.com/$REPO/releases/download/$BLOOFETCH_VERSION/$binary"

    echo "Downloading BlooFetch $BLOOFETCH_VERSION..."

    local tmpfile=$(mktemp)
    curl -sL "$url" -o "$tmpfile"

    local install_dir="$HOME/.local/bin"
    mkdir -p "$install_dir"
    mv "$tmpfile" "$install_dir/bloofetch"
    chmod +x "$install_dir/bloofetch"

    echo ""
    echo "✓ Installed to $install_dir/bloofetch"
    echo ""

    # Check if ~/.local/bin is in PATH
    case ":$PATH:" in
        *":$HOME/.local/bin:"*)
            echo "Run 'bloofetch' to start!"
            ;;
        *)
            echo "Add this to your ~/.zshrc (or ~/.bashrc):"
            echo ""
            echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
            echo ""
            echo "Then run: source ~/.zshrc"
            ;;
    esac
}

main "$@"
