#!/bin/bash

set -eu

ORIGINAL_REPO_SUBMODULE_PATH=external/junk

if ! git diff --staged --exit-code; then
    echo "There are staged uncommitted changes." > /dev/stderr
    exit 1
fi

git submodule update --remote "$ORIGINAL_REPO_SUBMODULE_PATH"

if git diff --exit-code; then
    echo "No changes in the submodule." > /dev/stderr
    exit 1
fi

git add "$ORIGINAL_REPO_SUBMODULE_PATH"

if ! git diff --exit-code; then
    echo "There are unstaged changes in the working tree other than the submodule." > /dev/stderr
    git restore --staged .
    exit 1
fi

perl -i -pe 's/^(version\s*=\s*"\d+\.\d+\.)(\d+)"/$1 . ($2 + 1) . "\""/ge' Cargo.toml

if git diff --exit-code; then
    echo "Failed to update Cargo.toml." > /dev/stderr
    git restore --staged .
    exit 1
fi

git add Cargo.toml

if ! git diff --exit-code; then
    echo "There are unexpected changes in the working tree." > /dev/stderr
    git restore --staged .
    exit 1
fi

if ! cargo test; then
    echo "Tests failed after updating the submodule." > /dev/stderr
    git restore --staged .
    exit 1
fi

git commit -m "Update the original junk library to latest version"
git push
cargo publish

exit 0

