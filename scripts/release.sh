#!/usr/bin/env bash

set -euxo pipefail
IFS=$'\n\t'

tag=$(git describe --tags --abbrev=0)

gh api "repos/$GITHUB_REPOSITORY/releases/generate-notes" \
  -f tag_name="${tag}" -q .body > CHANGELOG.md

gh release create "$tag" --notes-file CHANGELOG.md ./dist/*
