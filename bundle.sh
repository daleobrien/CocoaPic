#!/usr/bin/env bash
# Build CocoaPic.app from the release binary, as StorySearch's bundle.sh does.
#
#   ./bundle.sh              arm64 only, ad-hoc signed  (this machine)
#   ./bundle.sh --universal  arm64 + x86_64 via lipo    (any Mac)
#
# The icon is the old app's own, legacy/12C671.icns.
set -euo pipefail
cd "$(dirname "$0")"

APP="CocoaPic.app"
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
# Honour a shared CARGO_TARGET_DIR (or build.target-dir) rather than assuming ./target.
TARGET=$(cargo metadata --format-version 1 --no-deps | sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p')

# `cargo clean -p` before each build, so a build that thinks it has nothing to
# do cannot leave an old binary to be bundled (see StorySearch's bundle.sh).
if [[ "${1:-}" == "--universal" ]]; then
  rustup target add x86_64-apple-darwin aarch64-apple-darwin
  BIN="$TARGET/cocoapic-universal"
  rm -f "$BIN"
  cargo clean --release -p cocoapic --target x86_64-apple-darwin
  cargo clean --release -p cocoapic --target aarch64-apple-darwin
  cargo build --release --target x86_64-apple-darwin
  cargo build --release --target aarch64-apple-darwin
  lipo -create -output "$BIN" \
    "$TARGET/aarch64-apple-darwin/release/cocoapic" \
    "$TARGET/x86_64-apple-darwin/release/cocoapic"
else
  BIN="$TARGET/release/cocoapic"
  cargo clean --release -p cocoapic
  cargo build --release
fi

if [[ ! -f "$BIN" ]]; then
  echo "bundle.sh: the build produced no $BIN — not bundling" >&2
  exit 1
fi

rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/cocoapic"
cp legacy/12C671.icns "$APP/Contents/Resources/AppIcon.icns"

cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>              <string>CocoaPic</string>
  <key>CFBundleDisplayName</key>       <string>CocoaPic</string>
  <key>CFBundleIdentifier</key>        <string>net.dale.cocoapic</string>
  <key>CFBundleExecutable</key>        <string>cocoapic</string>
  <key>CFBundleIconFile</key>          <string>AppIcon</string>
  <key>CFBundlePackageType</key>       <string>APPL</string>
  <key>CFBundleShortVersionString</key><string>${VERSION}</string>
  <key>CFBundleVersion</key>           <string>${VERSION}</string>
  <key>LSMinimumSystemVersion</key>    <string>11.0</string>
  <key>NSHighResolutionCapable</key>   <true/>
  <key>NSPrincipalClass</key>          <string>NSApplication</string>
</dict>
</plist>
PLIST

codesign --force --deep -s - "$APP"
echo "built $APP  ($(du -sh "$APP" | cut -f1), $(lipo -archs "$APP/Contents/MacOS/cocoapic"))"
