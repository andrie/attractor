#!/bin/sh
# Copy the attractor-core crate from the sister repo into src/rust/attractor-core.
# Usage: tools/sync-core.sh [path-to-attractor-rs]   (default: ../attractor-rs)
set -e
SRC="${1:-../attractor-rs}/attractor-core"
DEST="src/rust/attractor-core"
[ -d "$SRC" ] || { echo "not found: $SRC" >&2; exit 1; }
rm -rf "$DEST"
mkdir -p "$DEST"
cp -R "$SRC/Cargo.toml" "$SRC/src" "$DEST/"
echo "synced $SRC -> $DEST"
