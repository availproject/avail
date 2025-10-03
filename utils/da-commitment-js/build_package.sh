#!/usr/bin/env bash
set -euo pipefail

# version argument, default 0.0.1
VERSION="${1:-0.0.1}"

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

echo "▶ Cleaning previous outputs…"
rm -rf pkg pkg_node pkg_web

# --- Node build ---
echo "▶ Building Node package…"
wasm-pack build --target nodejs --features wasm
mv pkg pkg_node

# patch package.json (name + version) using sed
sed -i.bak \
  -e "s/\"name\": \".*\"/\"name\": \"da-commitment-node\"/" \
  -e "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" \
  pkg_node/package.json
rm pkg_node/package.json.bak

# --- Web build ---
echo "▶ Building Web package…"
wasm-pack build --target web --features wasm
mv pkg pkg_web

# patch package.json (name + version)
sed -i.bak \
  -e "s/\"name\": \".*\"/\"name\": \"da-commitment-web\"/" \
  -e "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" \
  pkg_web/package.json
rm pkg_web/package.json.bak

echo "✅ Builds ready."
echo
echo "Node package.json:"
grep -E '"name"|"version"' pkg_node/package.json
echo
echo "Web package.json:"
grep -E '"name"|"version"' pkg_web/package.json
echo

# --- Publish both ---
echo "▶ Publishing Node package..."
cd pkg_node
npm publish --access public
cd ..

echo "▶ Publishing Web package..."
cd pkg_web
npm publish --access public
cd ..

echo "✅ Published da-commitment-node@$VERSION and da-commitment-web@$VERSION"
