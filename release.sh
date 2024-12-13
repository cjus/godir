#!/bin/bash
VERSION=$(grep '^version = ' Cargo.toml | cut -d '"' -f2)
echo "Creating tag v$VERSION..."
git tag "v$VERSION"
git push origin "v$VERSION"

echo "Calculating SHA256..."
SHASUM=$(curl -L "https://github.com/cjus/godir/archive/refs/tags/v$VERSION.tar.gz" | shasum -a 256 | cut -d ' ' -f1)
echo "SHA256: $SHASUM"

echo -n "Press Enter to install via Homebrew..."
read

brew tap cjus/godir
brew install godir

