#!/usr/bin/env bash

set -euo pipefail

tag=$(git describe --tags --abbrev=0)

prerelease=""

if [[ "${tag}" =~ .*-.* ]]; then
  prerelease="-p"
fi

gh api "repos/$GITHUB_REPOSITORY/releases/generate-notes" \
  -f tag_name="${tag}" -q .body > CHANGELOG.md

gh release create "$tag" "$prerelease" --notes-file CHANGELOG.md dist/*

