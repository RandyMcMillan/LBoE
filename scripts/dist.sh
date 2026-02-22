#!/usr/bin/env bash
# scripts/dist.sh — local cargo-dist helper
# Usage:
#   ./scripts/dist.sh                 # plan + build for current platform
#   ./scripts/dist.sh plan            # dry-run plan only
#   ./scripts/dist.sh build           # build artifacts
#   ./scripts/dist.sh generate        # regenerate CI manifests
#   ./scripts/dist.sh check           # check CI manifests are up to date
#   ./scripts/dist.sh install         # install/upgrade cargo-dist to pinned version
#   ./scripts/dist.sh tag [VERSION]   # create and push annotated git tag

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Read pinned version from workspace metadata
DIST_VERSION="$(grep 'cargo-dist-version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')"
DIST_BIN="${CARGO_HOME:-$HOME/.cargo}/bin/dist"

# ── helpers ──────────────────────────────────────────────────────────────────

die() { echo "error: $*" >&2; exit 1; }

installed_version() {
    "$DIST_BIN" --version 2>/dev/null | awk '{print $2}' || echo "none"
}

ensure_dist() {
    local have
    have="$(installed_version)"
    if [[ "$have" != "$DIST_VERSION" ]]; then
        echo "cargo-dist $have installed, need $DIST_VERSION — installing..."
        cargo install cargo-dist --version "$DIST_VERSION" --locked
    fi
}

current_tag() {
    # Most recent semver tag reachable from HEAD
    git describe --tags --match "v[0-9]*" --abbrev=0 2>/dev/null || echo ""
}

pkg_version() {
    grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

# ── commands ─────────────────────────────────────────────────────────────────

cmd_install() {
    echo "Installing cargo-dist $DIST_VERSION ..."
    cargo install cargo-dist --version "$DIST_VERSION" --locked
    echo "Installed: $("$DIST_BIN" --version)"
}

cmd_plan() {
    ensure_dist
    local tag="${1:-$(current_tag)}"
    [[ -n "$tag" ]] || die "no git tag found; pass a tag explicitly: $0 plan v0.2.0"
    echo "==> dist plan --tag=$tag"
    "$DIST_BIN" plan --tag="$tag"
}

cmd_build() {
    ensure_dist
    local tag="${1:-$(current_tag)}"
    [[ -n "$tag" ]] || die "no git tag found; pass a tag explicitly: $0 build v0.2.0"
    echo "==> dist build --tag=$tag"
    "$DIST_BIN" build --tag="$tag"
    echo ""
    echo "Artifacts written to: $REPO_ROOT/target/distrib/"
    ls -lh "$REPO_ROOT/target/distrib/" 2>/dev/null || true
}

cmd_generate() {
    ensure_dist
    echo "==> dist generate"
    "$DIST_BIN" generate
    echo "Done. Check .github/workflows/release.yml for changes."
}

cmd_check() {
    ensure_dist
    echo "==> dist generate --check"
    "$DIST_BIN" generate --check && echo "CI manifests are up to date."
}

cmd_tag() {
    local version="${1:-v$(pkg_version)}"
    # Normalise: ensure leading 'v'
    [[ "$version" == v* ]] || version="v$version"
    echo "Creating annotated tag $version ..."
    git tag -a "$version" -m "$version"
    echo "Tag created locally. Push with:"
    echo "  git push origin $version"
}

cmd_default() {
    ensure_dist
    local tag
    tag="$(current_tag)"
    if [[ -z "$tag" ]]; then
        tag="v$(pkg_version)"
        echo "No git tag found, using Cargo.toml version: $tag"
    fi

    echo "==> dist generate"
    "$DIST_BIN" generate
    echo ""

    echo "==> git diff (CI manifest changes)"
    git diff --stat
    git diff
    echo ""

    echo "==> dist generate --check"
    "$DIST_BIN" generate --check && echo "CI manifests are up to date."
    echo ""

    echo "==> dist plan --tag=$tag"
    "$DIST_BIN" plan --tag="$tag"
    echo ""

    echo "==> dist build --tag=$tag"
    "$DIST_BIN" build --tag="$tag"
    echo ""

    echo "Artifacts: $REPO_ROOT/target/distrib/"
    ls -lh "$REPO_ROOT/target/distrib/" 2>/dev/null || true
}

# ── dispatch ─────────────────────────────────────────────────────────────────

COMMAND="${1:-}"
shift || true

case "$COMMAND" in
    install)  cmd_install "$@" ;;
    plan)     cmd_plan "$@" ;;
    build)    cmd_build "$@" ;;
    generate) cmd_generate "$@" ;;
    check)    cmd_check "$@" ;;
    tag)      cmd_tag "$@" ;;
    "")       cmd_default ;;
    *)        die "unknown command '$COMMAND'. See header for usage." ;;
esac
