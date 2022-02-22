#!/usr/bin/env bash

set -euo pipefail

GIT_ROOT=$(git rev-parse --show-toplevel)

pushd "$GIT_ROOT" > /dev/null
cargo build --release 
cp target/release/gh-tf-mod gh-tf-mod
gh extension remove tf-mod 2> /dev/null || true
gh extension install .
