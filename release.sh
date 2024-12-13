#!/bin/bash
VERSION=$(grep '^version = ' Cargo.toml | cut -d '"' -f2)
git tag "v$VERSION"
git push origin "v$VERSION"

