#!/bin/bash

set -eu

CURRENT_COMMIT="$(git submodule status external/junk | awk '{print $1}')"
ORIGINAL_GIT_URL="$(git config --file=.gitmodules submodule.external/junk.url)"
LATEST_COMMIT=$(git ls-remote "$ORIGINAL_GIT_URL" main | awk '{print $1}')

if [ "$CURRENT_COMMIT" != "$LATEST_COMMIT" ]; then
    exit 0
else
    exit 1
fi

