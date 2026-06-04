#!/usr/bin/env bash
# fetch_bundled.sh — refresh bundled/ from upstream.
#
# Thin wrapper around `cargo xtask bundle`, which fetches every registry file at
# upstream HEAD through gloam's own acquisition path (the same one `--fetch`
# uses) and writes both the file bytes under bundled/ and the provenance
# manifest bundled/provenance.json.  Keeping a single Rust code path guarantees
# the bundled and --fetch provenance are produced identically.
#
# Set GITHUB_TOKEN to lift the GitHub API rate limit (strongly recommended; the
# unauthenticated limit of 60 req/hr is easily exhausted by a full refresh).
#
# Usage:
#   ./scripts/fetch_bundled.sh
#
# Requirements: a Rust toolchain (the bundler is a cargo xtask) and network.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT"

exec cargo xtask bundle "$@"
