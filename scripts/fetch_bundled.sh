#!/usr/bin/env bash
# fetch_bundled.sh — populate bundled/ from canonical upstream sources.
#
# Run this once before building gloam when you want to refresh the bundled
# copies.  The binary embeds these files at compile time via include_str!().
#
# Usage:
#   ./scripts/fetch_bundled.sh          # fetch everything
#   ./scripts/fetch_bundled.sh --xml    # XML specs only
#   ./scripts/fetch_bundled.sh --hdrs   # auxiliary headers only
#
# Requirements: curl (or wget as fallback), standard POSIX shell utilities.
# Exit codes:  0 = success, 1 = one or more fetches failed.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

XML_DIR="$ROOT/bundled/xml"
HDR_DIR="$ROOT/bundled/headers"

# ---------------------------------------------------------------------------
# Source URLs
# ---------------------------------------------------------------------------

GL_BASE="https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/main/xml"
EGL_BASE="https://raw.githubusercontent.com/KhronosGroup/EGL-Registry/main/api"
VK_XML_BASE="https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main/xml"
VK_HDR_BASE="https://raw.githubusercontent.com/KhronosGroup/Vulkan-Headers/main/include/vulkan"
ANGLE_BASE="https://raw.githubusercontent.com/google/angle/main/scripts"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
ERRORS=0

fetch() {
    local url="$1"
    local dest="$2"
    local label="${3:-$(basename "$dest")}"

    printf "  %-55s " "$label"

    # Try curl first, fall back to wget.
    if command -v curl &>/dev/null; then
        if curl -fsSL --retry 3 --retry-delay 2 -o "$dest" "$url" 2>/dev/null; then
            echo -e "${GREEN}ok${NC}"
            return 0
        fi
    elif command -v wget &>/dev/null; then
        if wget -q --tries=3 -O "$dest" "$url" 2>/dev/null; then
            echo -e "${GREEN}ok${NC}"
            return 0
        fi
    else
        echo -e "${RED}FAIL (no curl or wget)${NC}"
        ERRORS=$((ERRORS + 1))
        return 1
    fi

    echo -e "${RED}FAIL${NC}"
    # Leave the destination empty (or as-was) rather than writing partial data.
    : > "$dest"
    ERRORS=$((ERRORS + 1))
    return 1
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------

DO_XML=1
DO_HDRS=1

for arg in "$@"; do
    case "$arg" in
        --xml)  DO_HDRS=0 ;;
        --hdrs) DO_XML=0  ;;
        --help|-h)
            sed -n '2,14p' "$0" | sed 's/^# //'
            exit 0
            ;;
        *)
            echo "Unknown argument: $arg  (try --help)" >&2
            exit 1
            ;;
    esac
done

# ---------------------------------------------------------------------------
# XML specs
# ---------------------------------------------------------------------------

if [[ $DO_XML -eq 1 ]]; then
    echo ""
    echo "Fetching XML specs..."
    mkdir -p "$XML_DIR"

    fetch "$GL_BASE/gl.xml"          "$XML_DIR/gl.xml"
    fetch "$GL_BASE/glx.xml"         "$XML_DIR/glx.xml"
    fetch "$GL_BASE/wgl.xml"         "$XML_DIR/wgl.xml"
    fetch "$EGL_BASE/egl.xml"        "$XML_DIR/egl.xml"
    fetch "$VK_XML_BASE/vk.xml"      "$XML_DIR/vk.xml"

    echo ""
    echo "Fetching supplemental XML specs..."

    fetch "$ANGLE_BASE/gl_angle_ext.xml"   "$XML_DIR/gl_angle_ext.xml"
    fetch "$ANGLE_BASE/egl_angle_ext.xml"  "$XML_DIR/egl_angle_ext.xml"
fi

# ---------------------------------------------------------------------------
# Auxiliary headers
# ---------------------------------------------------------------------------

if [[ $DO_HDRS -eq 1 ]]; then
    echo ""
    echo "Fetching auxiliary headers..."

    mkdir -p "$HDR_DIR/KHR"
    mkdir -p "$HDR_DIR/EGL"
    mkdir -p "$HDR_DIR/vk_video"
    mkdir -p "$HDR_DIR/vulkan"

    # xxhash — use the single-file amalgamation from the official release.
    # We pin to a specific tag for reproducibility.
    XXHASH_TAG="v0.8.3"
    XXHASH_URL="https://raw.githubusercontent.com/Cyan4973/xxHash/${XXHASH_TAG}/xxhash.h"
    fetch "$XXHASH_URL" "$HDR_DIR/xxhash.h" "xxhash.h ($XXHASH_TAG)"

    # KHR / EGL platform headers — from the EGL registry.
    EGL_HDR_BASE="https://raw.githubusercontent.com/KhronosGroup/EGL-Registry/main/api"
    fetch "$EGL_HDR_BASE/KHR/khrplatform.h"   "$HDR_DIR/KHR/khrplatform.h"
    fetch "$EGL_HDR_BASE/EGL/eglplatform.h"   "$HDR_DIR/EGL/eglplatform.h"

    # Vulkan platform header.
    fetch "$VK_HDR_BASE/vk_platform.h"  "$HDR_DIR/vulkan/vk_platform.h"

    # Vulkan video extension headers.
    VK_VIDEO_HDRS=(
        vulkan_video_codecs_common.h
        vulkan_video_codec_h264std.h
        vulkan_video_codec_h264std_decode.h
        vulkan_video_codec_h264std_encode.h
        vulkan_video_codec_h265std.h
        vulkan_video_codec_h265std_decode.h
        vulkan_video_codec_h265std_encode.h
        vulkan_video_codec_av1std.h
        vulkan_video_codec_av1std_decode.h
        vulkan_video_codec_av1std_encode.h
        vulkan_video_codec_vp9std.h
        vulkan_video_codec_vp9std_decode.h
    )
    VK_VIDEO_BASE="https://raw.githubusercontent.com/KhronosGroup/Vulkan-Headers/main/include/vk_video"
    for hdr in "${VK_VIDEO_HDRS[@]}"; do
        fetch "$VK_VIDEO_BASE/$hdr" "$HDR_DIR/vk_video/$hdr"
    done
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------

echo ""
if [[ $ERRORS -eq 0 ]]; then
    echo -e "${GREEN}All files fetched successfully.${NC}"
    echo "Run 'cargo build' to compile with the updated bundled files."
else
    echo -e "${RED}$ERRORS file(s) failed to fetch.${NC}"
    echo "Check your network connection and try again."
    echo "Failed destinations were left empty; gloam will refuse to run"
    echo "until they are populated (or use --fetch at runtime instead)."
    exit 1
fi
